#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod get_plasm_my_number {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::collections::HashMap as StorageHashMap;
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct GetPlasmMyNumber {
        /// Stores a single `bool` value on the storage.
        numbers: StorageHashMap<AccountId, u128>,
        present_number: u128,
 
    }

    impl GetPlasmMyNumber {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { numbers:StorageHashMap::new(),present_number:0 }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        /// get plasm my number
        #[ink(message)]
        pub fn get_plasm_my_number(&mut self) {
            let caller = self.env().caller();
            if self.check_plasm_my_number(caller) == 0 {
                self.present_number = self.present_number + 1;
                self.numbers.insert(caller, self.present_number);
            }
        }

        /// confirm my own number
        // #[ink(message)]
        // pub fn confirm_plasm_my_number(&self) -> u128 {
        //     let caller = self.env().caller();
        //     self.check_plasm_my_number(caller)
        // }

        /// get present number
        #[ink(message)]
        pub fn get_present_number(&self) -> u128 {
            self.present_number
        }

        /// check my plasm number. If you have not get number, you get zero value.
        pub fn check_plasm_my_number(&self, address: AccountId) -> u128 {
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
