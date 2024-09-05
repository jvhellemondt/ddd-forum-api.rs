use axum::extract::Query;
use axum::response::IntoResponse;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::modules::posts::errors::PostsDomainErrors::{QueryParamSortInvalid, QueryParamSortMissing};
use crate::modules::posts::use_cases::list_posts;

use crate::shared::common::errors::CommonErrors::{ ServerError};
use crate::shared::infrastructure::utils::response::build_response;

#[derive(Serialize, Deserialize, Clone)]
pub struct ListPostsParams {
    pub sort: Option<String>,
}

pub async fn list_posts(Query(params): Query<ListPostsParams>) -> impl IntoResponse {
    if params.sort.is_none() || params.sort.as_deref() == Some("") {
        return build_response(StatusCode::BAD_REQUEST, None, Some(QueryParamSortMissing.to_string()));
    }
    if params.sort.as_deref() != Some("recent") {
        return build_response(StatusCode::BAD_REQUEST, None, Some(QueryParamSortInvalid.to_string()));
    }

    match list_posts::controller::handle(params).await {
        Ok(items) => build_response(
            StatusCode::OK,
            Some(items),
            None,
        ),
        _ => build_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
            Some(ServerError.to_string()),
        )
    }
}

