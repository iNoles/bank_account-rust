use account::{Account, CheckingAccount, SavingsAccount};

mod account;

fn main() {
    let mut checking = CheckingAccount::new("Alice Johnson", 1000.0, 500.0);
    checking.display_account_info();

    checking.make_withdrawal(500.0);
    println!("After Withdrawl: $ {}", checking.get_balance());

    checking.make_deposit(100.0);
    println!("After Deposit: $ {}", checking.get_balance());

    checking.display_account_info();

    let mut savings = SavingsAccount::new("Bob Smith", 2000.0, 3.5);
    savings.display_account_info();

    savings.make_deposit(500.0);
    println!("After Deposit: $ {}", savings.get_balance());

    savings.add_interest();

    savings.display_account_info();
}
