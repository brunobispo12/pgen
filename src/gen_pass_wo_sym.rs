use rand::rngs::ThreadRng;
use rand::{distributions::Alphanumeric, Rng};

pub fn gen_password_without_symbols(password_length: u8, rng: &mut ThreadRng) -> String {
    let password: String = {
        (0..password_length)
            .map(|_| rng.sample(Alphanumeric) as char)
            .collect()
    };

    password
}
