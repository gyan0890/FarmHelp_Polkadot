#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_storage::collections::{Vec, HashMap, Stash, Bitvec};
// use rand::Rng;

#[ink::contract]
mod farmhelp {

    enum LoanType {
        CropLoans,
        PostHarvest,
        LandPurchase
    }
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Farmhelp {
        /// Creating a structure for storing all thee loan related values
        loan_id: u32,
        photo_id: u32,
        address_proof: u32,
        landownership_id: u32,
        kissancredit_card: u32,
        loan_type: u32,
    }

    impl Farmhelp {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(photoid :u32, addressproof: u32, landownershipid: u32, kissancreditcard:u32, loantype: u32) -> Self {
            // let mut rng = rand::thread_rng();
            let num = 10;
            //let address = String::from(addressproof);
            // LoanType typeofloan = LoanType.CropLoans;
            // if(loantype == 0){
            //     typeofloan = LoanType.CropLoans;
            // }
            // elseif(loantype == 1){
            //     typeofloan = LoanType.PostHarvest;
            // }
            // elseif(loantype == 2){
            //     typeofloan = LoanType.LandPurchase;
            // }
            Self { 
                loan_id: num,
                photo_id: photoid,
                address_proof: addressproof,
                landownership_id: landownershipid,
                kissancredit_card: kissancreditcard,
                loan_type: loantype,
             }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        // #[ink(constructor)]
        // pub fn default() -> Self {
        //     Self::new(Default::default())
        // }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        // #[ink(message)]
        // pub fn flip(&mut self) {
        //     self.value = !self.value;
        // }

        /// Simply returns the current value of our loan ID.
        #[ink(message)]
        pub fn get(&self) -> u32 {
            self.loan_id
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[test]
        fn new_works() {
            let farmhelp = Farmhelp::new(12, "Hello", 21, 45, 0);
            assert_eq!(farmhelp.get(), 10);
        }

        
        // #[test]
        // fn it_works() {
        //     let mut farmhelp = Farmhelp::new(false);
        //     assert_eq!(farmhelp.get(), false);
        //     farmhelp.flip();
        //     assert_eq!(farmhelp.get(), true);
        // }
    }
}
