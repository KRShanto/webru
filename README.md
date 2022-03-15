
# Webru

Frequently used javascript functions for `Rust` and `WebAssembly`

`webru` is a binding for [web-sys](https://crates.io/crates/web-sys)

This crate assumes that you will only use this crate inside browser. Not any other javascript runtime such as `Node.js`

## Example

```rust

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};
use weblog::console_log;
use webru::{alert, body, callback, create_element, get_element_by_id, reload, set_timeout};

// javascript `Window.alert()` method
alert("You are in danger!");

// getting the <body> tag/element
let body: HtmlElement = body();

// creating a <h1> tag using javascript `document.createElement()` method
// This function will return the newly created element/tag
let h1: Element = create_element("h1");

// adding some attributes to <h1> tag
h1.set_id("heading-1"); // html `id` property
h1.set_class_name("heading"); // html `class` property`
h1.set_inner_html("Welcome to my website");

// pushing the <h1> tag into the DOM
body.append_child(&h1);

// This closure will be called the <button> will be clicked
let cb: Closure<dyn Fn()> = callback(|| {
    // javascript `console.log()` method
    console_log!("You clicked the button");
});

// creating a <button> element
let button: HtmlElement = create_element("button")
    .dyn_ref::<HtmlElement>()
    .unwrap()
    .clone();

// adding some attributes to <button> tag
button.set_inner_html("Click me"); // innerHTML property
button.set_onclick(Some(cb.as_ref().dyn_ref().unwrap())); // onclick event

cb.forget();

// pushing the <button> tag into the DOM
body.append_child(&button);

// javascript `setTimeout()` method
set_timeout(
    || {
        // javascript `console.log()` method
        console_log!("Time has completed");

        // reload the page
        // javascript `location.reload()` method
        reload();
    },
    10000, // 10 seconds
);

// getting the <h1> element from the DOM
// javascript `document.getElementById()` method
let h1_from_dom: Option<Element> = get_element_by_id("heading-1");

assert_eq!(h1_from_dom.unwrap(), h1);

```
