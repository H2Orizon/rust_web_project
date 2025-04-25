use rocket::form::Form;
use uuid::Uuid;
use rocket::fs::{TempFile, relative};
use tokio::fs;
use std::{{env, io}, path::PathBuf};

use super::img_for_items_services::can_add_more_imgs;


#[derive(FromForm)]
pub struct UploadForm<'r> {
    file: TempFile<'r>,
}

pub async fn file_load<'r>(mut form_data: Form<UploadForm<'r>>, dir: &str) -> Result<String, ()> {
    let file_extension = form_data
        .file
        .content_type()
        .and_then(|ct| ct.extension().map(|ext| ext.to_string()))
        .unwrap_or_else(|| "jpg".to_string());

    let allowed_extensions = ["jpg", "jpeg", "png", "webp"];

    if !allowed_extensions.contains(&file_extension.as_str()) {
        eprintln!("Заборонене розширення файлу: {}", file_extension);
        return Err(());
    }

    let unique_filename = format!("{}.{}", Uuid::new_v4(), file_extension);

    let base_path = env::current_dir().unwrap();

    let path_img = base_path.join("uploads").join(dir);

    if let Err(e) = tokio::fs::create_dir_all(&path_img).await {
        eprintln!("Помилка при створенні директорії: {}", e);
        return Err(());
    }

    let upload_path = path_img.join(&unique_filename);
    if let Err(e) = form_data.file.copy_to(&upload_path).await {
        eprintln!("Помилка при збереженні файлу: {}", e);
        return Err(());
    }

    Ok(format!("{}/{}", dir, unique_filename))
}


pub async fn file_load_for_item<'r>(db:&sea_orm::DatabaseConnection,mut form_data: Form<UploadForm<'r>>, item_id: i32) -> Result<String, ()> {
    match can_add_more_imgs(db, item_id).await {
        Ok(false) => {
            eprintln!("Досягнуто ліміту зображень (5)");
            return Ok("Досягнуто ліміту зображень (5)".to_string());
        }
        Err(e) => {
            eprintln!("Помилка при перевірці ліміту: {:?}", e);
            return Err(());
        }
        _ => {}
    }
    
    let allowed_extensions = ["jpg", "jpeg", "png", "webp"];
    
    let file_extension = form_data
        .file
        .content_type()
        .and_then(|ct| ct.extension().map(|ext| ext.to_string()))
        .unwrap_or_else(|| "jpg".to_string());

    if !allowed_extensions.contains(&file_extension.as_str()) {
        eprintln!("Заборонене розширення файлу: {}", file_extension);
        return Err(());
    }

    let unique_filename = format!("{}.{}", Uuid::new_v4(), file_extension);

    let base_path = env::current_dir().unwrap();
    let path_img = base_path.join("uploads").join("item_img").join(item_id.to_string());

    if let Err(e) = tokio::fs::create_dir_all(&path_img).await {
        eprintln!("Помилка при створенні директорії: {}", e);
        return Err(());
    }

    let upload_path = path_img.join(&unique_filename);
    if let Err(e) = form_data.file.copy_to(&upload_path).await {
        eprintln!("Помилка при збереженні файлу: {}", e);
        return Err(());
    }

    Ok(format!("item_img/{}/{}", item_id, unique_filename))
}

pub async fn delete_image(filename: &str) -> Result<(), io::Error> {
    let path: PathBuf = PathBuf::from(relative!("uploads")).join(filename);
    fs::remove_file(&path).await
}

pub async fn delete_item_folder(dir_name: &str) -> Result<(), io::Error> {
    let path: PathBuf = PathBuf::from(relative!("uploads/item_img")).join(dir_name);
    if path.exists() {
        fs::remove_dir_all(&path).await?;
    }
    Ok(())
}