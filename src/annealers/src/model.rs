use crate::node::{Node, SingleNode};
use crate::order::{Order, Quadric};
use crate::set::NodeSet;
use crate::solution::SingleSolution;
use crate::variable::Real;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::iter::IntoIterator;
use std::marker::PhantomData;

pub trait ModelView: Clone {
	type Node: Node;
	type NodesIter: IntoIterator<Item = usize>;
	fn node(&self) -> &Self::Node;
	fn nodes(&self) -> Self::NodesIter;
	#[inline]
	fn size(&self) -> usize {
		self.nodes().into_iter().count()
	}
}

pub trait SingleModelView: Clone {
	type Node: SingleNode;
	type NodesIter: IntoIterator<Item = usize>;
	type ProdsIter: IntoIterator<Item = <Self::Order as Order>::NodeSetType>;
	type NeighborsIter: IntoIterator<Item = <Self::Order as Order>::NodeSetType>;
	type Order: Order;

	fn order(&self) -> &Self::Order;
	fn node(&self) -> &Self::Node;
	fn nodes(&self) -> Self::NodesIter;

	#[inline]
	fn size(&self) -> usize {
		self.nodes().into_iter().count()
	}

	#[inline]
	fn get_weight(
		&self,
		p: &<Self::Order as Order>::NodeSetType,
	) -> <Self::Node as SingleNode>::RealType {
		assert!(self.prods().into_iter().any(|item| &item == p));
		unsafe { self.get_weight_unchecked(p) }
	}

	/// # Safety
	/// p is in nodes()
	unsafe fn get_weight_unchecked(
		&self,
		p: &<Self::Order as Order>::NodeSetType,
	) -> <Self::Node as SingleNode>::RealType;

	fn prods(&self) -> Self::ProdsIter;
	fn neighbors(&self, u: usize) -> Self::NeighborsIter;

	#[inline]
	fn calculate_prod(
		&self,
		p: &<Self::Order as Order>::NodeSetType,
		solution: &SingleSolution<Self::Node>,
	) -> <Self::Node as SingleNode>::RealType {
		let v = p.iter().map(|n| solution[n]).collect::<Vec<_>>();
		self.node().calculate_prod(&v)
	}
}

impl<T: SingleModelView> ModelView for T {
	type Node = T::Node;
	type NodesIter = T::NodesIter;
	fn node(&self) -> &T::Node {
		T::node(self)
	}
	fn nodes(&self) -> Self::NodesIter {
		T::nodes(self)
	}

	fn size(&self) -> usize {
		self.size()
	}
}

pub trait FixedSingleModelView: Clone {
	type Node: SingleNode;
	type Order: Order;

	fn order(&self) -> &Self::Order;
	fn node(&self) -> &Self::Node;
	fn size(&self) -> usize;

	#[inline]
	fn get_weight(
		&self,
		p: &<Self::Order as Order>::NodeSetType,
	) -> <Self::Node as SingleNode>::RealType {
		assert!(p.iter().all(|i| i < self.size()));
		unsafe { self.get_weight_unchecked(p) }
	}

	/// # Safety
	/// All items of p are less than size()
	unsafe fn get_weight_unchecked(
		&self,
		p: &<Self::Order as Order>::NodeSetType,
	) -> <Self::Node as SingleNode>::RealType;
}

/// Single Model with fixed size and no missing indexes of nodes, edges
impl<P: FixedSingleModelView> SingleModelView for P {
	type Node = P::Node;
	type NodesIter = std::ops::Range<usize>;
	type ProdsIter = Prods<<Self::Order as Order>::NodeSetType>;
	type NeighborsIter = Neighbors<<Self::Order as Order>::NodeSetType>;
	type Order = P::Order;
	fn order(&self) -> &Self::Order {
		self.order()
	}
	fn node(&self) -> &Self::Node {
		self.node()
	}
	fn nodes(&self) -> Self::NodesIter {
		0..self.size()
	}
	fn prods(&self) -> Self::ProdsIter {
		Prods::new(self.order().order(), self.size())
	}
	fn neighbors(&self, u: usize) -> Self::NeighborsIter {
		Neighbors::new(u, self.order().order(), self.size())
	}

