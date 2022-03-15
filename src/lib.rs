//! Frequently used javascript functions for `Rust` and `WebAssembly`
//!
//! `webru` is a binding for [web-sys](https://crates.io/crates/web-sys)
//!
//! This crate assumes that you will only use this crate inside browser. Not any other javascript runtime such as `Node.js`
//!

#![warn(missing_docs)]
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, Document, Element, HtmlCollection, HtmlElement, Location, Node, NodeList};

// exporting functions
pub use global::*;
pub use selectors::*;
pub use timer::*;

mod selectors {
    use super::*;

    /// Javascript [`document.getElementById`](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementById) method
    ///
    /// It returns an [`Element`](https://docs.rs/web-sys/0.3.56/web_sys/struct.Element.html) object describing the DOM element object matching the specified ID, or `None` if no matching element was found in the document.
    ///
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the element to locate. The ID is case-sensitive string which is unique within the document; only one element may have any given ID.
    ///
    ///
    /// # Panics
    ///
    /// This function will panic if you try to call this outside of the web such as `node.js` runtime
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use webru::{body, create_element, get_element_by_id};
    ///
    /// // `id` property for the new <p> element
    /// const ID: &str = "RandomId#$4";
    ///
    /// // Create a <p> element
    /// let p = create_element("p");
    ///
    /// // setting the id property
    /// p.set_id(ID);
    ///
    /// // Try to get the <p> element before inserting into the DOM
    /// let fetched_p = get_element_by_id(ID);
    ///
    /// // it will give None
    /// assert_eq!(fetched_p, None);
    ///
    /// // inserting the <p> into the DOM
    /// body().append_child(&p).unwrap();
    ///
    /// // Try to get the <p> element after inserting into the DOM
    /// let fetched_p = get_element_by_id(ID);
    ///
    /// // it will give Some(_)
    /// assert_eq!(fetched_p, Some(p));
    ///
    ///
    /// ```
    pub fn get_element_by_id(id: &str) -> Option<Element> {
        document().get_element_by_id(id)
    }

    /// Javascript [`document.getElementsByClassName`](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByClassName) method
    ///
    /// It returns An [`HTMLCollection`](https://docs.rs/web-sys/0.3.56/web_sys/struct.HtmlCollection.html) object. A collection of elements with the specified class name.
    ///
    /// The elements are sorted as they appear in the document.  
    ///
    ///
    /// # Arguments
    ///
    /// * `classname` - The class name of the elements.
    ///
    ///
    /// # Panics
    ///
    /// This function will panic if you try to call this outside of the web such as `node.js` runtime
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use webru::{body, create_element, get_elements_by_classname};
    ///
    /// // `class` property for the new <p> element
    /// const CLASS: &str = "RandomClass#$4";
    ///
    /// // Create a <p> element
    /// let p = create_element("p");
    ///
    /// // setting the class property
    /// p.set_class_name(CLASS);
    ///
    /// // inserting the <p> into the DOM
    /// body().append_child(&p).unwrap();
    ///
    /// // Get the <p> element
    /// let fetched_p = get_elements_by_classname(CLASS);
    ///
    /// // because in this example I've pushed one element, so its length should be 1
    /// assert_eq!(fetched_p.length(), 1);
    ///
    /// // Getting the 1st element of the `fetched_p`
    /// let fetched_p_0 = fetched_p.item(0).unwrap();
    ///
    /// assert_eq!(fetched_p_0.tag_name(), "P");
    /// assert_eq!(fetched_p_0.class_name(), CLASS);
    ///
    /// ```
    pub fn get_elements_by_classname(classname: &str) -> HtmlCollection {
        document().get_elements_by_class_name(classname)
    }

    /// Javascript [`document.getElementsByClassName`](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByClassName) method
    ///
    /// This function does the same thing as the [`get_elements_by_classname`] function does.
    ///
    /// But instead of returning [`HtmlCollection`] it returns [`Vec<Element>`]
    ///
    /// Behind the scene it uses [`get_elements_by_classname`] and converts [`HtmlCollection`] into [`Vec<Element>`]
    ///
    ///
    /// # Panics
    ///
    /// This function will panic if you try to call this outside of the web such as `node.js` runtime
    ///
    /// [`HtmlCollection`]: <https://docs.rs/web-sys/0.3.56/web_sys/struct.HtmlCollection.html>
    /// [`Vec<Element>`]: <https://docs.rs/web-sys/0.3.56/web_sys/struct.Element.html>
    ///
    pub fn get_elements_by_classname_inside_vec(classname: &str) -> Vec<Element> {
        let mut i = 0;
        let mut vec: Vec<Element> = Vec::new();

        while let Some(element) = get_elements_by_classname(classname).item(i) {
            vec.push(element);
            i += 1;
        }

        vec
    }

