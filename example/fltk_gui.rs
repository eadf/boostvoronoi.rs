use boostvoronoi as BV;
use BV::prelude::*;

use fltk::{app, button, dialog, draw, enums, frame, group, menu, prelude::*, window};
use geo::prelude::Intersects;
use geo_cr as geo;
use ordered_float::OrderedFloat;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

#[macro_use]
extern crate bitflags;

// frame size
const FH: i32 = 790;
const FW: i32 = 790;

// window size
const WH: i32 = 800;
const WW: i32 = FW + 180;

const COORD_X_OFFSET: i32 = 5;
const COORD_Y_OFFSET: i32 = 5;

bitflags! {
    pub struct ColorFlag: ColorType {
        const EXTERNAL     = 0b00000001;
        const PRIMARY      = 0b00000010;
        const CURVE        = 0b00000100;
        // The edge/vertex belongs to a Cell defined by a segment
        const CELL_SEGMENT = 0b00001000;
        // The edge/vertex belongs to a Cell defined by a point
        const CELL_POINT   = 0b00010000;
        const INFINITE     = 0b00100000;
        // The vertex coincides with a site point
        const SITE_VERTEX  = 0b01000000;
    }
}

bitflags! {
    pub struct DrawFilterFlag: u32 {
        /// Edges considered to be outside all closed input geometry
        const EXTERNAL =      0b000000000000001;
        const PRIMARY =       0b000000000000010;
        const CURVE =         0b000000000000100;
        const VERTICES=       0b000000000001000;
        /// All edges
        const EDGES=          0b000000000010000;
        const SECONDARY =     0b000000000100000;
        /// Input geometry points
        const INPUT_POINT =   0b000000001000000;
        /// Input geometry segments
        const INPUT_SEGMENT = 0b000000010000000;
        /// Draw infinite edges
        const INFINITE =      0b001000000000000;
        /// Draw curves as straight lines
        const CURVE_LINE =    0b010000000000000;
        const SITE_VERTEX =   0b100000000000000;
        const DRAW_ALL =      0b111111111111111;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Example {
    File,
    Simple,
    Complex,
    Clean,
}

#[derive(Debug, Clone, Copy)]
pub enum GuiMessage {
    Filter(DrawFilterFlag),
    MenuChoice(Example),
}

/// The integer input type used by the voronoi builder
type IType = i32;
/// The integer output type used by the voronoi builder
type FType = f64;

struct SharedData {
    draw_flag: DrawFilterFlag,
    last_message: Option<GuiMessage>,
    visualizer: VoronoiVisualizer<IType, FType>,
    last_click: Option<BV::Point<IType>>,
}

/// The Offscreen is slightly offset from the window.
/// This method gives the mouse coordinates usable inside the offscreen.
#[inline(always)]
fn offscreen_event_coords() -> (i32, i32) {
    let pos = app::event_coords();
    (pos.0 - COORD_X_OFFSET, pos.1 - COORD_Y_OFFSET)
}

///! This example intends to visualize the half edge output of the voronoi algorithm.
///! Read all about the half edge data structure here:
///! <https://www.boost.org/doc/libs/1_75_0/libs/polygon/doc/voronoi_diagram.htm>
fn main() -> Result<(), BvError> {
    let app = app::App::default();
    let mut wind = window::Window::default()
        .with_size(WW, WH)
        .center_screen()
        .with_label("Boost voronoi ported to Rust");

    let mut frame = frame::Frame::new(COORD_X_OFFSET, COORD_Y_OFFSET, FW, FH, "");
    frame.set_color(enums::Color::Black);
    frame.set_frame(enums::FrameType::DownBox);

    let mut pack = group::Pack::new(COORD_X_OFFSET + FW, COORD_Y_OFFSET, 170, WH, "");
    pack.set_spacing(5);

    let mut menu_but = menu::MenuButton::default()
        .with_size(170, 25)
        .with_label("Menu");
    menu_but.set_frame(enums::FrameType::PlasticUpBox);

    let mut input_points_button = button::RoundButton::default()
        .with_size(180, 25)
        .with_label("input points");
    input_points_button.toggle(true);
    input_points_button.set_frame(enums::FrameType::PlasticUpBox);

    let mut input_segments_button = button::RoundButton::default()
        .with_size(180, 25)
        .with_label("input segments");
    input_segments_button.toggle(true);
    input_segments_button.set_frame(enums::FrameType::PlasticUpBox);

    let mut external_button = button::RoundButton::default()
        .with_size(180, 25)
        .with_label("externals");
    external_button.toggle(true);
    external_button.set_frame(enums::FrameType::PlasticUpBox);

    let mut vertices_button = button::RoundButton::default()
        .with_size(180, 25)
        .with_label("vertices (all)");
    vertices_button.toggle(true);
    vertices_button.set_frame(enums::FrameType::PlasticUpBox);

    let mut site_vertices_button = button::RoundButton::default()
        .with_size(180, 25)
        .with_label("site vertices");
    site_vertices_button.toggle(true);
    site_vertices_button.set_frame(enums::FrameType::PlasticUpBox);

    let mut edges_button = button::RoundButton::default()
        .with_size(180, 25)
        .with_label("edges (all)");
    edges_button.toggle(true);
    edges_button.set_frame(enums::FrameType::PlasticUpBox);

    let mut infinite_button = button::RoundButton::default()
        .with_size(180, 25)
        .with_label("infinite edges");
    infinite_button.toggle(true);
    infinite_button.set_frame(enums::FrameType::PlasticUpBox);

    let mut curved_button = button::RoundButton::default()
        .with_size(180, 25)
        .with_label("arc edges");
    curved_button.toggle(true);
    curved_button.set_frame(enums::FrameType::PlasticUpBox);

    let mut curved_as_lines_button = button::RoundButton::default()
        .with_size(180, 25)
        .with_label("arc as lines");
    curved_as_lines_button.toggle(false);
    curved_as_lines_button.set_frame(enums::FrameType::PlasticUpBox);

    let mut primary_button = button::RoundButton::default()
        .with_size(180, 25)
        .with_label("primary edges");
    primary_button.toggle(true);
    primary_button.set_frame(enums::FrameType::PlasticUpBox);

    let mut secondary_button = button::RoundButton::default()
        .with_size(180, 25)
        .with_label("secondary edges");
    secondary_button.toggle(true);
    secondary_button.set_frame(enums::FrameType::PlasticUpBox);

    let mut pack_x = group::Pack::default()
        .with_size(180, 25)
        .below_of(&secondary_button, 25);
    pack_x.set_type(group::PackType::Horizontal);
    let _ = frame::Frame::default().with_size(10, 25).with_label(" x:");
    let mut x_label = frame::Frame::default().with_size(160, 25).with_label("");
    x_label.set_align(enums::Align::Right | enums::Align::Inside);
    pack_x.end();

    let mut pack_y = group::Pack::default()
        .with_size(180, 25)
        .below_of(&pack_x, 25);
    pack_y.set_type(group::PackType::Horizontal);
    let _ = frame::Frame::default().with_size(10, 25).with_label(" y:");
    let mut y_label = frame::Frame::default().with_size(160, 25).with_label("");
    y_label.set_align(enums::Align::Right | enums::Align::Inside);
    pack_y.end();

    pack.end();

    wind.set_color(enums::Color::White);
    wind.end();
    wind.show();
    let offs = draw::Offscreen::new(frame.width(), frame.height()).unwrap();
    #[cfg(not(target_os = "macos"))]
    {
        offs.begin();
        draw::set_draw_color(enums::Color::White);
        draw::draw_rectf(0, 0, FW, FH);
        offs.end();
    }
    let offs = Rc::from(RefCell::from(offs));
    let offs_rc = Rc::clone(&offs);

    let shared_data_rc = Rc::new(RefCell::new(SharedData {
        // draw all except 'curve as straight line' as default
        draw_flag: DrawFilterFlag::DRAW_ALL ^ DrawFilterFlag::CURVE_LINE,
        last_message: None,
        visualizer: VoronoiVisualizer::default(),
        last_click: None,
    }));

    let (sender, receiver) = app::channel::<GuiMessage>();

    menu_but.add_emit(
        "From file",
        enums::Shortcut::None,
        menu::MenuFlag::Normal,
        sender,
        GuiMessage::MenuChoice(Example::File),
    );
    menu_but.add_emit(
        "Simple",
        enums::Shortcut::None,
        menu::MenuFlag::Normal,
        sender,
        GuiMessage::MenuChoice(Example::Simple),
    );
    menu_but.add_emit(
        "Complex",
        enums::Shortcut::None,
        menu::MenuFlag::Normal,
        sender,
        GuiMessage::MenuChoice(Example::Complex),
    );
    menu_but.add_emit(
        "Clean",
        enums::Shortcut::None,
        menu::MenuFlag::Normal,
        sender,
        GuiMessage::MenuChoice(Example::Clean),
    );

    external_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::EXTERNAL));
    infinite_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::INFINITE));
    primary_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::PRIMARY));
    secondary_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::SECONDARY));
    input_points_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::INPUT_POINT));
    input_segments_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::INPUT_SEGMENT));
    curved_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::CURVE));
    vertices_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::VERTICES));
    site_vertices_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::SITE_VERTEX));
    edges_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::EDGES));
    curved_as_lines_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::CURVE_LINE));

    {
        // initialize visualizer
        let cl = Rc::clone(&shared_data_rc);
        let mut shared_data_bm = cl.borrow_mut();
        shared_data_bm.visualizer.read_data(Example::Complex);
        let _ = shared_data_bm.visualizer.build()?;
        shared_data_bm.visualizer.re_calculate_affine()?;
    }

    let shared_data_c = Rc::clone(&shared_data_rc);
    // This is called whenever the window is drawn and redrawn
    wind.draw(move |_| {
        if let Ok(data_b) = shared_data_c.try_borrow() {
            // todo, move the actual drawing away from draw() function, only keep the off-screen blit.
            offs_rc.borrow_mut().begin();

            draw::set_draw_color(enums::Color::White);
            draw::draw_rectf(0, 0, FW, FH);
            let _ = data_b.visualizer.draw(&data_b);
            offs_rc.borrow_mut().end();

            if offs_rc.borrow().is_valid() {
                offs_rc.borrow().copy(5, 5, FW, FH, 0, 0);
            } else {
                // this will almost never be called
                let data_b = shared_data_c.borrow();
                offs_rc.borrow_mut().begin();
                draw::set_draw_color(enums::Color::Yellow);
                draw::draw_rectf(5, 5, FW, FH);
                let _ = data_b.visualizer.draw(&data_b);
                offs_rc.borrow_mut().end();
            }
        }
    });

    let shared_data_c = Rc::clone(&shared_data_rc);
    let mut mouse_drag: Option<(i32, i32)> = None;

    wind.handle(move |_, ev| match ev {
        enums::Event::MouseWheel => {
            let mouse_position = offscreen_event_coords();
            let mut shared_data_bm = shared_data_c.borrow_mut();
            let event_dy = match app::event_dy() {
                app::MouseWheel::Up => 3,
                app::MouseWheel::Down => -3,
                _ => 0,
            };
            let reverse_middle = shared_data_bm
                .visualizer
                .affine
                .reverse_transform::<IType>(mouse_position.0 as f64, mouse_position.1 as f64);
            if reverse_middle.is_err() {
                println!("{:?}", reverse_middle.err().unwrap());
                return false;
            }
            let reverse_middle = reverse_middle.unwrap();
            if event_dy != 0 {
                let scale_mod = 1.01_f64.powf(event_dy as f64);
                shared_data_bm.visualizer.affine.scale[0] *= scale_mod;
                shared_data_bm.visualizer.affine.scale[1] *= scale_mod;
            }
            let new_middle = shared_data_bm
                .visualizer
                .affine
                .transform(reverse_middle[0] as f64, reverse_middle[1] as f64);
            // When zooming we want the center of screen remain at the same relative position.
            shared_data_bm.visualizer.affine.to_offset[0] +=
                mouse_position.0 as f64 - new_middle[0];
            shared_data_bm.visualizer.affine.to_offset[1] +=
                mouse_position.1 as f64 - new_middle[1];

            //println!("mouse wheel at dy:{:?} scale:{:?}", event_dy, shared_data_bm.visualizer.affine.scale);
            app::redraw();
            true
        }
        enums::Event::Drag => {
            let mouse_position = offscreen_event_coords();

            if mouse_drag.is_none() {
                mouse_drag = Some(mouse_position);
            } else {
                let md = mouse_drag.unwrap();
                let mut shared_data_bm = shared_data_c.borrow_mut();
                shared_data_bm.visualizer.affine.to_offset[0] += (mouse_position.0 - md.0) as f64;
                shared_data_bm.visualizer.affine.to_offset[1] += (mouse_position.1 - md.1) as f64;
                mouse_drag = Some(mouse_position);
                app::redraw();
            }
            true
        }
        enums::Event::Released => {
            let mouse_position = offscreen_event_coords();
            if mouse_drag.is_some() {
                mouse_drag = None;
            } else if app::event_key_down(enums::Key::from_char('L'))
                || app::event_key_down(enums::Key::from_char('S'))
            {
                let mut shared_data_bm = shared_data_c.borrow_mut();
                let point = shared_data_bm
                    .visualizer
                    .affine
                    .reverse_transform(mouse_position.0 as f64, mouse_position.1 as f64);
                if point.is_err() {
                    println!("{:?}", point.err().unwrap());
                    return false;
                }
                let point = BV::Point::from(point.unwrap());
                if let Some(last_point) = shared_data_bm.last_click {
                    let line = BV::Line {
                        start: last_point,
                        end: point,
                    };

                    if !shared_data_bm.visualizer.self_intersecting_check(&line) {
                        shared_data_bm.visualizer.segment_data_.push(line);

                        let _ = shared_data_bm.visualizer.build().unwrap();

                        if app::event_key_down(enums::Key::from_char('L')) {
                            shared_data_bm.last_click = None;
                        } else {
                            shared_data_bm.last_click = Some(point);
                        }
                        app::redraw();
                    }
                } else {
                    shared_data_bm.last_click = Some(point);
                }
            } else {
                if app::event_x() < FW {
                    let mut shared_data_bm = shared_data_c.borrow_mut();
                    {
                        let mouse_position = offscreen_event_coords();
                        let point = shared_data_bm
                            .visualizer
                            .affine
                            .reverse_transform(mouse_position.0 as f64, mouse_position.1 as f64);
                        if point.is_err() {
                            println!("{:?}", point.err().unwrap());
                            return false;
                        }
                        let point = BV::Point::from(point.unwrap());
                        shared_data_bm.visualizer.point_data_.push(point);
                    }
                    let _ = shared_data_bm.visualizer.build().unwrap();

                    shared_data_bm.last_click = None;
                    app::redraw();
                }
            }
            true
        }
        enums::Event::KeyDown => {
            if app::event_key_down(enums::Key::from_char('C')) {
                let mut shared_data_bm = shared_data_c.borrow_mut();
                shared_data_bm.last_click = None;
                shared_data_bm.visualizer.segment_data_.clear();
                shared_data_bm.visualizer.point_data_.clear();
                shared_data_bm.visualizer.diagram.clear();
                app::redraw();
            }
            false
        }
        enums::Event::KeyUp => {
            let mut shared_data_bm = shared_data_c.borrow_mut();
            shared_data_bm.last_click = None;
            false
        }
        enums::Event::Move => {
            // only update coordinate when hovering over the graphics
            let mouse_position = offscreen_event_coords();
            if mouse_position.0 < FW {
                if let Ok(shared_data_b) = shared_data_c.try_borrow() {
                    // todo: IDK why the borrow sometimes fails
                    let point = shared_data_b.visualizer.affine.reverse_transform::<IType>(
                        mouse_position.0 as f64,
                        mouse_position.1 as f64,
                    );
                    if let Ok(point) = point {
                        x_label.set_label(&point[0].to_string());
                        y_label.set_label(&point[1].to_string());
                    } else {
                        x_label.set_label("?");
                        y_label.set_label("?");
                    }
                } else {
                    println!("shared_data_c.try_borrow() failed");
                }
            }
            false
        }
        _ => false,
    });

    let shared_data_c = Rc::clone(&shared_data_rc);
    while app.wait() {
        if let Some(msg) = receiver.recv() {
            let mut shared_data_bm: RefMut<_> = shared_data_c.borrow_mut();
            match msg {
                GuiMessage::MenuChoice(v) => {
                    let new_title = shared_data_bm.visualizer.read_data(v);
                    {
                        let w = &mut wind;
                        w.set_label(new_title.as_str());
                    }
                    let _ = shared_data_bm.visualizer.build()?;
                    let _ = shared_data_bm.visualizer.re_calculate_affine();
                    //app::redraw();
                }
                GuiMessage::Filter(flag) => {
                    shared_data_bm.draw_flag ^= flag;
                }
            }
            shared_data_bm.last_message = Some(msg);
            app::redraw();
        }
    }
    Ok(())
}

