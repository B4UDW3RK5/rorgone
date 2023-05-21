use rand::Rng;

fn generate_orgone() -> &'static str {
   let orgone = &["█","▌","▐","▖","▗","▘","▙","▚","▛","▜","▝","▞","▟"];
   let mut rng = rand::thread_rng();
   let random_index = rng.gen_range(0..orgone.len());
   orgone[random_index]
}

fn main() {
   loop {
      let character = generate_orgone();
      print!("{}", character);
   }
}