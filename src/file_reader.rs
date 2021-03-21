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
    let line = line.split(" ").collect::<Vec<&str>>();
    //println!("line split: {:?}", line);
    match line.len() {
        1 => {
            let n = line[0].parse::<usize>();
            if n.is_ok() {
                return Some(InputData::Number(n.unwrap()));
            }
        }
        2 => {
            let x1 = line[0].parse::<i32>();
            let y1 = line[1].parse::<i32>();
            if x1.is_ok() && y1.is_ok() {
                return Some(InputData::Point(super::Point::<I1> {
                    x: super::TypeConverter2::<I1, F1>::i32_to_i1(x1.unwrap()),
                    y: super::TypeConverter2::<I1, F1>::i32_to_i1(y1.unwrap()),
                }));
            }
        }
        4 => {
            let x1 = line[0].parse::<i32>();
            let y1 = line[1].parse::<i32>();
            let x2 = line[2].parse::<i32>();
            let y2 = line[3].parse::<i32>();
            if x1.is_ok() && y1.is_ok() && x2.is_ok() && y2.is_ok() {
                return Some(InputData::Line(super::Line::<I1>::from([
                    super::TypeConverter2::<I1, F1>::i32_to_i1(x1.unwrap()),
                    super::TypeConverter2::<I1, F1>::i32_to_i1(y1.unwrap()),
                    super::TypeConverter2::<I1, F1>::i32_to_i1(x2.unwrap()),
                    super::TypeConverter2::<I1, F1>::i32_to_i1(y2.unwrap()),
                ])));
            } else {
                if !x1.is_ok() {
                    println!("failed to parse {}", line[0]);
                }
                if !y1.is_ok() {
                    println!("failed to parse {}", line[1]);
                }
                if !x2.is_ok() {
                    println!("failed to parse {}", line[2]);
                }
                if !y2.is_ok() {
                    println!("failed to parse {}", line[3]);
                }
            }
        }
        _ => println!("failed to parse whole line {:?}", line),
    }
    None
}

/// Reads an example file in the file format used by C++ boost voronoi
/// [number of points]
/// [X] [Y] (repeats)
/// [number of lines]
/// [X1] [Y1] [X2] [Y2](repeats)
/// This entire module is implemented in about 20 lines of code in C++ boost :/
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
                            "#{}: can't read line {}. state:{:?} ignoring it",
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
                                "#{}: can't read line {}. state:{:?} ignoring it",
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
                            "#{}: can't read line {}. state:{:?} ignoring it",
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
                                "#{}: can't read line {} state:{:?} ignoring it",
                                index + 1,
                                line,
                                state
                            );
                            break;
                        }
                    } else {
                        println!(
                            "#{}: Got too many lines {}. state:{:?} ignoring it",
                            index + 1,
                            line,
                            state
                        );
                        break;
                    }
                }
            };
            println!(
                "#{}: can't parse line {}. state:{:?} ignoring it",
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
