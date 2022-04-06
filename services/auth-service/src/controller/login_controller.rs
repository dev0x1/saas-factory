use std::sync::Arc;

use actix::Addr;
use actix_web::{
    post,
    web::{self},
    HttpResponse,
};
use bson::Uuid;
use chrono::Utc;
use common::{
    error::{ApiResult, InternalError},
    model::event::{
        v1::{
            auth::{prelude::SERVICE_AUTH_SUBJECT, UserCreatedMessage},
            Event,
        },
        EventMessage,
        EventMetadata,
    },
};
use futures::TryFutureExt;
use nats_actor::{publisher::NatsPublisher, EventMessage as NatsEventMessage};
use validator::Validate;

use crate::{
    context::AppContext,
    model::{
        domain::{
            login::LoginAttempt,
            user::{User, UserRole, UserStatus},
        },
        request::login::login_request::{Identify, Invite, InviteConfirmation, Verify},
    },
    repository::{login_repository, user_repository},
};

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(invite);
    cfg.service(invite_confirmation);
    cfg.service(identify);
    cfg.service(verify);
}

/// Adminstrator can invite users with this API by providing their email address that will be used
/// to send a auth login link to them.
#[tracing::instrument(name = "invite", skip(invite_request), level = "info")]
#[post("/invite")]
pub async fn invite(
    ctx: web::Data<AppContext>,
    web::Query(invite_request): web::Query<Invite>,
) -> ApiResult {
    // #TODO check if called by admin
    // #TODO check if email domain is whitelisted
    // #TODO generate random uuid, set expiry as per policy
    //       we can manage expiry date based on created_date
    // #TODO send email to user
    invite_request.validate()?;

    // #TODO if email is already invited or registered

    let now = Some(Utc::now());
    let to_create = User {
        id: Some(bson::Uuid::new()),
        email: invite_request.email,
        status: Some(UserStatus::Invited),
        role: invite_request.role,
        created_at: now,
        updated_at: now,
    };

    user_repository::insert_one(&to_create, ctx.db()).await?;

    Ok(HttpResponse::Ok().finish())
}

/// Users click the admin invitation link in their email and this API will be called for account
/// creation.
#[tracing::instrument(
    name = "invite_confirmation",
    skip(invite_confirmation),
    level = "info"
)]
#[post("/invite_confirmation")]
pub async fn invite_confirmation(
    ctx: web::Data<AppContext>,
    web::Query(invite_confirmation): web::Query<InviteConfirmation>,
) -> ApiResult {
    invite_confirmation.validate()?;
    // #TODO if invitation is expired, resend invitation error

    let email = invite_confirmation
        .email
        .ok_or(InternalError::RequestFormatError {
            reason: "require fields: `email`".to_string(),
        })?;
    // find user by email
    // #TODO sanitize email before db query
    let mut user = user_repository::find_by_email(&email, ctx.db())
        .await?
        .ok_or(InternalError::AuthInvalidInvitation {
            cause: "this email is not invited".to_string(),
        })?;

    user.status = Some(UserStatus::Active);
    let _ = user_repository::update_by_id(&user, ctx.db()).await?;

    // emit an event
    let user_created_event = Event::AuthUserCreated(EventMessage {
        meta: EventMetadata::new(SERVICE_AUTH_SUBJECT.into(), "trace_id"),
        payload: UserCreatedMessage {
            id: user.id.unwrap().to_string(),
            email: user.email.clone().unwrap(),
        },
    });
    let publisher = Arc::clone(&ctx.event_publisher);
    publisher.do_send(NatsEventMessage {
        event: user_created_event.try_into().unwrap(),
    });

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(user))
}

/// Users can initiate login using this API by providing their email. If the user is already
/// invited, they recieve an email with login data link(an uuid) and this login attempt is stored
/// for verification.
#[tracing::instrument(name = "identify", skip(identify), level = "info")]
#[post("/identify")]
pub async fn identify(
    ctx: web::Data<AppContext>,
    web::Query(identify): web::Query<Identify>,
) -> ApiResult {
    identify.validate()?;

    let email = identify.email.ok_or(InternalError::RequestFormatError {
        reason: "require fields: `email`".to_string(),
    })?;
    // check if user exists
    // #TODO sanitize email before db query
    // #TODO handle case: duplicate users by email
    let _ = user_repository::find_by_email(&email, ctx.db())
        .await?
        .ok_or(InternalError::AuthUserNotFound)?;

    // record login attempt
    let mut login_attempt = LoginAttempt {
        email: email.clone(),
        otp_code: Uuid::new(),
        otp_requests_count: 0,
    };
    let login_attempt = match login_repository::find_by_email(&email, ctx.db()).await? {
        Some(l) => {
            login_attempt.otp_requests_count = l.otp_requests_count + 1;
            // #TODO return error if max otp request policy is met
            login_attempt
        }
        None => login_attempt,
    };

    let _ = login_repository::insert_one(&login_attempt, ctx.db()).await?;

    // #TODO mail otp

    Ok(HttpResponse::Ok().finish())
}

/// Users can verify by clicking on the link in their mails, which will call this API. If the user
/// has attempted to login and their uuids match, this means the authentication was successful.
#[tracing::instrument(name = "verify", skip(verify), level = "info")]
#[post("/verify")]
pub async fn verify(
    ctx: web::Data<AppContext>,
    web::Query(verify): web::Query<Verify>,
) -> ApiResult {
    verify.validate()?;

    let otp_code = verify.id.ok_or(InternalError::RequestFormatError {
        reason: "require fields: `id`".to_string(),
    })?;
    // #TODO sanitize otp before db query
    let _ = login_repository::find_by_otp(&otp_code, ctx.db())
        .await?
        .ok_or(InternalError::AuthUserNotFound)?;

    // #TODO find user by email
    // send authorization token in response
    Ok(HttpResponse::Ok().finish())
}
