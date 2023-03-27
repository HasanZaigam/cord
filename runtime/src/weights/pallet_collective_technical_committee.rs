// This file is part of CORD – https://cord.network

// Copyright (C) Dhiway Networks Pvt. Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// CORD is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// CORD is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with CORD. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	/// Storage: TechnicalCommittee Members (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Voting (r:100 w:100)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (3232 ±0) + p * (3190 ±0)`
		//  Estimated: `19184 + m * (7799 ±17) + p * (10110 ±17)`
		// Minimum execution time: 18_644_000 picoseconds.
		Weight::from_parts(19_096_000, 0)
			.saturating_add(Weight::from_parts(0, 19184))
			// Standard Error: 48_031
			.saturating_add(Weight::from_parts(5_411_514, 0).saturating_mul(m.into()))
			// Standard Error: 48_031
			.saturating_add(Weight::from_parts(8_184_551, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_parts(0, 7799).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 10110).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `141 + m * (32 ±0)`
		//  Estimated: `1627 + m * (32 ±0)`
		// Minimum execution time: 17_826_000 picoseconds.
		Weight::from_parts(17_349_193, 0)
			.saturating_add(Weight::from_parts(0, 1627))
			// Standard Error: 19
			.saturating_add(Weight::from_parts(1_183, 0).saturating_mul(b.into()))
			// Standard Error: 198
			.saturating_add(Weight::from_parts(11_554, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `141 + m * (32 ±0)`
		//  Estimated: `5234 + m * (64 ±0)`
		// Minimum execution time: 19_874_000 picoseconds.
		Weight::from_parts(19_367_499, 0)
			.saturating_add(Weight::from_parts(0, 5234))
			// Standard Error: 20
			.saturating_add(Weight::from_parts(1_340, 0).saturating_mul(b.into()))
			// Standard Error: 215
			.saturating_add(Weight::from_parts(24_272, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalCount (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalCount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Voting (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `431 + m * (32 ±0) + p * (36 ±0)`
		//  Estimated: `9710 + m * (165 ±0) + p * (180 ±0)`
		// Minimum execution time: 25_910_000 picoseconds.
		Weight::from_parts(27_064_580, 0)
			.saturating_add(Weight::from_parts(0, 9710))
			// Standard Error: 69
			.saturating_add(Weight::from_parts(2_555, 0).saturating_mul(b.into()))
			// Standard Error: 724
			.saturating_add(Weight::from_parts(22_302, 0).saturating_mul(m.into()))
			// Standard Error: 715
			.saturating_add(Weight::from_parts(123_419, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 165).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 180).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[5, 100]`.
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `880 + m * (64 ±0)`
		//  Estimated: `6708 + m * (128 ±0)`
		// Minimum execution time: 23_818_000 picoseconds.
		Weight::from_parts(24_388_603, 0)
			.saturating_add(Weight::from_parts(0, 6708))
			// Standard Error: 242
			.saturating_add(Weight::from_parts(42_675, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 128).saturating_mul(m.into()))
	}
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `469 + m * (64 ±0) + p * (36 ±0)`
		//  Estimated: `8231 + m * (260 ±0) + p * (144 ±0)`
		// Minimum execution time: 28_872_000 picoseconds.
		Weight::from_parts(29_893_129, 0)
			.saturating_add(Weight::from_parts(0, 8231))
			// Standard Error: 400
			.saturating_add(Weight::from_parts(19_918, 0).saturating_mul(m.into()))
			// Standard Error: 390
			.saturating_add(Weight::from_parts(117_102, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 260).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 144).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `771 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `12392 + b * (4 ±0) + m * (264 ±0) + p * (160 ±0)`
		// Minimum execution time: 40_305_000 picoseconds.
		Weight::from_parts(41_411_443, 0)
			.saturating_add(Weight::from_parts(0, 12392))
			// Standard Error: 82
			.saturating_add(Weight::from_parts(2_252, 0).saturating_mul(b.into()))
			// Standard Error: 873
			.saturating_add(Weight::from_parts(22_533, 0).saturating_mul(m.into()))
			// Standard Error: 851
			.saturating_add(Weight::from_parts(146_263, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 4).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 264).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 160).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `489 + m * (64 ±0) + p * (36 ±0)`
		//  Estimated: `10265 + m * (325 ±0) + p * (180 ±0)`
		// Minimum execution time: 31_636_000 picoseconds.
		Weight::from_parts(32_125_817, 0)
			.saturating_add(Weight::from_parts(0, 10265))
			// Standard Error: 411
			.saturating_add(Weight::from_parts(26_426, 0).saturating_mul(m.into()))
			// Standard Error: 401
			.saturating_add(Weight::from_parts(119_480, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 325).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 180).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `791 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `14600 + b * (5 ±0) + m * (330 ±0) + p * (200 ±0)`
		// Minimum execution time: 42_654_000 picoseconds.
		Weight::from_parts(44_062_315, 0)
			.saturating_add(Weight::from_parts(0, 14600))
			// Standard Error: 88
			.saturating_add(Weight::from_parts(2_419, 0).saturating_mul(b.into()))
			// Standard Error: 937
			.saturating_add(Weight::from_parts(22_044, 0).saturating_mul(m.into()))
			// Standard Error: 914
			.saturating_add(Weight::from_parts(147_245, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 5).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 330).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 200).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Voting (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `298 + p * (32 ±0)`
		//  Estimated: `2379 + p * (96 ±0)`
		// Minimum execution time: 16_475_000 picoseconds.
		Weight::from_parts(18_514_104, 0)
			.saturating_add(Weight::from_parts(0, 2379))
			// Standard Error: 444
			.saturating_add(Weight::from_parts(113_460, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 96).saturating_mul(p.into()))
	}
}