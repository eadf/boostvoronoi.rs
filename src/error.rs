use std::error;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum BVError {
    RadiusLessThanZero,
    ThetaNotWithinRange,
    VerticesGoesFirst { txt: String },
    SomeError { txt: String },
}

impl fmt::Display for BVError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            BVError::RadiusLessThanZero => write!(
                f,
                "BVError error: given value for the radius is less than 0.0."
            ),
            BVError::ThetaNotWithinRange => write!(
                f,
                "BVError error: given value for theta not within the range [0, PI]"
            ),
            BVError::VerticesGoesFirst { txt } => write!(f, "BVError error: {}, ", txt),
            BVError::SomeError { txt } => write!(f, "BVError error: {}", txt),
        }
    }
}

impl error::Error for BVError {}
