
# Webru

Frequently used javascript functions for `Rust` and `WebAssembly`

`webru` is a binding for [web-sys](https://crates.io/crates/web-sys)

*This crate assumes that you will only use this crate inside browser. Not any other javascript runtime such as `Node.js`*

## Usage

### Using some global objects and functions

```rust
use webru::{alert, body, create_element, prompt};
use web_sys::{Element, HtmlElement};

// get the body
let body: HtmlElement = body();

// show an alert
alert("We use cookies to improve your experience.");

// Get the name from the user using "prompt" method
let name: Option<String> = prompt("What is your name?");

// creating a <h1> tag using javascript `document.createElement()` method
// This function will return the newly created element/tag
let h1: Element = create_element("h1");

// adding some attributes to <h1> tag
h1.set_id("heading-1"); // html `id` property
h1.set_class_name("heading"); // html `class` property`

match name {
    Some(name) if name.len() > 0 => {
        // if user entered a name then set the inner html of <h1> tag
        h1.set_inner_html(&format!("Hello {}", name));
    }
    _ => h1.set_inner_html("Hello World!"),
}

// pushing the <h1> tag into the DOM
body.append_child(&h1).unwrap();

```

### Using `setTimeout` and `setInterval`

```rust
use std::cell::Cell;
use weblog::console_log;

{
    use crate::{clear_interval, set_interval, set_timeout};

    let number = Cell::new(0);

    // javascript `setInterval()` method
    let interval_id: i32 = set_interval(
        move || {
            console_log!("Number: {}", number.get());

            // increment the number after every second
            number.set(number.get() + 1);
        },
        1000, // 1 second
    )
    .unwrap();

    // javascript `setTimeout()` method
    set_timeout(
        move || {
            console_log!("Stopping interval");

            // stop the interval after 5 seconds
            clear_interval(interval_id);
        },
        10000, // 10 seconds
    )
    .unwrap();
}
// Alternatively, you can use `Timeout` and `Interval` structs instead of `set_timeout` and `set_interval`
{
    use crate::{Interval, Timeout};

    let number: Cell<i32> = Cell::new(0);

    // javascript `setInterval()` method
    let interval: Interval = Interval::start(
        move || {
            console_log!("Number: {}", number.get());

            // increment the number after every second
            number.set(number.get() + 1);
        },
        1000, // 1 second
    );

    // javascript `setTimeout()` method
    Timeout::start(
        move || {
            console_log!("Stopping interval");

            // stop the interval after 5 seconds
            interval.stop();
        },
        10000, // 10 seconds
    );
}
```

### Using selectors

```rust
use webru::{body, create_element, get_element_by_id, get_elements_by_classname_inside_vec};
use web_sys::{Element, HtmlElement};

let class_name = "active-element";

// creating some elements with the class name "active-element" adding it to the DOM
{
    let h1_1: Element = create_element("h1");
    h1_1.set_class_name(class_name);
    h1_1.set_id("heading-1");
    h1_1.set_inner_html("I am heading 1");

    let h1_2: Element = create_element("h1");
    h1_2.set_class_name(class_name);
    h1_2.set_id("heading-2");
    h1_2.set_inner_html("I am heading 2");

    let h1_3: Element = create_element("h1");
    h1_3.set_class_name(class_name);
    h1_3.set_id("heading-3");
    h1_3.set_inner_html("I am heading 3");

    // render the elements into the DOM
    let body: HtmlElement = body();
    body.append_child(&h1_1).unwrap();
    body.append_child(&h1_2).unwrap();
    body.append_child(&h1_3).unwrap();
}
// selecting all elements with the class name "active-element"
{
    let elements: Vec<Element> = get_elements_by_classname_inside_vec(class_name);

    // iterating over the elements
    let mut number = 0;

    for element in elements {
        number += 1;    

        assert_eq!(element.class_name(), class_name);
        assert_eq!(element.id(), format!("heading-{}", number));
    }
}
// selecting the element with the id "heading-3"
{
    let element: Option<Element> = get_element_by_id("heading-3");

    assert_eq!(element.unwrap().class_name(), class_name);
}
```

### Using javascript callback

```rust
use webru::{body, callback, create_element};
use std::cell::Cell;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use weblog::console_log;

let count = Cell::new(0);

// "onclick" event handler for the <button> tag
let onclick = callback(move || {
    count.set(count.get() + 1);
    console_log!(format!("Button clicked {} times", count.get()));
});

// get the body object
let body: HtmlElement = body();

// createing a <button> tag
let button: HtmlElement = create_element("button")
    .dyn_ref::<HtmlElement>()
    .unwrap()
    .clone();

// adding some attributes to <button> tag
button.set_inner_html("Just click me"); // innerHTML property
button.set_onclick(Some(onclick.as_ref().dyn_ref().unwrap())); // onclick event

onclick.forget();

// pushing the <button> tag into the DOM
body.append_child(&button).unwrap();
```

### Using media query

```rust
use webru::media_query;
use weblog::console_log;

media_query(
    || {
        // if the media query matches, this closure will be called
        console_log!("Your window is under 1001px")
    },
    || {
        // if the media query doesn't match, this closure will be called
        console_log!("Your window is over 1000px")
    },
    1000,
);
```

## Warning

This project is not well tested. So make sure to test every functions from this crate you used in your project before going to production.

## Contributions

Your valuable PRs and Issues are welcome. Note that all the contribution submitted by you, shall be licensed as MIT or APACHE 2.0 at your choice, without any additional terms or conditions.
