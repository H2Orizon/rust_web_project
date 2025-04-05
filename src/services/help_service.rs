use rocket::form::Form;
use uuid::Uuid;
use rocket::fs::{TempFile, relative};
use tokio::fs;
use std::{{env, io}, path::PathBuf};


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

pub async fn delete_image(filename: &str) -> Result<(), io::Error> {
    let path: PathBuf = PathBuf::from(relative!("uploads")).join(filename);
    fs::remove_file(&path).await
}