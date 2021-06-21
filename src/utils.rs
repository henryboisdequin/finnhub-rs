/// Acquires and api key from an environment or .env file via a variable named, TEST_API_KEY.
#[allow(dead_code)]
pub fn get_test_api_key() -> String {
    use dotenv::dotenv;
    use std::env;

    dotenv().ok();

    let key = "TEST_API_KEY";
    let test_api_key = env::var(key).expect("Key, value pair not present in .env file");

    test_api_key
}

/// A dummy api key for testing.
#[allow(dead_code)]
pub fn get_dummy_api_key() -> String {
    "abc123".into()
}

/// Replaces occurrences of api keys in file content with the dummy api key.
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

/// Replace the file extension of a replay file with that of an expected. Not very robust.
#[cfg(test)]
pub fn load_expected_from_replay_filename(replay_filename: String) -> String {
    let expected_filename = replay_filename.replace(".replay", ".expected");
    std::fs::read_to_string(expected_filename).unwrap().trim().into()
}




/// Will create a data structure from the forex conversion pairs
pub fn extract_conversion_pairs<'de, D>(deserializer: D) -> Result<std::collections::BTreeMap<String, f64>, D::Error>
    where
        D: serde::Deserializer<'de>,
{
    let value: serde_json::Value = serde::Deserialize::deserialize(deserializer)?;

    let mut re_map: std::collections::BTreeMap<String, f64> = std::collections::BTreeMap::new();

    if value.is_object() && value.get("quote").is_some() {
        // Now we can extract the quote pairs

        let quote_pairs = value.get("quote").unwrap();




        for quote_pair in quote_pairs.as_object().unwrap().iter() {

            if quote_pair.1.is_number() {
                if quote_pair.1.is_f64() {
                    re_map.insert(String::from(quote_pair.0), quote_pair.1.as_f64().unwrap() );
                }else if quote_pair.1.is_i64() {
                    re_map.insert(String::from(quote_pair.0), quote_pair.1.as_i64().unwrap() as f64 );
                }
            }




        }
    }


    return Ok(re_map);


}



