pub mod comment;
pub mod post;
pub mod private_message;

#[cfg(feature = "prometheus-metrics")]
const APUB_SLO: autometrics::objectives::Objective =
  autometrics::objectives::Objective::new("activities_create_or_update_apub")
    .success_rate(autometrics::objectives::ObjectivePercentile::P99_9)
    .latency(
      autometrics::objectives::ObjectiveLatency::Ms250,
      autometrics::objectives::ObjectivePercentile::P99,
    );