/// struct to help deal with the voronoi diagram input and output
pub struct VoronoiVisualizer<I: InputType, F: OutputType> {
    screen_aabb: BV::Aabb2<F>,
    diagram: BV::Diagram<F>,
    points_aabb: BV::Aabb2<F>,

    point_data_: Vec<BV::Point<I>>,
    segment_data_: Vec<BV::Line<I>>,
    affine: BV::SimpleAffine<F>,
}

impl<I: InputType, F: OutputType> VoronoiVisualizer<I, F> {
    pub fn default() -> Self {
        Self {
            screen_aabb: BV::Aabb2::<F>::new_from_i32::<I>(0, 0, FW, FH),
            diagram: BV::Diagram::<F>::default(),
            points_aabb: BV::Aabb2::<F>::default(),
            point_data_: Vec::<BV::Point<I>>::new(),
            segment_data_: Vec::<BV::Line<I>>::new(),
            affine: BV::SimpleAffine::default(),
        }
    }

    /// recalculates the affine transformation, this should not be done every time
    /// the diagram is re-calculated or the screen will move around when adding new edges and points.
    pub fn re_calculate_affine(&mut self) -> Result<(), BvError> {
        self.affine = BV::SimpleAffine::new::<I>(&self.points_aabb, &self.screen_aabb)?;

        // Flip the z axis because fltk uses quadrant 4 when drawing
        self.affine.scale[1] *= F::from(-1.0).unwrap();
        Ok(())
    }

