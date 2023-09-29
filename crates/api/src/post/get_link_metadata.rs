use actix_web::web::{Data, Json};
use lemmy_api_common::{
  context::LemmyContext,
  post::{GetSiteMetadata, GetSiteMetadataResponse},
  request::fetch_site_metadata,
};
use lemmy_utils::error::LemmyError;

#[cfg_attr(feature = "prometheus-metrics", autometrics::autometrics(objective = super::API_SLO))]
#[tracing::instrument(skip(context))]
pub async fn get_link_metadata(
  data: Json<GetSiteMetadata>,
  context: Data<LemmyContext>,
) -> Result<Json<GetSiteMetadataResponse>, LemmyError> {
  let metadata = fetch_site_metadata(context.client(), &data.url).await?;

  Ok(Json(GetSiteMetadataResponse { metadata }))
}
