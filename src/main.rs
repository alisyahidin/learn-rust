use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn get_loop() {
    let mut y: u32 = 1;
    let max_loop: u32 = 10;
    loop {
        let mut x: u32 = 1;
        loop {
            if x == max_loop {
                break;
            }
            if x == y || max_loop - y == x {
                print!("*");
            } else {
                print!(" ");
            }
            x += 1;
        }
        println!("");
        if y == max_loop {
            break;
        };
        y += 1;
    }
}

fn multiply_by10(x: u32) -> u32 {
    let y: u32 = {
        let x = 50;
        x + 1
    };
    (y + x) * 10
}

fn main() {
    println!("Hello, rust!");

    let mut x: u8 = 5;
    println!("test : {{ {} }}", x);

    x = 4;

    println!("test : {{ {} }}", x);

    println!("getNumber {}", multiply_by10(10));

    get_loop();

    let secret_number = rand::thread_rng().gen_range(1..10);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
