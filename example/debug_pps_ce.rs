use fltk::{
    app,
    draw::{
        draw_circle, draw_line, draw_rectf, set_draw_color, set_line_style, LineStyle, Offscreen,
    },
    enums::{Color, FrameType},
    frame::Frame,
    prelude::*,
    window::Window,
};
use std::cell::RefCell;
use std::rc::Rc;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);

    let mut wind = Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label("Debug Circle events");
    let mut frame = Frame::default()
        .with_size(WIDTH - 10, HEIGHT - 10)
        .center_of(&wind);
    frame.set_color(Color::White);
    frame.set_frame(FrameType::DownBox);

    wind.end();
    wind.show();

    let offs = Offscreen::new(frame.width(), frame.height()).unwrap();
    #[cfg(not(target_os = "macos"))]
    {
        offs.begin();
        set_draw_color(Color::White);
        draw_rectf(0, 0, WIDTH - 10, HEIGHT - 10);
        offs.end();
    }

    let offs = Rc::from(RefCell::from(offs));
    let offs_rc = offs.clone();

    frame.draw(move |_| {
        if offs_rc.borrow().is_valid() {
            offs_rc.borrow().copy(5, 5, WIDTH - 10, HEIGHT - 10, 0, 0);
        } else {
            offs_rc.borrow_mut().begin();
            set_draw_color(Color::White);
            draw_rectf(0, 0, WIDTH - 10, HEIGHT - 10);
            offs_rc.borrow_mut().end();
        }
    });
    {
        let site1 = [-5165, -5162];
        let site2 = [-5404, -5134];
        let site3 = [-5011, -5195, -5404, -5134];
        let c1 = [-4909.985314685315, -1951.249650349650]; //lx=83.691104228608
        let c2 = [-4909.985314685315, -1951.249650349650]; //l_x=97.17800560721074

        // site3.point0 -> c
        let v_3_c = ((c1[0] - site3[0] as f64), (c1[1] - site3[1] as f64));
        // site3.point0 -> site3.point1
        let v_3 = ((site3[2] - site3[0]) as f64, (site3[3] - site3[1]) as f64);

        let dot = v_3_c.0 * v_3.0 + v_3_c.1 * v_3.1 / (v_3.0 * v_3.0 + v_3.1 * v_3.1);
        println!("dot:{:?}", dot);
        use geo::algorithm::euclidean_distance::*;
        let c = geo::Coordinate { x: c1[0], y: c1[1] };
        println!(
            "site1 dist:{:?}",
            c.euclidean_distance(&geo::Coordinate {
                x: site1[0] as f64,
                y: site1[1] as f64
            })
        );
        println!(
            "site2 dist:{:?}",
            c.euclidean_distance(&geo::Coordinate {
                x: site2[0] as f64,
                y: site2[1] as f64
            })
        );
        println!(
            "site2 dist:{:?}",
            c.euclidean_distance(&geo::Line::new(
                geo::Coordinate {
                    x: site3[0] as f64,
                    y: site3[1] as f64
                },
                geo::Coordinate {
                    x: site3[2] as f64,
                    y: site3[3] as f64
                }
            ),)
        );

        let d_aabb = boostvoronoi::visual_utils::Aabb2::<i64, f64>::new_from_i32(0, 0, 800, 600);
        let mut s_aabb = boostvoronoi::visual_utils::Aabb2::<i64, f64>::default();
        s_aabb.update_i64(site1[0], site1[1]);
        s_aabb.update_i64(site2[0], site2[1]);
        s_aabb.update_i64(site3[0], site3[1]);
        s_aabb.update_i64(site3[2], site3[3]);
        s_aabb.update_f64(c1[0], c1[1]);
        s_aabb.update_f64(c2[0], c2[1]);
        let mut affine =
            boostvoronoi::visual_utils::SimpleAffine::<i64, f64>::new(&s_aabb, &d_aabb).unwrap();
        // flip y
        affine.scale = [affine.scale[0], -affine.scale[1]];
        affine.zoom(0.9);
        let affine = affine;

        offs.borrow().begin();

        let point_f = |x: f64, y: f64| {
            let xy = affine.transform(x, y);
            draw_circle(xy[0], xy[1], 2.0);
        };
        let point_i = |x: i64, y: i64| {
            point_f(x as f64, y as f64);
        };
        let line_f = |x1: f64, y1: f64, x2: f64, y2: f64| {
            let xy1 = affine.transform(x1, y1);
            let xy2 = affine.transform(x2, y2);
            draw_line(xy1[0] as i32, xy1[1] as i32, xy2[0] as i32, xy2[1] as i32);
        };

        let circle_f = |x: f64, y: f64, r: f64| {
            let xy = affine.transform(x, y);
            let r = affine.scale(r);
            draw_circle(xy[0], xy[1], r)
        };

        let line_i = |x1: i64, y1: i64, x2: i64, y2: i64| {
            point_i(x1, y1);
            point_i(x2, y2);
            line_f(x1 as f64, y1 as f64, x2 as f64, y2 as f64);
        };

        set_line_style(LineStyle::Solid, 1);

        let r1 = (site1[0] as f64 - c1[0], site1[1] as f64 - c1[1]);
        let r1 = (r1.0 * r1.0 + r1.1 * r1.1).sqrt();
        let r2 = (site1[0] as f64 - c2[0], site1[1] as f64 - c2[1]);
        let r2 = (r2.0 * r2.0 + r2.1 * r2.1).sqrt();
        set_draw_color(Color::Red);
        point_f(c1[0], c1[1]);
        circle_f(c1[0], c1[1], r1);
        set_draw_color(Color::Cyan);
        point_f(c2[0], c2[1]);
        circle_f(c2[0], c2[1], r2);
        set_draw_color(Color::Black);
        point_i(site1[0], site1[1]);
        set_draw_color(Color::Blue);
        point_i(site2[0], site2[1]);
        set_draw_color(Color::Green);
        line_i(site3[0], site3[1], site3[2], site3[3]);
        set_draw_color(Color::DarkGreen);
        point_i(site3[0], site3[1]);

        offs.borrow().end();
    }
    app.run().unwrap();
}
