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
    use std::ops::{Index, IndexMut};

    type X = f32;
    type Y = f32;

    /// The inputs and outputs of a function call.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Call<const N: usize> {
        pub xs: [X; N],
        pub y: Y,
    }

    #[allow(unused)]
    pub const TWO_D_ZERO: Call<1> = Call {
        xs: [0.],
        y: 0.,
    };

    /// A workaround for the lack of `generic_const_exprs` on stable, which would be needed to express
    /// `[[X; N]; N + 1]`.
    #[derive(Debug)]
    pub struct Simplex<const N: usize> {
        pub n: [[X; N]; N],
        pub plus_one: [X; N],
    }

    impl <const N: usize> Index<usize> for Simplex<N> {
        type Output = [X; N];

        fn index(&self, index: usize) -> &Self::Output {
            if index == self.n.len() {
                &self.plus_one
            } else {
                // Panic as we would on an array if it is too large.
                &self.n[index]
            }
        }
    }

    impl <const N: usize> IndexMut<usize> for Simplex<N> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            if index == self.n.len() {
                &mut self.plus_one
            } else {
                // Panic as we would on an array if it is too large.
                &mut self.n[index]
            }
        }
    }

    impl <const N: usize> Simplex<N> {
        pub const fn len(&self) -> usize {
            N + 1
        }
    }

    /// A regular simplex centered at the origin.
    pub fn regular_simplex<const N: usize>() -> Simplex<N> {
        let mut output = Simplex {
            n: [[0.; N]; N],
            plus_one: [0.; N],
        };

        // Uses technique as described at https://en.wikipedia.org/wiki/Simplex#Cartesian_coordinates_for_a_regular_n-dimensional_simplex_in_Rn
        // but with a scale correction.

        let cos_45 = (2.0f32).sqrt() / 2.; // AKA 1 / sqrt(2)

        let n = N as f32;

        let base = -(cos_45 / n)*(1. - (1. / ((n + 1.).sqrt())));

        for i in 0..N {
            for j in 0..N {
                output[i][j] = if i == j {
                    cos_45 + base
                } else {
                    base
                };
            }
        }

        let plus_one_value = -(1. / (2. * (n + 1.)).sqrt());

        for i in 0..N {
            output.plus_one[i] = plus_one_value;
        }

        output
    }

    #[cfg(test)]
    mod regular_simplex_works {
        use super::*;

        fn dist_from_0(point: &[f32]) -> f32 {
            point.iter().map(|x| x * x).sum::<f32>().sqrt()
        }

        macro_rules! approx_eq {
            ($a: expr, $b: expr) => {
                assert!(($a - $b).abs() < 0.0001);
            };
        }

        #[test]
        fn in_1d() {
            let output = regular_simplex::<1>();

            let first_dist = dist_from_0(&output.plus_one);

            assert!(first_dist != 0.);

            for i in 0..output.n.len() {
                approx_eq!(dist_from_0(&output.n[i]), first_dist);
            }
        }

        #[test]
        fn in_2d() {
            let output = regular_simplex::<2>();

            let first_dist = dist_from_0(&output.plus_one);

            assert!(first_dist != 0.);

            for i in 0..output.n.len() {
                approx_eq!(dist_from_0(&output.n[i]), first_dist);
            }
        }
    }

    pub fn regular_simplex_centered_at<const N: usize>(
        scale: X,
        center: [X; N]
    ) -> Simplex<N> {
        let mut output = regular_simplex::<N>();

        for vertex_index in 0..output.len() {
            for i in 0..N {
                output[vertex_index][i] =
                    output[vertex_index][i] * scale + center[i];
            }
        }

        output
    }

    /// Find the minimum of the given function withing the given simplex.
    /// If in doubt of what to use for the simplex, pass
    /// `regular_simplex_centered_at(scale, center)` where `center` is a
    /// best guess for the minimum, and scale is large enough that the
    /// resulting simplex covers the desired minimum.
    pub fn minimize<const N: usize>(
        f: impl Fn([X; N]) -> Y,
        initial_simplex: Simplex<N>,
        // 64k iterations ought to be enough for anybody!
        iters: u16,
    ) -> Call<N> {
        // Nelder–Mead method
        // References used:
        // Wikipedia Article: https://en.wikipedia.org/wiki/Nelder%E2%80%93Mead_method
        // A paper: https://www.researchgate.net/publication/385833573_The_Nelder-Mead_Simplex_Algorithm_Is_Sixty_Years_Old_New_Convergence_Results_and_Open_Questions
        // For the name of the convergence constants we use the greek letter naming convention from the article.
        // Otherwise we use the naming convention from that paper. We implement the ordered version.
        const ALPHA: X = 1.;
        const GAMMA: X = 2.;
        const RHO: X = 0.5;
        const SIGMA: X = -0.5;

        let mut k = 0;

        // TODO? Do this on the stack?
        let mut s = Vec::with_capacity(N + 1);
        for i in 0..(N + 1) {
            let xs = initial_simplex[i];
            s.push(Call { xs, y: f(xs) });
        }


        while k < iters {
            // Order
            s.sort_by(|a, b| a.y.partial_cmp(&b.y).expect("should have no NaNs"));

            let l_k = 0;
            let h_k = s.len() - 1;

            let x_1 = s[l_k].xs;
            let f_1 = s[l_k].y;
            let f_n = s[N - 1].y;
            let f_n_1 = s[N].y;

            let x_h_k = s[h_k].xs;

            let x_c = {
                let mut sum = [0.; N];

                for call in s.iter() {
                    for i in 0..N {
                        sum[i] += call.xs[i];
                    }
                }

                let scale = 1. / (N as X);

                for i in 0..N {
                    sum[i] *= scale;
                }

                sum
            };

            let x_super_k = |alpha| {
                let mut output = x_c;
                for i in 0..N {
                    output[i] *= 1. + alpha;
                    output[i] -= alpha * x_h_k[i];
                }
                output
            };

            // Reflect
            let x_r = x_super_k(ALPHA);
            let f_r = f(x_r);

            if f_1 <= f_r && f_r < f_n {
                s[h_k] = Call { xs: x_r, y: f_r };
            }

            // Expand
            let x_e = x_super_k(GAMMA);
            let f_e = f(x_e);

            if f_r < f_1 && f_e < f_r {
                s[h_k] = Call { xs: x_e, y: f_e };
            } else if f_r < f_1 && f_r <= f_e {
                s[h_k] = Call { xs: x_r, y: f_r };
            }

            // Contract Outside
            let x_oc = x_super_k(RHO);
            let f_oc = f(x_oc);

            if f_n <= f_r && f_r < f_n_1 && f_oc <= f_r {
                s[h_k] = Call { xs: x_oc, y: f_oc };
            }

            // Contract Inside
            let x_ic = x_super_k(SIGMA);
            let f_ic = f(x_ic);

            if f_r >= f_n_1 && f_ic < f_n_1 && f_oc <= f_r {
                s[h_k] = Call { xs: x_ic, y: f_ic };
            }

            // Shrink
            if (f_n <= f_r && f_r < f_n_1 && f_oc > f_r) || if f_r < f_ic { f_r } else { /* NaN ends up here. Hopefully that case doesn't happen! */ f_ic }  >= f_n_1 {
                for i in 0..s.len() {
                    let mut xs = x_1;
                    for j in 0..N {
                        xs[j] += s[i].xs[j];
                        xs[j] *= 0.5;
                    }
                    s[i] = Call { xs, y: f(xs) };
                }
            }

            k += 1;
        }

        s[0]
    }

    #[cfg(test)]
    mod minimize_works {
        use super::*;

        #[test]
        fn on_x_squared() {
            // Start on the answer
            assert_eq!(
                minimize::<1>(|[x]| x * x, regular_simplex_centered_at(1.0, [0.0]), 100),
                TWO_D_ZERO,
            );

            // Start such that inital simplex contains the answer
            assert_eq!(
                minimize::<1>(|[x]| x * x, regular_simplex_centered_at(2.0, [1.0]), 100),
                TWO_D_ZERO,
            );

            // Start further away
            assert_eq!(
                minimize::<1>(|[x]| x * x, regular_simplex_centered_at(4.0, [-2.0]), 100),
                TWO_D_ZERO,
            );
        }
    }

}
use minimize::{minimize, regular_simplex_centered_at};

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
        regular_simplex_centered_at(100.0, [50.0]),
        100
    );

    println!(
        "minimum: {:?} -> {}",
        design_1_minimum_xy.xs,
        design_1_minimum_xy.y
    );
}
