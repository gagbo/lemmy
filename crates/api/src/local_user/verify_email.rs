use actix_web::web::{Data, Json};
use lemmy_api_common::{
  context::LemmyContext,
  person::{VerifyEmail, VerifyEmailResponse},
  utils::send_new_applicant_email_to_admins,
};
use lemmy_db_schema::{
  source::{
    email_verification::EmailVerification,
    local_user::{LocalUser, LocalUserUpdateForm},
    person::Person,
  },
  traits::Crud,
  RegistrationMode,
};
use lemmy_db_views::structs::SiteView;
use lemmy_utils::error::{LemmyError, LemmyErrorExt, LemmyErrorType};

pub async fn verify_email(
  data: Json<VerifyEmail>,
  context: Data<LemmyContext>,
) -> Result<Json<VerifyEmailResponse>, LemmyError> {
  let site_view = SiteView::read_local(&mut context.pool()).await?;
  let token = data.token.clone();
  let verification = EmailVerification::read_for_token(&mut context.pool(), &token)
    .await
    .with_lemmy_type(LemmyErrorType::TokenNotFound)?;

  let form = LocalUserUpdateForm {
    // necessary in case this is a new signup
    email_verified: Some(true),
    // necessary in case email of an existing user was changed
    email: Some(Some(verification.email)),
    ..Default::default()
  };
  let local_user_id = verification.local_user_id;

  let local_user = LocalUser::update(&mut context.pool(), local_user_id, &form).await?;

  EmailVerification::delete_old_tokens_for_local_user(&mut context.pool(), local_user_id).await?;

  // send out notification about registration application to admins if enabled
  if site_view.local_site.registration_mode == RegistrationMode::RequireApplication
    && site_view.local_site.application_email_admins
  {
    let person = Person::read(&mut context.pool(), local_user.person_id).await?;
    send_new_applicant_email_to_admins(&person.name, &mut context.pool(), context.settings())
      .await?;
  }

  Ok(Json(VerifyEmailResponse {}))
}
