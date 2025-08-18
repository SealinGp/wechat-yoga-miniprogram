use rocket::data::ToByteUnit;
use rocket::http::Status;
use rocket::{get, post, State};
use rocket::form::{Form, FromForm};
use rocket::fs::{TempFile, NamedFile};
use serde_json::json;
use std::fs;
use std::path::Path;
use uuid::Uuid;
use crate::models::settings::Settings;

#[derive(FromForm)]
pub struct Upload<'f> {
    file: TempFile<'f>,
}

#[post("/api/upload", data = "<upload>")]
pub async fn upload_file(
    mut upload: Form<Upload<'_>>,
    settings: &State<Settings>,
) -> Result<String, Status> {
    let file = &mut upload.file;
    
    // Check if file has a content type
    let content_type = match file.content_type() {
        Some(ct) => ct,
        None => {
            return Ok(json!({
                "error": "No content type provided"
            }).to_string());
        }
    };
    
    println!("file content_type: {}", content_type);
    
    // Check if content type is an image
    if content_type.top() != "image" {
        return Ok(json!({
            "error": "Only image files are allowed (JPEG, PNG, GIF, WebP)"
        }).to_string());
    }

    // Generate unique filename
    let file_extension = match content_type.sub().as_str() {
        "jpeg" => "jpg",
        "png" => "png", 
        "gif" => "gif",
        "webp" => "webp",
        _ => "jpg", // default fallback
    };
    
    let filename = format!("{}.{}", Uuid::new_v4(), file_extension);
    let upload_dir = &settings.image_dir;
    
    // Ensure upload directory exists
    if let Err(_) = fs::create_dir_all(upload_dir) {
        println!("Failed to create upload directory: {}", upload_dir);
        return Err(Status::InternalServerError);
    }
    
    let file_path = Path::new(upload_dir).join(&filename);
    
    // Move the temporary file to the target location
    match file.persist_to(&file_path).await {
        Ok(_) => {
            // Return the URL path for accessing uploaded images            
            let url = format!("{}/api/images/{}", settings.server_url(),filename);
            Ok(json!({
                "success": true,
                "url": url,
                "filename": filename
            }).to_string())
        }
        Err(e) => {
            println!("Failed to save file: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/api/admin/upload", data = "<upload>")]
pub async fn admin_upload_file(
    upload: Form<Upload<'_>>,
    settings: &State<Settings>,
) -> Result<String, Status> {
    // Same implementation as upload_file but for admin routes
    upload_file(upload, settings).await
}

#[get("/api/images/<filename>")]
pub async fn serve_image(
    filename: &str,
    settings: &State<Settings>,
) -> Option<NamedFile> {
    let file_path = Path::new(&settings.image_dir).join(filename);
    NamedFile::open(file_path).await.ok()
}