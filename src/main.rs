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

mod minimize {
    type X = f32;
    type Y = f32;

    /// The inputs and outputs of a function call.
    #[derive(Debug, PartialEq)]
    pub struct Call<const N: usize> {
        pub xs: [X; N],
        pub y: Y,
    }

    impl <const N: usize> Call<N> {
        pub const TWO_D_ZERO: Call<1> = Call {
            xs: [0.],
            y: 0.,
        };
    }

    pub fn minimize<const N: usize>(
        f: impl Fn([X; N]) -> Y,
        initial_guess: [X; N],
    ) -> Call<N> {
        // Nelder–Mead method
        // References used:
        // Wikipedia Article: https://en.wikipedia.org/wiki/Nelder%E2%80%93Mead_method
        // A paper: https://www.researchgate.net/publication/385833573_The_Nelder-Mead_Simplex_Algorithm_Is_Sixty_Years_Old_New_Convergence_Results_and_Open_Questions
        // For the name of the convergence constants we use the greek letter naming convention from the article.
        // Otherwise we use the naming convention from that paper. We implement the ordered version.
        //const ALPHA: X = 1.;
        //const GAMMA: X = 2.;
        //const RHO: X = 0.5;
        //const SIGMA: X = -0.5;
//
        //let mut k = 0;
//
        //const ITER_THRESHOLD: u8 = 100;
//
        //// (x, f(x))
        //let mut s = vec![(initial_guess, func(initial_guess))];
        //assert!(s.len() > 0);
//
        //while k < ITER_THRESHOLD {
            //// Order
            //s.sort_by(|(_x, y)| y);
//
            //let l_k = 0;
            //let h_k = s.len() - 1;
//
            //let f_1 = s[i_1].1;
            //let f_n = s[i_n].1;
//
            //let x_h_k = s[h_k].0;
//
            //let x_c = {
                //
            //};
//
            //let x_super_k = |alpha| {
                //(1 + alpha) * x_c - alpha
            //};
//
            //let x_r = x_super_k(ALPHA);
//
            //let f_r = f(x_r);
//
            //// Reflect
            //if f_1 <= f_r && f_r < f_n{
//
            //}
//
            //k += 1;
        //}

        println!("Dummy implmentation for now");
        Call {
            xs: initial_guess,
            y: f(initial_guess),
        }
    }

    #[cfg(test)]
    mod minimize_works {
        use super::*;

        #[test]
        fn on_x_squared() {
            assert_eq!(
                minimize::<1>(|[x]| x * x, [0.3]),
                Call::TWO_D_ZERO,
            );

            assert_eq!(
                minimize::<1>(|[x]| x * x, [-0.3]),
                Call::TWO_D_ZERO,
            );
        }
    }

}
use minimize::minimize;

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

    let design_1_minimum_xy = minimize(
        |[x]| performance_of_design(translate_design_FortnightlyDeposit, p!(x.round() as i32)),
        [3.0]
    );

    println!(
        "minimum: {:?} -> {}",
        design_1_minimum_xy.xs,
        design_1_minimum_xy.y
    );
}
