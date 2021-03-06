/* Copyright 2019-2020 Centrality Investments Limited
*
* Licensed under the LGPL, Version 3.0 (the "License");
* you may not use this file except in compliance with the License.
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
* You may obtain a copy of the License at the root of this project source code,
* or at:
*     https://centrality.ai/licenses/gplv3.txt
*     https://centrality.ai/licenses/lgplv3.txt
*/
//!
//! Extra CENNZX-Spot traits + implementations
//!
use super::Trait;
use crate::Module;
use cennznet_primitives::{traits::BuyFeeAsset, types::FeeExchange};
use core::convert::TryInto;
use frame_support::dispatch::{DispatchError, DispatchResult};
use sp_core::crypto::{UncheckedFrom, UncheckedInto};
use sp_runtime::traits::Hash;
use sp_std::{marker::PhantomData, prelude::*};

/// A function that generates an `AccountId` for a CENNZX-SPOT exchange / (core, asset) pair
pub trait ExchangeAddressFor<AssetId: Sized, AccountId: Sized> {
	fn exchange_address_for(core_asset_id: AssetId, asset_id: AssetId) -> AccountId;
}

// A CENNZX-Spot exchange address generator implementation
pub struct ExchangeAddressGenerator<T: Trait>(PhantomData<T>);

impl<T: Trait> ExchangeAddressFor<T::AssetId, T::AccountId> for ExchangeAddressGenerator<T>
where
	T::AccountId: UncheckedFrom<T::Hash> + AsRef<[u8]>,
	T::AssetId: Into<u64>,
{
	/// Generates an exchange address for the given core / asset pair
	fn exchange_address_for(core_asset_id: T::AssetId, asset_id: T::AssetId) -> T::AccountId {
		let mut buf = Vec::new();
		buf.extend_from_slice(b"cennz-x-spot:");
		buf.extend_from_slice(&u64_to_bytes(core_asset_id.into()));
		buf.extend_from_slice(&u64_to_bytes(asset_id.into()));

		T::Hashing::hash(&buf[..]).unchecked_into()
	}
}

fn u64_to_bytes(x: u64) -> [u8; 8] {
	x.to_le_bytes()
}

impl<T: Trait> BuyFeeAsset<T::AccountId, T::Balance> for Module<T> {
	type FeeExchange = FeeExchange;
	/// Use the CENNZX-Spot exchange to seamlessly buy fee asset
	fn buy_fee_asset(who: &T::AccountId, amount: T::Balance, exchange_op: &Self::FeeExchange) -> DispatchResult {
		let fee_exchange = match exchange_op {
			FeeExchange::V1(ex) => ex,
		};

		// TODO: Hard coded to use spending asset ID
		let fee_asset_id: T::AssetId = <pallet_generic_asset::Module<T>>::spending_asset_id();
		let max_payment = fee_exchange
			.max_payment
			.try_into()
			.map_err(|_| DispatchError::Other("Failed to convert max payment to balance type"))?;

		Self::make_asset_swap_output(
			&who,
			&who,
			&T::AssetId::from(fee_exchange.asset_id),
			&fee_asset_id,
			amount,
			max_payment,
			Self::fee_rate(),
		)
		.map(|_| ())
		.map_err(|_| DispatchError::Other("Failed to charge transaction fees during conversion"))
	}
}

#[cfg(test)]
pub(crate) mod impl_tests {
	use super::*;
	use crate::{
		mock::{self, CORE_ASSET_ID, FEE_ASSET_ID, TRADE_ASSET_A_ID},
		tests::{CennzXSpot, ExtBuilder, Test},
	};
	use cennznet_primitives::types::FeeExchangeV1;
	use frame_support::traits::Currency;
	use sp_core::H256;

	type CoreAssetCurrency = mock::CoreAssetCurrency<Test>;
	type TradeAssetCurrencyA = mock::TradeAssetCurrencyA<Test>;
	type FeeAssetCurrency = mock::FeeAssetCurrency<Test>;

