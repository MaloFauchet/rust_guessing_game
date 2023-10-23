use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess. ");

    let number: i8 = 54;

    main_loop(number);
}

fn main_loop(number: i8) {
    let mut guess:  String= String::new();

    while true {
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line, retry.");

        println!("\nYou guessed: {guess}");
        check_num(guess, number);
    }
}

fn check_num(guess: i8, number: i8) {
    if guess == number {
        println!("You win!");
    } else if guess < number {
        println!("Higher");
    } else {
        println!("Lower");
    }
}