#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod ownable {
    #[ink(event)]
    pub struct OwnershipTransferred {
        #[ink(topic)]
        previous_owner: AccountId,
        #[ink(topic)]
        new_owner: AccountId,
    }

    #[ink(storage)]
    pub struct Ownable {
        owner: AccountId,
    }

    impl Ownable {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            Self { owner: caller }
        }

        #[ink(message)]
        pub fn owner(&self) -> AccountId {
            self.owner
        }

        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) {
            let caller = Self::env().caller();
            assert_eq!(caller, self.owner);

            self.env().emit_event(OwnershipTransferred {
                previous_owner: self.owner,
                new_owner,
            });

            self.owner = new_owner;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn it_works() {
            let mut ownable = Ownable::new();

            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            assert_eq!(ownable.owner(), accounts.alice);

            ownable.transfer_ownership(accounts.bob);
            assert_eq!(ownable.owner(), accounts.bob);
        }
    }
}
