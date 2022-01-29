use fp_evm::{Context, ExitError, PrecompileOutput};
use pallet_evm::{AddressMapping, Precompile, PrecompileSet};
use sp_core::H160;
use sp_std::fmt::Debug;
use sp_std::marker::PhantomData;

use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_sha3fips::Sha3FIPS256;
use pallet_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};
use pallet_evm_precompile_dispatch::Dispatch;
use pallet_evm_precompile_blake2::Blake2F;
use pallet_evm_precompile_bn128::{Bn128Add, Bn128Mul, Bn128Pairing};

#[derive(Debug, Clone, Copy)]
pub struct UniqueOnePrecompiles<R>(PhantomData<R>);

impl<R> UniqueOnePrecompiles<R>
where
	R: pallet_evm::Config,
{

	/// Return all addresses that contain precompiles. This can be used to populate dummy code
	/// under the precompile.
	pub fn used_addresses() -> impl Iterator<Item = R::AccountId> {
		sp_std::vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1024, 1025, 1026, 2048, 2049, 2050, 2052, 2053, 2054]
		.into_iter()
		.map(|x| R::AddressMapping::into_account_id(hash(x)))
	}

}

impl<R> PrecompileSet for UniqueOnePrecompiles<R>
where
    Dispatch<R>: Precompile,
{
	fn execute(
		address: H160,
		input: &[u8],
		target_gas: Option<u64>,
		context: &Context,
	) -> Option<Result<PrecompileOutput, ExitError>> {
		match address {
			// Ethereum precompiles :
			a if a == hash(1) => Some(ECRecover::execute(input, target_gas, context)),
			a if a == hash(2) => Some(Sha256::execute(input, target_gas, context)),
			a if a == hash(3) => Some(Ripemd160::execute(input, target_gas, context)),
			a if a == hash(4) => Some(Identity::execute(input, target_gas, context)),
			a if a == hash(5) => Some(Modexp::execute(input, target_gas, context)),
            a if a == hash(6) => Some(Bn128Add::execute(input, target_gas, context)),
			a if a == hash(7) => Some(Bn128Mul::execute(input, target_gas, context)),
			a if a == hash(8) => Some(Bn128Pairing::execute(input, target_gas, context)),
			a if a == hash(9) => Some(Blake2F::execute(input, target_gas, context)),
			// Non-Frontier specific nor Ethereum precompiles :
			a if a == hash(1024) => Some(Sha3FIPS256::execute(input, target_gas, context)),
            a if a == hash(1025) => Some(Dispatch::<R>::execute(input, target_gas, context)),
			a if a == hash(1026) => Some(ECRecoverPublicKey::execute(input, target_gas, context)),
			_ => None,
		}
	}

}

fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}
