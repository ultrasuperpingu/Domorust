
pub(crate) mod login;
pub(crate) mod websocket;

use std::collections::HashMap;
use domorust_models::domorust::Domorust;
use warp::{self, Filter};

use crate::handlers;


/// Takes a list of handler expressions and `or`s them together
/// in a balanced tree. That is, instead of `a.or(b).or(c).or(d)`,
/// it produces `(a.or(b)).or(c.or(d))`, thus nesting the types
/// less deeply, which provides improvements in compile time.
///
/// It also applies `::warp::Filter::boxed` to each handler expression
/// when in `debug_assertions` mode, improving compile time further.
//
// The basic list splitting algorithm here is based on this gist:
// https://gist.github.com/durka/9fc479de2555225a787f
// It uses a counter from which two items are removed each time,
// stopping when the counter reaches 0. At each step, one item
// is moved from the left to the right, and thus at the end,
// there will be the same number of items in each list.
//
// The flow is as follows:
// - If there is one handler expression, debug_box it and return.
// - If there is more than one handler expression:
//   - First, copy the list into two: the one that will go into the
//     right side of the `or`, and one that will serve as a counter.
//     Recurse with these separated by semicolons, plus an empty `left`
//     list before the first semicolon.
//   - Then, as long as there are at least two items in the counter
//     list, remove them and move the first item on the right side of
//     the first semicolon (`head`) to the left side of the first semicolon.
//   - Finally, when there are one or zero items left in the counter,
//     move one last item to the left, make the call this macro on both the 
//     left and right sides, and `or` the two sides together.
//
// For example, balanced_or_tree!(a, b, c, d, e) would take the following steps:
//
// - balanced_or_tree!(a, b, c, d, e)
// - balanced_or_tree!(@internal ; a, b, c, d, e ; a, b, c, d, e) // initialise lists
// - balanced_or_tree!(@internal a ; b, c, d, e ; c, d, e) // move one elem; remove two
// - balanced_or_tree!(@internal a, b ; c, d, e ; e) // now only one elem in counter
// - balanced_or_tree!(a, b, c).or(balanced_or_tree(d, e)) // recurse on each sublist
#[macro_export]
macro_rules! balanced_or_tree {
    // Base case: just a single expression, return it wrapped in `debug_boxed`
    ($x:expr $(,)?) => { debug_boxed!($x) };
    // Multiple expressions: recurse with three lists: left, right and counter.
    ($($x:expr),+ $(,)?) => {
        balanced_or_tree!(@internal  ;     $($x),+; $($x),+)
        //                          ^ left ^ right  ^ counter
    };
    // Counter 1 or 2; move one more item and recurse on each sublist, and or them together
    (@internal $($left:expr),*; $head:expr, $($tail:expr),+; $a:expr $(,$b:expr)?) => {
        (balanced_or_tree!($($left,)* $head)).or(balanced_or_tree!($($tail),+))
    };
    // Counter > 2; move one item from the right to the left and subtract two from the counter
    (@internal $($left:expr),*; $head:expr, $($tail:expr),+; $a:expr, $b:expr, $($more:expr),+) => {
        balanced_or_tree!(@internal $($left,)* $head; $($tail),+; $($more),+)
    };
}
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_boxed {
    ($x:expr) => {
        ::warp::Filter::boxed($x)
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_boxed {
    ($x:expr) => {
        $x
    };
}


/// All customer routes
pub fn routes(domorust: &Domorust) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	let wwwroot=domorust.cli.wwwroot.clone();
	domorust_api()
	.or(domorust_api_old())
	.or(websocket::websocket())
	// TODO: ensure db is not writting
	.or(warp::path("domorust.db").and(warp::get()).and(warp::fs::file("domorust.db")))
	// return the corresponding file in webcontent dir
	.or(warp::fs::dir(wwwroot))
}

fn domorust_api_old() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	let get = warp::path("domorust-api")
		.and(warp::get())
		.and(warp::path::full())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(handlers::json_htm_get);
	let post = warp::path("domorust-api")
		.and(warp::post())
		.and(warp::query::<HashMap<String, String>>())
		.and(warp::multipart::form().max_length(500_000))
		.and_then(handlers::json_htm_post);
	get.or(post)
}

