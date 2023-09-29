pub mod create;
pub mod delete;
pub mod update;

#[cfg(feature = "prometheus-metrics")]
const CRUD_SLO: autometrics::objectives::Objective =
  autometrics::objectives::Objective::new("custom_emoji_crud")
    .success_rate(autometrics::objectives::ObjectivePercentile::P99_9)
    .latency(
      autometrics::objectives::ObjectiveLatency::Ms250,
      autometrics::objectives::ObjectivePercentile::P99,
    );
