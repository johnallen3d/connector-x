use crate::data_order::DataOrder;
use std::any::type_name;
use std::fmt;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ConnectorXError>;

/// Errors that can be raised from this library.
#[derive(Error, Debug)]
pub enum ConnectorXError {
    /// The required type does not same as the schema defined.
    #[error("Data type unexpected: {0:?} expected, {1} found.")]
    TypeCheckFailed(String, &'static str),

    #[error("Data order not supported {0:?}.")]
    UnsupportedDataOrder(DataOrder),

    #[error("Cannot resolve data order: got {0:?} from source, {1:?} from destination.")]
    CannotResolveDataOrder(Vec<DataOrder>, Vec<DataOrder>),

    #[error("Cannot produce a {0}, context: {1}.")]
    CannotProduce(&'static str, ProduceContext),

    #[error("No conversion rule from {0} to {1}.")]
    NoConversionRule(String, String),

    #[error("Only support single query with SELECT statement, got {0}.")]
    SqlQueryNotSupported(String),

    #[error("Cannot get total number of rows in advance.")]
    CountError(),

    #[error("File {0} not found.")]
    FileNotFoundError(String),

    #[error(transparent)]
    SQLParserError(#[from] sqlparser::parser::ParserError),

    #[error(transparent)]
    StdIOError(#[from] std::io::Error),

    #[cfg(feature = "federation")]
    #[error(transparent)]
    J4RSError(#[from] j4rs::errors::J4RsError),

    #[error(transparent)]
    StdVarError(#[from] std::env::VarError),

    #[cfg(feature = "federation")]
    #[error(transparent)]
    DataFusionError(#[from] datafusion::error::DataFusionError),

    #[cfg(feature = "federation")]
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    // #[error(transparent)]
    // PostgresError(#[from] crate::source::postgres::Error),

    // #[error(transparent)]
    // ArrowError(#[from] crate::destination::arrow::ArrowDestinationError),

    // #[error(transparent)]
    // PostgresArrowTransportError(#[from] crate::transports::PostgresArrowTransportError),
    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ConnectorXError {
    pub fn cannot_produce<T>(context: Option<String>) -> Self {
        ConnectorXError::CannotProduce(type_name::<T>(), context.into())
    }
}

#[derive(Debug)]
pub enum ProduceContext {
    NoContext,
    Context(String),
}

impl From<Option<String>> for ProduceContext {
    fn from(val: Option<String>) -> Self {
        match val {
            Some(c) => ProduceContext::Context(c),
            None => ProduceContext::NoContext,
        }
    }
}

impl fmt::Display for ProduceContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProduceContext::NoContext => write!(f, "No Context"),
            ProduceContext::Context(s) => write!(f, "{}", s),
        }
    }
}
