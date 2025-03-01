type Money = i32;

#[derive(Default)]
struct Account {
    balance: Money,
}

impl Account {
    fn deposit(&mut self, amount: Money) {
        self.balance = self.balance + amount;
    }

    fn withdraw(&mut self, amount: Money) {
        self.balance = self.balance - amount;
    }
}

#[derive(Clone, Copy)]
enum Kind {
    Deposit,
    Withdraw
}

#[derive(Clone, Copy)]
struct Transaction {
    kind: Kind, 
    amount: Money
}

macro_rules! t {
    (d, $amount: expr) => {
        Transaction { 
            kind: Kind::Deposit,
            amount: $amount,
        }
    };
    (w, $amount: expr) => {
        Transaction { 
            kind: Kind::Withdraw,
            amount: $amount,
        }
    };
}

fn simulate_transaction(account: &mut Account, Transaction { kind, amount }: Transaction) {
    use Kind::*;
    match kind {
        Deposit => account.deposit(amount),
        Withdraw => account.withdraw(amount),
    }
}

fn simulate_balance(transactions: &[Transaction]) -> Vec<Money> {
    let mut account = Account::default();
    let mut balances = vec![account.balance];
    for &t in transactions {
        simulate_transaction(&mut account, t);
        balances.push(account.balance);
    }

    return balances
}
  

fn main() {
    let tx = [t!(d, 10), t!(d, 20), t!(w, 5)];
    println!("{:?}", simulate_balance(&tx));
}
