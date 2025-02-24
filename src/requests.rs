struct CreateUrlRequest {
    url: String,
    expiration_time: u32,
    custom_alias : Option<String>,
}

struct DeleteUrlRequest {
    custom_alias: String
}