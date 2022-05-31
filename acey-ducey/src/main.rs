use rand::Rng;
use std::io::stdin;

fn swap(x: u8, y: u8) -> (u8, u8) {
    if x < y {
        (x, y)
    } else {
        (y, x)
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut balance = 100;

    loop {
        if balance <= 0 {
            println!("You are out of money. Booo...");
        }

        let mut card_a: u8 = rng.gen_range(1..13);
        let mut card_b: u8 = rng.gen_range(1..13);

        (card_a, card_b) = swap(card_a, card_b);
        println!("Dealer deals {} and {}", card_a, card_b);

        // print!("> ");
        let mut option = String::new();
        stdin()
            .read_line(&mut option)
            .expect("Error reading input.");
        match &option[..] {
            "1\n" => {
                let next: u8 = rng.gen_range(1..13);
                println!("Next card is... {}", next);
                if card_a < next && next < card_b {
                    println!("You won!");
                    println!("Balance += $50");
                    balance += 50;
                } else {
                    println!("You lost!");
                    println!("Balance -= $25");
                    balance -= 25;
                }

                println!("Your balance: ${}", balance);
            }
            "2\n" => {
                println!("Skipping...");
                println!("Your balance: ${}", balance);
            }
            "q\n" => {
                println!("Your balance: ${}", balance);
                println!("Goodbye!");
                break;
            }
            _ => (),
        }
    }
}
