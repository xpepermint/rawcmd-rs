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
    pub fn accepts_value(&self) -> bool {
        self.accepts_value
    }
}

/// Flag structure implementation.
impl Flag {

    // Returns new instance.
    pub fn with_name<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            alias: None,
            description: None,
            default_value: None,
            accepts_value: false,
        }
    }

    /// Sets alias name.
    pub fn with_alias<S: Into<String>>(mut self, val: S) -> Self {
        self.alias = Some(val.into());
        self
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

    /// Sets value.
    pub fn accept_value(mut self) -> Self {
        self.accepts_value = true;
        self
    }
}
