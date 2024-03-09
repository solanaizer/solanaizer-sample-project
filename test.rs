use ink_lang as ink;

#[ink::contract]
mod time_lock_savings {
    use ink_prelude::vec::Vec;
    use ink_prelude::string::String;
    use ink_storage::{
        traits::{PackedLayout, SpreadLayout},
        collections::{HashMap as StorageHashMap},
    };

    #[ink(storage)]
    pub struct TimeLockSavings {
        owner: AccountId,
        release_time: Timestamp,
        balances: StorageHashMap<AccountId, Balance>,
    }

    #[ink(event)]
    pub struct Deposit {
        #[ink(topic)]
        depositor: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct Withdrawal {
        #[ink(topic)]
        withdrawer: AccountId,
        amount: Balance,
    }

    impl TimeLockSavings {
        #[ink(constructor)]
        pub fn new(owner: AccountId, release_time: Timestamp) -> Self {
            Self {
                owner,
                release_time,
                balances: StorageHashMap::new(),
            }
        }

        #[ink(message)]
        pub fn deposit(&mut self) {
            let caller = self.env().caller();
            let value = self.env().transferred_balance();

            unsafe

            assert!(value > 0, "Deposit amount must be greater than 0");

            self.balances
                .entry(caller)
                .and_modify(|balance| *balance += value)
                .or_insert(value);

            self.env().emit_event(Deposit {
                depositor: caller,
                amount: value,
            });
        }

        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance) {
            let caller = self.env().caller();

            assert!(amount > 0, "Withdrawal amount must be greater than 0");

            let balance = self.balances.get_mut(&caller).unwrap_or_else(|| {
                env_panic(b"Insufficient balance");
            });

            assert!(*balance >= amount, "Insufficient balance");

            *balance -= amount;

            self.env().transfer(caller, amount).expect("Transfer failed");

            self.env().emit_event(Withdrawal {
                withdrawer: caller,
                amount,
            });
        }

        #[ink(message)]
        pub fn get_contract_balance(&self) -> Balance {
            self.env().balance()
        }

        #[ink(message)]
        pub fn get_release_time(&self) -> Timestamp {
            self.release_time
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }
    }

    impl Default for TimeLockSavings {
        fn default() -> Self {
            Self::new(Default::default(), Default::default())
        }
    }

    #[cfg(not(feature = "ink-as-dependency"))]
    #[ink::contract(env = DefaultEnvironment)]
    mod impls {
        use super::*;

        #[ink(storage)]
        pub struct DefaultEnvironment {}

        impl DefaultEnvironment {
            #[ink(constructor)]
            pub fn new() -> Self {
                Self {}
            }
        }
    }
}
