// #![allow(dead_code, unused_variables, unused_imports)]

use wasm_bindgen::JsCast;
use wasm_bindgen_test::wasm_bindgen_test_configure;
use wasm_bindgen_test::*;
use web_sys::{window, HtmlElement};
use weblog::console_log;

use std::cell::Cell;
use std::rc::Rc;

use webru::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn create_element_test() {
    const INNER_HTML: &str = "Testing time";
    const ID: &str = "heading";

    // Creating an html element `h1` with the `create_element()` function
    let h1 = create_element("h1");

    // Setting some attributes
    h1.set_inner_html(INNER_HTML);
    h1.set_id(ID);

    // inserting the element inside the <body>
    body().append_child(&h1).unwrap();

    // Now asserting the newly created element
    assert_eq!(h1.inner_html(), INNER_HTML);
    assert_eq!(h1.id(), ID);
    assert_ne!(h1.inner_html(), "Wrong heading");
    assert_ne!(h1.id(), "wrong id");
}

#[wasm_bindgen_test]
fn get_element_by_id_test() {
    const PARAGRAPH_ID: &str = "little_para";
    const PARAGRAPH_CLASS: &str = "para graph";

    // Creating an html element <p>
    let p = create_element("p");

    // adding some attributes
    p.set_id(PARAGRAPH_ID);
    p.set_class_name(PARAGRAPH_CLASS);

    {
        // getting the element with the `get_element_by_id` without inserting it into the DOM
        let p = get_element_by_id(PARAGRAPH_ID);

        // at this moment, this should be None because the element hasn't been inserted in the <body>
        assert_eq!(p, None);
    }

    // inserting the element into the <body>
    body().append_child(&p).unwrap();

    // getting this element with the `get_element_by_id` function after inserting it into the DOM
    let p = get_element_by_id(PARAGRAPH_ID);

    // asserting
    assert_eq!(p.clone().unwrap().class_name(), PARAGRAPH_CLASS);
    assert_ne!(p, None);
}

#[wasm_bindgen_test]
fn get_elements_by_classname_test() {
    /// classname of the 1st and 2nd <p>
    const PARAGRAPH_CLASS: &str = "sad fsda fasdf";

    /// id of the 1st <p>
    const PARAGRAPH_1_ID: &str = "@%ASF";
    /// id of the 2nd <p>
    const PARAGRAPH_2_ID: &str = "@%ASGAH";

    // Creating 1st html element <p>
    let p1 = create_element("p");

    // adding some attributes
    p1.set_class_name(PARAGRAPH_CLASS);
    p1.set_id(PARAGRAPH_1_ID);

    // Creating 2nd html element <p>
    let p2 = create_element("p");

    // adding some attributes
    p2.set_class_name(PARAGRAPH_CLASS);
    p2.set_id(PARAGRAPH_2_ID);

    {
        // getting the element with the `get_elements_by_classname` function without inserting it into the DOM
        let p = get_elements_by_classname(PARAGRAPH_CLASS);

        assert_eq!(p.length(), 0);
    }

    // inserting the elements into the DOM
    body().append_child(&p1).unwrap();
    body().append_child(&p2).unwrap();

    // getting the element with the `get_elements_by_classname` function after inserting it into the DOM
    let p = get_elements_by_classname(PARAGRAPH_CLASS);

    // asserting
    assert_eq!(p.length(), 2);
    assert_eq!(p.item(0).unwrap().id(), PARAGRAPH_1_ID);
    assert_eq!(p.item(1).unwrap().id(), PARAGRAPH_2_ID);
}

