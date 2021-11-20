#[allow(unused_imports)]
use boostvoronoi::diagram::Diagram;
#[allow(unused_imports)]
use boostvoronoi::{cast, BvError, InputType, OutputType};

#[allow(dead_code)]
pub fn almost_equal<F: OutputType>(x1: F, x2: F, y1: F, y2: F) -> bool {
    let delta = cast::<f64, F>(0.000001);
    assert!(F::abs(x1 - x2) < delta, "{} != {}", x1, x2);
    assert!(F::abs(y1 - y2) < delta, "{} != {}", y1, y2);

    (F::abs(x1 - x2) < delta) && (F::abs(y1 - y2) < delta)
}

#[cfg(feature = "geo")]
#[allow(dead_code)]
/// A brute force-check to see if all the vertices really are at the midpoint
/// between (at least) two segments or points. O(v*(p+s))
pub fn diagram_sanity_check<I: InputType + geo::CoordNum, F: OutputType + geo::GeoFloat>(
    diagram: &Diagram<F>,
    points: &[boostvoronoi::geometry::Point<I>],
    segments: &[boostvoronoi::geometry::Line<I>],
    delta: F,
) -> Result<(), BvError> {
    use geo::algorithm::euclidean_distance::*;

    // check that delta has a sane value
    assert!(delta.is_sign_positive() && delta <= cast::<f64, F>(0.0001));

    let coordinates: Vec<_> = points
        .iter()
        .map(|p| geo::Coordinate::<F>::from([cast::<I, F>(p.x), cast::<I, F>(p.y)]))
        .collect();
    let lines: Vec<_> = segments
        .iter()
        .map(|l| {
            geo::Line::<F>::from([
                (cast::<I, F>(l.start.x), cast::<I, F>(l.start.y)),
                (cast::<I, F>(l.end.x), cast::<I, F>(l.end.y)),
            ])
        })
        .collect();

    // this vec will contain distances of equal value, it will be cleared whenever a smaller
    // value is found. Hence the name "heap"
    let mut heap: Vec<F> = Vec::new();

    for v in diagram.vertices().iter() {
        let v = geo::Coordinate::from(&v.get());
        for l in lines.iter() {
            let distance = v.euclidean_distance(l);
            //print!("s{:?} -> v {:?} = {:?}", s, v, distance);
            if let Some(peek) = heap.first() {
                if distance <= *peek {
                    if *peek - distance > delta {
                        // this sample is smaller than anything before
                        heap.clear();
                    }
                } else if distance - *peek > delta {
                    // ignore this sample, get a new sample
                    continue;
                }
            }
            //println!();
            heap.push(distance);
        }
        for c in coordinates.iter() {
            let distance = v.euclidean_distance(c);
            //print!("s{:?} -> v {:?} = {:?}", s, v, distance);
            if let Some(peek) = heap.first() {
                if distance <= *peek {
                    if *peek - distance > delta {
                        // this sample is smaller than anything before
                        heap.clear();
                    }
                } else if distance - *peek > delta {
                    // ignore this sample, get a new sample
                    continue;
                }
            }
            //println!();
            heap.push(distance);
        }
        if heap.len() < 2 {
            let err_msg = format!(
                "Got a vertex with only one close neighbour: {:?}, dist:{:?}",
                v,
                heap.get(0)
            );

            eprintln!("{}", err_msg);
            return Err(BvError::InternalError(err_msg));
        }
        heap.clear();
    }
    Ok(())
}
