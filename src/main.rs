/*
IntPolMulNonbity - stochastic parallelised search for counter- and extreme
examples to inequalities between "nonbities" of 2 polynomials with nonnegative
coefficients and of their product, where a nonbity of a polynomial quantifies
its "distinctiveness" from a polynomial whose coefficients are 0 and 1 only.
Related to unfair 0-1-polynomials conjecture.

https://github.com/serhon/intpolmulnonbity

Copyright (c) 2025 IntPolMulNonbity fitters

IntPolMulNonbity is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

IntPolMulNonbity is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with IntPolMulNonbity. If not, see <https://www.gnu.org/licenses/>.
*/

extern crate num_cpus;
extern crate rand;
extern crate rand_xoshiro;
extern crate rayon;

use rand::prelude::*;
#[allow(unused_imports)]
use rand_xoshiro::Xoshiro256PlusPlus;
use rayon::prelude::*;

#[allow(unused_imports)]
use std::{
    io::{self, Write},
    time::Instant
};

struct CoefRng {
    rndgen: Xoshiro256PlusPlus,
    denom: i64,
    grav: i8
}

struct Searcher {
    deg1: usize,
    deg2: usize,
    denom1: i64,
    denom2: i64,
    grav1: i8,
    grav2: i8,
    lowest_is_unit: bool,
    gradesc_max_steps: usize,
    batch_size: usize,
    threshold: f64
}

#[derive(Clone)]
struct Example {
    pol1: Vec<i64>,
    prop1: i64,
    pol2: Vec<i64>,
    prop2: i64,
    pol_prod: Vec<i64>,
    prop_prod: i64,
    surplus: f64
}

struct SearchResult {
    counterexamples: Vec<Example>,
    extremum_example: Example,
    n_tested: u128,
    n_gradesc1_steps: u128,
    n_gradesc2_steps: u128,
    extreme_surplus: f64
}

fn print_pol(pol: & Vec<i64>, denom: i64) {
    for &c in pol {
        print!(" {:3}", c);
    }
    print!("\n");
    for &c in pol {
        print!(" {:.4}", (c as f64) / (denom as f64));
    }
    print!("\n");
}

#[allow(dead_code)]
fn has_fracs_pol(pol: & Vec<i64>, denom: i64) -> bool {
    for &c in pol {
        if (c != 0) && (c != denom) {
            return true;
        }
    } 
    false
}

fn multiply_pol(pol_prod: &mut Vec<i64>, pol1: & Vec<i64>, pol2: & Vec<i64>) {
    let deg1 = pol1.len() - 1;
    let deg2 = pol2.len() - 1;
    let deg_prod = deg1 + deg2;
    for i in 0..=deg_prod {
            pol_prod[i] = 0;
    }
    for i in 0..=deg1 {
        for j in 0..=deg2 {
            pol_prod[i + j] += pol1[i] * pol2[j];
        }
    }
}

#[allow(dead_code)]
fn has_coefs_above_denom_pol(pol: & Vec<i64>, denom: i64) -> bool {
    for &c in pol {
        if c > denom {
            return true;
        }
    }
    false
}

#[allow(dead_code)]
fn sum_coefs_pol(pol: & Vec<i64>) -> i64 {
    pol.iter().sum::<i64>()
}

#[allow(dead_code)]
fn nonbity(pol: & Vec<i64>, denom: i64) -> i64 {
    // pol.iter().fold(0, |nb, &c| nb.max(c * (denom - c).abs()))
    // pol.iter().fold(0, |nb, &c| nb.max(c.min((denom - c).abs())))
    // pol.iter().fold(0, |nb, &c| nb.max(c * c * (denom - c).abs()))
    // pol.iter().fold(0, |nb, &c| nb.max(c * (denom - c) * (denom - c)))

    pol.iter().map(|&c| c * (denom - c).abs()).sum::<i64>()
    // pol.iter().map(|&c| c.min((denom - c).abs())).sum::<i64>()
}

#[allow(dead_code)]
fn randomise_pol(pol: &mut Vec<i64>, coef_rng: &mut CoefRng, lowest_is_unit: bool) {
    let deg = pol.len() - 1;
    for i in 0..=deg {
        pol[i] = coef_rng.gen();
    }
    if lowest_is_unit {
        pol[0] = coef_rng.denom;
    }
}

