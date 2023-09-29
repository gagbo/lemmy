pub mod distinguish;
pub mod like;
pub mod save;

#[cfg(feature = "prometheus-metrics")]
const API_SLO: autometrics::objectives::Objective =
  autometrics::objectives::Objective::new("comment_api")
    .success_rate(autometrics::objectives::ObjectivePercentile::P99_9)
    .latency(
      autometrics::objectives::ObjectiveLatency::Ms250,
      autometrics::objectives::ObjectivePercentile::P99,
    );
