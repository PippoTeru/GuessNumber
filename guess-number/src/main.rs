use rand::Rng;
use std::io;
use std::io::Write;

fn get_input() -> String {
    let mut word = String::new();
    io::stdin().read_line(&mut word).ok();
    return word.trim().to_string();
}

fn main() {
    let num = rand::thread_rng().gen_range(1..100);

    println!("***Guess the number!***");

    loop {
        print!("\nPlease input your guess. >>>");
        io::stdout().flush().unwrap();

        let num_input: i32 = get_input().parse().unwrap();

        if num_input > num {
            println!("Your guess is bigger.");
        } else if num_input < num {
            println!("Your guess is lower.");
        } else {
            break;
        }
    }
    println!("\n*** Your guess is correct! ***");
    println!("***        You win!        ***")
}
