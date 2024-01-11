use std::iter;
use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn clear_response(response: Vec<u8>) -> Vec<u8> {
    let string = String::from_utf8(response).unwrap();
    string.replace("\n", "").into_bytes()
}

pub fn generate_random_code() -> String {
    let mut rng = rand::thread_rng();
    let code = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(4)
        .collect::<String>()
        .to_uppercase();
    code
}