    pub fn build(&mut self) -> Result<String, BvError> {
        if false {
            // This generates Rust test data
            println!(
                "Running voronoi with this input (in case of a crash, copy&paste and make a test case)"
            );
            print!("  let points:[[i32;2];{}]=[", self.point_data_.len());
            for p in self.point_data_.iter() {
                print!("[{},{}],", p.x, p.y)
            }
            println!("];");
            print!("  let segments:[[i32;4];{}]=[", self.segment_data_.len());
            for s in self.segment_data_.iter() {
                print!("[{},{},{},{}],", s.start.x, s.start.y, s.end.x, s.end.y)
            }
            println!("];");
        }
        if false {
            // This generates C++ test data
            print!("  int INPUT_PTS[{}][2]={{", self.point_data_.len());
            for p in self.point_data_.iter() {
                print!("{{{},{}}},", p.x, p.y)
            }
            println!("}};");

            print!("  int INPUT_SGS[{}][4]={{", self.segment_data_.len());
            for s in self.segment_data_.iter() {
                print!("{{{},{},{},{}}},", s.start.x, s.start.y, s.end.x, s.end.y)
            }
            println!("}};");
        }
        self.diagram = BV::Builder::<I, F>::default()
            .with_vertices(self.point_data_.iter())?
            .with_segments(self.segment_data_.iter())?
            // Construct voronoi diagram.
            .build()?;
        println!("Result: found {} vertices", self.diagram.vertices().len());
        self.points_aabb = {
            let mut aabb = BV::Aabb2::default();
            for p in self.point_data_.iter() {
                aabb.update_point(p);
            }
            for l in self.segment_data_.iter() {
                aabb.update_line(l);
            }
            aabb.grow_percent::<I>(20);
            aabb
        };

        // Color exterior edges.
        self.diagram.color_exterior_edges(ColorFlag::EXTERNAL.bits);

        // Color infinite edges
        for it in self.diagram.edges().iter() {
            let edge_id = it.get().id();
            if !self.diagram.edge_is_finite(edge_id)? {
                self.diagram
                    .edge_or_color(edge_id, ColorFlag::INFINITE.bits)?;
            }
        }

        Result::Ok("".to_string())
    }

