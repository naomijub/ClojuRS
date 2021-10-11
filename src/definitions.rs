use std::{cmp::Ordering, collections::{BTreeMap, BTreeSet, HashMap, HashSet}, fmt::Display, hash::Hash};

use ordered_float::OrderedFloat;

use crate::{Env, funtions::eval_list};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefinitionTypes {
  Symbol(String),
  Keyword(String),
  String(String),
  Char(char),
  Bool(bool),
  Double(OrderedFloat<f64>),
  Float(OrderedFloat<f32>),
  UnsignedLong(u128),
  Long(i128),
  UnsignedInt(u64),
  Int(i64),
  UnsignedShort(u32),
  Short(i32),
  Byte(u8),
  SignedByte(i8),
  Rational(i128, u128),
  HashSet(HashSet<DefinitionTypes>),
  OrderedSet(BTreeSet<DefinitionTypes>),
  HashMap(HashMap<DefinitionTypes, DefinitionTypes>),
  OrderedMap(BTreeMap<DefinitionTypes,DefinitionTypes>),
  List(Vec<DefinitionTypes>),
  Vector(Vec<DefinitionTypes>),
  Nil,
} 

impl Hash for DefinitionTypes {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            DefinitionTypes::HashSet(set) => {
                let set = set.into_iter().collect::<Vec<&DefinitionTypes>>();
                state.write(format!("HS={:?}", set).as_bytes())
            },
            DefinitionTypes::HashMap(map) => {
                let mut map = map.into_iter().collect::<Vec<(&DefinitionTypes, &DefinitionTypes)>>();
                map.sort();
                state.write(format!("HM={:?}", map).as_bytes())
            },
            _ => state.write(format!("{:?}", self).as_bytes())

        }
        
        state.finish();
    }
}

impl Display for DefinitionTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DefinitionTypes::Keyword(key) => write!(f, ":{}", key),
            DefinitionTypes::Rational(num, den) => write!(f, "{}/{}", num, den),
            DefinitionTypes::HashSet(set) => {
                let set = set.into_iter().collect::<Vec<&DefinitionTypes>>();
                write!(f, "HS={:?}", set)
            },
            DefinitionTypes::HashMap(map) => {
                let mut map = map.into_iter().collect::<Vec<(&DefinitionTypes, &DefinitionTypes)>>();
                map.sort();
                write!(f, "HM={:?}", map)
            },
            _ => write!(f, "{:?}", self)
        }
    }
}

impl Ord for DefinitionTypes {
    fn cmp(&self, other: &Self) -> Ordering {
        let s = self.to_string();
        let o = other.to_string();
        s.cmp(&o)
    }
}

impl PartialOrd for DefinitionTypes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let s = self.to_string();
        let o = other.to_string();
        Some(s.cmp(&o))
    }
}

#[derive(Debug)]
pub enum Error {
    Reason(String),
    ArityException(u16, String),
    UnknownSymbol,
    CantEval(Option<String>)
}

impl From<std::num::ParseIntError> for Error {
    fn from(s: std::num::ParseIntError) -> Self {
        Error::Reason(s.to_string())
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(s: std::num::ParseFloatError) -> Self {
        Error::Reason(s.to_string())
    }
}

impl From<std::str::ParseBoolError> for Error {
    fn from(s: std::str::ParseBoolError) -> Self {
        Error::Reason(s.to_string())
    }
}

impl DefinitionTypes {
    pub fn print(&self, env: &mut Env) -> Result<String, Error> {
        let res = match self.clone() {
            DefinitionTypes::Symbol(el) => format!("{}", el),
            DefinitionTypes::Keyword(el) => format!(":{}", el),
            DefinitionTypes::String(el) => format!("\"{}\"", el),
            DefinitionTypes::Char(el) => format!("\\{}", el),
            DefinitionTypes::Bool(el) => format!("{}", el),
            DefinitionTypes::Double(el) => format!("{}", el.0),
            DefinitionTypes::Float(el) => format!("{}", el.0),
            DefinitionTypes::UnsignedLong(el) => format!("{}", el),
            DefinitionTypes::Long(el) => format!("{}", el),
            DefinitionTypes::UnsignedInt(el) => format!("{}", el),
            DefinitionTypes::Int(el) => format!("{}", el),
            DefinitionTypes::UnsignedShort(el) => format!("{}", el),
            DefinitionTypes::Short(el) => format!("{}", el),
            DefinitionTypes::Byte(el) => format!("{}", el),
            DefinitionTypes::SignedByte(el) => format!("{}", el),
            DefinitionTypes::Rational(num, den) => format!("{}/{}", num, den),
            DefinitionTypes::Nil => format!("nil",),

            DefinitionTypes::HashSet(set) => {
                let mut s = String::from("#{");
                for el in set {
                    s.push_str(&el.print(env)?);
                    s.push_str(" ");
                }
                s.push('}');
                s
            },
            DefinitionTypes::OrderedSet(set) => {
                let mut s = String::from("#{");
                for el in set {
                    s.push_str(&el.print(env)?);
                    s.push_str(" ");
                }
                s.push('}');
                s
            },
            DefinitionTypes::Vector(vec) => {
                let mut s = String::from('[');
                for el in vec {
                    s.push_str(&el.print(env)?);
                    s.push_str(" ");
                }
                s.push(']');
                s
            },
            DefinitionTypes::HashMap(map) => {
                let mut s = String::from('{');
                for (key, val) in map {
                    s.push_str(&key.print(env)?);
                    s.push_str(" ");
                    s.push_str(&val.print(env)?);
                    s.push_str(" ");
                }
                s.push('}');
                s
            },
            DefinitionTypes::OrderedMap(map) => {
                let mut s = String::from('{');
                for (key, val) in map {
                    s.push_str(&key.print(env)?);
                    s.push_str(" ");
                    s.push_str(&val.print(env)?);
                    s.push_str(" ");
                }
                s.push('}');
                s
            },
            DefinitionTypes::List(mut list) => eval_list(&mut list, env)?, 
        };

        Ok(res)
    }

}