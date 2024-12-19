mod bank;


use bank::account::Account;
fn main() {
    let mut account = Account::new();
    account.deposit(100.0);
    account.withdraw(50.0);
    account.deposit(200.0);
    account.withdraw(100.0);
    
    println!("Balance: {}", account.get_balance());
    Account::display_statement(&account);



}
