use plotters::prelude::*;
use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};
use rand_pcg::Pcg64Mcg;

fn main() {
    let a = 0.0;
    let b = 8.0;
    let max = 2.0;


    let drawing_area = BitMapBackend::new("images/2.0.png", (1024, 768))
        .into_drawing_area();

    let _chart = ChartBuilder::on(&drawing_area)
        .build_cartesian_2d(a..=b, 0..max)
        .unwrap();
    
    for i in (50..=5000).step_by(50) {
        println!("{} {}", i, monte_carlo_integrate(a, b, max, i, f_cube_root));
    }
}

fn f_cube_root(x: f64, y: f64) -> bool { //funkcja jest niemalejąca więc możemy użyć funkcji przeciwnej do sprawdzenia czy punkt leży pod wykresem (będzie szybciej)
    return if y.powf(3.0) < x {
        true
    } else {
        false
    };
}

fn monte_carlo_integrate(x_start: f64, x_end: f64, max: f64, sample_number: usize, f: fn(f64, f64) -> bool) -> f64 {
    let mut rng = Pcg64Mcg::from_entropy();
    let x_range = Uniform::new_inclusive(x_start, x_end);
    let y_range = Uniform::new_inclusive(0.0, max);

    let mut count = 0;
    for u in 0..sample_number {
        let x = x_range.sample(&mut rng);
        let y = y_range.sample(&mut rng);
        if f(x, y) {
            count += 1;
        }
    }

    return (count as f64 / sample_number as f64) * (x_end - x_start) * max;
}