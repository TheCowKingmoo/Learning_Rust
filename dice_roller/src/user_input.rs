use std::io;

  pub fn user_input() -> String {

    let mut input = String::new();
    println!( "Usage: side on dice, num of times to roll EX: 6, 10 --> rolls 10 six sided dice\n");
    println!( "num of sides, num of times to roll");

    io::stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line\n");

    return input;

  }