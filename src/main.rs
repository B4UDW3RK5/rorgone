use std::env;
use rand::Rng;

// generate a single orgone unit
fn generate_orgone() -> &'static str {
   let orgone = &["█","▌","▐","▖","▗","▘","▙","▚","▛","▜","▝","▞","▟"];
   let mut rng = rand::thread_rng();
   let random_index = rng.gen_range(0..orgone.len());
   orgone[random_index]
}

// if a quantity is provided, print that amount of orgone, otherwise print
// orgone forever
fn main() {
   let args: Vec<String> = env::args().collect();
   let quantity: usize = if args.len() > 1 {
      args[1].parse().unwrap_or(0)
   } else {
      0
   };

   if quantity > 0 {
      for _ in 0..quantity {
         let energy = generate_orgone();
         print!("{}", energy);
      }
   } else {
      loop {
         let energy = generate_orgone();
         print!("{}", energy);
      }
   }
}