#[wasm_bindgen_test]
fn get_elements_by_classname_inside_vec_test() {
    /// classname of the 1st and 2nd <p>
    const PARAGRAPH_CLASS: &str = "as dfsadfasd  ffa";

    /// id of the 1st <p>
    const PARAGRAPH_1_ID: &str = "@%asdr4";
    /// id of the 2nd <p>
    const PARAGRAPH_2_ID: &str = "@%@%^^GA";

    // Creating 1st html element <p>
    let p1 = create_element("p");

    // adding some attributes
    p1.set_class_name(PARAGRAPH_CLASS);
    p1.set_id(PARAGRAPH_1_ID);

    // Creating 2nd html element <p>
    let p2 = create_element("p");

    // adding some attributes
    p2.set_class_name(PARAGRAPH_CLASS);
    p2.set_id(PARAGRAPH_2_ID);

    {
        // getting the element with the `get_elements_by_classname` function without inserting it into the DOM
        let p = get_elements_by_classname_inside_vec(PARAGRAPH_CLASS);

        assert_eq!(p.len(), 0);
    }

    // inserting the elements into the DOM
    body().append_child(&p1).unwrap();
    body().append_child(&p2).unwrap();

    // getting the element with the `get_elements_by_classname` function after inserting it into the DOM
    let p = get_elements_by_classname_inside_vec(PARAGRAPH_CLASS);

    // asserting
    assert_eq!(p.len(), 2);
    assert_eq!(p[0].id(), PARAGRAPH_1_ID);
    assert_eq!(p[1].id(), PARAGRAPH_2_ID);
}

#[wasm_bindgen_test]
fn query_selector_test() {
    /// id of the <p>
    const PARAGRAPH_ID: &str = "peraid";

    /// class of the <p>
    const PARAGRAPH_CLASS: &str = "perapera";

    // Creating an html element <p>
    let p = create_element("p");

    // adding some attributes
    p.set_id(PARAGRAPH_ID);
    p.set_class_name(PARAGRAPH_CLASS);

    {
        // getting the element with the `query_selector` without inserting it into the DOM
        let p = query_selector(&format!("#{}", PARAGRAPH_ID));

        // at this moment, this should be None because the element hasn't been inserted in the <body>
        assert_eq!(p, None);
    }

    // inserting the element into the <body>
    body().append_child(&p).unwrap();

    // getting the right element with the `query_selector` function after inserting it into the DOM
    let p_right = query_selector(&format!("#{}", PARAGRAPH_ID));

    // geeting the wrong element with the `query_selector` function after inserting it into the DOM
    let p_wrong = query_selector(&format!("#{}", "ewsdfs"));

    // asserting
    assert_eq!(p_right.clone().unwrap().class_name(), PARAGRAPH_CLASS);
    assert_ne!(p_right, p_wrong);
    assert_ne!(p_right, None);
}

#[wasm_bindgen_test]
fn query_selector_all_test() {
    /// classname of the 1st and 2nd <p>
    const PARAGRAPH_CLASS: &str = "sadjfalsdf";

    /// id of the 1st <p>
    const PARAGRAPH_1_ID: &str = "@***>>>";
    /// id of the 2nd <p>
    const PARAGRAPH_2_ID: &str = "!!__<<";

    // Creating 1st html element <p>
    let p1 = create_element("p");

    // adding some attributes
    p1.set_class_name(PARAGRAPH_CLASS);
    p1.set_id(PARAGRAPH_1_ID);

    // Creating 2nd html element <p>
    let p2 = create_element("p");

    // adding some attributes
    p2.set_class_name(PARAGRAPH_CLASS);
    p2.set_id(PARAGRAPH_2_ID);

    {
        // getting the element with the `get_elements_by_classname` function without inserting it into the DOM
        let p = query_selector_all(&format!(".{}", PARAGRAPH_CLASS));

        assert_eq!(p.length(), 0);
    }

    // inserting the elements into the DOM
    body().append_child(&p1).unwrap();
    body().append_child(&p2).unwrap();

    // getting the element with the `get_elements_by_classname` function after inserting it into the DOM
    let p = query_selector_all(&format!(".{}", PARAGRAPH_CLASS));

    // asserting
    assert_eq!(p.length(), 2);
}

#[wasm_bindgen_test]
fn query_selector_all_inside_vec_test() {
    /// classname of the 1st and 2nd <p>
    const PARAGRAPH_CLASS: &str = "cleardldls";

    /// id of the 1st <p>
    const PARAGRAPH_1_ID: &str = "@***>>>";
    /// id of the 2nd <p>
    const PARAGRAPH_2_ID: &str = "!!__<<";

    // Creating 1st html element <p>
    let p1 = create_element("p");

    // adding some attributes
    p1.set_class_name(PARAGRAPH_CLASS);
    p1.set_id(PARAGRAPH_1_ID);

    // Creating 2nd html element <p>
    let p2 = create_element("p");

    // adding some attributes
    p2.set_class_name(PARAGRAPH_CLASS);
    p2.set_id(PARAGRAPH_2_ID);

    {
        // getting the element with the `get_elements_by_classname` function without inserting it into the DOM
        let p = query_selector_all_inside_vec(&format!(".{}", PARAGRAPH_CLASS));

        assert_eq!(p.len(), 0);
    }

    // inserting the elements into the DOM
    body().append_child(&p1).unwrap();
    body().append_child(&p2).unwrap();

    // getting the element with the `get_elements_by_classname` function after inserting it into the DOM
    let p = query_selector_all_inside_vec(&format!(".{}", PARAGRAPH_CLASS));

    // asserting
    assert_eq!(p.len(), 2);
}

