use std::ops::{Deref, DerefMut};

use super::{AttributeNameError, Element};

pub struct HTMLElement {
    element: Element,
}

impl HTMLElement {
    pub fn new(local_name: impl Into<String>) -> Self {
        Self {
            element: Element::new(local_name),
        }
    }

    pub fn element(&self) -> &Element {
        &self.element
    }

    pub fn element_mut(&mut self) -> &mut Element {
        &mut self.element
    }

    pub fn namespace_uri(&self) -> Option<&str> {
        self.element.namespace_uri()
    }

    pub fn prefix(&self) -> Option<&str> {
        self.element.prefix()
    }

    pub fn local_name(&self) -> &str {
        self.element.local_name()
    }

    pub fn tag_name(&self) -> String {
        self.element.tag_name()
    }

    pub fn attributes(&self) -> &super::Attributes {
        self.element.attributes()
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        self.element.has_attribute(name)
    }

    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.element.get_attribute(name)
    }

    pub fn set_attribute(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.element.set_attribute(name, value)
    }

    pub fn remove_attribute(
        &mut self,
        name: &str,
    ) -> Option<super::Attribute> {
        self.element.remove_attribute(name)
    }

    pub fn id(&self) -> Option<&str> {
        self.element.id()
    }

    pub fn set_id(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.element.set_id(value)
    }

    pub fn class_name(&self) -> Option<&str> {
        self.element.class_name()
    }

    pub fn set_class_name(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.element.set_class_name(value)
    }

    pub fn title(&self) -> Option<&str> {
        self.get_attribute("title")
    }

    pub fn set_title(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.set_attribute("title", value)
    }

    pub fn lang(&self) -> Option<&str> {
        self.get_attribute("lang")
    }

    pub fn set_lang(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.set_attribute("lang", value)
    }

    pub fn dir(&self) -> Option<&str> {
        self.get_attribute("dir")
    }

    pub fn set_dir(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.set_attribute("dir", value)
    }

    pub fn slot(&self) -> Option<&str> {
        self.get_attribute("slot")
    }

    pub fn set_slot(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.set_attribute("slot", value)
    }

    pub fn access_key(&self) -> Option<&str> {
        self.get_attribute("accesskey")
    }

    pub fn set_access_key(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.set_attribute("accesskey", value)
    }

    pub fn autocapitalize(&self) -> Option<&str> {
        self.get_attribute("autocapitalize")
    }

    pub fn set_autocapitalize(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.set_attribute("autocapitalize", value)
    }

    pub fn autocorrect(&self) -> Option<&str> {
        self.get_attribute("autocorrect")
    }

    pub fn set_autocorrect(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.set_attribute("autocorrect", value)
    }

    pub fn enter_key_hint(&self) -> Option<&str> {
        self.get_attribute("enterkeyhint")
    }

    pub fn set_enter_key_hint(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.set_attribute("enterkeyhint", value)
    }

    pub fn input_mode(&self) -> Option<&str> {
        self.get_attribute("inputmode")
    }

    pub fn set_input_mode(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.set_attribute("inputmode", value)
    }

    pub fn nonce(&self) -> Option<&str> {
        self.get_attribute("nonce")
    }

    pub fn set_nonce(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.set_attribute("nonce", value)
    }

    pub fn style(&self) -> Option<&str> {
        self.get_attribute("style")
    }

    pub fn set_style(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.set_attribute("style", value)
    }

    pub fn is_hidden(&self) -> bool {
        self.has_attribute("hidden")
    }

    pub fn set_hidden(
        &mut self,
        hidden: bool,
    ) -> Result<(), AttributeNameError> {
        if hidden {
            self.set_attribute("hidden", "")
        } else {
            self.remove_attribute("hidden");
            Ok(())
        }
    }

    pub fn is_inert(&self) -> bool {
        self.has_attribute("inert")
    }

    pub fn set_inert(
        &mut self,
        inert: bool,
    ) -> Result<(), AttributeNameError> {
        if inert {
            self.set_attribute("inert", "")
        } else {
            self.remove_attribute("inert");
            Ok(())
        }
    }

    pub fn is_draggable(&self) -> bool {
        self.get_attribute("draggable")
            .map(|value| value == "true")
            .unwrap_or(false)
    }

    pub fn set_draggable(
        &mut self,
        draggable: bool,
    ) -> Result<(), AttributeNameError> {
        self.set_attribute(
            "draggable",
            if draggable { "true" } else { "false" },
        )
    }

    pub fn spell_check(&self) -> Option<&str> {
        self.get_attribute("spellcheck")
    }

    pub fn set_spell_check(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.set_attribute("spellcheck", value)
    }

    pub fn translate(&self) -> Option<&str> {
        self.get_attribute("translate")
    }

    pub fn set_translate(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.set_attribute("translate", value)
    }

    pub fn popover(&self) -> Option<&str> {
        self.get_attribute("popover")
    }

    pub fn set_popover(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.set_attribute("popover", value)
    }

    pub fn item_id(&self) -> Option<&str> {
        self.get_attribute("itemid")
    }

    pub fn set_item_id(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.set_attribute("itemid", value)
    }

    pub fn item_prop(&self) -> Option<&str> {
        self.get_attribute("itemprop")
    }

    pub fn set_item_prop(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.set_attribute("itemprop", value)
    }

    pub fn item_ref(&self) -> Option<&str> {
        self.get_attribute("itemref")
    }

    pub fn set_item_ref(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.set_attribute("itemref", value)
    }

    pub fn item_scope(&self) -> bool {
        self.has_attribute("itemscope")
    }

    pub fn set_item_scope(
        &mut self,
        enabled: bool,
    ) -> Result<(), AttributeNameError> {
        if enabled {
            self.set_attribute("itemscope", "")
        } else {
            self.remove_attribute("itemscope");
            Ok(())
        }
    }

    pub fn item_type(&self) -> Option<&str> {
        self.get_attribute("itemtype")
    }

    pub fn set_item_type(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.set_attribute("itemtype", value)
    }

    pub fn heading_offset(&self) -> Option<&str> {
        self.get_attribute("headingoffset")
    }

    pub fn set_heading_offset(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.set_attribute("headingoffset", value)
    }

    pub fn heading_reset(&self) -> Option<&str> {
        self.get_attribute("headingreset")
    }

    pub fn set_heading_reset(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.set_attribute("headingreset", value)
    }

    pub fn writing_suggestions(&self) -> Option<&str> {
        self.get_attribute("writingsuggestions")
    }

    pub fn set_writing_suggestions(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.set_attribute("writingsuggestions", value)
    }
}

