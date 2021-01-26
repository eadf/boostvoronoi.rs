extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;
extern crate touch_visualizer;

use boostvoronoi::voronoi_builder as VB;
use boostvoronoi::voronoi_diagram as VD;
use boostvoronoi::voronoi_diagram::VoronoiEdgeIndex;
use boostvoronoi::voronoi_error::BVError;
use boostvoronoi::voronoi_visual_utils as VV;
use boostvoronoi::TypeConverter;
use boostvoronoi::{BigFloatType, BigIntType, InputType, OutputType};
use ordered_float::OrderedFloat;
use sdl2_window::Sdl2Window;
use std::cell::RefCell;
use std::collections::HashSet;
use std::marker::PhantomData;
use std::ops::Neg;
use std::rc::Rc;

use geo::algorithm::intersects::Intersects;
use geo::{Coordinate, Line, Rect};

use graphics::math::Scalar;
use graphics::{Context, Graphics};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::{Window, WindowSettings};

use touch_visualizer::TouchVisualizer;

const EXTERNAL_COLOR: u32 = 1;
static DEFAULT_WINDOW_HEIGHT: u32 = 800;
static DEFAULT_WINDOW_WIDTH: u32 = 800;

fn main() {
    println!("Simple piston gui, controls:");
    println!("--------------------------------------");
    println!(" 's' : press and hold + mouse click -> line strings");
    println!(" 'l' : press and hold + mouse click -> line segments");
    println!(" mouse click -> point");
    println!(" 'c' : clear everything");
    println!(" 'i' : toggle draw internal edges only ");
    println!(" 'p' : toggle draw primary edges only");
    println!(" '1' : load simple example data");
    println!(" '2' : load complex example data (can be slow)");
    println!(" move mouse to trigger refresh for 'i' and 'p'");
    println!();
    let _ = event_loop();
}

fn event_loop() -> Result<String, BVError> {
    let mut data_is_dirty = false;

    let visualizer = Rc::new(RefCell::new(
        VorVisualizer::<i32, Scalar, i64, Scalar>::new(),
    ));
    {
        let mut vis = visualizer.borrow_mut();
        // Clear all containers.
        vis.clear();
        vis.read_data(true);
        vis.build()?;
    }

    let opengl = OpenGL::V3_2;
    let mut window: Sdl2Window = WindowSettings::new(
        "Rusted Boost Voronoi",
        [DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH],
    )
    .exit_on_esc(true)
    .graphics_api(opengl)
    .build()
    .unwrap();

    let gl = &mut GlGraphics::new(opengl);
    let mut touch_visualizer = TouchVisualizer::new();
    let mut events = Events::new(EventSettings::new().lazy(true));
    //let mut colors = Vec::new();
    let mut is_drawing_lines = false;
    let mut is_drawing_line_strings = false;
    let mut previous_dot: Option<Coordinate<i32>> = None;
    let mut mx: i32 = 0;
    let mut my: i32 = 0;

    while let Some(e) = events.next(&mut window) {
        touch_visualizer.event(window.size(), &e);
        e.mouse_cursor(|mouse_coord| {
            mx = num::cast::<f64, i32>(mouse_coord[0]).unwrap();
            my = num::cast::<f64, i32>(mouse_coord[1]).unwrap();
        });
        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(key) => {
                    if key == piston::input::keyboard::Key::L {
                        is_drawing_lines = true;
                        is_drawing_line_strings = false;
                    }
                    if key == piston::input::keyboard::Key::S {
                        is_drawing_lines = false;
                        is_drawing_line_strings = true;
                    }
                    if key == piston::input::keyboard::Key::D1
                        || key == piston::input::keyboard::Key::D2
                    {
                        let mut vis = visualizer.borrow_mut();
                        // Clear all containers.
                        vis.clear();
                        vis.read_data(key == piston::input::keyboard::Key::D1);
                        data_is_dirty = true;
                    }
                }
                _ => (),
            }
        }
        if let Some(button) = e.release_args() {
            match button {
                Button::Keyboard(key) => {
                    if key == piston::input::keyboard::Key::C {
                        let mut vis = visualizer.borrow_mut();
                        vis.clear();
                        data_is_dirty = true;
                        vis.previous_points.clear();
                    }
                    if key == piston::input::keyboard::Key::L {
                        is_drawing_lines = false;
                        previous_dot = None;
                    }
                    if key == piston::input::keyboard::Key::S {
                        is_drawing_line_strings = false;
                        previous_dot = None;
                    }
                    if key == piston::input::keyboard::Key::I {
                        let mut vis = visualizer.borrow_mut();
                        vis.internal_edges_only_ = !vis.internal_edges_only_;
                    }
                    if key == piston::input::keyboard::Key::P {
                        let mut vis = visualizer.borrow_mut();
                        vis.primary_edges_only_ = !vis.primary_edges_only_;
                    }
                }
                Button::Mouse(_) => {
                    let point = Coordinate { x: mx, y: my };
                    let mut vis = visualizer.borrow_mut();

                    // Two points at the same place is a problem
                    if !vis.previous_points.contains(&point) {
                        vis.previous_points.insert(point);
                        if is_drawing_lines {
                            if let Some(tpp) = previous_dot {
                                let line = Line::new(tpp, point);
                                if !vis.self_intersecting_check(&line) {
                                    vis.segment_data_.push(line);
                                    data_is_dirty = true;
                                } else {
                                    println!(
                                        "Line {:?} intersects with the previous lines. not added",
                                        line
                                    );
                                }
                                previous_dot = None;
                            } else {
                                previous_dot = Some(point);
                            }
                        } else if is_drawing_line_strings {
                            if let Some(tpp) = previous_dot {
                                let line = Line::new(tpp, point);
                                if !vis.self_intersecting_check(&line) {
                                    vis.segment_data_.push(line);
                                    data_is_dirty = true;
                                    previous_dot = Some(point);
                                } else {
                                    println!(
                                        "Line {:?} intersects with the previous lines. not added",
                                        line
                                    );
                                }
                            } else {
                                previous_dot = Some(point);
                            }
                        } else {
                            vis.point_data_.push(Coordinate { x: mx, y: my });
                            data_is_dirty = true;
                        }
                    }
                }
                _ => (),
            }
        };
        if data_is_dirty {
            let mut vis = visualizer.borrow_mut();
            println!("vis.build()");
            vis.build()?;
            data_is_dirty = false;
        }
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                graphics::clear([1.0; 4], g);
                {
                    let vis = visualizer.borrow();
                    vis.draw_edges(&c, g);
                    vis.draw_points(&c, g);
                    vis.draw_vertices(&c, g);
                    vis.draw_segments(&c, g);
                }
            });
        }
    }
    Result::Ok("".to_string())
}

