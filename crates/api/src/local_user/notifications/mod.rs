pub mod list_mentions;
pub mod list_replies;
pub mod mark_all_read;
pub mod mark_mention_read;
pub mod mark_reply_read;
pub mod unread_count;

#[cfg(feature = "prometheus-metrics")]
const API_SLO: autometrics::objectives::Objective =
  autometrics::objectives::Objective::new("local_user_notifications_api")
    .success_rate(autometrics::objectives::ObjectivePercentile::P99_9)
    .latency(
      autometrics::objectives::ObjectiveLatency::Ms250,
      autometrics::objectives::ObjectivePercentile::P99,
    );
