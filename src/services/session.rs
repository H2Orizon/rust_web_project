use rocket::{Request, request::{FromRequest, Outcome}};
use rocket::http::Status;
use sea_orm::DatabaseConnection;
use serde::Serialize;

use crate::services::user_service::get_user;
use crate::models::user_model::UserDTO;

#[derive(Serialize)]
pub struct UserSession {
    pub user: UserDTO,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserSession {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = req.cookies();
        let db = req.rocket().state::<DatabaseConnection>().unwrap();

        if let Some(cookie) = cookies.get_private("user_id") {
            if let Ok(id) = cookie.value().parse::<i32>() {
                if let Ok(user) = get_user(db, id).await {
                    let dto = UserDTO {
                        id: user.id,
                        username: user.username,
                        email: user.email,
                        phone_num: user.phone_num,
                        role: user.role,
                        img_url: user.img_url
                    };
                    return Outcome::Success(UserSession { user: dto });
                }
            }
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}