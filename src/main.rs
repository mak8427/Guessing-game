use rand::Rng;
use std::cmp::Ordering;
use plotters::prelude::*;


fn sme(x:i32, arr:&Vec<i32>)->Vec<i32> {
    let mut i=0;
    let len_arr= arr.len();
    let mut avg_arr = vec![0;len_arr-(x as usize)];
    let mut sum=0;
    while i<len_arr-(x as usize){
        for h in i..i+(x as usize){
            sum=sum+arr[h];
        }
    avg_arr[i]=sum/x;
    sum=0;
    i=i+1;
    }
    return avg_arr;
}





fn main()-> Result<(), Box<dyn std::error::Error>>{
    const MAX: usize =1000000000;
    const STEP: usize = 100000;
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

    let avg=sme(100,&array);

    let root_area = BitMapBackend::new("../Guessing-game/0.png", (640, 480)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Line Plot Demo", ("sans-serif", 40))
        .build_cartesian_2d(0..MAX, 0..50)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        LineSeries::new((0..).step_by(STEP).zip(array.iter().map(|x|*x)), &GREEN)
    );
    ctx.draw_series(
            LineSeries::new((0..).step_by(STEP).zip(avg.iter().map(|x|*x)), &BLUE)
    );
        Ok(())




}
