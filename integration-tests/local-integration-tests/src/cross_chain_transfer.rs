use crate::{env_logger_init, kusama_test_net::*};
use codec::Encode;
use common::AccountId;
use composable_traits::assets::{RemoteAssetRegistry, XcmAssetLocation};
use dali_runtime as picasso_runtime;
use orml_traits::currency::MultiCurrency;
use picasso_runtime::{MaxInstructions, UnitWeightCost};
use primitives::currency::*;
use sp_runtime::assert_eq_error_rate;
use support::assert_ok;
use xcm::latest::prelude::*;
use xcm_emulator::TestExt;
use xcm_executor::XcmExecutor;

/// assumes that our parachain has native relay token on relay account
/// and kusama can send xcm message to our network and transfer native token onto local network
#[test]
fn transfer_from_relay_chain() {
	crate::kusama_test_net::KusamaNetwork::reset();
	env_logger_init();
	let bob_before = Picasso::execute_with(|| {
		assert_ok!(picasso_runtime::AssetsRegistry::set_location(
			CurrencyId::KSM, // KSM id as it is locally
			// if we get tokens from parent chain, these can be only native token
			XcmAssetLocation::RELAY_NATIVE,
		));
		picasso_runtime::Assets::free_balance(CurrencyId::KSM, &AccountId::from(BOB))
	});

	let transfer_amount = 3 * KSM;
	KusamaRelay::execute_with(|| {
		let alice_before = kusama_runtime::Balances::free_balance(&AccountId::from(ALICE));
		let transfered = kusama_runtime::XcmPallet::reserve_transfer_assets(
			kusama_runtime::Origin::signed(ALICE.into()),
			Box::new(Parachain(PICASSO_PARA_ID).into().into()),
			Box::new(
				Junction::AccountId32 { id: crate::kusama_test_net::BOB, network: NetworkId::Any }
					.into()
					.into(),
			),
			Box::new((Here, transfer_amount).into()),
			0,
		);
		assert_ok!(transfered);
		let alice_after = kusama_runtime::Balances::free_balance(&AccountId::from(ALICE));
		assert_eq!(alice_before, alice_after + transfer_amount);
	});

	Picasso::execute_with(|| {
		let bob_after =
			picasso_runtime::Assets::free_balance(CurrencyId::KSM, &AccountId::from(BOB));
		assert_eq_error_rate!(
			bob_after - bob_before,
			transfer_amount,
			(UnitWeightCost::get() * 10) as u128
		);
	});
}

#[test]
fn transfer_to_relay_chain() {
	crate::kusama_test_net::KusamaNetwork::reset();
	env_logger_init();
	Picasso::execute_with(|| {
		assert_ok!(<picasso_runtime::AssetsRegistry as RemoteAssetRegistry>::set_location(
			CurrencyId::KSM,
			XcmAssetLocation::RELAY_NATIVE,
		));
		let transferred = picasso_runtime::XTokens::transfer(
			picasso_runtime::Origin::signed(ALICE.into()),
			CurrencyId::KSM,
			3 * PICA,
			Box::new(
				MultiLocation::new(
					1,
					X1(Junction::AccountId32 { id: BOB, network: NetworkId::Any }),
				)
				.into(),
			),
			4_600_000_000,
		);

		assert_ok!(transferred);

		let remaining =
			picasso_runtime::Assets::free_balance(CurrencyId::KSM, &AccountId::from(ALICE));

		assert_eq!(remaining, ALICE_PARACHAIN_KSM - 3 * PICA);
	});

	KusamaRelay::execute_with(|| {
		assert_eq!(
			kusama_runtime::Balances::free_balance(&AccountId::from(BOB)),
			2999893333340 // 3 * PICA - fee
		);
	});
}

