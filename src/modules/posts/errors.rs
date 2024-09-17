use thiserror::Error;
use serde::Serialize;
use crate::shared::common::errors::CommonErrors;

#[derive(Serialize, Debug, Error)]
pub enum PostsDomainErrors {
    #[error("Missing 'sort' parameter")]
    QueryParamSortMissing,

    #[error("Invalid 'sort' parameter")]
    QueryParamSortInvalid,
}

#[derive(Debug, Error)]
pub enum PostsModuleErrors {
    #[error("Domain error: {0}")]
    DomainError(#[from] PostsDomainErrors),

    #[error("Common error: {0}")]
    CommonError(#[from] CommonErrors),

}