    /// Javascript [`document.querySelector`](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelector) method
    ///    
    /// It returns an [`Element`](https://docs.rs/web-sys/0.3.56/web_sys/struct.Element.html) object representing the first element in the document that matches the specified set of [`CSS selectors`](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Selectors), or `None` is returned if there are no matches.
    ///
    ///     
    /// # Arguments
    ///
    /// * `selector` - A `&str` containing one or more selectors to match. This string must be a valid CSS selector string; if it isn't, it will be `panic`
    ///
    ///
    /// # Panics
    ///
    /// * This function will panic if you try to call this outside of the web such as `node.js` runtime
    ///  
    /// * This function will panic if the `selector` is not a valid CSS selector
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use webru::{body, create_element, query_selector};
    ///
    /// // id of the <p>
    /// const PARAGRAPH_ID: &str = "asdfsd";
    ///
    /// // class of the <p>
    /// const PARAGRAPH_CLASS: &str = "sadfsaa";
    ///
    /// // Creating an html element <p>
    /// let p = create_element("p");
    ///
    /// // adding attributes
    /// p.set_id(PARAGRAPH_ID);
    /// p.set_class_name(PARAGRAPH_CLASS);
    ///
    /// // inserting the element into the <body>
    /// body().append_child(&p).unwrap();
    ///
    /// // getting the right element with the `query_selector` function with its `id`
    /// let p_right_from_id = query_selector(&format!("#{}", PARAGRAPH_ID));
    ///
    /// // getting the right element with the `query_selector` function with its `class`
    /// let p_right_from_class = query_selector(&format!(".{}", PARAGRAPH_CLASS));
    ///
    /// // getting the wrong element with the `query_selector` function with its `id`
    /// let p_wrong = query_selector(&format!("#{}", "ewsdfs"));
    ///
    /// assert_eq!(p_right_from_id.clone().unwrap(), p);
    /// assert_eq!(
    ///     p_right_from_id.clone().unwrap(),
    ///     p_right_from_class.clone().unwrap()
    /// );
    /// assert_ne!(p_right_from_class, p_wrong);
    /// assert_ne!(p_right_from_class, None);
    ///
    /// ```
    pub fn query_selector(selector: &str) -> Option<Element> {
        document().query_selector(selector).unwrap()
    }

    /// Javascript [`document.querySelectorAll`](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelectorAll) method
    ///
    ///
    /// # Panics
    ///
    /// * This function will panic if you try to call this outside of the web such as `node.js` runtime
    ///  
    /// * This function will panic if the `selector` is not a valid CSS selector
    pub fn query_selector_all(selector: &str) -> NodeList {
        document().query_selector_all(selector).unwrap()
    }

    /// Javascript [`document.querySelectorAll`](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelectorAll) method
    ///
    /// This function does the same thing as the [`query_selector_all`] function does.
    ///
    /// But instead of returning [`NodeList`] it returns [`Vec<Node>`]
    ///
    /// Behind the scene it uses [`query_selector_all`] and converts [`NodeList`] into [`Vec<Node>`]
    ///
    /// [`NodeList`]: <https://docs.rs/web-sys/0.3.56/web_sys/struct.NodeList.html>
    /// [`Node`]: <https://docs.rs/web-sys/0.3.56/web_sys/struct.Node.html>
    /// [`Vec<Node>`]: <https://docs.rs/web-sys/0.3.56/web_sys/struct.Node.html>
    ///
    ///
    /// # Panics
    ///
    /// * This function will panic if you try to call this outside of the web such as `node.js` runtime
    ///  
    /// * This function will panic if the `selector` is not a valid CSS selector
    ///
    pub fn query_selector_all_inside_vec(selector: &str) -> Vec<Node> {
        let mut i = 0;
        let mut vec: Vec<Node> = Vec::new();

        while let Some(element) = query_selector_all(selector).item(i) {
            vec.push(element);
            i += 1;
        }

        vec
    }
}

mod timer {
    use super::*;

