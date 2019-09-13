// Copyright (C) 2019 Centrality Investments Limited
// This file is part of CENNZnet.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
use parity_codec::{Decode, Encode, Input};
use primitives::{
	crypto::UncheckedFrom,
	ed25519::{self},
	sr25519::{self},
};
use rstd::prelude::*;

use crate::util::encode_with_vec_prefix;
use crate::{AccountId, Moment};
use runtime_primitives::doughnut::DoughnutV0;
use runtime_primitives::traits::{DoughnutApi, DoughnutVerify, Verify};

/// The CENNZnet doughnut type. It wraps an encoded v0 doughnut
/// Wrapping it like this provides length prefix support for the SCALE codec used by the extrinsic format
/// and type conversions into runtime data types
#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Eq, PartialEq, Clone)]
pub struct CennznetDoughnut(DoughnutV0);

impl CennznetDoughnut {
	/// Create a new CennznetDoughnut
	pub fn new(doughnut: DoughnutV0) -> Self {
		Self(doughnut)
	}
}

impl Decode for CennznetDoughnut {
	fn decode<I: Input>(input: &mut I) -> Option<Self> {
		// This is a little more complicated than usual since the binary format must be compatible
		// with substrate's generic `Vec<u8>` type. Basically this just means accepting that there
		// will be a prefix of vector length (we don't need to use this).
		let _length_do_not_remove_me_see_above: Vec<()> = Decode::decode(input)?;
		let doughnut = DoughnutV0::decode(input)?;
		Some(CennznetDoughnut(doughnut))
	}
}

impl Encode for CennznetDoughnut {
	fn encode(&self) -> Vec<u8> {
		encode_with_vec_prefix::<Self, _>(|v| self.0.encode_to(v))
	}
}

// TODO: Convert doughnut fields to runtime types here, remove shim traits from executive
impl DoughnutApi for CennznetDoughnut {
	/// The holder and issuer account id type
	type PublicKey = AccountId;
	/// The expiry timestamp type
	type Timestamp = Moment;
	/// The signature types
	type Signature = [u8; 64];
	/// Return the doughnut holder
	fn holder(&self) -> Self::PublicKey {
		AccountId::unchecked_from(self.0.holder())
	}
	/// Return the doughnut issuer
	fn issuer(&self) -> Self::PublicKey {
		AccountId::unchecked_from(self.0.issuer())
	}
	/// Return the doughnut expiry timestamp
	fn expiry(&self) -> Self::Timestamp {
		self.0.expiry().into()
	}
	/// Return the doughnut not_before timestamp
	fn not_before(&self) -> Self::Timestamp {
		self.0.not_before().into()
	}
	/// Return the doughnut payload bytes
	fn payload(&self) -> Vec<u8> {
		self.0.payload()
	}
	/// Return the doughnut signature
	fn signature(&self) -> Self::Signature {
		self.0.signature()
	}
	/// Return the payload for domain, if it exists in the doughnut
	fn get_domain(&self, domain: &str) -> Option<&[u8]> {
		self.0.get_domain(domain)
	}
	fn signature_version(&self) -> u8 {
		self.0.signature_version()
	}
}

// Re-implemented here due to sr25519 verification requiring an external
// wasm VM call when using `no std`
impl DoughnutVerify for CennznetDoughnut {
	/// Verify the doughnut signature. Returns `true` on success, false otherwise
	fn verify(&self) -> bool {
		match self.signature_version() {
			// sr25519
			0 => {
				let signature = sr25519::Signature(self.signature());
				let issuer = sr25519::Public(self.issuer().into());
				return sr25519::Signature::verify(&signature, &self.payload()[..], &issuer);
			}
			// ed25519
			1 => {
				let signature = ed25519::Signature(self.signature());
				let issuer = ed25519::Public(self.issuer().into());
				return ed25519::Signature::verify(&signature, &self.payload()[..], &issuer);
			}
			// signature version unsupported
			_ => false,
		}
	}
}
