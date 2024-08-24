use warp::{reject::Rejection, Filter};

pub(crate) mod handlers;
pub(crate) mod routing;
pub(crate) mod responses;
pub(crate) mod session_store;

pub fn with_const<T: std::fmt::Debug + Send + Sync + Clone + 'static>(
	value: T,
) -> impl Filter<Extract = (T,), Error = Rejection> + Clone {
	warp::any()
		.and(warp::any().map(move || value.clone()))
		.and_then(
		|val: T| async move {
			Ok::<_, Rejection>(val.clone())
		})
}

/*
fn with_domorust(
	domorust: Domorust
) -> impl Filter<Extract = (Domorust,), Error = Infallible> + Clone {
	warp::any().map(move || domorust.clone())
}
*/