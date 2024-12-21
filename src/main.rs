mod bank;

use bank::account::Account;
use chrono::TimeZone;
fn main() {
    let mut account = Account::new();
    account.deposit(100.0, chrono::Utc.with_ymd_and_hms(2024, 1, 1, 13, 36, 0).unwrap());
    account.deposit(50.0, chrono::Utc.with_ymd_and_hms(2024, 1, 2, 10, 2, 55).unwrap());
    account.withdraw(150.0, chrono::Utc.with_ymd_and_hms(2024, 1, 2, 18, 4, 2).unwrap());
    account.withdraw(10000.0, chrono::Utc.with_ymd_and_hms(2024, 1, 3, 9, 46, 38).unwrap());
    account.deposit(200.0, chrono::Utc.with_ymd_and_hms(2024, 1, 4, 10, 33, 1).unwrap());
    account.withdraw(100.0, chrono::Utc.with_ymd_and_hms(2024, 1, 22, 10, 58, 13).unwrap());

    println!("--------------------\nTotal Balance: {}\n--------------------\n", account.get_balance());

    Account::display_statement(&account);
}
