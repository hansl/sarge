use std::collections::{HashMap, HashSet};

#[derive(Clone, PartialEq)]
pub enum ArgumentType {
    /// This argument does not take type. It is completely ignored.
    None,

    /// This argument accepts any type.
    Any,

    /// This argument is a flag; it is either there or not.
    Bool,

    /// This argument accepts an integer as a value.
    Integer,

    /// This argument accepts a float as a value.
    Float,

    /// This argument accepts a string as a value.
    String,

    /// This argument can be specified multiple times.
    List,
    Map,
}

#[derive(Clone)]
pub enum ArgumentValue {
    None,
    Bool(bool),
    Integer(u64),
    Float(f64),
    String(String),

    List(HashSet<ArgumentValue>),
    Map(HashMap<String, ArgumentValue>),
}

impl PartialEq for ArgumentValue {
    fn eq(&self, other: &ArgumentValue) -> bool {
        use ArgumentValue::*;
        match self {
            None | Bool(_) | Integer(_) | String(_) | List(_) | Map(_) => self == other,
            Float(x) => {
                match other {
                    Float(y) => f64::abs(x - y) < 0.000001,
                    _ => false,
                }
            }
        }
    }
}
impl From<bool> for ArgumentValue {
    fn from(v: bool) -> ArgumentValue { ArgumentValue::Bool(v) }
}
impl From<u64> for ArgumentValue {
    fn from(v: u64) -> ArgumentValue { ArgumentValue::Integer(v) }
}
impl From<f64> for ArgumentValue {
    fn from(v: f64) -> ArgumentValue { ArgumentValue::Float(v) }
}
impl From<&str> for ArgumentValue {
    fn from(v: &str) -> ArgumentValue { ArgumentValue::String(String::from(v)) }
}
impl From<String> for ArgumentValue {
    fn from(v: String) -> ArgumentValue { ArgumentValue::String(v) }
}

#[derive(Default, Clone)]
pub struct OptionDescription {
    name_: String,

    long_: HashSet<String>,
    short_: HashSet<char>,

    description_: String,
    usage_: Option<String>,
    disabled_: Option<String>,
    deprecated_: Option<String>,

    argtype_: Option<ArgumentType>,
    subtype_: Option<ArgumentType>,
    positional_: Option<u32>,
    values_: Option<Vec<ArgumentValue>>,

    validator_: Option<fn(&ArgumentValue) -> Result<(), String>>,
    default_: Option<fn(&ArgumentValue) -> Result<(), String>>,
}

impl PartialEq for OptionDescription {
    fn eq(&self, other: &OptionDescription) -> bool {
        // Two descriptions are equal if some of their fields are equal.
        self.name_ == other.name_
        && self.argtype_ == other.argtype_
        && self.subtype_ == other.subtype_
        && self.positional_ == other.positional_
        && self.disabled_.is_none() == other.disabled_.is_none()
        && self.deprecated_.is_none() == other.deprecated_.is_none()
    }
}

impl OptionDescription {
    pub fn new<T: AsRef<str>>(name: T) -> OptionDescription {
        OptionDescription {
            name_: name.as_ref().into(),
            ..OptionDescription::default()
        }
    }

    pub fn long<T: AsRef<str>>(&mut self, long: T) -> &Self {
        self.long_.insert(long.as_ref().into());
        self
    }
    pub fn short(&mut self, short: char) -> &Self {
        self.short_.insert(short);
        self
    }

    pub fn description<T: AsRef<str>>(&mut self, description: T) -> &Self {
        self.description_ = description.as_ref().into();
        self
    }

    pub fn usage<T: AsRef<str>>(&mut self, usage: T) -> &Self {
        self.usage_ = Some(usage.as_ref().into());
        self
    }

    pub fn disabled<T: AsRef<str>>(&mut self, disabled: T) -> &Self {
        self.disabled_ = Some(disabled.as_ref().into());
        self
    }

    pub fn deprecated<T: AsRef<str>>(&mut self, deprecated: T) -> &Self {
        self.deprecated_ = Some(deprecated.as_ref().into());
        self
    }

    pub fn argtype(&mut self, argtype: ArgumentType) -> &Self {
        self.argtype_ = Some(argtype);
        self
    }
}


#[cfg(test)]
mod mod_tests;
