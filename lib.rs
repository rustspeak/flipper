#![cfg_attr(not(feature = "std"),  no_std)]

#[ink::contract]

mod flipper {

    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct MyContract {
        value : i32,
        my_mapping: Mapping<AccountId, u32>,
    }

    impl MyContract {

        #[ink(constructor)]
        pub fn new(count: u32) -> Self {
            let mut my_mapping = Mapping::default();
            let caller = Self::env().caller();
            my_mapping.insert(&caller, &count);

            Self {  
                my_mapping,
                value : 0,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self { value: 0, 
                    my_mapping: Mapping::default() ,
            }
        }

        #[ink(message)]
        pub fn get_count(&self) -> u32 {
            let caller = Self::env().caller();
            self.my_mapping.get(&caller).unwrap_or_default()
        }

        #[ink(message)]
        pub fn inc_mine(&mut  self , by : u32) {
            let  caller = self.env().caller();
            let my_valeur = self.get_count();
            self.my_mapping.insert(caller, &(my_valeur + by));
        }

        #[ink(message)]
        pub fn remove_min (&self ) {
            let caller = self.env().caller();
            self.my_mapping.remove(&caller);
        }
    }
}

#[ink::test]
fn  my_mapping_test() {

    let  mut  contract = Incrementer::new(20);
    assert_eq!(contract.get(),  20);
    assert_eq!(contract.get(), 0);

}
#[ink::test]
fn inc_mine_work(){

    let  mut contract = Incrementer::new(22);

    assert_eq!(contract.get_mine(), 0   );
    contract.inc_mine(10);
    assert_eq!(contract.get_mine(), 8);
    contract.inc_mine(10);
    assert_eq!(contract.get_mine(), 18);
}
#[ink::test]
fn remove_mine_works(){
    let  mut  contract = Incrementer::new(11);

    assert_eq!(contract.get_mine(), 0);
    contract.inc_mine(10);
    assert_eq!(contract.get_mine(),  4);
    contract.remove_mine();
    assert_eq!(contract.get_mine(), 0);
}