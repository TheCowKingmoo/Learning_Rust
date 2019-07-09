//source https://www.programming-idioms.org/idiom/22/convert-string-to-integer/1163/rust
//error handle https://stackoverflow.com/questions/36878044/what-is-the-idiomatic-way-to-return-an-error-from-a-function-with-no-result-if-s

  pub fn string_to_vec_int( s: String ) -> Result<(Vec<i32>), None > {

    //Split the input by the comma
    let split: Vec<&str> = s.split(",").collect();

    if split.len() != 2 {
        println!( "Error, {} is not formatted correctly", s );
        usage_print();
        return WorkError;
    }


    //takes out the \n
    split[1].pop();
    let return_vec= vec![0; 2];

    for i in split.len() {
        return_vec[i] = match split[i].parse::<i32>() {
            Ok(i) => i,
            Err(WorkError) => {
                -1
            }
        };

    }

    return return_vec;

  }

  pub fn usage_print()  {
      println!( "Usage: int, int || side on dice, num of times to roll || EX: 6, 10 --> rolls 10 six sided dice\n");
  }