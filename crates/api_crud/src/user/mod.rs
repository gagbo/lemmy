pub mod create;
pub mod delete;

#[cfg(feature = "prometheus-metrics")]
const CRUD_SLO: autometrics::objectives::Objective =
  autometrics::objectives::Objective::new("user_crud")
    .success_rate(autometrics::objectives::ObjectivePercentile::P99_9)
    .latency(
      autometrics::objectives::ObjectiveLatency::Ms250,
      autometrics::objectives::ObjectivePercentile::P99,
    );