    // returns true if l intersects with any of the lines in self.segment_data_
    fn self_intersecting_check(&self, l: &BV::Line<I>) -> bool {
        let l_ = Self::line_i_to_f64(l);
        for s in self.segment_data_.iter() {
            // allow end point intersection
            if (s.start.x == l.start.x && s.start.y == l.start.y)
                && (s.end.x != l.end.x && s.end.y != l.end.y)
            {
                continue;
            }
            if (s.start.x == l.end.x && s.start.y == l.end.y)
                && (s.end.x != l.start.x && s.end.y != l.start.y)
            {
                continue;
            }
            if (s.end.x == l.end.x && s.end.y == l.end.y)
                && (s.start.x != l.start.x && s.start.y != l.start.y)
            {
                continue;
            }
            if (s.end.x == l.start.x && s.end.y == l.start.y)
                && (s.start.x != l.end.x && s.start.y != l.end.y)
            {
                continue;
            }

            let s_ = Self::line_i_to_f64(s);
            let s_: geo::Line<f64> = s_.into();
            let l_: geo::Line<f64> = l_.into();

            if l_.intersects(&s_) {
                return true;
            }
        }
        false
    }

    fn draw(&self, config: &SharedData) -> Result<(), BvError> {
        draw::set_line_style(draw::LineStyle::Solid, 1);

        draw::set_draw_color(enums::Color::Red);
        if config.draw_flag.contains(DrawFilterFlag::INPUT_POINT) {
            self.draw_input_points(&self.affine);
        }
        if config.draw_flag.contains(DrawFilterFlag::INPUT_SEGMENT) {
            self.draw_input_segments(&self.affine);
        }
        if config.draw_flag.contains(DrawFilterFlag::EDGES) {
            draw::set_draw_color(enums::Color::Green);
            self.draw_edges(&config, &self.affine)?;
        }
        if config.draw_flag.contains(DrawFilterFlag::VERTICES) {
            draw::set_draw_color(enums::Color::Blue);
            self.draw_vertices(&config, &self.affine);
        }
        Ok(())
    }

