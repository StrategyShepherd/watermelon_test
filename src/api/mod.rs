use std::net::TcpListener;
use std::ops::Deref;
use std::time::Duration;

use actix_web::dev::Server;
use actix_web::middleware::{Compress, Logger, NormalizePath};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web, delete, post};
use actix_web::error::ParseError::Utf8;
use actix_web::web::{get, Query};
use crate::database::create_link;
use crate::requests::{CreateUrlRequest, DeleteUrlRequest};
use crate::state::State;
use crate::utility::Utility;

fn api_config(_cfg: &mut web::ServiceConfig) {
    todo!()
}

async fn not_found_handler(_request: HttpRequest) -> HttpResponse {
    HttpResponse::NotFound().json(serde_json::json!({ "error": "Not found" }))
}

#[post("/urls/{url}/{expirationTime}")]
async fn create_url(db_client : web::Data<State>, path : web::Path<(String)>, query_params : web::Query<CreateUrlRequest>) -> HttpResponse {
    let (url,expire_time) = path.into_inner();
    if (Utility::is_valid_url(url)) {
        /// return invalid response
    }
    
    let generated_alias = Utility::generate_alias(url.parse().unwrap());
    let mut request = CreateUrlRequest::new(&generated_alias, expire_time);
    request.set_custom_alias(Some((&generated_alias).to_string()));
    create_link(db_client, url).await?
    return HttpResponse::Ok().json(generated_alias);
    // call to DB
    /// check alias id///
    /// start transaction///
    ///
}
#[delete("/urls/{url}/")]
async fn delete_url(path : web::Path<String>) -> HttpResponse {
    // let utils = Utility::generate_alias()
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
            .service(web::scope("/api").configure(api_config))
            .default_service(web::route().to(not_found_handler))
    };
    let server = HttpServer::new(create_app)
        .keep_alive(Duration::from_secs(60))
        .listen(listener)?
        .run();
    Ok(server)
}