fn domorust_api() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	/*crate::server::routing::applications::applications()
	.or(crate::server::routing::login::login())
	.or(crate::server::routing::custom_icons::custom_icons())
	.or(crate::server::routing::devices::devices())
	.or(crate::server::routing::hardwares::hardwares())
	.or(crate::server::routing::infos::infos())
	.or(crate::server::routing::scenes::scenes())
	.or(crate::server::routing::timers::timers())
	.or(crate::server::routing::themes::themes(domorust))
	.or(crate::server::routing::user_variables::user_variables())
	.or(crate::server::routing::users::users())
	.or(crate::server::routing::plans::plans())
	.or(crate::server::routing::settings::settings())
	.or(crate::server::routing::events_scripts::events_scripts())*/
	balanced_or_tree!(
		//crate::server::routing::applications::applications(),
		crate::server::handlers::applications::route_get_applications(),
		crate::server::handlers::applications::route_add_application(),
		crate::server::handlers::applications::route_update_application(),
		crate::server::handlers::applications::route_delete_application(),

		//crate::server::routing::custom_icons::custom_icons(),
		crate::server::handlers::custom_icons::route_get_custom_icons(),
		crate::server::handlers::custom_icons::route_get_custom_icon(),
		crate::server::handlers::custom_icons::route_add_custom_icon(),
		crate::server::handlers::custom_icons::route_update_custom_icon(),
		crate::server::handlers::custom_icons::route_delete_custom_icon(),
		crate::server::handlers::custom_icons::route_get_custom_icon_small_image(),
		crate::server::handlers::custom_icons::route_get_custom_icon_on_image(),
		crate::server::handlers::custom_icons::route_get_custom_icon_off_image(),
		crate::server::handlers::custom_icons::route_get_images(),
		
		//crate::server::routing::devices::devices(),
		crate::server::handlers::devices::route_get_devices(),
		crate::server::handlers::devices::route_get_light_switches(),
		crate::server::handlers::devices::route_get_device(),
		crate::server::handlers::devices::route_add_device(),
		crate::server::handlers::devices::route_update_device(),
		crate::server::handlers::devices::route_delete_device(),
		crate::server::handlers::devices::route_get_device_graph(),
		crate::server::handlers::devices::route_allow_new_devices(),
		crate::server::handlers::devices::route_make_favorite_device(),
		crate::server::handlers::devices::route_send_device_command(),

		//crate::server::routing::events_scripts::events_scripts()
		crate::server::handlers::events_scripts::route_get_events_scripts(),
		crate::server::handlers::events_scripts::route_add_events_script(),
		crate::server::handlers::events_scripts::route_update_events_script(),
		crate::server::handlers::events_scripts::route_delete_events_script(),
		crate::server::handlers::events_scripts::route_get_devices_current_status(),

		//crate::server::routing::hardwares::hardwares(),
		crate::server::handlers::hardwares::route_get_hardwares(),
		crate::server::handlers::hardwares::route_get_hardware(),
		crate::server::handlers::hardwares::route_add_hardware(),
		crate::server::handlers::hardwares::route_update_hardware(),
		crate::server::handlers::hardwares::route_delete_hardware(),

		//crate::server::routing::infos::infos(),
		crate::server::handlers::infos::route_get_version(),
		crate::server::handlers::infos::route_get_uptime(),
		crate::server::handlers::infos::route_get_sun_rise_set(),
		crate::server::handlers::infos::route_get_serial_devices(),
		crate::server::handlers::infos::route_get_hardwaretypes(),
		crate::server::handlers::infos::route_get_switchtypes(),
		crate::server::handlers::infos::route_get_metertypes(),
		crate::server::handlers::infos::route_get_languages(),
		crate::server::handlers::infos::route_get_config(),

		crate::server::routing::login::login(),

		//crate::server::routing::plans::plans(),
		crate::server::handlers::plans::route_get_floorplans(),
		crate::server::handlers::plans::route_get_floorplan(),
		crate::server::handlers::plans::route_get_floorplan_image(),
		crate::server::handlers::plans::route_get_plans(),
		crate::server::handlers::plans::route_get_plan(),
		crate::server::handlers::plans::route_add_plan(),
		crate::server::handlers::plans::route_update_plan(),
		crate::server::handlers::plans::route_delete_plan(),
		crate::server::handlers::plans::route_get_plan_devices(),

		//crate::server::routing::scenes::scenes(),
		crate::server::handlers::scenes::route_get_scenes(),
		crate::server::handlers::scenes::route_add_scene(),
		crate::server::handlers::scenes::route_update_scene(),
		crate::server::handlers::scenes::route_delete_scene(),
		crate::server::handlers::scenes::route_get_scene_activations(),
		crate::server::handlers::scenes::route_get_scene_devices(),
		crate::server::handlers::scenes::route_get_scene_logs(),
		crate::server::handlers::scenes::route_get_scene_timers(),
		crate::server::handlers::scenes::route_add_scene_activation(),
		crate::server::handlers::scenes::route_add_scene_device(),
		crate::server::handlers::scenes::route_add_scene_timer(),

		//crate::server::routing::settings::settings(),
		crate::server::handlers::settings::route_get_settings(),
		crate::server::handlers::settings::route_store_settings(),
		crate::server::handlers::settings::route_get_setting(),
		crate::server::handlers::settings::route_set_setting(),

		//crate::server::routing::themes::themes(domorust),
		crate::server::handlers::themes::route_get_themes(),
		crate::server::handlers::themes::route_get_theme(),
		crate::server::handlers::themes::route_set_theme(),

		//crate::server::routing::timers::timers(),
		crate::server::handlers::timers::route_get_timers(),
		crate::server::handlers::timers::route_get_timer(),
		crate::server::handlers::timers::route_get_device_timers(),
		crate::server::handlers::timers::route_add_device_timer(),
		crate::server::handlers::timers::route_update_timer(),
		crate::server::handlers::timers::route_delete_timer(),
		crate::server::handlers::timers::route_delete_device_timers(),
		crate::server::handlers::timers::route_get_timerplans(),
		crate::server::handlers::timers::route_add_timerplan(),
		crate::server::handlers::timers::route_update_timerplan(),
		crate::server::handlers::timers::route_delete_timerplan(),
		crate::server::handlers::timers::route_duplicate_timerplan(),
		
		//crate::server::routing::user_variables::user_variables(),
		crate::server::handlers::user_variables::route_get_user_variables(),
		crate::server::handlers::user_variables::route_get_user_variable(),
		crate::server::handlers::user_variables::route_add_user_variable(),
		crate::server::handlers::user_variables::route_update_user_variable(),
		crate::server::handlers::user_variables::route_delete_user_variable(),

		//crate::server::routing::users::users(),
		crate::server::handlers::users::route_get_users(),
		crate::server::handlers::users::route_get_user(),
		crate::server::handlers::users::route_add_user(),
		crate::server::handlers::users::route_update_user(),
		crate::server::handlers::users::route_delete_user(),


	)
}

pub(crate) fn redirect_to_https() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::any()
		.and(warp::filters::host::optional())
		.and(warp::filters::path::full())
		.and_then(crate::server::handlers::redirect_to_https)
}