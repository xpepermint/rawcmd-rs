use std::str::FromStr;

/// Structure which holds param summary.
#[derive(Debug, Clone, PartialEq)]
pub struct ParamSummary {
    name: String,
    description: Option<String>,
    value: Option<String>,
    default_value: Option<String>,
    provided: bool,
}

/// Structure implementation.
impl ParamSummary {

    // Returns new instance.
    pub fn with_name<
        S: Into<String>,
    >(
        name: S,
        description: Option<String>,
        value: Option<String>,
        default_value: Option<String>,
        provided: bool,
    ) -> Self {
        Self {
            name: name.into(),
            description,
            value,
            default_value,
            provided,
        }
    }
}

/// Structure implementation.
impl ParamSummary {

    /// Returns name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns description.
    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    /// Returns value.
    pub fn value(&self) -> &Option<String> {
        &self.value
    }

    /// Returns value.
    pub fn to_value<T>(&self) -> Option<T>
        where
        T: FromStr,
    {
        match &self.value {
            Some(v) => match v.parse::<T>() {
                Ok(v) => Some(v),
                Err(_) => None,
            },
            None => None,
        }
    }

    /// Returns default value.
    pub fn default_value(&self) -> &Option<String> {
        &self.default_value
    }

    /// Returns true if the param has value.
    pub fn provided(&self) -> bool {
        self.provided
    }
}
