extern crate annealers;
extern crate classical_solver;
extern crate rand;

use annealers::model::FixedSingleQuadricModel;
use annealers::node::Binary;
use annealers::prelude::*;
use classical_solver::sa::SimulatedAnnealerGenerator;

#[test]
fn sa_test() {
	let mut model = FixedSingleQuadricModel::new(Binary::new(), 3);
	model.add_weight(0, 1, 3.0f64);
	model.add_weight(0, 2, 3.0);
	model.add_weight(0, 0, -3.0);
	let mut gen = SimulatedAnnealerGenerator::new();
	gen.sweeps_per_round = 1;

	let solver = gen.generate(&model).unwrap();
	let solutions = solver.solve_with_rng(&mut rand::thread_rng()).unwrap();
	for sol in solutions.iter() {
		assert_eq!(sol.state.to_vec(), vec![true, false, false]);
	}
}
