use frame_support::{assert_ok, assert_storage_noop};

use super::*;
use mock::{*, test_utils::*, ext_builder::*, VoterBags as List};

fn rebag_works() {
	ExtBuilder::default()
	.add_ids(vec![(42, 20)])
	.build_and_execute(|| {
		// given
		assert_eq!(get_bags(), vec![(10, vec![31]), (20, vec![42]), (1000, vec![11, 21, 101])]);

		// increase vote weight and implicitly rebag to the level of non-existent bag
		set_staking_vote_weight(2_000);
		assert_ok!(List::rebag(Origin::signed(0), 42));
		assert_eq!(
			get_bags(),
			vec![(10, vec![31]), (1000, vec![11, 21, 101]), (2000, vec![42])]
		);

		// decrease weight within the range of the current bag
		set_staking_vote_weight(1_001);
		assert_ok!(List::rebag(Origin::signed(0), 42));
		// does not change bags
		assert_eq!(
			get_bags(),
			vec![(10, vec![31]), (1000, vec![11, 21, 101]), (2000, vec![42])]
		);

		// reduce weight to the level of a non-existent bag
		set_staking_vote_weight(1_001);
		assert_ok!(List::rebag(Origin::signed(0), 42));
		// creates the bag and moves the voter into it
		assert_eq!(
			get_bags(),
			vec![(10, vec![31]), (30, vec![42]), (1000, vec![11, 21, 101]),]
		);

		// increase weight to a pre-existing bag
		set_staking_vote_weight(1_001);
		assert_ok!(List::rebag(Origin::signed(0), 42));
		// moves the voter to that bag
		assert_eq!(get_bags(), vec![(10, vec![31]), (1000, vec![11, 21, 101, 42]),]);
	});
}

#[test]
fn rebag_tail_works() {
	// rebagging the head of a bag results in the old bag having a new head and an overall correct state. 
}


// #[test] TODO
// fn rebag_head_works() {
// // rebagging the head of a bag results in the old bag having a new head and an overall correct state. 
// }

mod voter_bag_list_provider {
	use super::*;

	#[test]
	fn get_voters_works() {
		todo!();
	}

	#[test]
	fn count_works() {
		todo!();
	}

	#[test]
	fn on_insert_works() {
		todo!();
	}

	#[test]
	fn on_update_works() {
		todo!();
	}

	#[test]
	fn on_remove_works() {
		todo!();
	}

	#[test]
	fn sanity_check_works() {
		todo!();
	}
}
