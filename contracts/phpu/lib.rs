#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
        
#[openbrush::contract]
pub mod phpu {
    // imports from ink!
	use ink_storage::traits::SpreadAllocate;
    use ink_prelude::string::String;
    use ink_prelude::string::ToString;
    use ink_prelude::vec::Vec;
    
    // imports from openbrush
	use openbrush::contracts::psp22::extensions::burnable::*;
	use openbrush::contracts::psp22::extensions::mintable::*;
    use openbrush::contracts::psp22::extensions::metadata::*;
	use openbrush::traits::Storage;
    
    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Contract {
    	#[storage_field]
		psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
    }
    
    // Section contains default implementation without any modifications
	impl PSP22 for Contract {}
	impl PSP22Burnable for Contract {}
	impl PSP22Mintable for Contract {}
    
    impl Contract {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Contract|{
                _instance.metadata.name = name;
                _instance.metadata.symbol = symbol;
                _instance.metadata.decimals = decimal;
				_instance._mint(_instance.env().caller(), initial_supply).expect("Should mint"); 
			})
        }

        #[ink(message)]
        pub fn mint_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            self.mint(account, amount)
        }

        #[ink(message)]
        pub fn burn_from_many(&mut self, accounts: Vec<(AccountId, Balance)>) -> Result<(), PSP22Error> {
            for account in accounts.iter() {
                self.burn(account.0, account.1)?;
            }
            Ok(())
        }

        #[ink(message)]
        pub fn name(&self) -> String {
            match &self.metadata.name {
                Some(name) => name.clone(),
                None => "NA".to_string(),
            }
        }

        #[ink(message)]
        pub fn symbol(&self) -> String {
            match &self.metadata.symbol {
                Some(symbol) => symbol.clone(),
                None => "NA".to_string(),
            }
        }

        #[ink(message)]
        pub fn decimal(&self) -> u8 {
            self.metadata.decimals.clone()
        }

    }
}