use rocket::{form::Form, request::FlashMessage, response::{Flash, Redirect}, State};
use rocket_dyn_templates::{Template,context};
use sea_orm::DatabaseConnection;

use crate::{models::{category_model::{DeleteCommUrl, NewCategory}, item_model::NewItemForm}, 
services::{comment_service::get_all_item_comments, help_service::{delete_image, file_load_for_item, UploadForm}, img_for_items_services::{add_img_to_item, delete_image_db, get_img_url_as_string}, item_service::{create_category_f, create_new_item, delete_item_f, get_all_categoris, get_all_item, get_one_item, update_item}, session::UserSession, user_service::get_user_dto}};

#[get("/items")]
pub async fn get_items(db: &State<DatabaseConnection>, flash: Option<FlashMessage<'_>>, user_session: Option<UserSession>) -> Template {

    let (flash_msg, flash_kind) = if let Some(f) = &flash {
        (f.message(), f.kind())
    } else {
        ("", "")
    };

    let items = get_all_item(db).await.unwrap_or_default();
    let categories = get_all_categoris(db).await.unwrap_or_default();
    Template::render("items/items", context! {
            title:"Items",
            items:items,                 
            flash_msg: flash_msg,
            flash_kind: flash_kind,
            categories: categories,
            user: user_session.map(|s| s.user)
        }
    )
}

#[get("/create_category")]
pub fn create_category(user_session: UserSession, flash: Option<FlashMessage<'_>>) -> Template{
    if user_session.user.role == "admin"{
        let (flash_msg, flash_kind) = if let Some(f) = &flash {
            (f.message(), f.kind())
        } else {
            ("", "")
        };

        return Template::render("items/creat_new_category", context!{
            title:"Create new category",
            flash_msg: flash_msg,
            flash_kind: flash_kind,
            user: user_session.user
        });
    }
    Template::render("error/403", context! {title:"Виникла помилка!", message: "Invalid session" })
}

#[post("/create_category", data = "<form_data>")]
pub async fn post_create_category(db: &State<DatabaseConnection>, user_session: UserSession, form_data: Form<NewCategory>) -> Flash<Redirect> {
    if user_session.user.role == "admin"{
        match create_category_f(db.inner(), &form_data).await{
            Ok(_) => Flash::success(Redirect::to("/items/create_category"), "Категорія успішно створена"),
            Err(e) => Flash::error(Redirect::to("/items/create_category"), &e.to_string())
        };
    }
    Flash::error(Redirect::to("/log_in"), "Будь ласка, увійдіть у свій акаунт.")
}

#[get("/<item_id>")]
pub async fn get_item(db: &State<DatabaseConnection>, item_id: i32, user_session: UserSession) -> Template {
    let redirect_url = format!("/items/{}", item_id);

    match get_one_item(db, item_id).await {
        Ok(item) => {
            let comment_dtos = match get_all_item_comments(db, item_id).await {
                Ok(comments) => comments,
                Err(err) => {
                    eprintln!("Помилка при завантаженні коментарів: {:?}", err);
                    vec![]
                }
            };
            let user = user_session.user;
            let creator = match get_user_dto(db, item.user_id).await {
                Ok(u) => u,
                Err(_) => {
                    return Template::render("error/404", context! {
                        title: "Користувача не знайдено",
                        message: "Автор цього товару не знайдений."
                    });
                }
            };
            Template::render("items/item", context! {item:item, item_id:item_id, comments: comment_dtos, user: user, redirect_url:redirect_url, creator:creator})
        }
        Err(_) => Template::render("error/404", context! {
            title: "Товар не знайдено",
            message: "Схоже, цього товару не існує або він був видалений."
        }),
    }
}

#[get("/item_create")]
pub async fn create(db: &State<DatabaseConnection>, user_session: UserSession, flash: Option<FlashMessage<'_>>) -> Template {
    if user_session.user.id != 0{
        let (flash_msg, flash_kind) = if let Some(f) = &flash {
            (f.message(), f.kind())
        } else {
            ("", "")
        };
        
        let categories = get_all_categoris(db).await.unwrap_or_default(); 
        return Template::render("items/add_item", context!{
            title:"Create new item", 
            categories:categories, 
            flash_msg: flash_msg, 
            flash_kind: flash_kind,
            user: user_session.user
    });
    }
    Template::render("error/403", context! { message: "Invalid session" })
}

#[post("/item_create", data="<form_data>")]
pub async fn post_create(db: &State<DatabaseConnection>, form_data: Form<NewItemForm>, user_session: UserSession) -> Flash<Redirect> {
    if user_session.user.id != 0{
        match create_new_item(db.inner(), &form_data, user_session.user.id).await {
            Ok(_) => return Flash::success(Redirect::to(uri!(get_items)), "Товар створено успішно!"),
            Err(e) =>{ 
                return Flash::error(Redirect::to("/items/item_create"), &e.to_string()) 
            }
        }
    }
    Flash::error(Redirect::to("/log_in"), "Будь ласка, увійдіть у свій акаунт.")
}

