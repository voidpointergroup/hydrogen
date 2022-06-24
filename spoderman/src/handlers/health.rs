use actix_web::{
    get,
    Error,
    HttpResponse,
};

#[derive(serde::Serialize)]
struct Health {
    status: String,
}

#[get("/health")]
pub async fn handler() -> Result<HttpResponse, Error> {
    Ok(actix_web::HttpResponse::Ok().json(Health {
        status: "up".to_owned(),
    }))
}
