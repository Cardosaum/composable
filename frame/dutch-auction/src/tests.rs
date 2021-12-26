use crate::runtime::*;
use composable_tests_helpers::test::currency::MockCurrencyId;
use composable_traits::{
	auction::{AuctionStepFunction, LinearDecrease},
	defi::{CurrencyPair, Sell, Take},
};
use frame_system::pallet_prelude::BlockNumberFor;
use orml_traits::MultiReservableCurrency;
use pallet_balances;

use frame_support::{
	assert_err, assert_noop, assert_ok,
	dispatch::DispatchErrorWithPostInfo,
	traits::{
		fungible::{self, Mutate as NativeMutate},
		fungibles::{Inspect, Mutate},
		Hooks,
	},
};

pub fn new_test_externalities() -> sp_io::TestExternalities {
	let mut storage = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
	let balances = vec![
		(AccountId::from(ALICE), 1_000_000_000_000_000_000_000_000),
		(AccountId::from(BOB), 1_000_000_000_000_000_000_000_000),
	];

	pallet_balances::GenesisConfig::<Runtime> { balances: balances.clone() }
		.assimilate_storage(&mut storage)
		.unwrap();

	// use crate::runtime::sp_api_hidden_includes_construct_runtime::hidden_include::traits::
	// GenesisBuild; orml_tokens::GenesisConfig::<Runtime> { balances:
	//     vec![
	//         (AccountId::from(ALICE), MockCurrencyId::PICA,  1_000_000_000_000_000_000_000_000),
	//         (AccountId::from(BOB),  MockCurrencyId::PICA, 1_000_000_000_000_000_000_000_000),
	//      ]
	//      }
	// .assimilate_storage(&mut storage)
	// .unwrap();

	let mut externatlities = sp_io::TestExternalities::new(storage);
	externatlities.execute_with(|| {
		System::set_block_number(42);
		Timestamp::set_timestamp(System::block_number() * MILLISECS_PER_BLOCK);
	});
	externatlities
}

#[test]
fn setup_sell() {
	new_test_externalities().execute_with(|| {
		Tokens::mint_into(MockCurrencyId::PICA, &ALICE, 1_000_000_000_000_000_000_000).unwrap();
		Balances::mint_into(&ALICE, NativeExistentialDeposit::get() * 3).unwrap();
		<Balances as NativeMutate<_>>::mint_into(&ALICE, NativeExistentialDeposit::get() * 3)
			.unwrap();
		Tokens::mint_into(MockCurrencyId::BTC, &ALICE, 100000000000).unwrap();
		let seller = AccountId::from_raw(ALICE.0);
		let sell = Sell::new(MockCurrencyId::BTC, MockCurrencyId::USDT, 1, 1000);
		let invalid = crate::OrdersIndex::<Runtime>::get();
		let configuration = AuctionStepFunction::LinearDecrease(LinearDecrease { total: 42 });
		let not_reserved = Assets::reserved_balance(MockCurrencyId::BTC, &ALICE);
		DutchAuction::ask(Origin::signed(seller), sell, configuration).unwrap();
		let reserved = Assets::reserved_balance(MockCurrencyId::BTC, &ALICE);
		assert!(not_reserved < reserved && reserved == 1);
		let order_id = crate::OrdersIndex::<Runtime>::get();
		assert_ne!(invalid, order_id);
		let initiative: u128 = Assets::reserved_balance(MockCurrencyId::PICA, &ALICE).into();
		let taken = <() as crate::weights::WeightInfo>::liquidate();
		assert!(initiative == taken.into());
	});
}

