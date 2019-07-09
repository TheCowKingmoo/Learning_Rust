mod roll;
mod parse;
mod user_input;

const NUM_SIDE_IDX: i32 = 0;
const NUM_ROLL_IDX: i32 = 1;
const BAD_RETURN: i32 = -1;

  fn main() {

      let mut input_return = user_input::user_input();
      let return_vec = parse::string_to_vec_int( input_return );

      if return_vec.is_empty() {
          println!( "parsing failed\n");
          return;

      }

      let dice_array = vec![0; return_vec[NUM_ROLL_IDX as usize] as usize];

      let mut dice_sum = 0;
      for dice in 0..dice_array.iter().len() {
          let r_return = roll::input_roll( return_vec[NUM_SIDE_IDX as usize]);
          dice_sum = dice_sum + r_return;
          println!("roll {} = {}", dice, r_return);
      }
      print!( "Total is  {} \n", dice_sum );

  }  //end main
