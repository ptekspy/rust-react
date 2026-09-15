#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attribute {
    namespace_uri: Option<String>,
    prefix: Option<String>,
    local_name: String,
    value: String,
}

impl Attribute {
    pub fn new(
        namespace_uri: Option<String>,
        prefix: Option<String>,
        local_name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, AttributeNameError> {
        let local_name = local_name.into();

        validate_attribute_local_name(&local_name)?;

        if prefix.is_some() && namespace_uri.is_none() {
            return Err(AttributeNameError::PrefixWithoutNamespace);
        }

        Ok(Self {
            namespace_uri,
            prefix,
            local_name,
            value: value.into(),
        })
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

    pub fn name(&self) -> &str {
        &self.local_name
    }

    pub fn qualified_name(&self) -> String {
        match &self.prefix {
            Some(prefix) => format!("{prefix}:{}", self.local_name),
            None => self.local_name.clone(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AttributeNameError {
    Empty,
    InvalidCharacter(char),
    PrefixWithoutNamespace,
}

fn validate_attribute_local_name(name: &str) -> Result<(), AttributeNameError> {
    if name.is_empty() {
        return Err(AttributeNameError::Empty);
    }

    for character in name.chars() {
        if character.is_ascii_whitespace()
            || matches!(character, '\0' | '/' | '=' | '>')
        {
            return Err(AttributeNameError::InvalidCharacter(character));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Attribute, AttributeNameError};

    #[test]
    fn creates_attribute() {
        let attribute = Attribute::new(None, None, "id", "app").unwrap();

        assert_eq!(attribute.namespace_uri(), None);
        assert_eq!(attribute.prefix(), None);
        assert_eq!(attribute.local_name(), "id");
        assert_eq!(attribute.name(), "id");
        assert_eq!(attribute.qualified_name(), "id");
        assert_eq!(attribute.value(), "app");
    }

    #[test]
    fn creates_namespaced_attribute() {
        let attribute = Attribute::new(
            Some(String::from("http://www.w3.org/1999/xlink")),
            Some(String::from("xlink")),
            "href",
            "/home",
        )
        .unwrap();

        assert_eq!(
            attribute.namespace_uri(),
            Some("http://www.w3.org/1999/xlink")
        );
        assert_eq!(attribute.prefix(), Some("xlink"));
        assert_eq!(attribute.local_name(), "href");
        assert_eq!(attribute.qualified_name(), "xlink:href");
    }

    #[test]
    fn rejects_empty_name() {
        let result = Attribute::new(None, None, "", "value");

        assert_eq!(result, Err(AttributeNameError::Empty));
    }

    #[test]
    fn rejects_invalid_name_character() {
        let result = Attribute::new(None, None, "data value", "value");

        assert_eq!(
            result,
            Err(AttributeNameError::InvalidCharacter(' '))
        );
    }

    #[test]
    fn rejects_prefix_without_namespace() {
        let result = Attribute::new(
            None,
            Some(String::from("xlink")),
            "href",
            "/home",
        );

        assert_eq!(
            result,
            Err(AttributeNameError::PrefixWithoutNamespace)
        );
    }

    #[test]
    fn changes_value() {
        let mut attribute =
            Attribute::new(None, None, "id", "before").unwrap();

        attribute.set_value("after");

        assert_eq!(attribute.value(), "after");
    }
}