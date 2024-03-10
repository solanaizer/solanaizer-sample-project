use std::sync::{Arc, Mutex};

struct Bank {
    balance: u64,
}

impl Bank {
    fn new(balance: u64) -> Self {
        Bank { balance }
    }

    fn transfer(&mut self, amount: u64) {
        // Simulate a transfer operation
        // In a real-world scenario, this would involve updating balances or interacting with external systems
        self.balance -= amount;
    }

    fn withdraw(&mut self, amount: u64) {
        // Simulate a withdrawal operation
        // In a real-world scenario, this would involve updating balances or interacting with external systems
        self.balance -= amount;
    }
}

fn main() {
    let bank = Arc::new(Mutex::new(Bank::new(100)));

    // Thread 1: Reentrant call
    let bank_clone = Arc::clone(&bank);
    let handle = std::thread::spawn(move || {
        let mut bank = bank_clone.lock().unwrap();
        println!("Thread 1: Current balance: {}", bank.balance);
        bank.withdraw(50);
        println!("Thread 1: After withdrawal, balance: {}", bank.balance);
        bank.transfer(25); // Potential reentrancy vulnerability
        println!("Thread 1: After transfer, balance: {}", bank.balance);
    });

    // Thread 2: Normal operation
    let mut bank = bank.lock().unwrap();
    println!("Thread 2: Current balance: {}", bank.balance);
    bank.transfer(10);
    println!("Thread 2: After transfer, balance: {}", bank.balance);

    handle.join().unwrap();
}
