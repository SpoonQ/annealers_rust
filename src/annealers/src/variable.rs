use std::fmt::{Debug, Display};
use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Real:
	Copy
	+ Clone
	+ Default
	+ Display
	+ Debug
	+ PartialEq
	+ PartialOrd
	+ Add<Self, Output = Self>
	+ AddAssign<Self>
	+ Mul<Self, Output = Self>
	+ MulAssign<Self>
	+ Div<Self, Output = Self>
	+ DivAssign<Self>
	+ Sub<Self, Output = Self>
	+ SubAssign<Self>
	+ 'static
	+ Send
	+ Sync
	+ Sum
	+ Product
	+ Neg<Output = Self>
{
	const MAX: Self;
	const MIN: Self;
	fn as_f64(&self) -> f64;
	fn from_i32(i: i32) -> Self;
	fn from_f64(f: f64) -> Self;
	fn is_default(&self) -> bool;
	fn abs(self) -> Self;
	fn min(self, other: Self) -> Self;
	fn max(self, other: Self) -> Self;
	fn nan_or(other: Self) -> Self;
	fn is_finite(self) -> bool;
}

macro_rules! impl_nan_or {
	(true, $typ:ty) => {
		#[inline]
		fn nan_or(_other: Self) -> Self {
			(0.0 as $typ) / (0.0 as $typ)
		}
	};
	($b:expr, $typ:ty) => {
		#[inline]
		fn nan_or(other: Self) -> Self {
			other
		}
	};
}

macro_rules! impl_real_as_f64 {
	($b:expr, $typ:ty, $pat:path) => {
		/// Implementation of Real for $typ
		impl Real for $typ {
			const MAX: $typ = <$typ>::MAX;
			const MIN: $typ = <$typ>::MIN;

			#[inline]
			fn as_f64(&self) -> f64 {
				*self as f64
			}

			#[inline]
			fn from_i32(i: i32) -> Self {
				i as $typ
			}

			#[inline]
			fn from_f64(f: f64) -> Self {
				f as $typ
			}

			#[inline]
			fn is_default(&self) -> bool {
				self == &Default::default()
			}

			#[inline]
			fn abs(self) -> Self {
				<$typ>::abs(self)
			}

			#[inline]
			fn min(self, other: Self) -> Self {
				use $pat as cmproot;
				cmproot::min(self, other)
			}

			#[inline]
			fn max(self, other: Self) -> Self {
				use $pat as cmproot;
				cmproot::max(self, other)
			}

			impl_nan_or!($b, $typ);

			#[inline]
			fn is_finite(self) -> bool {
				if $b {
					self.is_finite()
				} else {
					true
				}
			}
		}
	};
}

impl_real_as_f64!(true, f32, f32);
impl_real_as_f64!(true, f64, f64);
impl_real_as_f64!(false, i8, std::cmp);
impl_real_as_f64!(false, i16, std::cmp);
impl_real_as_f64!(false, i32, std::cmp);
impl_real_as_f64!(false, i64, std::cmp);
impl_real_as_f64!(false, i128, std::cmp);
