use serde::Serialize;

#[derive(Serialize)]
pub struct CreateUrlRequest {
    url: String,
    expiration_time: u32,
    custom_alias : Option<String>,
}
impl CreateUrlRequest {
    pub fn new(url_link: &String, expiration_time: u32) -> CreateUrlRequest {
    CreateUrlRequest{
        url: url_link.to_string(),
        expiration_time,
        custom_alias: None,
    } }
    pub fn set_expiration_time(&mut self, expiration_time: u32) {
        self.expiration_time = expiration_time;
    }
    pub fn set_custom_alias(&mut self, custom_alias: Option<String>) {
        match custom_alias {
            Some(custom_alias) => {self.custom_alias = Option::from(custom_alias);}
            None => {self.custom_alias = None}
        }
        
    }
        
}

pub struct DeleteUrlRequest {
    custom_alias: String
}