#[get("/<item_id>/item_edit")]
pub async  fn item_edit( db: &State<DatabaseConnection>,item_id: i32, user_session: UserSession, flash: Option<FlashMessage<'_>>) ->Template {
    if user_session.user.id != 0{
        let (flash_msg, flash_kind) = if let Some(f) = &flash {
            (f.message(), f.kind())
        } else {
            ("", "")
        };

        match get_one_item(db, item_id).await {
            Ok(item) => {
                if item.user_id == user_session.user.id {
                    let categories = get_all_categoris(db).await.unwrap_or_default(); 
                    return Template::render("items/edit_item", context!{
                            title: format!("Edit Item {}", item_id), 
                            categories:categories, 
                            item_id:item_id,
                            flash_msg: flash_msg, 
                            flash_kind: flash_kind,
                            user: user_session.user
                        }
                    )
                }
            }
            Err(_) => {
                return Template::render("error/403", context!{title: format!("Edit Item {}", item_id)})
            }
        }
    }
    Template::render("error/403", context!{title: format!("Edit Item {}", item_id)})
}

#[patch("/<item_id>/item_edit", data = "<form_data>")]
pub async fn patch_item_edit( db: &State<DatabaseConnection>, item_id: i32, form_data: Form<NewItemForm>, user_session: UserSession) -> Flash<Redirect> {
    if user_session.user.id == 0 {
        return Flash::error(Redirect::to("/log_in"), "Увійдіть в аккаунт");
    }
    let item = match get_one_item(db, item_id).await {
        Ok(item) => item,
        Err(_) => {
            return Flash::error(Redirect::to("/items"), "Товар не знайдено або ви не маєте до нього доступу");
        }
    };

    if item.user_id != user_session.user.id {
        return Flash::error(
            Redirect::to("/items"), "У вас немає доступу до редагування цього товару");
    }

    match update_item(db, item_id, &form_data).await {
        Ok(_) => Flash::success(Redirect::to(format!("/items/{}", item_id)), "Товар успішно змінено"),
        Err(e) => Flash::error(Redirect::to(format!("/items/{}", item_id)), &e.to_string()),
    }
}

#[delete("/<item_id>/delete", data="<form_data>")]
pub async fn delete_item(db: &State<DatabaseConnection>, item_id: i32, form_data: Form<DeleteCommUrl>, user_session: UserSession) -> Flash<Redirect> {
    if user_session.user.id == 0{
        return Flash::error(Redirect::to("/log_in"), "Увійдіть в аккаунт");
    }
    let item = match get_one_item(db, item_id).await {
        Ok(item) => item,
        Err(_) => {
            return Flash::error(Redirect::to("/items"), "Товар не знайдено або ви не маєте до нього доступу");
        }
    };

    if item.user_id != user_session.user.id {
        return Flash::error(Redirect::to("/items"), "У вас немає доступу до редагування цього товару");
    }

    match delete_item_f(db, item_id).await {
        Ok(_) => return Flash::warning(Redirect::to(form_data.redirect_url.clone()), "Товар успішно видалино"),
        Err(e) => return Flash::error(Redirect::to(form_data.redirect_url.clone()), e.to_string()) 
    }
}

#[post("/<item_id>/add_item_img", data = "<form_data>")]
pub async fn post_add_img_to_item<'r>(db: &State<DatabaseConnection>, item_id: i32, form_data: Form<UploadForm<'r>>, user_session: UserSession) -> Redirect {
    if user_session.user.id == 0{
        return Redirect::to("/log_in");
    }
    let item = match get_one_item(db, item_id).await {
        Ok(item) => item,
        Err(_) => {
            return Redirect::to("/items");
        }
    };
    if item.user_id != user_session.user.id {
        return Redirect::to("/items");
    }
    let filename = match file_load_for_item(&db,form_data, item_id).await {
        Ok(name) => name,
        Err(_) => {
            eprintln!("Не вдалося зберегти файл");
            return Redirect::to(format!("/items/{}",item_id))
        }
    };
    println!("Файл збережено в: {}", filename);

    if let Err(err) = add_img_to_item(db, item_id, filename).await{
        eprintln!("Помилка при оновленні картинки: {:?}", err);
    }

    Redirect::to(format!("/items/{}",item_id))
}

#[delete("/<item_id>/<img_id>/delete")]
pub async fn delete_item_img(db: &State<DatabaseConnection>, img_id: i32, item_id: i32, user_session: UserSession) -> Redirect {
    if user_session.user.id == 0{
        return Redirect::to("/log_in");
    }
    let item = match get_one_item(db, item_id).await {
        Ok(item) => item,
        Err(_) => {
            return Redirect::to("/items");
        }
    };
    if item.user_id != user_session.user.id {
        return Redirect::to("/items");
    }
    let img_url = get_img_url_as_string(db, img_id).await;
    match delete_image(&img_url).await {
        Ok(_) => {
            match delete_image_db(db, img_id).await{
                Ok(_) => return Redirect::to(format!("/items/{}",item_id)),
                Err(_) => return Redirect::to(format!("/items/{}",item_id))
            }
            
        },
        Err(_) => return Redirect::to(format!("/items/{}",item_id))
    }
}