#[test]
fn transfer_from_dali() {
	crate::kusama_test_net::KusamaNetwork::reset();
	env_logger_init();

	Picasso::execute_with(|| {
		assert_ok!(<picasso_runtime::AssetsRegistry as RemoteAssetRegistry>::set_location(
			CurrencyId::PICA,
			composable_traits::assets::XcmAssetLocation(MultiLocation::new(
				1,
				X2(Parachain(DALI_PARA_ID), GeneralKey(CurrencyId::PICA.encode()))
			))
		));
	});

	let local_withdraw_amount = 3 * PICA;
	Dali::execute_with(|| {
		assert_ok!(dali_runtime::XTokens::transfer(
			dali_runtime::Origin::signed(ALICE.into()),
			CurrencyId::PICA,
			local_withdraw_amount,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Junction::Parachain(PICASSO_PARA_ID),
						Junction::AccountId32 { id: BOB, network: NetworkId::Any }
					)
				)
				.into()
			),
			399_600_000_000
		));
		assert_eq!(
			dali_runtime::Assets::free_balance(CurrencyId::PICA, &AccountId::from(ALICE)),
			200 * PICA - local_withdraw_amount
		);
	});

	Picasso::execute_with(|| {
		let balance =
			picasso_runtime::Assets::free_balance(CurrencyId::PICA, &AccountId::from(BOB));
		assert_eq_error_rate!(balance, local_withdraw_amount, (UnitWeightCost::get() * 10) as u128);
	});
}

#[test]
fn transfer_from_picasso_to_dali() {
	crate::kusama_test_net::KusamaNetwork::reset();
	env_logger_init();

	Dali::execute_with(|| {
		assert_ok!(<dali_runtime::AssetsRegistry as RemoteAssetRegistry>::set_location(
			// local id
			CurrencyId::PICA,
			// remote id
			// first part is remote network,
			// second part is id of asset on remote
			composable_traits::assets::XcmAssetLocation(MultiLocation::new(
				1,
				X2(Parachain(PICASSO_PARA_ID), GeneralKey(CurrencyId::PICA.encode()))
			))
		));
	});

	Picasso::execute_with(|| {
		assert_ok!(<picasso_runtime::AssetsRegistry as RemoteAssetRegistry>::set_location(
			CurrencyId::PICA,
			composable_traits::assets::XcmAssetLocation(MultiLocation::new(
				1,
				X2(Parachain(DALI_PARA_ID), GeneralKey(CurrencyId::PICA.encode()))
			))
		));

		assert_ok!(picasso_runtime::XTokens::transfer(
			picasso_runtime::Origin::signed(ALICE.into()),
			CurrencyId::PICA,
			3 * PICA,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Junction::Parachain(DALI_PARA_ID),
						Junction::AccountId32 { id: BOB, network: NetworkId::Any }
					)
				)
				.into()
			),
			399_600_000_000
		));
		assert_eq!(
			picasso_runtime::Balances::free_balance(&AccountId::from(ALICE)),
			200 * PICA - 3 * PICA
		);
	});

	Dali::execute_with(|| {
		let balance = dali_runtime::Assets::free_balance(CurrencyId::PICA, &AccountId::from(BOB));
		assert_eq_error_rate!(balance, 3 * PICA, (UnitWeightCost::get() * 10) as u128);
	});
}

// from: Hydra
#[test]
fn transfer_insufficient_amount_should_fail() {
	crate::kusama_test_net::KusamaNetwork::reset();
	env_logger_init();
	Dali::execute_with(|| {
		assert_ok!(dali_runtime::XTokens::transfer(
			dali_runtime::Origin::signed(ALICE.into()),
			CurrencyId::PICA,
			1_000_000 - 1,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Junction::Parachain(PICASSO_PARA_ID),
						Junction::AccountId32 { id: BOB, network: NetworkId::Any }
					)
				)
				.into()
			),
			399_600_000_000
		));
		assert_eq!(dali_runtime::Balances::free_balance(&AccountId::from(ALICE)), 199999999000001);
	});

	Picasso::execute_with(|| {
		// Xcm should fail therefore nothing should be deposit into beneficiary account
		assert_eq!(
			picasso_runtime::Tokens::free_balance(CurrencyId::PICA, &AccountId::from(BOB)),
			0
		);
	});
}

