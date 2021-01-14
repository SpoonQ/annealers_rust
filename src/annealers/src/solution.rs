use crate::model::SingleModel;
use crate::node::{Node, SingleNode};
use crate::repr::BinaryRepr;
use std::marker::PhantomData;

pub trait Solution {
	type NodeType: Node;
}

#[derive(Clone)]
pub struct SingleSolution<NodeType: SingleNode> {
	pub state: BinaryRepr,
	pub energy: Option<NodeType::RealType>,
	pub occurrences: usize,
	_phantom: PhantomData<NodeType>,
}

impl<M: SingleNode> Solution for SingleSolution<M> {
	type NodeType = M;
}

impl<M: SingleNode> SingleSolution<M> {
	pub fn from_value(value: &[bool]) -> Self {
		Self::from_state(BinaryRepr::from_vec(value))
	}

	pub fn from_vec(v: &[bool]) -> Self {
		Self::from_state(BinaryRepr::from_vec(v))
	}

	pub fn from_state(state: BinaryRepr) -> Self {
		Self {
			state,
			energy: None,
			occurrences: 1,
			_phantom: PhantomData,
		}
	}

	pub fn compare_energy(&self, other: &Self) -> Option<std::cmp::Ordering> {
		if let (Some(e1), Some(e2)) = (self.energy, other.energy) {
			e1.partial_cmp(&e2)
		} else {
			None
		}
	}

	pub fn with_energy<P: SingleModel<NodeType = M>>(mut self, model: &P) -> Self {
		self.energy = Some(self.calculate_energy(model));
		self
	}

	pub fn calculate_energy<P: SingleModel<NodeType = M>>(
		&self,
		model: &P,
	) -> <P::NodeType as Node>::RealType {
		if let Some(e) = self.energy {
			e
		} else {
			let mut energy = Default::default();
			for prod in model.prods() {
				energy += model.get_weight(&prod) * model.calculate_prod(&prod, self);
			}
			energy
		}
	}

	#[inline]
	pub unsafe fn get_unchecked(&self, index: usize) -> &bool {
		if self.state.get_unchecked(index) {
			&TRUE_VAL
		} else {
			&FALSE_VAL
		}
	}
}

const TRUE_VAL: bool = true;
const FALSE_VAL: bool = false;
impl<M: SingleNode> std::ops::Index<usize> for SingleSolution<M> {
	type Output = bool;
	#[inline]
	fn index(&self, index: usize) -> &bool {
		if self.state.get(index) {
			&TRUE_VAL
		} else {
			&FALSE_VAL
		}
	}
}
