use rand::Rng;
use std::cmp::Ordering;
use std::io;

mod holla;
mod routes;

fn fib(num: i32) -> i32 {
    match num {
        0 => 0,
        1 => 1,
        2 => 1,
        _ => fib(num - 1) + fib(num - 2),
    }
}

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

fn understand_ownership() {
    let s1 = String::from("Test"); // string store in heap, NOT in STACK
    let s2 = s1.clone(); // clone the value, because value has stored in HEAP cannot have multiple owner a.k.a variable
    println!("string in heap {} {}", s1, s2);

    let x = "test";
    let _y = x; // it's coppied because data stored in STACK
    println!("{}", x);

    let s = String::from("test");
    take_ownership(s);
    // s variable is no longer valid here

    let t: u32 = 10;
    copy_value(t);
    // println!("{}", t); // t is still valid because function only copy the value from params
}

fn take_ownership(a: String) {
    println!("{}", a); // value in parameter a will be dropped
}

fn copy_value(a: u32) {
    println!("{}", a);
}

fn references_borrowing() {
    let mut s = String::from("Holla");
    change(&mut s);
    println!("borrowing {}", s);
}

fn change(s: &mut String) {
    s.push_str(" world!");
}

struct Address(f32, f32, f32);
struct User {
    name: String,
    email: String,
    age: u32,
    address: Address,
}

impl User {
    // method
    fn get_address(&self) -> f32 {
        self.address.2
    }

    // associated function
    fn get_random() -> u32 {
        rand::thread_rng().gen_range(1..10)
    }
}

fn get_struct() {
    let user = User {
        name: String::from("ali"),
        email: String::from("ali@mail.com"),
        age: 217,
        address: Address(3.4, 3.5, 4.5),
    };

    println!(
        "{} {} {} {} {} rand {}",
        user.name,
        user.email,
        user.age,
        user.address.0,
        user.get_address(),
        User::get_random(),
    );
}

// enum IPAddress {
//     Ipv4,
//     Ipv6,
// }

fn get_enum() {
    let address: Option<String> = Some(String::from("Test"));
    // let ip4 = IPAddress::Ipv4;
    println!("{}", address.unwrap());
}

fn main() {
    println!("Hello, rust!");

    let mut x: u8 = 5;
    println!("test : {{ {} }}", x);

    x = 4;

    println!("test : {{ {} }}", x);

    println!("fibonaci 40: {}", fib(40));

    get_loop();

    understand_ownership();

    references_borrowing();

    get_struct();

    get_enum();

    holla::get_message("Ali");

    routes::user::get_index();

    println!("getNumber {}", multiply_by10(10));

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
