use actix_multipart::Multipart;
use actix_web::{Error, HttpResponse};

async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename));

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
        }
    }
    Ok(HttpResponse::Ok().into())
}