use axum::{
    extract::{Request, Multipart},
    http::StatusCode
};
use tokio::fs;
use std::collections::HashMap;

pub struct FormWithImage {
    pub fields: HashMap<String, String>,
    pub image_path: Option<String>,
}

#[axum::async_trait]
impl<S>axum::extract::FromRequest<S> for FormWithImage
where
    S: Send + Sync,
    {
        type Rejection = (StatusCode, String);

        async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection>
        {
            let mut multipart = Multipart::from_request(req, state).await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

            let mut fields = HashMap::new();
            let mut image_path = None;

            while let Some(field) = 
            multipart.next_field().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?{
                let name = field.name().unwrap_or("").to_string();

                if name == "image" {
                    let data = field.bytes().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

                    let filename = format!("{}.jpg", uuid::Uuid::new_v4());
                    let path = format!("uploads/items/{}", filename);

                    fs::create_dir_all("uploads/items").await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                    fs::write(&path, data).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                    image_path = Some(format!("/uploads/items/{}", filename));
                } else {
                    let value = field.text().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
                    fields.insert(name, value);
                }
            }

            Ok(FormWithImage { fields, image_path })
        }
    }