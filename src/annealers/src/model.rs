use crate::node::{Node, SingleNode};
use crate::order::{Order, Quadric};
use crate::set::NodeSet;
use std::iter::IntoIterator;
use std::marker::PhantomData;

pub trait Model: Clone {
	type NodeType: Node;
	type NodesIter: IntoIterator<Item = usize>;
	fn node(&self) -> Self::NodeType;
	fn nodes(&self) -> Self::NodesIter;
	#[inline]
	fn size(&self) -> usize {
		self.nodes().into_iter().count()
	}
}

pub trait SingleModel: Clone {
	type NodeType: SingleNode;
	type NodeSetType: NodeSet;
	type NodesIter: IntoIterator<Item = usize>;
	type ProdsIter: IntoIterator<Item = Self::NodeSetType>;
	type NeighborsIter: IntoIterator<Item = Self::NodeSetType>;
	type Order: Order;

	fn order(&self) -> Self::Order;
	fn node(&self) -> Self::NodeType;
	fn nodes(&self) -> Self::NodesIter;

	#[inline]
	fn size(&self) -> usize {
		self.nodes().into_iter().count()
	}

	#[inline]
	fn get_weight(&self, p: &Self::NodeSetType) -> <Self::NodeType as SingleNode>::RealType {
		assert!(self.prods().into_iter().any(|item| &item == p));
		unsafe { self.get_weight_unchecked(p) }
	}

	unsafe fn get_weight_unchecked(
		&self,
		p: &Self::NodeSetType,
	) -> <Self::NodeType as SingleNode>::RealType;

	fn prods(&self) -> Self::ProdsIter;
	fn neighbors(&self, u: usize) -> Self::NeighborsIter;
}

impl<T: SingleModel> Model for T {
	type NodeType = T::NodeType;
	type NodesIter = T::NodesIter;
	fn node(&self) -> T::NodeType {
		T::node(self)
	}
	fn nodes(&self) -> Self::NodesIter {
		T::nodes(self)
	}

	fn size(&self) -> usize {
		self.size()
	}
}

pub trait FixedSingleModel: Clone {
	type NodeType: SingleNode;
	type Order: Order;

	fn order(&self) -> Self::Order;
	fn node(&self) -> Self::NodeType;
	fn size(&self) -> usize;

	#[inline]
	fn get_weight(
		&self,
		p: &<Self::Order as Order>::NodeSetType,
	) -> <Self::NodeType as SingleNode>::RealType {
		assert!(p.to_it().all(|i| i < self.size()));
		unsafe { self.get_weight_unchecked(p) }
	}

	unsafe fn get_weight_unchecked(
		&self,
		p: &<Self::Order as Order>::NodeSetType,
	) -> <Self::NodeType as SingleNode>::RealType;
}

/// Single Model with fixed size and no missing indexes of nodes, edges
impl<P: FixedSingleModel> SingleModel for P {
	type NodeType = P::NodeType;
	type NodeSetType = <P::Order as Order>::NodeSetType;
	type NodesIter = std::ops::Range<usize>;
	type ProdsIter = FixedSingleProdIter<Self::NodeSetType>;
	type NeighborsIter = FixedSingleNeighborIter<Self::NodeSetType>;
	type Order = P::Order;
	fn order(&self) -> Self::Order {
		self.order()
	}
	fn node(&self) -> Self::NodeType {
		self.node()
	}
	fn nodes(&self) -> Self::NodesIter {
		0..self.size()
	}
	fn prods(&self) -> Self::ProdsIter {
		FixedSingleProdIter::new(self.order().order(), self.size())
	}
	fn neighbors(&self, u: usize) -> Self::NeighborsIter {
		FixedSingleNeighborIter::new(u, self.order().order(), self.size())
	}

	#[inline]
	fn get_weight(
		&self,
		p: &<Self::Order as Order>::NodeSetType,
	) -> <Self::NodeType as SingleNode>::RealType {
		self.get_weight(p)
	}

	#[inline]
	unsafe fn get_weight_unchecked(
		&self,
		p: &Self::NodeSetType,
	) -> <P::NodeType as Node>::RealType {
		self.get_weight_unchecked(p)
	}
}

#[derive(Clone)]
pub struct FixedSingleQuadricModel<NodeType: SingleNode> {
	size: usize,
	node: NodeType,
	matrix: Vec<NodeType::RealType>,
}

impl<M: SingleNode> FixedSingleQuadricModel<M> {
	pub fn new(node: M, size: usize) -> Self {
		Self {
			size,
			node,
			matrix: std::iter::repeat(<M::RealType as Default>::default())
				.take(size * (size + 1) / 2)
				.collect(),
		}
	}

