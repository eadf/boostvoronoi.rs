pub mod extended_exp_fpt;
pub mod extended_int;
pub mod robust_fpt;

use num_traits::NumCast;

#[inline(always)]
/// Convert from one numeric type to another.
/// # Panics
/// panics if the conversion fails
pub fn cast<T: NumCast, U: NumCast>(n: T) -> U {
    NumCast::from(n).unwrap()
}

