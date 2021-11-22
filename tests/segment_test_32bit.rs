use boostvoronoi::prelude::*;

type I = i32;
type F = f32;

//#[ignore]
#[test]
// this test crashed once, but seem to be work now???
fn segment_32bit_1() -> Result<(), BvError> {
    let _output = {
        let points: [[I; 2]; 1] = [[428, 263]];
        let segments: [[I; 4]; 6] = [
            [0, 0, 100, 0],
            [100, 0, 100, 100],
            [100, 100, 0, 100],
            [0, 100, 0, 0],
            [40, 50, 60, 50],
            [204, 40, 136, 294],
        ];

        Builder::<I, F>::default()
            .with_vertices(points.iter())?
            .with_segments(segments.iter())?
            .build()?
    };
    Ok(())
}
