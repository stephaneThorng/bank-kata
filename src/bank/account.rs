use super::transaction::{Operation, Transaction};
use crate::bank::transaction::Operation::Deposit;
use crate::bank::transaction::Operation::Withdrawal;

pub struct Account {
    pub transactions: Vec<Transaction>,
}

impl Account {
    pub fn new() -> Account {
        Self {
            transactions: Vec::new(),
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        self.transactions.push(Transaction::new(Deposit(amount)));
    }
    pub fn withdraw(&mut self, amount: f64) {
        self.transactions.push(Transaction::new(Withdrawal(amount)));
    }
    pub fn get_balance(&self) -> f64 {
        self.transactions
            .iter()
            .fold(0.0, |acc, transaction| match transaction.operation {
                Operation::Deposit(amount) => acc + amount,
                Operation::Withdrawal(amount) => acc - amount,
            })
    }

    pub fn display_statement(account: &Account) {
        println!("Date                | Operations");
        println!("--------------------+-----------");

        let mut transactions = account.transactions.clone();

        transactions.sort_by_key(|t| t.date);

        transactions.iter().rev().for_each(|t| {
            println!(
                "{} | {:?}",
                t.date.format("%Y-%m-%d %H:%M:%S"),
                t.operation
            );
        })
    }
}
