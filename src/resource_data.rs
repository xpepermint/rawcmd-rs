/// Structure which holds resource summary.
#[derive(Debug, Clone, PartialEq)]
pub struct ResourceData {
    name: String,
    description: Option<String>,
}

/// Structure implementation.
impl ResourceData {

    // Returns new instance.
    pub fn with_name<
        S: Into<String>,
    >(
        name: S,
        description: Option<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description,
        }
    }
}

/// Structure implementation.
impl ResourceData {

    /// Returns name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns description.
    pub fn description(&self) -> &Option<String> {
        &self.description
    }
}
