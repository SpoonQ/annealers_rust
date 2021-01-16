use crate::algo::simulated_annealing;
use crate::beta::BetaType;
use crate::NoneError;
use annealers::model::SingleModelView;
use annealers::node::{Binary, Node};
use annealers::order::Quadric;
use annealers::repr::BinaryRepr;
use annealers::solution::SingleSolution;
use annealers::solver::{ClassicalSolver, Solver, SolverGenerator, UnstructuredSolverGenerator};
use annealers::variable::Real;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct SimulatedAnnealerGenerator<'a, P: SingleModelView> {
	pub sweeps_per_round: usize,
	pub beta: BetaType<<P::Node as Node>::RealType>,
	_phantom: PhantomData<&'a P>,
}

pub struct SimulatedAnnealer<'a, P: SingleModelView, R> {
	sweeps_per_round: usize,
	beta_schedule: Vec<<P::Node as Node>::RealType>,
	model: &'a P,
	_phantom: PhantomData<R>,
}

impl<'a, P: SingleModelView> SimulatedAnnealerGenerator<'a, P> {
	pub fn new() -> Self {
		Self {
			sweeps_per_round: 30,
			beta: BetaType::Count(100),
			_phantom: PhantomData,
		}
	}
}

impl<'a, P: SingleModelView + Send + Sync> SolverGenerator<'a, P>
	for SimulatedAnnealerGenerator<'a, P>
{
	type SolverType = SimulatedAnnealer<'a, P, <P::Node as Node>::RealType>;
	type ErrorType = NoneError;

	fn generate(&self, model: &'a P) -> Result<Self::SolverType, Self::ErrorType> {
		// TODO: prevent copying model
		let schedule = crate::beta::generate_schedule(&self.beta, model);
		Ok(SimulatedAnnealer {
			sweeps_per_round: self.sweeps_per_round,
			beta_schedule: schedule,
			model: model,
			_phantom: PhantomData,
		})
	}
}

impl<'a, P: SingleModelView + Send + Sync> UnstructuredSolverGenerator<'a, P>
	for SimulatedAnnealerGenerator<'a, P>
{
	type Order = Quadric;
	fn order(&self) -> Quadric {
		Quadric
	}
}

impl<'a, P: SingleModelView + Send + Sync> Solver
	for SimulatedAnnealer<'a, P, <P::Node as Node>::RealType>
{
	type ErrorType = NoneError;
	type SolutionType = SingleSolution<P::Node>;
}

impl<'a, R: Real, P: SingleModelView<Node = Binary<R>> + Send + Sync> ClassicalSolver
	for SimulatedAnnealer<'a, P, R>
{
	fn solve_with_rng<T: rand::Rng>(
		&self,
		r: &mut T,
	) -> Result<Vec<SingleSolution<P::Node>>, NoneError> {
		let mut state = BinaryRepr::new_random(self.model.size(), r);
		// let mut state = BinaryRepr::from_vec(&vec![true, false, true]);
		simulated_annealing(
			r,
			&mut state,
			self.beta_schedule.as_slice(),
			self.sweeps_per_round,
			self.model,
		);
		Ok(vec![SingleSolution::from_state(state)])
	}
}
