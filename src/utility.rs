use url::{Url};
use url_hash::UrlHash;

const URL_LENGTH: i32 = 200;
pub struct Utility {}

impl Utility {
    pub fn new() -> Self {
        Utility {}
    }
    pub fn is_valid_url(url: &str) -> bool {
        let url = Url::parse(url);
        if url.is_err() {
            return false;
        }
        true
    }
    pub fn is_over_accepted_url_length(url : &str) -> bool {
        let url = Url::parse(url);
        if url.unwrap().as_str().len() as i32 > URL_LENGTH {
            return true;
        }
        false
    }
    
    pub fn generate_alias(url : String) -> String {
     let hashed_url = UrlHash::from(Url::from(url.parse().unwrap()));
     hashed_url.to_string()
    }

}