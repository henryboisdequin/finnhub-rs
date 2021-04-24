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
pub fn url_to_replay_name(url: String, url_root: String) -> String {
    use regex::Regex;

    // Chop off the root.
    let root_len = url_root.len() + 1;
    let rootless_name = &url[root_len..];

    // Replace the token with dummy token.
    let re = Regex::new(r"token=[0-9A-Za-z]+").unwrap();
    let dummy_token = format!("token={}", get_dummy_api_key());
    let name = re.replace_all(rootless_name, dummy_token);

    // Path from root.
    format!("test/{}.replay", name)
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