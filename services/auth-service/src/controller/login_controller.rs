use actix_web::{
    post,
    web::{self},
    HttpResponse,
};
use bson::Uuid;
use chrono::Utc;
use common::error::{ApiResult, InternalError};

use crate::{
    context::AppContext,
    model::domain::user::{User, UserRole, UserStatus},
    repository::{auth_repository, user_repository},
};

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(invite);
    cfg.service(invite_confirmation);
    cfg.service(identify);
    cfg.service(verify);
}

/// Adminstrator can invite users with this API by providing their email address that will be used
/// to send a auth login link to them.
#[tracing::instrument(name = "invite", level = "info")]
#[post("/invite")]
pub async fn invite(ctx: web::Data<AppContext>) -> ApiResult {
    // #TODO check if called by admin
    // #TODO check if email domain is whitelisted
    // #TODO check if email is formated correctly
    // #TODO generate random uuid, set expiry as per policy
    // #TODO record above in database
    // #TODO send email to user
    todo!()
}

/// Users click the admin invitation link in their email and this API will be called for account
/// creation.
#[tracing::instrument(name = "invite_confirmation", level = "info")]
#[post("/invite_confirmation")]
pub async fn invite_confirmation(ctx: web::Data<AppContext>) -> ApiResult {
    todo!()
}

/// Users can initiate login using this API by providing their email. If the user is already
/// invited, they recieve an email with login data link(an uuid) and this login attempt is stored
/// for verification.
#[tracing::instrument(name = "identify", level = "info")]
#[post("/identify")]
pub async fn identify(ctx: web::Data<AppContext>) -> ApiResult {
    todo!()
}

/// Users can verify by clicking on the link in their mails, which will call this API. If the user
/// has attempted to login and their uuids match, this means the authentication was successful.
#[tracing::instrument(name = "verify", level = "info")]
#[post("/verify")]
pub async fn verify(ctx: web::Data<AppContext>) -> ApiResult {
    todo!()
}
