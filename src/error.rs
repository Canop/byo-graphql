use {
    thiserror::Error,
    serde::Deserialize,
};

#[derive(Debug, Error)]
pub enum ByoError {
    #[error("server returned an error: {0:?}")]
    Graphql(Vec<GraphqlError>),
    #[error("no data in response")]
    NoData,
    #[error("requesting error")]
    RequestError(#[from] reqwest::Error)
}

pub type ByoResult<T> = Result<T, ByoError>;

#[derive(Debug, Deserialize)]
pub struct GraphqlError {
    pub path: Option<Vec<String>>,
    pub message: Option<String>,
}