    /// Draw input points and endpoints of the input segments.
    fn draw_input_points(&self, affine: &BV::SimpleAffine<F>) {
        let draw = |point: [F; 2]| {
            draw::draw_circle(cast::<F, f64>(point[0]), cast::<F, f64>(point[1]), 2.0);
        };

        for i in self.point_data_.iter() {
            draw(affine.transform_p(&i));
        }

        for i in self.segment_data_.iter() {
            draw(affine.transform_p(&i.start));
            draw(affine.transform_p(&i.end));
        }
    }

    /// Draw input segments.
    fn draw_input_segments(&self, affine: &BV::SimpleAffine<F>) {
        for i in self.segment_data_.iter() {
            let sp = affine.transform_p(&i.start);
            let ep = affine.transform_p(&i.end);
            draw::draw_line(
                cast::<F, i32>(sp[0]),
                cast::<F, i32>(sp[1]),
                cast::<F, i32>(ep[0]),
                cast::<F, i32>(ep[1]),
            );
        }
    }

    /// Draw voronoi vertices aka circle events.
    fn draw_vertices(&self, config: &SharedData, affine: &BV::SimpleAffine<F>) {
        let draw = |x: f64, y: f64| {
            draw::draw_circle(x, y, 1.0);
        };
        let draw_external = config.draw_flag.contains(DrawFilterFlag::EXTERNAL);
        let draw_site_vertex = config.draw_flag.contains(DrawFilterFlag::SITE_VERTEX);

        for it in self.diagram.vertices().iter().enumerate() {
            let vertex = it.1.get();

            if (!draw_site_vertex) && vertex.is_site_point() {
                continue;
            }
            if (!draw_external)
                && ColorFlag::from_bits(vertex.get_color())
                    .unwrap()
                    .contains(ColorFlag::EXTERNAL)
            {
                continue;
            }

            draw(
                cast::<F, f64>(affine.transform_x(vertex.x())),
                cast::<F, f64>(affine.transform_y(vertex.y())),
            );
        }
    }

