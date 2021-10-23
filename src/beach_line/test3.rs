#![allow(unused_imports)]
#![allow(dead_code)]
use super::super::beach_line as VB;
use super::super::geometry::Point;
use super::super::predicate as VP;
use super::super::site_event as VSE;
use super::super::InputType;
use super::super::OutputType;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::ops::Neg;
use std::path::Path;

lazy_static! {
    static ref RE_PREDICATE: Regex =
    #[allow(unused_braces)] // rustfmt adds those for no reason
    { Regex::new(r"(?P<node1>L.+), (?P<node2>L.+)\)==(?P<result>.+)").unwrap() };
    static ref RE_NODE: Regex =
    #[allow(unused_braces)] // rustfmt adds those for no reason
    { Regex::new(r"L:(?P<left>.+),R:(?P<right>.+)").unwrap() };
    static ref RE_NODE_SINGLE: Regex = {
        Regex::new(r"#(?P<si>\d+)\((?P<x>-?\d+),(?P<y>-?\d+)\),ii:(?P<ii>-?\d+),f:(?P<f>\d+)").unwrap()
    };
    static ref RE_NODE_DOUBLE: Regex = {
        Regex::new(r"#(?P<si>\d+)\((?P<x1>-?\d+),(?P<y1>-?\d+)\)-\((?P<x2>-?\d+),(?P<y2>-?\d+)\),ii:(?P<ii>-?\d+),f:(?P<f>\d+)").unwrap()
    };
    static ref RE_NODE_INVERS: Regex = {
        Regex::new(r"#(?P<si>\d+)\((?P<x1>-?\d+),(?P<y1>-?\d+)\)¿\((?P<x2>-?\d+),(?P<y2>-?\d+)\),ii:(?P<ii>-?\d+),f:(?P<f>\d+)").unwrap()
    };
}

fn coord<I: InputType>(x: I, y: I) -> Point<I> {
    Point::<I> { x, y }
}

fn parse_site<I, F>(site_input: &str) -> VSE::SiteEvent<I, F>
where
    I: InputType + std::str::FromStr,
    <I as std::str::FromStr>::Err: std::fmt::Debug,
    F: OutputType,
{
    let site = if let Some(caps) = RE_NODE_SINGLE.captures(site_input) {
        let ii = (&caps)["ii"].parse::<usize>().unwrap();
        print!(
            "si:{},x:{},y:{},ii:{},f:{}",
            &caps["si"], &caps["x"], &caps["y"], ii, &caps["f"]
        );
        let mut site = VSE::SiteEvent::<I, F>::new_2(
            coord(
                (&caps)["x"].parse::<I>().unwrap(),
                (&caps)["y"].parse::<I>().unwrap(),
            ),
            ii,
        );
        site.set_flags((&caps)["f"].parse::<u32>().unwrap());
        site.set_sorted_index((&caps)["si"].parse::<usize>().unwrap());
        site
    } else if let Some(caps) = RE_NODE_DOUBLE.captures(site_input) {
        let ii = (&caps)["ii"].parse::<usize>().unwrap();
        print!(
            "si:{},x1:{},y1:{},x2:{},y2:{},ii:{},f:{}",
            &caps["si"], &caps["x1"], &caps["y1"], &caps["x2"], &caps["y2"], ii, &caps["f"]
        );
        let mut site = VSE::SiteEvent::<I, F>::new_3(
            coord(
                (&caps)["x1"].parse::<I>().unwrap(),
                (&caps)["y1"].parse::<I>().unwrap(),
            ),
            coord(
                (&caps)["x2"].parse::<I>().unwrap(),
                (&caps)["y2"].parse::<I>().unwrap(),
            ),
            ii,
        );
        site.set_flags((&caps)["f"].parse::<u32>().unwrap());
        site.set_sorted_index((&caps)["si"].parse::<usize>().unwrap());
        site
    } else if let Some(caps) = RE_NODE_INVERS.captures(site_input) {
        let ii = (&caps)["ii"].parse::<usize>().unwrap();
        /*print!(
            "si:{},x1:{},y1:{},x2:{},y2:{},ii:{},f:{}",
            &caps["si"], &caps["x1"], &caps["y1"], &caps["x2"], &caps["y2"], ii, &caps["f"]
        );*/
        let mut site = VSE::SiteEvent::<I, F>::new_3(
            coord(
                (&caps)["x1"].parse::<I>().unwrap(),
                (&caps)["y1"].parse::<I>().unwrap(),
            ),
            coord(
                (&caps)["x2"].parse::<I>().unwrap(),
                (&caps)["y2"].parse::<I>().unwrap(),
            ),
            ii,
        );
        site.set_flags((&caps)["f"].parse::<u32>().unwrap());
        site.set_sorted_index((&caps)["si"].parse::<usize>().unwrap());
        site
    } else {
        panic!(
            "All re_single & re_double, re_inverse failed for {}",
            site_input
        )
    };
    let result = format!("{:?}", site);
    //println!("\n{} -> {}", site_input, result);
    assert_eq!(site_input, result);
    site
}

fn parse_node<I, F>(node: &str) -> VB::BeachLineNodeKey<I, F>
where
    I: InputType + std::str::FromStr,
    <I as std::str::FromStr>::Err: std::fmt::Debug,
    F: OutputType,
{
    let caps = RE_NODE.captures(node).unwrap();
    println!("LEFT:{}", &caps["left"]);
    let site1 = parse_site(&caps["left"]);

    println!("RIGHT:{}", &caps["right"]);
    let site2 = parse_site(&caps["right"]);
    assert_eq!(format!("{:?}", site2), &caps["right"]);
    println!();
    let key = VB::BeachLineNodeKey::<I, F>::new_2(site1, site2);
    assert_eq!(format!("{:?}", key).replace(", id=0", ""), node);
    key
}

#[ignore]
#[test]
/// This test is massive, 9+ megs of node_comparison_predicate() -> disabled by default, but it works
fn beachline_multiple_2() -> io::Result<()> {
    let file = File::open(Path::new("src/beach_line/node_comparisons.txt"))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("new line:{}", line);
        let mut found_any = false;

        for cap in RE_PREDICATE.captures_iter(line.as_str()) {
            let node1 = parse_node::<i64, f64>(&cap["node1"]);
            let node2 = parse_node::<i64, f64>(&cap["node2"]);
            let result =
                VP::NodeComparisonPredicate::<i64, f64>::node_comparison_predicate(&node1, &node2);
            println!("Result:{}", &cap["result"]);
            let expected_result = (&cap)["result"].parse::<bool>().unwrap();
            println!("result:{}, expected_result:{}", result, expected_result);
            assert_eq!(result, expected_result);
            found_any = true;
        }
        if line.len() > 1 {
            assert!(found_any);
        }
    }
    Ok(())
}
