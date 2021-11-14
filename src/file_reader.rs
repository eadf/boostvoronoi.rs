// Boost.Polygon library detail/robust_fpt.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history of C++ code..

// Ported from C++ boost 1.76.0 to Rust in 2020/2021 by Eadf (github.com/eadf)

//! Utility for reading example files.

use crate::{cast, geometry, BvError};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
enum InputData<T>
where
    T: super::InputType,
{
    Number(usize),
    Point(geometry::Point<T>),
    Line(geometry::Line<T>),
}

#[derive(Eq, PartialEq, Debug)]
enum StateMachine {
    StartingPoints,
    ExpectPoints,
    StartingLines,
    ExpectLines,
}

fn line_to_data<I>(line: &str) -> Option<InputData<I>>
where
    I: super::InputType,
{
    let line = line.split(' ').collect::<Vec<&str>>();
    //tln!("line split: {:?}", line);
    match line.len() {
        1 => {
            if let Ok(n) = line[0].parse::<usize>() {
                return Some(InputData::Number(n));
            } else {
                eprintln!("failed to parse {}, ignoring line", line[0]);
            }
        }
        2 => {
            if let Ok(x1) = line[0].parse::<i32>() {
                if let Ok(y1) = line[1].parse::<i32>() {
                    return Some(InputData::Point(geometry::Point::<I> {
                        x: cast::<i32, I>(x1),
                        y: cast::<i32, I>(y1),
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
                            return Some(InputData::Line(geometry::Line::<I>::from([
                                cast::<i32, I>(x1),
                                cast::<i32, I>(y1),
                                cast::<i32, I>(x2),
                                cast::<i32, I>(y2),
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
/// \[number of points\]
/// \[X\] \[Y\] (repeats)
/// \[number of lines\]
/// \[X1\] \[Y1\] \[X2\] \[Y2\](repeats)
/// This entire module is implemented in about 20 lines of code in C++ boost :/
#[allow(clippy::type_complexity)]
pub fn read_boost_input_file<I>(
    filename: &Path,
) -> Result<(Vec<geometry::Point<I>>, Vec<geometry::Line<I>>), BvError>
where
    I: super::InputType,
{
    if !filename.is_file() || !filename.exists() {
        return Err(BvError::ValueError(format!("{:?} not a file", filename)));
    }
    // Open the file in read-only mode
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    read_boost_input_buffer::<I, _>(reader)
}

/// Reads an example from a buffer using the format used by C++ boost voronoi
/// \[number of points\]
/// \[X\] \[Y\] (repeats)
/// \[number of lines\]
/// \[X1\] \[Y1\] \[X2\] \[Y2\](repeats)
#[allow(clippy::type_complexity)]
pub fn read_boost_input_buffer<I, F>(
    reader: BufReader<F>,
) -> Result<(Vec<geometry::Point<I>>, Vec<geometry::Line<I>>), BvError>
where
    I: super::InputType,
    F: std::io::Read,
{
    let mut points = Vec::<geometry::Point<I>>::default();
    let mut lines = Vec::<geometry::Line<I>>::default();
    let mut state = StateMachine::StartingPoints;
    let mut expected_points = 0;
    let mut expected_lines = 0;

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            //println!("got {:?}, state:{:?}", line, state);
            if let Some(data) = line_to_data::<I>(&line) {
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
                        if let InputData::Point(geometry::Point::<I> { x, y }) = data {
                            points.push(geometry::Point { x, y });
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
                            if expected_lines == lines.len() {
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
