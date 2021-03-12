use boostvoronoi;
use boostvoronoi::builder::{to_points, to_segments, Builder};
use criterion::{criterion_group, criterion_main, Criterion};

type I1 = i32;
type F1 = f64;
type I2 = i64;
type F2 = f64;

#[cfg(test)]
fn bench_1(c: &mut Criterion) {

    c.bench_function("bench_1", |b| {
        b.iter({
            || {
                let (_output, _v, _s) = {
                    let points: [[I1; 2]; 45] = [
                        [303, 108],
                        [180, 257],
                        [115, 405],
                        [226, 536],
                        [177, 599],
                        [43, 709],
                        [127, 740],
                        [158, 683],
                        [593, 759],
                        [583, 672],
                        [587, 543],
                        [514, 503],
                        [408, 543],
                        [401, 642],
                        [580, 380],
                        [308, 304],
                        [260, 343],
                        [258, 240],
                        [344, 238],
                        [346, 189],
                        [198, 150],
                        [238, 59],
                        [679, 37],
                        [727, 134],
                        [733, 276],
                        [679, 327],
                        [698, 404],
                        [744, 544],
                        [764, 673],
                        [710, 770],
                        [611, 687],
                        [298, 536],
                        [244, 675],
                        [313, 618],
                        [433, 760],
                        [301, 773],
                        [127, 758],
                        [40, 644],
                        [612, 105],
                        [743, 37],
                        [581, 26],
                        [402, 20],
                        [318, 45],
                        [130, 102],
                        [98, 141],
                    ];
                    let segments: [[I1; 4]; 51] = [
                        [200, 200, 200, 400],
                        [200, 400, 400, 400],
                        [400, 400, 400, 200],
                        [400, 200, 200, 200],
                        [529, 242, 367, 107],
                        [667, 431, 464, 554],
                        [464, 554, 230, 588],
                        [230, 588, 88, 464],
                        [88, 464, 80, 236],
                        [80, 236, 178, 97],
                        [178, 97, 463, 56],
                        [463, 56, 670, 175],
                        [670, 175, 732, 346],
                        [732, 346, 735, 479],
                        [735, 479, 512, 643],
                        [512, 643, 257, 710],
                        [257, 710, 100, 615],
                        [100, 615, 36, 470],
                        [36, 470, 53, 195],
                        [53, 195, 82, 83],
                        [82, 83, 211, 30],
                        [379, 35, 614, 55],
                        [759, 140, 784, 390],
                        [734, 594, 678, 686],
                        [485, 742, 203, 745],
                        [103, 724, 12, 537],
                        [22, 425, 38, 45],
                        [43, 25, 308, 17],
                        [512, 691, 629, 758],
                        [629, 758, 643, 601],
                        [499, 455, 618, 281],
                        [618, 281, 612, 209],
                        [612, 209, 486, 125],
                        [462, 458, 361, 494],
                        [215, 498, 148, 470],
                        [453, 233, 494, 371],
                        [494, 371, 560, 262],
                        [560, 262, 563, 200],
                        [563, 200, 451, 141],
                        [451, 141, 421, 82],
                        [421, 82, 243, 111],
                        [243, 111, 145, 187],
                        [145, 187, 144, 319],
                        [144, 319, 177, 442],
                        [177, 442, 266, 484],
                        [266, 484, 336, 541],
                        [336, 541, 433, 497],
                        [433, 497, 525, 467],
                        [525, 467, 594, 427],
                        [594, 427, 617, 342],
                        [617, 342, 675, 292],
                    ];

                    let _v = to_points::<I1>(&points);
                    let _s = to_segments::<I1>(&segments);

                    let mut vb = Builder::<I1, F1, I2, F2>::new();
                    vb.with_vertices(_v.iter()).expect("bench_1");
                    vb.with_segments(_s.iter()).expect("bench_1");
                    (vb.construct().expect("bench_1"), _v, _s)
                };
            }
        })
    });
}

criterion_group!(benches1, bench_1);
criterion_main!(benches1);