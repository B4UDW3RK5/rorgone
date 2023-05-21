use rand::Rng;
fn main() {
   let characters = ["█","▌","▐","▖","▗","▘","▙","▚","▛","▜","▝","▞","▟"];
   let mut rng = rand::thread_rng();
   loop {
      let random_index = rng.gen_range(0..characters.len());
      print!("{}", characters[random_index]);
   }
}