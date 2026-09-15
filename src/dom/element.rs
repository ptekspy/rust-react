use super::{Attribute, AttributeNameError, Node, NodeType};

pub struct Attributes {
    items: Vec<Attribute>,
}

impl Attributes {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get(&self, name: &str) -> Option<&Attribute> {
        self.items
            .iter()
            .find(|attribute| attribute.qualified_name() == name)
    }

    pub fn get_value(&self, name: &str) -> Option<&str> {
        self.get(name).map(Attribute::value)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.get(name).is_some()
    }

    pub fn names(&self) -> Vec<String> {
        self.items
            .iter()
            .map(Attribute::qualified_name)
            .collect()
    }

    pub fn set(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        let name = name.into();
        let value = value.into();

        if let Some(attribute) = self
            .items
            .iter_mut()
            .find(|attribute| attribute.qualified_name() == name)
        {
            attribute.set_value(value);
            return Ok(());
        }

        self.items
            .push(Attribute::new(None, None, name, value)?);

        Ok(())
    }

    pub fn set_namespaced(
        &mut self,
        namespace_uri: Option<String>,
        prefix: Option<String>,
        local_name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        let attribute =
            Attribute::new(namespace_uri, prefix, local_name, value)?;

        let qualified_name = attribute.qualified_name();

        if let Some(existing) = self
            .items
            .iter_mut()
            .find(|existing| existing.qualified_name() == qualified_name)
        {
            *existing = attribute;
            return Ok(());
        }

        self.items.push(attribute);

        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> Option<Attribute> {
        let index = self
            .items
            .iter()
            .position(|attribute| attribute.qualified_name() == name)?;

        Some(self.items.remove(index))
    }

    pub fn iter(&self) -> impl Iterator<Item = &Attribute> {
        self.items.iter()
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Element {
    node: Node,
    namespace_uri: Option<String>,
    prefix: Option<String>,
    local_name: String,
    attributes: Attributes,
}

impl Element {
    pub fn new(local_name: impl Into<String>) -> Self {
        let local_name = local_name.into();

        Self {
            node: Node::new(NodeType::Element, local_name.clone()),
            namespace_uri: None,
            prefix: None,
            local_name,
            attributes: Attributes::new(),
        }
    }

    pub fn new_namespaced(
        namespace_uri: Option<String>,
        prefix: Option<String>,
        local_name: impl Into<String>,
    ) -> Self {
        let local_name = local_name.into();

        let node_name = match &prefix {
            Some(prefix) => format!("{prefix}:{local_name}"),
            None => local_name.clone(),
        };

        Self {
            node: Node::new(NodeType::Element, node_name),
            namespace_uri,
            prefix,
            local_name,
            attributes: Attributes::new(),
        }
    }

    pub fn node(&self) -> &Node {
        &self.node
    }

    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    pub fn namespace_uri(&self) -> Option<&str> {
        self.namespace_uri.as_deref()
    }

    pub fn prefix(&self) -> Option<&str> {
        self.prefix.as_deref()
    }

    pub fn local_name(&self) -> &str {
        &self.local_name
    }

    pub fn tag_name(&self) -> String {
        match &self.prefix {
            Some(prefix) => format!("{prefix}:{}", self.local_name),
            None => self.local_name.clone(),
        }
    }

    pub fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    pub fn has_attributes(&self) -> bool {
        !self.attributes.is_empty()
    }

    pub fn attribute_names(&self) -> Vec<String> {
        self.attributes.names()
    }

    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes.get_value(name)
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes.contains(name)
    }

    pub fn set_attribute(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.attributes.set(name, value)
    }

    pub fn remove_attribute(&mut self, name: &str) -> Option<Attribute> {
        self.attributes.remove(name)
    }

    pub fn id(&self) -> Option<&str> {
        self.get_attribute("id")
    }

    pub fn set_id(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.set_attribute("id", value)
    }

    pub fn class_name(&self) -> Option<&str> {
        self.get_attribute("class")
    }

    pub fn set_class_name(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AttributeNameError> {
        self.set_attribute("class", value)
    }
}

#[cfg(test)]
mod tests {
    use super::Element;

    #[test]
    fn creates_element() {
        let element = Element::new("div");

        assert_eq!(element.node().node_type_value(), 1);
        assert_eq!(element.node().node_name(), "div");
        assert_eq!(element.namespace_uri(), None);
        assert_eq!(element.prefix(), None);
        assert_eq!(element.local_name(), "div");
        assert_eq!(element.tag_name(), "div");
    }

    #[test]
    fn creates_namespaced_element() {
        let element = Element::new_namespaced(
            Some(String::from("http://www.w3.org/2000/svg")),
            None,
            "svg",
        );

        assert_eq!(
            element.namespace_uri(),
            Some("http://www.w3.org/2000/svg")
        );
        assert_eq!(element.local_name(), "svg");
        assert_eq!(element.tag_name(), "svg");
    }

    #[test]
    fn creates_prefixed_element() {
        let element = Element::new_namespaced(
            Some(String::from("http://example.com")),
            Some(String::from("custom")),
            "widget",
        );

        assert_eq!(element.prefix(), Some("custom"));
        assert_eq!(element.local_name(), "widget");
        assert_eq!(element.tag_name(), "custom:widget");
        assert_eq!(element.node().node_name(), "custom:widget");
    }

    #[test]
    fn manages_attributes() {
        let mut element = Element::new("div");

        element.set_attribute("role", "button").unwrap();

        assert!(element.has_attributes());
        assert!(element.has_attribute("role"));
        assert_eq!(element.get_attribute("role"), Some("button"));
        assert_eq!(element.attribute_names(), vec!["role"]);
    }

    #[test]
    fn replaces_existing_attribute() {
        let mut element = Element::new("div");

        element.set_attribute("id", "first").unwrap();
        element.set_attribute("id", "second").unwrap();

        assert_eq!(element.get_attribute("id"), Some("second"));
        assert_eq!(element.attribute_names(), vec!["id"]);
    }

    #[test]
    fn preserves_attribute_order() {
        let mut element = Element::new("div");

        element.set_attribute("id", "app").unwrap();
        element.set_attribute("class", "container").unwrap();
        element.set_attribute("role", "main").unwrap();

        assert_eq!(
            element.attribute_names(),
            vec![
                String::from("id"),
                String::from("class"),
                String::from("role"),
            ]
        );
    }

    #[test]
    fn exposes_id_reflection() {
        let mut element = Element::new("div");

        assert_eq!(element.id(), None);

        element.set_id("app").unwrap();

        assert_eq!(element.id(), Some("app"));
        assert_eq!(element.get_attribute("id"), Some("app"));
    }

    #[test]
    fn exposes_class_name_reflection() {
        let mut element = Element::new("div");

        assert_eq!(element.class_name(), None);

        element.set_class_name("container active").unwrap();

        assert_eq!(
            element.class_name(),
            Some("container active")
        );
        assert_eq!(
            element.get_attribute("class"),
            Some("container active")
        );
    }

    #[test]
    fn removes_attributes() {
        let mut element = Element::new("div");

        element.set_attribute("id", "app").unwrap();

        let removed = element.remove_attribute("id");

        assert!(removed.is_some());
        assert_eq!(element.get_attribute("id"), None);
        assert!(!element.has_attribute("id"));
    }
}