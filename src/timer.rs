#![allow(dead_code, unused)]

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::window;

use std::rc::Rc;

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

/// Combination of [`set_timeout()`] and [`clear_timeout()`] functions
///
/// # Panics
/// This function will panic if you try to call this outside of the web such as `node.js` runtime
///
/// # Example
///
/// ```rust
/// use std::cell::Cell;
/// use std::rc::Rc;
/// use webru::{set_timeout, Timeout};
///
/// #[derive(Clone, Copy, Debug, PartialEq)]
/// enum EarthShape {
///     Flat,
///     Round,
/// }
///
/// let earth_shape = Rc::new(Cell::new(EarthShape::Flat));
///
/// {
///     let earth_shape = Rc::clone(&earth_shape);
///
///     // setTimeout without stopping it
///     // it will run after 4 secs
///     Timeout::start(
///         move || {
///             // mutating the state of the earth_shape
///             earth_shape.set(EarthShape::Round);
///         },
///         4000, // 4 secs
///     );
/// }
///
/// set_timeout(
///     move || {
///         // After 4 seconds, the earth_shape should be `Round`
///         assert_eq!(earth_shape.get(), EarthShape::Round);
///     },
///     5000, // 5 secs
/// )
/// .unwrap();
///
/// let earth_shape = Rc::new(Cell::new(EarthShape::Flat));
///
/// {
///     let earth_shape = Rc::clone(&earth_shape);
///
///     // setTimeout by stopping it
///     // it will not run because we are stopping it by calling the `stop` method
///     let timeout: Timeout = Timeout::start(
///         move || {
///            // mutating the state of the earth_shape
///             earth_shape.set(EarthShape::Round);
///         },
///         4000, // 4 secs
///     );
///     // stop the timeout. This method is equivalent to `clearTimeout` in the browser
///     timeout.stop();
/// }
///
/// set_timeout(
///     move || {
///         // As the timeout is stopped after creating it, the earth_shape should be `Flat`
///         assert_eq!(earth_shape.get(), EarthShape::Flat);
///     },
///     5000, // 5 secs
/// )
/// .unwrap();
///
/// ```
#[derive(Clone)]
pub struct Timeout {
    timeout_id: i32,
}

impl Timeout {
    /// Starts the timeout.
    ///
    /// This method is equivalent to [`set_timeout`] function.
    ///
    /// After starting the timeout, you can stop it by calling the `stop` method.
    ///  
    pub fn start<T: 'static>(handler: T, timeout: i32) -> Self
    where
        T: Fn(),
    {
        let timeout_id = set_timeout(handler, timeout).unwrap();

        Self { timeout_id }
    }

    /// Stops the timeout.
    ///
    /// This method is equivalent to [`clear_timeout`] function.
    ///     
    pub fn stop(&self) {
        clear_timeout(self.timeout_id);
    }
}

/// Combination of [`set_interval()`] and [`clear_interval()`] functions
///
/// # Panics
/// This function will panic if you try to call this outside of the web such as `node.js` runtime
///
/// # Example
///
/// ```rust
/// use wasm_bindgen::JsCast;
/// use web_sys::HtmlElement;
/// use weblog::console_log;
/// use webru::{body, callback, create_element, Interval};
///
/// // Start the interval
/// // It will run every 2 seconds until you press the button to stop it.
/// let interval: Interval = Interval::start(
///     || {
///         console_log!("You are in danger :(");
///     },
///     2000,
/// ); // every 2 seconds
///
/// // onclick event for stopping the interval
/// let onclick = callback({
///     move || {
///         // Stop the interval
///         interval.stop();
///     }
/// });
///
/// // Creating a <button> element for stopping the interval
/// let button: HtmlElement = create_element("button")
///     .dyn_ref::<HtmlElement>()
///     .unwrap()
///     .clone();
///
/// // Set the inner html of the button
/// button.set_inner_html("Stop interval");
/// // Set the onclick event for the button
/// button.set_onclick(Some(&onclick.as_ref().dyn_ref().unwrap()));
///
/// // Now when you press the button the interval should be stop
///
/// onclick.forget();
///
/// // Append the button to the body
/// body().append_child(&button).unwrap();
/// ```
pub struct Interval {
    interval_id: i32,
}

impl Interval {
    /// Starts the interval.
    ///
    /// This method is equivalent to [`set_interval`] function.
    ///
    /// After starting the interval, you can stop it by calling the `stop` method.
    ///
    pub fn start<T: 'static>(handler: T, timeout: i32) -> Self
    where
        T: Fn(),
    {
        let interval_id = set_interval(handler, timeout).unwrap();

        Self { interval_id }
    }

    /// Stops the interval.
    ///
    /// This method is equivalent to [`clear_interval`] function.
    ///
    pub fn stop(&self) {
        clear_interval(self.interval_id);
    }
}
