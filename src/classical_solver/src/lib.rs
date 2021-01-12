//! Default annealing implementation.
//!
//! This annealer use *simulated annealing* on local machine.
//!
//! # Example:
//! ```ignore
//! use annealers::anneal::DefaultAnnealerInfo;
//! use annealers::prelude::*;
//! let ainfo = DefaultAnnealerInfo::new();
//! let h = vec![1.0, -1.0, 0.0];
//! let neighbors = vec![vec![(2, 1.0)], vec![], vec![]];
//! let annealer = ainfo.build_with_ising(h, neighbors).unwrap();
//! assert_eq!(annealer.anneal().unwrap(), vec![false, true, true]);
//! ```

extern crate annealers;
extern crate rand;

pub mod algo;
pub mod beta;
pub mod sa;

/// `NoneError` means the error will never be returned. It will be replaced with
/// `!` when `!` type annotations is stabilized.
#[derive(Debug)]
pub enum NoneError {}
impl std::fmt::Display for NoneError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Debug::fmt(self, f)
	}
}

impl std::error::Error for NoneError {}
