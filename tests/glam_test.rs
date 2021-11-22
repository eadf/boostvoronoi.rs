#[cfg(feature = "glam")]
#[test]
/// Test glam type conversions
fn glam_conversion_test_vec2() -> Result<(), boostvoronoi::BvError> {
    use boostvoronoi::glam::{IVec2, Vec2};
    use boostvoronoi::prelude::*;

    // At the moment, there are no glam 64bit signed integer 2d vectors so
    // `boostvoronoi::Point::from()` can only accept `glam::IVec`
    type I = i32;
    type F = f32;

    let output = Builder::<I, F>::default()
        .with_vertices([IVec2::new(3_i32, -3)].iter())?
        .with_segments([[IVec2::new(0_i32, 0), IVec2::new(2, -7)]].iter())?
        .build()?;
    for v in output.vertices().iter().map(|v| v.get()) {
        let _glam_vec2_from_ref = Vec2::from(&v);
        let _glam_vec2 = Vec2::from(v);
    }
    Ok(())
}

#[cfg(feature = "glam")]
#[test]
/// Test glam type conversions
fn glam_conversion_test_dvec2() -> Result<(), boostvoronoi::BvError> {
    use boostvoronoi::glam::{DVec2, IVec2};
    use boostvoronoi::prelude::*;

    // At the moment, there are no glam 64bit signed integer 2d vectors so
    // `boostvoronoi::Point::from()` can only accept `glam::IVec`
    type I = i32;
    type F = f64;

    let output = Builder::<I, F>::default()
        .with_vertices([IVec2::new(3_i32, -3)].iter())?
        .with_segments([[IVec2::new(0_i32, 0), IVec2::new(2, -7)]].iter())?
        .build()?;
    for v in output.vertices().iter().map(|v| v.get()) {
        let _glam_dvec2_from_ref = DVec2::from(&v);
        let _glam_dvec2 = DVec2::from(v);
    }
    Ok(())
}
