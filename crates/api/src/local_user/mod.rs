pub mod add_admin;
pub mod ban_person;
pub mod block;
pub mod change_password;
pub mod change_password_after_reset;
pub mod generate_totp_secret;
pub mod get_captcha;
pub mod list_banned;
pub mod login;
pub mod notifications;
pub mod report_count;
pub mod reset_password;
pub mod save_settings;
pub mod update_totp;
pub mod verify_email;

#[cfg(feature = "prometheus-metrics")]
const API_SLO: autometrics::objectives::Objective =
  autometrics::objectives::Objective::new("local_user_api")
    .success_rate(autometrics::objectives::ObjectivePercentile::P99_9)
    .latency(
      autometrics::objectives::ObjectiveLatency::Ms250,
      autometrics::objectives::ObjectivePercentile::P99,
    );
