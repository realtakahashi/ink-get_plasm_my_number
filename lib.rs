#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
mod get_plasm_my_number {
    use ink_core::storage;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    struct GetPlasmMyNumber {
        numbers: storage::HashMap<AccountId, u128>,
        present_number: storage::Value<u128>,
    }

    impl GetPlasmMyNumber {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        fn new(&mut self) {
            self.present_number.set(0);
        }

        /// get plasm my number
        #[ink(message)]
        fn get_plasm_my_number(&mut self) {
            let caller = self.env().caller();
            if self.check_plasm_my_number(caller) == 0 {
                self.present_number.set(self.present_number.get() + 1);
                self.numbers.insert(caller, *self.present_number.get());
            }
        }

        /// confirm my own number
        #[ink(message)]
        fn confirm_plasm_my_number(&self) -> u128 {
            let caller = self.env().caller();
            self.check_plasm_my_number(caller)
        }

        /// check my plasm number. If you have not get number, you get zero value.
        fn check_plasm_my_number(&self, address: AccountId) -> u128 {
            match self.numbers.get(&address) {
                Some(result) => *result,
                None => 0,
            }
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test a simple use case of our contract.
        #[test]
        fn it_works() {
            let mut get_plasm_my_number = GetPlasmMyNumber::new();
            get_plasm_my_number.get_plasm_my_number();
            assert_eq!(get_plasm_my_number.confirm_plasm_my_number(), 1);
        }
    }
}
