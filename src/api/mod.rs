use std::net::TcpListener;
use std::time::Duration;

use crate::database::{create_link, delete_link};
use crate::requests::CreateUrlRequest;
use crate::state::State;
use crate::utility::Utility;
use actix_web::dev::Server;
use actix_web::middleware::{Compress, Logger, NormalizePath};
use actix_web::{delete, web, App, HttpRequest, HttpResponse, HttpServer};
use serde_json::json;

async fn not_found_handler(_request: HttpRequest) -> HttpResponse {
    HttpResponse::NotFound().json(json!({ "error": "Not found" }))
}

async fn create_url(req: HttpRequest, app_state : web::Data<State>, path : web::Path<String>) -> HttpResponse {
    let client = app_state.get_ref().clone().database_client().await.unwrap();
    let url = path.into_inner();
    if Utility::is_valid_url(&*url) && Utility::is_over_accepted_url_length(&*url) {
        return HttpResponse::BadRequest().json(json!({ "error": "Invalid URL" }))
    }
    if Utility::is_over_accepted_url_length(&*url) {
        return HttpResponse::BadRequest().json(json!({ "error": "URL over accepted length" }))
    }
    let mut generated_alias = Utility::generate_alias(url.parse().unwrap());
    while !app_state.find_alias(&generated_alias) {
        generated_alias = Utility::generate_alias(url.parse().unwrap());
    }
    let mut request = CreateUrlRequest::new(&generated_alias, 0);
    let generated_result = [req.full_url().domain().unwrap().to_string(), String::from('/'), generated_alias].join("");
    request.set_custom_alias(Some((&generated_result).to_string()));
    match create_link(&client, &*url).await {
        Ok(generated_alias) => HttpResponse::TemporaryRedirect().json(json!({ "link": generated_result })),
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({ "error": "Internal server error", "details": e.to_string() }))
        }
    }
}

#[delete("/urls/{aliasId}")]
async fn delete_url(path : web::Path<String>, state :web::Data<State>) -> HttpResponse {
    let client = state.get_ref().clone().database_client().await.unwrap();
    let alias_id = path.into_inner();
    match delete_link(&client, &*alias_id).await {
    Ok(_) => HttpResponse::NoContent().finish(),
    Err(e) => HttpResponse::NotFound().json(serde_json::json!({ "error": "Not found" }))}

}
fn api_config(_cfg: &mut web::ServiceConfig) {
        _cfg.service(delete_url)
            .route("urls/createUrl/{url}", web::post().to(create_url))
            .service(delete_url);
    }

    pub fn listen(listener: TcpListener, state: State) -> std::io::Result<Server> {
    let state = web::Data::new(state);
    let create_app = move || {
        let app = App::new().app_data(state.clone());
        app
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(Logger::new(r#"%a "%r" %s %b (%{Content-Length}i %{Content-Type}i) "%{Referer}i" "%{User-Agent}i" %T"#))
            .wrap(Compress::default())
            .wrap(NormalizePath::trim())
            .service(web::scope("/api")
                 .configure(api_config))
          
            .default_service(web::route().to(not_found_handler))
    };
    let server = HttpServer::new(create_app)
        .keep_alive(Duration::from_secs(60))
        .listen(listener)?
        .run();
    Ok(server)
}
    
    
