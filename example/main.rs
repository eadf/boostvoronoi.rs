use boostvoronoi::builder as VB;
use boostvoronoi::diagram as VD;
use boostvoronoi::BvError;
use num_traits::float::FloatConst;
//use boostvoronoi::InputType;

type I = i64;
type F = f64;

const EXTERNAL: VD::ColorType = 1;

#[allow(dead_code)]
fn almost_equal(x1: F, x2: F, y1: F, y2: F) -> bool {
    let delta = 0.001;
    assert!(F::abs(x1 - x2) < delta, "{} != {}", x1, x2);
    assert!(F::abs(y1 - y2) < delta, "{} != {}", y1, y2);
    (F::abs(x1 - x2) < delta) && (F::abs(y1 - y2) < delta)
}

#[allow(dead_code)]
fn circle(cx: I, cy: I, r: F, n: I) -> Vec<[I; 4]> {
    let i_2_f = |x| num::cast::<I, f64>(x).unwrap();
    let f_2_i = |x| num::cast::<f64, I>(x).unwrap();

    let d_angle = f64::PI() * 2.0 / num::cast::<I, f64>(n).unwrap();
    let mut angle = 0_f64;
    let mut last: [I; 2] = [cx + f_2_i(angle.cos() * r), cy + f_2_i(angle.sin() * r)];
    let mut rv = Vec::<[I; 4]>::new();
    for i in 1..n {
        angle = d_angle * i_2_f(i);
        let next = [cx + f_2_i(angle.cos() * r), cy + f_2_i(angle.sin() * r)];
        rv.push([last[0], last[1], next[0], next[1]]);
        last = next;
    }
    rv
}

