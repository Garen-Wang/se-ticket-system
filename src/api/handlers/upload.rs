use std::io::Write;

use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use base64::Engine;
use futures::{StreamExt, TryStreamExt};

use crate::{
    api::{request::upload::UploadFileV2Request, response::upload::UploadFileV2Response},
    error::{new_ok_error, AppError},
    utils::{constant::IMAGE_URL_PREFIX, response::CommonResponse},
};

pub async fn save_file(mut payload: Multipart) -> Result<HttpResponse, AppError> {
    if let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition();
        let filename = content_type.get_filename().unwrap();
        let file_path = format!("static/{}", filename);
        let resp = UploadFileV2Response {
            url: format!("{}/{}", IMAGE_URL_PREFIX, file_path),
        };

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
        Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
    } else {
        Err(new_ok_error("上传图片失败"))
    }
}

pub async fn save_file_v2(form: web::Json<UploadFileV2Request>) -> Result<HttpResponse, AppError> {
    let file_path = format!("static/{}", form.name);
    let content = base64::engine::GeneralPurpose::new(
        &base64::alphabet::URL_SAFE,
        base64::engine::general_purpose::NO_PAD,
    )
    .decode(&form.file)
    .unwrap();

    let mut f = web::block(|| std::fs::File::create(file_path))
        .await
        .unwrap()
        .unwrap();

    web::block(move || f.write_all(&content).map(|_| f))
        .await
        .unwrap()
        .unwrap();
    let resp = UploadFileV2Response {
        url: format!("{}/static/{}", IMAGE_URL_PREFIX, form.name),
    };
    Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
}
