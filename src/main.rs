use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Welcome!");
    //lets generate a random variable
    let secret_number:u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("please enter a guess number");
    //lets create a variable to store the input
    let mut guess: String=String::new();
    //append the input in guess variable
    io::stdin().read_line(&mut guess).expect("Operating system problem oocured");
    //parse user guess input 
    let guess: u32= match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    println!("You guessed: {guess}");

    //let's conpare and decide if won or not
    match guess.cmp(&secret_number) {
        Ordering::Equal => {
            println!("You win!");
            break;
        },
        Ordering::Greater => println!("Number is greater"),
        Ordering::Less => println!("Number is smaller"),
    }
    }
}