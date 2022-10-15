use super::{accounts, auth, users};
use axum::Json;
use utoipa::{openapi, OpenApi};

#[derive(OpenApi)]
#[openapi(
    paths(
        users::find_users,
        users::find_user_by_id,
        accounts::register,
        auth::authorize,
        auth::revalidate
    ),
    components(schemas(
        users::UserResponse,
        users::UsersResponse,
        auth::AuthBody,
        auth::AuthResponse,
        auth::RevalidateBody,
        auth::RevalidateResponse,
        accounts::RegisterBody,
        accounts::RegisterResponse,
        crate::Error
    ))
)]
pub(super) struct ApiDoc;

pub async fn get_openapi() -> Json<openapi::OpenApi> {
    let doc = ApiDoc::openapi();

    Json(doc)
}
