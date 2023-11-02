use std::f64::consts::PI;

mod functions;
mod monte_carlo;



fn main() {
    let mut ichi = monte_carlo::MonteCarlo::new(0.0, 8.0, 2.0, functions::cube_root);
    ichi.draw_chart("cuberoot", 12f64, 2f64);

    let mut ni = monte_carlo::MonteCarlo::new(0.0, PI, 1.0, functions::sinus);
    ni.draw_chart("sinus", 2f64, 0.5);

    let mut san = monte_carlo::MonteCarlo::new(0.0, 1.0, 1.0, functions::wielomian);
    san.draw_chart("wielomian", 0.2, 0.1);

    let mut shin = monte_carlo::MonteCarlo::new(-1.0, 1.0, 2.0, functions::circle);
    shin.draw_chart("koło", PI, 0.5);
}