impl Deref for HTMLElement {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.element
    }
}

impl DerefMut for HTMLElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.element
    }
}

#[cfg(test)]
mod tests {
    use super::HTMLElement;

    #[test]
    fn creates_html_element() {
        let element = HTMLElement::new("div");

        assert_eq!(element.local_name(), "div");
        assert_eq!(element.tag_name(), "div");
    }

    #[test]
    fn inherits_element_attributes() {
        let mut element = HTMLElement::new("div");

        element.set_id("app").unwrap();
        element.set_class_name("container").unwrap();

        assert_eq!(element.id(), Some("app"));
        assert_eq!(element.class_name(), Some("container"));
    }

    #[test]
    fn supports_global_string_attributes() {
        let mut element = HTMLElement::new("div");

        element.set_title("Hello").unwrap();
        element.set_lang("en").unwrap();
        element.set_dir("ltr").unwrap();

        assert_eq!(element.title(), Some("Hello"));
        assert_eq!(element.lang(), Some("en"));
        assert_eq!(element.dir(), Some("ltr"));
    }

    #[test]
    fn supports_boolean_attributes() {
        let mut element = HTMLElement::new("div");

        assert!(!element.is_hidden());
        assert!(!element.is_inert());

        element.set_hidden(true).unwrap();
        element.set_inert(true).unwrap();

        assert!(element.is_hidden());
        assert!(element.is_inert());

        element.set_hidden(false).unwrap();
        element.set_inert(false).unwrap();

        assert!(!element.is_hidden());
        assert!(!element.is_inert());
    }

    #[test]
    fn supports_slot() {
        let mut element = HTMLElement::new("div");

        element.set_slot("content").unwrap();

        assert_eq!(element.slot(), Some("content"));
    }

    #[test]
    fn supports_metadata_attributes() {
        let mut element = HTMLElement::new("div");

        element.set_item_id("product-1").unwrap();
        element.set_item_prop("name").unwrap();
        element.set_item_ref("description").unwrap();
        element.set_item_scope(true).unwrap();
        element.set_item_type("https://schema.org/Product").unwrap();

        assert_eq!(element.item_id(), Some("product-1"));
        assert_eq!(element.item_prop(), Some("name"));
        assert_eq!(element.item_ref(), Some("description"));
        assert!(element.item_scope());
        assert_eq!(
            element.item_type(),
            Some("https://schema.org/Product")
        );
    }

    #[test]
    fn dereferences_through_element_to_node() {
        let element = HTMLElement::new("div");

        assert_eq!(element.node_name(), "div");
        assert_eq!(element.node_type_value(), 1);
    }

    #[test]
    fn dereferences_through_node_to_event_target() {
        let mut element = HTMLElement::new("div");

        element.add_event_listener("click", |_| {});
    }
}