    /// Javascript [`setTimeout()`](https://developer.mozilla.org/en-US/docs/Web/API/setTimeout) function
    ///
    /// This function sets a timer which executes a function or specified piece of code once the timer expires.
    ///
    /// It returns the ID of this timer which can be used with [`clearTimeout()`](https://developer.mozilla.org/en-US/docs/Web/API/clearTimeout) to cancel the timer
    ///
    ///
    /// # Arguments
    ///
    /// * `handler` - The Rust closure to execute
    ///
    /// * `timeout` - Tumber of milliseconds to wait before executing the code in `handler`.
    ///
    ///
    /// # Panics
    ///
    /// This function will panic if you try to call this outside of the web such as `node.js` runtime
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use webru::{set_timeout};
    /// use weblog::console_log;
    ///
    /// set_timeout(
    ///      // This closure will execute after 4 seconds
    ///     || {
    ///         console_log!("Hello world");
    ///     },
    ///     4000,
    /// )
    /// .unwrap();
    ///
    /// let author = "Shanto".to_string();
    ///
    /// set_timeout(
    ///     // This closure will execute after 5 seconds
    ///     move || {
    ///         if author == "Shanto".to_string() {
    ///             console_log!("Welcome Shanto");
    ///         } else {
    ///             console_log!("We don't know you")
    ///         }
    ///     },
    ///     5000,
    /// )
    /// .unwrap();
    /// ```
    ///
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

        result
    }

    /// Javascript [`clearTimeout()`](https://developer.mozilla.org/en-US/docs/Web/API/clearTimeout) function
    ///
    /// This function cancels a timeout previously established by calling javascript's [`setTimeout()`] fucntion.
    ///
    /// # Arguments
    ///
    /// * `timeout_id` - The identifier of the timeout you want to cancel. This ID was returned by the corresponding call to [`setTimeout()`]
    ///
    ///
    /// # Panics
    ///
    /// This function will panic if you try to call this outside of the web such as `node.js` runtime
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use weblog::console_log;
    /// use webru::{clear_timeout, set_timeout};
    ///
    /// let timeout_id = set_timeout(
    ///     // This closure will not execute because we are clearing this timeout before this closure gets called
    ///     || {
    ///         console_log!("Hello world");
    ///     },
    ///     4000,
    /// )
    /// .unwrap();
    ///
    /// // Clearing the timeout
    /// clear_timeout(timeout_id)
    /// ```
    ///
    /// [`setTimeout()`]: <https://developer.mozilla.org/en-US/docs/Web/API/setTimeout>
    pub fn clear_timeout(timeout_id: i32) {
        let window = window().unwrap();

        window.clear_timeout_with_handle(timeout_id);
    }

    /// Javascript [`setInterval()`](https://developer.mozilla.org/en-US/docs/Web/API/setInterval) method
    ///
    ///  The `setInterval()` method, offered on the [`Window`](https://developer.mozilla.org/en-US/docs/Web/API/Window) and [`Worker`](https://developer.mozilla.org/en-US/docs/Web/API/Worker) interfaces, repeatedly calls a function or executes a code snippet, with a fixed time delay between each call.
    ///
    /// This method returns an interval ID which uniquely identifies the interval, so you can remove it later by calling javascript's [`clearInterval()`] function or [`clear_interval`] function
    ///
    ///
    /// # Arguments
    ///
    /// * `handler` - A Rust closure to be executed every `timeout` milliseconds. The first execution happens after `timeout` milliseconds.
    ///
    /// * `timeout` - The execution interval in milliseconds. 1000 milliseconds == 1 second    
    ///
    ///
    /// # Panics
    ///
    /// This function will panic if you try to call this outside of the web such as `node.js` runtime
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use std::cell::Cell;    
    /// use weblog::console_log;
    /// use webru::set_interval;
    ///
    /// let counter: Cell<usize> = Cell::new(0);
    ///
    /// set_interval(
    ///     // This closure will execute after every 2 secends
    ///     move || {
    ///         console_log!("The counter: ", counter.get());
    ///         counter.set(counter.get() + 1);
    ///     },
    ///     2000,
    /// )
    /// .unwrap();
    /// ```
    ///
    /// [`clearInterval()`]: <https://developer.mozilla.org/en-US/docs/Web/API/clearInterval>
    ///
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

    /// Javascript [`clearInterval()`] function
    ///
    /// The global `clearInterval()` method cancels a timer, repeating action which was previously established by a call to [`setInterval()`]
    ///
    ///
    /// # Arguments
    ///
    /// * `timeout` - The identifier of the repeated action you want to cancel. This ID was returned by the corresponding call to [`setInterval()`]
    ///
    ///     
    /// # Panics
    ///
    /// This function will panic if you try to call this outside of the web such as `node.js` runtime
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use std::cell::Cell;
    ///
    /// use weblog::console_log;
    /// use webru::{clear_interval, set_interval};
    ///
    /// let counter: Cell<usize> = Cell::new(0);
    ///
    /// let interval_id = set_interval(
    ///     // This closure will not execute because we are clearing this interval before this closure gets called
    ///     move || {
    ///         console_log!("The counter: ", counter.get());
    ///         counter.set(counter.get() + 1);
    ///     },
    ///     2000,
    /// )
    /// .unwrap();
    ///
    /// // Clearing the interval
    /// clear_interval(interval_id)
    /// ```
    ///
    /// [`setInterval()`]: <https://developer.mozilla.org/en-US/docs/Web/API/setInterval>
    /// [`clearInterval()`]: <https://developer.mozilla.org/en-US/docs/Web/API/clearInterval>
    ///
    pub fn clear_interval(timeout: i32) {
        let window = window().unwrap();

        window.clear_interval_with_handle(timeout);
    }
}

