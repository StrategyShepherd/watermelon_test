


struct Utility {}

impl Utility {

    fn is_valid_url(url: &str) -> bool {

    }
    fn generate_alias(url : String) -> String {
     let hashed_url = UrlHash::from(url);
     hashed_url.to_string();
    }

}