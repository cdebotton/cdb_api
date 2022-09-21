use super::{accounts, users};
use axum::Json;
use utoipa::{openapi, OpenApi};

#[derive(OpenApi)]
#[openapi(
    paths(
        users::find_users,
        users::find_user_by_id,
        accounts::authorize,
        accounts::revalidate
    ),
    components(schemas(
        users::UsersResponse,
        users::UserResponse,
        users::Account,
        accounts::AuthPayload,
        accounts::AuthBody,
        accounts::RevalidatePayload,
        crate::Error
    ))
)]
pub(super) struct ApiDoc;

pub async fn get_openapi() -> Json<openapi::OpenApi> {
    let doc = ApiDoc::openapi();

    Json(doc)
}
