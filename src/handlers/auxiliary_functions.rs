use rand::distributions::Alphanumeric;
use rand::Rng;
use std::iter;

pub fn clear_response(response: Vec<u8>) -> Vec<u8> {
    let string = String::from_utf8(response).unwrap();
    string.replace("\n", "").into_bytes()
}

pub fn clear_string(string: String) -> String {
    string.replace("\n", "")
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

pub fn string_to_vecu8(string: &str) -> Vec<u8> {
    string.as_bytes().to_vec()
}

pub fn parse_windows_volume(response: String) -> Result<Vec<u8>, Vec<u8>> {
    if let Some(start_idx) = response.find("Master audio level = ") {
        // Extract the substring starting from the end of the "Master audio level = " string
        let value_str = &response[start_idx + "Master audio level = ".len()..];

        // Find the position of the next newline character to determine the end of the number
        if let Some(end_idx) = value_str.find('\r') {
            // Extract the substring until the newline character
            let number_str = &value_str[..end_idx];

            return Ok(number_str.as_bytes().to_vec());
        } else {
            return Err("No newline character found after 'Master audio level = '.".into());
        }
    } else {
        return Err("'Master audio level = ' not found in the string.".into());
    }
}
