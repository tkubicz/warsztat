use warp::{Filter};
use std::convert::Infallible;

pub fn injectState<T: Clone + Sized + Send>(state: T) -> impl Filter<Extract = (T,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}
