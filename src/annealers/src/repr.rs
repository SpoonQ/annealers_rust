use rand::prelude::*;

#[derive(Clone)]
pub struct BinaryRepr {
	state: Vec<u8>,
	len: usize,
}

static BITVALUES: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
const BYTESIZE: usize = 8;

impl BinaryRepr {
	#[inline]
	pub fn new_random<T: Rng>(len: usize, r: &mut T) -> Self {
		let mut ret = unsafe { Self::with_len_unchecked(len) };
		r.fill_bytes(&mut ret.state);
		ret
	}

	/// # Safety
	/// Given len is less than len()
	#[inline]
	pub unsafe fn with_len_unchecked(len: usize) -> Self {
		let size = (len + BYTESIZE - 1) / BYTESIZE;
		let mut v = Vec::with_capacity(size);
		v.set_len(size);
		Self { state: v, len }
	}

	pub fn from_vec(v: &[bool]) -> Self {
		let mut ret = unsafe { Self::with_len_unchecked(v.len()) };
		for (i, b) in v.iter().enumerate() {
			ret.set(i, *b);
		}
		ret
	}

	pub fn to_vec(&self) -> Vec<bool> {
		let mut v = Vec::with_capacity(self.len);
		for i in 0..self.len() {
			v.push(unsafe { self.get_unchecked(i) });
		}
		v
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.len
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.len == 0
	}

	#[inline]
	pub fn get(&self, loc: usize) -> bool {
		assert!(loc < self.len());
		unsafe { self.get_unchecked(loc) }
	}

	#[inline]
	pub fn set(&mut self, loc: usize, val: bool) {
		assert!(loc < self.len);
		unsafe { self.set_unchecked(loc, val) }
	}

	/// # Safety
	/// Given loc is less than len()
	#[inline]
	pub unsafe fn set_unchecked(&mut self, loc: usize, val: bool) {
		if val {
			self.state[loc / BYTESIZE] |= BITVALUES[loc % BYTESIZE];
		} else {
			self.state[loc / BYTESIZE] &= !BITVALUES[loc % BYTESIZE];
		}
	}

	/// # Safety
	/// Given loc is less than len()
	#[inline]
	pub unsafe fn get_unchecked(&self, loc: usize) -> bool {
		(self.state.get_unchecked(loc / BYTESIZE) & BITVALUES.get_unchecked(loc % BYTESIZE)) > 0
	}

	#[inline]
	pub fn flip(&mut self, loc: usize) {
		assert!(loc < self.len());
		unsafe { self.flip_unchecked(loc) }
	}

	/// # Safety
	/// Given loc is less than len()
	#[inline]
	pub unsafe fn flip_unchecked(&mut self, loc: usize) {
		*self.state.get_unchecked_mut(loc / BYTESIZE) ^= BITVALUES.get_unchecked(loc % BYTESIZE);
	}

	pub fn iter(&self) -> BinaryReprIter<'_> {
		BinaryReprIter(self, 0)
	}
}

pub struct BinaryReprIter<'a>(&'a BinaryRepr, usize);

impl<'a> Iterator for BinaryReprIter<'a> {
	type Item = bool;
	fn next(&mut self) -> Option<bool> {
		if self.1 < self.0.len() {
			let b = unsafe { self.0.get_unchecked(self.1) };
			self.1 += 1;
			Some(b)
		} else {
			None
		}
	}
}

impl std::ops::Index<usize> for BinaryRepr {
	type Output = bool;
	fn index(&self, loc: usize) -> &bool {
		if self.get(loc) {
			&crate::TRUE_VAL
		} else {
			&crate::FALSE_VAL
		}
	}
}

impl std::fmt::Debug for BinaryRepr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.iter()).finish()
	}
}
