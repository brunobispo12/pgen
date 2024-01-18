use rand::rngs::ThreadRng;
use rand::Rng;
pub fn gen_password_with_symbols(password_length: u8, rng: &mut ThreadRng) -> String {
    let special_chars = b"!@#$%^&*?/";
    let mut password = String::new();

    for _ in 0..password_length {
        let idx = rng.gen_range(0..62 + special_chars.len());
        let ch = if idx < 62 {
            (if idx < 10 {
                b'0' + idx as u8
            } else if idx < 36 {
                b'a' + (idx - 10) as u8
            } else {
                b'A' + (idx - 36) as u8
            }) as char
        } else {
            special_chars[idx - 62] as char
        };

        password.push(ch);
    }

    password
}
