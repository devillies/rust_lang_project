use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
  println!("guess the number\n" );
  let secret_number = rand::thread_rng().gen_range(1,101);
  loop{
  println!("input a number");
  let mut guess = String::new();
  io::stdin().read_line(&mut guess).expect("failed to readline");
   let guess: u32 = guess.trim().parse().expect("Please type a number!");
println!("you guess {}\n",guess);
match guess.cmp(&secret_number){
    Ordering::Greater => println!("too big!\n"),
    Ordering::Less => println!("too small!\n"),
    Ordering::Equal => {
      println!("You win!\n");
      break;
    },

}
}
}

