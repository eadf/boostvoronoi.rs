#[cfg(feature = "mint")]
#[test]
/// Test mint type conversions
fn mint_conversion_test_32() -> Result<(), boostvoronoi::BvError> {
    use boostvoronoi::mint::Point2;
    use boostvoronoi::prelude::*;

    type I = i32;
    type F = f32;

    let output = Builder::<I, F>::default()
        .with_vertices([Point2::from([3_i32, -3])].iter())?
        .with_segments([[Point2::from([0_i32, 0]), Point2::from([2, -7])]].iter())?
        .build()?;
    for v in output.vertices().iter().map(|v| v.get()) {
        let _mint_vertice = Point2::<F>::from(&v);
        let _mint_vertice = Point2::<F>::from(v);
    }
    Ok(())
}

#[cfg(feature = "mint")]
#[test]
/// Test mint type conversions
fn mint_conversion_test_64() -> Result<(), boostvoronoi::BvError> {
    use boostvoronoi::mint::Point2;
    use boostvoronoi::prelude::*;

    type I = i64;
    type F = f64;

    let output = Builder::<I, F>::default()
        .with_vertices([Point2::from([3_i64, -3])].iter())?
        .with_segments([[Point2::from([0_i64, 0]), Point2::from([2, -7])]].iter())?
        .build()?;
    for v in output.vertices().iter().map(|v| v.get()) {
        let _mint_vertice = Point2::<F>::from(&v);
        let _mint_vertice = Point2::<F>::from(v);
    }
    Ok(())
}
