#[cfg(test)]
pub fn get_test_api_key() -> String {
    use dotenv::dotenv;
    use std::env;

    dotenv().ok();

    let key = "TEST_API_KEY";
    let test_api_key = env::var(key).expect("Key, value pair not present in .env file");

    test_api_key
}
