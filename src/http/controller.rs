use std::sync::Arc;

use super::{Request, Response};

pub type FnController<T> =
    Arc<dyn Fn(&Request, &T) -> Result<Response, Box<dyn std::error::Error>>>;
