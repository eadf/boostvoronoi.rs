use super::RobustDif;
use super::RobustFpt;

#[test]
fn robustdif_sum_1() {
    let a_: f32 = 5.;
    let b_: f32 = 4.;

    let a = RobustDif::<f32>::new_1(a_);
    let b = RobustDif::<f32>::new_1(b_);
    let s = a + b;

    assert_eq!(s.dif().fpv(), a_ + b_);
    assert_eq!(s.dif().fpv(), 9.0)
}

#[test]
fn robustdif_sum_2() {
    type F2 = f64;

    let newf = RobustFpt::<F2>::new_2;
    let newd = RobustDif::<F2>::new_from_2;
    let c_x = newd(&newf(12.0, 2.0), &newf(0.5, 13.0));
    let mut r = newd(&newf(24.0, 2.0), &newf(26.0, 2.0));
    let line_a: F2 = -2.0;
    let line_a_2 = RobustFpt::<F2>::new_1(line_a);

    assert_eq!(line_a_2.fpv(), line_a);
    dbg!(&c_x);
    dbg!(&line_a_2);
    let v = c_x * line_a_2;
    dbg!(&v);
    assert_eq!(v.dif().fpv(), (12.0 - 0.5) * -2.0);

    dbg!(&c_x, &line_a, &r);
    r += c_x * RobustFpt::<F2>::new_1(line_a);
    dbg!(&r);

    assert_eq!(r.positive().fpv(), 25.0);
    //assert_eq!(r.positive().re(), 15.0);

    assert_eq!(r.negative().fpv(), 50.0);
    //assert_eq!(r.negative().re(), 4.0);
}
