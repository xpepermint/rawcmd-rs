use crate::{ParamResolver};

/// Param structure which represents command-line option.
#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    name: String,
    description: Option<String>,
    default_value: Option<String>,
    resolver: Option<ParamResolver>,
}

/// Param structure implementation.
impl Param {

    /// Returns name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns description.
    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    /// Returns default value.
    pub fn default_value(&self) -> &Option<String> {
        &self.default_value
    }

    /// Returns resolver.
    pub fn resolver(&self) -> &Option<ParamResolver> {
        &self.resolver
    }
}

/// Param structure implementation.
impl Param {

    // Returns new instance.
    pub fn with_name<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            description: None,
            default_value: None,
            resolver: None,
        }
    }

    /// Sets description.
    pub fn with_description<S: Into<String>>(mut self, val: S) -> Self {
        self.description = Some(val.into());
        self
    }

    /// Sets value.
    pub fn with_default_value<S: Into<String>>(mut self, val: S) -> Self {
        self.default_value = Some(val.into());
        self
    }

    /// Sets resolver.
    pub fn with_resolver(mut self, val: ParamResolver) -> Self {
        self.resolver = Some(val);
        self
    }
}
