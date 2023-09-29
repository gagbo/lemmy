pub(crate) mod community_featured;
pub(crate) mod community_moderators;
pub(crate) mod community_outbox;

#[cfg(feature = "prometheus-metrics")]
const APUB_SLO: autometrics::objectives::Objective =
  autometrics::objectives::Objective::new("collections_apub")
    .success_rate(autometrics::objectives::ObjectivePercentile::P99_9)
    .latency(
      autometrics::objectives::ObjectiveLatency::Ms250,
      autometrics::objectives::ObjectivePercentile::P99,
    );
