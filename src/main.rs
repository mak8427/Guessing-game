use rand::Rng;
use std::cmp::Ordering;
use plotters::prelude::*;


fn main()-> Result<(), Box<dyn std::error::Error>>  {
    const MAX: usize =10000;
    let mut  array = vec![0; MAX];
    println!("Guess the number");
    let range = rand::thread_rng().gen_range(1..100000000);

    for n in (5..MAX).step_by(10) {
        let secret_number = rand::thread_rng().gen_range(1..n);
        let mut guess = range / 2;
        let mut variator = range / 2;
        let mut times = 0;
        loop {
            match guess.cmp(&secret_number) {
                Ordering::Less => {
                    guess = guess + variator;
                    variator = variator / 2;
                    if variator == 0 {
                        variator = 1
                    }
                    times = times + 1
                }
                Ordering::Greater => {
                    guess = guess - variator;
                    variator = variator / 2;
                    if variator == 0 {
                        variator = 1
                    }
                    times = times + 1
                }
                Ordering::Equal => {
                    break;
                }
            }
        }
        array[n]=times;
        println!("your guess {} the right one {}, times done are {}, variator {}, range is {}", guess, secret_number, times, variator, n);
    }

    let root = BitMapBackend::new("../rust_2/0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            &RED,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())






}
