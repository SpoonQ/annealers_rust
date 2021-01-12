use rand::prelude::*;

#[derive(Clone)]
pub struct BinaryRepr {
	state: Vec<u8>,
	len: usize,
}

static BITVALUES: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
const BYTESIZE: usize = std::mem::size_of::<u8>();

impl BinaryRepr {
	#[inline]
	pub fn new_random<T: Rng>(len: usize, r: &mut T) -> Self {
		let mut ret = unsafe { Self::with_len_unchecked(len) };
		r.fill_bytes(&mut ret.state);
		ret
	}

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

	#[allow(unused)]
	#[inline]
	pub fn get(&self, loc: usize) -> bool {
		debug_assert!(loc < self.len);
		(self.state[loc / BYTESIZE] & BITVALUES[loc % BYTESIZE]) > 0
	}

	#[allow(unused)]
	#[inline]
	pub fn set(&mut self, loc: usize, val: bool) {
		debug_assert!(loc < self.len);
		if val {
			self.state[loc / BYTESIZE] |= BITVALUES[loc % BYTESIZE];
		} else {
			self.state[loc / BYTESIZE] &= !BITVALUES[loc % BYTESIZE];
		}
	}

	#[inline]
	pub unsafe fn get_unchecked(&self, loc: usize) -> bool {
		(self.state.get_unchecked(loc / BYTESIZE) & BITVALUES.get_unchecked(loc % BYTESIZE)) > 0
	}

	#[allow(unused)]
	#[inline]
	pub fn flip(&mut self, loc: usize) {
		self.state[loc / BYTESIZE] ^= BITVALUES[loc % BYTESIZE];
	}

	#[inline]
	pub unsafe fn flip_unchecked(&mut self, loc: usize) {
		*self.state.get_unchecked_mut(loc / BYTESIZE) ^= BITVALUES.get_unchecked(loc % BYTESIZE);
	}
}

impl std::fmt::Debug for BinaryRepr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for i in 0..self.len {
			if self.get(i) {
				f.write_str("1")?;
			} else {
				f.write_str("0")?;
			}
		}
		Ok(())
	}
}
