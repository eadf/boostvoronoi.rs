// Boost.Polygon library voronoi_diagram.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history.

// Ported from C++ boost 1.74.0 to Rust in 2020 by Eadf (github.com/eadf)

use super::voronoi_beachline as VB;
use super::voronoi_diagram as VD;
use super::voronoi_predicate as VP;
use ordered_float::OrderedFloat;
use std::cmp::Ordering;

use super::{BigFloatType, BigIntType, InputType, OutputType};
use num::{NumCast, PrimInt};
use std::fmt;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::Neg;
use std::rc::Rc;
use std::rc::Weak;

mod tests;




