use leptos::prelude::*;
use leptos_use::use_media_query;

const MOBILE_BREAKPOINT: u32 = 768;

pub fn use_is_mobile() -> Signal<bool> {
    use_media_query(format!("(max-width: {}px)", MOBILE_BREAKPOINT - 1))
}
