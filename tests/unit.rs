//  Copyright 2021 PolyCrypt GmbH
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

mod common;
use common::mock::*;
use common::utils::*;

use frame_support::assert_noop;
use pallet_perun::types::{BalanceOf, PkOf};
use pallet_perun::Error;
use sp_runtime::traits::BadOrigin;

#[test]
/// All functions need signed origins.
fn unsigned_tx() {
	run_test(|_| {
		assert_noop!(
			Perun::deposit(Origin::none(), Default::default(), Default::default()),
			BadOrigin
		);
	});
}
