#[derive(Debug)]
pub struct UrlBuilder {
    /// Root URL
    root: String,
}

impl UrlBuilder {
    pub fn new(root: &str) -> Self {
        Self { root: root.to_string() }
    }

    pub fn url(&self, endpoint: &str, params: &Vec<(&str, String)>) -> String {
        format!("{}/{}?{}", &self.root, endpoint, UrlBuilder::join_params(params))
    }

    fn join_params(params: &Vec<(&str, String)>) -> String {
        if params.is_empty() {
            "".to_string()
        } else {
            params
                .iter()
                .map(|(key, val)| format!("{}={}", *key, *val))
                .collect::<Vec<String>>()
                .join("&")
        }
    }

    #[cfg(test)]
    pub fn replay_filename(&self, url: String) -> String {
        self.test_filename(url, "replay".into())
    }

    #[cfg(test)]
    pub fn expected_filename(&self, url: String) -> String {
        self.test_filename(url, "expected".into())
    }

    #[cfg(test)]
    pub fn test_filename(&self, url: String, extension: String) -> String {
        use regex::Regex;
        use crate::utils::get_dummy_api_key;

        // Chop off the root.
        let root_len = self.root.len() + 1; // include '/'
        let rootless_name = &url[root_len..];

        // Replace the token with dummy token.
        let re = Regex::new(r"token=[0-9A-Za-z]+").unwrap();
        let dummy_token = format!("token={}", get_dummy_api_key());
        let name = re.replace_all(rootless_name, dummy_token);

        // Path from root.
        format!("test/{}.{}", name, extension)
    }
}

// Tests in this file since functions are private.
#[cfg(test)]
mod test {
    use crate::url_builder::UrlBuilder;

    #[test]
    fn url() {
        let token = "abc123";
        let root = "https://finnhub.io/api/v1";
        let endpoint = "company-news";
        let params = vec![
            ("symbol", "GOOGL".into()),
            ("from", "2020-12-10".into()),
            ("to", "2021-01-10".into()),
            ("token", token.to_string()),
        ];
        let pstr = "symbol=GOOGL&from=2020-12-10&to=2021-01-10&";

        let expected = format!("{}/{}?{}token={}", root, endpoint, pstr, token);
        let builder = UrlBuilder::new(root);
        let actual = builder.url(endpoint, &params);
        assert_eq!(actual, expected);
    }

    #[test]
    fn gen_param_str() {
        let token = "abc123";
        let params = vec![
            ("symbol", "GOOGL".into()),
            ("from", "2020-12-10".into()),
            ("to", "2021-01-10".into()),
            ("token", token.to_string()),
        ];
        let expected = format!("symbol=GOOGL&from=2020-12-10&to=2021-01-10&token={}", token);
        let actual = UrlBuilder::join_params(&params);
        assert_eq!(actual, expected);
    }
}
