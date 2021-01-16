/// The representation of *beta schedule* of some annealer.
/// Generally, beta schedule is array of `f64`, but effecient beta schedule is
/// generated from *beta range* or *beta count*. So you can specify them in
/// place of beta schedule.
use annealers::model::SingleModelView;
use annealers::node::{Node, SingleNode};
use annealers::variable::Real;

#[derive(Clone, Debug)]
pub enum BetaType<R: Real> {
	/// Specify beta schedule by *beta count*.
	Count(usize),
	/// Specify beta schedule by *beta count* and *beta range*.
	CountRange(usize, R, R),
	/// Specify *beta schedule* manually. This values should take larger
	/// as the index incleases.
	Schedule(Vec<R>),
}

macro_rules! real_typ {
	($p:ty) => {
		<<$p>::Node as Node>::RealType
	};
}

fn generate_beta_range<P: SingleModelView>(model: &P) -> (real_typ!(P), real_typ!(P)) {
	macro_rules! nan_or_min {
		() => {
			<real_typ!(P)>::nan_or(<real_typ!(P)>::MIN)
		};
	}
	let node = model.node();
	let ndiff = node.get_value(true) - node.get_value(false);
	let eg_min = model
		.prods()
		.into_iter()
		.map(|p| model.get_weight(&p).abs())
		.fold(nan_or_min!(), |p, n: real_typ!(P)| n.max(p));
	let eg_max = model
		.nodes()
		.into_iter()
		.map(|n| {
			model
				.neighbors(n)
				.into_iter()
				.map(|p| model.get_weight(&p).abs())
				.sum()
		})
		.fold(nan_or_min!(), |p, n: real_typ!(P)| n.max(p));
	if eg_max.is_finite() && eg_min.is_finite() {
		(
			<real_typ!(P)>::from_f64(f64::ln(2.0) / (ndiff * eg_max).as_f64()),
			<real_typ!(P)>::from_f64(f64::ln(100.0) / (ndiff * eg_min).as_f64()),
		)
	} else {
		(<real_typ!(P)>::one(), <real_typ!(P)>::from_i32(10))
	}
}

/// Generate *beta schedule* from given parameters.
/// the meanings of the parameters is the same of
/// [`AnnealerInfo::build_with_ising()`].
pub(crate) fn generate_schedule<P: SingleModelView>(
	beta_type: &BetaType<real_typ!(P)>,
	model: &P,
) -> Vec<real_typ!(P)> {
	match beta_type {
		BetaType::Schedule(v) => v.clone(),
		BetaType::Count(count) | BetaType::CountRange(count, _, _) => {
			let (min, max) = if let BetaType::CountRange(_, min, max) = beta_type {
				(*min, *max)
			} else {
				generate_beta_range(model)
			};
			generate_beta_schedule(min, max, *count)
		}
	}
}

fn generate_beta_schedule<R: Real>(beta_min: R, beta_max: R, count: usize) -> Vec<R> {
	let r = f64::ln(beta_max.as_f64() / beta_min.as_f64()) / (count as f64 - 1.0);
	(0..count)
		.map(|index| R::from_f64(beta_min.as_f64() * f64::exp(index as f64 * r)))
		.collect()
}
