//! # Mono
//!
//! Module containing type for handling monophonic audio data.

use super::*;
use bae_utils::*;

/// Type for a track of [`Mono`] samples
///
/// [`Mono`]: struct.Mono.html
pub type MonoTrackT = Vec<Mono>;

/// Struct representing a monophonic audio sample.
#[derive(Debug, Copy, Clone, Default, PartialEq)]
#[repr(C)]
pub struct Mono {
    /// The single, monophonic sample.
    pub mono: Sample,
}

impl Mono {
    /// Creates a new Mono object with a default value of 0.
    pub fn new() -> Self {
        Mono::default()
    }

    /// Creates a new Mono object from the given sample value.
    pub fn from(s: Sample) -> Self {
        Mono { mono: s }
    }
}

impl SampleFormat for Mono {
    fn from_sample(x: Sample) -> Self {
        Mono { mono: x }
    }

    fn into_sample(self) -> Sample {
        self.mono
    }

    fn num_samples() -> usize {
        1
    }
}

impl<T> Panner<T> for Mono {
    fn to_sample_format(s: Sample, _: T) -> Self {
        Mono { mono: s }
    }
}

use std::ops::*;

impl Neg for Mono {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Mono { mono: Sample(-self.mono.0) }
    }
}

impl Add<Mono> for Mono {
    type Output = Self;

    fn add(self, rhs: Mono) -> Self::Output {
        Mono {
            mono: Sample(self.mono.0 + rhs.mono.0),
        }
    }
}
impl AddAssign<Mono> for Mono {
    fn add_assign(&mut self, rhs: Mono) {
        self.mono.0 += rhs.mono.0;
    }
}

impl Sub<Mono> for Mono {
    type Output = Self;

    fn sub(self, rhs: Mono) -> Self {
        Mono {
            mono: Sample(self.mono.0 + rhs.mono.0),
        }
    }
}
impl SubAssign<Mono> for Mono {
    fn sub_assign(&mut self, rhs: Mono) {
        self.mono.0 -= rhs.mono.0;
    }
}

impl Mul<Mono> for Mono {
    type Output = Mono;

    fn mul(self, rhs: Mono) -> Self::Output {
        Mono {
            mono: Sample(self.mono.0 * rhs.mono.0),
        }
    }
}
impl MulAssign<Mono> for Mono {
    fn mul_assign(&mut self, rhs: Mono) {
        self.mono.0 *= rhs.mono.0;
    }
}

impl Mul<Sample> for Mono {
    type Output = Mono;

    fn mul(self, rhs: Sample) -> Self::Output {
        Mono {
            mono: Sample(self.mono.0 * rhs.0),
        }
    }
}
impl MulAssign<Sample> for Mono {
    fn mul_assign(&mut self, rhs: Sample) {
        self.mono.0 *= rhs.0;
    }
}

impl Mul<Math> for Mono {
    type Output = Mono;

    fn mul(self, rhs: Math) -> Self::Output {
        Mono {
            mono: Sample((self.mono.0 as AccurateMath * rhs.0) as FastMath),
        }
    }
}
impl MulAssign<Math> for Mono {
    fn mul_assign(&mut self, rhs: Math) {
        self.mono = Sample((self.mono.0 as AccurateMath * rhs.0) as FastMath);
    }
}

impl From<Sample> for Mono {
    fn from(s: Sample) -> Self {
        Mono::from_sample(s)
    }
}
impl Into<Sample> for Mono {
    fn into(self) -> Sample {
        Mono::into_sample(self)
    }
}

impl TryFrom<Vec<u8>> for Mono {
    type Error = String;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            Err(format!(
                "ERROR: Given vector was length {}. This function requires length 1.",
                v.len()
            ))
        } else {
            Ok(Mono {
                mono: sample_from_u8(v[0]),
            })
        }
    }
}
impl Into<Vec<u8>> for Mono {
    fn into(self) -> Vec<u8> {
        vec![sample_to_u8(self.mono)]
    }
}

impl TryFrom<Vec<i16>> for Mono {
    type Error = String;

    fn try_from(v: Vec<i16>) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            Err(format!(
                "ERROR: Given vector was length {}. This function requires length 1.",
                v.len()
            ))
        } else {
            Ok(Mono {
                mono: sample_from_i16(v[0]),
            })
        }
    }
}
impl Into<Vec<i16>> for Mono {
    fn into(self) -> Vec<i16> {
        vec![sample_to_i16(self.mono)]
    }
}

impl TryFrom<Vec<i32>> for Mono {
    type Error = String;

    fn try_from(v: Vec<i32>) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            Err(format!(
                "ERROR: Given vector was length {}. This function requires length 1.",
                v.len()
            ))
        } else {
            Ok(Mono {
                mono: sample_from_i24(v[0]),
            })
        }
    }
}
impl Into<Vec<i32>> for Mono {
    fn into(self) -> Vec<i32> {
        vec![sample_to_i24(self.mono)]
    }
}
