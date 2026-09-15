use std::ops::{Deref, DerefMut};

use super::HTMLElement;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HTMLButtonType {
    Submit,
    Reset,
    Button,
}

impl HTMLButtonType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Submit => "submit",
            Self::Reset => "reset",
            Self::Button => "button",
        }
    }
}

pub struct HTMLButtonElement {
    html_element: HTMLElement,
}

impl HTMLButtonElement {
    pub fn new() -> Self {
        Self {
            html_element: HTMLElement::new("button"),
        }
    }

    pub fn html_element(&self) -> &HTMLElement {
        &self.html_element
    }

    pub fn html_element_mut(&mut self) -> &mut HTMLElement {
        &mut self.html_element
    }

    pub fn disabled(&self) -> bool {
        self.has_attribute("disabled")
    }

    pub fn set_disabled(&mut self, disabled: bool) {
        if disabled {
            self.set_attribute("disabled", "")
                .expect("static attribute name is valid");
        } else {
            self.remove_attribute("disabled");
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.get_attribute("name")
    }

    pub fn set_name(&mut self, value: impl Into<String>) {
        self.set_attribute("name", value)
            .expect("static attribute name is valid");
    }

    pub fn button_type(&self) -> HTMLButtonType {
        match self.get_attribute("type") {
            Some("reset") => HTMLButtonType::Reset,
            Some("button") => HTMLButtonType::Button,
            _ => HTMLButtonType::Submit,
        }
    }

    pub fn set_button_type(&mut self, button_type: HTMLButtonType) {
        self.set_attribute("type", button_type.as_str())
            .expect("static attribute name is valid");
    }

    pub fn value(&self) -> Option<&str> {
        self.get_attribute("value")
    }

    pub fn set_value(&mut self, value: impl Into<String>) {
        self.set_attribute("value", value)
            .expect("static attribute name is valid");
    }
}

impl Deref for HTMLButtonElement {
    type Target = HTMLElement;

    fn deref(&self) -> &Self::Target {
        &self.html_element
    }
}

impl DerefMut for HTMLButtonElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html_element
    }
}

impl Default for HTMLButtonElement {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{HTMLButtonElement, HTMLButtonType};

    #[test]
    fn creates_button_element() {
        let button = HTMLButtonElement::new();

        assert_eq!(button.tag_name(), "button");
        assert_eq!(button.local_name(), "button");
    }

    #[test]
    fn defaults_to_submit_type() {
        let button = HTMLButtonElement::new();

        assert_eq!(button.button_type(), HTMLButtonType::Submit);
    }

    #[test]
    fn supports_button_type() {
        let mut button = HTMLButtonElement::new();

        button.set_button_type(HTMLButtonType::Button);

        assert_eq!(button.button_type(), HTMLButtonType::Button);
        assert_eq!(button.get_attribute("type"), Some("button"));
    }

    #[test]
    fn supports_reset_type() {
        let mut button = HTMLButtonElement::new();

        button.set_button_type(HTMLButtonType::Reset);

        assert_eq!(button.button_type(), HTMLButtonType::Reset);
        assert_eq!(button.get_attribute("type"), Some("reset"));
    }

    #[test]
    fn supports_submit_type() {
        let mut button = HTMLButtonElement::new();

        button.set_button_type(HTMLButtonType::Submit);

        assert_eq!(button.button_type(), HTMLButtonType::Submit);
        assert_eq!(button.get_attribute("type"), Some("submit"));
    }

    #[test]
    fn supports_disabled() {
        let mut button = HTMLButtonElement::new();

        assert!(!button.disabled());

        button.set_disabled(true);

        assert!(button.disabled());
        assert!(button.has_attribute("disabled"));

        button.set_disabled(false);

        assert!(!button.disabled());
        assert!(!button.has_attribute("disabled"));
    }

    #[test]
    fn supports_name() {
        let mut button = HTMLButtonElement::new();

        assert_eq!(button.name(), None);

        button.set_name("submit-button");

        assert_eq!(button.name(), Some("submit-button"));
        assert_eq!(
            button.get_attribute("name"),
            Some("submit-button")
        );
    }

    #[test]
    fn supports_value() {
        let mut button = HTMLButtonElement::new();

        assert_eq!(button.value(), None);

        button.set_value("save");

        assert_eq!(button.value(), Some("save"));
        assert_eq!(button.get_attribute("value"), Some("save"));
    }

    #[test]
    fn inherits_html_element_behaviour() {
        let mut button = HTMLButtonElement::new();

        button.set_id("submit").unwrap();
        button.set_class_name("primary").unwrap();
        button.set_title("Submit form").unwrap();

        assert_eq!(button.id(), Some("submit"));
        assert_eq!(button.class_name(), Some("primary"));
        assert_eq!(button.title(), Some("Submit form"));
    }

    #[test]
    fn inherits_node_behaviour() {
        let button = HTMLButtonElement::new();

        assert_eq!(button.node_type_value(), 1);
        assert_eq!(button.node_name(), "button");
    }

    #[test]
    fn inherits_event_target_behaviour() {
        let mut button = HTMLButtonElement::new();

        button.add_event_listener("click", |_| {});
    }

    #[test]
    fn exposes_underlying_layers() {
        let button = HTMLButtonElement::new();

        assert_eq!(button.html_element().tag_name(), "button");
        assert_eq!(button.element().tag_name(), "button");
        assert_eq!(button.node().node_name(), "button");
    }
}