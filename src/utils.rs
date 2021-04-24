#[allow(dead_code)]
pub fn get_test_api_key() -> String {
    use dotenv::dotenv;
    use std::env;

    dotenv().ok();

    let key = "TEST_API_KEY";
    let test_api_key = env::var(key).expect("Key, value pair not present in .env file");

    test_api_key
}

#[allow(dead_code)]
pub fn get_dummy_api_key() -> String {
    "abc123".into()
}

#[cfg(test)]
pub fn clean_key_from_file(filename: String, key: String) {
    // from: https://stackoverflow.com/questions/27215396/how-to-replace-a-word-in-a-file-in-a-txt
    // Read file.
    use std::io::{Read, Write};
    let mut src = std::fs::File::open(filename.clone()).unwrap();
    let mut data = String::new();
    src.read_to_string(&mut data).unwrap();
    drop(src);

    // Make replacement.
    let real_token = format!("token={}", key);
    let dummy_token = format!("token={}", get_dummy_api_key());
    let new_data = data.replace(&real_token, dummy_token.as_str());

    // Rewrite file.
    let mut dst = std::fs::File::create(filename).unwrap();
    dst.write(new_data.as_bytes()).unwrap();
}

#[cfg(test)]
pub fn load_expected_from_replay_filename(replay_filename: String) -> String {
    let expected_filename = replay_filename.replace(".replay", ".expected");
    std::fs::read_to_string(expected_filename).unwrap().trim().into()
}