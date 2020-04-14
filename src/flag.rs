/// Flag structure which represents command-line option.
#[derive(Debug, Clone, PartialEq)]
pub struct Flag {
    name: String,
    alias: Option<String>,
    hint: Option<String>,
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

    /// Returns hint.
    pub fn hint(&self) -> &Option<String> {
        &self.hint
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
    pub fn with_name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            alias: None,
            hint: None,
            default_value: None,
            accepts_value: false,
        }
    }

    /// Sets alias name.
    pub fn with_alias(mut self, val: &str) -> Self {
        self.alias = Some(val.to_string());
        self
    }

    /// Sets hint.
    pub fn with_hint(mut self, val: &str) -> Self {
        self.hint = Some(val.to_string());
        self
    }

    /// Sets value.
    pub fn with_default_value(mut self, val: &str) -> Self {
        self.default_value = Some(val.to_string());
        self
    }

    /// Sets value.
    pub fn accept_value(mut self) -> Self {
        self.accepts_value = true;
        self
    }
}
