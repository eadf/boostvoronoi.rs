use boostvoronoi::diagram as VD;
use boostvoronoi::diagram::VoronoiEdgeIndex;
use boostvoronoi::file_reader;
use boostvoronoi::visual_utils as VU;
use boostvoronoi::BvError;
use boostvoronoi::{builder as VB, Line, Point, TypeConverter1, TypeConverter2};
use boostvoronoi::{InputType, OutputType};

use std::ops::Neg;

use fltk::{app, button, dialog, draw, enums, frame, group, menu, prelude::*, window};

use geo::prelude::Intersects;
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

bitflags! {
    pub struct ColorFlag: VD::ColorType {
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
        /// Edge belonging to cells defined by a segment
        const E_CELL_SEGMENT= 0b000000100000000;
        /// Edge belonging to cells defined by a point
        const E_CELL_POINT =  0b000001000000000;
        /// Vertices belonging to cells defined by a segment
        const V_CELL_SEGMENT= 0b000010000000000;
        /// Vertices belonging to cells defined by a point
        const V_CELL_POINT =  0b000100000000000;
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

#[allow(dead_code)]
struct SharedData {
    draw_flag: DrawFilterFlag,
    last_message: Option<GuiMessage>,
    visualizer: VoronoiVisualizer<i32, f64>,
    last_click: Option<Point<i32>>,
}

///! This example intends to visualize the half edge output of the voronoi algorithm
///!
///! As an experiment I added (semi-useful) filters for :
///!    cell segment edges = edges only belonging to cells defined by segments
///!    cell point edges = edges only belonging to cells defined by points
///!    cell segment vertices = vertices only belonging to cells defined by segments
///!    cell point vertices = vertices only belonging to cells defined by points
///!
///! Read all about the half edge data structure here:
///! <https://www.boost.org/doc/libs/1_75_0/libs/polygon/doc/voronoi_diagram.htm>
fn main() -> Result<(), BvError> {
    let app = app::App::default();
    let mut wind = window::Window::default()
        .with_size(WW, WH)
        .center_screen()
        .with_label("Boost voronoi ported to Rust");

    let mut frame = frame::Frame::new(5, 5, FW, FH, "");
    frame.set_color(enums::Color::Black);
    frame.set_frame(enums::FrameType::DownBox);

    let mut pack = group::Pack::new(5 + FW, 5, 170, WH, "");
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

    /*let mut e_segment_cell_button = button::RoundButton::default()
            .with_size(180, 25)
            .with_label("cell segment edges");
        e_segment_cell_button.toggle(true);
        e_segment_cell_button.set_frame(FrameType::PlasticUpBox);

        let mut e_point_cell_button = button::RoundButton::default()
            .with_size(180, 25)
            .with_label("cell point edges");
        e_point_cell_button.toggle(true);
        e_point_cell_button.set_frame(FrameType::PlasticUpBox);

        let mut v_segment_cell_button = button::RoundButton::default()
            .with_size(180, 25)
            .with_label("cell segment vertices");
        v_segment_cell_button.toggle(true);
        v_segment_cell_button.set_frame(FrameType::PlasticUpBox);

        let mut v_point_cell_button = button::RoundButton::default()
            .with_size(180, 25)
            .with_label("cell point vertices");
        v_point_cell_button.toggle(true);
        v_point_cell_button.set_frame(FrameType::PlasticUpBox);
    */
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

    //e_segment_cell_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::E_CELL_SEGMENT));
    //e_point_cell_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::E_CELL_POINT));
    //v_segment_cell_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::V_CELL_SEGMENT));
    //v_point_cell_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::V_CELL_POINT));
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
        let _ = shared_data_bm.visualizer.build();
        shared_data_bm.visualizer.re_calculate_affine()?;
    }

    let shared_data_c = Rc::clone(&shared_data_rc);
    // This is called whenever the window is drawn and redrawn
    wind.draw(move |_| {
        if let Ok(data_b) = shared_data_c.try_borrow() {
            // todo, move the actual drawing away from draw() function, only keep the offscreen blit.
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
            let event = &app::event_coords();
            let mut shared_data_bm = shared_data_c.borrow_mut();
            let event_dy = match app::event_dy() {
                app::MouseWheel::Up => 3,
                app::MouseWheel::Down => -3,
                _ => 0,
            };
            let reverse_middle = shared_data_bm
                .visualizer
                .affine
                .reverse_transform(event.0 as f64, event.1 as f64);
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
            shared_data_bm.visualizer.affine.to_offset[0] += (event.0 as f64) - new_middle[0];
            shared_data_bm.visualizer.affine.to_offset[1] += (event.1 as f64) - new_middle[1];

            //println!("mouse wheel at dy:{:?} scale:{:?}", event_dy, shared_data_bm.visualizer.affine.scale);
            app::redraw();
            true
        }
        enums::Event::Drag => {
            let event = &app::event_coords();
            if mouse_drag.is_none() {
                mouse_drag = Some(*event);
            } else {
                let md = mouse_drag.unwrap();
                let mut shared_data_bm = shared_data_c.borrow_mut();
                shared_data_bm.visualizer.affine.to_offset[0] += (event.0 - md.0) as f64;
                shared_data_bm.visualizer.affine.to_offset[1] += (event.1 - md.1) as f64;
                mouse_drag = Some(*event);
                app::redraw();
            }
            true
        }
        enums::Event::Released => {
            let event = &app::event_coords();
            //let  ke = &app::event_key();
            if mouse_drag.is_some() {
                mouse_drag = None;
            } else if app::event_key_down(enums::Key::from_char('L'))
                || app::event_key_down(enums::Key::from_char('S'))
            {
                let mut shared_data_bm = shared_data_c.borrow_mut();
                let point = shared_data_bm
                    .visualizer
                    .affine
                    .reverse_transform(app::event_x() as f64, app::event_y() as f64);
                if point.is_err() {
                    println!("{:?}", point.err().unwrap());
                    return false;
                }
                let point = Point::from(point.unwrap());
                if let Some(last_point) = shared_data_bm.last_click {
                    let line = Line {
                        start: last_point,
                        end: point,
                    };

                    if !shared_data_bm.visualizer.self_intersecting_check(&line) {
                        shared_data_bm.visualizer.segment_data_.push(line);

                        let _ = shared_data_bm.visualizer.build();

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
                    println!("mouse at {:?}", event);
                    let mut shared_data_bm = shared_data_c.borrow_mut();
                    {
                        let point = shared_data_bm
                            .visualizer
                            .affine
                            .reverse_transform(app::event_x() as f64, app::event_y() as f64);
                        if point.is_err() {
                            println!("{:?}", point.err().unwrap());
                            return false;
                        }
                        let point = Point::from(point.unwrap());
                        shared_data_bm.visualizer.point_data_.push(point);
                    }
                    let _ = shared_data_bm.visualizer.build();

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
                    let _ = shared_data_bm.visualizer.build();
                    let _ = shared_data_bm.visualizer.re_calculate_affine();
                    app::redraw();
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
pub struct VoronoiVisualizer<I1, F1>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
{
    screen_aabb: VU::Aabb2<I1, F1>,
    diagram: VD::VoronoiDiagram<I1, F1>,
    points_aabb: VU::Aabb2<I1, F1>,

    point_data_: Vec<boostvoronoi::Point<I1>>,
    segment_data_: Vec<boostvoronoi::Line<I1>>,
    affine: VU::SimpleAffine<I1, F1>,
}

impl<I1, F1> VoronoiVisualizer<I1, F1>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
{
    pub fn default() -> Self {
        Self {
            screen_aabb: VU::Aabb2::<I1, F1>::new_from_i32(0, 0, FW, FH),
            diagram: VD::VoronoiDiagram::<I1, F1>::new(0),
            points_aabb: VU::Aabb2::<I1, F1>::default(),
            point_data_: Vec::<boostvoronoi::Point<I1>>::new(),
            segment_data_: Vec::<boostvoronoi::Line<I1>>::new(),
            affine: VU::SimpleAffine::default(),
        }
    }

    /// recalculates the affine transformation, this should not be done every time
    /// the diagram is re-calculated or the screen will move around when adding new edges and points.
    pub fn re_calculate_affine(&mut self) -> Result<(), BvError> {
        self.affine = VU::SimpleAffine::new(&self.points_aabb, &self.screen_aabb)?;

        // Flip the z axis because fltk uses quadrant 4 when drawing
        self.affine.scale[1] *= F1::from(-1.0).unwrap();
        Ok(())
    }

    pub fn build(&mut self) -> Result<String, BvError> {
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

        let mut vb = VB::Builder::<I1, F1>::default();
        vb.with_vertices(self.point_data_.iter())?;
        vb.with_segments(self.segment_data_.iter())?;

        // Construct voronoi diagram.
        self.diagram = vb.construct()?;
        self.points_aabb = {
            let mut aabb = VU::Aabb2::default();
            for p in self.point_data_.iter() {
                aabb.update_point(p);
            }
            for l in self.segment_data_.iter() {
                aabb.update_line(l);
            }
            aabb.grow_percent(20);
            aabb
        };

        // Color exterior edges.
        self.diagram.color_exterior_edges(ColorFlag::EXTERNAL.bits);

        // Color infinite edges
        for it in self.diagram.edges().iter() {
            let edge_id = Some(it.get().get_id());
            if !self.diagram.edge_is_finite(edge_id).unwrap() {
                self.diagram
                    .edge_or_color(edge_id, ColorFlag::INFINITE.bits);
            }
        }

        // Color edges and vertices based upon how their cell is created, by a segment or a point
        for it in self.diagram.cells().iter() {
            let is_segment = it.get().contains_segment();
            let edge_id_o = it.get().get_incident_edge();
            if let Some(edge_id) = edge_id_o {
                let flag = if is_segment {
                    ColorFlag::CELL_SEGMENT.bits
                } else {
                    ColorFlag::CELL_POINT.bits
                };
                self.diagram.edge_or_color(edge_id_o, flag);
                self.diagram
                    .vertex_or_color(self.diagram.edge_get_vertex0(edge_id_o), flag);
                self.diagram
                    .vertex_or_color(self.diagram.edge_get_vertex1(edge_id_o), flag);

                let mut another_edge = self.diagram.get_edge(edge_id).get().next();
                while another_edge.is_some() && another_edge != edge_id_o {
                    self.diagram.edge_or_color(another_edge, flag);
                    self.diagram
                        .vertex_or_color(self.diagram.edge_get_vertex0(another_edge), flag);
                    self.diagram
                        .vertex_or_color(self.diagram.edge_get_vertex1(another_edge), flag);

                    another_edge = self.diagram.get_edge(another_edge.unwrap()).get().next();
                }
            }
        }

        Result::Ok("".to_string())
    }

    // returns true if l intersects with any of the lines in self.segment_data_
    fn self_intersecting_check(&self, l: &boostvoronoi::Line<I1>) -> bool {
        let l_ = Self::line_i1_to_f64(l);
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

            let s_ = Self::line_i1_to_f64(s);
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
            self.draw_edges(&config, &self.affine);
        }
        if config.draw_flag.contains(DrawFilterFlag::VERTICES) {
            draw::set_draw_color(enums::Color::Blue);
            self.draw_vertices(&config, &self.affine);
        }
        Ok(())
    }

    #[allow(dead_code)]
    /// Draw bounding box.
    fn draw_bb(&self) {
        let min_x = Self::f1_to_i32(self.screen_aabb.get_low().unwrap()[0]);
        let max_x = Self::f1_to_i32(self.screen_aabb.get_high().unwrap()[0]);
        let min_y = Self::f1_to_i32(self.screen_aabb.get_low().unwrap()[1]);
        let max_y = Self::f1_to_i32(self.screen_aabb.get_high().unwrap()[1]);

        draw::draw_line(min_x, min_y, max_x, min_y);
        draw::draw_line(min_x, max_y, max_x, max_y);
        draw::draw_line(min_x, min_y, min_x, max_y);
        draw::draw_line(max_x, min_y, max_x, max_y);
    }

    /// Draw input points and endpoints of the input segments.
    fn draw_input_points(&self, affine: &VU::SimpleAffine<I1, F1>) {
        let draw = |point: [F1; 2]| {
            draw::draw_circle(Self::f1_to_f64(point[0]), Self::f1_to_f64(point[1]), 2.0);
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
    fn draw_input_segments(&self, affine: &VU::SimpleAffine<I1, F1>) {
        for i in self.segment_data_.iter() {
            let sp = affine.transform_p(&i.start);
            let ep = affine.transform_p(&i.end);
            draw::draw_line(
                Self::f1_to_i32(sp[0]),
                Self::f1_to_i32(sp[1]),
                Self::f1_to_i32(ep[0]),
                Self::f1_to_i32(ep[1]),
            );
        }
    }

    /// Draw voronoi vertices aka circle events.
    fn draw_vertices(&self, config: &SharedData, affine: &VU::SimpleAffine<I1, F1>) {
        let draw = |x: f64, y: f64| {
            draw::draw_circle(x, y, 1.0);
        };
        let draw_external = config.draw_flag.contains(DrawFilterFlag::EXTERNAL);
        let draw_cell_points = config.draw_flag.contains(DrawFilterFlag::V_CELL_POINT);
        let draw_cell_segment = config.draw_flag.contains(DrawFilterFlag::V_CELL_SEGMENT);
        let draw_site_vertex = config.draw_flag.contains(DrawFilterFlag::SITE_VERTEX);

        for it in self.diagram.vertex_iter().enumerate() {
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
            if (!draw_cell_points)
                && ColorFlag::from_bits(vertex.get_color())
                    .unwrap()
                    .contains(ColorFlag::CELL_POINT)
            {
                continue;
            }
            if (!draw_cell_segment)
                && ColorFlag::from_bits(vertex.get_color())
                    .unwrap()
                    .contains(ColorFlag::CELL_SEGMENT)
            {
                continue;
            }

            draw(
                Self::f1_to_f64(affine.transform_x(vertex.x())),
                Self::f1_to_f64(affine.transform_y(vertex.y())),
            );
        }
    }

    /// Draw voronoi edges.
    fn draw_edges(&self, config: &SharedData, affine: &VU::SimpleAffine<I1, F1>) {
        let draw_external = config.draw_flag.contains(DrawFilterFlag::EXTERNAL);
        let draw_primary = config.draw_flag.contains(DrawFilterFlag::PRIMARY);
        let draw_secondary = config.draw_flag.contains(DrawFilterFlag::SECONDARY);
        let draw_curved = config.draw_flag.contains(DrawFilterFlag::CURVE);
        let draw_curved_as_line = config.draw_flag.contains(DrawFilterFlag::CURVE_LINE);
        let draw_cell_segment = config.draw_flag.contains(DrawFilterFlag::E_CELL_SEGMENT);
        let draw_cell_point = config.draw_flag.contains(DrawFilterFlag::E_CELL_POINT);
        let draw_infinite_edges = config.draw_flag.contains(DrawFilterFlag::INFINITE);

        let mut already_drawn = yabf::Yabf::default();

        for it in self.diagram.edges().iter().enumerate() {
            draw::set_draw_color(enums::Color::DarkGreen);
            let edge_id = VoronoiEdgeIndex(it.0);
            let edge = it.1.get();
            if already_drawn.bit(edge_id.0) {
                // already done this, or rather - it's twin
                continue;
            }
            // no point in setting current edge as drawn, the edge id will not repeat
            // already_drawn.set_bit(edge_id.0, true);
            if let Some(twin) = self.diagram.edge_get_twin(Some(edge_id)) {
                already_drawn.set_bit(twin.0, true);
            }

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
            if (!draw_cell_point)
                && ColorFlag::from_bits(edge.get_color())
                    .unwrap()
                    .contains(ColorFlag::CELL_POINT)
            {
                continue;
            }
            if (!draw_cell_segment)
                && ColorFlag::from_bits(edge.get_color())
                    .unwrap()
                    .contains(ColorFlag::CELL_SEGMENT)
            {
                continue;
            }

            // the coordinates in samples must be 'screen' coordinates, i.e. affine transformed
            let mut samples = Vec::<[F1; 2]>::new();
            if !self.diagram.edge_is_finite(Some(edge_id)).unwrap() {
                let a = self.clip_infinite_edge(&affine, edge_id, &mut samples);
                if let Err(err) = a {
                    println!("Ignoring error : {:?}", err);
                }
            } else {
                let vertex0 = self.diagram.vertex_get(edge.vertex0()).unwrap().get();

                samples.push(affine.transform(vertex0.x(), vertex0.y()));
                let vertex1 = self.diagram.edge_get_vertex1(Some(edge_id));
                let vertex1 = self.diagram.vertex_get(vertex1).unwrap().get();

                samples.push(affine.transform(vertex1.x(), vertex1.y()));
                if edge.is_curved() {
                    if draw_curved_as_line {
                        for i in 0..samples.len() - 1 {
                            if let Ok(x1) = Self::try_f1_to_i32(samples[i][0]) {
                                if let Ok(y1) = Self::try_f1_to_i32(samples[i][1]) {
                                    if let Ok(x2) = Self::try_f1_to_i32(samples[i + 1][0]) {
                                        if let Ok(y2) = Self::try_f1_to_i32(samples[i + 1][1]) {
                                            draw::draw_line(x1, y1, x2, y2);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if draw_curved {
                        self.sample_curved_edge(&affine, VoronoiEdgeIndex(it.0), &mut samples);
                    } else {
                        continue;
                    }
                }
            }
            if samples.len() > 1 {
                for i in 0..samples.len() - 1 {
                    let x1 = Self::try_f1_to_i32(samples[i][0]);
                    if x1.is_err() {
                        break;
                    }
                    let y1 = Self::try_f1_to_i32(samples[i][1]);
                    if y1.is_err() {
                        break;
                    }
                    let x2 = Self::try_f1_to_i32(samples[i + 1][0]);
                    if x2.is_err() {
                        break;
                    }
                    let y2 = Self::try_f1_to_i32(samples[i + 1][1]);
                    if y2.is_err() {
                        break;
                    }

                    draw::draw_line(x1.unwrap(), y1.unwrap(), x2.unwrap(), y2.unwrap());
                }
            }
        }
    }

    fn clip_infinite_edge(
        &self,
        affine: &VU::SimpleAffine<I1, F1>,
        edge_id: VD::VoronoiEdgeIndex,
        clipped_edge: &mut Vec<[F1; 2]>,
    ) -> Result<(), BvError> {
        let edge = self.diagram.get_edge(edge_id);
        //const cell_type& cell1 = *edge.cell();
        let cell1_id = self.diagram.edge_get_cell(Some(edge_id)).unwrap();
        let cell1 = self.diagram.get_cell(cell1_id).get();
        //const cell_type& cell2 = *edge.twin()->cell();
        let cell2_id = self
            .diagram
            .edge_get_twin(Some(edge_id))
            .and_then(|e| self.diagram.edge_get_cell(Some(e)))
            .unwrap();
        let cell2 = self.diagram.get_cell(cell2_id).get();

        let mut origin = [F1::default(), F1::default()];
        let mut direction = [F1::default(), F1::default()];
        // Infinite edges could not be created by two segment sites.
        if cell1.contains_point() && cell2.contains_point() {
            let p1 = self.retrieve_point(cell1_id);
            let p2 = self.retrieve_point(cell2_id);
            origin[0] = (Self::i1_to_f1(p1.x) + Self::i1_to_f1(p2.x)) * Self::f64_to_f1(0.5);
            origin[1] = (Self::i1_to_f1(p1.y) + Self::i1_to_f1(p2.y)) * Self::f64_to_f1(0.5);
            direction[0] = Self::i1_to_f1(p1.y) - Self::i1_to_f1(p2.y);
            direction[1] = Self::i1_to_f1(p2.x) - Self::i1_to_f1(p1.x);
        } else {
            origin = if cell1.contains_segment() {
                let p = self.retrieve_point(cell2_id);
                [Self::i1_to_f1(p.x), Self::i1_to_f1(p.y)]
            } else {
                let p = self.retrieve_point(cell1_id);
                [Self::i1_to_f1(p.x), Self::i1_to_f1(p.y)]
            };
            let segment = if cell1.contains_segment() {
                self.retrieve_segment(cell1_id)
            } else {
                self.retrieve_segment(cell2_id)
            };
            let dx = segment.end.x - segment.start.x;
            let dy = segment.end.y - segment.start.y;
            if ([
                Self::i1_to_f1(segment.start.x),
                Self::i1_to_f1(segment.start.y),
            ] == origin)
                ^ cell1.contains_point()
            {
                direction[0] = Self::i1_to_f1(dy);
                direction[1] = Self::i1_to_f1(-dx);
            } else {
                direction[0] = Self::i1_to_f1(-dy);
                direction[1] = Self::i1_to_f1(dx);
            }
        }

        let side =
            Self::i1_to_f1(affine.reverse_transform_x(self.screen_aabb.get_high().unwrap()[0])?)
                - Self::i1_to_f1(
                    affine.reverse_transform_x(self.screen_aabb.get_low().unwrap()[0])?,
                );
        // absolute value is taken in case the affine transform flips one coordinate
        let side = side.abs();
        let koef = side / Self::max_f1(direction[0].abs(), direction[1].abs());

        let vertex0 = edge.get().vertex0();
        if vertex0.is_none() {
            clipped_edge.push([
                affine.transform_x(origin[0] - direction[0] * koef),
                affine.transform_y(origin[1] - direction[1] * koef),
            ]);
        } else {
            let vertex0 = self.diagram.vertex_get(vertex0).unwrap().get();
            clipped_edge.push([
                affine.transform_x(vertex0.x()),
                affine.transform_y(vertex0.y()),
            ]);
        }
        let vertex1 = self.diagram.edge_get_vertex1(Some(edge_id));
        if vertex1.is_none() {
            clipped_edge.push([
                affine.transform_x(origin[0] + direction[0] * koef),
                affine.transform_y(origin[1] + direction[1] * koef),
            ]);
        } else {
            let vertex1 = self.diagram.vertex_get(vertex1).unwrap().get();
            clipped_edge.push([
                affine.transform_x(vertex1.x()),
                affine.transform_y(vertex1.y()),
            ]);
        }
        Ok(())
    }

    /// Important: sampled_edge should contain both edge endpoints initially.
    /// sampled_edge should be 'screen' coordinates, i.e. affine transformed from voronoi output
    fn sample_curved_edge(
        &self,
        affine: &VU::SimpleAffine<I1, F1>,
        edge_id: VD::VoronoiEdgeIndex,
        sampled_edge: &mut Vec<[F1; 2]>,
    ) {
        let max_dist = Self::f64_to_f1(1E-3)
            * (self.screen_aabb.get_high().unwrap()[0] - self.screen_aabb.get_low().unwrap()[0]);

        let cell_id = self.diagram.edge_get_cell(Some(edge_id)).unwrap();
        let cell = self.diagram.get_cell(cell_id).get();
        let twin_id = self.diagram.edge_get_twin(Some(edge_id)).unwrap();
        let twin_cell_id = self.diagram.edge_get_cell(Some(twin_id)).unwrap();

        let point = if cell.contains_point() {
            self.retrieve_point(cell_id)
        } else {
            self.retrieve_point(twin_cell_id)
        };
        let segment = if cell.contains_point() {
            self.retrieve_segment(twin_cell_id)
        } else {
            self.retrieve_segment(cell_id)
        };
        VU::VoronoiVisualUtils::<I1, F1>::discretize(
            &point,
            segment,
            max_dist,
            affine,
            sampled_edge,
        );
    }

    /// Retrieves a point from the voronoi input in the order it was presented to
    /// the voronoi builder
    fn retrieve_point(&self, cell_id: VD::VoronoiCellIndex) -> boostvoronoi::Point<I1> {
        let (index, cat) = self.diagram.get_cell(cell_id).get().source_index_2();
        match cat {
            VD::SourceCategory::SinglePoint => self.point_data_[index],
            VD::SourceCategory::SegmentStart => {
                self.segment_data_[index - self.point_data_.len()].start
            }
            VD::SourceCategory::Segment | VD::SourceCategory::SegmentEnd => {
                self.segment_data_[index - self.point_data_.len()].end
            }
        }
    }

    /// Retrieves a segment from the voronoi input in the order it was presented to
    /// the voronoi builder
    fn retrieve_segment(&self, cell_id: VD::VoronoiCellIndex) -> &boostvoronoi::Line<I1> {
        let cell = self.diagram.get_cell(cell_id).get();
        let index = cell.source_index() - self.point_data_.len();
        &self.segment_data_[index]
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

        let _segments_rust_logo: [[i32; 4]; 352] = [
            [402, 1580, 395, 1580],
            [408, 1577, 402, 1580],
            [476, 1573, 469, 1574],
            [328, 1574, 322, 1572],
            [335, 1571, 328, 1574],
            [481, 1567, 476, 1573],
            [322, 1572, 318, 1567],
            [264, 1551, 257, 1553],
            [548, 1550, 540, 1553],
            [257, 1553, 250, 1549],
            [552, 1544, 548, 1550],
            [395, 1580, 370, 1543],
            [429, 1543, 408, 1577],
            [362, 1542, 335, 1571],
            [469, 1574, 438, 1542],
            [370, 1543, 362, 1542],
            [438, 1542, 429, 1543],
            [495, 1531, 481, 1567],
            [318, 1567, 305, 1531],
            [295, 1529, 264, 1551],
            [540, 1553, 504, 1529],
            [504, 1529, 495, 1531],
            [305, 1531, 295, 1529],
            [198, 1518, 191, 1518],
            [613, 1515, 607, 1519],
            [191, 1518, 185, 1513],
            [393, 1513, 404, 1513],
            [382, 1507, 393, 1513],
            [616, 1508, 613, 1515],
            [404, 1513, 415, 1506],
            [558, 1506, 552, 1544],
            [250, 1549, 241, 1506],
            [241, 1506, 233, 1502],
            [566, 1502, 558, 1506],
            [233, 1502, 198, 1518],
            [607, 1519, 566, 1502],
            [377, 1497, 382, 1507],
            [415, 1506, 420, 1496],
            [615, 1493, 616, 1508],
            [377, 1485, 377, 1497],
            [420, 1496, 420, 1485],
            [615, 1480, 615, 1493],
            [382, 1475, 377, 1485],
            [420, 1485, 414, 1475],
            [451, 1476, 471, 1472],
            [318, 1469, 348, 1476],
            [140, 1472, 133, 1471],
            [667, 1470, 660, 1472],
            [185, 1513, 185, 1469],
            [614, 1469, 615, 1480],
            [392, 1469, 382, 1475],
            [414, 1475, 403, 1469],
            [403, 1469, 392, 1469],
            [670, 1466, 667, 1470],
            [133, 1471, 128, 1466],
            [471, 1472, 510, 1459],
            [185, 1469, 177, 1464],
            [622, 1464, 614, 1469],
            [289, 1458, 318, 1469],
            [177, 1464, 140, 1472],
            [660, 1472, 622, 1464],
            [671, 1460, 670, 1466],
            [262, 1445, 289, 1458],
            [510, 1459, 545, 1440],
            [348, 1476, 384, 1438],
            [237, 1428, 262, 1445],
            [405, 1435, 451, 1476],
            [384, 1438, 395, 1433],
            [395, 1433, 405, 1435],
            [545, 1440, 577, 1417],
            [128, 1466, 136, 1423],
            [663, 1423, 671, 1460],
            [213, 1409, 237, 1428],
            [668, 1415, 663, 1423],
            [136, 1423, 131, 1415],
            [131, 1415, 92, 1417],
            [707, 1417, 668, 1415],
            [713, 1414, 707, 1417],
            [92, 1417, 85, 1414],
            [717, 1409, 713, 1414],
            [85, 1414, 82, 1409],
            [577, 1417, 606, 1390],
            [717, 1402, 717, 1409],
            [82, 1409, 82, 1402],
            [192, 1389, 213, 1409],
            [490, 1389, 192, 1389],
            [501, 1387, 490, 1389],
            [606, 1390, 630, 1358],
            [701, 1367, 717, 1402],
            [82, 1402, 98, 1367],
            [529, 1378, 501, 1387],
            [705, 1359, 701, 1367],
            [98, 1367, 94, 1359],
            [630, 1358, 641, 1341],
            [94, 1359, 56, 1352],
            [743, 1352, 705, 1359],
            [56, 1352, 50, 1350],
            [749, 1350, 743, 1352],
            [50, 1350, 47, 1343],
            [752, 1342, 749, 1350],
            [47, 1343, 49, 1336],
            [750, 1336, 752, 1342],
            [584, 1329, 556, 1362],
            [119, 1307, 132, 1309],
            [666, 1308, 677, 1306],
            [132, 1309, 142, 1305],
            [728, 1305, 750, 1336],
            [49, 1336, 71, 1305],
            [654, 1304, 666, 1308],
            [728, 1304, 728, 1305],
            [172, 1304, 211, 1304],
            [590, 1295, 584, 1329],
            [211, 1304, 211, 1126],
            [641, 1341, 622, 1296],
            [173, 1301, 172, 1304],
            [337, 1302, 337, 1250],
            [337, 1302, 432, 1302],
            [432, 1302, 435, 1302],
            [111, 1299, 119, 1307],
            [677, 1306, 686, 1298],
            [71, 1305, 69, 1299],
            [69, 1299, 69, 1298],
            [150, 1295, 142, 1305],
            [647, 1295, 654, 1304],
            [69, 1298, 69, 1295],
            [730, 1295, 728, 1304],
            [435, 1302, 457, 1292],
            [106, 1289, 111, 1299],
            [686, 1298, 690, 1288],
            [150, 1295, 152, 1284],
            [646, 1283, 647, 1295],
            [69, 1295, 33, 1282],
            [767, 1282, 730, 1295],
            [622, 1296, 613, 1278],
            [108, 1277, 106, 1289],
            [457, 1292, 465, 1275],
            [690, 1288, 688, 1276],
            [33, 1282, 27, 1277],
            [772, 1276, 767, 1282],
            [184, 1274, 173, 1301],
            [152, 1284, 147, 1274],
            [649, 1272, 646, 1283],
            [773, 1271, 772, 1276],
            [27, 1277, 26, 1270],
            [582, 1263, 590, 1295],
            [115, 1268, 108, 1277],
            [613, 1278, 613, 1267],
            [688, 1276, 681, 1267],
            [147, 1274, 138, 1266],
            [659, 1265, 649, 1272],
            [770, 1265, 773, 1271],
            [181, 1263, 184, 1274],
            [126, 1264, 115, 1268],
            [138, 1266, 126, 1264],
            [681, 1267, 670, 1263],
            [465, 1275, 458, 1260],
            [670, 1263, 659, 1265],
            [613, 1267, 618, 1258],
            [174, 1256, 181, 1263],
            [123, 1233, 174, 1256],
            [458, 1260, 438, 1252],
            [438, 1252, 428, 1250],
            [428, 1250, 337, 1250],
            [26, 1270, 58, 1238],
            [742, 1238, 770, 1265],
            [618, 1258, 675, 1231],
            [742, 1231, 742, 1238],
            [548, 1226, 582, 1263],
            [675, 1231, 676, 1231],
            [742, 1230, 742, 1231],
            [58, 1238, 57, 1230],
            [533, 1217, 548, 1226],
            [121, 1213, 123, 1233],
            [57, 1230, 23, 1209],
            [742, 1230, 776, 1209],
            [550, 1203, 533, 1217],
            [780, 1204, 776, 1209],
            [23, 1209, 20, 1203],
            [558, 1195, 550, 1203],
            [780, 1198, 780, 1204],
            [20, 1203, 20, 1196],
            [562, 1190, 558, 1195],
            [776, 1192, 780, 1198],
            [565, 1184, 562, 1190],
            [676, 1231, 677, 1182],
            [569, 1178, 565, 1184],
            [123, 1177, 121, 1213],
            [677, 1182, 647, 1182],
            [647, 1182, 645, 1180],
            [337, 1175, 411, 1175],
            [337, 1124, 337, 1175],
            [411, 1175, 417, 1174],
            [742, 1171, 776, 1192],
            [20, 1196, 57, 1171],
            [574, 1162, 569, 1178],
            [417, 1174, 434, 1166],
            [645, 1180, 644, 1158],
            [742, 1162, 742, 1171],
            [57, 1171, 58, 1162],
            [126, 1151, 123, 1177],
            [434, 1166, 450, 1144],
            [450, 1144, 453, 1137],
            [644, 1158, 637, 1137],
            [580, 1138, 574, 1162],
            [58, 1162, 29, 1136],
            [770, 1136, 742, 1162],
            [132, 1126, 126, 1151],
            [588, 1130, 580, 1138],
            [773, 1129, 770, 1136],
            [29, 1136, 26, 1129],
            [637, 1137, 623, 1127],
            [211, 1126, 132, 1126],
            [605, 1125, 588, 1130],
            [623, 1127, 605, 1125],
            [406, 1124, 337, 1124],
            [771, 1123, 773, 1129],
            [26, 1129, 28, 1122],
            [407, 1122, 406, 1124],
            [766, 1119, 771, 1123],
            [730, 1105, 766, 1119],
            [28, 1122, 69, 1105],
            [728, 1096, 730, 1105],
            [69, 1105, 71, 1096],
            [453, 1137, 464, 1085],
            [464, 1085, 467, 1073],
            [71, 1096, 49, 1065],
            [752, 1060, 728, 1096],
            [49, 1065, 47, 1059],
            [47, 1059, 50, 1052],
            [749, 1052, 752, 1060],
            [467, 1073, 485, 1048],
            [50, 1052, 56, 1048],
            [743, 1048, 749, 1052],
            [705, 1042, 743, 1048],
            [56, 1048, 94, 1042],
            [407, 1040, 407, 1122],
            [485, 1048, 502, 1038],
            [407, 1039, 407, 1040],
            [502, 1038, 506, 1038],
            [405, 1038, 407, 1039],
            [621, 1038, 624, 1038],
            [175, 1038, 405, 1038],
            [506, 1038, 621, 1038],
            [94, 1042, 98, 1034],
            [701, 1034, 705, 1042],
            [255, 1019, 200, 1010],
            [548, 1019, 537, 1017],
            [711, 1012, 701, 1034],
            [265, 1015, 255, 1019],
            [624, 1038, 600, 1010],
            [537, 1017, 529, 1010],
            [200, 1010, 175, 1038],
            [600, 1010, 598, 1008],
            [270, 1007, 265, 1015],
            [598, 1008, 548, 1019],
            [98, 1034, 82, 999],
            [717, 999, 711, 1012],
            [717, 991, 717, 999],
            [82, 999, 82, 991],
            [227, 991, 238, 990],
            [558, 989, 571, 990],
            [714, 987, 717, 991],
            [82, 991, 86, 987],
            [216, 985, 227, 991],
            [571, 990, 581, 984],
            [238, 990, 248, 983],
            [86, 987, 92, 984],
            [707, 984, 714, 987],
            [92, 984, 131, 986],
            [668, 986, 707, 984],
            [549, 982, 558, 989],
            [666, 983, 668, 986],
            [131, 986, 136, 978],
            [663, 978, 666, 983],
            [210, 975, 216, 985],
            [581, 984, 587, 974],
            [248, 983, 253, 972],
            [544, 972, 549, 982],
            [209, 963, 210, 975],
            [587, 974, 588, 962],
            [281, 959, 270, 1007],
            [253, 972, 252, 961],
            [544, 960, 544, 972],
            [214, 953, 209, 963],
            [588, 962, 582, 952],
            [252, 961, 247, 952],
            [551, 951, 544, 960],
            [283, 951, 281, 959],
            [291, 947, 283, 951],
            [529, 1010, 514, 950],
            [224, 947, 214, 953],
            [236, 946, 247, 952],
            [582, 952, 572, 946],
            [561, 945, 551, 951],
            [236, 946, 224, 947],
            [572, 946, 561, 945],
            [136, 978, 128, 941],
            [514, 950, 478, 936],
            [329, 934, 291, 947],
            [671, 935, 663, 978],
            [128, 941, 129, 934],
            [614, 932, 622, 937],
            [177, 937, 185, 932],
            [666, 930, 671, 935],
            [129, 934, 134, 929],
            [478, 936, 439, 928],
            [622, 937, 659, 929],
            [659, 929, 666, 930],
            [134, 929, 177, 937],
            [368, 927, 329, 934],
            [439, 928, 419, 926],
            [389, 925, 368, 927],
            [419, 926, 389, 925],
            [558, 895, 566, 899],
            [233, 899, 241, 895],
            [185, 932, 183, 893],
            [614, 888, 614, 932],
            [183, 893, 186, 887],
            [198, 883, 233, 899],
            [566, 899, 601, 883],
            [609, 883, 614, 888],
            [186, 887, 191, 882],
            [191, 882, 198, 883],
            [601, 883, 609, 883],
            [495, 870, 504, 872],
            [295, 872, 305, 870],
            [241, 895, 248, 857],
            [552, 857, 558, 895],
            [362, 858, 370, 858],
            [429, 858, 438, 858],
            [549, 851, 552, 857],
            [248, 857, 251, 851],
            [504, 872, 535, 850],
            [542, 848, 549, 851],
            [251, 851, 259, 848],
            [259, 848, 295, 872],
            [535, 850, 542, 848],
            [305, 870, 318, 834],
            [481, 834, 495, 870],
            [438, 858, 464, 830],
            [334, 829, 362, 858],
            [318, 834, 322, 829],
            [476, 828, 481, 834],
            [464, 830, 470, 827],
            [322, 829, 327, 827],
            [327, 827, 334, 829],
            [470, 827, 476, 828],
            [370, 858, 391, 824],
            [404, 821, 429, 858],
            [391, 824, 397, 820],
            [397, 820, 404, 821],
            [556, 1362, 529, 1378],
        ];

        // Preparing Input Geometries.
        let (mut new_points, mut new_segments) = match example {
            Example::Simple => {
                rv = "Simple example".to_string();
                (
                    Vec::<Point<I1>>::default(),
                    VB::to_segments::<i32, I1>(&_simple_segments),
                )
            }
            Example::Complex => {
                rv = "Rust logo".to_string();
                (
                    Vec::<Point<I1>>::default(),
                    VB::to_segments::<i32, I1>(&_segments_rust_logo),
                )
            }
            Example::Clean => {
                rv = "Clean".to_string();
                let clean: [[i32; 4]; 0] = [];
                (
                    Vec::<Point<I1>>::default(),
                    VB::to_segments::<i32, I1>(&clean),
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
                        file_reader::read_boost_input_file::<I1>(filename.as_path())
                    {
                        rv = filename.to_str().unwrap().to_string();
                        file_parse_result
                    } else {
                        rv = "Failed to read file".to_string();
                        (Vec::<Point<I1>>::default(), Vec::<Line<I1>>::default())
                    }
                } else {
                    rv = "Failed to read file".to_string();
                    (Vec::<Point<I1>>::default(), Vec::<Line<I1>>::default())
                }
            }
        };
        self.point_data_.append(&mut new_points);
        self.segment_data_.append(&mut new_segments);
        rv
    }

    fn line_i1_to_f64(value: &boostvoronoi::Line<I1>) -> geo::Line<f64> {
        let ps = geo::Coordinate {
            x: Self::i1_to_f64(value.start.x),
            y: Self::i1_to_f64(value.start.y),
        };
        let pe = geo::Coordinate {
            x: Self::i1_to_f64(value.end.x),
            y: Self::i1_to_f64(value.end.y),
        };
        geo::Line::<f64>::new(ps, pe)
    }

    #[inline(always)]
    fn max_f1(a: F1, b: F1) -> F1 {
        OrderedFloat(a).max(OrderedFloat(b)).into_inner()
    }

    #[inline(always)]
    pub fn i1_to_f1(value: I1) -> F1 {
        TypeConverter2::<I1, F1>::i1_to_f1(value)
    }
    #[inline(always)]
    pub fn f1_to_i32(value: F1) -> i32 {
        TypeConverter2::<I1, F1>::f1_to_i32(value)
    }

    #[inline(always)]
    pub fn f64_to_f1(value: f64) -> F1 {
        TypeConverter2::<I1, F1>::f64_to_f1(value)
    }

    #[inline(always)]
    pub fn f1_to_i1(value: F1) -> I1 {
        TypeConverter2::<I1, F1>::f1_to_i1(value)
    }

    #[inline(always)]
    pub fn i1_to_f64(value: I1) -> f64 {
        TypeConverter1::<I1>::i1_to_f64(value)
    }

    #[inline(always)]
    pub fn try_f1_to_i32(value: F1) -> Result<i32, BvError> {
        TypeConverter2::<I1, F1>::try_f1_to_i32(value)
    }

    #[inline(always)]
    pub fn f1_to_f64(v: F1) -> f64 {
        num::cast::<F1, f64>(v).unwrap()
    }

    #[inline(always)]
    pub fn f1_to_f64o(v: F1) -> OrderedFloat<f64> {
        OrderedFloat(num::cast::<F1, f64>(v).unwrap())
    }
}
