use web_sys::{Element, HtmlCollection, Node, NodeList};

use crate::document;

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
