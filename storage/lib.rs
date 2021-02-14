#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod storage {
    #[ink(storage)]
    pub struct Storage {
        value: u32,
    }

    impl Storage {
        #[ink(constructor)]
        pub fn new(init_value: u32) -> Self {
            Self { value: init_value }
        }

        #[ink(message)]
        pub fn set(&mut self, new_value: u32) {
            self.value = new_value;
        }

        #[ink(message)]
        pub fn get(&self) -> u32 {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_works() {
            let mut storage = Storage::new(42);
            assert_eq!(storage.get(), 42);
            storage.set(24);
            assert_eq!(storage.get(), 24);
        }
    }
}
