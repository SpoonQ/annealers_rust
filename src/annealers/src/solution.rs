use crate::model::SingleModelView;
use crate::node::{Node, SingleNode};
use crate::repr::BinaryRepr;
use std::marker::PhantomData;

pub trait Solution {
	type Node: Node;
}

#[derive(Clone)]
pub struct SingleSolution<NodeType: SingleNode> {
	pub state: BinaryRepr,
	pub energy: Option<NodeType::RealType>,
	pub occurrences: usize,
	_phantom: PhantomData<NodeType>,
}

impl<M: SingleNode> Solution for SingleSolution<M> {
	type Node = M;
}

impl<M: SingleNode> SingleSolution<M> {
	/// Generate SingleSolution from qubit values
	pub fn from_vec(v: &[bool]) -> Self {
		Self::from_state(BinaryRepr::from_vec(v))
	}

	/// Generate SingleSolution from BinaryRepr
	pub fn from_state(state: BinaryRepr) -> Self {
		Self {
			state,
			energy: None,
			occurrences: 1,
			_phantom: PhantomData,
		}
	}

	/// Get the number of solutions
	pub fn len(&self) -> usize {
		self.state.len()
	}

	/// Compare two SingleSolution by energy.
	pub fn compare_energy(&self, other: &Self) -> Option<std::cmp::Ordering> {
		if let (Some(e1), Some(e2)) = (self.energy, other.energy) {
			e1.partial_cmp(&e2)
		} else {
			None
		}
	}

	/// Ensure that SingleSolution has energy.
	pub fn with_energy<P: SingleModelView<Node = M>>(mut self, model: &P) -> Self {
		self.energy = Some(self.calculate_energy(model));
		self
	}

	/// Calculate energy with model.
	pub fn calculate_energy<P: SingleModelView<Node = M>>(
		&self,
		model: &P,
	) -> <P::Node as Node>::RealType {
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

	/// Get the qubit value located in `index`.
	pub fn get(&self, index: usize) -> bool {
		self.state.get(index)
	}

	/// Get the qubit value located in `index`.
	/// # Safety
	/// Given index must be less than `len()`
	#[inline]
	pub unsafe fn get_unchecked(&self, index: usize) -> bool {
		self.state.get_unchecked(index)
	}
}

impl<M: SingleNode> std::ops::Index<usize> for SingleSolution<M> {
	type Output = bool;
	#[inline]
	fn index(&self, index: usize) -> &bool {
		if self.state.get(index) {
			&crate::TRUE_VAL
		} else {
			&crate::FALSE_VAL
		}
	}
}
