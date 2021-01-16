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

/// This trait is implemented between all Real types.
pub trait ConvertForce<R: Real>: Real {
	fn convert_force(self) -> R;
}

impl<R: Real> ConvertForce<R> for R {
	fn convert_force(self) -> R {
		self
	}
}

macro_rules! impl_convert_force {
	($r1:ty,$r2:ty) => {
		impl ConvertForce<$r2> for $r1 {
			#[inline]
			fn convert_force(self) -> $r2 {
				self as $r2
			}
		}
	};
	($r:ty) => {
		impl_convert_force!($r, i8);
		impl_convert_force!($r, i16);
		impl_convert_force!($r, i32);
		impl_convert_force!($r, i64);
		impl_convert_force!($r, i128);
		impl_convert_force!($r, f32);
		impl_convert_force!($r, f64);
	};
}
impl_convert_force!(i8, i16);
impl_convert_force!(i8, i32);
impl_convert_force!(i8, i64);
impl_convert_force!(i8, i128);
impl_convert_force!(i8, f32);
impl_convert_force!(i8, f64);
impl_convert_force!(i16, i8);
impl_convert_force!(i16, i32);
impl_convert_force!(i16, i64);
impl_convert_force!(i16, i128);
impl_convert_force!(i16, f32);
impl_convert_force!(i16, f64);
impl_convert_force!(i32, i8);
impl_convert_force!(i32, i16);
impl_convert_force!(i32, i64);
impl_convert_force!(i32, i128);
impl_convert_force!(i32, f32);
impl_convert_force!(i32, f64);
impl_convert_force!(i64, i8);
impl_convert_force!(i64, i16);
impl_convert_force!(i64, i32);
impl_convert_force!(i64, i128);
impl_convert_force!(i64, f32);
impl_convert_force!(i64, f64);
impl_convert_force!(i128, i8);
impl_convert_force!(i128, i16);
impl_convert_force!(i128, i32);
impl_convert_force!(i128, i64);
impl_convert_force!(i128, f32);
impl_convert_force!(i128, f64);
impl_convert_force!(f32, i8);
impl_convert_force!(f32, i16);
impl_convert_force!(f32, i32);
impl_convert_force!(f32, i64);
impl_convert_force!(f32, i128);
impl_convert_force!(f32, f64);
impl_convert_force!(f64, i8);
impl_convert_force!(f64, i16);
impl_convert_force!(f64, i32);
impl_convert_force!(f64, i64);
impl_convert_force!(f64, i128);
impl_convert_force!(f64, f32);

pub trait ConvertFrom<R: Real>: ConvertForce<R> {
	fn convert_from(f: R) -> Self;
}

impl<R: Real + ConvertForce<R>> ConvertFrom<R> for R {
	fn convert_from(f: R) -> Self {
		f
	}
}

macro_rules! impl_real_convert_from {
	($r1: ty, $r2:ty) => {
		impl ConvertFrom<$r1> for $r2 {
			#[inline]
			fn convert_from(f: $r1) -> Self {
				f as $r2
			}
		}
	};
}

pub trait ConvertTo<R: Real>: ConvertForce<R> {
	fn convert_to(self) -> R;
}

impl<R2: ConvertFrom<R1>, R1: ConvertForce<R2>> ConvertTo<R2> for R1 {
	fn convert_to(self) -> R2 {
		R2::convert_from(self)
	}
}

pub trait ConvertWith<Rhs: Real>: Real {
	type Output: ConvertFrom<Self> + ConvertFrom<Rhs>;
	fn convert_lhs(lhs: Self) -> <Self as ConvertWith<Rhs>>::Output;
	fn convert_rhs(rhs: Rhs) -> <Self as ConvertWith<Rhs>>::Output;
}

impl<R: Real + ConvertFrom<R>> ConvertWith<R> for R {
	type Output = R;
	fn convert_lhs(lhs: R) -> R {
		lhs
	}
	fn convert_rhs(rhs: R) -> R {
		rhs
	}
}

macro_rules! impl_real_convert_with {
	($r1:ty,$r2:ty) => {
		impl_real_convert_with!($r1, $r2, $r2);
		impl_real_convert_with!($r2, $r1, $r2);
	};
	($lhs:ty,$rhs:ty,$out:ty) => {
		impl ConvertWith<$rhs> for $lhs {
			type Output = $out;
			fn convert_lhs(lhs: $lhs) -> $out {
				lhs as $out
			}
			fn convert_rhs(rhs: $rhs) -> $out {
				rhs as $out
			}
		}
	};
}

macro_rules! impl_all {
	($r1:ty,$r2:ty) => {
		impl_real_convert_with!($r1, $r2);
		impl_real_convert_from!($r1, $r2);
	};
}

impl_all!(i8, i16);
impl_all!(i8, i32);
impl_all!(i8, i64);
impl_all!(i8, i128);
impl_all!(i8, f32);
impl_all!(i8, f64);
impl_all!(i16, i32);
impl_all!(i16, i64);
impl_all!(i16, i128);
impl_all!(i16, f32);
impl_all!(i16, f64);
impl_all!(i32, i64);
impl_all!(i32, i128);
impl_all!(i32, f32);
impl_all!(i32, f64);
impl_all!(i64, i128);
impl_all!(i64, f32);
impl_all!(i64, f64);
impl_all!(i128, f32);
impl_all!(i128, f64);
impl_all!(f32, f64);