    /// Draw voronoi edges.
    fn draw_edges(&self, config: &SharedData, affine: &BV::SimpleAffine<F>) -> Result<(), BvError> {
        let draw_external = config.draw_flag.contains(DrawFilterFlag::EXTERNAL);
        let draw_primary = config.draw_flag.contains(DrawFilterFlag::PRIMARY);
        let draw_secondary = config.draw_flag.contains(DrawFilterFlag::SECONDARY);
        let draw_curved = config.draw_flag.contains(DrawFilterFlag::CURVE);
        let draw_curved_as_line = config.draw_flag.contains(DrawFilterFlag::CURVE_LINE);
        let draw_infinite_edges = config.draw_flag.contains(DrawFilterFlag::INFINITE);

        let mut already_drawn = {
            let l = self.diagram.edges().len();
            let mut vb = vob::Vob::<u32>::new_with_storage_type(l);
            vb.resize(l, false);
            vb
        };

        for it in self.diagram.edges().iter().enumerate() {
            draw::set_draw_color(enums::Color::DarkGreen);
            let edge_id = EdgeIndex(it.0);
            let edge = it.1.get();
            if already_drawn.get(edge_id.0).unwrap_or(false) {
                // already done this, or rather - it's twin
                continue;
            }
            // no point in setting current edge as drawn, the edge id will not repeat
            // already_drawn.set_bit(edge_id.0, true);
            let twin = self.diagram.edge_get_twin(edge_id)?;
            already_drawn.set(twin.0, true);

            //#[allow(unused_assignments)]
            if (!draw_primary) && edge.is_primary() {
                continue;
            }
            if edge.is_secondary() && (!draw_secondary) {
                continue;
            }
            if ColorFlag::from_bits(edge.get_color())
                .unwrap()
                .contains(ColorFlag::INFINITE)
            {
                if !draw_infinite_edges {
                    continue;
                } else {
                    draw::set_draw_color(enums::Color::Green);
                }
            }

            if ColorFlag::from_bits(edge.get_color())
                .unwrap()
                .contains(ColorFlag::EXTERNAL)
            {
                if !draw_external {
                    continue;
                } else {
                    draw::set_draw_color(enums::Color::Green);
                }
            }

            // the coordinates in samples must be 'screen' coordinates, i.e. affine transformed
            let mut samples = Vec::<[F; 2]>::new();
            if self.diagram.edge_is_infinite(edge_id)? {
                let a = self.clip_infinite_edge(&affine, edge_id, &mut samples);
                if let Err(err) = a {
                    println!("Ignoring error : {:?}", err);
                }
            } else {
                // edge is finite, so vertex0 & vertex1 must exists -> unwrap is safe
                let vertex0 = self.diagram.vertex_get(edge.vertex0().unwrap())?.get();

                samples.push(affine.transform(vertex0.x(), vertex0.y()));
                let vertex1 = self.diagram.edge_get_vertex1(edge_id)?.unwrap();
                let vertex1 = self.diagram.vertex_get(vertex1).unwrap().get();

                samples.push(affine.transform(vertex1.x(), vertex1.y()));
                if edge.is_curved() {
                    if draw_curved_as_line {
                        for i in 0..samples.len() - 1 {
                            if let Ok(x1) = try_cast::<F, i32>(samples[i][0]) {
                                if let Ok(y1) = try_cast::<F, i32>(samples[i][1]) {
                                    if let Ok(x2) = try_cast::<F, i32>(samples[i + 1][0]) {
                                        if let Ok(y2) = try_cast::<F, i32>(samples[i + 1][1]) {
                                            draw::draw_line(x1, y1, x2, y2);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if draw_curved {
                        self.sample_curved_edge(&affine, EdgeIndex(it.0), &mut samples)?;
                    } else {
                        continue;
                    }
                }
            }
            if samples.len() > 1 {
                for i in 0..samples.len() - 1 {
                    let x1 = try_cast::<F, i32>(samples[i][0]);
                    if x1.is_err() {
                        break;
                    }
                    let y1 = try_cast::<F, i32>(samples[i][1]);
                    if y1.is_err() {
                        break;
                    }
                    let x2 = try_cast::<F, i32>(samples[i + 1][0]);
                    if x2.is_err() {
                        break;
                    }
                    let y2 = try_cast::<F, i32>(samples[i + 1][1]);
                    if y2.is_err() {
                        break;
                    }

                    draw::draw_line(x1.unwrap(), y1.unwrap(), x2.unwrap(), y2.unwrap());
                }
            }
        }
        Ok(())
    }

    fn clip_infinite_edge(
        &self,
        affine: &BV::SimpleAffine<F>,
        edge_id: BV::EdgeIndex,
        clipped_edge: &mut Vec<[F; 2]>,
    ) -> Result<(), BvError> {
        let edge = self.diagram.get_edge(edge_id)?;
        //const cell_type& cell1 = *edge.cell();
        let cell1_id = self.diagram.edge_get_cell(edge_id)?;
        let cell1 = self.diagram.get_cell(cell1_id)?.get();
        //const cell_type& cell2 = *edge.twin()->cell();
        let cell2_id = {
            let twin = self.diagram.edge_get_twin(edge_id)?;
            self.diagram.edge_get_cell(twin)?
        };
        let cell2 = self.diagram.get_cell(cell2_id)?.get();

        let mut origin = [F::default(), F::default()];
        let mut direction = [F::default(), F::default()];
        // Infinite edges could not be created by two segment sites.
        if cell1.contains_point() && cell2.contains_point() {
            let p1 = self.retrieve_point(cell1_id)?;
            let p2 = self.retrieve_point(cell2_id)?;
            origin[0] = (cast::<I, F>(p1.x) + cast::<I, F>(p2.x)) * cast::<f64, F>(0.5);
            origin[1] = (cast::<I, F>(p1.y) + cast::<I, F>(p2.y)) * cast::<f64, F>(0.5);
            direction[0] = cast::<I, F>(p1.y) - cast::<I, F>(p2.y);
            direction[1] = cast::<I, F>(p2.x) - cast::<I, F>(p1.x);
        } else {
            origin = if cell1.contains_segment() {
                let p = self.retrieve_point(cell2_id)?;
                [cast::<I, F>(p.x), cast::<I, F>(p.y)]
            } else {
                let p = self.retrieve_point(cell1_id)?;
                [cast::<I, F>(p.x), cast::<I, F>(p.y)]
            };
            let segment = if cell1.contains_segment() {
                self.retrieve_segment(cell1_id)?
            } else {
                self.retrieve_segment(cell2_id)?
            };
            let dx = segment.end.x - segment.start.x;
            let dy = segment.end.y - segment.start.y;
            if ([cast::<I, F>(segment.start.x), cast::<I, F>(segment.start.y)] == origin)
                ^ cell1.contains_point()
            {
                direction[0] = cast::<I, F>(dy);
                direction[1] = cast::<I, F>(-dx);
            } else {
                direction[0] = cast::<I, F>(-dy);
                direction[1] = cast::<I, F>(dx);
            }
        }

        let side =
            cast::<I, F>(affine.reverse_transform_x(self.screen_aabb.get_high().unwrap()[0])?)
                - cast::<I, F>(affine.reverse_transform_x(self.screen_aabb.get_low().unwrap()[0])?);
        // absolute value is taken in case the affine transform flips one coordinate
        let side = side.abs();
        let coefficient = side / Self::max_f(direction[0].abs(), direction[1].abs());

        if let Some(vertex0) = edge.get().vertex0() {
            let vertex0 = self.diagram.vertex_get(vertex0)?.get();
            clipped_edge.push([
                affine.transform_x(vertex0.x()),
                affine.transform_y(vertex0.y()),
            ]);
        } else {
            clipped_edge.push([
                affine.transform_x(origin[0] - direction[0] * coefficient),
                affine.transform_y(origin[1] - direction[1] * coefficient),
            ]);
        }

        if let Some(vertex1) = self.diagram.edge_get_vertex1(edge_id)? {
            let vertex1 = self.diagram.vertex_get(vertex1)?.get();
            clipped_edge.push([
                affine.transform_x(vertex1.x()),
                affine.transform_y(vertex1.y()),
            ]);
        } else {
            clipped_edge.push([
                affine.transform_x(origin[0] + direction[0] * coefficient),
                affine.transform_y(origin[1] + direction[1] * coefficient),
            ]);
        }
        Ok(())
    }

    /// Important: sampled_edge should contain both edge endpoints initially.
    /// sampled_edge should be 'screen' coordinates, i.e. affine transformed from voronoi output
    fn sample_curved_edge(
        &self,
        affine: &BV::SimpleAffine<F>,
        edge_id: BV::EdgeIndex,
        sampled_edge: &mut Vec<[F; 2]>,
    ) -> Result<(), BvError> {
        let max_dist = cast::<f64, F>(1E-3)
            * (self.screen_aabb.get_high().unwrap()[0] - self.screen_aabb.get_low().unwrap()[0]);

        let cell_id = self.diagram.edge_get_cell(edge_id)?;
        let cell = self.diagram.get_cell(cell_id)?.get();
        let twin_id = self.diagram.edge_get_twin(edge_id)?;
        let twin_cell_id = self.diagram.edge_get_cell(twin_id)?;

        let point = if cell.contains_point() {
            self.retrieve_point(cell_id)?
        } else {
            self.retrieve_point(twin_cell_id)?
        };
        let segment = if cell.contains_point() {
            self.retrieve_segment(twin_cell_id)?
        } else {
            self.retrieve_segment(cell_id)?
        };
        BV::VoronoiVisualUtils::discretize::<I, F>(&point, segment, max_dist, affine, sampled_edge);
        Ok(())
    }

    /// Retrieves a point from the voronoi input in the order it was presented to
    /// the voronoi builder
    fn retrieve_point(&self, cell_id: BV::CellIndex) -> Result<BV::Point<I>, BvError> {
        let (index, cat) = self.diagram.get_cell(cell_id)?.get().source_index_2();
        match cat {
            BV::SourceCategory::SinglePoint => Ok(self.point_data_[index]),
            BV::SourceCategory::SegmentStart => {
                Ok(self.segment_data_[index - self.point_data_.len()].start)
            }
            BV::SourceCategory::Segment | BV::SourceCategory::SegmentEnd => {
                Ok(self.segment_data_[index - self.point_data_.len()].end)
            }
        }
    }

    /// Retrieves a segment from the voronoi input in the order it was presented to
    /// the voronoi builder
    fn retrieve_segment(&self, cell_id: BV::CellIndex) -> Result<&BV::Line<I>, BvError> {
        let cell = self.diagram.get_cell(cell_id)?.get();
        let index = cell.source_index() - self.point_data_.len();
        Ok(&self.segment_data_[index])
    }

    #[allow(unused_assignments)]
    fn read_data(&mut self, example: Example) -> String {
        self.segment_data_.clear();
        self.point_data_.clear();
        self.diagram.clear();
        let mut rv = String::new();

        let _simple_segments: [[i32; 4]; 5] = [
            [300, 300, 300, 500],
            [300, 500, 500, 500],
            [500, 500, 500, 300],
            [500, 300, 300, 300],
            [629, 342, 467, 207],
        ];

        let _segments_rust_logo: [[i32; 4]; 350] = include!("input_data/rust_logo.rs");

        // Preparing Input Geometries.
        let (mut new_points, mut new_segments) = match example {
            Example::Simple => {
                rv = "Simple example".to_string();
                (
                    Vec::<BV::Point<I>>::default(),
                    _simple_segments
                        .iter()
                        .map(|l| Line::from(l).cast::<I>())
                        .collect::<Vec<_>>(),
                )
            }
            Example::Complex => {
                rv = "Rust logo".to_string();
                (
                    Vec::<BV::Point<I>>::default(),
                    _segments_rust_logo
                        .iter()
                        .map(|l| Line::from(l).cast::<I>())
                        .collect::<Vec<_>>(),
                )
            }
            Example::Clean => {
                rv = "Clean".to_string();
                let clean: [[I; 4]; 0] = [];
                (
                    Vec::<BV::Point<I>>::default(),
                    clean
                        .iter()
                        .map(|l| Line::<I>::from(l).cast::<I>())
                        .collect::<Vec<_>>(),
                )
            }
            Example::File => {
                let mut chooser = dialog::NativeFileChooser::new(dialog::FileDialogType::BrowseDir);

                let _ = chooser.set_directory(std::path::Path::new("examples"));
                let _ = chooser.set_title("select your input data");
                chooser.set_filter("*.txt");
                chooser.show();
                if let Some(filename) = chooser.filenames().first() {
                    if let Ok(file_parse_result) =
                        BV::read_boost_input_file::<I>(filename.as_path())
                    {
                        rv = filename.to_str().unwrap().to_string();
                        file_parse_result
                    } else {
                        rv = "Failed to read file".to_string();
                        (
                            Vec::<BV::Point<I>>::default(),
                            Vec::<BV::Line<I>>::default(),
                        )
                    }
                } else {
                    rv = "Failed to read file".to_string();
                    (
                        Vec::<BV::Point<I>>::default(),
                        Vec::<BV::Line<I>>::default(),
                    )
                }
            }
        };
        self.point_data_.append(&mut new_points);
        self.segment_data_.append(&mut new_segments);
        rv
    }

    #[inline(always)]
    /// converts from BV::Line to geo::Line.
    /// I wonder why my nice geo::Line::from(BV::Line) does not work here, feature gated?.
    fn line_i_to_f64(value: &BV::Line<I>) -> geo::Line<f64> {
        let ps = geo::Coordinate {
            x: cast::<I, f64>(value.start.x),
            y: cast::<I, f64>(value.start.y),
        };
        let pe = geo::Coordinate {
            x: cast::<I, f64>(value.end.x),
            y: cast::<I, f64>(value.end.y),
        };
        geo::Line::<f64>::new(ps, pe)
    }

    #[inline(always)]
    fn max_f(a: F, b: F) -> F {
        OrderedFloat(a).max(OrderedFloat(b)).into_inner()
    }
}
