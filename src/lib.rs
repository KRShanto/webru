use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, Document, Element, HtmlCollection, HtmlElement, Location, Node, NodeList};

// exporting functions
pub use global::*;
pub use selectors::*;
pub use timer::*;

mod selectors {
    use super::*;

    // tested
    pub fn get_element_by_id(id: &str) -> Option<Element> {
        document().get_element_by_id(id)
    }

    // tested
    pub fn get_elements_by_classname(classname: &str) -> HtmlCollection {
        document().get_elements_by_class_name(classname)
    }

    // tested
    pub fn get_elements_by_classname_inside_vec(classname: &str) -> Vec<Element> {
        let mut i = 0;
        let mut vec: Vec<Element> = Vec::new();

        while let Some(element) = get_elements_by_classname(classname).item(i) {
            vec.push(element);
            i += 1;
        }

        vec
    }

    // tested
    pub fn query_selector(selector: &str) -> Option<Element> {
        document().query_selector(selector).unwrap()
    }

    // tested
    pub fn query_selector_all(selector: &str) -> NodeList {
        document().query_selector_all(selector).unwrap()
    }

    // tested
    pub fn query_selector_all_inside_vec(selector: &str) -> Vec<Node> {
        let mut i = 0;
        let mut vec: Vec<Node> = Vec::new();

        while let Some(element) = query_selector_all(selector).item(i) {
            vec.push(element);
            i += 1;
        }

        vec
    }

    // tested
    pub fn create_element(element_name: &str) -> Element {
        document().create_element(element_name).unwrap()
    }
}

mod timer {
    use super::*;

    pub fn set_timeout<T: 'static>(handler: T, timeout: i32) -> Result<i32, JsValue>
    where
        T: Fn(),
    {
        let window = window().unwrap();

        let callback = Closure::wrap(Box::new(handler) as Box<dyn Fn()>);

        let result = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            timeout,
        );

        callback.forget();
        // TODO: I will mension some details about this code and why `.forget()` is important when I will create documentation

        result
    }

    pub fn clear_timeout(timeout: i32) {
        let window = window().unwrap();

        window.clear_timeout_with_handle(timeout);
    }

    pub fn set_interval<T: 'static>(handler: T, timeout: i32) -> Result<i32, JsValue>
    where
        T: Fn(),
    {
        let window = window().unwrap();

        let callback = Closure::wrap(Box::new(handler) as Box<dyn Fn()>);

        let result = window.set_interval_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            timeout,
        );

        callback.forget();
        // TODO: I will mension some details about this code and why `.forget()` is important when I will create documentation

        result
    }

    pub fn clear_interval(timeout: i32) {
        let window = window().unwrap();

        window.clear_interval_with_handle(timeout);
    }
}

mod global {
    use super::*;

    // tested
    pub fn document() -> Document {
        let window = window().unwrap();
        window.document().unwrap()
    }
    // tested
    pub fn location() -> Location {
        document().location().unwrap()
    }

    // tested
    pub fn url() -> String {
        location().href().unwrap()
    }

    // tested
    pub fn body() -> HtmlElement {
        document().body().unwrap()
    }

    // tested
    pub fn domain_name() -> String {
        location().hostname().unwrap()
    }

    // tested
    pub fn path_name() -> String {
        location().pathname().unwrap()
    }

    // tested
    pub fn reload() {
        location().reload().unwrap()
    }

    // tested
    pub fn alert(msg: &str) {
        window().unwrap().alert_with_message(msg).unwrap()
    }

    // tested
    pub fn prompt(msg: &str) -> Option<String> {
        window().unwrap().prompt_with_message(msg).unwrap() // if None: it means the user clicked "cancel" button, else it means the user clicked "OK" button
    }

    // this doesn't need to be tested
    pub fn callback<T: 'static>(handler: T) -> Closure<dyn Fn()>
    where
        T: Fn(),
    {
        Closure::wrap(Box::new(handler) as Box<dyn Fn()>)
    }
}
