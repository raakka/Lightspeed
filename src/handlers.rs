#[path = "models.rs"]
mod models;

pub mod handlers {
    use crate::handlers::models::models::Somestruct;
    use actix_web::{
        web,
        Error,
        HttpResponse
    };

    pub async fn redisfn(
        jsonreq: web::Json<Somestruct>
    ) -> Result<HttpResponse, Error> {

        // I LIKE ALL MY STUFF JSON SO WE GOTTA UNWRAP
        let in_json: Somestruct = jsonreq.into_inner();    

        // DO SOME STUFF
        // WHY???
        Ok(HttpResponse::Ok().json(in_json))
    }
}
