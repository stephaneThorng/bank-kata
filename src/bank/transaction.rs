use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Transaction {
    pub date: DateTime<Utc>,
    pub operation: Operation,
}

impl Transaction {
    pub fn new(operation: Operation, date: DateTime<Utc>) -> Transaction {
        Self {
            operation,
            date,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Operation {
    Deposit(f64),
    Withdrawal(f64),
}