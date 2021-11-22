#[cfg(feature = "geo")]
#[test]
/// Test geo type conversions 32 bit
fn geo_conversion_test_32() -> Result<(), boostvoronoi::BvError> {
    use boostvoronoi::geo;
    use boostvoronoi::prelude::*;

    type I = i32;
    type F = f32;

    let output = Builder::<I, F>::default()
        // `boostvoronoi::Point::from()` accepts `geo::Coordinate` and `geo::Point`
        .with_vertices([geo::Coordinate::from([3_i32, -3])].iter())?
        .with_vertices([geo::Point::new(4_i32, -3)].iter())?
        // `boostvoronoi::Line::from()` accepts `[geo::Coordinate;2]`, `[geo::Point;2]` and `geo::Line`
        .with_segments(
            [[
                geo::Coordinate::from([0_i32, 0]),
                geo::Coordinate::from([2, -7]),
            ]]
            .iter(),
        )?
        .with_segments([[geo::Point::new(0_i32, 0), geo::Point::new(2, 7)]].iter())?
        .with_segments([geo::Line::from([(0_i32, 0), (-6, 7)])].iter())?
        .build()?;
    for v in output.vertices().iter().map(|v| v.get()) {
        // geo::Coordinate::from(&boostvoronoi::Vertex)
        let _geo_coordinate_from_ref = geo::Coordinate::<F>::from(&v);
        // geo::Coordinate::from(boostvoronoi::Vertex)
        let _geo_coordinate = geo::Coordinate::<F>::from(v.clone());
        // geo::Point::from(&boostvoronoi::Vertex)
        let _geo_point_from_ref = geo::Point::<F>::from(&v);
        // geo::Point::from(&boostvoronoi::Vertex)
        let _geo_point = geo::Point::<F>::from(v);
    }
    Ok(())
}

#[cfg(feature = "geo")]
#[test]
/// Test geo type conversions 64 bit
fn geo_conversion_test_64() -> Result<(), boostvoronoi::BvError> {
    use boostvoronoi::geo;
    use boostvoronoi::prelude::*;

    type I = i64;
    type F = f64;

    let output = Builder::<I, F>::default()
        // `boostvoronoi::Point::from()` accepts `geo::Coordinate` and `geo::Point`
        .with_vertices([geo::Coordinate::from([3_i64, -3])].iter())?
        .with_vertices([geo::Point::new(4_i64, -3)].iter())?
        // `boostvoronoi::Line::from()` accepts `[geo::Coordinate;2]`, `[geo::Point;2]` and `geo::Line`
        .with_segments(
            [[
                geo::Coordinate::from([0_i64, 0]),
                geo::Coordinate::from([2, -7]),
            ]]
            .iter(),
        )?
        .with_segments([[geo::Point::new(0_i64, 0), geo::Point::new(2, 7)]].iter())?
        .with_segments([geo::Line::from([(0_i64, 0), (-6, 7)])].iter())?
        .build()?;
    for v in output.vertices().iter().map(|v| v.get()) {
        // geo::Coordinate::from(&boostvoronoi::Vertex)
        let _geo_coordinate_from_ref = geo::Coordinate::<F>::from(&v);
        // geo::Coordinate::from(boostvoronoi::Vertex)
        let _geo_coordinate = geo::Coordinate::<F>::from(v.clone());
        // geo::Point::from(&boostvoronoi::Vertex)
        let _geo_point_from_ref = geo::Point::<F>::from(&v);
        // geo::Point::from(&boostvoronoi::Vertex)
        let _geo_point = geo::Point::<F>::from(v);
    }
    Ok(())
}
