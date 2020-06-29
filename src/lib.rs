//! # Sample Format
//!
//! Module containing different output formats like stereo, 2.1, 5.1, 7.1, etc.
//!
//! All functions that deal with converting raw bytes to numeric types assume
//! the bytes are in little-endian format.
//!
//! As there is no i24 built-in type, i32 is used in it's place where
//! applicable. In most cases where a 24-bit sample is stored in a 32-bit data
//! type, the upper byte is ignored or explicitly set to 0.

#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/bae_sf/0.14.2")]

use bae_types::*;

pub mod mono;
pub mod stereo;
pub use mono::*;
pub use stereo::*;

use std::convert::TryFrom;
use std::ops::*;

/// Trait implementing the ability to perform math operations with a polyphonic
/// sample format and a monophonic sample.
///
/// # Dependencies:
///
/// * Default - A good default value for audio samples is 0.
/// * Most mathematical operators are required to be implemented to be able to
/// perform common operations on sample values.
/// * [`Mul`]/[`MulAssign`] is defined for both [`Math`] as well as [`Sample`]
/// for the convenience of common audio operations.
/// * [`From`]/[`Into`] implemented for [`Sample`] - These functions should be
/// simple calls to [`from_sample`] and [`into_sample`] respectively.
/// * [`TryFrom`]/[`Into`] implemented for [`Vec<_>`] - These functions should
/// convert the sample values to the given standard integer types. As [`Vec`]s
/// are generic types, it cannot be assumed that any attempted conversions of
/// [`Vec`]s to a given sample format will succeed. Therefore those conversions
/// use [`TryFrom`] to indicate when there is an issue, which can be
/// communicated with the given [`String`] used for the error type. An example
/// of such an error could be (for the [`Stereo`] type):
///
/// ```rust
/// # use bae_rs::Stereo;
/// # use std::convert::TryFrom;
///
/// let v: Vec<i16> = vec![];
///
/// assert_eq!(Err("ERROR: Given vector was length 0. This function requires length 2.".to_owned()), Stereo::try_from(v));
/// ```
///
/// [`Mul`]: https://doc.rust-lang.org/std/ops/trait.Mul.html
/// [`MulAssign`]: https://doc.rust-lang.org/std/ops/trait.MulAssign.html
/// [`Math`]: ../type.Math.html
/// [`Sample`]: ../type.Sample.html
/// [`From`]: https://doc.rust-lang.org/std/convert/trait.From.html
/// [`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
/// [`from_sample`]: #tymethod.from_sample
/// [`into_sample`]: #tymethod.into_sample
/// [`TryFrom`]: https://doc.rust-lang.org/std/convert/trait.TryFrom.html
/// [`Vec<_>`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// [`String`]: https://doc.rust-lang.org/std/convert/trait.From.html
/// [`Stereo`]: stereo/struct.Stereo.html
pub trait SampleFormat:
    Default
    + Neg<Output = Self>
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Mul<Self, Output = Self>
    + MulAssign<Self>
    + Mul<Sample, Output = Self>
    + MulAssign<Sample>
    + Mul<Math, Output = Self>
    + MulAssign<Math>
    + From<Sample>
    + Into<Sample>
    + TryFrom<Vec<u8>, Error = String>
    + Into<Vec<u8>>
    + TryFrom<Vec<i16>, Error = String>
    + Into<Vec<i16>>
    + TryFrom<Vec<i32>, Error = String>
    + Into<Vec<i32>>
{
    /// Creates an object from a single monophonic sample.
    fn from_sample(x: Sample) -> Self;

    /// Converts the given polyphonic sample to a monophonic sample.
    fn into_sample(self) -> Sample;

    /// Returns the number of [`Sample`] values held within a given
    /// [`SampleFormat`]. A common use for this would be for ensuring [`Vec`]s
    /// given to [`try_from`] have the correct size.
    ///
    /// [`Sample`]: ../type.Sample.html
    /// [`SampleFormat`]: trait.SampleFormat.html
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    /// [`try_from`]: https://doc.rust-lang.org/std/convert/trait.TryFrom.html#tymethod.try_from
    fn num_samples() -> usize;
}

/// Trait implementing the ability to pan a monophonic sample into a polyphonic
/// sample. This is generic for the polyphonic type and the type that defines
/// how it is panned. To see an implementation, see
/// [`Stereo::to_sample_format`].
///
/// [`Stereo::to_sample_format`]: stereo/struct.Stereo.html#method.to_sample_format
pub trait Panner<G>: SampleFormat {
    /// Converts the monophonic sample into a polyphonic sample.
    fn to_sample_format(s: Sample, g: G) -> Self;
}