	#[inline]
	fn get_weight(
		&self,
		p: &<Self::Order as Order>::NodeSetType,
	) -> <Self::Node as SingleNode>::RealType {
		self.get_weight(p)
	}

	#[inline]
	unsafe fn get_weight_unchecked(
		&self,
		p: &<Self::Order as Order>::NodeSetType,
	) -> <P::Node as Node>::RealType {
		self.get_weight_unchecked(p)
	}
}

#[derive(Clone)]
pub struct SingleModel<NodeType: SingleNode, O: Order> {
	node: NodeType,
	order: O,
	nodes: BTreeSet<usize>,
	inner: HashMap<O::NodeSetType, NodeType::RealType>,
}

impl<M: SingleNode, O: Order> SingleModel<M, O> {
	pub fn new(node: M, order: O) -> Self {
		Self {
			node,
			order,
			nodes: BTreeSet::new(),
			inner: HashMap::new(),
		}
	}

	#[inline]
	pub fn add_weight(&mut self, prod: O::NodeSetType, w: M::RealType) {
		for node in prod.iter() {
			self.nodes.insert(node);
		}
		*self.inner.entry(prod).or_insert(M::RealType::zero()) += w;
	}
}

impl<M: SingleNode, O: Order> SingleModelView for SingleModel<M, O> {
	type Node = M;
	type NodesIter = std::collections::btree_set::IntoIter<usize>;
	type ProdsIter = Box<dyn Iterator<Item = O::NodeSetType>>;
	type NeighborsIter = Box<dyn Iterator<Item = O::NodeSetType>>;
	type Order = O;

	fn order(&self) -> &Self::Order {
		&self.order
	}

	fn node(&self) -> &Self::Node {
		&self.node
	}

	fn nodes(&self) -> Self::NodesIter {
		self.nodes.clone().into_iter()
	}

	fn get_weight(&self, p: &O::NodeSetType) -> M::RealType {
		unsafe { self.get_weight_unchecked(p) }
	}

	/// # Safety
	/// it is always safe
	unsafe fn get_weight_unchecked(&self, p: &O::NodeSetType) -> M::RealType {
		*self.inner.get(p).unwrap()
	}

	fn prods(&self) -> Self::ProdsIter {
		let v = self.inner.keys().cloned().collect::<Vec<O::NodeSetType>>();
		Box::new(v.into_iter()) as Box<dyn Iterator<Item = O::NodeSetType>>
	}

	fn neighbors(&self, u: usize) -> Self::NeighborsIter {
		Box::new(self.prods().filter(move |p| p.contains(u)))
			as Box<dyn Iterator<Item = O::NodeSetType>>
	}
}

// pub type SingleQuadricModel<NodeType: SingleNode> = SingleModelView<NodeType,
// order::Quadric>;

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

const QUADRIC: Quadric = Quadric;
impl<M: SingleNode> FixedSingleModelView for FixedSingleQuadricModel<M> {
	type Node = M;
	type Order = Quadric;

	#[inline]
	fn node(&self) -> &Self::Node {
		&self.node
	}

	#[inline]
	fn order(&self) -> &Self::Order {
		&QUADRIC
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
pub struct Prods<S: NodeSet> {
	order: usize,
	size: usize,
	indices: Vec<usize>,
	_phantom: PhantomData<S>,
}

impl<S: NodeSet> Prods<S> {
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

impl<S: NodeSet> std::iter::Iterator for Prods<S> {
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

pub struct Neighbors<S: NodeSet> {
	u: usize,
	item: usize,
	size: usize,
	_phantom: PhantomData<S>,
}

impl<S: NodeSet> Neighbors<S> {
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

impl<S: NodeSet> std::iter::Iterator for Neighbors<S> {
	type Item = S;
	fn next(&mut self) -> Option<S> {
		if self.item < self.size {
			let v = self.item;
			self.item += 1;
			match self.u.cmp(&v) {
				Ordering::Greater => Some(S::from_vec(vec![v, self.u]).unwrap()),
				Ordering::Equal => Some(S::from_vec(vec![v]).unwrap()),
				Ordering::Less => Some(S::from_vec(vec![self.u, v]).unwrap()),
			}
		} else {
			None
		}
	}
}

#[test]
fn proditer_test() {
	let mut it: Prods<[usize; 2]> = Prods::new(2, 4);
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
