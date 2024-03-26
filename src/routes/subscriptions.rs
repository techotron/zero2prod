use actix_web::{web, HttpResponse};

// This macro automatically implements the correct serialise/deserialise implementation for us
//  In our case, it'll parse the definition of the FormData type and generate the right implementation for it
#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String
}

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
