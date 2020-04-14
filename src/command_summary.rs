/// Structure with command summary.
#[derive(Debug, Clone, PartialEq)]
pub struct CommandSummary {
    name: String,
    description: Option<String>,
    hint: Option<String>,
    author: Option<String>,
    version: Option<String>,
}

/// Structure implementation.
impl CommandSummary {

    // Returns new instance.
    pub fn with_name(
        name: &str,
        description: Option<String>,
        hint: Option<String>,
        author: Option<String>,
        version: Option<String>,
    ) -> Self {
        Self {
            name: name.to_string(),
            description,
            hint,
            author,
            version,
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

    /// Returns hint.
    pub fn hint(&self) -> &Option<String> {
        &self.hint
    }

    /// Returns author.
    pub fn author(&self) -> &Option<String> {
        &self.author
    }

    /// Returns version.
    pub fn version(&self) -> &Option<String> {
        &self.version
    }
}
