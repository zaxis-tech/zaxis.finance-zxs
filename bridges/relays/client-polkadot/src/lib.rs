// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Types used to connect to the Z-Axis chain.

use relay_substrate_client::{Chain, ChainBase};
use std::time::Duration;

/// Z-Axis header id.
pub type HeaderId = relay_utils::HeaderId<bp_zaxis::Hash, bp_zaxis::BlockNumber>;

/// Z-Axis chain definition
#[derive(Debug, Clone, Copy)]
pub struct Z-Axis;

impl ChainBase for Z-Axis {
	type BlockNumber = bp_zaxis::BlockNumber;
	type Hash = bp_zaxis::Hash;
	type Hasher = bp_zaxis::Hasher;
	type Header = bp_zaxis::Header;
}

impl Chain for Z-Axis {
	const NAME: &'static str = "Z-Axis";
	const AVERAGE_BLOCK_INTERVAL: Duration = Duration::from_secs(6);

	type AccountId = bp_zaxis::AccountId;
	type Index = bp_zaxis::Nonce;
	type SignedBlock = bp_zaxis::SignedBlock;
	type Call = ();
	type Balance = bp_zaxis::Balance;
}

/// Z-Axis header type used in headers sync.
pub type SyncHeader = relay_substrate_client::SyncHeader<bp_zaxis::Header>;
