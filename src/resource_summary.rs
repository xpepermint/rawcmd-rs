/// Structure which holds resource summary.
#[derive(Debug, Clone, PartialEq)]
pub struct ResourceSummary {
    name: String,
    hint: Option<String>,
}

/// Structure implementation.
impl ResourceSummary {

    // Returns new instance.
    pub fn with_name(
        name: &str,
        hint: Option<String>,
    ) -> Self {
        Self {
            name: name.to_string(),
            hint,
        }
    }
}

/// Structure implementation.
impl ResourceSummary {

    /// Returns name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns hint.
    pub fn hint(&self) -> &Option<String> {
        &self.hint
    }
}
