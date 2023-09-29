pub mod create;
pub mod list;
pub mod resolve;

#[cfg(feature = "prometheus-metrics")]
const API_SLO: autometrics::objectives::Objective =
  autometrics::objectives::Objective::new("priv_msg_report_api")
    .success_rate(autometrics::objectives::ObjectivePercentile::P99_9)
    .latency(
      autometrics::objectives::ObjectiveLatency::Ms250,
      autometrics::objectives::ObjectivePercentile::P99,
    );
