#![deny(clippy::all)]
use rand::prelude::*;
use std::io;

struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f64,
    password: String,
}

struct BankAccountList {
    accounts: Vec<BankAccount>,
    next_id: u32,
}

trait Account {
    fn deposit(&mut self, acc_num: String, amount: f64) -> Result<String, String>;
    fn withdraw(&mut self, acc_num: String, amount: f64) -> Result<String, String>;
    fn balance(&mut self, acc_num: String) -> f64;
    fn new_account_list() -> BankAccountList;
    fn check_password(&mut self, password: String, account_number: &str) -> Result<String, String>;
    fn add_account(
        &mut self,
        account_number: String,
        holder_name: String,
        password: String,
    );
}

impl Account for BankAccountList {
    fn deposit(&mut self, acc_num: String, amount: f64) -> Result<String, String> {
        if self.accounts.is_empty() {
            return Err("No accounts available".to_string());
        }
        if amount == 0.0 {
            return Err("Nothing to deposit".to_string());
        }
        for account in &mut self.accounts {
            if account.account_number == acc_num {
                if amount > 20_000.0 {
                    return Err("Maximum daily deposit limit of 20,000".to_string());
                } else {
                    account.balance += amount;
                    println!("Your balance is {}", account.balance);
                    return Ok(format!("{amount} successfully deposited!"));
                }
            }
        }
        Err("Account not found".to_string())
    }

    fn withdraw(&mut self, acc_num: String, amount: f64) -> Result<String, String> {
        for account in &mut self.accounts {
            if account.account_number == acc_num {
                if amount > account.balance {
                    return Err("Withdraw failed: Insufficient funds".to_string());
                } else {
                    account.balance -= amount;
                    println!("Your balance is {}", account.balance);
                    return Ok("{amount} successfully withdrawn!!".to_string());
                }
            }
        }
        Err("Account not found".to_string())
    }

    fn balance(&mut self, acc_num: String) -> f64 {
        let mut wanted_balance = 0.0;
        for account in &mut self.accounts {
            if account.account_number == acc_num {
                wanted_balance = account.balance;
            } else {
                println!("Account not found");
            }
        }
        wanted_balance
    }

    fn new_account_list() -> BankAccountList {
        BankAccountList {
            accounts: Vec::new(),
            next_id: 1,
        }
    }

    fn check_password(&mut self, password: String, account_number: &str)  -> Result<String, String> {
        for account in &mut self.accounts {
            if account.account_number == account_number {
                if account.password == password {
                    return Ok("Authentication Successful".to_string());
                }
                else {
                    return Err("Authentication Failed".to_string());
                }
            }
        }
        Err("Account not found".to_string())
    }

    fn add_account(
        &mut self,
        account_number: String,
        holder_name: String,
        password: String,
    ) {
        let account = BankAccount {
            account_number,
            holder_name,
            password,
            balance: 0.0,
        };
        println!(
            "The user {} has been created and their account number is {}",
            &account.holder_name, &account.account_number
        );
        self.accounts.push(account);
        self.next_id += 1;
    }
}

fn main() {
    let mut rng = rand::rng();
    let mut bank_account_list = BankAccountList::new_account_list();
    println!("\n==================================");
    println!("== Welcome to Doma's Rust Bank ==");
    println!("==================================");
    loop {
        println!("\nPlease Choose One Of The Following");
        println!("1. Create an account");
        println!("2. Check balance");
        println!("3. Deposit an amount");
        println!("4. Withdraw an amount");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to pick a choice");
        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Pick a number from 1 to 5");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Please enter your full name");
                let mut full_name = String::new();
                io::stdin()
                    .read_line(&mut full_name)
                    .expect("Failed to get client's name");
                let full_name = full_name.trim();

                let acc_num: u64 = rng.random_range(0000_0000_0000_0000..9999_9999_9999_9999);
                let acc_num = format!("{acc_num:016}");
                let acc_num = format!(
                    "{} {} {} {}",
                    &acc_num[0..4],
                    &acc_num[4..8],
                    &acc_num[8..12],
                    &acc_num[12..16],
                );

                let mut password = String::new();
                loop {
                    println!("Please enter a password, spaces at the end don't count");
                    password.clear();
                    io::stdin()
                        .read_line(&mut password)
                        .expect("Failed to get client's password");
                    let password = password.trim();
                    if password.len() >= 4 {
                        let password = password.to_string();
                        loop {
                            println!("Please re-enter your password");
                            let mut password1 = String::new();
                            io::stdin()
                                .read_line(&mut password1)
                                .expect("Failed to get client's password");
                            let password1 = password1.trim().to_string();
                            if password == password1 {
                                    bank_account_list.add_account(
                                    acc_num,
                                    full_name.to_string(),
                                    password,
                                );
                                break;
                            }
                            println!("Passwords don't match");
                        }
                        break;
                    }
                    println!("Password must be at least 4 characters long");
                }
                continue;
            }
            2 => {
                println!("Please enter your account number");
                let mut acc_num = String::new();
                io::stdin()
                    .read_line(&mut acc_num)
                    .expect("Failed to get account number");
                let acc_num = acc_num.trim().to_string();

                println!("Please enter your password");
                let mut password = String::new();
                io::stdin()
                    .read_line(&mut password)
                    .expect("Failed to get account number");
                let password = password.trim().to_string();

                let authentication = bank_account_list.check_password(password, &acc_num);

                match authentication {
                    Ok(_) => {
                        let balance = bank_account_list.balance(acc_num);
                        println!("Your balance is {balance}");
                        continue;
                    }
                    Err(e) => {
                        println!("{e}");
                        continue;
                    }
                }
            }
            3 => {
                println!("Please enter your account number");
                let mut acc_num = String::new();
                io::stdin()
                    .read_line(&mut acc_num)
                    .expect("Failed to get account number");
                let acc_num = acc_num.trim().to_string();

                println!("Please enter your password");
                let mut password = String::new();
                io::stdin()
                    .read_line(&mut password)
                    .expect("Failed to get account number");
                let password = password.trim().to_string();

                let authentication = bank_account_list.check_password(password, &acc_num);

                match authentication {
                    Ok(_) => {
                        println!("Please enter the amount to deposit");
                        let mut amount = String::new();
                        io::stdin()
                            .read_line(&mut amount)
                            .expect("Failed to get the amount");
                        let amount: f64 = amount.trim().parse().unwrap_or(0.0);
                        let _deposited = bank_account_list.deposit(acc_num, amount);
                        continue;
                    }
                    Err(e) => {
                        println!("{e}");
                        continue;
                    }
                }
            }
            4 => {
                println!("Please enter your account number");
                let mut acc_num = String::new();
                io::stdin()
                    .read_line(&mut acc_num)
                    .expect("Failed to get account number");
                let acc_num = acc_num.trim().to_string();

                println!("Please enter your password");
                let mut password = String::new();
                io::stdin()
                    .read_line(&mut password)
                    .expect("Failed to get account number");
                let password = password.trim().to_string();

                let authentication = bank_account_list.check_password(password, &acc_num);

                match authentication {
                    Ok(_) => {
                        println!("Please enter the amount to withdraw");
                        let mut amount = String::new();
                        io::stdin()
                            .read_line(&mut amount)
                            .expect("Failed to get the amount");
                        let amount: f64 = amount.trim().parse().unwrap_or(0.0);
                        let _withdrawn = bank_account_list.withdraw(acc_num, amount);
                        continue;
                    }
                    Err(e) => {
                        println!("{e}");
                        continue;
                    }
                }
            }
            5 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Pick a number from 1 to 5");
                continue;
            }
        }
    }
}
