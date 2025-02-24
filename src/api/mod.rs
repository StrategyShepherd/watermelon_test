use std::net::TcpListener;
use std::time::Duration;

use crate::database::{create_link, delete_link, get_link};
use crate::requests::CreateUrlRequest;
use crate::state::State;
use crate::utility::Utility;
use actix_web::dev::Server;
use actix_web::middleware::{Compress, Logger, NormalizePath};
use actix_web::{delete, web, App, HttpRequest, HttpResponse, HttpServer};

fn api_config(_cfg: &mut web::ServiceConfig) {
    todo!()
}

async fn not_found_handler(_request: HttpRequest) -> HttpResponse {
    HttpResponse::NotFound().json(serde_json::json!({ "error": "Not found" }))
}

async fn create_url(app_state : web::Data<State>, path : web::Path<(String)>, query_params : web::Query<CreateUrlRequest>) -> HttpResponse {
    let mut client = app_state.get_ref().clone().database_client().await.unwrap();
    let (url,expire_time) = path.into_inner();
    if (Utility::is_valid_url(url)) {
        /// return invalid response
    }
    
    let mut generated_alias = Utility::generate_alias(url.parse().unwrap());
    while(!app_state.find_alias(&generated_alias)) {
        generated_alias = Utility::generate_alias(url.parse().unwrap());
    }
    let mut request = CreateUrlRequest::new(&generated_alias, expire_time);
    /// check with previous alises 
    get_link(&mut client, &generated_alias);
    request.set_custom_alias(Some((&generated_alias).to_string()));
    match create_link(&client, url).await {
        Ok(link) => HttpResponse::Created().json(serde_json::json!({ "link": link })),
        Ok(..) => HttpResponse::NotFound().json(serde_json::json!({ "error": "Not found" })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": "Internal server error" })),
        _ => {}
    }

    HttpResponse::Ok().json(generated_alias)
}
#[delete("/urls/{aliasId}/")]
async fn delete_url(path : web::Path<String>, state :web::Data<State>) -> HttpResponse {
    let mut client = state.get_ref().clone().database_client().await.unwrap();
    let alias_id = path.into_inner();
    delete_link(&client, &*alias_id);
    /// what happens if you try to delete something that not exists from a DB perspective, you should still get a 204 roughly
    HttpResponse::NoContent().json(serde_json::json!({ "error": "Not found" }))
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
            .route("/{url}/{expirationTime}", web::post().to(create_url))
            .service(delete_url)
            .default_service(web::route().to(not_found_handler))
    };
    let server = HttpServer::new(create_app)
        .keep_alive(Duration::from_secs(60))
        .listen(listener)?
        .run();
    Ok(server)
}
