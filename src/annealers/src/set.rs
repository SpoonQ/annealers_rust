use std::collections::BTreeSet;
use std::hash::Hash;
use std::iter::{FromIterator, IntoIterator, Iterator};

pub trait NodeSet: Clone + Hash + PartialEq + std::fmt::Debug {
	type Iter: Iterator<Item = usize>;
	#[inline]
	fn into_set(self) -> BTreeSet<usize> {
		self.to_it().collect()
	}

	#[inline]
	fn into_vec(self) -> Vec<usize> {
		self.to_it().collect()
	}

	#[inline]
	fn from_set(set: BTreeSet<usize>) -> Option<Self> {
		Self::from_it(set)
	}

	#[inline]
	fn from_vec(mut vec: Vec<usize>) -> Option<Self> {
		vec.sort();
		unsafe { Self::from_vec_unchecked(vec) }
	}

	// SAFETY: vec must be sorted
	#[inline]
	unsafe fn from_vec_unchecked(vec: Vec<usize>) -> Option<Self> {
		Self::from_it(vec)
	}

	fn from_it<T: IntoIterator<Item = usize>>(iter: T) -> Option<Self>;
	// TODO: fix name
	fn to_it(&self) -> <Self as NodeSet>::Iter;

	// NonZero
	#[inline]
	fn len(&self) -> usize {
		self.to_it().count()
	}
}

impl NodeSet for BTreeSet<usize> {
	type Iter = Box<dyn Iterator<Item = usize>>;
	#[inline]
	fn into_set(self) -> BTreeSet<usize> {
		self
	}

	#[inline]
	fn from_it<T: IntoIterator<Item = usize>>(iter: T) -> Option<Self> {
		Some(BTreeSet::from_iter(iter.into_iter()))
	}

	#[inline]
	fn from_set(set: BTreeSet<usize>) -> Option<Self> {
		Some(set)
	}

	#[inline]
	fn to_it(&self) -> <Self as NodeSet>::Iter {
		Box::new(self.clone().into_iter()) as Box<dyn Iterator<Item = usize>>
	}

	#[inline]
	fn into_vec(self) -> Vec<usize> {
		self.to_it().collect()
	}

	#[inline]
	fn len(&self) -> usize {
		self.len()
	}
}

impl NodeSet for Vec<usize> {
	type Iter = Box<dyn Iterator<Item = usize>>;

	#[inline]
	fn from_it<T: IntoIterator<Item = usize>>(iter: T) -> Option<Self> {
		Some(Vec::from_iter(iter))
	}

	#[inline]
	unsafe fn from_vec_unchecked(vec: Vec<usize>) -> Option<Self> {
		Some(vec)
	}

	#[inline]
	fn to_it(&self) -> <Self as NodeSet>::Iter {
		Box::new(self.clone().into_iter()) as Box<dyn Iterator<Item = usize>>
	}

	#[inline]
	fn len(&self) -> usize {
		self.len()
	}
}

// SAFETY: arr must be sorted
// TODO: composite in struct for safety
impl NodeSet for [usize; 2] {
	type Iter = std::vec::IntoIter<usize>; // TODO:

	#[inline]
	fn from_it<T: IntoIterator<Item = usize>>(iter: T) -> Option<Self> {
		let v = iter.into_iter().collect::<Vec<_>>();
		match v.as_slice() {
			&[i] => Some([i, i]),
			&[i, j] => Some([i, j]),
			_ => None,
		}
	}

	#[inline]
	fn to_it(&self) -> <Self as NodeSet>::Iter {
		if self[0] == self[1] {
			vec![self[0]].into_iter()
		} else {
			vec![self[0], self[1]].into_iter()
		}
	}

	#[inline]
	fn len(&self) -> usize {
		if self[0] == self[1] {
			1
		} else {
			2
		}
	}
}
