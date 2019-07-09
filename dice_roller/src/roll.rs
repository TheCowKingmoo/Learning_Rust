extern crate rand;

  use rand::Rng;

  const DICE_START: i32 = 1;

  pub fn input_roll( num_of_sides: i32 ) -> i32  {

    let mut rng = rand::thread_rng();
    let roll = rng.gen_range( DICE_START, num_of_sides + 1 );
    return roll;

  }  //end roll