fn surplusity(_denom1: i64, _denom2: i64, nb_prod: i64, nb1: i64, nb2: i64, _sum1: i64, _sum2: i64, _deg1: usize, _deg2: usize) -> f64 {
    let _deg_max = _deg1.max(_deg2) as i64;
    if (nb1 > 0) && (nb2 > 0) {
        // (nb_prod as f64) / ((nb1 * nb2) as f64)
        // ((nb_prod * _sum1 * _sum2) as f64) / ((_denom1 * _denom2 * nb1 * nb2) as f64)
        ((nb_prod * (1 + _deg_max) * (1 +_deg_max)) as f64) / (((1 + (_deg_max << 1)) * nb1 * nb2) as f64)
    } else {
        f64::MAX
    }

    // (nb_prod - nb1 * nb2) as f64
}

impl CoefRng {
    fn new(denom: i64, grav: i8) -> Self {
        let rndgen = Xoshiro256PlusPlus::from_rng(thread_rng()).unwrap();
        Self{rndgen, denom, grav}
    }

    fn gen(&mut self) -> i64 {
        let mut r = self.rndgen.gen_range(0..=self.denom);
        if self.grav > 0 {
            for _ in 1..=self.grav {
                r = r.min(self.rndgen.gen_range(0..=self.denom));
            }
        } else if self.grav < 0 {
            for _ in (-self.grav)..=(-1) {
                r = r.max(self.rndgen.gen_range(0..=self.denom));
            }
        }
        r
    }

}

impl Example {
    fn new_default() -> Self {
        Self {
            pol1: vec![],
            prop1: i64::MIN,
            pol2: vec![],
            prop2: i64::MIN,
            pol_prod: vec![],
            prop_prod: i64::MIN,
            surplus: f64::NAN
        }
    }

    fn new(pol1: & Vec<i64>, prop1: i64, pol2: & Vec<i64>, prop2: i64, pol_prod: & Vec<i64>, prop_prod: i64, surplus: f64) -> Self {
        Self {
            pol1: pol1.clone(),
            prop1,
            pol2: pol2.clone(),
            prop2,
            pol_prod: pol_prod.clone(),
            prop_prod,
            surplus
        }
    }

    fn print(&self, denom1: i64, denom2: i64) {
        println!("Pol_1 (Nonbity = {}):", self.prop1);
        print_pol(& self.pol1, denom1);
        println!("Pol_2 (Nonbity = {}):", self.prop2);
        print_pol(& self.pol2, denom2);
        println!("Pol_prod (Nonbity = {}):", self.prop_prod);
        print_pol(& self.pol_prod, denom1 * denom2);
        println!("Surplus = {}\n", self.surplus);
    }

}

impl Searcher {
    fn new(deg1: usize, deg2: usize, denom1: i64, denom2: i64, grav1: i8, grav2: i8, lowest_is_unit: bool, gradesc_max_steps: usize, batch_size: usize, threshold: f64) -> Self {
        Self{
            deg1, deg2, denom1, denom2, grav1, grav2, lowest_is_unit, gradesc_max_steps, batch_size, threshold
        }
    }

