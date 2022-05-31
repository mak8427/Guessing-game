use rand::Rng;
use std::cmp::Ordering;
use plotters::prelude::*;


fn main() {
    const MAX: usize =1000000;
    const STEP: usize = 1;
    let mut  array = vec![0; MAX/STEP];
    println!("Guess the number");
    let mut i=0;
    for n in (5..MAX).step_by(STEP) {
        let secret_number = rand::thread_rng().gen_range(1..n);
        let mut guess = n / 2;
        let mut variator = n / 2;
        let mut times = 0;
        loop {
            match guess.cmp(&secret_number) {
                Ordering::Less => {
                    if variator == 0 {
                        variator = 1
                    }
                    guess = guess + variator;
                    variator = variator / 2;
                    times = times + 1
                }
                Ordering::Greater => {
                    if variator == 0 {
                        variator = 1
                    }
                    guess = guess - variator;
                    variator = variator / 2;
                    times = times + 1
                }
                Ordering::Equal => {
                    break;
                }
            }
        }
        array[i]=times;
        println!("your guess {} the right one {}, times done are {}, variator {}, range is {}", guess, secret_number, times, variator, n);
        i=i+1;
    }
    let root_area = BitMapBackend::new("../Guessing-game/0.png", (640, 480)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Line Plot Demo", ("sans-serif", 40))
        .build_cartesian_2d(0..MAX, 0..100)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        LineSeries::new((0..).step_by(STEP).zip(array.iter().map(|x|*x)), &GREEN)
    ).unwrap();





}
