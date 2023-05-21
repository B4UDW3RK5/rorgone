use rand::Rng;
fn main() {
   let characters = ["█","▌","▐","▖","▗","▘","▙","▚","▛","▜","▝","▞","▟"];
   let mut rng = rand::thread_rng();
   for _ in 0..9 {
      let random_index = rng.gen_range(0..characters.len());
      print!("{}", characters[random_index]);
   }
   println!();
}