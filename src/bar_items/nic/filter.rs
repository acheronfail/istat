use std::error::Error;
use std::str::FromStr;

use serde::{de, Deserialize, Serialize};

use crate::util::net::{Interface, InterfaceKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceFilter {
    name: String,
    kind: Option<InterfaceKind>,
}

impl InterfaceFilter {
    pub fn new(name: impl AsRef<str>, kind: Option<InterfaceKind>) -> InterfaceFilter {
        InterfaceFilter {
            name: name.as_ref().to_owned(),
            kind,
        }
    }

    pub fn matches(&self, interface: &Interface) -> bool {
        let name_match = if self.name.is_empty() {
            true
        } else {
            self.name == interface.name
        };

        match self.kind {
            None => name_match,
            Some(k) => name_match && k == interface.kind,
        }
    }
}

impl ToString for InterfaceFilter {
    fn to_string(&self) -> String {
        match self.kind {
            Some(kind) => format!("{}:{}", self.name, kind.to_string()),
            None => self.name.clone(),
        }
    }
}

impl FromStr for InterfaceFilter {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let d = ':';
        if !s.contains(d) {
            return Ok(InterfaceFilter::new(s, None));
        }

        // SAFETY: we just checked for the delimiter above
        let (name, kind) = s.split_once(d).unwrap();
        match kind.parse() {
            Ok(kind) => Ok(InterfaceFilter::new(name, Some(kind))),
            Err(e) => Err(e),
        }
    }
}

impl Serialize for InterfaceFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for InterfaceFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.parse::<InterfaceFilter>() {
            Ok(value) => Ok(value),
            Err(e) => Err(de::Error::custom(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;

    #[test]
    fn interface_filter_to_string() {
        use InterfaceFilter as F;

        assert_eq!(F::new("foo", None).to_string(), "foo");
        assert_eq!(F::new("bar", Some(InterfaceKind::V4)).to_string(), "bar:v4");
        assert_eq!(F::new("baz", Some(InterfaceKind::V6)).to_string(), "baz:v6");
        assert_eq!(F::new("", None).to_string(), "");
        assert_eq!(F::new("", Some(InterfaceKind::V4)).to_string(), ":v4");
        assert_eq!(F::new("", Some(InterfaceKind::V6)).to_string(), ":v6");
    }

    #[test]
    fn interface_filter_from_str() {
        use InterfaceFilter as F;

        let p = |s: &str| s.parse::<F>().unwrap();
        assert_eq!(p("foo"), F::new("foo", None));
        assert_eq!(p("bar:v4"), F::new("bar", Some(InterfaceKind::V4)));
        assert_eq!(p("baz:v6"), F::new("baz", Some(InterfaceKind::V6)));
        assert_eq!(p(""), F::new("", None));
        assert_eq!(p(":v4"), F::new("", Some(InterfaceKind::V4)));
        assert_eq!(p(":v6"), F::new("", Some(InterfaceKind::V6)));
    }

    #[test]
    fn interface_filter_ser() {
        let to_s = |i| serde_json::to_value(&i).unwrap();

        assert_eq!(to_s(InterfaceFilter::new("foo", None)), "foo");
        assert_eq!(
            to_s(InterfaceFilter::new("bar", Some(InterfaceKind::V4))),
            "bar:v4"
        );
        assert_eq!(
            to_s(InterfaceFilter::new("baz", Some(InterfaceKind::V6))),
            "baz:v6"
        );
        assert_eq!(to_s(InterfaceFilter::new("", None)), "");
        assert_eq!(
            to_s(InterfaceFilter::new("", Some(InterfaceKind::V4))),
            ":v4"
        );
        assert_eq!(
            to_s(InterfaceFilter::new("", Some(InterfaceKind::V6))),
            ":v6"
        );
    }

    #[test]
    fn interface_filter_de() {
        let from_s =
            |s: &str| match serde_json::from_value::<InterfaceFilter>(Value::String(s.into())) {
                Ok(x) => x,
                Err(e) => panic!("input: {}, error: {}", s, e),
            };

        assert_eq!(from_s("foo"), InterfaceFilter::new("foo", None));
        assert_eq!(
            from_s("bar:v4"),
            InterfaceFilter::new("bar", Some(InterfaceKind::V4))
        );
        assert_eq!(
            from_s("baz:v6"),
            InterfaceFilter::new("baz", Some(InterfaceKind::V6))
        );
        assert_eq!(from_s(""), InterfaceFilter::new("", None));
        assert_eq!(
            from_s(":v4"),
            InterfaceFilter::new("", Some(InterfaceKind::V4))
        );
        assert_eq!(
            from_s(":v6"),
            InterfaceFilter::new("", Some(InterfaceKind::V6))
        );
    }
}
