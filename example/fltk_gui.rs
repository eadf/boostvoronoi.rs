use boostvoronoi::diagram as VD;
use boostvoronoi::diagram::VoronoiEdgeIndex;
use boostvoronoi::visual_utils as VU;
use boostvoronoi::BvError;
use boostvoronoi::{builder as VB, Line, Point};
use boostvoronoi::{BigFloatType, BigIntType, InputType, OutputType};
use boostvoronoi::{TypeConverter2, TypeConverter4};

use std::ops::Neg;

use fltk::app::event_key_down;
use fltk::app::{event_x, event_y, redraw};
use fltk::button::RoundButton;
use fltk::enums::Key;
use fltk::menu::MenuButton;
use fltk::*;
use fltk::{app, draw::*, frame::*};

use fltk::group::Pack;
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
    }
}

bitflags! {
    pub struct DrawFilterFlag: u32 {
        /// Edges considered to be outside all closed input geometry
        const EXTERNAL =      0b00000000000001;
        const PRIMARY =       0b00000000000010;
        const CURVE =         0b00000000000100;
        const VERTICES=       0b00000000001000;
        /// All edges
        const EDGES=          0b00000000010000;
        const SECONDARY =     0b00000000100000;
        /// Input geometry points
        const INPUT_POINT =   0b00000001000000;
        /// Input geometry segments
        const INPUT_SEGMENT = 0b00000010000000;
        /// Edge belonging to cells defined by a segment
        const E_CELL_SEGMENT= 0b00000100000000;
        /// Edge belonging to cells defined by a point
        const E_CELL_POINT =  0b00001000000000;
        /// Vertices belonging to cells defined by a segment
        const V_CELL_SEGMENT= 0b00010000000000;
        /// Vertices belonging to cells defined by a point
        const V_CELL_POINT =  0b00100000000000;
        /// Draw infinite edges
        const INFINITE =      0b01000000000000;
        /// Draw curves as straight lines
        const CURVE_LINE =    0b10000000000000;
        const DRAW_ALL =      0b11111111111111;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Example {
    Simple,
    Complex,
    Atest,
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
    visualizer: VoronoiVisualizer<i32, f32, i64, f64>,
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

    let mut frame = Frame::new(5, 5, FW, FH, "");
    frame.set_color(Color::Black);
    frame.set_frame(FrameType::DownBox);

    let mut pack = Pack::new(5 + FW, 5, 170, WH, "");
    pack.set_spacing(5);

    let mut menu_but = MenuButton::default().with_size(170, 25).with_label("Menu");
    menu_but.set_frame(FrameType::PlasticUpBox);

    let mut input_points_button = RoundButton::default()
        .with_size(180, 25)
        .with_label("input points");
    input_points_button.toggle(true);
    input_points_button.set_frame(FrameType::PlasticUpBox);

    let mut input_segments_button = RoundButton::default()
        .with_size(180, 25)
        .with_label("input segments");
    input_segments_button.toggle(true);
    input_segments_button.set_frame(FrameType::PlasticUpBox);

    let mut external_button = RoundButton::default()
        .with_size(180, 25)
        .with_label("externals");
    external_button.toggle(true);
    external_button.set_frame(FrameType::PlasticUpBox);

    let mut infinite_button = RoundButton::default()
        .with_size(180, 25)
        .with_label("infinite edges");
    infinite_button.toggle(true);
    infinite_button.set_frame(FrameType::PlasticUpBox);

    let mut vertices_button = RoundButton::default()
        .with_size(180, 25)
        .with_label("vertices");
    vertices_button.toggle(true);
    vertices_button.set_frame(FrameType::PlasticUpBox);

    let mut edges_button = RoundButton::default()
        .with_size(180, 25)
        .with_label("edges");
    edges_button.toggle(true);
    edges_button.set_frame(FrameType::PlasticUpBox);

    let mut curved_button = RoundButton::default()
        .with_size(180, 25)
        .with_label("arc edges");
    curved_button.toggle(true);
    curved_button.set_frame(FrameType::PlasticUpBox);

    let mut curved_as_lines_button = RoundButton::default()
        .with_size(180, 25)
        .with_label("arc as lines");
    curved_as_lines_button.toggle(false);
    curved_as_lines_button.set_frame(FrameType::PlasticUpBox);

    let mut primary_button = RoundButton::default()
        .with_size(180, 25)
        .with_label("primary edges");
    primary_button.toggle(true);
    primary_button.set_frame(FrameType::PlasticUpBox);

    let mut secondary_button = RoundButton::default()
        .with_size(180, 25)
        .with_label("secondary edges");
    secondary_button.toggle(true);
    secondary_button.set_frame(FrameType::PlasticUpBox);

    let mut e_segment_cell_button = RoundButton::default()
        .with_size(180, 25)
        .with_label("cell segment edges");
    e_segment_cell_button.toggle(true);
    e_segment_cell_button.set_frame(FrameType::PlasticUpBox);

    let mut e_point_cell_button = RoundButton::default()
        .with_size(180, 25)
        .with_label("cell point edges");
    e_point_cell_button.toggle(true);
    e_point_cell_button.set_frame(FrameType::PlasticUpBox);

    let mut v_segment_cell_button = RoundButton::default()
        .with_size(180, 25)
        .with_label("cell segment vertices");
    v_segment_cell_button.toggle(true);
    v_segment_cell_button.set_frame(FrameType::PlasticUpBox);

    let mut v_point_cell_button = RoundButton::default()
        .with_size(180, 25)
        .with_label("cell point vertices");
    v_point_cell_button.toggle(true);
    v_point_cell_button.set_frame(FrameType::PlasticUpBox);

    pack.end();

    wind.set_color(Color::White);
    wind.end();
    wind.show();
    let offs = Offscreen::new(frame.width(), frame.height()).unwrap();
    #[cfg(not(target_os = "macos"))]
    {
        offs.begin();
        set_draw_color(Color::White);
        draw_rectf(0, 0, FW, FH);
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
        "Simple",
        Shortcut::None,
        menu::MenuFlag::Normal,
        sender,
        GuiMessage::MenuChoice(Example::Simple),
    );
    menu_but.add_emit(
        "Complex",
        Shortcut::None,
        menu::MenuFlag::Normal,
        sender,
        GuiMessage::MenuChoice(Example::Complex),
    );
    menu_but.add_emit(
        "Clean",
        Shortcut::None,
        menu::MenuFlag::Normal,
        sender,
        GuiMessage::MenuChoice(Example::Clean),
    );
    menu_but.add_emit(
        "A Test",
        Shortcut::None,
        menu::MenuFlag::Normal,
        sender,
        GuiMessage::MenuChoice(Example::Atest),
    );

    e_segment_cell_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::E_CELL_SEGMENT));
    e_point_cell_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::E_CELL_POINT));
    v_segment_cell_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::V_CELL_SEGMENT));
    v_point_cell_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::V_CELL_POINT));
    external_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::EXTERNAL));
    infinite_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::INFINITE));
    primary_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::PRIMARY));
    secondary_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::SECONDARY));
    input_points_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::INPUT_POINT));
    input_segments_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::INPUT_SEGMENT));
    curved_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::CURVE));
    vertices_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::VERTICES));
    edges_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::EDGES));
    curved_as_lines_button.emit(sender, GuiMessage::Filter(DrawFilterFlag::CURVE_LINE));

    {
        // initialize visualizer
        let cl = Rc::clone(&shared_data_rc);
        let mut shared_data_bm = cl.borrow_mut();
        shared_data_bm.visualizer.read_data(Example::Simple);
        let _ = shared_data_bm.visualizer.build();
        shared_data_bm.visualizer.recalulate_affine()?;
    }

    let shared_data_c = Rc::clone(&shared_data_rc);
    // This is called whenever the window is drawn and redrawn
    wind.draw(move || {
        // todo, move the actual drawing away from draw() function, only keep the offscreen blit.
        offs_rc.borrow_mut().begin();
        let data_b = shared_data_c.borrow();
        set_draw_color(Color::White);
        draw_rectf(0, 0, FW, FH);
        let _ = data_b.visualizer.draw(&data_b);
        offs_rc.borrow_mut().end();

        if offs_rc.borrow().is_valid() {
            offs_rc.borrow().copy(5, 5, FW, FH, 0, 0);
        } else {
            // this will almost never be called
            let data_b = shared_data_c.borrow();
            offs_rc.borrow_mut().begin();
            set_draw_color(Color::Yellow);
            draw_rectf(5, 5, FW, FH);
            let _ = data_b.visualizer.draw(&data_b);
            offs_rc.borrow_mut().end();
        }
    });

    let shared_data_c = Rc::clone(&shared_data_rc);
    let mut mouse_drag: Option<(i32, i32)> = None;

    wind.handle(move |ev| match ev {
        enums::Event::MouseWheel => {
            let event = &app::event_coords();
            let mut shared_data_bm = shared_data_c.borrow_mut();
            let event_dy = app::event_dy();
            let reverse_middle = shared_data_bm
                .visualizer
                .affine
                .reverse_transform(event.0 as f32, event.1 as f32);

            if event_dy != 0 {
                shared_data_bm.visualizer.affine.scale *= 1.01_f32.powf(event_dy as f32);
            }
            let new_middle = shared_data_bm
                .visualizer
                .affine
                .transform(reverse_middle[0] as f32, reverse_middle[1] as f32);
            // When zooming we want the center of screen remain at the same relative position.
            shared_data_bm.visualizer.affine.to_offset[0] += (event.0 as f32) - new_middle[0];
            shared_data_bm.visualizer.affine.to_offset[1] += (event.1 as f32) - new_middle[1];

            //println!("mouse wheel at dy:{:?} scale:{:?}", event_dy, shared_data_bm.visualizer.affine.scale);
            redraw();
            true
        }
        enums::Event::Drag => {
            let event = &app::event_coords();
            if mouse_drag.is_none() {
                mouse_drag = Some(*event);
            } else {
                let md = mouse_drag.unwrap();
                let mut shared_data_bm = shared_data_c.borrow_mut();
                shared_data_bm.visualizer.affine.to_offset[0] += (event.0 - md.0) as f32;
                shared_data_bm.visualizer.affine.to_offset[1] += (event.1 - md.1) as f32;
                mouse_drag = Some(*event);
                redraw();
            }
            true
        }
        enums::Event::Released => {
            let event = &app::event_coords();
            //let  ke = &app::event_key();
            if mouse_drag.is_some() {
                mouse_drag = None;
            } else if event_key_down(Key::from_char('L')) || event_key_down(Key::from_char('S')) {
                let mut shared_data_bm = shared_data_c.borrow_mut();
                let point = Point {
                    x: shared_data_bm
                        .visualizer
                        .affine
                        .reverse_transform_x(event_x() as f32),
                    y: shared_data_bm
                        .visualizer
                        .affine
                        .reverse_transform_y(event_y() as f32),
                };
                if let Some(last_point) = shared_data_bm.last_click {
                    let line = Line {
                        start: last_point,
                        end: point,
                    };

                    if !shared_data_bm.visualizer.self_intersecting_check(&line) {
                        shared_data_bm.visualizer.segment_data_.push(line);

                        let _ = shared_data_bm.visualizer.build();

                        if event_key_down(Key::from_char('L')) {
                            shared_data_bm.last_click = None;
                        } else {
                            shared_data_bm.last_click = Some(point);
                        }
                        redraw();
                    }
                } else {
                    shared_data_bm.last_click = Some(point);
                }
            } else {
                if event_x() < FW {
                    println!("mouse at {:?}", event);
                    let mut shared_data_bm = shared_data_c.borrow_mut();
                    {
                        let x = shared_data_bm
                            .visualizer
                            .affine
                            .reverse_transform_x(event_x() as f32);
                        let y = shared_data_bm
                            .visualizer
                            .affine
                            .reverse_transform_y(event_y() as f32);
                        shared_data_bm.visualizer.point_data_.push(Point { x, y });
                    }
                    let _ = shared_data_bm.visualizer.build();

                    shared_data_bm.last_click = None;
                    redraw();
                }
            }
            true
        }
        enums::Event::KeyDown => {
            if event_key_down(Key::from_char('C')) {
                let mut shared_data_bm = shared_data_c.borrow_mut();
                shared_data_bm.last_click = None;
                shared_data_bm.visualizer.segment_data_.clear();
                shared_data_bm.visualizer.point_data_.clear();
                shared_data_bm.visualizer.diagram.clear();
                redraw();
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
                    shared_data_bm.visualizer.read_data(v);
                    let _ = shared_data_bm.visualizer.build();
                    let _ = shared_data_bm.visualizer.recalulate_affine();
                    redraw();
                }
                GuiMessage::Filter(flag) => {
                    shared_data_bm.draw_flag ^= flag;
                }
            }
            shared_data_bm.last_message = Some(msg);
            redraw();
        }
    }
    Ok(())
}

