use boostvoronoi as BV;
use boostvoronoi::BvError;

//#[ignore]
#[test]
fn transform_test_1() -> Result<(), BvError> {
    let mut aabb_source = BV::Aabb2::<f32>::default();
    let mut aabb_dest = BV::Aabb2::<f32>::default();

    // source is (0,0)-(1,1)
    aabb_source.update_vertex(0., 0.);
    aabb_source.update_vertex(1., 1.);

    // dest is (1,1)-(2,2)
    aabb_dest.update_vertex(1., 1.);
    aabb_dest.update_vertex(2., 2.);

    let transform = BV::SimpleAffine::new::<i32>(&aabb_source, &aabb_dest)?;
    assert_eq!(transform.transform(0., 0.), [1., 1.]);
    assert_eq!(transform.transform(1., 1.), [2., 2.]);
    assert_eq!(transform.transform(0., 1.), [1., 2.]);
    assert_eq!(transform.transform(1., 0.), [2., 1.]);

    Ok(())
}

#[test]
fn transform_test_2() -> Result<(), BvError> {
    let mut aabb_source = BV::Aabb2::<f32>::default();
    let mut aabb_dest = BV::Aabb2::<f32>::default();

    // source is (-100,-100)-(100,100)
    aabb_source.update_vertex(-100., -100.);
    aabb_source.update_vertex(100., 100.);

    // dest is (0,0)-(800,800.)
    aabb_dest.update_vertex(0., 0.);
    aabb_dest.update_vertex(800., 800.);

    let transform = BV::SimpleAffine::new::<i32>(&aabb_source, &aabb_dest)?;
    //println!("Affine:{:?}", transform);

    assert_eq!(transform.transform(-100., -100.), [0., 0.]);
    assert_eq!(transform.transform(100., 100.), [800., 800.]);

    Ok(())
}
