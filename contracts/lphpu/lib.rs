#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
        
#[openbrush::contract]
pub mod lphpu {
    // imports from ink!
	use ink_storage::traits::SpreadAllocate;
    
    // imports from openbrush
	use openbrush::contracts::psp22::extensions::burnable::*;
	use openbrush::contracts::psp22::extensions::mintable::*;
	use openbrush::traits::Storage;
    
    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Contract {
    	#[storage_field]
		psp22: psp22::Data,
    }
    
    // Section contains default implementation without any modifications
	impl PSP22 for Contract {}
	impl PSP22Burnable for Contract {}
	impl PSP22Mintable for Contract {}
    
    impl Contract {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Contract|{
				_instance._mint(_instance.env().caller(), initial_supply).expect("Should mint"); 
			})
        }
    }
}