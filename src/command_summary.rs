/// Structure with command summary.
#[derive(Debug, Clone, PartialEq)]
pub struct CommandSummary {
    name: String,
    description: Option<String>,
}

/// Structure implementation.
impl CommandSummary {

    // Returns new instance.
    pub fn new(
        name: &str,
        description: Option<String>,
    ) -> Self {
        Self {
            name: name.to_string(),
            description,
        }
    }
}

/// Structure implementation.
impl CommandSummary {

    /// Returns name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns description.
    pub fn description(&self) -> &Option<String> {
        &self.description
    }
}