    fn search(&self) -> SearchResult {
        let denom_prod = self.denom1 * self.denom2;
        let min_change_deg: usize = if self.lowest_is_unit { 1 } else { 0 };

        let mut pol1 = vec![0i64; self.deg1 + 1];
        let mut pol2 = vec![0i64; self.deg2 + 1];
        let mut pol1_next = vec![0i64; self.deg1 + 1];
        let mut pol2_next = vec![0i64; self.deg2 + 1];

        // Partial "fastest descents"
        let mut pfds1 = vec![0i64; self.deg1 + 1];
        let mut pfds2 = vec![0i64; self.deg2 + 1];

        let mut pol_prod = vec![0i64; self.deg1 + self.deg2 + 1];

        let mut coef1_rng = CoefRng::new(self.denom1, self.grav1);
        let mut coef2_rng = CoefRng::new(self.denom2, self.grav2);

        let mut counterexamples = Vec::<Example>::new();
        let mut extremum_example = Example::new_default();
        let mut n_tested: u128 = 0;
        let mut n_gradesc1_steps: u128 = 0;
        let mut n_gradesc2_steps: u128 = 0;
        let mut extreme_surplus = f64::MAX;

        for _ in 0..self.batch_size {
            randomise_pol(&mut pol1, &mut coef1_rng, self.lowest_is_unit);
            randomise_pol(&mut pol2, &mut coef2_rng, self.lowest_is_unit);

            if has_fracs_pol(&pol1, self.denom1) && has_fracs_pol(&pol2, self.denom2) {
                n_tested += 1;

                let mut nb1 = nonbity(&pol1, self.denom1);
                let mut sum1 = sum_coefs_pol(&pol1);
                let mut nb2 = nonbity(&pol2, self.denom2);
                let mut sum2 = sum_coefs_pol(&pol2);

                multiply_pol(&mut pol_prod, &pol1, &pol2);

                let mut surplus = surplusity(self.denom1, self.denom2, nonbity(&pol_prod, denom_prod), nb1, nb2, sum1, sum2, self.deg1, self.deg2);

                // "Gradient descent I" (all variables)

                pol1_next.clone_from(&pol1);
                pol2_next.clone_from(&pol2);

                for _ in 0..self.gradesc_max_steps {
                    n_gradesc1_steps += 1;

                    for i in min_change_deg..=self.deg1 {
                        // Try increment
                        pol1_next[i] = (pol1[i] + 1).min(self.denom1);
                        nb1 = nonbity(&pol1_next, self.denom1);
                        sum1 = sum_coefs_pol(&pol1_next);
                        multiply_pol(&mut pol_prod, &pol1_next, &pol2);
                        let surplus_inc = surplusity(self.denom1, self.denom2, nonbity(&pol_prod, denom_prod), nb1, nb2, sum1, sum2, self.deg1, self.deg2);
                        let delta_inc = surplus_inc - surplus;

                        // Try decrement
                        pol1_next[i] = (pol1[i] - 1).max(0);
                        nb1 = nonbity(&pol1_next, self.denom1);
                        sum1 = sum_coefs_pol(&pol1_next);
                        multiply_pol(&mut pol_prod, &pol1_next, &pol2);
                        let surplus_dec = surplusity(self.denom1, self.denom2, nonbity(&pol_prod, denom_prod), nb1, nb2, sum1, sum2, self.deg1, self.deg2);
                        let delta_dec = surplus_dec - surplus;

                        pol1_next[i] = pol1[i]; // restore
                        
                        // Choose "fastest descent" (inc / keep / dec)
                        pfds1[i] = 0; // keep by default
                        if (delta_inc < 0.0) && (delta_inc <= delta_dec) { // inc
                            pfds1[i] = 1;
                        } else if (delta_dec < 0.0) && (delta_dec <= delta_inc) { // dec
                            pfds1[i] = -1;
                        }
                    }

                    // Another restoration
                    nb1 = nonbity(&pol1, self.denom1);
                    sum1 = sum_coefs_pol(&pol1);

                    for i in min_change_deg..=self.deg2 {
                        // Try increment
                        pol2_next[i] = (pol2[i] + 1).min(self.denom2);
                        nb2 = nonbity(&pol2_next, self.denom2);
                        sum2 = sum_coefs_pol(&pol2_next);
                        multiply_pol(&mut pol_prod, &pol1, &pol2_next);
                        let surplus_inc = surplusity(self.denom1, self.denom2, nonbity(&pol_prod, denom_prod), nb1, nb2, sum1, sum2, self.deg1, self.deg2);
                        let delta_inc = surplus_inc - surplus;

                        // Try decrement
                        pol2_next[i] = (pol2[i] - 1).max(0);
                        nb2 = nonbity(&pol2_next, self.denom2);
                        sum2 = sum_coefs_pol(&pol2_next);
                        multiply_pol(&mut pol_prod, &pol1, &pol2_next);
                        let surplus_dec = surplusity(self.denom1, self.denom2, nonbity(&pol_prod, denom_prod), nb1, nb2, sum1, sum2, self.deg1, self.deg2);
                        let delta_dec = surplus_dec - surplus;

                        pol2_next[i] = pol2[i]; // restore
                        
                        // Choose "fastest descent" (inc / keep / dec)
                        pfds2[i] = 0; // keep by default
                        if (delta_inc < 0.0) && (delta_inc <= delta_dec) { // inc
                            pfds2[i] = 1;
                        } else if (delta_dec < 0.0) && (delta_dec <= delta_inc) { // dec
                            pfds2[i] = -1;
                        }
                    }

                    for i in min_change_deg..=self.deg1 {
                        pol1_next[i] = (pol1[i] + pfds1[i]).max(0).min(self.denom1);
                    }
                    nb1 = nonbity(&pol1_next, self.denom1);
                    sum1 = sum_coefs_pol(&pol1_next);

                    for i in min_change_deg..=self.deg2 {
                        pol2_next[i] = (pol2[i] + pfds2[i]).max(0).min(self.denom2);
                    }
                    nb2 = nonbity(&pol2_next, self.denom2);
                    sum2 = sum_coefs_pol(&pol2_next);

                    multiply_pol(&mut pol_prod, &pol1_next, &pol2_next);

                    let surplus_next = surplusity(self.denom1, self.denom2, nonbity(&pol_prod, denom_prod), nb1, nb2, sum1, sum2, self.deg1, self.deg2);
                    if surplus_next < surplus {
                        pol1.clone_from(&pol1_next);
                        pol2.clone_from(&pol2_next);
                        surplus = surplus_next;
                    } else {
                        break;
                    }

                }

                // "Gradient descent II" (one by one variable)

                pol1_next.clone_from(&pol1);
                pol2_next.clone_from(&pol2);

                'gradesc2: for _ in 0..self.gradesc_max_steps {
                    n_gradesc2_steps += 1;

                    nb2 = nonbity(&pol2, self.denom2);
                    sum2 = sum_coefs_pol(&pol2);

                    for i in min_change_deg..=self.deg1 {
                        // Try increment
                        pol1_next[i] = (pol1[i] + 1).min(self.denom1);
                        nb1 = nonbity(&pol1_next, self.denom1);
                        sum1 = sum_coefs_pol(&pol1_next);
                        multiply_pol(&mut pol_prod, &pol1_next, &pol2);
                        let surplus_next = surplusity(self.denom1, self.denom2, nonbity(&pol_prod, denom_prod), nb1, nb2, sum1, sum2, self.deg1, self.deg2);
                        if surplus_next < surplus {
                            pol1[i] = pol1_next[i];
                            surplus = surplus_next;
                            continue 'gradesc2; 
                        }

                        // Try decrement
                        pol1_next[i] = (pol1[i] - 1).max(0);
                        nb1 = nonbity(&pol1_next, self.denom1);
                        sum1 = sum_coefs_pol(&pol1_next);
                        multiply_pol(&mut pol_prod, &pol1_next, &pol2);
                        let surplus_next = surplusity(self.denom1, self.denom2, nonbity(&pol_prod, denom_prod), nb1, nb2, sum1, sum2, self.deg1, self.deg2);
                        if surplus_next < surplus {
                            pol1[i] = pol1_next[i];
                            surplus = surplus_next;
                            continue 'gradesc2;
                        } else {
                            pol1_next[i] = pol1[i];
                        }
                    }

                    nb1 = nonbity(&pol1, self.denom1);
                    sum1 = sum_coefs_pol(&pol1);

                    for i in min_change_deg..=self.deg2 {
                        // Try increment
                        pol2_next[i] = (pol2[i] + 1).min(self.denom2);
                        nb2 = nonbity(&pol2_next, self.denom2);
                        sum2 = sum_coefs_pol(&pol2_next);
                        multiply_pol(&mut pol_prod, &pol1, &pol2_next);
                        let surplus_next = surplusity(self.denom1, self.denom2, nonbity(&pol_prod, denom_prod), nb1, nb2, sum1, sum2, self.deg1, self.deg2);
                        if surplus_next < surplus {
                            pol2[i] = pol2_next[i];
                            surplus = surplus_next;
                            continue 'gradesc2;
                        }

                        // Try decrement
                        pol2_next[i] = (pol2[i] - 1).max(0);
                        nb2 = nonbity(&pol2_next, self.denom2);
                        sum2 = sum_coefs_pol(&pol2_next);
                        multiply_pol(&mut pol_prod, &pol1, &pol2_next);
                        let surplus_next = surplusity(self.denom1, self.denom2, nonbity(&pol_prod, denom_prod), nb1, nb2, sum1, sum2, self.deg1, self.deg2);
                        if surplus_next < surplus {
                            pol2[i] = pol2_next[i];
                            surplus = surplus_next;
                            continue 'gradesc2; 
                        } else {
                            pol2_next[i] = pol2[i];
                        }
                    }

                    // All tries failed
                    break;
                }

                if (surplus < extreme_surplus) || (surplus < self.threshold) {
                    nb1 = nonbity(&pol1, self.denom1);
                    nb2 = nonbity(&pol2, self.denom2);
                    multiply_pol(&mut pol_prod, &pol1, &pol2);
                    let nb_prod = nonbity(&pol_prod, denom_prod);
                    let example = Example::new(&pol1, nb1, &pol2, nb2, &pol_prod, nb_prod, surplus);
                    if surplus < extreme_surplus {
                        extreme_surplus = surplus;
                        extremum_example = example.clone();
                    }
                    if surplus < self.threshold {
                        counterexamples.push(example);
                    }                    
                }
            }
        }

