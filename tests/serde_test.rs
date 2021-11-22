#[cfg(feature = "serde")]
#[test]
fn serde_test_1() -> Result<(), boostvoronoi::BvError> {
    use boostvoronoi::prelude::*;
    use std::io::{BufReader, Cursor};

    type I = i32;
    type F = f64;

    let output: boostvoronoi::SyncDiagram<F> = {
        let input = r#"1
3 -3
1
0 0 2 -7
"#;
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = boostvoronoi::read_boost_input_buffer::<I, _>(br)?;
        Builder::<I, F>::default()
            .with_vertices(points.iter())?
            .with_segments(segments.iter())?
            .build()?
            .into()
    };
    let output_serde: boostvoronoi::SyncDiagram<F> =
        serde_json::from_str(serde_json::to_string(&output).unwrap().as_str()).unwrap();
    assert_eq!(output_serde.vertices().len(), output.vertices().len());
    Ok(())
}