/// This example will fail, something is wrong with the beach-line ordering
fn main() -> Result<(), BvError> {
    #[allow(unused_variables)]
    let output = {
        let points: [[I; 2]; 0] = [];
        // problematic polygon from https://github.com/boostorg/polygon/issues/43
        // not even 1.75 C++ boost handles this correctly
        let segments: [[I; 4]; 6] = [
            [0, 10000000, 700000, 1],
            [700000, 1, 700000, 9000000],
            [700000, 9000000, 9100000, 9000000],
            [9100000, 9000000, 9100000, 0],
            [9100000, 0, 10000000, 10000000],
            [10000000, 10000000, 0, 10000000],
        ];
        let segments_1: [[I; 4]; 2] = [
            [35058881, -35000000, 31058881, -35000000],
            [31058881, -35000000, 25058881, -35000001],
        ];
        let segments_1: [[I; 4]; 2] = [
            [35058881, -35000000, 35058881, -25732145],
            [35058881, -25732145, 35058882, -19586070],
        ];

        let segments_1: [[I; 4]; 4] = [
            [35058881, -35000000, 35058881, -25732145],
            [35058881, -25732145, 35058881, -19586070],
            [35058881, -19586070, -31657205, -35000000],
            [-31657205, -35000000, 35058881, -35000000],
        ];
        let segments_1: [[I; 4]; 2] = [
            [35058881, -35000000, 31058881, -35000000],
            [31058881, -35000000, 25058881, -35000001],
        ];

        let points: [[I; 2]; 0] = [];
        let segments: [[I; 4]; 3] = [
            [1403829871, 74, 1403829871, 275],
            [1403829871, 275, 1403829741, 275],
            [1403829741, 275, 1403829744, 73],
        ];

        let points: [[I; 2]; 0] = [];
        let segments: [[I; 4]; 20] = [
            [100, 0, 95, 30],
            [95, 30, 80, 58],
            [80, 58, 58, 80],
            [58, 80, 30, 95],
            [30, 95, 0, 99],
            [0, 99, -30, 95],
            [-30, 95, -58, 80],
            [-58, 80, -80, 58],
            [-80, 58, -95, 30],
            [-95, 30, -99, 0],
            [-99, 0, -95, -30],
            [-95, -30, -80, -58],
            [-80, -58, -58, -80],
            [-58, -80, -30, -95],
            [-30, -95, 0, -99],
            [0, -99, 30, -95],
            [30, -95, 58, -80],
            [58, -80, 80, -58],
            [80, -58, 95, -30],
            [95, -30, 100, 0],
        ];

        let points: [[I; 2]; 0] = [];
        let segments: [[I; 4]; 23] = [
            [-12, 4, -12, -4],
            [-12, -4, -8, -4],
            [-8, -4, -8, -1],
            [-8, -1, -9, 0],
            [-9, 0, -8, 1],
            [-8, 1, -8, 4],
            [-8, 4, -12, 4],
            [-4, 4, -4, -4],
            [-4, -4, 0, -4],
            [0, -4, 0, 4],
            [0, 4, -4, 4],
            [4, 4, 4, -4],
            [4, -4, 8, -4],
            [8, -4, 8, 4],
            [8, 4, 4, 4],
            [-4, -8, -8, -8],
            [-8, -8, -8, -12],
            [-8, -12, -4, -12],
            [-4, -12, -4, -16],
            [-4, -16, -8, -16],
            [0, -8, 2, -8],
            [2, -8, 4, -8],
            [2, -8, 2, -16],
        ];

        let points: [[I; 2]; 0] = [];
        let segments: [[I; 4]; 12] = [
            [-1, 10, 1, 10],
            [10, -1, 10, 1],
            [-1, -10, 1, -10],
            [-10, -1, -10, 1],
            [-6, 8, -2, 11],
            [-8, 6, -11, 2],
            [6, 8, 2, 11],
            [8, 6, 11, 2],
            [6, -8, 2, -11],
            [8, -6, 11, -2],
            [-6, -8, -2, -11],
            [-8, -6, -11, -2],
        ];

        let segments: [[I; 4]; 6] = [
            [0, 10000000, 700000, 1],
            [700000, 1, 700000, 9000000],
            [700000, 9000000, 9100000, 9000000],
            [9100000, 9000000, 9100000, 0],
            [9100000, 0, 10000000, 10000000],
            [10000000, 10000000, 0, 10000000],
        ];
        let _v = VB::to_points::<I, I>(&points);
        let _s = VB::to_segments::<I, I>(&segments);
        println!("-------\n{}", points.len());
        for p in points.iter() {
            println!("{} {}", p[0], p[1]);
        }
        println!("{}", segments.len());
        for s in segments.iter() {
            println!("{} {} {} {}", s[0], s[1], s[2], s[3]);
        }
        println!("-------");
        println!("int INPUT_PTS[{}][2] = {{", points.len());
        for p in points.iter() {
            print!("{{{},{}}},", p[0], p[1]);
        }
        println!("}};");
        println!("int INPUT_SGS[{}][4] = {{", segments.len());
        for s in segments.iter() {
            print!("{{{},{},{},{}}},", s[0], s[1], s[2], s[3]);
        }
        println!("}};");
        println!("-------");
        let mut vb = VB::Builder::<I, F>::default();
        vb.with_vertices(_v.iter())?;
        vb.with_segments(_s.iter())?;
        vb.build()?
    };
    println!();
    for (i, v) in output.vertices().iter().enumerate() {
        println!(
            "vertex #{} contains a point: ({:.12}, {:.12}) ie:{:?}",
            i,
            v.get().x(),
            v.get().y(),
            v.get().get_incident_edge()?.0
        );
    }

    println!("cells:{}", output.cells().len());
    println!("vertices:{}", output.vertices().len());
    //println!("edges:{}", output.edges().len());
    output.color_exterior_edges(EXTERNAL);

    println!("edges:{}", output.edges().len());
    for (i, e) in output.edges().iter().enumerate() {
        let e = e.get();
        println!("Edge:#{}=>{:?}", e.id().0, &e);
        assert_eq!(i, e.id().0);
    }
    Ok(())
}
