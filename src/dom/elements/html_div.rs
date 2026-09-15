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

    pub fn element(&self) -> &super::Element {
        self.html_element.element()
    }

    pub fn element_mut(&mut self) -> &mut super::Element {
        self.html_element.element_mut()
    }

    pub fn node(&self) -> &super::Node {
        self.html_element.element().node()
    }

    pub fn node_mut(&mut self) -> &mut super::Node {
        self.html_element.element_mut().node_mut()
    }

    pub fn id(&self) -> Option<&str> {
        self.html_element.id()
    }

    pub fn set_id(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), super::AttributeNameError> {
        self.html_element.set_id(value)
    }

    pub fn class_name(&self) -> Option<&str> {
        self.html_element.class_name()
    }

    pub fn set_class_name(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), super::AttributeNameError> {
        self.html_element.set_class_name(value)
    }

    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.html_element.get_attribute(name)
    }

    pub fn set_attribute(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), super::AttributeNameError> {
        self.html_element.set_attribute(name, value)
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        self.html_element.has_attribute(name)
    }

    pub fn remove_attribute(&mut self, name: &str) -> Option<super::Attribute> {
        self.html_element.remove_attribute(name)
    }

    pub fn tag_name(&self) -> String {
        self.html_element.tag_name()
    }

    pub fn local_name(&self) -> &str {
        self.html_element.local_name()
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
    fn exposes_lower_dom_layers() {
        let div = HTMLDivElement::new();

        assert_eq!(div.element().local_name(), "div");
        assert_eq!(div.node().node_name(), "div");
    }
}