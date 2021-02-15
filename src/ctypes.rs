// Boost.Polygon library detail/voronoi_ctypes.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history.

// Ported from C++ boost 1.74.0 to Rust in 2020 by Eadf (github.com/eadf)

use std::cmp::Ordering;

union UlpMemCpy {
    f64: f64,
    u64: u64,
}

/// If two floating-point numbers in the same format are ordered (x < y),
/// then they are ordered the same way when their bits are reinterpreted as
/// sign-magnitude integers. Values are considered to be almost equal if
/// their integer bits reinterpretations differ in not more than maxUlps units.
pub(crate) struct UlpComparison {}

impl UlpComparison {
    pub(crate) fn ulp_comparison(a: f64, b: f64, max_ulps: u64) -> Ordering {
        // Reinterpret double bits as 64-bit signed integer.
        let mut ll_a: u64 = unsafe {
            let amemcpy = UlpMemCpy { f64: a };
            amemcpy.u64
        };
        let mut ll_b: u64 = unsafe {
            let amemcpy = UlpMemCpy { f64: b };
            amemcpy.u64
        };

        // Positive 0.0 is integer zero. Negative 0.0 is 0x8000000000000000.
        // Map negative zero to an integer zero representation - making it
        // identical to positive zero - the smallest negative number is
        // represented by negative one, and downwards from there.
        if ll_a < 0x8000000000000000u64 {
            ll_a = 0x8000000000000000u64 - ll_a;
        }
        if ll_b < 0x8000000000000000u64 {
            ll_b = 0x8000000000000000u64 - ll_b;
        }

        // Compare 64-bit signed integer representations of input values.
        // Difference in 1 Ulp is equivalent to a relative error of between
        // 1/4,000,000,000,000,000 and 1/8,000,000,000,000,000.
        if ll_a > ll_b {
            if ll_a - ll_b <= max_ulps {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        } else if ll_b - ll_a <= max_ulps {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}