#[test]
fn with_immediate_exact_buy() {
	new_test_externalities().execute_with(|| {
		let a = 1_000_000_000_000_000_000_000;
		let b = 10;
		Tokens::mint_into(MockCurrencyId::USDT, &BOB, a).unwrap();
		Tokens::mint_into(MockCurrencyId::BTC, &ALICE, b).unwrap();
		let seller = AccountId::from_raw(ALICE.0);
		let buyer = AccountId::from_raw(BOB.0);
		let sell_amount = 1;
		let take_amount = 1000;
		let sell = Sell::new(MockCurrencyId::BTC, MockCurrencyId::USDT, sell_amount, take_amount);
		let configuration = AuctionStepFunction::LinearDecrease(LinearDecrease { total: 42 });
		DutchAuction::ask(Origin::signed(seller), sell, configuration).unwrap();
		let order_id = crate::OrdersIndex::<Runtime>::get();
		let result = DutchAuction::take(Origin::signed(buyer), order_id, Take::new(1, 999));
		assert!(!result.is_ok());
		let not_reserved = Assets::reserved_balance(MockCurrencyId::USDT, &BOB);
		let result = DutchAuction::take(Origin::signed(buyer), order_id, Take::new(1, 1000));
		let reserved = Assets::reserved_balance(MockCurrencyId::USDT, &BOB);
		assert!(not_reserved < reserved && reserved == take_amount);
		assert_ok!(result);
		let order = crate::SellOrders::<Runtime>::get(order_id).unwrap();
		DutchAuction::on_finalize(42);
		let not_found = crate::SellOrders::<Runtime>::get(order_id);
		assert!(not_found.is_none());
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 1000);
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &BOB), 1);
	});
}

#[test]
fn with_two_takes_higher_than_limit_and_not_enough_for_all() {
	new_test_externalities().execute_with(|| {
		let a = 1_000_000_000_000_000_000_000;
		let b = 1_000_000_000_000_000_000_000;
		Tokens::mint_into(MockCurrencyId::USDT, &BOB, a).unwrap();
		Tokens::mint_into(MockCurrencyId::BTC, &ALICE, b).unwrap();
		let seller = AccountId::from_raw(ALICE.0);
		let buyer = AccountId::from_raw(BOB.0);
		let sell_amount = 3;
		let take_amount = 3000;
		let configuration = AuctionStepFunction::LinearDecrease(LinearDecrease { total: 42 });

		let sell = Sell::new(MockCurrencyId::BTC, MockCurrencyId::USDT, sell_amount, take_amount);
		DutchAuction::ask(Origin::signed(seller), sell, configuration).unwrap();
		let order_id = crate::OrdersIndex::<Runtime>::get();
		DutchAuction::take(Origin::signed(buyer), order_id, Take::new(1, 1001));
		DutchAuction::take(Origin::signed(buyer), order_id, Take::new(1, 1002));

		DutchAuction::on_finalize(42);

		let order = crate::SellOrders::<Runtime>::get(order_id);
		assert!(order.is_some(), "not filled order exists");
	});
}

#[test]
fn liquidation() {
	new_test_externalities().execute_with(|| {
		Tokens::mint_into(MockCurrencyId::BTC, &ALICE, 10).unwrap();
		let seller = AccountId::from_raw(ALICE.0);
		let sell = Sell::new(MockCurrencyId::BTC, MockCurrencyId::USDT, 1, 1000);
		let invalid = crate::OrdersIndex::<Runtime>::get();
		let configuration = AuctionStepFunction::LinearDecrease(LinearDecrease { total: 42 });
		DutchAuction::ask(Origin::signed(seller), sell, configuration).unwrap();
		let order_id = crate::OrdersIndex::<Runtime>::get();
		let balance_before = <Balances as fungible::Inspect<_>>::balance(&ALICE);
		DutchAuction::liquidate(Origin::signed(seller), order_id).unwrap();
		let balance_after = <Balances as fungible::Inspect<_>>::balance(&ALICE);
		assert!(
			balance_before - <() as crate::weights::WeightInfo>::liquidate() as u128 ==
				balance_after
		);
		let not_found = crate::SellOrders::<Runtime>::get(order_id);
		assert!(not_found.is_none());
		let reserved = Assets::reserved_balance(MockCurrencyId::BTC, &ALICE);
		assert_eq!(reserved, 0);
	});
}
