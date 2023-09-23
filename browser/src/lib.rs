#![allow(non_upper_case_globals)]

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use prelude::*;
// use serde::de::DeserializeOwned;
use shared::prelude::*;
use web_sys::HtmlElement;

#[cfg(feature = "dev")]
mod dev;
mod extensions;
mod prelude;

// #[wasm_bindgen]
// extern "C" {
//     // JS alert function.
//     fn alert(s: &str);
// }

// Callable from JS.
// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello!");
// }

// Called when the wasm module is instantiated.
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    #[cfg(feature = "dev")]
    dev::main();

    // load_more_poems_on_link_click();

    mount_component(show_if_scrolled::name, hydrate_show_if_scrolled);
    mount_component(parallax::name, hydrate_parallax);

    Ok(())
}

// fn mount_component<Props: DeserializeOwned>(
fn mount_component(component_name: &'static str, hydrate: fn(HtmlElement)) {
    // console::log!("Mounting components named:", component_name);

    let window = web_sys::window().expect("web_sys::window() failed.");
    let document = window.document().expect("window.document() failed.");

    // let selector = format!("[data-browser-component-name=\"{}\"]", component_name);
    let selector = format!(".{}", component_name).replace(':', "\\:");
    let nodes = document
        .query_selector_all(&selector)
        .expect("document.query_selector_all() failed.");

    for index in 0..nodes.length() {
        let node = nodes.get(index).expect("nodes.get() failed.");
        let element: HtmlElement = node.dyn_into().expect("element.dyn_into() failed.");
        // let serialized_props = element
        //     .get_attribute("data-browser-component-props")
        //     .expect("element.get_attribute() failed.");
        // let deserialized_props: Props =
        //     serde_json::from_str(&serialized_props).expect("serde_json::from_str() failed.");
        // hydrate(element, deserialized_props);
        hydrate(element);
    }
}

// fn load_more_poems_on_link_click() {
//     let window = web_sys::window().expect("web_sys::window() failed.");
//     let document = window.document().expect("window.document() failed.");
//     let load_more_links = document
//         .query_selector_all(".script\\:load-more-poems")
//         .expect("document.query_selector_all() failed.");

//     console::log!(
//         "Registering event listeners for {} load more links.",
//         load_more_links.length()
//     );

//     for index in 0..load_more_links.length() {
//         let link = load_more_links
//             .get(index)
//             .expect("load_more_links.get() failed.");

//         EventListener::new(&link, "click", move |_: &Event| {
//             console::log!("Clicked load more poems link.");
//         })
//         .forget();
//     }
// }