#[wasm_bindgen_test]
fn document_test() {
    assert_eq!(document(), window().unwrap().document().unwrap());
}

#[wasm_bindgen_test]
fn location_test() {
    assert_eq!(
        location(),
        window().unwrap().document().unwrap().location().unwrap()
    );
}

#[wasm_bindgen_test]
fn url_test() {
    // For this test, my url is http://127.0.0.1:8000/. Your might be different.

    console_log!("The value of `url()` for your website is: ", url());

    assert_eq!(
        url(),
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .url()
            .unwrap()
    );
}

#[wasm_bindgen_test]
fn body_test() {
    let body = body();

    assert_eq!(body, window().unwrap().document().unwrap().body().unwrap());
    assert_eq!(body.tag_name(), "BODY");
}

#[wasm_bindgen_test]
fn domain_name_test() {
    // In my case the domain name is 127.0.0.1
    assert_eq!(domain_name(), "127.0.0.1");

    console_log!(
        "The value of `domain_name()` for your website is: ",
        domain_name()
    )
}

#[wasm_bindgen_test]
fn path_name_test() {
    assert_eq!(path_name(), "/")
}

#[wasm_bindgen_test]
fn reload_test() {
    // NOTE: You need to test it manually

    // creating a <button> for reloading the page
    let button = create_element("button")
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .clone();

    // adding some attributes
    button.set_inner_html("reload page");

    // callback/callback when the button will be clicked
    let callback = callback(|| {
        // reloading the page
        // if the page is reloaded, then this test is passed
        reload();
    });

    // setting `onclick` property to the <button>
    button.set_onclick(Some(callback.as_ref().dyn_ref().unwrap()));

    callback.forget();

    // creating a <p> for showing a message
    let p = create_element("p");

    // adding innerHtml
    p.set_inner_html("Click this button, if the website reloads after clicking, the test `reload_test` has passed");

    // <hr>
    let hr1 = create_element("hr");
    let hr2 = create_element("hr");

    // inserting the <hr> into the DOM
    body().append_child(&hr1).unwrap();
    // inserting the <p> into the DOM
    body().append_child(&p).unwrap();
    // inserting the <button> into the DOM
    body().append_child(&button).unwrap();
    // inserting the <hr> into the DOM
    body().append_child(&hr2).unwrap();
}

#[wasm_bindgen_test]
fn alert_test() {
    alert("If you see this message, the test `alert_test` has passed");
}

#[wasm_bindgen_test]
fn prompt_test() {
    prompt("If you see this prompt, the test `prompt_test` has passed");
}

