use std::fmt::{Display, Formatter};
use serde::de::{Expected, StdError, Unexpected};

#[derive(Debug, Default)]
pub struct FortniteError {
    name: String,
    message: String,
    source: Option<& 'static(dyn StdError + 'static)>
}
impl Display for FortniteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n{}: {}\n", self.name, self.message)
    }
}
impl std::error::Error for FortniteError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
    }
    fn description(&self) -> &str {
        &self.message
    }
}
impl From<reqwest::Error> for FortniteError {
    fn from(value: reqwest::Error) -> Self {
        Self {
            name: String::from("Reqwest error - ") + &*match &(&value).url() {
                Some(some) => some.to_string(),
                None => "(none)".to_string()
            },
            message: (&value).to_string(),
            .. Default::default()
        }
    }
}
impl From<serde_json::Error> for FortniteError {
    fn from(value: serde_json::Error) -> Self {
        Self {
            name: "serde_json error".to_string(),
            message: value.to_string(),
            .. Default::default()
        }
    }
}
impl serde::de::Error for FortniteError {
    fn custom<T>(msg: T) -> Self where T: Display {
        Self {
            name: "Serde error: Custom".to_string(),
            message: msg.to_string(),
            .. Default::default()
        }
    }

    fn invalid_type(unexp: Unexpected, exp: &dyn Expected) -> Self {
        Self {
            name: "Serde error: Invalid type".to_string(),
            message: format!("\n- Unexpected \"{unexp}\".\n- Expected \"{exp}\""),
            .. Default::default()
        }
    }

    fn invalid_value(unexp: Unexpected, exp: &dyn Expected) -> Self {
        Self {
            name: "Serde error: Invalid value".to_string(),
            message: format!("\n- Unexpected \"{unexp}\".\n- Expected \"{exp}\""),
            .. Default::default()
        }
    }

    fn invalid_length(len: usize, exp: &dyn Expected) -> Self {
        Self {
            name: "Serde error: Invalid length".to_string(),
            message: format!("\n- Unexpected length \"{len}\".\n- Expected \"{exp}\""),
            .. Default::default()
        }
    }

    fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self {
        Self {
            name: "Serde error: Unknown variant".to_string(),
            message: format!("\n- Unknown variant \"{variant}\".\n- Expected [{}]", expected.join(", ")),
            .. Default::default()
        }
    }

    fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self {
        Self {
            name: "Serde error: Unknown field".to_string(),
            message: format!("\n- Unknown field \"{field}\".\n- Expected [{}]", expected.join(", ")),
            .. Default::default()
        }
    }

    fn missing_field(field: &'static str) -> Self {
        Self {
            name: "Serde error: Missing field".to_string(),
            message: format!("\n- Missing field \"{field}\"."),
            .. Default::default()
        }
    }

    fn duplicate_field(field: &'static str) -> Self {
        Self {
            name: "Serde error: Duplicate field".to_string(),
            message: format!("\n- Duplicate field \"{field}\"."),
            .. Default::default()
        }
    }
}
