/// Structure which holds flag summary.
#[derive(Debug, Clone, PartialEq)]
pub struct FlagSummary {
    name: String,
    alias: Option<String>,
    description: Option<String>,
    value: Option<String>,
    takes: bool,
}

/// Structure implementation.
impl FlagSummary {

    // Returns new instance.
    pub fn new(
        name: &str,
        alias: Option<String>,
        description: Option<String>,
        value: Option<String>,
        takes: bool,
    ) -> Self {
        Self {
            name: name.to_string(),
            alias,
            description,
            value,
            takes,
        }
    }
}

/// Structure implementation.
impl FlagSummary {

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

    /// Returns value.
    pub fn value(&self) -> &Option<String> {
        &self.value
    }

    /// Returns true if the flag has value.
    pub fn takes(&self) -> &bool {
        &self.takes
    }

    /// Returns alias.
    pub fn set_alias(&mut self, val: &str) {
        self.alias = Some(val.to_string())
    }

    /// Returns description.
    pub fn set_description(&mut self, val: &str) {
        self.description = Some(val.to_string())
    }

    // Sets value.
    pub fn set_value(mut self, val: Option<&str>) -> Self {
        self.value = match val {
            Some(v) => Some(v.to_string()),
            None => None,
        };
        self
    }

    // Sets value.
    pub fn set_takes(mut self, takes: bool) -> Self {
        self.takes = takes;
        self
    }
}
