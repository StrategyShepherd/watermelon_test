
#[derive(Serialize)]
struct CreateUrlResponse{
    short_alias: String,
}

#[derive(Serialize)]
struct DeleteUrlResponse{
    message: String,
}

#[derive(Serialize)]
struct InvalidUrlResponse{
    message: String,
}