        SearchResult{
            counterexamples,
            extremum_example,
            n_tested,
            n_gradesc1_steps,
            n_gradesc2_steps,
            extreme_surplus
        }
    }

}

fn run_search_rand() {
    let mut rng = thread_rng();

    let deg_min: usize = 3;
    let deg_max: usize = 17;

    let denom_min: i64 = 4;
    let denom_max: i64 = 8;

    let grav_min: i8 = -4;
    let grav_max: i8 = 1;

    let lowest_is_unit = false;

    let gradesc_max_steps: usize = 4000;

    let batch_size: usize = 1000;

    let threshold: f64 = 1.0;

    let n_searchers = num_cpus::get();

    print!("Using {} parallel searchers.\n\n", n_searchers);
    io::stdout().flush().unwrap_or(());

    let mut n_attempts: u128 = 0;
    let mut n_tested: u128 = 0;
    let mut n_gradesc1_steps: u128 = 0;
    let mut n_gradesc2_steps: u128 = 0;
    let mut n_counterexamples: u128 = 0;
    let mut extreme_surplus = f64::MAX;

    let t_start = Instant::now();

    loop {
        let deg1: usize = rng.gen_range(deg_min..=deg_max);
        let deg2: usize = rng.gen_range(deg_min..=deg_max);

        let denom1 = rng.gen_range(denom_min..=denom_max);
        let denom2 = rng.gen_range(denom_min..=denom_max);

        let grav1 = rng.gen_range(grav_min..=grav_max);
        let grav2 = rng.gen_range(grav_min..=grav_max);

        print!("\rD1={:2} D2={:2} | N1={:3} N2={:3} | G1={:2} G2={:2}\t", deg1, deg2, denom1, denom2, grav1, grav2);
        io::stdout().flush().unwrap_or(());

        let searcher = Searcher::new(
            deg1, deg2,
            denom1, denom2,
            grav1, grav2,
            lowest_is_unit,
            gradesc_max_steps,
            batch_size,
            threshold
        );

        let search_results: Vec<SearchResult> = (0..n_searchers).into_par_iter().map(|_| {
            searcher.search()
        }).collect();

        n_attempts += (n_searchers as u128) * (batch_size as u128);

        let mut n_new_counterexamples: u128 = 0;
        let mut new_extreme_surplus = extreme_surplus;
        let mut extremum_example = Example::new_default();

        search_results.iter().for_each(|sr| {
            n_new_counterexamples += sr.counterexamples.len() as u128;
            n_tested += sr.n_tested;
            n_gradesc1_steps += sr.n_gradesc1_steps;
            n_gradesc2_steps += sr.n_gradesc2_steps;
            if sr.extreme_surplus < new_extreme_surplus {
                new_extreme_surplus = sr.extreme_surplus;
                extremum_example = sr.extremum_example.clone();
            }            
        });

        let rate = 1e3 * (n_tested as f64) / (t_start.elapsed().as_millis().max(1) as f64);

        print!("CNTRS: {} | Extr: {:.4e} | AvgGDS I {}, II {} | Tested {}, {:.2}% ({:.3e}/s)\t", n_counterexamples + n_new_counterexamples, new_extreme_surplus, n_gradesc1_steps / n_tested.max(1), n_gradesc2_steps / n_tested.max(1), n_tested, 100.0 * (n_tested as f64) / (n_attempts as f64), rate);
        io::stdout().flush().unwrap_or(());        

        search_results.iter().for_each(|sr| {
            for ce in & sr.counterexamples {
                n_counterexamples += 1;
                print!("\n\nCOUNTEREXAMPLE {}:\n", n_counterexamples);
                ce.print(denom1, denom2);
                io::stdout().flush().unwrap_or(());
            }
        });

        if new_extreme_surplus < extreme_surplus {
            extreme_surplus = new_extreme_surplus;
            print!("\n\nNew extremum example:\n");
            extremum_example.print(denom1, denom2);
            io::stdout().flush().unwrap_or(());
        }
    }

}

fn main() {
    print!("IntPolMulNonbity v{}\n\n", env!("CARGO_PKG_VERSION"));
    io::stdout().flush().unwrap_or(());
    run_search_rand();
}