/// Acala's tests
#[test]
#[ignore]
fn transfer_from_relay_chain_deposit_to_treasury_if_below_existential_deposit() {
	crate::kusama_test_net::KusamaNetwork::reset();
	env_logger_init();

	KusamaRelay::execute_with(|| {
		assert_ok!(kusama_runtime::XcmPallet::reserve_transfer_assets(
			kusama_runtime::Origin::signed(ALICE.into()),
			Box::new(Parachain(PICASSO_PARA_ID).into().into()),
			Box::new(Junction::AccountId32 { id: BOB, network: NetworkId::Any }.into().into()),
			Box::new((Here, 128_000_111 / 50).into()),
			0
		));
	});

	Picasso::execute_with(|| {
		assert_eq!(
			picasso_runtime::Tokens::free_balance(CurrencyId::KSM, &AccountId::from(BOB)),
			0
		);
		// TODO: add treasury like in Acala to get available payment even if it lower than needed to
		// treasury (add treasury call) assert_eq!(
		// 	picasso_runtime::Tokens::free_balance(
		// 		CurrencyId::KSM,
		// 		&picasso_runtime::TreasuryAccount::get()
		// 	),
		// 	1_000_128_000_111
		// );
	});
}

/// from: Acala
/// this test resonably iff we know ratio of KSM to PICA, if not, it should be rewritten to ensure
/// permissioned execution of some very specific action from other chains
#[test]
fn xcm_transfer_execution_barrier_trader_works() {
	crate::kusama_test_net::KusamaNetwork::reset();
	env_logger_init();

	let unit_instruction_weight = UnitWeightCost::get() / 50;
	assert!(unit_instruction_weight > 0, "barrier makes sence iff there is pay for messages");

	// relay-chain use normal account to send xcm, destination para-chain can't pass Barrier check
	let tiny = 100;
	let message = Xcm(vec![
		ReserveAssetDeposited((Parent, tiny).into()),
		BuyExecution { fees: (Parent, tiny).into(), weight_limit: Unlimited },
		DepositAsset { assets: All.into(), max_assets: 1, beneficiary: Here.into() },
	]);
	KusamaRelay::execute_with(|| {
		let r = pallet_xcm::Pallet::<kusama_runtime::Runtime>::send(
			kusama_runtime::Origin::signed(ALICE.into()),
			Box::new(Parachain(PICASSO_PARA_ID).into().into()),
			Box::new(xcm::VersionedXcm::from(message)),
		);
		assert_ok!(r);
	});
	Picasso::execute_with(|| {
		assert!(picasso_runtime::System::events().iter().any(|r| {
			matches!(
				r.event,
				picasso_runtime::Event::DmpQueue(
					cumulus_pallet_dmp_queue::Event::ExecutedDownward(
						_,
						Outcome::Error(XcmError::Barrier)
					)
				)
			)
		}));
	});

	// AllowTopLevelPaidExecutionFrom barrier test case:
	// para-chain use XcmExecutor `execute_xcm()` method to execute xcm.
	// if `weight_limit` in BuyExecution is less than `xcm_weight(max_weight)`, then Barrier can't
	// pass. other situation when `weight_limit` is `Unlimited` or large than `xcm_weight`, then
	// it's ok.
	let expect_weight_limit = UnitWeightCost::get() * (MaxInstructions::get() as u64) * 100;
	let message = Xcm::<picasso_runtime::Call>(vec![
		ReserveAssetDeposited((Parent, tiny).into()),
		BuyExecution { fees: (Parent, tiny).into(), weight_limit: Limited(100) },
		DepositAsset { assets: All.into(), max_assets: 1, beneficiary: Here.into() },
	]);
	Picasso::execute_with(|| {
		let r = XcmExecutor::<picasso_runtime::XcmConfig>::execute_xcm(
			Parent,
			message,
			expect_weight_limit,
		);
		assert_eq!(r, Outcome::Error(XcmError::Barrier));
	});

	// trader inside BuyExecution have TooExpensive error if payment less than calculated weight
	// amount. the minimum of calculated weight amount(`FixedRateOfFungible<KsmPerSecond>`) is
	let ksm_per_second = UnitWeightCost::get() as u128 / 50 - 1_000; // TODO: define all calculation somehow in runtime as in Acala
	let message = Xcm::<picasso_runtime::Call>(vec![
		ReserveAssetDeposited((Parent, ksm_per_second).into()),
		BuyExecution {
			fees: (Parent, ksm_per_second).into(),
			weight_limit: Limited(expect_weight_limit),
		},
		DepositAsset { assets: All.into(), max_assets: 1, beneficiary: Here.into() },
	]);
	Picasso::execute_with(|| {
		let r = XcmExecutor::<picasso_runtime::XcmConfig>::execute_xcm(
			Parent,
			message,
			expect_weight_limit,
		);
		assert_eq!(
			r,
			Outcome::Incomplete(
				unit_instruction_weight * 2 * 50, /* so here we have report in PICA, while we
				                                   * allowed to pay in KSM */
				XcmError::TooExpensive
			)
		);
	});

	// all situation fulfilled, execute success
	let total = (unit_instruction_weight * MaxInstructions::get() as u64) as u128;
	let message = Xcm::<picasso_runtime::Call>(vec![
		ReserveAssetDeposited((Parent, total).into()),
		BuyExecution { fees: (Parent, total).into(), weight_limit: Limited(expect_weight_limit) },
		DepositAsset { assets: All.into(), max_assets: 1, beneficiary: Here.into() },
	]);
	Picasso::execute_with(|| {
		let r = XcmExecutor::<picasso_runtime::XcmConfig>::execute_xcm(
			Parent,
			message,
			expect_weight_limit,
		);
		assert_eq!(r, Outcome::Complete(unit_instruction_weight * 3 * 50));
	});
}

