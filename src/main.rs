use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");
    let range = rand::thread_rng().gen_range(1..1000000);
    let secret_number=rand::thread_rng().gen_range(1..range);
    let mut guess= range/2;
    let mut variator =range/2;
    let mut times=0;
    loop{
        println!("input your guess");
        match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too small");
            guess=guess+variator;
            variator=variator/2+1;
            times=times+1
        },
        Ordering::Greater => {
            println!("Too big");
            guess=guess-variator;
            variator=variator/2+1;
            times=times+1
        },
        Ordering::Equal => {
            println!("You win");
            break;
        },
        }
        println!("your guess {} the right one {}, times done are {} ", guess, secret_number, times);

    }

}
