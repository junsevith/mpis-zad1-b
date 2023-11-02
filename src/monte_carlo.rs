use plotters::prelude::*;
use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};
use rand_pcg::Pcg64Mcg;

//ile razy wykonywany jest algorytm dla danej liczby próbek
const COUNT: usize = 50;

pub(crate) struct MonteCarlo {
    rng: Pcg64Mcg,
    x_range: Uniform<f64>,
    y_range: Uniform<f64>,

    function: fn(f64, f64) -> bool,
    operation: f64,
}

impl MonteCarlo {
    pub(crate) fn new(x_start: f64, x_end: f64, sup: f64, f: fn(f64, f64) -> bool) -> Self {
        let rng = Pcg64Mcg::from_entropy();
        let x_range = Uniform::new_inclusive(x_start, x_end);
        let y_range = Uniform::new_inclusive(0.0, sup);
        let result = (x_end - x_start) * sup;

        return MonteCarlo {
            rng,
            x_range,
            y_range,
            function: f,
            operation: result,
        };
    }

    fn integrate(&mut self, sample_number: usize) -> f64 {
        let mut count = 0;
        for _u in 0..sample_number {
            let x = self.x_range.sample(&mut self.rng);
            let y = self.y_range.sample(&mut self.rng);
            if (self.function)(x, y) {
                count += 1;
            }
        }

        return (count as f64 / sample_number as f64) * self.operation;
    }


    pub(crate) fn get_data(&mut self, sample_number: usize) -> [(f64, f64); COUNT] {
        let mut data = [(sample_number as f64, 0.0); COUNT];
        for i in 0..50 {
            data[i].1 = self.integrate(sample_number);
        }
        return data;
    }

    pub(crate) fn draw_chart(&mut self, filename: &str, integral_val: f64, range: f64) {
        let xrange = 50f64..5000f64;
        let yrange = (integral_val - range)..(integral_val + range);
        let intrange = (50..=5000).step_by(50);
        let file = format!("{}.png", filename);


        let drawing_area = BitMapBackend::new(&file, (1024, 768))
            .into_drawing_area();

        drawing_area.fill(&WHITE).unwrap();

        let mut ctx = ChartBuilder::on(&drawing_area)
            .caption("Przybliżona wartość całki", ("Arial", 30))
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(xrange.clone(), yrange.clone())
            .unwrap();

        ctx.configure_mesh().draw().unwrap();



        for i in intrange.clone() {
            ctx.draw_series(self.get_data(i).iter().map(|point| Circle::new(*point, 2, Into::<ShapeStyle>::into(&RED).filled()).into_dyn()))
                .unwrap();
        }

        ctx.draw_series(
            LineSeries::new(intrange.clone().map(|x| (x as f64, integral_val)), &BLUE),
        ).unwrap();
    }
}