#[wasm_bindgen_test]
fn set_timeout_clear_timeout_test() {
    // NOTE: You need to test it manually

    #[derive(Debug)]
    struct SetTimeoutId {
        id: Cell<Option<i32>>,
    }

    const HIDE_DURATION: u16 = 4000; // milli seconds
    let settimeout_id = Rc::new(SetTimeoutId {
        id: Cell::new(None),
    });

    // When this button is clicked, it will be hidden for `HIDE_DURATION` sec, and after that it will be shown again
    let hide_button = create_element("button")
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .clone();

    hide_button.set_inner_html(&format!(
        "Hide this button for {} seconds",
        HIDE_DURATION / 1000
    ));

    // When this button is clicked, if the `hide_button` is hidden, it will be hide forever. Click this button only if the `hide_button` is hidden.
    let cancel_hide_button = create_element("button")
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .clone();

    cancel_hide_button.set_inner_html("Hide that button forever");

    // callback/callback for onclick event for `hide_button`
    let cb = callback({
        let hide_button = hide_button.clone();
        let settimeout_id = Rc::clone(&settimeout_id.clone());

        move || {
            // hiding
            hide_button.style().set_property("display", "none").unwrap();

            let settimeout_id = Rc::clone(&settimeout_id.clone());
            let hide_button = hide_button.clone();

            let timeout_id = set_timeout(
                move || {
                    // display
                    hide_button
                        .style()
                        .set_property("display", "block")
                        .unwrap();
                },
                HIDE_DURATION as i32,
            )
            .unwrap();

            settimeout_id.id.set(Some(timeout_id));
        }
    });

    // setting `onclick` event
    hide_button.set_onclick(Some(cb.as_ref().dyn_ref().unwrap()));

    cb.forget();

    // callback/callback for onclick event for `cancel_hide_button`
    let cb = callback({
        move || {
            let settimeout_id = Rc::clone(&settimeout_id.clone());
            console_log!(format!("set timeout: {:?}", settimeout_id));

            if let Some(id) = settimeout_id.id.get() {
                clear_timeout(id);

                // setting its value to None
                settimeout_id.id.set(None);
            }
        }
    });

    cancel_hide_button.set_onclick(Some(cb.as_ref().dyn_ref().unwrap()));

    cb.forget();

    // <hr>
    let hr1 = create_element("hr");
    let hr2 = create_element("hr");

    // inserting the <hr> into the DOM
    body().append_child(&hr1).unwrap();
    // inserting `hide_button` into the DOM
    body().append_child(&hide_button).unwrap();
    // inserting `cancel_hide_button` into the DOM
    body().append_child(&cancel_hide_button).unwrap();
    // inserting the <hr> into the DOM
    body().append_child(&hr2).unwrap();
}

#[wasm_bindgen_test]
fn set_interval_clear_interval_test() {
    #[derive(Debug)]
    struct SetIntervalId {
        id: Cell<Option<i32>>,
    }

    #[derive(Debug)]
    struct Counter {
        count: Cell<usize>,
    }

    const INTERVAL_SPEED: u32 = 1000; // milliseconds
    let setinterval_id = Rc::new(SetIntervalId {
        id: Cell::new(None),
    });
    let counter = Rc::new(Counter {
        count: Cell::new(0),
    });

    // <p> element for displaying the couter
    let counter_element = create_element("p");
    counter_element.set_inner_html("0");

    // <button> element for start the counter
    let start_counter_button = create_element("button")
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .clone();
    start_counter_button.set_inner_html("Start counter");

    // <button> element for stop the counter
    let stop_counter_button = create_element("button")
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .clone();
    stop_counter_button.set_inner_html("Stop counter");

    // callback for `start_counter_button`
    let cb = callback({
        let counter_element = counter_element.clone();
        let setinterval_id = Rc::clone(&setinterval_id);

        move || {
            let counter = Rc::clone(&counter);

            let interval_id = set_interval(
                {
                    let counter_element = counter_element.clone();

                    move || {
                        counter.count.set(counter.count.get() + 1);
                        counter_element.set_inner_html(&format!("{}", counter.count.get()));
                    }
                },
                INTERVAL_SPEED as i32,
            );
            setinterval_id.id.set(Some(interval_id.unwrap()));
        }
    });

    start_counter_button.set_onclick(Some(cb.as_ref().dyn_ref().unwrap()));

    cb.forget();

    let cb = callback({
        move || {
            if let Some(id) = setinterval_id.id.get() {
                clear_interval(id)
            }
            setinterval_id.id.set(None);
        }
    });

    stop_counter_button.set_onclick(Some(cb.as_ref().dyn_ref().unwrap()));

    cb.forget();

    // <hr>
    let hr1 = create_element("hr");
    let hr2 = create_element("hr");

    // inserting the <hr> into the DOM
    body().append_child(&hr1).unwrap();
    // inserting `counter_element` into the DOM
    body().append_child(&counter_element).unwrap();
    // inserting `hide_button` into the DOM
    body().append_child(&start_counter_button).unwrap();
    // inserting `cancel_hide_button` into the DOM
    body().append_child(&stop_counter_button).unwrap();
    // inserting the <hr> into the DOM
    body().append_child(&hr2).unwrap();
}
