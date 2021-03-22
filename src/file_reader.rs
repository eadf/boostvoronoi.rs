// Boost.Polygon library detail/robust_fpt.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history.

// Ported from C++ boost 1.75.0 to Rust in 2020 by Eadf (github.com/eadf)

use crate::BvError;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Neg;
use std::path::Path;

#[derive(Debug)]
enum InputData<T>
where
    T: super::InputType + Neg<Output = T>,
{
    Number(usize),
    Point(super::Point<T>),
    Line(super::Line<T>),
}

#[derive(Eq, PartialEq, Debug)]
enum StateMachine {
    StartingPoints,
    ExpectPoints,
    StartingLines,
    ExpectLines,
    Ended,
}

fn line_to_data<I1, F1>(line: &str) -> Option<InputData<I1>>
where
    I1: super::InputType + Neg<Output = I1>,
    F1: super::OutputType + Neg<Output = F1>,
{
    let line = line.split(' ').collect::<Vec<&str>>();
    //println!("line split: {:?}", line);
    match line.len() {
        1 => {
            if let Ok(n) = line[0].parse::<usize>(){
                return Some(InputData::Number(n));
            } else {
                println!("failed to parse {}, ignoring line", line[0]);
            }
        }
        2 => {
            if let Ok(x1) = line[0].parse::<i32>() {
                if let Ok(y1) = line[1].parse::<i32>() {
                    return Some(InputData::Point(super::Point::<I1> {
                        x: super::TypeConverter::<I1, F1>::i32_to_i1(x1),
                        y: super::TypeConverter::<I1, F1>::i32_to_i1(y1),
                    }));
                } else {
                    println!("failed to parse {}, ignoring line", line[1]);
                }
            } else {
                println!("failed to parse {}, ignoring line", line[0]);
            }
        }
        4 => {
            if let Ok(x1) = line[0].parse::<i32>() {
                if let Ok(y1) = line[1].parse::<i32>() {
                    if let Ok(x2) = line[2].parse::<i32>() {
                        if let Ok(y2) = line[3].parse::<i32>() {
                            return Some(InputData::Line(super::Line::<I1>::from([
                                super::TypeConverter::<I1, F1>::i32_to_i1(x1),
                                super::TypeConverter::<I1, F1>::i32_to_i1(y1),
                                super::TypeConverter::<I1, F1>::i32_to_i1(x2),
                                super::TypeConverter::<I1, F1>::i32_to_i1(y2),
                            ])));
                        } else {
                            println!("failed to parse {}, ignoring line", line[3]);
                        }
                    } else {
                        println!("failed to parse {}, ignoring line", line[2]);
                    }
                } else {
                    println!("failed to parse {}, ignoring line", line[1]);
                }
            } else {
                println!("failed to parse {}, ignoring line", line[0]);
            }
        }
        _ => println!("failed to parse {:?}, ignoring line", line),
    }
    None
}

/// Reads an example file in the file format used by C++ boost voronoi
/// [number of points]
/// [X] [Y] (repeats)
/// [number of lines]
/// [X1] [Y1] [X2] [Y2](repeats)
/// This entire module is implemented in about 20 lines of code in C++ boost :/
#[allow(clippy::type_complexity)]
pub fn read_boost_input_file<I1, F1>(
    filename: &Path,
) -> Result<(Vec<super::Point<I1>>, Vec<super::Line<I1>>), BvError>
where
    I1: super::InputType + Neg<Output = I1>,
    F1: super::OutputType + Neg<Output = F1>,
{
    let mut points = Vec::<super::Point<I1>>::default();
    let mut lines = Vec::<super::Line<I1>>::default();
    let mut state = StateMachine::StartingPoints;
    let mut expected_points = 0;
    let mut expected_lines = 0;

    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            //println!("got {:?}, state:{:?}", line, state);
            if let Some(data) = line_to_data::<I1, F1>(&line) {
                if state == StateMachine::StartingPoints {
                    if let InputData::Number(n) = data {
                        expected_points = n;
                        if expected_points == 0 {
                            state = StateMachine::StartingLines
                        } else {
                            state = StateMachine::ExpectPoints
                        }
                        continue;
                    } else {
                        println!(
                            "#{}: can't read line {}. state:{:?} ignoring line",
                            index + 1,
                            line,
                            state
                        );
                        break;
                    }
                } else if state == StateMachine::ExpectPoints {
                    if expected_points > points.len() {
                        if let InputData::Point(super::Point::<I1> { x, y }) = data {
                            points.push(super::Point { x, y });
                            if expected_points == points.len() {
                                state = StateMachine::StartingLines;
                            }
                        } else {
                            println!(
                                "#{}: can't read line {}. state:{:?} ignoring line",
                                index + 1,
                                line,
                                state
                            );
                            break;
                        }
                        continue;
                    } else {
                        println!(
                            "#{}: Got too many points {}. state:{:?} ignoring it",
                            index + 1,
                            line,
                            state
                        );
                        break;
                    }
                } else if state == StateMachine::StartingLines {
                    if let InputData::Number(n) = data {
                        expected_lines = n;
                        state = StateMachine::ExpectLines;
                        continue;
                    } else {
                        println!(
                            "#{}: can't read line {}. state:{:?} ignoring line",
                            index + 1,
                            line,
                            state
                        );
                        break;
                    }
                } else if state == StateMachine::ExpectLines {
                    if expected_lines > lines.len() {
                        if let InputData::Line(a_line) = data {
                            lines.push(a_line);
                            #[allow(unused_assignments)]
                            if expected_lines == lines.len() {
                                state = StateMachine::Ended;
                                break;
                            }
                            continue;
                        } else {
                            println!(
                                "#{}: can't read line {} state:{:?} ignoring line",
                                index + 1,
                                line,
                                state
                            );
                            break;
                        }
                    } else {
                        println!(
                            "#{}: Got too many lines {}. state:{:?} ignoring line",
                            index + 1,
                            line,
                            state
                        );
                        break;
                    }
                }
            };
            println!(
                "#{}: can't parse line {}. state:{:?} ignoring line",
                index + 1,
                line,
                state
            );
            break;
        }
    }
    if expected_lines < lines.len() {
        println!(
            "Expected to get {} lines, got {}",
            expected_lines,
            lines.len()
        );
    }
    if expected_points < points.len() {
        println!(
            "Expected to get {} points, got {}",
            expected_points,
            points.len()
        );
    }
    Ok((points, lines))
}
