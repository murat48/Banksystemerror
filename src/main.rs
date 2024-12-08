fn main() {
    let mut account1 = BankAccount {
        account_number: 10,
        holder_name: String::from("Patika"),
        balance: 500.00,
    };

    let mut account2 = BankAccount {
        account_number: 11,
        holder_name: String::from("Risein"),
        balance: 300.0,
    };

    let _ = account1.deposit(200.0);

    let _ = account2.withdraw(300.0);

    println!(
        "Account {} ({}) balance: ${:.2}",
        account1.account_number,
        account1.holder_name,
        account1.balance()
    );
    println!(
        "Account {} ({}) balance: ${:.2}",
        account2.account_number,
        account2.holder_name,
        account2.balance()
    );
}
trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}
struct BankAccount {
    account_number: u64,
    holder_name: String,
    balance: f64,
}
impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount > 0.0 && self.balance > 0.0 {
            self.balance += amount;
            println!(
                "Deposited ${:.2} into account {}. New balance: ${:.2}",
                amount, self.account_number, self.balance
            );
            return Ok(());
        } else {
            // println!("Deposit amount must be greater than zero.");
            // let var_name = Err("Deposit amount must be greater than zero.".to_string());
            // var_name
            println!("Withdraw amount must be greater than zero.");
            Err("Withdraw amount must be greater than zero.".to_string())
        }
    }
    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount > 0.0 && self.balance >= amount {
            self.balance -= amount;
            println!(
                "Withdrew ${:.2} from account {}. New balance: ${:.2}",
                amount, self.account_number, self.balance
            );
            return Ok(());
        }
        // else if amount > self.balance {
        //     println!("error");
        //     Err(format!(
        //         "Insufficient funds! Cannot withdraw ${:.2} from account {}. Current balance: ${:.2}",
        //         amount, self.account_number, self.balance
        //     ))
        // }
        else {
            println!("Withdraw amount must be greater than zero.");
            Err("Withdraw amount must be greater than zero.".to_string())
        }
    }
    fn balance(&self) -> f64 {
        self.balance
    }
}