mod global {
    use super::*;

    /// Get the [`web_sys::Document`](https://docs.rs/web-sys/0.3.56/web_sys/struct.Document.html) Object
    ///
    /// This function is equivalent to javascript's `document` property
    ///
    ///     
    /// # Panics
    ///
    /// This function will panic if you try to call this outside of the web such as `node.js` runtime
    ///
    ///
    /// # Example    
    ///
    /// ```rust
    /// use weblog::console_log;
    /// use webru::document;
    ///
    /// // Get the `web_sys::Document` object by calling this function
    /// let document = document();
    ///
    /// // Get the url of the page
    /// let url = document.url().unwrap();
    ///
    /// // print the url in console
    /// console_log!(url.clone());
    ///
    /// assert_eq!(
    ///     url,
    ///     web_sys::window()
    ///         .unwrap()
    ///         .document()
    ///         .unwrap()
    ///         .url()
    ///         .unwrap()
    /// )
    /// ```
    ///
    ///    
    pub fn document() -> Document {
        let window = window().unwrap();
        window.document().unwrap()
    }

    /// Get the [`web_sys::Location`](https://docs.rs/web-sys/0.3.56/web_sys/struct.Location.html) object
    ///
    /// This function is equivalent to javascript's `location` property        
    ///
    ///        
    /// # Panics
    ///
    /// This function will panic if you try to call this outside of the web such as `node.js` runtime
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// use weblog::console_log;
    /// use webru::location;
    ///
    /// // Get the `web_sys::Location` object
    /// let location = location();
    ///
    /// // print the full path of your webpage
    /// console_log!(location.href().unwrap());
    ///
    /// // print the host/domain of your webpage
    /// console_log!(location.host().unwrap());
    ///
    /// ```
    ///
    pub fn location() -> Location {
        document().location().unwrap()
    }

    /// Get the full url of the web page.
    ///
    /// This function is equivalent to javascript's [`location.href`](https://developer.mozilla.org/en-US/docs/Web/API/Location/href) property
    ///
    /// It returns the full url of the web page
    ///
    /// If your web page's url is `https://www.example.com/path/name?key=2345` then it will return `https://www.example.com/path/name?key=2345`
    ///
    ///      
    /// # Panics
    ///
    /// This function will panic if you try to call this outside of the web such as `node.js` runtime    
    ///
    pub fn url() -> String {
        location().href().unwrap()
    }

    /// Get the [`body`] property.
    ///
    /// This function is equivalent to javascript's [`document.body`](https://developer.mozilla.org/en-US/docs/Web/API/Document/body) property
    ///
    ///
    /// # Panics
    ///
    /// This function will panic if you try to call this outside of the web such as `node.js` runtime
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// use webru::create_element;
    /// use webru::body;
    ///
    /// // Create an element with `create_element` function
    /// let h1: web_sys::Element = create_element("h1");
    ///
    /// // get the `body` field
    /// let body = body();
    ///
    /// // insert the element `h1` into the <body>
    /// body.append_child(&h1).unwrap();
    ///
    /// assert_eq!(
    ///     body,
    ///     web_sys::window()
    ///         .unwrap()
    ///         .document()
    ///         .unwrap()
    ///         .body()
    ///         .unwrap()
    /// );
    ///    
    /// ```
    ///
    /// [`body`]: <https://developer.mozilla.org/en-US/docs/Web/API/Document/body>
    pub fn body() -> HtmlElement {
        document().body().unwrap()
    }

