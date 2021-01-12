//! Solver module contains abstraction of many solver type.
//!
//! # Trait `SolverGenerator`
//!
//! ```text
//!            SolverGenerator
//!                  /   \
//!                 /     \
//!      Unstructured     (Structured)
//! ```
//!
//! SolverGenerator is classified as one of:
//!
//! - Unsized and Unstructured
//! - Sized and Unstructured
//! - Structured
//!
//! If the solver is unsized (which means the maximum node number is infinity),
//! implement `UnsizedSolverGenerator` trait. Then `UnstructuredSolverGenerator`
//! trait and `SolverGenerator` trait are automatically derived.
//!
//! If the solver has fixed number of the nodes, implement
//! `UnsizedSolverGenerator` trait. Then `SolverGenerator` trait is
//! automatically derived.
//!
//! If the solver has fixed structure (like chimera graph), implement
//! `SolverGenerator` trait directly.
//!
//! # Trait `Solver`
//!
//! All solvers must implement `Solver` trait manually. Then implement some of
//! the following traits:
//!
//! - `AsyncSolver`
//! - `SyncSolver`
//! - `RngSolver`
extern crate async_trait;
use crate::model::Model;
use crate::node::Node;
use crate::order::Order;
use crate::solution::Solution;
use crate::variable::Real;
use async_trait::async_trait;
use rand::prelude::*;
use std::collections::BTreeSet;
use std::error::Error;
use std::iter::Iterator;
use std::marker::PhantomData;

macro_rules! get_real_typ {
	($typ:ty) => {
		<<$typ as Model>::NodeType as Node>::RealType
	};
}

pub trait SolverGenerator<ProblemType: Model> {
	type SolverType: Solver<ErrorType = Self::ErrorType>;
	type ErrorType: Error + Send + Sync;

	fn value_range(&self) -> (get_real_typ!(ProblemType), get_real_typ!(ProblemType)) {
		(
			<get_real_typ!(ProblemType)>::MAX,
			<get_real_typ!(ProblemType)>::MAX,
		)
	}

	fn generate(&self, model: &ProblemType) -> Result<Self::SolverType, Self::ErrorType>;
}

pub trait StructuredSolverGenerator<ProblemType: Model>: SolverGenerator<ProblemType> {
	fn nodes(&self) -> Box<dyn Iterator<Item = usize>>;
	fn prods(&self) -> Box<dyn Iterator<Item = BTreeSet<usize>>>;
}

pub trait UnstructuredSolverGenerator<ProblemType: Model>: SolverGenerator<ProblemType> {
	type Order: Order;

	fn order(&self) -> Self::Order;
	fn size(&self) -> Option<usize> {
		None
	}

	fn to_structured(self) -> AsStructuredSolverGeneratorWrapper<Self, ProblemType>
	where
		Self: Sized,
	{
		AsStructuredSolverGeneratorWrapper(self, PhantomData)
	}
}

pub struct AsStructuredSolverGeneratorWrapper<G: UnstructuredSolverGenerator<P>, P: Model>(
	G,
	PhantomData<P>,
);

impl<G: UnstructuredSolverGenerator<P>, P: Model> SolverGenerator<P>
	for AsStructuredSolverGeneratorWrapper<G, P>
{
	type SolverType = G::SolverType;
	type ErrorType = G::ErrorType;
	fn generate(&self, model: &P) -> Result<Self::SolverType, Self::ErrorType> {
		self.0.generate(model)
	}
}

impl<G: UnstructuredSolverGenerator<P>, P: Model> StructuredSolverGenerator<P>
	for AsStructuredSolverGeneratorWrapper<G, P>
{
	fn nodes(&self) -> Box<dyn Iterator<Item = usize>> {
		if let Some(cap) = self.0.size() {
			Box::new(0..cap) as Box<dyn Iterator<Item = usize>>
		} else {
			Box::new(0usize..) as Box<dyn Iterator<Item = usize>>
		}
	}

	fn prods(&self) -> Box<dyn Iterator<Item = BTreeSet<usize>>> {
		Box::new(UnstructuredEdgeIter::from_iter(
			self.nodes(),
			self.0.order().order(),
		)) as Box<dyn Iterator<Item = BTreeSet<usize>>>
	}
}

pub trait Solver: Send + Sync {
	type ErrorType: Error;
	type SolutionType: Solution;
}

pub trait ClassicalSolver: Solver {
	fn solve_with_rng<T: Rng>(
		&self,
		_r: &mut T,
	) -> Result<Vec<<Self as Solver>::SolutionType>, <Self as Solver>::ErrorType>;
}

#[async_trait]
pub trait AsyncSolver: Solver {
	async fn solve_async(
		&self,
	) -> Result<Vec<<Self as Solver>::SolutionType>, <Self as Solver>::ErrorType>;
}

pub trait SyncSolver: Solver {
	fn solve(&self) -> Result<Vec<<Self as Solver>::SolutionType>, <Self as Solver>::ErrorType>;
}

#[test]
fn unstructured_edge_iter_test() {
	let iter = Box::new(2usize..5) as Box<dyn Iterator<Item = usize>>;
	let mut iter = UnstructuredEdgeIter::from_iter(iter, 2);
	assert_eq!(
		iter.next(),
		Some(vec![2, 3].into_iter().collect::<BTreeSet<_>>())
	);
	assert_eq!(
		iter.next(),
		Some(vec![2, 4].into_iter().collect::<BTreeSet<_>>())
	);
	assert_eq!(
		iter.next(),
		Some(vec![3, 4].into_iter().collect::<BTreeSet<_>>())
	);
	assert_eq!(iter.next(), None);
}

pub struct UnstructuredEdgeIter(usize, usize, Vec<usize>, Box<dyn Iterator<Item = usize>>);

impl UnstructuredEdgeIter {
	fn from_iter(iter: Box<dyn Iterator<Item = usize>>, max_order: usize) -> Self {
		if max_order != 2 {
			unimplemented!();
		}
		Self(0, 0, Vec::new(), iter)
	}
}

impl Iterator for UnstructuredEdgeIter {
	type Item = BTreeSet<usize>;
	fn next(&mut self) -> Option<BTreeSet<usize>> {
		if self.0 == self.1 {
			self.1 += 1;
			self.0 = 0;
			while self.2.len() <= self.1 {
				self.2.push(self.3.next()?);
			}
			self.next()
		} else {
			let i = self.2[self.0];
			let j = self.2[self.1];
			self.0 += 1;
			Some({
				let mut s = BTreeSet::new();
				s.insert(i);
				s.insert(j);
				s
			})
		}
	}
}
