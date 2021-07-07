use boostvoronoi::builder as VB;
use boostvoronoi::file_reader as FR;
use boostvoronoi::BvError;
use std::io::{BufReader, Cursor};

type I = i64;
type F = f64;

/// This example will fail, something is wrong with the beach-line ordering
fn main() -> Result<(), BvError> {
    let input_list = [
        r#"0
3
-5138 -5149 -5038 -5142
-5042 -5069 -5165 -5162
-5011 -5195 -5404 -5134
"#,
        r#"0
3
-5205 -5210 -5095 -5152
-5166 -5197 -5099 -5209
-5029 -5002 -5500 -5319
"#,
        r#"0
3
759 -242 631 128
189 -303 843 693
-911 -920 921 853
"#,
        r#"0
3
580 -833 552 -566
-671 955 604 -936
535 -110 412 -549
"#,
        r#"0
3
386 -353 -252 -451
921 -884 846 922
-845 -98 35 -103
"#,
        r#"0
3
963 -74 -944 707
694 281 853 211
326 220 803 441
"#,
        r#"0
3
415 -54 955 703
976 38 -916 -467
909 424 962 401
"#,
        r#"0
3
365 113 741 366
768 -67 601 187
-814 662 817 -285
"#,
        r#"0
3
673 903 -985 -362
-238 248 -179 453
-136 323 -90 419
"#,
    ];

    for input in input_list {
        let mut vb = VB::Builder::<I, F>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I, _>(br)?;

        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        //panic!();
        let _ = vb.build()?;
    }

    Ok(())
}
