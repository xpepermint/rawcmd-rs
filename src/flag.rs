use crate::{FlagSummary};

/// Flag structure which represents command-line option.
#[derive(Debug, Clone, PartialEq)]
pub struct Flag {
    name: String,
    alias: Option<String>,
    description: Option<String>,
    default_value: Option<String>,
    accepts_value: bool,
}

/// Flag structure implementation.
impl Flag {

    /// Returns name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns alias.
    pub fn alias(&self) -> &Option<String> {
        &self.alias
    }

    /// Returns description.
    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    /// Returns default value.
    pub fn default_value(&self) -> &Option<String> {
        &self.default_value
    }

    /// Returns true if the flag accepts value.
    pub fn accepts_value(&self) -> &bool {
        &self.accepts_value
    }

    /// Builds summary for this command.
    pub fn summarize(&self) -> FlagSummary {
        FlagSummary::new(
            self.name.clone().as_str(),
            self.alias.clone(),
            self.description.clone(),
            self.default_value.clone(),
            self.accepts_value.clone(),
        )
    }
}

/// Flag structure implementation.
impl Flag {

    // Returns new instance.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            alias: None,
            description: None,
            default_value: None,
            accepts_value: false,
        }
    }

    /// Sets alias name.
    pub fn with_alias(mut self, val: &str) -> Self {
        self.alias = Some(val.to_string());
        self
    }

    /// Sets description.
    pub fn with_description(mut self, val: &str) -> Self {
        self.description = Some(val.to_string());
        self
    }

    /// Sets value.
    pub fn with_default_value(mut self, default: &str) -> Self {
        self.default_value = Some(default.to_string());
        self
    }

    /// Sets value.
    pub fn accept_value(mut self) -> Self {
        self.accepts_value = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_summary() {
        let flag = Flag::new("name")
            .with_alias("alias")
            .with_description("description")
            .with_default_value("default_value")
            .accept_value();
        assert_eq!(flag.summarize(), FlagSummary::new(
            "name",
            Some("alias".to_string()),
            Some("description".to_string()),
            Some("default_value".to_string()),
            true,
        ));
    }
}
