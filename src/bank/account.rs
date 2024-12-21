use chrono::Utc;

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


    pub fn deposit(&mut self, amount: f64, date: chrono::DateTime<Utc>) {
        self.transactions
            .push(Transaction::new(Deposit(amount), date));
        println!("Deposited : {}", amount);
    }


    pub fn withdraw(&mut self, amount: f64, date: chrono::DateTime<Utc>) {
        let balance = self.get_balance();
        if balance < amount {
            println!("Withdrawn : {} ... Insufficient funds !", amount);
            return ();
        }

        self.transactions
            .push(Transaction::new(Withdrawal(amount), date));
        println!("Withdrawn : {}", amount);
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
        println!("Date                 | Operations           |         Balance");
        println!("---------------------+----------------------+----------------");

        let mut transactions = account.transactions.clone();
        let mut balance = 0.0; 

        transactions.sort_by_key(|t| t.date);
        transactions.iter().for_each(|t| {

            balance += match t.operation {
                Operation::Deposit(amount) => amount,
                Operation::Withdrawal(amount) => -amount,
            };
            
            let date_str = format!("{}", t.date.format("%Y-%m-%d %H:%M:%S"));
            let operation_str = format!("{:?}", t.operation);
            let balance_str = format!("{:.2}", balance);
            println!("{:<20} | {:<20} | {:>15}", date_str, operation_str, balance_str);
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_deposit() {
        let mut account = Account::new();
        account.deposit(100.0, Utc::now());
        assert_eq!(account.get_balance(), 100.0);
    }



    #[test]
    fn test_withdraw() {
        let mut account = Account::new();
        account.deposit(100.0, Utc::now());
        account.withdraw(50.0, Utc::now());
        assert_eq!(account.get_balance(), 50.0);

        // Withdrawal with insufficient funds
        account.withdraw(100.0, Utc::now());
        assert_eq!(account.get_balance(), 50.0);
    }
}
