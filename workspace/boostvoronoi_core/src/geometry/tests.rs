#[cfg(feature = "nalgebra")]
#[test]
/// test nalgebra points and lines
fn nalgebra_from_intoiter() -> Result<(), crate::BvError> {
    use nalgebra::Point2;

    let v = vec![
        Point2::new(10, 11),
        Point2::new(0, 100),
        Point2::new(10, 11),
        Point2::new(0, 100),
    ];
    let l = vec![
        [Point2::new(10, 11), Point2::new(1, 100)],
        [Point2::new(10, 11), Point2::new(32, 100)],
    ];

    let _ = crate::builder::Builder::<i32, f32>::default()
        .with_vertices(v)?
        .with_segments(l)?;
    Ok(())
}

#[cfg(feature = "cgmath")]
#[test]
/// test cgmath points and lines
fn cgmath_from_intoiter() -> Result<(), crate::BvError> {
    use cgmath::Point2;

    let v = vec![
        Point2::new(10, 11),
        Point2::new(0, 100),
        Point2::new(10, 11),
        Point2::new(0, 100),
    ];
    let l = vec![
        [Point2::new(10, 11), Point2::new(1, 100)],
        [Point2::new(10, 11), Point2::new(32, 100)],
    ];

    let _ = crate::builder::Builder::<i32, f32>::default()
        .with_vertices(v)?
        .with_segments(l)?;
    Ok(())
}

#[cfg(feature = "glam")]
#[test]
/// test glam points and lines
fn glam_from_intoiter() -> Result<(), crate::BvError> {
    use glam::IVec2;

    let v = vec![
        IVec2::new(10, 11),
        IVec2::new(0, 100),
        IVec2::new(10, 11),
        IVec2::new(0, 100),
    ];
    let l = vec![
        [IVec2::new(10, 11), IVec2::new(1, 100)],
        [IVec2::new(10, 11), IVec2::new(32, 100)],
    ];

    let _ = crate::builder::Builder::<i32, f32>::default()
        .with_vertices(v)?
        .with_segments(l)?;
    Ok(())
}

#[cfg(feature = "mint")]
#[test]
/// test mint points and lines
fn mint_from_intoiter() -> Result<(), crate::BvError> {
    use mint::Point2;

    let v = vec![
        Point2 { x: 10, y: 11 },
        Point2 { x: 0, y: 100 },
        Point2 { x: 10, y: 11 },
        Point2 { x: 0, y: 100 },
    ];
    let l = vec![
        [Point2 { x: 10, y: 11 }, Point2 { x: 1, y: 100 }],
        [Point2 { x: 10, y: 11 }, Point2 { x: 32, y: 100 }],
    ];

    let _ = crate::builder::Builder::<i32, f32>::default()
        .with_vertices(v)?
        .with_segments(l)?;
    Ok(())
}

#[cfg(feature = "geo")]
#[test]
/// test geo points and lines
fn geo_from_intoiter() -> Result<(), crate::BvError> {
    use geo::Line;
    use geo::Point;

    let v = vec![
        Point::new(10, 11),
        Point::new(0, 100),
        Point::new(10, 11),
        Point::new(0, 100),
    ];
    let l1 = vec![
        [Point::new(10, 11), Point::new(1, 100)],
        [Point::new(10, 11), Point::new(32, 100)],
    ];
    let l2 = vec![
        Line::from([(10, 11), (1, 100)]),
        Line::from([(10, 11), (32, 100)]),
    ];

    let _ = crate::builder::Builder::<i32, f32>::default()
        .with_vertices(v)?
        .with_segments(l1)?
        .with_segments(l2)?;
    Ok(())
}

#[test]
/// test primitive array points and lines
fn array_from_intoiter() -> Result<(), crate::BvError> {
    let v = vec![[10, 11], [0, 100], [10, 11], [0, 100]];
    let l = vec![[10, 11, 1, 100], [10, 11, 32, 100]];

    let _ = crate::builder::Builder::<i32, f32>::default()
        .with_vertices(v)?
        .with_segments(l)?;

    Ok(())
}
