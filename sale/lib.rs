#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(clippy::new_without_default)]

use erc1155::{ContractRef, Erc1155, Error, TokenId};
use ink::prelude::vec::Vec;
use ink_env::call::FromAccountId;

#[ink::contract]
mod sale {

    use super::*;

    #[ink(storage)]
    pub struct Sale {
        erc1155_contract: ContractRef,
        price: Balance, // A0 price
        erc1155_address: AccountId,
    }

    impl Sale {
        #[ink(constructor)]
        pub fn new(erc1155_contract: AccountId, price: Balance) -> Self {
            Self {
                erc1155_contract: ContractRef::from_account_id(erc1155_contract),
                price,
                erc1155_address: erc1155_contract,
            }
        }

        #[ink(message)]
        #[ink(payable)]
        pub fn sale(
            &self,
            owner: AccountId,
            token_id: TokenId,
            value: Balance,
            data: Vec<u8>,
        ) -> Result<(), Error> {
            let buyer = self.env().caller();
            let owner_token_balance = Erc1155::balance_of(&self.erc1155_contract, owner, token_id);
            let fee = self
                .price
                .checked_mul(value)
                .unwrap()
                .checked_div(10_u128.pow(12_u32))
                .unwrap();
            if owner_token_balance < value || self.env().transferred_value() < fee {
                return Err(Error::NotEnoughBalance);
            }
            let mut erc1155_call: ContractRef =
                FromAccountId::from_account_id(self.erc1155_address);
            let is_approved = Erc1155::is_approved_for_all(
                &self.erc1155_contract,
                owner,
                self.env().account_id(),
            );

            if is_approved {
                let tranfers_result = Erc1155::safe_transfer_from(
                    &mut erc1155_call,
                    owner,
                    buyer,
                    token_id,
                    value,
                    data,
                );

                if tranfers_result.is_err() {
                    return Err(Error::CannotTransfer);
                }
            } else {
                return Err(Error::NotApproved);
            }

            Ok(())
        }

        #[ink(message)]
        pub fn set_price(&mut self, price: Balance) -> Result<(), Error> {
            Ok(self.price = price)
        }

        #[ink(message)]
        pub fn get_erc115_address(&self) -> AccountId {
            self.erc1155_address
        }

        #[ink(message)]
        pub fn get_price(&self) -> Balance {
            self.price
        }
    }
}
