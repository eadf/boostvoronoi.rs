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
use num;

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

#[allow(dead_code)]
fn scale(x: i32) -> i32 {
    let x: f64 = num::cast::<i32, f64>(x).unwrap();
    let x: f64 = (x * 760.0 / 800.0) + 20.0;
    num::cast::<f64, i32>(x).unwrap()
}

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
                1.0,
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

        for it in self.vd_.vertex_iter().enumerate() {
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
        let (index, cat) = self.vd_.get_cell(cell_id).get().source_index_2();
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
