/// Structure with command summary.
#[derive(Debug, Clone, PartialEq)]
pub struct CommandSummary {
    name: String,
    about: Option<String>,
    description: Option<String>,
    author: Option<String>,
    version: Option<String>,
}

/// Structure implementation.
impl CommandSummary {

    // Returns new instance.
    pub fn with_name<
        S: Into<String>,
    >(
        name: S,
        about: Option<String>,
        description: Option<String>,
        author: Option<String>,
        version: Option<String>,
    ) -> Self {
        Self {
            name: name.into(),
            about,
            description,
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

    /// Returns about.
    pub fn about(&self) -> &Option<String> {
        &self.about
    }

    /// Returns description.
    pub fn description(&self) -> &Option<String> {
        &self.description
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
