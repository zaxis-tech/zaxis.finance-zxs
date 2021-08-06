// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Z-Axis.

// Z-Axis is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Z-Axis is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Z-Axis.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_identity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-07-01, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 128

// Executed Command:
// target/release/zaxis
// benchmark
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=pallet_identity
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
	fn add_registrar(r: u32) -> Weight {
		(21_339_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((236_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_identity(r: u32, x: u32) -> Weight {
		(50_839_000 as Weight)
			// Standard Error: 14_000
			.saturating_add((223_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 1_000
			.saturating_add((954_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_subs_new(s: u32) -> Weight {
		(40_807_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((6_374_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn set_subs_old(p: u32) -> Weight {
		(40_924_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_059_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	fn clear_identity(r: u32, s: u32, x: u32) -> Weight {
		(51_123_000 as Weight)
			// Standard Error: 7_000
			.saturating_add((110_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 0
			.saturating_add((2_042_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((595_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn request_judgement(r: u32, x: u32) -> Weight {
		(53_230_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((223_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 0
			.saturating_add((1_169_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn cancel_request(r: u32, x: u32) -> Weight {
		(48_425_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((161_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 0
			.saturating_add((1_161_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_fee(r: u32) -> Weight {
		(8_028_000 as Weight)
			// Standard Error: 0
			.saturating_add((202_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_account_id(r: u32) -> Weight {
		(8_682_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((203_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_fields(r: u32) -> Weight {
		(8_010_000 as Weight)
			// Standard Error: 0
			.saturating_add((203_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn provide_judgement(r: u32, x: u32) -> Weight {
		(34_291_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((203_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 0
			.saturating_add((1_164_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn kill_identity(r: u32, s: u32, x: u32) -> Weight {
		(62_644_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((111_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 0
			.saturating_add((2_048_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn add_sub(s: u32) -> Weight {
		(54_234_000 as Weight)
			// Standard Error: 0
			.saturating_add((154_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn rename_sub(s: u32) -> Weight {
		(16_622_000 as Weight)
			// Standard Error: 0
			.saturating_add((21_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn remove_sub(s: u32) -> Weight {
		(55_325_000 as Weight)
			// Standard Error: 0
			.saturating_add((138_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn quit_sub(s: u32) -> Weight {
		(34_002_000 as Weight)
			// Standard Error: 0
			.saturating_add((138_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
