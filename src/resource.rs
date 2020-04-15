/// Structure holding external resource data.
#[derive(Debug, Clone, PartialEq)]
pub struct Resource {
    name: String,
    hint: Option<String>,
}

/// Structure implementation.
impl Resource {

    /// Returns name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns hint.
    pub fn hint(&self) -> &Option<String> {
        &self.hint
    }
}

/// Flag structure implementation.
impl Resource {

    // Returns new instance.
    pub fn with_name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            hint: None,
        }
    }

    /// Sets hint.
    pub fn with_hint(mut self, val: &str) -> Self {
        self.hint = Some(val.to_string());
        self
    }
}
