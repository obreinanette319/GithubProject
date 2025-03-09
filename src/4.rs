use rand::prelude::*;

fn generate_random_code() -> String {
    let mut rng = rand::thread_rng();
    let characters = ["a", "b", "c", "d", "e", "f"];
    let code: String = (0..10).map(|_| characters.choose(&mut rng).unwrap().to_string()).collect();
    return code;
}
