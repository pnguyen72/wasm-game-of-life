use std::fmt::Debug;
use web_sys::console;

pub trait JsResult<T> {
    /** Like `Result::ok`, but also logs the error to console. */
    fn js_ok(self) -> Option<T>;
}

macro_rules! console_error {
    ($($t:tt)*) => (console::error_1(&format_args!($($t)*).to_string().into()))
}

impl<T, E: Debug> JsResult<T> for Result<T, E> {
    fn js_ok(self) -> Option<T> {
        match self {
            Ok(a) => Some(a),
            Err(e) => {
                console_error!("Error: {e:?}");
                None
            }
        }
    }
}