	#[inline]
	fn get_index(&self, i: usize, j: usize) -> usize {
		assert!(i < self.size, "i should be less than {}", self.size);
		assert!(j < self.size, "j should be less than {}", self.size);
		let (i, j) = if i < j { (i, j) } else { (j, i) };
		unsafe { self.get_index_unchecked(i, j) }
	}

	#[inline]
	unsafe fn get_index_unchecked(&self, i: usize, j: usize) -> usize {
		debug_assert!(i <= j && j < self.size);
		j * (j + 1) / 2 + i
	}

	#[inline]
	pub fn add_weight(&mut self, i: usize, j: usize, w: M::RealType) {
		let idx = self.get_index(i, j);
		self.matrix[idx] += w;
	}
}

impl<M: SingleNode> FixedSingleModel for FixedSingleQuadricModel<M> {
	type NodeType = M;
	type Order = Quadric;

	#[inline]
	fn node(&self) -> Self::NodeType {
		self.node.clone()
	}

	#[inline]
	fn order(&self) -> Self::Order {
		Quadric
	}

	#[inline]
	fn size(&self) -> usize {
		self.size
	}

	#[inline]
	unsafe fn get_weight_unchecked(&self, p: &[usize; 2]) -> M::RealType {
		*self
			.matrix
			.get_unchecked(self.get_index_unchecked(p[0], p[1]))
	}
}

#[allow(unused)]
pub struct FixedSingleProdIter<S: NodeSet> {
	order: usize,
	size: usize,
	indices: Vec<usize>,
	_phantom: PhantomData<S>,
}

impl<S: NodeSet> FixedSingleProdIter<S> {
	pub fn new(order: usize, size: usize) -> Self {
		assert!(order == 2); // TODO:
		Self {
			order,
			size,
			indices: vec![0],
			_phantom: PhantomData,
		}
	}
}

impl<S: NodeSet> std::iter::Iterator for FixedSingleProdIter<S> {
	type Item = S;
	fn next(&mut self) -> Option<S> {
		// TODO: high-order
		let finger = self.indices.len() - 1;
		if self.indices[finger] < self.size {
			let ret = S::from_vec(self.indices.clone()).unwrap();
			self.indices[finger] += 1;
			Some(ret)
		} else if self.indices.len() == 1 {
			self.indices = vec![0, 1];
			let ret = S::from_vec(self.indices.clone()).unwrap();
			self.indices[1] += 1;
			Some(ret)
		} else {
			self.indices[0] += 1;
			self.indices[1] = self.indices[0] + 1;
			if self.indices[1] < self.size {
				let ret = S::from_vec(self.indices.clone()).unwrap();
				self.indices[1] += 1;
				Some(ret)
			} else {
				None
			}
		}
	}
}

pub struct FixedSingleNeighborIter<S: NodeSet> {
	u: usize,
	item: usize,
	size: usize,
	_phantom: PhantomData<S>,
}

impl<S: NodeSet> FixedSingleNeighborIter<S> {
	pub fn new(u: usize, order: usize, size: usize) -> Self {
		assert!(order == 2); // TODO:
		Self {
			size,
			item: 0,
			u,
			_phantom: PhantomData,
		}
	}
}

impl<S: NodeSet> std::iter::Iterator for FixedSingleNeighborIter<S> {
	type Item = S;
	fn next(&mut self) -> Option<S> {
		if self.item < self.size {
			let v = self.item;
			self.item += 1;
			if self.u > v {
				Some(S::from_vec(vec![v, self.u]).unwrap())
			} else if self.u == v {
				Some(S::from_vec(vec![v]).unwrap())
			} else {
				Some(S::from_vec(vec![self.u, v]).unwrap())
			}
		} else {
			None
		}
	}
}

#[test]
fn proditer_test() {
	let mut it: FixedSingleProdIter<[usize; 2]> = FixedSingleProdIter::new(2, 4);
	assert_eq!(it.next(), Some([0, 0]));
	assert_eq!(it.next(), Some([1, 1]));
	assert_eq!(it.next(), Some([2, 2]));
	assert_eq!(it.next(), Some([3, 3]));
	assert_eq!(it.next(), Some([0, 1]));
	assert_eq!(it.next(), Some([0, 2]));
	assert_eq!(it.next(), Some([0, 3]));
	assert_eq!(it.next(), Some([1, 2]));
	assert_eq!(it.next(), Some([1, 3]));
	assert_eq!(it.next(), Some([2, 3]));
	assert_eq!(it.next(), None);
}
