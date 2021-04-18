#[derive(Debug)]
pub struct UrlBuilder {
    /// Root URL
    root: String,
}

impl UrlBuilder {
    pub fn new(root: &str) -> Self {
        Self { root: root.to_string() }
    }

    pub fn url(&self, endpoint: &str, params: &Vec<(&str, &str)>) -> String {
        format!("{}/{}?{}", &self.root, endpoint, UrlBuilder::join_params(params))
    }

    fn join_params(params: &Vec<(&str, &str)>) -> String {
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
            ("symbol", "GOOGL"),
            ("from", "2020-12-10"),
            ("to", "2021-01-10"),
            ("token", token),
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
            ("symbol", "GOOGL"),
            ("from", "2020-12-10"),
            ("to", "2021-01-10"),
            ("token", token),
        ];
        let expected = format!("symbol=GOOGL&from=2020-12-10&to=2021-01-10&token={}", token);
        let actual = UrlBuilder::join_params(&params);
        assert_eq!(actual, expected);
    }
}