/// struct to help deal with the voronoi diagram input and output
pub struct VoronoiVisualizer<I1, F1, I2, F2>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
    I2: BigIntType + Neg<Output = I2>,
    F2: BigFloatType + Neg<Output = F2>,
{
    bounding_rect: VU::Aabb2<I1, F1>,
    diagram: VD::VoronoiDiagram<I1, F1, I2, F2>,
    vertex_aabb: VU::Aabb2<I1, F1>,

    point_data_: Vec<boostvoronoi::Point<I1>>,
    segment_data_: Vec<boostvoronoi::Line<I1>>,
    affine: VU::SimpleAffine<I1, F1>,
}

impl<I1, F1, I2, F2> VoronoiVisualizer<I1, F1, I2, F2>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
    I2: BigIntType + Neg<Output = I2>,
    F2: BigFloatType + Neg<Output = F2>,
{
    pub fn default() -> Self {
        Self {
            bounding_rect: VU::Aabb2::<I1, F1>::new_from_i32(0, 0, FW, FH),
            diagram: VD::VoronoiDiagram::<I1, F1, I2, F2>::new(0),
            vertex_aabb: VU::Aabb2::<I1, F1>::default(),
            point_data_: Vec::<boostvoronoi::Point<I1>>::new(),
            segment_data_: Vec::<boostvoronoi::Line<I1>>::new(),
            affine: VU::SimpleAffine::default(),
        }
    }

    /// recalculates the affine transformation, this should not be done every time
    /// the diagram is re-calculated or the screen will move around when adding new edges and points.
    pub fn recalulate_affine(&mut self) -> Result<(), BvError> {
        self.affine = VU::SimpleAffine::new(&self.vertex_aabb, &self.bounding_rect)?;
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

        let mut vb = VB::Builder::<I1, F1, I2, F2>::new();
        vb.with_vertices(self.point_data_.iter())?;
        vb.with_segments(self.segment_data_.iter())?;

        // Construct voronoi diagram.
        self.diagram = vb.construct()?;
        self.vertex_aabb = self.diagram.vertices_get_aabb();

        // Color exterior edges.
        for it in self.diagram.edges().iter() {
            let edge_id = Some(it.get().get_id());
            if !self.diagram.edge_is_finite(edge_id).unwrap() {
                self.color_exterior(edge_id);
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

    /// Todo something is wrong here, some external edges will remain unmarked
    /// some secondary internal edges are marked too.
    fn color_exterior(&self, edge_id: Option<VD::VoronoiEdgeIndex>) {
        if edge_id.is_none()
            || ColorFlag::from_bits(self.diagram.edge_get_color(edge_id).unwrap())
                .unwrap()
                .contains(ColorFlag::EXTERNAL)
        {
            // This edge has already been colored, break recursion
            return;
        }
        // Color this and the twin edge as EXTERNAL
        self.diagram
            .edge_or_color(edge_id, ColorFlag::EXTERNAL.bits);
        self.diagram.edge_or_color(
            self.diagram.edge_get_twin(edge_id),
            ColorFlag::EXTERNAL.bits,
        );
        let v = self.diagram.edge_get_vertex1(edge_id);
        if v.is_none() || !self.diagram.get_edge(edge_id.unwrap()).get().is_primary() {
            // stop if this edge does not have a vertex1 (e.g is infinite)
            // or if this edge isn't a primary edge.
            return;
        }
        self.diagram.vertex_set_color(v, ColorFlag::EXTERNAL.bits);
        let incident_edge = self.diagram.vertex_get_incident_edge(v);
        for e in self.diagram.edge_rot_next_iterator(incident_edge) {
            // mark all surrounding edges as EXTERNAL, but only recurse on primary edges
            self.color_exterior(Some(e));
        }
    }

    fn draw(&self, config: &SharedData) -> Result<(), BvError> {
        set_line_style(LineStyle::Solid, 1);

        draw::set_draw_color(Color::Red);
        if config.draw_flag.contains(DrawFilterFlag::INPUT_POINT) {
            self.draw_input_points(&self.affine);
        }
        if config.draw_flag.contains(DrawFilterFlag::INPUT_SEGMENT) {
            self.draw_input_segments(&self.affine);
        }
        if config.draw_flag.contains(DrawFilterFlag::EDGES) {
            draw::set_draw_color(Color::Green);
            self.draw_edges(&config, &self.affine);
        }
        if config.draw_flag.contains(DrawFilterFlag::VERTICES) {
            set_draw_color(Color::Blue);
            self.draw_vertices(&config, &self.affine);
        }
        Ok(())
    }

    #[allow(dead_code)]
    /// Draw bounding box.
    fn draw_bb(&self) {
        let min_x = Self::f1_to_i32(self.bounding_rect.get_low().unwrap()[0]);
        let max_x = Self::f1_to_i32(self.bounding_rect.get_high().unwrap()[0]);
        let min_y = Self::f1_to_i32(self.bounding_rect.get_low().unwrap()[1]);
        let max_y = Self::f1_to_i32(self.bounding_rect.get_high().unwrap()[1]);

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

        for it in self.diagram.vertex_iter().enumerate() {
            let vertex = it.1.get();
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

        set_draw_color(Color::Green);
        let mut already_drawn = yabf::Yabf::default();

        for it in self.diagram.edges().iter().enumerate() {
            let edge_id = VoronoiEdgeIndex(it.0);
            let edge = it.1.get();
            if already_drawn.bit(edge_id.0) {
                // already done this or, rather - it's twin
                continue;
            }
            already_drawn.set_bit(edge_id.0, true);
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
            if (!draw_infinite_edges)
                && ColorFlag::from_bits(edge.get_color())
                    .unwrap()
                    .contains(ColorFlag::INFINITE)
            {
                continue;
            }

            if (!draw_external)
                && ColorFlag::from_bits(edge.get_color())
                    .unwrap()
                    .contains(ColorFlag::EXTERNAL)
            {
                continue;
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

            let mut samples = Vec::<[F1; 2]>::new();
            if !self.diagram.edge_is_finite(Some(edge_id)).unwrap() {
                self.clip_infinite_edge(&affine, edge_id, &mut samples);
            } else {
                let vertex0 = self.diagram.vertex_get(edge.vertex0()).unwrap().get();

                samples.push(affine.transform(vertex0.x(), vertex0.y()));
                let vertex1 = self.diagram.edge_get_vertex1(Some(edge_id));
                let vertex1 = self.diagram.vertex_get(vertex1).unwrap().get();

                samples.push(affine.transform(vertex1.x(), vertex1.y()));
                if edge.is_curved() {
                    if draw_curved_as_line {
                        for i in 0..samples.len() - 1 {
                            let x1 = Self::f1_to_i32(samples[i][0]);
                            let y1 = Self::f1_to_i32(samples[i][1]);
                            let x2 = Self::f1_to_i32(samples[i + 1][0]);
                            let y2 = Self::f1_to_i32(samples[i + 1][1]);
                            draw::draw_line(x1, y1, x2, y2);
                        }
                    }
                    if draw_curved {
                        self.sample_curved_edge(&affine, VoronoiEdgeIndex(it.0), &mut samples);
                    } else {
                        continue;
                    }
                }
            }
            for i in 0..samples.len() - 1 {
                let x1 = Self::f1_to_i32(samples[i][0]);
                let y1 = Self::f1_to_i32(samples[i][1]);
                let x2 = Self::f1_to_i32(samples[i + 1][0]);
                let y2 = Self::f1_to_i32(samples[i + 1][1]);
                draw::draw_line(x1, y1, x2, y2);
            }
        }
    }

    fn clip_infinite_edge(
        &self,
        affine: &VU::SimpleAffine<I1, F1>,
        edge_id: VD::VoronoiEdgeIndex,
        clipped_edge: &mut Vec<[F1; 2]>,
    ) {
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
            let p1 = affine.transform_p(&self.retrieve_point(cell1_id));
            let p2 = affine.transform_p(&self.retrieve_point(cell2_id));
            origin[0] = (p1[0] + p2[0]) * Self::f32_to_f1(0.5);
            origin[1] = (p1[1] + p2[1]) * Self::f32_to_f1(0.5);
            direction[0] = p1[1] - p2[1];
            direction[1] = p2[0] - p1[0];
        } else {
            origin = if cell1.contains_segment() {
                affine.transform_p(&self.retrieve_point(cell2_id))
            } else {
                affine.transform_p(&self.retrieve_point(cell1_id))
            };
            let segment = if cell1.contains_segment() {
                self.retrieve_segment(cell1_id)
            } else {
                self.retrieve_segment(cell2_id)
            };
            let dx = affine.transform_ix(segment.end.x) - affine.transform_ix(segment.start.x);
            let dy = affine.transform_iy(segment.end.y) - affine.transform_iy(segment.start.y);
            if (affine.transform_p(&segment.start) == origin) ^ cell1.contains_point() {
                direction[0] = dy;
                direction[1] = -dx;
            } else {
                direction[0] = -dy;
                direction[1] = dx;
            }
        }
        let side =
            self.bounding_rect.get_high().unwrap()[0] - self.bounding_rect.get_low().unwrap()[0];
        let koef = side / Self::max_f1(direction[0].abs(), direction[1].abs());

        let vertex0 = edge.get().vertex0();
        if vertex0.is_none() {
            clipped_edge.push([
                origin[0] - direction[0] * koef,
                origin[1] - direction[1] * koef,
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
                origin[0] + direction[0] * koef,
                origin[1] + direction[1] * koef,
            ]);
        } else {
            let vertex1 = self.diagram.vertex_get(vertex1).unwrap().get();
            clipped_edge.push([
                affine.transform_x(vertex1.x()),
                affine.transform_y(vertex1.y()),
            ]);
        }
    }

    /// Important: sampled_edge should contain both edge endpoints initially.
    fn sample_curved_edge(
        &self,
        affine: &VU::SimpleAffine<I1, F1>,
        edge_id: VD::VoronoiEdgeIndex,
        sampled_edge: &mut Vec<[F1; 2]>,
    ) {
        let max_dist = Self::f32_to_f1(1E-3)
            * (self.bounding_rect.get_high().unwrap()[0]
                - self.bounding_rect.get_low().unwrap()[0]);

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
        VU::VoronoiVisualUtils::<I1, F1, I2, F2>::discretize(
            &point,
            segment,
            max_dist,
            affine,
            sampled_edge,
        );
    }

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

    fn retrieve_segment(&self, cell_id: VD::VoronoiCellIndex) -> &boostvoronoi::Line<I1> {
        let cell = self.diagram.get_cell(cell_id).get();
        let index = cell.source_index() - self.point_data_.len();
        &self.segment_data_[index]
    }

    fn read_data(&mut self, example: Example) {
        self.segment_data_.clear();
        self.point_data_.clear();
        self.diagram.clear();

        let i32_to_i1 = |x| I1::from(x).unwrap();

        let to_points = |points: &[[i32; 2]]| {
            let mut rv = Vec::new();
            for p in points.iter() {
                rv.push(boostvoronoi::Point {
                    x: i32_to_i1(p[0]), // + i32_to_i1(100),
                    y: i32_to_i1(p[1]), // + i32_to_i1(100),
                });
            }
            rv
        };

        let _to_segments = |segments_: &[[i32; 4]]| {
            let mut rv = Vec::new();
            for p in segments_.iter() {
                let line = boostvoronoi::Line::<I1>::new(
                    boostvoronoi::Point {
                        x: i32_to_i1(p[0]), // + i32_to_i1(100),
                        y: i32_to_i1(p[1]), // + i32_to_i1(100),
                    },
                    boostvoronoi::Point {
                        x: i32_to_i1(p[2]), // + i32_to_i1(100),
                        y: i32_to_i1(p[3]), // + i32_to_i1(100),
                    },
                );
                rv.push(line);
            }
            rv
        };

        let points: [[i32; 2]; 0] = [];
        let _simple_segments: [[i32; 4]; 5] = [
            [300, 300, 300, 500],
            [300, 500, 500, 500],
            [500, 500, 500, 300],
            [500, 300, 300, 300],
            [629, 342, 467, 207],
        ];
        let _test_segments: [[i32; 4]; 12] = [
            [-39092, 94519, -91873, 73333],
            [-91873, 73333, -119937, -42834],
            [-119937, -42834, -155353, -59623],
            [-155353, -59623, -250514, -39563],
            [-250514, -39563, -296960, -94300],
            [-296960, -94300, 144469, -94698],
            [144469, -94698, 289762, -122601],
            [289762, -122601, 296960, -115045],
            [296960, -115045, 210691, 117441],
            [210691, 117441, 113416, 122601],
            [113416, 122601, 73916, 119690],
            [73916, 119690, -39092, 94519],
        ];

        let _segments_rust: [[i32; 4]; 352] = [
            [402, 20, 395, 20],
            [408, 23, 402, 20],
            [476, 27, 469, 26],
            [328, 26, 322, 28],
            [335, 29, 328, 26],
            [481, 33, 476, 27],
            [322, 28, 318, 33],
            [264, 49, 257, 47],
            [548, 50, 540, 47],
            [257, 47, 250, 51],
            [552, 56, 548, 50],
            [395, 20, 370, 57],
            [429, 57, 408, 23],
            [362, 58, 335, 29],
            [469, 26, 438, 58],
            [370, 57, 362, 58],
            [438, 58, 429, 57],
            [495, 69, 481, 33],
            [318, 33, 305, 69],
            [295, 71, 264, 49],
            [540, 47, 504, 71],
            [504, 71, 495, 69],
            [305, 69, 295, 71],
            [198, 82, 191, 82],
            [613, 85, 607, 81],
            [191, 82, 185, 87],
            [393, 87, 404, 87],
            [382, 93, 393, 87],
            [616, 92, 613, 85],
            [404, 87, 415, 94],
            [558, 94, 552, 56],
            [250, 51, 241, 94],
            [241, 94, 233, 98],
            [566, 98, 558, 94],
            [233, 98, 198, 82],
            [607, 81, 566, 98],
            [377, 103, 382, 93],
            [415, 94, 420, 104],
            [615, 107, 616, 92],
            [377, 115, 377, 103],
            [420, 104, 420, 115],
            [615, 120, 615, 107],
            [382, 125, 377, 115],
            [420, 115, 414, 125],
            [451, 124, 471, 128],
            [318, 131, 348, 124],
            [140, 128, 133, 129],
            [667, 130, 660, 128],
            [185, 87, 185, 131],
            [614, 131, 615, 120],
            [392, 131, 382, 125],
            [414, 125, 403, 131],
            [403, 131, 392, 131],
            [670, 134, 667, 130],
            [133, 129, 128, 134],
            [471, 128, 510, 141],
            [185, 131, 177, 136],
            [622, 136, 614, 131],
            [289, 142, 318, 131],
            [177, 136, 140, 128],
            [660, 128, 622, 136],
            [671, 140, 670, 134],
            [262, 155, 289, 142],
            [510, 141, 545, 160],
            [348, 124, 384, 162],
            [237, 172, 262, 155],
            [405, 165, 451, 124],
            [384, 162, 395, 167],
            [395, 167, 405, 165],
            [545, 160, 577, 183],
            [128, 134, 136, 177],
            [663, 177, 671, 140],
            [213, 191, 237, 172],
            [668, 185, 663, 177],
            [136, 177, 131, 185],
            [131, 185, 92, 183],
            [707, 183, 668, 185],
            [713, 186, 707, 183],
            [92, 183, 85, 186],
            [717, 191, 713, 186],
            [85, 186, 82, 191],
            [577, 183, 606, 210],
            [717, 198, 717, 191],
            [82, 191, 82, 198],
            [192, 211, 213, 191],
            [490, 211, 192, 211],
            [501, 213, 490, 211],
            [606, 210, 630, 242],
            [701, 233, 717, 198],
            [82, 198, 98, 233],
            [529, 222, 501, 213],
            [705, 241, 701, 233],
            [98, 233, 94, 241],
            [630, 242, 641, 259],
            [94, 241, 56, 248],
            [743, 248, 705, 241],
            [56, 248, 50, 250],
            [749, 250, 743, 248],
            [50, 250, 47, 257],
            [752, 258, 749, 250],
            [47, 257, 49, 264],
            [750, 264, 752, 258],
            [584, 271, 556, 238],
            [119, 293, 132, 291],
            [666, 292, 677, 294],
            [132, 291, 142, 295],
            [728, 295, 750, 264],
            [49, 264, 71, 295],
            [654, 296, 666, 292],
            [728, 296, 728, 295],
            [172, 296, 211, 296],
            [590, 305, 584, 271],
            [211, 296, 211, 474],
            [641, 259, 622, 304],
            [173, 299, 172, 296],
            [337, 298, 337, 350],
            [337, 298, 432, 298],
            [432, 298, 435, 298],
            [111, 301, 119, 293],
            [677, 294, 686, 302],
            [71, 295, 69, 301],
            [69, 301, 69, 302],
            [150, 305, 142, 295],
            [647, 305, 654, 296],
            [69, 302, 69, 305],
            [730, 305, 728, 296],
            [435, 298, 457, 308],
            [106, 311, 111, 301],
            [686, 302, 690, 312],
            [150, 305, 152, 316],
            [646, 317, 647, 305],
            [69, 305, 33, 318],
            [767, 318, 730, 305],
            [622, 304, 613, 322],
            [108, 323, 106, 311],
            [457, 308, 465, 325],
            [690, 312, 688, 324],
            [33, 318, 27, 323],
            [772, 324, 767, 318],
            [184, 326, 173, 299],
            [152, 316, 147, 326],
            [649, 328, 646, 317],
            [773, 329, 772, 324],
            [27, 323, 26, 330],
            [582, 337, 590, 305],
            [115, 332, 108, 323],
            [613, 322, 613, 333],
            [688, 324, 681, 333],
            [147, 326, 138, 334],
            [659, 335, 649, 328],
            [770, 335, 773, 329],
            [181, 337, 184, 326],
            [126, 336, 115, 332],
            [138, 334, 126, 336],
            [681, 333, 670, 337],
            [465, 325, 458, 340],
            [670, 337, 659, 335],
            [613, 333, 618, 342],
            [174, 344, 181, 337],
            [123, 367, 174, 344],
            [458, 340, 438, 348],
            [438, 348, 428, 350],
            [428, 350, 337, 350],
            [26, 330, 58, 362],
            [742, 362, 770, 335],
            [618, 342, 675, 369],
            [742, 369, 742, 362],
            [548, 374, 582, 337],
            [675, 369, 676, 369],
            [742, 370, 742, 369],
            [58, 362, 57, 370],
            [533, 383, 548, 374],
            [121, 387, 123, 367],
            [57, 370, 23, 391],
            [742, 370, 776, 391],
            [550, 397, 533, 383],
            [780, 396, 776, 391],
            [23, 391, 20, 397],
            [558, 405, 550, 397],
            [780, 402, 780, 396],
            [20, 397, 20, 404],
            [562, 410, 558, 405],
            [776, 408, 780, 402],
            [565, 416, 562, 410],
            [676, 369, 677, 418],
            [569, 422, 565, 416],
            [123, 423, 121, 387],
            [677, 418, 647, 418],
            [647, 418, 645, 420],
            [337, 425, 411, 425],
            [337, 476, 337, 425],
            [411, 425, 417, 426],
            [742, 429, 776, 408],
            [20, 404, 57, 429],
            [574, 438, 569, 422],
            [417, 426, 434, 434],
            [645, 420, 644, 442],
            [742, 438, 742, 429],
            [57, 429, 58, 438],
            [126, 449, 123, 423],
            [434, 434, 450, 456],
            [450, 456, 453, 463],
            [644, 442, 637, 463],
            [580, 462, 574, 438],
            [58, 438, 29, 464],
            [770, 464, 742, 438],
            [132, 474, 126, 449],
            [588, 470, 580, 462],
            [773, 471, 770, 464],
            [29, 464, 26, 471],
            [637, 463, 623, 473],
            [211, 474, 132, 474],
            [605, 475, 588, 470],
            [623, 473, 605, 475],
            [406, 476, 337, 476],
            [771, 477, 773, 471],
            [26, 471, 28, 478],
            [407, 478, 406, 476],
            [766, 481, 771, 477],
            [730, 495, 766, 481],
            [28, 478, 69, 495],
            [728, 504, 730, 495],
            [69, 495, 71, 504],
            [453, 463, 464, 515],
            [464, 515, 467, 527],
            [71, 504, 49, 535],
            [752, 540, 728, 504],
            [49, 535, 47, 541],
            [47, 541, 50, 548],
            [749, 548, 752, 540],
            [467, 527, 485, 552],
            [50, 548, 56, 552],
            [743, 552, 749, 548],
            [705, 558, 743, 552],
            [56, 552, 94, 558],
            [407, 560, 407, 478],
            [485, 552, 502, 562],
            [407, 561, 407, 560],
            [502, 562, 506, 562],
            [405, 562, 407, 561],
            [621, 562, 624, 562],
            [175, 562, 405, 562],
            [506, 562, 621, 562],
            [94, 558, 98, 566],
            [701, 566, 705, 558],
            [255, 581, 200, 590],
            [548, 581, 537, 583],
            [711, 588, 701, 566],
            [265, 585, 255, 581],
            [624, 562, 600, 590],
            [537, 583, 529, 590],
            [200, 590, 175, 562],
            [600, 590, 598, 592],
            [270, 593, 265, 585],
            [598, 592, 548, 581],
            [98, 566, 82, 601],
            [717, 601, 711, 588],
            [717, 609, 717, 601],
            [82, 601, 82, 609],
            [227, 609, 238, 610],
            [558, 611, 571, 610],
            [714, 613, 717, 609],
            [82, 609, 86, 613],
            [216, 615, 227, 609],
            [571, 610, 581, 616],
            [238, 610, 248, 617],
            [86, 613, 92, 616],
            [707, 616, 714, 613],
            [92, 616, 131, 614],
            [668, 614, 707, 616],
            [549, 618, 558, 611],
            [666, 617, 668, 614],
            [131, 614, 136, 622],
            [663, 622, 666, 617],
            [210, 625, 216, 615],
            [581, 616, 587, 626],
            [248, 617, 253, 628],
            [544, 628, 549, 618],
            [209, 637, 210, 625],
            [587, 626, 588, 638],
            [281, 641, 270, 593],
            [253, 628, 252, 639],
            [544, 640, 544, 628],
            [214, 647, 209, 637],
            [588, 638, 582, 648],
            [252, 639, 247, 648],
            [551, 649, 544, 640],
            [283, 649, 281, 641],
            [291, 653, 283, 649],
            [529, 590, 514, 650],
            [224, 653, 214, 647],
            [236, 654, 247, 648],
            [582, 648, 572, 654],
            [561, 655, 551, 649],
            [236, 654, 224, 653],
            [572, 654, 561, 655],
            [136, 622, 128, 659],
            [514, 650, 478, 664],
            [329, 666, 291, 653],
            [671, 665, 663, 622],
            [128, 659, 129, 666],
            [614, 668, 622, 663],
            [177, 663, 185, 668],
            [666, 670, 671, 665],
            [129, 666, 134, 671],
            [478, 664, 439, 672],
            [622, 663, 659, 671],
            [659, 671, 666, 670],
            [134, 671, 177, 663],
            [368, 673, 329, 666],
            [439, 672, 419, 674],
            [389, 675, 368, 673],
            [419, 674, 389, 675],
            [558, 705, 566, 701],
            [233, 701, 241, 705],
            [185, 668, 183, 707],
            [614, 712, 614, 668],
            [183, 707, 186, 713],
            [198, 717, 233, 701],
            [566, 701, 601, 717],
            [609, 717, 614, 712],
            [186, 713, 191, 718],
            [191, 718, 198, 717],
            [601, 717, 609, 717],
            [495, 730, 504, 728],
            [295, 728, 305, 730],
            [241, 705, 248, 743],
            [552, 743, 558, 705],
            [362, 742, 370, 742],
            [429, 742, 438, 742],
            [549, 749, 552, 743],
            [248, 743, 251, 749],
            [504, 728, 535, 750],
            [542, 752, 549, 749],
            [251, 749, 259, 752],
            [259, 752, 295, 728],
            [535, 750, 542, 752],
            [305, 730, 318, 766],
            [481, 766, 495, 730],
            [438, 742, 464, 770],
            [334, 771, 362, 742],
            [318, 766, 322, 771],
            [476, 772, 481, 766],
            [464, 770, 470, 773],
            [322, 771, 327, 773],
            [327, 773, 334, 771],
            [470, 773, 476, 772],
            [370, 742, 391, 776],
            [404, 779, 429, 742],
            [391, 776, 397, 780],
            [397, 780, 404, 779],
            [556, 238, 529, 222],
        ];
        // Preparing Input Geometries.
        self.point_data_.append(&mut to_points(&points));
        let mut new_segments = match example {
            Example::Simple => VB::to_segments::<i32, I1>(&_simple_segments),
            Example::Complex => VB::to_segments::<i32, I1>(&_segments_rust),
            Example::Atest => VB::to_segments::<i32, I1>(&_test_segments), //, 1.0 / 1024.0, 350, 350),
            Example::Clean => {
                let clean: [[i32; 4]; 0] = [];
                VB::to_segments_t::<I1>(&clean, 1.0, 0, 0)
            }
        };
        for s in new_segments.iter() {
            assert!(!self.self_intersecting_check(s));
        }

        self.segment_data_.append(&mut new_segments);
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
    pub fn f32_to_f1(value: f32) -> F1 {
        TypeConverter4::<I1, F1, I2, F2>::f32_to_f1(value)
    }

    #[inline(always)]
    pub fn f1_to_i1(value: F1) -> I1 {
        TypeConverter2::<I1, F1>::f1_to_i1(value)
    }

    #[inline(always)]
    pub fn i1_to_f64(value: I1) -> f64 {
        TypeConverter2::<I1, F1>::i1_to_f64(value)
    }

    #[inline(always)]
    pub fn i1_to_i32(value: I1) -> i32 {
        TypeConverter2::<I1, F1>::i1_to_i32(value)
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
