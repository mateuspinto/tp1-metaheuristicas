#![allow(dead_code)]
use rand::Rng;

fn max(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

fn min(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

fn bound_percent(bounds: (f64, f64), noise_module_percent: f64) -> f64 {
    (bounds.0 - bounds.1).abs() * noise_module_percent
}

fn noised_number(
    noise_module: f64,
    value: f64,
    bounds: (f64, f64),
    rng: &mut rand::prelude::ThreadRng,
) -> f64 {
    rng.gen_range(max(value - noise_module, bounds.0)..=min(value + noise_module, bounds.1))
}

pub fn hill_climb(
    start_solution: (f64, f64),
    f: fn((f64, f64)) -> f64,
    bounds: ((f64, f64), (f64, f64)),
    noise_chance: f64,
    noise_module_percent: f64,
    max_useless_iterations: u64,
    rng: &mut rand::prelude::ThreadRng,
) -> (f64, f64) {
    let mut actual_solution = start_solution;
    let mut candidate_solution = actual_solution.clone();
    let mut useless_iterations = 0;

    while useless_iterations < max_useless_iterations {
        if rng.gen_range(0.0..=1.0) <= noise_chance {
            candidate_solution.0 = noised_number(
                bound_percent(bounds.0, noise_module_percent),
                candidate_solution.0,
                bounds.0,
                rng,
            );
        }

        if rng.gen_range(0.0..=1.0) <= noise_chance {
            candidate_solution.1 = noised_number(
                bound_percent(bounds.1, noise_module_percent),
                candidate_solution.1,
                bounds.1,
                rng,
            );
        }

        if f(candidate_solution) < f(actual_solution) {
            actual_solution = candidate_solution.clone();
            useless_iterations = 0;
        } else {
            candidate_solution = actual_solution.clone();
            useless_iterations += 1;
        }
    }
    actual_solution
}

pub fn ils(
    start_solution: (f64, f64),
    f: fn((f64, f64)) -> f64,
    bounds: ((f64, f64), (f64, f64)),
    noise_chance: f64,
    noise_module_percent: f64,
    max_useless_iterations: u64,
    rng: &mut rand::prelude::ThreadRng,
) -> (f64, f64) {
    let mut actual_solution = start_solution;
    let mut candidate_solution = actual_solution.clone();
    let mut useless_iterations = 0;

    while useless_iterations < max_useless_iterations {
        if rng.gen_range(0.0..=1.0) <= noise_chance {
            candidate_solution.0 = noised_number(
                bound_percent(bounds.0, noise_module_percent),
                candidate_solution.0,
                bounds.0,
                rng,
            );
        }

        if rng.gen_range(0.0..=1.0) <= noise_chance {
            candidate_solution.1 = noised_number(
                bound_percent(bounds.1, noise_module_percent),
                candidate_solution.1,
                bounds.1,
                rng,
            );
        }

        candidate_solution =
            hill_climb(candidate_solution, f, bounds, noise_chance, 0.01, 100, rng);

        if f(candidate_solution) < f(actual_solution) {
            actual_solution = candidate_solution.clone();
            useless_iterations = 0;
        } else {
            candidate_solution = actual_solution.clone();
            useless_iterations += 1;
        }
    }
    actual_solution
}
