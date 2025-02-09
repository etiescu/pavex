use pavex::blueprint::{constructor::Lifecycle, Blueprint};
use pavex::f;
use pavex::request::body::BodySizeLimit;

pub fn body_size_limit() -> BodySizeLimit {
    BodySizeLimit::Disabled
}

pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();
    bp.constructor(f!(crate::body_size_limit), Lifecycle::Singleton);
    bp.route(
        pavex::blueprint::router::GET,
        "/no_limit",
        f!(crate::no_limit::handler),
    );
    bp
}
