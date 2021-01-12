use crate::algo::simulated_annealing;
use crate::beta::BetaType;
use crate::NoneError;
use annealers::model::{FixedSingleModel, SingleModel};
use annealers::node::{Binary, Node};
use annealers::order::Quadric;
use annealers::repr::BinaryRepr;
use annealers::solution::SingleSolution;
use annealers::solver::{ClassicalSolver, Solver, SolverGenerator, UnstructuredSolverGenerator};
use annealers::variable::Real;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct SimulatedAnnealerGenerator<P: SingleModel> {
	pub sweeps_per_round: usize,
	pub beta: BetaType<<P::NodeType as Node>::RealType>,
	_phantom: PhantomData<P>,
}

pub struct SimulatedAnnealer<P: FixedSingleModel, R> {
	sweeps_per_round: usize,
	beta_schedule: Vec<<P::NodeType as Node>::RealType>,
	model: P,
	_phantom: PhantomData<R>,
}

impl<P: FixedSingleModel> SimulatedAnnealerGenerator<P> {
	pub fn new() -> Self {
		Self {
			sweeps_per_round: 30,
			beta: BetaType::Count(100),
			_phantom: PhantomData,
		}
	}
}

impl<P: FixedSingleModel + Send + Sync> SolverGenerator<P> for SimulatedAnnealerGenerator<P> {
	type SolverType = SimulatedAnnealer<P, <P::NodeType as Node>::RealType>;
	type ErrorType = NoneError;

	fn generate(&self, model: &P) -> Result<Self::SolverType, Self::ErrorType> {
		// TODO: prevent copying model
		let schedule = crate::beta::generate_schedule(&self.beta, model);
		Ok(SimulatedAnnealer {
			sweeps_per_round: self.sweeps_per_round,
			beta_schedule: schedule,
			model: model.clone(),
			_phantom: PhantomData,
		})
	}
}

impl<P: FixedSingleModel + Send + Sync> UnstructuredSolverGenerator<P>
	for SimulatedAnnealerGenerator<P>
{
	type Order = Quadric;
	fn order(&self) -> Quadric {
		Quadric
	}
}

impl<P: FixedSingleModel + Send + Sync> Solver
	for SimulatedAnnealer<P, <P::NodeType as Node>::RealType>
{
	type ErrorType = NoneError;
	type SolutionType = SingleSolution<P::NodeType>;
}

impl<R: Real, P: FixedSingleModel<NodeType = Binary<R>> + Send + Sync> ClassicalSolver
	for SimulatedAnnealer<P, R>
{
	fn solve_with_rng<T: rand::Rng>(
		&self,
		r: &mut T,
	) -> Result<Vec<SingleSolution<P::NodeType>>, NoneError> {
		//let mut state = BinaryRepr::new_random(self.model.size(), r);
		let mut state = BinaryRepr::from_vec(&vec![true, false, true]);
		simulated_annealing(
			r,
			&mut state,
			self.beta_schedule.as_slice(),
			self.sweeps_per_round,
			&self.model,
		);
		Ok(vec![SingleSolution::from_state(state)])
	}
}