/// source: Acala
#[test]
fn para_chain_subscribe_version_notify_of_sibling_chain() {
	crate::kusama_test_net::KusamaNetwork::reset();
	env_logger_init();
	Picasso::execute_with(|| {
		let r = pallet_xcm::Pallet::<picasso_runtime::Runtime>::force_subscribe_version_notify(
			picasso_runtime::Origin::root(),
			Box::new((Parent, Parachain(DALI_PARA_ID)).into()),
		);
		assert_ok!(r);
	});
	Picasso::execute_with(|| {
		assert!(picasso_runtime::System::events().iter().any(|r| matches!(
			r.event,
			picasso_runtime::Event::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent(
				Some(_)
			))
		)));
	});
	Dali::execute_with(|| {
		assert!(dali_runtime::System::events().iter().any(|r| matches!(
			r.event,
			picasso_runtime::Event::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent(
				Some(_)
			)) | picasso_runtime::Event::XcmpQueue(cumulus_pallet_xcmp_queue::Event::Success(
				Some(_)
			))
		)));
	});
}

/// source: Acala
#[test]
fn para_chain_subscribe_version_notify_of_relay_chain() {
	Picasso::execute_with(|| {
		let r = pallet_xcm::Pallet::<picasso_runtime::Runtime>::force_subscribe_version_notify(
			picasso_runtime::Origin::root(),
			Box::new(Parent.into()),
		);
		assert_ok!(r);
	});
	Picasso::execute_with(|| {
		picasso_runtime::System::assert_has_event(picasso_runtime::Event::RelayerXcm(
			pallet_xcm::Event::SupportedVersionChanged(
				MultiLocation { parents: 1, interior: Here },
				2,
			),
		));
	});
}

/// source: Acala
#[test]
fn relay_chain_subscribe_version_notify_of_para_chain() {
	KusamaRelay::execute_with(|| {
		let r = pallet_xcm::Pallet::<kusama_runtime::Runtime>::force_subscribe_version_notify(
			kusama_runtime::Origin::root(),
			Box::new(Parachain(PICASSO_PARA_ID).into().into()),
		);
		assert_ok!(r);
	});
	KusamaRelay::execute_with(|| {
		kusama_runtime::System::assert_has_event(kusama_runtime::Event::XcmPallet(
			pallet_xcm::Event::SupportedVersionChanged(
				MultiLocation { parents: 0, interior: X1(Parachain(PICASSO_PARA_ID)) },
				2,
			),
		));
	});
}
