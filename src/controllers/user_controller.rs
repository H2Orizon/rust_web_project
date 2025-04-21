use rocket::request::FlashMessage;
use rocket::response::Flash;
use rocket_dyn_templates::{Template,context};
use rocket::http::{Cookie, CookieJar};
use rocket::State;
use rocket::{form::Form, response::Redirect};
use sea_orm::DatabaseConnection;
use crate::models::user_model::{ChangePasswordForm, EditUserForm, LogInUserForm};
use crate::services::comment_service::get_all_user_comments;
use crate::services::item_service::get_all_user_item;
use crate::services::session::UserSession;
use crate::services::user_service::{change_img, change_password_f, edit_profile_f, get_all_users, get_userDTO};
use crate::{models::user_model::NewUserForm, services::user_service::{create_user, log_in as log_inF}};
use crate::services::help_service::{file_load,UploadForm};

#[get("/log_in")]
pub fn log_in(flash: Option<FlashMessage<'_>>) -> Template {

    let (flash_msg, flash_kind) = if let Some(f) = &flash {
        (f.message(), f.kind())
    } else {
        ("", "")
    };

    Template::render("user/log_in", context! {
        title:"Log in",
        flash_msg: flash_msg,
        flash_kind: flash_kind
    })
}

#[post("/log_in", data = "<form_data>")]
pub async fn post_log_in(
    db: & State<DatabaseConnection>, 
    form_data: Form<LogInUserForm>,
    cookies: & CookieJar<'_>
) -> Flash<Redirect>{
    match log_inF(&db, &form_data).await {
        Ok(user) => {
            cookies.add_private(Cookie::new("user_id", user.id.to_string()));
            Flash::success(Redirect::to("/profile"), "Вітаю у вашому профілю")
        },
        Err(e) => Flash::error( Redirect::to("/log_in?error=invalid_credentials"), &e.to_string()),
    }   
}

#[get("/register")]
pub fn register(flash: Option<FlashMessage<'_>>) -> Template {

    let (flash_msg, flash_kind) = if let Some(f) = &flash {
        (f.message(), f.kind())
    } else {
        ("", "")
    };

    Template::render("user/register", context! {
        title:"Register",
        flash_msg: flash_msg,
        flash_kind: flash_kind
    })
}

#[post("/register", data = "<form_data>")]
pub async fn post_register(
    db: &State<DatabaseConnection>, 
    form_data: Form<NewUserForm>
) -> Flash<Redirect> {
    match create_user(db.inner(), &form_data).await {
        Ok(_) => Flash::success(Redirect::to("/log_in"), "Акаунт успішно створиний"),
        Err(e) => Flash::error( Redirect::to("/register"), e.to_string()),
    }
}

#[get("/profile")]
pub async fn profile(db: &State<DatabaseConnection>, user_session: UserSession, flash: Option<FlashMessage<'_>>) -> Template{
    let (flash_msg, flash_kind) = if let Some(f) = &flash {
        (f.message(), f.kind())
    } else {
        ("", "")
    };

    match get_userDTO(db.inner(), user_session.user.id).await {
        Ok(user) =>{
            let redirect_url = format!("/profile");
            let user_comments = get_all_user_comments(db, user.id).await.unwrap_or_default();
            let user_item = get_all_user_item(db, user.id).await.unwrap_or_default();
            return Template::render("user/profile", context!{
                title:"My profile",
                user: user,
                comments: user_comments,
                items: user_item,
                redirect_url: redirect_url,
                user_in_jar: user_session.user.id,
                flash_kind: flash_kind,
                flash_msg: flash_msg
            })
        }
        Err(_) => return Template::render("error/404", context! { 
            message: "User not found",
            flash_kind: flash_kind,
            flash_msg: flash_msg
        })
    }
}

#[get("/<user_id>/profile")]
pub async fn profile_user(db: &State<DatabaseConnection>, user_id: i32, user_session: UserSession) -> Result<Template, Redirect>{
    if user_id == user_session.user.id{
        return Err(Redirect::to("/profile"));
    }

    match get_userDTO(db.inner(), user_id).await {
        Ok(user) =>{
            let user_comments = get_all_user_comments(db, user_id).await.unwrap_or_default();
            let user_item = get_all_user_item(db, user_id).await.unwrap_or_default();
            Ok(Template::render("user/profile", context!{
                title:"My profile",
                user:user,
                comments: user_comments,
                items: user_item,
                redirect_url: "None",
                delete_url: "None",
                user_in_jar: "None"
            }))
        }
        Err(_) => Err(Redirect::to(uri!("/404")))
    }
} 

