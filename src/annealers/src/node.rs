use crate::variable::Real;
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

pub trait Node: Clone + Send + Sync {
	type RealType: Real;
}

pub trait SingleNode: Clone + Send + Sync {
	type RealType: Real;
	fn get_value(&self, b: bool) -> Self::RealType;
}

impl<T: SingleNode> Node for T {
	type RealType = T::RealType;
}

#[allow(unused)]
#[derive(Clone)]
pub struct DiscreteNode<Single: SingleNode, Key: Hash + Debug> {
	keys: BTreeSet<Key>,
	node: Single,
}

impl<S: SingleNode, K: Hash + Debug + Clone + Send + Sync> Node for DiscreteNode<S, K> {
	type RealType = S::RealType;
}

#[derive(Clone)]
pub struct Spin<R: Real> {
	_phantom: PhantomData<R>,
}

impl<R: Real> Spin<R> {
	pub fn new() -> Self {
		Self {
			_phantom: PhantomData,
		}
	}
}

impl<R: Real> SingleNode for Spin<R> {
	type RealType = R;
	fn get_value(&self, b: bool) -> R {
		R::from_i32(if b { 1 } else { -1 })
	}
}

#[derive(Clone)]
pub struct Binary<R: Real> {
	_phantom: PhantomData<R>,
}

impl<R: Real> Binary<R> {
	pub fn new() -> Self {
		Self {
			_phantom: PhantomData,
		}
	}
}

impl<R: Real> SingleNode for Binary<R> {
	type RealType = R;
	fn get_value(&self, b: bool) -> R {
		R::from_i32(if b { 1 } else { 0 })
	}
}

#[derive(Clone)]
pub struct TwoVal<R: Real> {
	true_value: R,
	false_value: R,
}

impl<R: Real> TwoVal<R> {
	pub fn new(true_value: R, false_value: R) -> Self {
		Self {
			true_value,
			false_value,
		}
	}
}

impl<R: Real> SingleNode for TwoVal<R> {
	type RealType = R;
	fn get_value(&self, b: bool) -> R {
		if b {
			self.true_value
		} else {
			self.false_value
		}
	}
}

// pub struct DiscreteNode<K: Hash + Eq, N: SingleNode>(HashSet<K>, N);
//
// impl<K: Hash + Eq, N: SingleNode> Node for DiscreteNode<K, N> {}