    /// Get the domain name of the website
    ///
    /// This function is equivalent to javascript's [`location.hostname`](https://developer.mozilla.org/en-US/docs/Web/API/Location/hostname) property
    ///
    /// If your website's url is `https://www.example.com/path/name?key=2345` then it will return `www.example.com`
    ///
    /// If your website's url is `https://sub.example.org` then it will return `sub.example.org`
    ///
    ///     
    /// # Panics
    ///
    /// This function will panic if you try to call this outside of the web such as `node.js` runtime
    ///
    pub fn domain_name() -> String {
        location().hostname().unwrap()
    }

    /// Get the the path of the website
    ///
    /// This function is equivalent to javascript's [`location.pathname`](https://developer.mozilla.org/en-US/docs/Web/API/Location/pathname) property
    ///
    /// If your website's url is `https://www.example.com/path/name?key=234` then it will return `/path/name`
    ///
    /// If your website's url is `https://www.example.com` then it will return `/`
    ///
    ///
    /// # Panics
    ///
    /// This function will panic if you try to call this outside of the web such as `node.js` runtime
    ///
    pub fn path_name() -> String {
        location().pathname().unwrap()
    }

    /// Reloads the page
    ///
    /// This function is equivalent to javascript's `location.reload()` function
    ///    
    ///
    /// # Panics
    ///
    /// This function will panic if you try to call this outside of the web such as `node.js` runtime
    ///
    pub fn reload() {
        location().reload().unwrap()
    }

    /// Javascript [`alert`](https://developer.mozilla.org/en-US/docs/Web/API/Window/alert) method
    ///
    ///
    /// # Arguments
    ///
    /// * `msg` - The text to display in the alert box
    ///
    ///
    /// # Panics
    ///
    /// This function will panic if you try to call this outside of the web such as `node.js` runtime
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// use webru::alert;
    ///
    /// // show an popup/alert message
    /// alert("You do not have access to this site!");
    /// ```
    ///
    pub fn alert(msg: &str) {
        window().unwrap().alert_with_message(msg).unwrap()
    }

    /// Javascript [`prompt`](https://developer.mozilla.org/en-US/docs/Web/API/Window/prompt) method
    ///
    /// It returns the text the user entered in the dialog box
    ///
    /// If the user clicks on the `Okay` button and if the user typed anything in the prompt, then it will return `Some("whatever user typed")`, else if the user didn't typed anything it will return `Some("")`
    ///
    /// If the user clicks on the `Cancel` button then this function will return `None`  
    ///
    ///
    /// # Arguments
    ///
    /// * `msg` - The text to display in the dialog box    
    ///
    ///
    /// # Panics
    ///
    /// This function will panic if you try to call this outside of the web such as `node.js` runtime
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// use webru::prompt;
    /// use weblog::console_log;
    ///
    /// // taking user's name
    /// let name: Option<String> = prompt("What is your name?");
    ///
    /// if let Some(name) = name {
    ///     console_log!("The name of the user is: ", name);
    /// } else {
    ///     console_log!("The user didn't specify his/her name")
    /// }
    ///
    /// ```
    pub fn prompt(msg: &str) -> Option<String> {
        window().unwrap().prompt_with_message(msg).unwrap() // if None: it means the user clicked "cancel" button, else it means the user clicked "OK" button
    }

