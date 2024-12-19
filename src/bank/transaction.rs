use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Transaction {
    pub date: DateTime<Utc>,
    pub operation: Operation,
}

impl Transaction {
    pub fn new(operation: Operation) -> Transaction {
        Self {
            date: Utc::now(),
            operation,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Operation {
    Deposit(f64),
    Withdrawal(f64),
}