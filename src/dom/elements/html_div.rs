use std::ops::{Deref, DerefMut};

use super::HTMLElement;

pub struct HTMLDivElement {
    html_element: HTMLElement,
}

impl HTMLDivElement {
    pub fn new() -> Self {
        Self {
            html_element: HTMLElement::new("div"),
        }
    }

    pub fn html_element(&self) -> &HTMLElement {
        &self.html_element
    }

    pub fn html_element_mut(&mut self) -> &mut HTMLElement {
        &mut self.html_element
    }
}

impl Deref for HTMLDivElement {
    type Target = HTMLElement;

    fn deref(&self) -> &Self::Target {
        &self.html_element
    }
}

impl DerefMut for HTMLDivElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html_element
    }
}

impl Default for HTMLDivElement {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::HTMLDivElement;

    #[test]
    fn creates_div_element() {
        let div = HTMLDivElement::new();

        assert_eq!(div.tag_name(), "div");
        assert_eq!(div.local_name(), "div");
    }

    #[test]
    fn inherits_html_element_behaviour() {
        let mut div = HTMLDivElement::new();

        div.set_id("app").unwrap();
        div.set_class_name("container").unwrap();
        div.set_attribute("title", "Hello").unwrap();

        assert_eq!(div.id(), Some("app"));
        assert_eq!(div.class_name(), Some("container"));
        assert_eq!(div.get_attribute("title"), Some("Hello"));
    }

    #[test]
    fn inherits_element_behaviour() {
        let mut div = HTMLDivElement::new();

        div.set_attribute("role", "main").unwrap();

        assert!(div.has_attribute("role"));
        assert_eq!(div.get_attribute("role"), Some("main"));
    }

    #[test]
    fn inherits_node_behaviour() {
        let div = HTMLDivElement::new();

        assert_eq!(div.node_type_value(), 1);
        assert_eq!(div.node_name(), "div");
    }

    #[test]
    fn inherits_event_target_behaviour() {
        let mut div = HTMLDivElement::new();

        div.add_event_listener("click", |_| {});
    }

    #[test]
    fn exposes_underlying_layers() {
        let div = HTMLDivElement::new();

        assert_eq!(div.html_element().tag_name(), "div");
        assert_eq!(div.element().tag_name(), "div");
        assert_eq!(div.node().node_name(), "div");
    }
}