    /// Javascript [`Callback`](https://developer.mozilla.org/en-US/docs/Glossary/Callback_function)
    ///
    /// [`web-sys`](https://crates.io/crates/web-sys) uses [`Function`] for taking closures from Rust and pass it to javascript.
    ///
    /// So you can create a [`Closure`] by calling this function
    /// and then call the `.as_ref().dyn_ref().unwrap()` method to convert it from [`Closure<dyn Fn()>`] to [`Function`]
    ///
    /// It returns a [`Closure`] which can be used in any JavaScript function that takes a callback/closure as argument
    ///
    /// You can see more info about using rust closures with wasm-bindgen [here](https://rustwasm.github.io/docs/wasm-bindgen/examples/closures.html)
    ///
    ///
    /// # Arguments
    ///
    /// * `handler` - A Rust closure
    ///
    ///
    /// # Examples
    ///
    /// Javascript's `setTimeout` function
    ///
    /// ```rust
    /// use wasm_bindgen::JsCast;
    /// use webru::callback;
    /// use weblog::console_log;
    ///
    /// // Getting the `Window` object
    /// let window = web_sys::window().unwrap();
    ///
    /// // Getting the `wasm_bindgen::closure::Closure` by passing a Rust closure
    /// let callback = callback(|| {
    ///     console_log!("Hello world");
    /// });
    ///
    /// // The first argument of `set_timeout_with_callback_and_timeout_and_arguments_0` is a `&::js_sys::Function`
    /// // which can be used by converting the `Closure` into the `&::js_sys::Function`
    /// // by calling `.as_ref().unchecked_ref()` method on it.
    ///
    /// // The secend argument of `set_timeout_with_callback_and_timeout_and_arguments_0` is a timer,
    /// // how much time do I want to delay
    /// window
    ///     .set_timeout_with_callback_and_timeout_and_arguments_0(
    ///         callback.as_ref().unchecked_ref(),
    ///         2000,
    ///     )
    ///     .unwrap();
    ///
    /// // The call of `.forget()` is necessary. If we don't call this method, it will create a memory leak :(
    /// callback.forget();
    /// ```
    ///
    ///
    /// `onclick` property
    ///
    /// ```rust
    /// use webru::{body, callback, create_element};
    /// use web_sys::HtmlElement;
    /// use wasm_bindgen::JsCast;
    /// use weblog::console_log;
    ///     
    ///
    /// // Creating a <button>
    /// let button: HtmlElement = create_element("button")
    ///     .dyn_ref::<HtmlElement>()
    ///     .unwrap()
    ///     .clone();
    ///
    /// // setting the innerHTML field
    /// button.set_inner_html("click me");
    ///
    /// // This callback will be called the button will be clicked
    /// let callback = callback(|| {
    ///     console_log!("You clicked the button");
    /// });
    ///
    /// // setting `onclick` property to the <button>
    /// // converting the `Closure<dyn Fn()>` to `&js_sys::Function` by calling `.as_ref().dyn_ref().unwrap()` method
    /// button.set_onclick(Some(callback.as_ref().dyn_ref().unwrap()));
    ///
    /// // The call of `.forget()` is necessary. If we don't call this method, it will create a memory leak :(
    /// callback.forget();
    ///
    /// // inserting the button into the <body>
    /// body().append_child(&button).unwrap();
    ///
    /// // Now you the button is clicked, the text `You clicked the button` will be in console
    /// ```
    ///
    /// [`Closure`]: <https://docs.rs/wasm-bindgen/0.2.79/wasm_bindgen/closure/struct.Closure.html>
    /// [`Closure<dyn Fn()>`]: <https://docs.rs/wasm-bindgen/0.2.79/wasm_bindgen/closure/struct.Closure.html>
    /// [`Function`]: <https://docs.rs/js-sys/0.3.56/js_sys/struct.Function.html>
    ///
    pub fn callback<T: 'static>(handler: T) -> Closure<dyn Fn()>
    where
        T: Fn(),
    {
        Closure::wrap(Box::new(handler) as Box<dyn Fn()>)
    }

    /// Javascript [`document.createElement`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElement) method
    ///
    /// This function will create a new [`Element`](https://docs.rs/web-sys/0.3.56/web_sys/struct.Element.html) and return it.
    ///
    /// Note that this function will create and return the element, not push to the DOM
    ///
    ///
    /// # Arguments
    ///
    /// * `element_name` - A string that specifies the type of element to be created.
    ///
    ///
    /// # Panics
    ///
    /// * This function will panic if you try to call this outside of the web such as `node.js` runtime
    ///  
    /// * This function will panic if the `element_name` is not a valid tag name. For example if you pass `1h` as argument, it will `panic`. But if you pass `h1` as argument, it will not `panic`
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use webru::{body, create_element};
    ///
    /// // Create a <p> tag
    /// let p = create_element("p");
    ///
    /// // Create a <h1> tag
    /// let h1 = create_element("h1");
    ///
    /// // Create a <button> tag
    /// let button = create_element("button");
    ///
    /// // Create a <div> tag
    /// let div = create_element("div");
    ///
    /// // Pushing these elements into the DOM
    /// body().append_child(&p).unwrap();
    /// body().append_child(&h1).unwrap();
    /// body().append_child(&button).unwrap();
    /// body().append_child(&div).unwrap();
    ///```
    pub fn create_element(element_name: &str) -> Element {
        document().create_element(element_name).unwrap()
    }
}