pub struct VorVisualizer<I1, F1, I2, F2>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
    I2: BigIntType + Neg<Output = I2>,
    F2: BigFloatType + Neg<Output = F2>,
{
    bounding_rect: Rect<F1>,
    vd_: VD::VoronoiDiagram<I1, F1, I2, F2>,
    primary_edges_only_: bool,
    internal_edges_only_: bool,
    point_data_: Vec<Coordinate<I1>>,
    segment_data_: Vec<Line<I1>>,
    pub previous_points: HashSet<Coordinate<i32>>,
    _pdo: PhantomData<F1>,
}

impl<I1, F1, I2, F2> VorVisualizer<I1, F1, I2, F2>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
    I2: BigIntType + Neg<Output = I2>,
    F2: BigFloatType + Neg<Output = F2>,
{
    pub fn new() -> Self {
        Self {
            bounding_rect: Rect::<F1>::new(
                Coordinate {
                    x: F1::from(DEFAULT_WINDOW_HEIGHT).unwrap(),
                    y: F1::from(DEFAULT_WINDOW_WIDTH).unwrap(),
                },
                Coordinate {
                    x: F1::from(0).unwrap(),
                    y: F1::from(0).unwrap(),
                },
            ),
            vd_: VD::VoronoiDiagram::<I1, F1, I2, F2>::new(0),
            primary_edges_only_: false,
            internal_edges_only_: false,
            point_data_: Vec::<Coordinate<I1>>::new(),
            segment_data_: Vec::<Line<I1>>::new(),
            previous_points: HashSet::new(),
            _pdo: PhantomData,
        }
    }

    pub fn build(&mut self) -> Result<String, BVError> {
        println!(
            "Running voronoi with this input (in case of a crash, copy&paste and make a test case)"
        );
        print!("  let points:[[I1;2];{}]=[", self.point_data_.len());
        for p in self.point_data_.iter() {
            print!("[{},{}],", p.x, p.y)
        }
        println!("];");
        print!("  let segments:[[I1;4];{}]=[", self.segment_data_.len());
        for s in self.segment_data_.iter() {
            print!("[{},{},{},{}],", s.start.x, s.start.y, s.end.x, s.end.y)
        }
        println!("];");

        let mut vb = VB::VoronoiBuilder::<I1, F1, I2, F2>::new();
        vb.with_vertices(self.point_data_.iter())?;
        vb.with_segments(self.segment_data_.iter())?;

        // Construct voronoi diagram.
        self.vd_ = vb.construct()?;

        // Color exterior edges.
        for it in self.vd_.edges().iter() {
            let edge_id = Some(it.get().get_id());
            if !self.vd_.edge_is_finite(edge_id).unwrap() {
                self.color_exterior(edge_id);
            }
        }
        Result::Ok("".to_string())
    }

    pub fn show_primary_edges_only(&mut self) {
        self.primary_edges_only_ ^= true;
    }

    pub fn show_internal_edges_only(&mut self) {
        self.internal_edges_only_ ^= true;
    }

    fn clear(&mut self) {
        self.point_data_.clear();
        self.segment_data_.clear();
        self.vd_.clear();
    }

    // returns true if l intersects with any of the lines in self.segment_data_
    fn self_intersecting_check(&self, l: &Line<I1>) -> bool {
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
            // todo: co-linear overlapping lines are intersecting

            let s_ = Self::line_i1_to_f64(s);
            if l_.intersects(&s_) {
                return true;
            }
        }
        false
    }

    fn read_data(&mut self, simple_example: bool) {
        let i32_to_i1 = |x| I1::from(x).unwrap();

        let to_points = |points: &[[i32; 2]]| {
            let mut rv = Vec::new();
            for p in points.iter() {
                rv.push(Coordinate {
                    x: i32_to_i1(p[0]),
                    y: i32_to_i1(p[1]),
                });
            }
            rv
        };

        let to_segments = |segments_: &[[i32; 4]]| {
            let mut rv = Vec::new();
            for p in segments_.iter() {
                let line = Line::<I1>::new(
                    Coordinate {
                        x: i32_to_i1(p[0]),
                        y: i32_to_i1(p[1]),
                    },
                    Coordinate {
                        x: i32_to_i1(p[2]),
                        y: i32_to_i1(p[3]),
                    },
                );
                rv.push(line);
            }
            rv
        };
        /*
        let points: [[i32; 2]; 0] = [];
        let c:i32 = 200;
        let segments: [[i32; 4]; 5] = [[c, c, c, 200 + c], [c, 200 + c, 200 + c, 200 + c], [200 + c, 200 + c, 200 + c, c], [200 + c, c, c, c], [529,242,367,107],];
        */
        let points: [[i32; 2]; 0] = [];
        let _simple_segments: [[i32; 4]; 5] = [
            [200, 200, 200, 400],
            [200, 400, 400, 400],
            [400, 400, 400, 200],
            [400, 200, 200, 200],
            [529, 242, 367, 107],
        ];
        let _segments_rust: [[i32; 4]; 355] = [
            [403, 0, 395, 1],
            [409, 4, 403, 0],
            [481, 8, 473, 7],
            [325, 7, 318, 9],
            [332, 10, 325, 7],
            [486, 14, 481, 8],
            [318, 9, 314, 14],
            [257, 31, 250, 29],
            [556, 32, 548, 29],
            [250, 29, 243, 33],
            [560, 38, 556, 32],
            [395, 1, 369, 39],
            [431, 39, 409, 4],
            [360, 40, 332, 10],
            [473, 7, 440, 40],
            [369, 39, 360, 40],
            [440, 40, 431, 39],
            [501, 52, 486, 14],
            [314, 14, 300, 52],
            [290, 54, 257, 31],
            [548, 29, 510, 54],
            [510, 54, 501, 52],
            [300, 52, 290, 54],
            [188, 66, 180, 66],
            [625, 69, 618, 65],
            [180, 66, 174, 71],
            [393, 71, 405, 71],
            [382, 77, 393, 71],
            [628, 76, 625, 69],
            [405, 71, 416, 78],
            [567, 78, 560, 38],
            [243, 33, 233, 78],
            [233, 78, 225, 83],
            [575, 83, 567, 78],
            [225, 83, 188, 66],
            [618, 65, 575, 83],
            [376, 88, 382, 77],
            [416, 78, 422, 89],
            [627, 92, 628, 76],
            [376, 100, 376, 88],
            [422, 89, 422, 101],
            [627, 106, 627, 92],
            [382, 111, 376, 100],
            [422, 101, 415, 111],
            [454, 110, 475, 114],
            [314, 117, 346, 110],
            [127, 114, 119, 115],
            [682, 116, 674, 114],
            [174, 71, 174, 117],
            [626, 117, 627, 106],
            [392, 117, 382, 111],
            [415, 111, 404, 117],
            [404, 117, 392, 117],
            [685, 120, 682, 116],
            [119, 115, 114, 121],
            [475, 114, 516, 128],
            [174, 117, 166, 123],
            [634, 123, 626, 117],
            [284, 129, 314, 117],
            [166, 123, 127, 114],
            [674, 114, 634, 123],
            [686, 127, 685, 120],
            [255, 143, 284, 129],
            [516, 128, 553, 148],
            [346, 110, 384, 150],
            [229, 160, 255, 143],
            [406, 153, 454, 110],
            [384, 150, 395, 155],
            [395, 155, 406, 153],
            [553, 148, 587, 172],
            [114, 121, 123, 166],
            [677, 166, 686, 127],
            [204, 180, 229, 160],
            [683, 174, 677, 166],
            [123, 166, 117, 174],
            [117, 174, 76, 172],
            [724, 172, 683, 174],
            [730, 175, 724, 172],
            [76, 172, 69, 175],
            [734, 180, 730, 175],
            [69, 175, 66, 181],
            [587, 172, 617, 201],
            [734, 188, 734, 180],
            [66, 181, 66, 188],
            [182, 202, 204, 180],
            [495, 202, 182, 202],
            [507, 204, 495, 202],
            [617, 201, 643, 234],
            [717, 225, 734, 188],
            [66, 188, 83, 225],
            [536, 213, 507, 204],
            [722, 233, 717, 225],
            [83, 225, 78, 233],
            [643, 234, 654, 252],
            [78, 233, 38, 240],
            [762, 240, 722, 233],
            [38, 240, 32, 243],
            [768, 243, 762, 240],
            [32, 243, 29, 250],
            [771, 251, 768, 243],
            [29, 250, 31, 257],
            [769, 257, 771, 251],
            [594, 265, 565, 230],
            [105, 288, 118, 286],
            [680, 287, 692, 289],
            [118, 286, 129, 290],
            [746, 290, 769, 257],
            [31, 257, 54, 290],
            [668, 291, 680, 287],
            [746, 291, 746, 290],
            [161, 291, 202, 291],
            [601, 300, 594, 265],
            [202, 291, 202, 478],
            [654, 252, 634, 299],
            [162, 294, 161, 291],
            [334, 293, 334, 348],
            [334, 293, 434, 293],
            [434, 293, 437, 293],
            [96, 296, 105, 288],
            [692, 289, 702, 297],
            [54, 290, 52, 296],
            [52, 296, 52, 297],
            [137, 300, 129, 290],
            [661, 301, 668, 291],
            [52, 297, 52, 300],
            [748, 300, 746, 291],
            [437, 293, 460, 304],
            [91, 307, 96, 296],
            [702, 297, 706, 308],
            [137, 300, 139, 312],
            [659, 313, 661, 301],
            [52, 300, 14, 314],
            [787, 314, 748, 300],
            [634, 299, 625, 318],
            [93, 319, 91, 307],
            [460, 304, 469, 322],
            [706, 308, 704, 320],
            [14, 314, 8, 319],
            [792, 320, 787, 314],
            [173, 323, 162, 294],
            [139, 312, 134, 323],
            [663, 325, 659, 313],
            [793, 326, 792, 320],
            [8, 319, 7, 327],
            [592, 334, 601, 300],
            [101, 329, 93, 319],
            [625, 318, 625, 330],
            [704, 320, 696, 330],
            [134, 323, 125, 331],
            [673, 332, 663, 325],
            [790, 332, 793, 326],
            [170, 334, 173, 323],
            [112, 333, 101, 329],
            [125, 331, 112, 333],
            [696, 330, 685, 334],
            [469, 322, 462, 337],
            [685, 334, 673, 332],
            [625, 330, 630, 339],
            [163, 342, 170, 334],
            [109, 366, 163, 342],
            [462, 337, 440, 346],
            [440, 346, 430, 348],
            [430, 348, 334, 348],
            [7, 327, 40, 360],
            [760, 360, 790, 332],
            [630, 339, 690, 368],
            [761, 368, 760, 360],
            [556, 373, 592, 334],
            [690, 368, 691, 368],
            [761, 369, 761, 368],
            [40, 360, 39, 369],
            [541, 383, 556, 373],
            [107, 387, 109, 366],
            [39, 369, 4, 391],
            [761, 369, 796, 391],
            [558, 397, 541, 383],
            [800, 396, 796, 391],
            [4, 391, 0, 397],
            [567, 406, 558, 397],
            [800, 403, 800, 396],
            [0, 397, 1, 405],
            [571, 411, 567, 406],
            [796, 409, 800, 403],
            [574, 417, 571, 411],
            [691, 368, 692, 419],
            [578, 424, 574, 417],
            [109, 425, 107, 387],
            [692, 419, 661, 419],
            [661, 419, 658, 422],
            [334, 427, 412, 427],
            [334, 481, 334, 427],
            [412, 427, 418, 428],
            [761, 431, 796, 409],
            [1, 405, 39, 431],
            [584, 441, 578, 424],
            [418, 428, 436, 436],
            [658, 422, 657, 445],
            [760, 440, 761, 431],
            [39, 431, 40, 440],
            [112, 452, 109, 425],
            [436, 436, 453, 459],
            [453, 459, 456, 467],
            [657, 445, 650, 467],
            [590, 466, 584, 441],
            [40, 440, 10, 468],
            [790, 468, 760, 440],
            [118, 478, 112, 452],
            [598, 474, 590, 466],
            [793, 475, 790, 468],
            [10, 468, 7, 475],
            [650, 467, 635, 477],
            [202, 478, 118, 478],
            [616, 479, 598, 474],
            [635, 477, 616, 479],
            [407, 481, 334, 481],
            [791, 482, 793, 475],
            [407, 481, 407, 481],
            [7, 475, 9, 483],
            [408, 483, 407, 481],
            [408, 483, 408, 483],
            [786, 486, 791, 482],
            [748, 501, 786, 486],
            [9, 483, 52, 501],
            [746, 510, 748, 501],
            [52, 501, 54, 510],
            [456, 467, 468, 522],
            [468, 522, 471, 534],
            [54, 510, 31, 543],
            [771, 548, 746, 510],
            [31, 543, 29, 549],
            [29, 549, 32, 556],
            [768, 556, 771, 548],
            [471, 534, 490, 560],
            [32, 556, 38, 560],
            [762, 560, 768, 556],
            [722, 567, 762, 560],
            [38, 560, 78, 567],
            [408, 569, 408, 483],
            [490, 560, 508, 571],
            [408, 570, 408, 569],
            [508, 571, 512, 571],
            [406, 571, 408, 570],
            [406, 571, 406, 571],
            [633, 571, 636, 571],
            [164, 571, 406, 571],
            [512, 571, 633, 571],
            [78, 567, 83, 575],
            [717, 575, 722, 567],
            [248, 591, 190, 601],
            [556, 591, 545, 593],
            [728, 598, 717, 575],
            [258, 595, 248, 591],
            [636, 571, 611, 600],
            [545, 593, 536, 601],
            [190, 601, 164, 571],
            [611, 600, 609, 603],
            [264, 604, 258, 595],
            [609, 603, 556, 591],
            [83, 575, 66, 612],
            [734, 612, 728, 598],
            [734, 620, 734, 612],
            [66, 612, 66, 620],
            [218, 621, 230, 622],
            [567, 623, 580, 622],
            [731, 625, 734, 620],
            [66, 620, 70, 625],
            [207, 627, 218, 621],
            [580, 622, 591, 628],
            [230, 622, 240, 629],
            [70, 625, 76, 628],
            [724, 628, 731, 625],
            [76, 628, 117, 626],
            [683, 626, 724, 628],
            [557, 630, 567, 623],
            [681, 629, 683, 626],
            [117, 626, 123, 634],
            [677, 634, 681, 629],
            [200, 637, 207, 627],
            [591, 628, 597, 638],
            [240, 629, 246, 640],
            [552, 641, 557, 630],
            [199, 650, 200, 637],
            [597, 638, 598, 651],
            [275, 654, 264, 604],
            [246, 640, 245, 652],
            [552, 653, 552, 641],
            [205, 661, 199, 650],
            [598, 651, 592, 662],
            [245, 652, 239, 662],
            [559, 663, 552, 653],
            [277, 663, 275, 654],
            [286, 667, 277, 663],
            [536, 601, 521, 664],
            [215, 667, 205, 661],
            [228, 668, 239, 662],
            [592, 662, 582, 668],
            [570, 669, 559, 663],
            [228, 668, 215, 667],
            [582, 668, 570, 669],
            [123, 634, 114, 673],
            [521, 664, 483, 678],
            [326, 681, 286, 667],
            [686, 679, 677, 634],
            [114, 673, 115, 681],
            [626, 683, 634, 677],
            [166, 677, 174, 683],
            [681, 685, 686, 679],
            [115, 681, 121, 686],
            [483, 678, 442, 687],
            [634, 677, 673, 686],
            [673, 686, 681, 685],
            [121, 686, 166, 677],
            [367, 688, 326, 681],
            [442, 687, 421, 689],
            [389, 690, 367, 688],
            [421, 689, 389, 690],
            [567, 722, 575, 717],
            [225, 717, 233, 722],
            [174, 683, 172, 724],
            [626, 729, 626, 683],
            [172, 724, 175, 730],
            [188, 734, 225, 717],
            [575, 717, 612, 734],
            [620, 734, 626, 729],
            [175, 730, 181, 735],
            [181, 735, 188, 734],
            [612, 734, 620, 734],
            [501, 748, 510, 746],
            [290, 746, 300, 748],
            [233, 722, 240, 762],
            [560, 762, 567, 722],
            [360, 760, 369, 761],
            [431, 761, 440, 760],
            [557, 768, 560, 762],
            [240, 762, 244, 768],
            [510, 746, 543, 769],
            [550, 771, 557, 768],
            [244, 768, 252, 771],
            [252, 771, 290, 746],
            [543, 769, 550, 771],
            [300, 748, 314, 786],
            [486, 786, 501, 748],
            [440, 760, 468, 790],
            [331, 791, 360, 760],
            [314, 786, 318, 791],
            [481, 792, 486, 786],
            [468, 790, 474, 793],
            [318, 791, 324, 793],
            [324, 793, 331, 791],
            [474, 793, 481, 792],
            [369, 761, 391, 796],
            [405, 799, 431, 761],
            [391, 796, 397, 800],
            [397, 800, 405, 799],
            [565, 230, 536, 213],
        ];
        // Preparing Input Geometries.
        self.point_data_.append(&mut to_points(&points));
        let mut new_segments = if simple_example {
            to_segments(&_simple_segments)
        } else {
            to_segments(&_segments_rust)
        };
        for s in new_segments.iter() {
            assert!(!self.self_intersecting_check(s));
        }

        self.segment_data_.append(&mut new_segments);
    }

    fn color_exterior(&self, edge_id: Option<VD::VoronoiEdgeIndex>) {
        if edge_id.is_none() {
            return;
        }
        if self.vd_.edge_get_color(edge_id).unwrap() == EXTERNAL_COLOR {
            return;
        }
        self.vd_.edge_set_color(edge_id, EXTERNAL_COLOR);
        self.vd_
            .edge_set_color(self.vd_.edge_get_twin(edge_id), EXTERNAL_COLOR);
        let v = self.vd_.edge_get_vertex1(edge_id);
        if v.is_none() || !self.vd_.get_edge(edge_id.unwrap()).get().is_primary() {
            return;
        }
        self.vd_.vertex_set_color(v, EXTERNAL_COLOR);
        let mut e = self.vd_.vertex_get_incident_edge(v);
        let v_incident_edge = e;
        while e.is_some() {
            self.color_exterior(e);
            e = self.vd_.edge_rot_next(e);
            if e == v_incident_edge {
                break;
            }
        }
    }

    /// Draw input points and endpoints of the input segments.
    fn draw_points<G: Graphics>(&self, c: &Context, g: &mut G) {
        let color = [0.8, 0.1, 0.1, 1.0];
        let mut draw = |point: &Coordinate<F1>| {
            graphics::ellipse(
                color,
                graphics::ellipse::circle(
                    Self::f1_to_f64(point.x).into_inner(),
                    Self::f1_to_f64(point.y).into_inner(),
                    3.0,
                ),
                c.transform,
                g,
            );
        };

        for i in self.point_data_.iter() {
            draw(&Self::coord_i1_to_f1(&i));
        }
        for i in self.segment_data_.iter() {
            let lp = Self::coord_i1_to_f1(&i.start);
            draw(&lp);
            let hp = Self::coord_i1_to_f1(&i.end);
            draw(&hp);
        }
    }

    /// Draw input segments.
    fn draw_segments<G: Graphics>(&self, c: &Context, g: &mut G) {
        let color = [1.0, 0.0, 0.0, 1.0];

        for i in self.segment_data_.iter() {
            let lp = Self::coord_i1_to_f1(&i.start);
            let hp = Self::coord_i1_to_f1(&i.end);
            graphics::line(
                color,
                2.0,
                [
                    Self::f1_to_f64(lp.x).into(),
                    Self::f1_to_f64(lp.y).into(),
                    Self::f1_to_f64(hp.x).into(),
                    Self::f1_to_f64(hp.y).into(),
                ],
                c.transform,
                g,
            )
        }
    }

    /// Draw voronoi vertices aka circle events.
    fn draw_vertices<G: Graphics>(&self, c: &Context, g: &mut G) {
        let color = [0.0, 0.0, 0.6, 1.0];

        for it in self.vd_.vertex_iter() {
            let it = it.1.get();
            if self.internal_edges_only_ && it.get_color() == EXTERNAL_COLOR {
                continue;
            }
            let point = Coordinate {
                x: it.x(),
                y: it.y(),
            };
            graphics::ellipse(
                color,
                graphics::ellipse::circle(
                    Self::f1_to_f64(point.x).into(),
                    Self::f1_to_f64(point.y).into(),
                    2.0,
                ),
                c.transform,
                g,
            );
        }
    }

    /// Draw voronoi edges.
    fn draw_edges<G: Graphics>(&self, c: &Context, g: &mut G) {
        #[allow(unused_assignments)]
        let mut color = [0.0, 6.0, 0.0, 1.0];

        for it in self.vd_.edges().iter().enumerate() {
            let edge_id = VoronoiEdgeIndex(it.0);
            let edge = it.1.get();
            assert_eq!(edge.get_id(), edge_id);

            #[allow(unused_assignments)]
            if edge.is_primary() {
                // primary edge color
                color = [0.5, 0.5, 0.7, 1.0];
            } else {
                if self.primary_edges_only_ {
                    continue;
                }
                // non-primary edge color
                color = [0.5, 0.2, 0.1, 1.0];
            }
            if self.internal_edges_only_ && (edge.get_color() == EXTERNAL_COLOR) {
                continue;
            } else {
                // internal edgecolor
                color = [0.2, 0.7, 0.0, 1.0];
            }
            let mut samples = Vec::<Coordinate<F1>>::new();
            if !self.vd_.edge_is_finite(Some(edge_id)).unwrap() {
                self.clip_infinite_edge(edge_id, &mut samples);
            } else {
                let vertex0 = self.vd_.vertex_get(edge.vertex0()).unwrap().get();
                let vertex0 = Coordinate {
                    x: vertex0.x(),
                    y: vertex0.y(),
                };
                samples.push(vertex0);
                let vertex1 = self.vd_.edge_get_vertex1(Some(edge_id));
                let vertex1 = self.vd_.vertex_get(vertex1).unwrap().get();
                let vertex1 = Coordinate {
                    x: vertex1.x(),
                    y: vertex1.y(),
                };
                samples.push(vertex1);
                if edge.is_curved() {
                    self.sample_curved_edge(VoronoiEdgeIndex(it.0), &mut samples);
                }
            }
            for i in 0..samples.len() - 1 {
                let vertex1 = samples[i];
                let vertex2 = samples[i + 1];

                //vertex1.deconvolve(&self.shift_);
                //vertex2.deconvolve(&self.shift_);
                graphics::line(
                    color,
                    1.0,
                    [
                        Self::f1_to_f64(vertex1.x).into(),
                        Self::f1_to_f64(vertex1.y).into(),
                        Self::f1_to_f64(vertex2.x).into(),
                        Self::f1_to_f64(vertex2.y).into(),
                    ],
                    c.transform,
                    g,
                );
            }
        }
    }

    fn clip_infinite_edge(
        &self,
        edge_id: VD::VoronoiEdgeIndex,
        clipped_edge: &mut Vec<Coordinate<F1>>,
    ) {
        let edge = self.vd_.get_edge(edge_id);
        //const cell_type& cell1 = *edge.cell();
        let cell1_id = self.vd_.edge_get_cell(Some(edge_id)).unwrap();
        let cell1 = self.vd_.get_cell(cell1_id).get();
        //const cell_type& cell2 = *edge.twin()->cell();
        let cell2_id = self
            .vd_
            .edge_get_twin(Some(edge_id))
            .and_then(|e| self.vd_.edge_get_cell(Some(e)))
            .unwrap();
        let cell2 = self.vd_.get_cell(cell2_id).get();

        let mut origin: Coordinate<F1> = Coordinate {
            x: F1::default(),
            y: F1::default(),
        };
        let mut direction: Coordinate<F1> = Coordinate {
            x: F1::default(),
            y: F1::default(),
        };
        // Infinite edges could not be created by two segment sites.
        if cell1.contains_point() && cell2.contains_point() {
            let p1 = Self::coord_i1_to_f1(&self.retrieve_point(cell1_id));
            let p2 = Self::coord_i1_to_f1(&self.retrieve_point(cell2_id));
            origin.x = (p1.x + p2.x) * Self::f32_to_f1(0.5);
            origin.y = (p1.y + p2.y) * Self::f32_to_f1(0.5);
            direction.x = p1.y - p2.y;
            direction.y = p2.x - p1.x;
        } else {
            origin = if cell1.contains_segment() {
                Self::coord_i1_to_f1(&self.retrieve_point(cell2_id))
            } else {
                Self::coord_i1_to_f1(&self.retrieve_point(cell1_id))
            };
            let segment = if cell1.contains_segment() {
                self.retrieve_segment(cell1_id)
            } else {
                self.retrieve_segment(cell2_id)
            };
            let dx = Self::i1_to_f1(segment.end.x - segment.start.x);
            let dy = Self::i1_to_f1(segment.end.y - segment.start.y);
            if (Self::coord_i1_to_f1(&segment.start) == origin) ^ cell1.contains_point() {
                direction.x = dy;
                direction.y = -dx;
            } else {
                direction.x = -dy;
                direction.y = dx;
            }
        }
        let side = self.bounding_rect.max().x - self.bounding_rect.min().x;
        let koef = side / Self::max_f1(direction.x.abs(), direction.y.abs());

        let vertex0 = edge.get().vertex0();
        if vertex0.is_none() {
            clipped_edge.push(Coordinate {
                x: origin.x - direction.x * koef,
                y: origin.y - direction.y * koef,
            });
        } else {
            let vertex0 = self.vd_.vertex_get(vertex0).unwrap().get();
            clipped_edge.push(Coordinate {
                x: vertex0.x(),
                y: vertex0.y(),
            });
        }
        let vertex1 = self.vd_.edge_get_vertex1(Some(edge_id));
        if vertex1.is_none() {
            clipped_edge.push(Coordinate {
                x: origin.x + direction.x * koef,
                y: origin.y + direction.y * koef,
            });
        } else {
            let vertex1 = self.vd_.vertex_get(vertex1).unwrap().get();
            clipped_edge.push(Coordinate {
                x: vertex1.x(),
                y: vertex1.y(),
            });
        }
    }

    fn sample_curved_edge(
        &self,
        edge_id: VD::VoronoiEdgeIndex,
        sampled_edge: &mut Vec<Coordinate<F1>>,
    ) {
        let max_dist =
            Self::f32_to_f1(1E-3) * (self.bounding_rect.max().x - self.bounding_rect.min().x);

        let cell_id = self.vd_.edge_get_cell(Some(edge_id)).unwrap();
        let cell = self.vd_.get_cell(cell_id).get();
        let twin_id = self.vd_.edge_get_twin(Some(edge_id)).unwrap();
        let twin_cell_id = self.vd_.edge_get_cell(Some(twin_id)).unwrap();

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
        VV::VoronoiVisualUtils::<I1, F1, I2, F2>::discretize(
            &point,
            segment,
            max_dist,
            sampled_edge,
        );
    }

    fn retrieve_point(&self, cell_id: VD::VoronoiCellIndex) -> Coordinate<I1> {
        let cell = self.vd_.get_cell(cell_id).get();
        let mut index = cell.source_index();
        let category = cell.source_category();
        if category.0 == VD::SourceCategory::SOURCE_CATEGORY_SINGLE_POINT.0 {
            return self.point_data_[index];
        }
        index -= self.point_data_.len();
        if category.0 == VD::SourceCategory::SOURCE_CATEGORY_SEGMENT_START_POINT.0 {
            self.segment_data_[index].start
        } else {
            self.segment_data_[index].end
        }
    }

    fn retrieve_segment(&self, cell_id: VD::VoronoiCellIndex) -> &Line<I1> {
        let cell = self.vd_.get_cell(cell_id).get();
        let index = cell.source_index() - self.point_data_.len();
        &self.segment_data_[index]
    }

    fn line_i1_to_f64(value: &Line<I1>) -> Line<f64> {
        let ps = Coordinate {
            x: Self::i1_to_f64(value.start.x),
            y: Self::i1_to_f64(value.start.y),
        };
        let pe = Coordinate {
            x: Self::i1_to_f64(value.end.x),
            y: Self::i1_to_f64(value.end.y),
        };
        Line::<f64>::new(ps, pe)
    }

    fn coord_i1_to_f1(value: &Coordinate<I1>) -> Coordinate<F1> {
        Coordinate {
            x: Self::i1_to_f1(value.x),
            y: Self::i1_to_f1(value.y),
        }
    }

    #[inline(always)]
    fn max_f1(a: F1, b: F1) -> F1 {
        OrderedFloat(a).max(OrderedFloat(b)).into_inner()
    }

    #[inline(always)]
    pub fn i1_to_f1(value: I1) -> F1 {
        TypeConverter::<I1, F1, I2, F2>::i1_to_f1(value)
    }

    #[inline(always)]
    pub fn f32_to_f1(value: f32) -> F1 {
        TypeConverter::<I1, F1, I2, F2>::f32_to_f1(value)
    }

    #[inline(always)]
    pub fn f1_to_i1(value: F1) -> I1 {
        TypeConverter::<I1, F1, I2, F2>::f1_to_i1(value)
    }

    #[inline(always)]
    pub fn i1_to_f64(value: I1) -> f64 {
        TypeConverter::<I1, F1, I2, F2>::i1_to_f64(value)
    }

    #[inline(always)]
    pub fn f1_to_f64(v: F1) -> OrderedFloat<f64> {
        OrderedFloat(num::cast::<F1, f64>(v).unwrap())
    }
}
