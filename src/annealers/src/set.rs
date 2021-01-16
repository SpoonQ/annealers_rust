use std::collections::BTreeSet;
use std::hash::Hash;
use std::iter::{FromIterator, IntoIterator, Iterator};

#[allow(clippy::len_without_is_empty)] //< NodeSet should not empty
pub trait NodeSet: 'static + Clone + Hash + PartialEq + Eq + std::fmt::Debug {
	type Iter: Iterator<Item = usize>;
	#[inline]
	fn into_set(self) -> BTreeSet<usize> {
		self.iter().collect()
	}

	#[inline]
	fn into_vec(self) -> Vec<usize> {
		self.iter().collect()
	}

	#[inline]
	fn from_set(set: BTreeSet<usize>) -> Option<Self> {
		Self::from_it(set)
	}

	#[inline]
	fn from_vec(mut vec: Vec<usize>) -> Option<Self> {
		// Because ordering of two usize value is Equal, they are the same
		// So stable sort is not required.
		vec.sort_unstable();
		unsafe { Self::from_vec_unchecked(vec) }
	}

	/// # Safety
	/// Given vec must be sorted.
	#[inline]
	unsafe fn from_vec_unchecked(vec: Vec<usize>) -> Option<Self> {
		Self::from_it(vec)
	}

	fn from_it<T: IntoIterator<Item = usize>>(iter: T) -> Option<Self>;
	fn iter(&self) -> <Self as NodeSet>::Iter;

	/// # Important
	/// `len()` should return non-zero value.
	#[inline]
	fn len(&self) -> usize {
		self.iter().count()
	}

	#[inline]
	fn contains(&self, node: usize) -> bool {
		self.iter().any(|n| n == node)
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
		Some(iter.into_iter().collect())
	}

	#[inline]
	fn from_set(set: BTreeSet<usize>) -> Option<Self> {
		Some(set)
	}

	#[inline]
	fn iter(&self) -> <Self as NodeSet>::Iter {
		Box::new(self.clone().into_iter()) as Box<dyn Iterator<Item = usize>>
	}

	#[inline]
	fn into_vec(self) -> Vec<usize> {
		<Self as NodeSet>::iter(&self).collect()
	}

	#[inline]
	fn len(&self) -> usize {
		self.len()
	}

	#[inline]
	fn contains(&self, node: usize) -> bool {
		self.contains(&node)
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
	fn iter(&self) -> <Self as NodeSet>::Iter {
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
		match *v.as_slice() {
			[i] => Some([i, i]),
			[i, j] => Some([i, j]),
			_ => None,
		}
	}

	#[inline]
	fn iter(&self) -> <Self as NodeSet>::Iter {
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

	#[inline]
	fn contains(&self, node: usize) -> bool {
		self[0] == node || self[1] == node
	}
}
