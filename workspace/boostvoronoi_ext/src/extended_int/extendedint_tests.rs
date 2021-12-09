#[test]
fn extended_int_test_1() {
    let aa = 830003450000_f64;
    let bb = 320034543430_f64;
    let cc = 32_f64;
    let a = crate::extended_int::ExtendedInt::from(aa as i64);
    let b = crate::extended_int::ExtendedInt::from(bb as i64);
    let c = crate::extended_int::ExtendedInt::from(cc as i64);
    println!("a:{:?} d():{}", &a, a.d());
    println!("b:{:?} d():{}", &b, b.d());
    println!("c:{:?} d():{}", &c, c.d());

    let mut r = &a * &b;
    for _i in 0..32 {
        r = &r + &a + &b;
    }
    for _i in 0..32 {
        r = &r - &a - &b;
    }
    for _i in 0..32 {
        r = &r + &a + &b;
    }
    for _i in 0..32 {
        r = &r - &a - &b;
    }
    println!("r:{:?} d():{}", r, r.d());
    approx::assert_ulps_eq!(r.d(), aa * bb);
}

#[test]
fn extended_int_test_2() {
    let aa = -830003450000_f64;
    let bb = -320034543430_f64;
    let cc = -32_f64;
    let a = crate::extended_int::ExtendedInt::from(aa as i64);
    let b = crate::extended_int::ExtendedInt::from(bb as i64);
    let c = crate::extended_int::ExtendedInt::from(cc as i64);
    println!("a:{:?} d():{}", &a, a.d());
    println!("b:{:?} d():{}", &b, b.d());
    println!("c:{:?} d():{}", &c, c.d());

    let mut r = &a * &b;
    for _i in 0..32 {
        r = &r + &a + &b;
    }
    for _i in 0..32 {
        r = &r - &a - &b;
    }
    for _i in 0..32 {
        r = &r + &a + &b;
    }
    for _i in 0..32 {
        r = &r - &a - &b;
    }
    println!("r:{:?} d():{}", r, r.d());
    approx::assert_ulps_eq!(r.d(), aa * bb);
}
