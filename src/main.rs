mod common;
use rand::Rng;

fn f(xs: (f64, f64)) -> f64 {
    //1 // degree_to_rad(xs.0 + xs.1).sin() + (xs.0 - xs.1).powi(2) - 1.5 * xs.0 + 2.5 * xs.1 + 1.0
    //2 // -1.0 * (xs.1 + 47.0) * degree_to_rad((xs.0 / 2.0 + (xs.1 + 47.0)).abs().sqrt()).sin() - 1.0 * xs.0 * degree_to_rad((xs.0 - (xs.1 + 47.0)).abs().sqrt()).sin()
    //
    degree_to_rad(xs.0 + xs.1).sin() + (xs.0 - xs.1).powi(2) - 1.5 * xs.0 + 2.5 * xs.1 + 1.0
}

fn main() {
    let mut rng = rand::thread_rng();
    let bounds: ((f64, f64), (f64, f64)) = ((-1.5, 4.0), (-3.0, 4.0));
    //1a // ((-1.5, 4.0), (-3.0, 4.0));
    //1b // ((-1.0, 0.0), (-2.0, -1.0));
    //2a // ((-512.0, 512.0), (-512.0, 512.0));
    //2b // ((511.0, 512.0), (404.0, 405.0));

    println!("f");
    for _ in 0..30 {
        let start_solution: (f64, f64) = (
            rng.gen_range(bounds.0 .0..=bounds.0 .1),
            rng.gen_range(bounds.1 .0..=bounds.1 .1),
        );
        let result = common::hill_climb(start_solution, f, bounds, 0.5, 0.1, 1000, &mut rng);
        // common::hill_climb(start_solution, f, bounds, 0.5, 0.1, 1000, &mut rng);
        // common::ils(start_solution, f, bounds, 0.5, 1.0, 1000, &mut rng);

        println!("{}", f(result));
    }
}

fn degree_to_rad(degree: f64) -> f64 {
    degree * std::f64::consts::PI / 180.0
}
