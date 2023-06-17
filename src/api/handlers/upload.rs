use std::io::Write;

use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use futures::{StreamExt, TryStreamExt};

use crate::{error::AppError, utils::response::new_ok_response};

pub async fn save_file(mut payload: Multipart) -> Result<HttpResponse, AppError> {
    if let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition();
        let filename = content_type.get_filename().unwrap();
        let file_path = format!("static/{}", filename);

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(file_path))
            .await
            .unwrap()
            .unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f))
                .await
                .unwrap()
                .unwrap();
        }
    }
    Ok(HttpResponse::Ok().json(new_ok_response("上传成功")))
}
