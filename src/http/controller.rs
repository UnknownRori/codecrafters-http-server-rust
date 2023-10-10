use std::sync::Arc;

use super::{Request, Response};

pub type FnController<T> =
    Arc<dyn Fn(&Request, &T) -> Result<Response, Box<dyn std::error::Error>>>;
// pub type AsyncFnController<T> = Arc<
//     dyn Fn(&Request, &T) -> Pin<Box<dyn Future<Output = Result<Response, AsyncError>>>>
//         + Send
//         + Sync
//         + 'static,
// >;

// pub struct WrapperController<T: Send + Sync + 'static>(pub(crate) Arc<FnController<T>>);
//
// unsafe impl<T: Send + Sync + 'static> Send for WrapperController<T> {}
// unsafe impl<T: Send + Sync + 'static> Sync for WrapperController<T> {}
