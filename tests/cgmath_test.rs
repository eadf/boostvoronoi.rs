#[cfg(feature = "cgmath")]
#[test]
/// Test cgmath type conversions 32 bit
fn cgmath_conversion_test_32() -> Result<(), boostvoronoi::BvError> {
    use boostvoronoi::cgmath::Point2;
    use boostvoronoi::prelude::*;

    type I = i32;
    type F = f32;

    let output = Builder::<I, F>::default()
        .with_vertices([Point2::new(3_i32, -3)].iter())?
        .with_segments([[Point2::new(0_i32, 0), Point2::new(2, -7)]].iter())?
        .build()?;
    for v in output.vertices().iter().map(|v| v.get()) {
        let _cgmath_vertice = Point2::<F>::from(&v);
        let _cgmath_vertice = Point2::<F>::from(v);
    }
    Ok(())
}

#[cfg(feature = "cgmath")]
#[test]
/// Test cgmath type conversions 64 bit
fn cgmath_conversion_test_64() -> Result<(), boostvoronoi::BvError> {
    use boostvoronoi::cgmath::Point2;
    use boostvoronoi::prelude::*;

    type I = i64;
    type F = f64;

    let output = Builder::<I, F>::default()
        .with_vertices([Point2::new(3_i64, -3)].iter())?
        .with_segments([[Point2::new(0_i64, 0), Point2::new(2, -7)]].iter())?
        .build()?;
    for v in output.vertices().iter().map(|v| v.get()) {
        let _cgmath_vertice = Point2::<F>::from(&v);
        let _cgmath_vertice = Point2::<F>::from(v);
    }
    Ok(())
}