	#[test]
	fn buy_fee_asset() {
		ExtBuilder::default().build().execute_with(|| {
			with_exchange!(CoreAssetCurrency => 10_000, TradeAssetCurrencyA => 10_000);
			with_exchange!(CoreAssetCurrency => 10_000, FeeAssetCurrency => 10_000);

			let user = with_account!(CoreAssetCurrency => 0, TradeAssetCurrencyA => 1_000);
			let target_fee = 510;
			let scale_factor = 1_000_000;
			let fee_rate = 3_000; // fee is 0.3%
			let fee_rate_factor = scale_factor + fee_rate; // 1_000_000 + 3_000

			assert_ok!(<CennzXSpot as BuyFeeAsset<_, _>>::buy_fee_asset(
				&user,
				target_fee,
				&FeeExchange::V1(FeeExchangeV1::new(TRADE_ASSET_A_ID, 2_000_000)),
			));

			// For more detail, see `fn get_output_price` in lib.rs
			let core_asset_price = {
				let output_amount = target_fee;
				let input_reserve = 10_000; // CoreAssetCurrency reserve
				let output_reserve = 10_000; // FeeAssetCurrency reserve
				let denom = output_reserve - output_amount; // 10000 - 510 = 9490
				let res = (input_reserve * output_amount) / denom; // 537 (decimals truncated)
				let price = res + 1; // 537 + 1 = 538
				(price * fee_rate_factor) / scale_factor // price adjusted with fee
			};

			let trade_asset_price = {
				let output_amount = core_asset_price;
				let input_reserve = 10_000; // TradeAssetCurrencyA reserve
				let output_reserve = 10_000; // CoreAssetCurrency reserve
				let denom = output_reserve - output_amount; // 10000 - 539 = 9461
				let res = (input_reserve * output_amount) / denom; // 569 (decimals truncated)
				let price = res + 1; // 569 + 1 = 570
				(price * fee_rate_factor) / scale_factor // price adjusted with fee
			};

			assert_eq!(core_asset_price, 539);
			assert_eq!(trade_asset_price, 571);

			let exchange1_core = 10_000 - core_asset_price;
			let exchange1_trade = 10_000 + trade_asset_price;

			let exchange2_core = 10_000 + core_asset_price;
			let exchange2_fee = 10_000 - target_fee;

			assert_exchange_balance_eq!(
				CoreAssetCurrency => exchange1_core,
				TradeAssetCurrencyA => exchange1_trade
			);
			assert_exchange_balance_eq!(
				CoreAssetCurrency => exchange2_core,
				FeeAssetCurrency => exchange2_fee
			);

			let trade_asset_remainder = 1_000 - trade_asset_price;
			assert_balance_eq!(user, CoreAssetCurrency => 0);
			assert_balance_eq!(user, FeeAssetCurrency => target_fee);
			assert_balance_eq!(user, TradeAssetCurrencyA => trade_asset_remainder);
		});
	}

	#[test]
	fn buy_fee_asset_insufficient_trade_asset() {
		ExtBuilder::default().build().execute_with(|| {
			let user = with_account!(CoreAssetCurrency => 0, TradeAssetCurrencyA => 10);

			assert_err!(
				<CennzXSpot as BuyFeeAsset<_, _>>::buy_fee_asset(
					&user,
					51,
					&FeeExchange::V1(FeeExchangeV1::new(TRADE_ASSET_A_ID, 2_000_000)),
				),
				"Failed to charge transaction fees during conversion"
			);

			assert_balance_eq!(user, CoreAssetCurrency => 0);
			assert_balance_eq!(user, TradeAssetCurrencyA => 10);
		});
	}

	#[test]
	fn u64_to_bytes_works() {
		assert_eq!(u64_to_bytes(80_000), [128, 56, 1, 0, 0, 0, 0, 0]);
	}
}
