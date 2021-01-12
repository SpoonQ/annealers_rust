use annealers::model::SingleModel;
use annealers::node::{Binary, Node, SingleNode};
use annealers::repr::BinaryRepr;
use annealers::set::NodeSet;
use annealers::variable::Real;
use rand::prelude::*;

#[inline]
unsafe fn calculate_flip_cost<S: NodeSet, M: SingleNode>(
	node: &M,
	prod: &S,
	state: &BinaryRepr,
	index: usize,
) -> M::RealType {
	let term = prod.to_it().fold(M::RealType::from_i32(1), |p, i| {
		if i == index {
			p
		} else {
			p * node.get_value(state.get_unchecked(i))
		}
	});
	let d = node.get_value(false) - node.get_value(true);
	if state.get_unchecked(index) {
		d * term
	} else {
		-d * term
	}
}

pub fn simulated_annealing<T: Rng, P: SingleModel<NodeType = Binary<R>>, R: Real>(
	random: &mut T,
	state: &mut BinaryRepr,
	beta_schedule: &[<P::NodeType as Node>::RealType],
	sweeps_per_round: usize,
	model: &P,
) {
	assert!(state.len() == model.size());
	let size = model.size();
	let node = model.node();
	let mut energy_diffs =
		std::iter::repeat(<<P::NodeType as Node>::RealType as Default>::default())
			.take(size)
			.collect::<Vec<_>>();
	let d = node.get_value(true) - node.get_value(false);
	let dd = d * d;
	for prod in model.prods() {
		let weight = model.get_weight(&prod);
		for i in prod.to_it() {
			energy_diffs[i] += unsafe { calculate_flip_cost(&node, &prod, &state, i) } * weight;
		}
	}
	for beta in beta_schedule.iter() {
		for _ in 0..sweeps_per_round {
			let threshold = 44.36142 / beta.as_f64();
			for i in 0..state.len() {
				let ed = energy_diffs[i];
				if ed.as_f64() > threshold {
					continue;
				}
				if ed.as_f64() <= 0.0
					|| f64::exp(-(ed * *beta).as_f64()) > random.gen_range(0.0, 1.0)
				{
					unsafe {
						state.flip_unchecked(i);
					}
					let stat = unsafe { state.get_unchecked(i) };
					energy_diffs[i] *= <P::NodeType as Node>::RealType::from_i32(-1);
					for neigh in model.neighbors(i) {
						if neigh.len() != 1 {
							let weight = model.get_weight(&neigh);
							for j in neigh.to_it() {
								if i != j {
									if stat != unsafe { state.get_unchecked(j) } {
										energy_diffs[j] += dd * weight;
									} else {
										energy_diffs[j] -= dd * weight;
									}
								}
							}
						}
					}
				}
			}
		}
	}
}

// T: 5, F: 3
//
// i: T -> F
//
// j, weight に対して
//
// j = F なら
//
// もともと 15W -> 25W eff = 10W ( = 5 * 5 - 5 * 3 )
// 今は　　9W -> 15W   eff = 6W ( = 5 * 3 - 3 * 3 )
// eff -= 4W
//
// j = T なら
//
// もともと　25W -> 15W  eff = -10W ( = 5 * 3 - 5 * 5 )
// 今は      15W -> 9W   eff  = -6W ( = 3 * 3 - 5 * 3 )
// eff += 4W
//
// 4 = FF - TF - TF + TT = (T - F) ^ 2
// i: F -> T
