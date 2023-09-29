pub mod approve;
pub mod list;
pub mod unread_count;

#[cfg(feature = "prometheus-metrics")]
const API_SLO: autometrics::objectives::Objective =
  autometrics::objectives::Objective::new("site_registration_applications_api")
    .success_rate(autometrics::objectives::ObjectivePercentile::P99_9)
    .latency(
      autometrics::objectives::ObjectiveLatency::Ms250,
      autometrics::objectives::ObjectivePercentile::P99,
    );
