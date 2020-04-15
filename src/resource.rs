/// Structure holding external resource data.
#[derive(Debug, Clone, PartialEq)]
pub struct Resource {
    name: String,
    description: Option<String>,
}

/// Structure implementation.
impl Resource {

    /// Returns name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns description.
    pub fn description(&self) -> &Option<String> {
        &self.description
    }
}

/// Flag structure implementation.
impl Resource {

    // Returns new instance.
    pub fn with_name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: None,
        }
    }

    /// Sets description.
    pub fn with_description(mut self, val: &str) -> Self {
        self.description = Some(val.to_string());
        self
    }
}
