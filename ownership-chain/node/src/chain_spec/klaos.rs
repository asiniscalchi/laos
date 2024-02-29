use super::{get_collator_keys_from_seed, predefined_accounts, Extensions, SAFE_XCM_VERSION};
use cumulus_primitives_core::ParaId;
use fp_evm::GenesisAccount;
use klaos_ownership_runtime::{AccountId, AuraId, Precompiles, REVERT_BYTECODE};
use sc_service::ChainType;
use sp_core::{H160, U256};
use std::{collections::BTreeMap, str::FromStr};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<(), Extensions>;

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn template_session_keys(keys: AuraId) -> klaos_ownership_runtime::SessionKeys {
	klaos_ownership_runtime::SessionKeys { aura: keys }
}

pub fn development_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "UNIT".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::builder(
		klaos_ownership_runtime::WASM_BINARY.expect("WASM binary was not build, please build it!"),
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: 2001,
		},
	)
	.with_name("Development")
	.with_id("dev")
	.with_chain_type(ChainType::Development)
	.with_genesis_config_patch(testnet_genesis(
		// initial collators.
		vec![(predefined_accounts::ALITH.into(), get_collator_keys_from_seed("Alice"))],
		predefined_accounts::accounts(),
		// Give Alice root privileges
		Some(predefined_accounts::ALITH.into()),
		2001.into(),
	))
	.build()
}

pub fn local_testnet_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "UNIT".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::builder(
		klaos_ownership_runtime::WASM_BINARY.expect("WASM binary was not build, please build it!"),
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: 2001,
		},
	)
	.with_name("Local Testnet")
	.with_id("klaos_local_testnet")
	.with_chain_type(ChainType::Local)
	.with_protocol_id("template-local")
	.with_genesis_config_patch(testnet_genesis(
		// initial collators.
		vec![(predefined_accounts::ALITH.into(), get_collator_keys_from_seed("Alice"))],
		predefined_accounts::accounts(),
		// Give Alice root privileges
		Some(predefined_accounts::ALITH.into()),
		2001.into(),
	))
	.build()
}

fn testnet_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	root_key: Option<AccountId>,
	id: ParaId,
) -> serde_json::Value {
	let mut evm_accounts: BTreeMap<_, _> = Precompiles::used_addresses()
		.iter()
		.map(|&address| {
			(
				address,
				GenesisAccount {
					nonce: Default::default(),
					balance: Default::default(),
					storage: Default::default(),
					code: REVERT_BYTECODE.into(),
				},
			)
		})
		.collect();

	evm_accounts.insert(
		// H160 address of Alice dev account
		// Derived from SS58 (42 prefix) address
		// SS58: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
		// hex: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
		// Using the full hex key, truncating to the first 20 bytes (the first 40 hex
		// chars)
		H160::from_str("d43593c715fdd31c61141abd04a99fd6822c8558")
			.expect("internal H160 is valid; qed"),
		fp_evm::GenesisAccount {
			balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
				.expect("internal U256 is valid; qed"),
			code: Default::default(),
			nonce: Default::default(),
			storage: Default::default(),
		},
	);
	evm_accounts.insert(
		// H160 address of CI test runner account
		H160::from_str("6be02d1d3665660d22ff9624b7be0551ee1ac91b")
			.expect("internal H160 is valid; qed"),
		fp_evm::GenesisAccount {
			balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
				.expect("internal U256 is valid; qed"),
			code: Default::default(),
			nonce: Default::default(),
			storage: Default::default(),
		},
	);
	evm_accounts.insert(
		// H160 address for benchmark usage
		H160::from_str("1000000000000000000000000000000000000001")
			.expect("internal H160 is valid; qed"),
		fp_evm::GenesisAccount {
			nonce: U256::from(1),
			balance: U256::from(1_000_000_000_000_000_000_000_000u128),
			storage: Default::default(),
			code: vec![0x00],
		},
	);
	evm_accounts.insert(
		// H160 address of dev account
		// Private key :
		// 0xb9d2ea9a615f3165812e8d44de0d24da9bbd164b65c4f0573e1ce2c8dbd9c8df
		predefined_accounts::FAITH.into(),
		fp_evm::GenesisAccount {
			balance: U256::from_str("0xef000000000000000000000000000")
				.expect("internal U256 is valid; qed"),
			code: Default::default(),
			nonce: Default::default(),
			storage: Default::default(),
		},
	);

	serde_json::json!({
		"balances": {
			"balances": endowed_accounts.iter().cloned().map(|k| (k, 1e22 as u64)).collect::<Vec<_>>(),
		},
		"parachainInfo": {
			"parachainId": id,
		},
		"collatorSelection": {
			"invulnerables": invulnerables.iter().cloned().map(|(acc, _)| acc).collect::<Vec<_>>(),
			"candidacyBond": 0
		},
		"session": {
			"keys": invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                 // account id
						acc,                         // validator id
						template_session_keys(aura), // session keys
					)
				})
			.collect::<Vec<_>>(),
		},
		"polkadotXcm": {
			"safeXcmVersion": Some(SAFE_XCM_VERSION),
		},
		"sudo": { "key": root_key },
		// EVM compatibility
		"evmChainId": {
			"chainId": 667,
		},
		"evm": {
			"accounts": evm_accounts
		}
	})
}
