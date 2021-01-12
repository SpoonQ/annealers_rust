pub mod model;
pub mod node;
pub mod repr;
pub mod set;
pub mod solution;
pub mod solver;
pub mod variable;

pub mod prelude {
	pub use crate::model::{FixedSingleModel, Model, SingleModel};
	pub use crate::solver::{ClassicalSolver, SolverGenerator, UnstructuredSolverGenerator};
}

pub mod order {
	use crate::set::NodeSet;
	use std::collections::BTreeSet;
	use std::fmt::Debug;
	use std::hash::Hash;

	pub trait Order:
		Clone + Debug + PartialEq + Eq + PartialOrd + Ord + Hash + Send + Sync
	{
		type NodeSetType: NodeSet;
		fn order(&self) -> usize;
	}

	#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct Quadric;

	impl Order for Quadric {
		type NodeSetType = [usize; 2];
		fn order(&self) -> usize {
			2
		}
	}

	impl Debug for Quadric {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.write_fmt(format_args!("{}", self.order()))
		}
	}

	#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct HighOrder(usize);

	impl HighOrder {
		pub fn new(order: usize) -> Self {
			Self(order)
		}
	}

	impl Order for HighOrder {
		type NodeSetType = BTreeSet<usize>;
		fn order(&self) -> usize {
			self.0
		}
	}

	impl Debug for HighOrder {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.write_fmt(format_args!("{}", self.order()))
		}
	}
}
