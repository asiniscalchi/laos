// Copyright 2023-2024 Freeverse.io
// This file is part of LAOS.

// LAOS is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// LAOS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with LAOS.  If not, see <http://www.gnu.org/licenses/>.

//! LAOS precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::ExitError;
use frame_support::{pallet_prelude::Weight, DefaultNoBound};
use frame_system::{pallet_prelude::BlockNumberFor, RawOrigin};
use pallet_evm::GasWeightMapping;
use pallet_vesting::{Pallet as PalletVesting, VestingInfo as VestingInfoP};
use precompile_utils::prelude::{revert, solidity, Address, EvmResult, PrecompileHandle};
use scale_info::prelude::{format, string::String};
use sp_core::U256;
use sp_runtime::{
	traits::{Convert, ConvertBack, PhantomData, StaticLookup},
	DispatchError,
};
use sp_std::vec::Vec;

#[derive(Default, solidity::Codec)]
pub struct VestingInfo {
	locked: U256,
	per_block: U256,
	starting_block: U256,
}

// WrapperVestingInfo is a wrapper around the VestingInfo struct, adding a marker to keep track
// of the Runtime type. This is used to bridge the types from the pallet_vesting to the precompile context.
struct WrapperVestingInfo<Runtime> {
	inner: VestingInfo,
	_marker: PhantomData<Runtime>,
}

impl<Runtime: Config> From<VestingInfoP<BalanceOf<Runtime>, BlockNumberFor<Runtime>>>
	for WrapperVestingInfo<Runtime>
where
	BalanceOf<Runtime>: Into<U256>,
	BlockNumberFor<Runtime>: Into<U256>,
{
	fn from(vesting_info: VestingInfoP<BalanceOf<Runtime>, BlockNumberFor<Runtime>>) -> Self {
		Self {
			inner: VestingInfo {
				locked: Runtime::BalanceOfToU256::convert(vesting_info.locked()),
				per_block: Runtime::BalanceOfToU256::convert(vesting_info.per_block()),
				starting_block: Runtime::BlockNumberForToU256::convert(
					vesting_info.starting_block(),
				),
			},
			_marker: PhantomData,
		}
	}
}

impl<Runtime> From<VestingInfo> for VestingInfoP<BalanceOf<Runtime>, BlockNumberFor<Runtime>>
where
	Runtime: Config,
	BalanceOf<Runtime>: Into<U256>,
	BlockNumberFor<Runtime>: Into<U256>,
{
	fn from(vesting: VestingInfo) -> Self {
		Self::new(
			Runtime::BalanceOfToU256::convert(vesting.locked.into()),
			Runtime::BalanceOfToU256::convert(vesting.per_block),
			Runtime::BlockNumberForToU256::convert(vesting.starting_block),
		)
	}
}

#[derive(Clone, DefaultNoBound)]
pub struct VestingPrecompile<Runtime>(PhantomData<Runtime>);

impl<Runtime> VestingPrecompile<Runtime> {
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

#[precompile_utils::precompile]
impl<Runtime> VestingPrecompile<Runtime>
where
	Runtime: Config,
{
	#[precompile::public("vesting(address)")]
	#[precompile::view]
	pub fn vesting(
		handle: &mut impl PrecompileHandle,
		account: Address,
	) -> EvmResult<Vec<VestingInfo>> {
		match PalletVesting::<Runtime>::vesting(Runtime::AccountIdToH160::convert_back(
			account.into(),
		)) {
			Some(v) => {
				register_cost::<Runtime>(
					handle,
					<Runtime as wrapper::pallet::Config>::WeightInfo::precompile_vesting(
						v.len() as u32
					),
				)?;
				let output: Vec<WrapperVestingInfo<Runtime>> =
					v.into_iter().map(|i| i.into()).collect();

				Ok(output.into_iter().map(|i| i.inner).collect())
			},
			None => {
				register_cost::<Runtime>(
					handle,
					<Runtime as wrapper::pallet::Config>::WeightInfo::precompile_vesting(0),
				)?;
				Ok(Vec::new())
			},
		}
	}

	#[precompile::public("vest()")]
	pub fn vest(handle: &mut impl PrecompileHandle) -> EvmResult<()> {
		register_cost::<Runtime>(
			handle,
			<Runtime as wrapper::pallet::Config>::WeightInfo::precompile_vest(),
		)?;

		match PalletVesting::<Runtime>::vest(
			<Runtime as frame_system::Config>::RuntimeOrigin::from(RawOrigin::from(Some(
				Runtime::AccountIdToH160::convert_back(handle.context().caller),
			))),
		) {
			Ok(_) => Ok(()),
			Err(err) => Err(revert(convert_dispatch_error_to_string(err))),
		}
	}

	#[precompile::public("vestOther(address)")]
	pub fn vest_other(handle: &mut impl PrecompileHandle, account: Address) -> EvmResult<()> {
		register_cost::<Runtime>(
			handle,
			<Runtime as wrapper::pallet::Config>::WeightInfo::precompile_vest_other(),
		)?;

		let origin = <Runtime as frame_system::Config>::RuntimeOrigin::from(RawOrigin::from(Some(
			Runtime::AccountIdToH160::convert_back(handle.context().caller),
		)));
		let account_id = Runtime::AccountIdToH160::convert_back(account.into());
		let target =
			<<Runtime as frame_system::Config>::Lookup as StaticLookup>::unlookup(account_id);
		match PalletVesting::<Runtime>::vest_other(origin, target) {
			Ok(_) => Ok(()),
			Err(err) => Err(revert(convert_dispatch_error_to_string(err))),
		}
	}
}

fn convert_dispatch_error_to_string(err: DispatchError) -> String {
	match err {
		DispatchError::Module(mod_err) => mod_err.message.unwrap_or("Unknown module error").into(),
		_ => format!("{:?}", err),
	}
}

fn register_cost<Runtime: Config>(
	handle: &mut impl PrecompileHandle,
	weight: Weight,
) -> Result<(), ExitError> {
	let required_gas = Runtime::GasWeightMapping::weight_to_gas(weight);
	let remaining_gas = handle.remaining_gas();
	if required_gas > remaining_gas {
		return Err(ExitError::OutOfGas);
	}
	handle.record_cost(required_gas)?;
	handle.record_external_cost(Some(weight.ref_time()), Some(weight.proof_size()))?;
	Ok(())
}

mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
pub mod weights;
pub use weights::*;
mod wrapper;
pub use wrapper::*;