#[post("/profile/log_out")]
pub fn log_out(cookies: &CookieJar<'_>) -> Redirect{
    cookies.remove_private("user_id");
    Redirect::to(uri!(crate::controllers::home_controller::index))
}

#[get("/all_users")]
pub async fn get_all_user(db: &State<DatabaseConnection>) -> Template {
    let users = get_all_users(db).await.unwrap_or_default();
    Template::render("user/all_users", context!{
        title:"All users",
        message:"All users",
        users:users
    })
}

#[get("/edit_profile")]
pub fn edit_profile(user_session: UserSession, flash: Option<FlashMessage<'_>>) -> Template {

    let (flash_msg, flash_kind) = if let Some(f) = &flash {
        (f.message(), f.kind())
    } else {
        ("", "")
    };

    if user_session.user.id != 0{
        return Template::render("user/edit_profile", context!{
        title:"edit_profile",
        flash_kind: flash_kind,
        flash_msg: flash_msg
        })
    }
    Template::render("error/403", context! { message: "Invalid session" })
}

#[patch("/edit_profile", data = "<form_data>")]
pub async fn patch_edit_profile(db: &State<DatabaseConnection>,user_session: UserSession, form_data: Form<EditUserForm>) -> Flash<Redirect> {
    if user_session.user.id != 0{
        match edit_profile_f(db, user_session.user.id, &form_data).await {
            Ok(_) => return Flash::success( Redirect::to(uri!(profile)), "Дані аккаунт успішно змінено"),
            Err(e) => return Flash::error( Redirect::to("/user/edit_profile"), e.to_string()),
        }
    }
    Flash::error(Redirect::to(uri!(log_in)), "Invalid session")
}

#[post("/add_img", data = "<form_data>")]
pub async fn add_img<'r>(db: &State<DatabaseConnection>,form_data: Form<UploadForm<'r>>, user_session: UserSession) -> Flash<Redirect>{
    let dir = "user_img";

    if user_session.user.id == 0{
        return Flash::error( Redirect::to(uri!(log_in)), "Користувач не в сесії")
    }
    
    let filename = match file_load(form_data, &dir).await {
        Ok(name) => name,
        Err(_) => {
            eprintln!("Не вдалося зберегти файл");
            return Flash::error( Redirect::to(uri!(profile)), "Не вдалося зберегти файл")
        }
    };

    println!("Файл збережено в: {}", filename);

    if let Err(err) = change_img(db, user_session.user.id, filename).await {
        eprintln!("Помилка при оновленні картинки: {:?}", err);
    }

    Flash::success(Redirect::to(uri!(profile)),"Картинка успішно змінина")
}


#[get("/change_password")]
pub fn change_password(user_session: UserSession, flash: Option<FlashMessage<'_>>) -> Template{
    let (flash_msg, flash_kind) = if let Some(f) = &flash {
        (f.message(), f.kind())
    } else {
        ("", "")
    };

    if user_session.user.id != 0{
        return Template::render("user/change_password", context!{
            title:"Зміна пароля",
            flash_msg: flash_msg,
            flash_kind: flash_kind
        });
    }
    Template::render("error/403", context! { message: "Invalid session" })
}

#[patch("/change_password", data = "<form_data>")]
pub async fn patch_change_password(db: &State<DatabaseConnection>, user_session: UserSession, form_data: Form<ChangePasswordForm>) -> Flash<Redirect> {
    if user_session.user.id != 0{
        match change_password_f(db, &form_data.into_inner(), user_session.user.id).await {
            Ok(_) => return Flash::success(Redirect::to(uri!(profile)), "Пароль успішно змінино"),
            Err(e) => return Flash::error( Redirect::to("/user/change_password"), e.to_string())
        }
    }
    Flash::error(Redirect::to(uri!(log_in)), "Invalid session")
}