use wasm_bindgen::closure::Closure;
use web_sys::{window, Document, Element, HtmlElement, Location};

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
