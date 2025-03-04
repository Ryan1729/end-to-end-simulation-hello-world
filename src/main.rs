#![allow(non_snake_case)] // Keep the names from the article.

const ANNUAL_FORTNIGHTS: u8 = 26;

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

type Performance = f32;

fn translate_performance_TargetBalance(balances: &[Money], target: Money) -> Performance {
    let mut sum = 0;
    for b in balances {
        sum += (b - target).abs();
    }
    (sum as Performance) / (balances.len() as Performance)
}

fn translate_performance_Target100(balances: &[Money]) -> Performance {
    translate_performance_TargetBalance(balances, 100)
}

type DesignParameters = (Money, Money);

macro_rules! p {
    ($_0: expr $(,)?) => {
        ($_0, 0)
    };
    ($_0: expr, $_1: expr) => {
        ($_0, $_1)
    };
}

type DesignTranslator = fn (design_parameters: DesignParameters) -> Vec<Transaction>;

fn translate_design_FortnightlyDeposit(design_parameters: DesignParameters) -> Vec<Transaction> {
    vec![t!(d, design_parameters.0); ANNUAL_FORTNIGHTS as _]
}

fn translate_design_InitialAndFortnightlyDeposit(design_parameters: DesignParameters) -> Vec<Transaction> {
    let mut output = vec![t!(d, design_parameters.1); ANNUAL_FORTNIGHTS as usize + 1];

    output[0] = t!(d, design_parameters.0);

    output
}

fn performance_of_design(design_translator: DesignTranslator, design_parameters: DesignParameters) -> Performance {
  return translate_performance_Target100(
        &simulate_balance(
            &design_translator(design_parameters)
        )
    )
}

macro_rules! evaluate {
    ($design_translator: ident, $design_parameters: expr) => {
        println!("\nevaluating account balance target 100");
        println!("with {} {:?}", stringify!($design_translator), $design_parameters);
        println!("the mean abs delta is {:.2}", performance_of_design($design_translator, $design_parameters));
    }
}


fn sample_performance_of_design(design_translator: DesignTranslator, design_variants: &[Money]) -> Vec<Performance> {
    design_variants
        .iter()
        .map(|&m| performance_of_design(design_translator, p!(m)))
        .collect()
}

fn visualise_performance_of_designs(performances: &[Performance], designs: &[Money]) {
    assert_eq!(performances.len(), designs.len());
    print!("[");
    for i in 0..performances.len() {
        print!("({},{}),", designs[i], performances[i]);
    }
    println!("]");
}

fn main() {
    let tx = [t!(d, 10), t!(d, 20), t!(w, 5)];
    let sb = simulate_balance(&tx);
    println!("{sb:?}");

    println!("{:?}", translate_performance_Target100(&sb));

    println!("{:?}", simulate_balance(&translate_design_FortnightlyDeposit(p!(10,))));

    let design_1 = p!(9,);

    evaluate!(translate_design_FortnightlyDeposit, design_1);

    println!("{:?}", simulate_balance(&translate_design_FortnightlyDeposit(design_1)));

    let design_2 = p!(90, 1);

    println!("{:?}", simulate_balance(&translate_design_InitialAndFortnightlyDeposit(design_2)));

    let design_sweep = (0..16).collect::<Vec<_>>();
    let performances = sample_performance_of_design(translate_design_FortnightlyDeposit, &design_sweep);

    visualise_performance_of_designs(&performances, &design_sweep);
}
