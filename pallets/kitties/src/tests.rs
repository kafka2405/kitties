use crate::{mock::*, Error, Event, Gender};
use frame_support::{assert_noop, assert_ok};
use frame_support::sp_tracing::event;
use log::log;

#[test]
fn it_works_for_create_kitty() {
	new_test_ext().execute_with(|| {
		let sender = 1u64;
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(sender)));
	});
}

#[test]
fn it_works_for_set_price() {
	new_test_ext().execute_with(|| {
		let sender = 1u64;
		let kitty_id = SubstrateKitties::mint(&sender, None, None).unwrap();
		assert_ok!(SubstrateKitties::set_price(Origin::signed(sender), kitty_id, Some(10)));
	});
}

#[test]
fn it_works_for_transfer() {
	new_test_ext().execute_with(|| {
		let sender = 1u64;
		let receiver = 2u64;
		let kitty_id = SubstrateKitties::mint(&sender, None, None).unwrap();
		assert_ok!(SubstrateKitties::transfer(Origin::signed(sender), receiver, kitty_id));
	});
}

#[test]
fn it_works_for_buy_kitty() {
	new_test_ext().execute_with(|| {
		let owner = 1u64;
		let buyer = 2u64;
		let kitty_id = SubstrateKitties::mint(&owner, None, None).unwrap();
		SubstrateKitties::set_price(Origin::signed(owner), kitty_id, Some(10));
		assert_ok!(SubstrateKitties::buy_kitty(Origin::signed(buyer), kitty_id, 10));
	});
}

#[test]
fn it_works_for_breed_kitty() {
	new_test_ext().execute_with(|| {
		let owner = 1u64;
		let kitty_mother_id = SubstrateKitties::mint(&owner, None, Some(Gender::Female)).unwrap();
		System::set_block_number(2);
		let kitty_father_id = SubstrateKitties::mint(&owner, None, Some(Gender::Male)).unwrap();
		assert_ok!(SubstrateKitties::breed_kitty(Origin::signed(owner), kitty_mother_id, kitty_father_id));
	});
}
