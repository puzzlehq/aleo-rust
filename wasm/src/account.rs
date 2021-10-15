// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Aleo library.

// The Aleo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo library. If not, see <https://www.gnu.org/licenses/>.

use snarkvm_wasm::{network::testnet2::Testnet2, Account as AleoAccount, AccountScheme, PrivateKey};

use rand::{rngs::StdRng, SeedableRng};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Account {
    pub(crate) account: AleoAccount<Testnet2>,
}

#[wasm_bindgen]
impl Account {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let rng = &mut StdRng::from_entropy();
        Self {
            account: AleoAccount::new(rng),
        }
    }

    #[wasm_bindgen]
    pub fn from_private_key(private_key: &str) -> Self {
        let private_key = PrivateKey::from_str(private_key).unwrap();

        Self {
            account: AleoAccount::from(private_key),
        }
    }

    #[wasm_bindgen]
    pub fn to_private_key(&self) -> String {
        self.account.private_key().to_string()
    }

    #[wasm_bindgen]
    pub fn to_view_key(&self) -> String {
        self.account.view_key().to_string()
    }

    #[wasm_bindgen]
    pub fn to_address(&self) -> String {
        self.account.address().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use wasm_bindgen_test::*;

    const ALEO_TESTNET2_PRIVATE_KEY: &str = "APrivateKey1zkp8cC4jgHEBnbtu3xxs1Ndja2EMizcvTRDq5Nikdkukg1p";
    const ALEO_TESTNET2_VIEW_KEY: &str = "AViewKey1iAf6a7fv6ELA4ECwAth1hDNUJJNNoWNThmREjpybqder";
    const ALEO_TESTNET2_ADDRESS: &str = "aleo1d5hg2z3ma00382pngntdp68e74zv54jdxy249qhaujhks9c72yrsydapc4";

    #[wasm_bindgen_test]
    pub fn from_private_key_test() {
        let account = Account::from_private_key(ALEO_TESTNET2_PRIVATE_KEY);

        println!(
            "{} == {}",
            ALEO_TESTNET2_PRIVATE_KEY,
            account.account.private_key().to_string()
        );
        assert_eq!(ALEO_TESTNET2_PRIVATE_KEY, account.account.private_key().to_string());

        println!(
            "{} == {}",
            ALEO_TESTNET2_VIEW_KEY,
            account.account.view_key().to_string()
        );
        assert_eq!(ALEO_TESTNET2_VIEW_KEY, account.account.view_key().to_string());

        println!("{} == {}", ALEO_TESTNET2_ADDRESS, account.account.address().to_string());
        assert_eq!(ALEO_TESTNET2_ADDRESS, account.account.address().to_string());
    }

    #[wasm_bindgen_test]
    pub fn to_private_key_test() {
        let account = Account::from_private_key(ALEO_TESTNET2_PRIVATE_KEY);

        println!("{} == {}", ALEO_TESTNET2_PRIVATE_KEY, account.to_private_key());
        assert_eq!(ALEO_TESTNET2_PRIVATE_KEY, account.to_private_key());
    }

    #[wasm_bindgen_test]
    pub fn to_view_key_test() {
        let account = Account::from_private_key(ALEO_TESTNET2_PRIVATE_KEY);

        println!("{} == {}", ALEO_TESTNET2_VIEW_KEY, account.to_view_key());
        assert_eq!(ALEO_TESTNET2_VIEW_KEY, account.to_view_key());
    }

    #[wasm_bindgen_test]
    pub fn to_address_test() {
        let account = Account::from_private_key(ALEO_TESTNET2_PRIVATE_KEY);

        println!("{} == {}", ALEO_TESTNET2_ADDRESS, account.to_address());
        assert_eq!(ALEO_TESTNET2_ADDRESS, account.to_address());
    }
}
