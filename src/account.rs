pub trait Account {
    fn display_account_info(&self);
    fn make_deposit(&mut self, amount: f64);
    fn make_withdrawal(&mut self, amount: f64);
    fn get_balance(&self) -> f64;
}

struct BasicAccount {
    account_holder_name: String,
    balance: f64
}

impl BasicAccount {
    fn new(holder_name: &str, inital_balance: f64) -> Self {
        BasicAccount {
            account_holder_name: holder_name.to_string(),
            balance: inital_balance
        }
    }
}

impl Account for BasicAccount {
    fn display_account_info(&self) {
        println!("Account Holder: {}", self.account_holder_name);
        println!("Balance: {}", self.balance);
    }

    fn make_deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn make_withdrawal(&mut self, amount: f64) {
        if self.balance >= amount {
            self.balance -= amount;
        } else {
            println!("Insufficient funds");
        }
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }
}

pub struct CheckingAccount {
    overdraft_limit: f64,
    base_account: BasicAccount
}

impl CheckingAccount {
   pub fn new(holder_name: &str, inital_balance: f64, overdraft: f64) -> Self {
        CheckingAccount {
            overdraft_limit: overdraft,
            base_account: BasicAccount::new(holder_name, inital_balance)
        }
    }
}

impl Account for CheckingAccount {
    fn display_account_info(&self) {
        println!("Checking Account Information");
        self.base_account.display_account_info();
        println!("Overdraft Limit: {}", self.overdraft_limit);
    }

    fn make_deposit(&mut self, amount: f64) {
        self.base_account.make_deposit(amount);
    }

    fn make_withdrawal(&mut self, amount: f64) {
        if self.base_account.balance + self.overdraft_limit >= amount {
            self.base_account.make_withdrawal(amount);
        } else {
            println!("Exceeds overdraft limit");
        }
    }

    fn get_balance(&self) -> f64 {
        self.base_account.get_balance()
    }
}

pub struct SavingsAccount {
    interest_rate: f64,
    base_account: BasicAccount
}

impl SavingsAccount {
    pub fn new(holder_name: &str, inital_balance: f64, interest: f64) -> Self {
        SavingsAccount {
            interest_rate: interest,
            base_account: BasicAccount::new(holder_name, inital_balance)
        }
    }

    pub fn add_interest(&mut self) {
        let interest_amount = self.base_account.balance * (self.interest_rate / 100.0);
        self.base_account.balance += interest_amount;
    }
}

impl Account for SavingsAccount {
    fn display_account_info(&self) {
        println!("Savings Account Information");
        self.base_account.display_account_info();
        println!("Interest Rate: {}%", self.interest_rate);
    }

    fn make_deposit(&mut self, amount: f64) {
        self.base_account.balance += amount;
    }

    fn make_withdrawal(&mut self, amount: f64) {
        if self.base_account.balance >= amount {
            self.base_account.balance -= amount;
        } else {
            println!("Insufficient funds");
        }
    }

    fn get_balance(&self) -> f64 {
        self.base_account.balance
    }
}
