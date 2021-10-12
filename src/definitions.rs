use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fmt::Display,
    hash::Hash,
};

use num_bigint::BigInt;
use num_traits::ToPrimitive;
use ordered_float::OrderedFloat;

use crate::{error::Error, funtions::eval_list};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefinitionTypes {
    Symbol(String),
    Keyword(String),
    String(String),
    Char(char),
    Bool(bool),
    Double(OrderedFloat<f64>),
    Int(BigInt),
    Rational(BigInt, BigInt),
    HashSet(HashSet<DefinitionTypes>),
    OrderedSet(BTreeSet<DefinitionTypes>),
    HashMap(HashMap<DefinitionTypes, DefinitionTypes>),
    OrderedMap(BTreeMap<DefinitionTypes, DefinitionTypes>),
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
            }
            DefinitionTypes::HashMap(map) => {
                let mut map = map
                    .into_iter()
                    .collect::<Vec<(&DefinitionTypes, &DefinitionTypes)>>();
                map.sort();
                state.write(format!("HM={:?}", map).as_bytes())
            }
            _ => state.write(format!("{:?}", self).as_bytes()),
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
            }
            DefinitionTypes::HashMap(map) => {
                let mut map = map
                    .into_iter()
                    .collect::<Vec<(&DefinitionTypes, &DefinitionTypes)>>();
                map.sort();
                write!(f, "HM={:?}", map)
            }
            _ => write!(f, "{:?}", self),
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

impl DefinitionTypes {
    pub fn print(&self) -> Result<String, Error> {
        let res = match self.clone() {
            DefinitionTypes::Symbol(el) => format!("{}", el),
            DefinitionTypes::Keyword(el) => format!(":{}", el),
            DefinitionTypes::String(el) => format!("\"{}\"", el),
            DefinitionTypes::Char(el) => format!("\\{}", el),
            DefinitionTypes::Bool(el) => format!("{}", el),
            DefinitionTypes::Double(el) => format!("{}", el.0),
            DefinitionTypes::Int(el) => format!("{}", el),
            DefinitionTypes::Rational(num, den) => format!("{}/{}", num, den),
            DefinitionTypes::Nil => format!("nil",),

            DefinitionTypes::HashSet(set) => {
                let mut s = String::from("#{");
                for el in set {
                    s.push_str(&el.print()?);
                    s.push_str(" ");
                }
                s.push('}');
                s
            }
            DefinitionTypes::OrderedSet(set) => {
                let mut s = String::from("#{");
                for el in set {
                    s.push_str(&el.print()?);
                    s.push_str(" ");
                }
                s.push('}');
                s
            }
            DefinitionTypes::Vector(vec) => {
                let mut s = String::from('[');
                for el in vec {
                    s.push_str(&el.print()?);
                    s.push_str(" ");
                }
                s.push(']');
                s
            }
            DefinitionTypes::HashMap(map) => {
                let mut s = String::from('{');
                for (key, val) in map {
                    s.push_str(&key.print()?);
                    s.push_str(" ");
                    s.push_str(&val.print()?);
                    s.push_str(" ");
                }
                s.push('}');
                s
            }
            DefinitionTypes::OrderedMap(map) => {
                let mut s = String::from('{');
                for (key, val) in map {
                    s.push_str(&key.print()?);
                    s.push_str(" ");
                    s.push_str(&val.print()?);
                    s.push_str(" ");
                }
                s.push('}');
                s
            }
            DefinitionTypes::List(mut list) => eval_list(&mut list)?,
        };

        Ok(res)
    }
}

use std::ops;

impl ops::Add for DefinitionTypes {
    type Output = Result<Self, Error>;

    fn add(self, rhs: Self) -> Self::Output {
        let res = match self {
            DefinitionTypes::Symbol(_) => todo!(),
            DefinitionTypes::Keyword(_) => Err(Error::CantEval(Some(String::from(
                "Can't eval add of keyword",
            )))),
            DefinitionTypes::String(s) => {
                if let DefinitionTypes::String(rhs_s) = rhs {
                    let s = String::new() + &s + &rhs_s;
                    Ok(DefinitionTypes::String(s))
                } else {
                    Err(Error::CantEval(Some(String::from(
                        "Can't add non-string to string using `+`",
                    ))))
                }
            }
            DefinitionTypes::Char(_) => Err(Error::CantEval(Some(String::from(
                "Can't eval add of char",
            )))),
            DefinitionTypes::Bool(_) => Err(Error::CantEval(Some(String::from(
                "Can't eval add of bool",
            )))),
            DefinitionTypes::Double(num) => match rhs {
                DefinitionTypes::Double(rhs_num) => Ok(DefinitionTypes::Double(num + rhs_num)),
                DefinitionTypes::Int(rhs_num) => Ok(DefinitionTypes::Double(
                    (num.0 + rhs_num.to_f64().ok_or_else(|| Error::IntParseError)?).into(),
                )),
                DefinitionTypes::Rational(rhs_num, rhs_den) => Ok(DefinitionTypes::Double(
                    (((num.0 * rhs_den.to_f64().ok_or_else(|| Error::IntParseError)?)
                        + rhs_num.to_f64().ok_or_else(|| Error::IntParseError)?)
                        / rhs_den.to_f64().ok_or_else(|| Error::IntParseError)?)
                    .into(),
                )),
                DefinitionTypes::Nil => Ok(DefinitionTypes::Nil),
                _ => Err(Error::CantEval(Some(String::from(
                    "Can't add non-numeric to numeric using `+`",
                )))),
            },
            DefinitionTypes::Int(num) => match rhs {
                DefinitionTypes::Double(rhs_num) => Ok(DefinitionTypes::Double(
                    (num.to_f64().ok_or_else(|| Error::IntParseError)? + rhs_num.0).into(),
                )),
                DefinitionTypes::Int(rhs_num) => Ok(DefinitionTypes::Int(num + rhs_num)),
                DefinitionTypes::Rational(rhs_num, rhs_den) => Ok(DefinitionTypes::Rational(
                    rhs_num + (num * &rhs_den),
                    rhs_den,
                )),
                DefinitionTypes::Nil => Ok(DefinitionTypes::Nil),
                _ => Err(Error::CantEval(Some(String::from(
                    "Can't add non-numeric to numeric using `+`",
                )))),
            },
            DefinitionTypes::Rational(num, den) => match rhs {
                DefinitionTypes::Double(rhs_num) => Ok(DefinitionTypes::Double(
                    ((num.to_f64().ok_or_else(|| Error::IntParseError)?
                        / den.to_f64().ok_or_else(|| Error::IntParseError)?)
                        + rhs_num.0)
                        .into(),
                )),
                DefinitionTypes::Int(rhs_num) => {
                    Ok(DefinitionTypes::Rational(num + (rhs_num * &den), den))
                }
                DefinitionTypes::Rational(rhs_num, rhs_den) => Ok(DefinitionTypes::Rational(
                    (rhs_num * &den) + (num * &rhs_den),
                    rhs_den * den,
                )),
                DefinitionTypes::Nil => Ok(DefinitionTypes::Nil),
                _ => Err(Error::CantEval(Some(String::from(
                    "Can't add non-numeric to numeric using `+`",
                )))),
            },
            DefinitionTypes::HashSet(v) => {
                if let DefinitionTypes::HashSet(rhs_v) = rhs {
                    let mut v = v.clone();
                    for k in rhs_v {
                        v.insert(k);
                    }
                    Ok(DefinitionTypes::HashSet(v))
                } else {
                    Err(Error::CantEval(Some(String::from(
                        "Can't add non-ordered-map to ordered-map using `+`",
                    ))))
                }
            }
            DefinitionTypes::OrderedSet(v) => {
                if let DefinitionTypes::OrderedSet(rhs_v) = rhs {
                    let mut v = v.clone();
                    let mut rhs_v = rhs_v.clone();
                    v.append(&mut rhs_v);
                    Ok(DefinitionTypes::OrderedSet(v))
                } else {
                    Err(Error::CantEval(Some(String::from(
                        "Can't add non-ordered-map to ordered-map using `+`",
                    ))))
                }
            }
            DefinitionTypes::HashMap(v) => {
                if let DefinitionTypes::HashMap(rhs_v) = rhs {
                    let mut v = v.clone();
                    for (k, val) in rhs_v {
                        v.insert(k, val);
                    }
                    Ok(DefinitionTypes::HashMap(v))
                } else {
                    Err(Error::CantEval(Some(String::from(
                        "Can't add non-hash-map to hash-map using `+`",
                    ))))
                }
            }
            DefinitionTypes::OrderedMap(v) => {
                if let DefinitionTypes::OrderedMap(rhs_v) = rhs {
                    let mut v = v.clone();
                    let mut rhs_v = rhs_v.clone();
                    v.append(&mut rhs_v);
                    Ok(DefinitionTypes::OrderedMap(v))
                } else {
                    Err(Error::CantEval(Some(String::from(
                        "Can't add non-ordered-map to ordered-map using `+`",
                    ))))
                }
            }
            DefinitionTypes::List(_) => todo!("eval list not implemented"),
            DefinitionTypes::Vector(v) => {
                if let DefinitionTypes::Vector(rhs_v) = rhs {
                    let mut v = v.clone();
                    let mut rhs_v = rhs_v.clone();
                    v.append(&mut rhs_v);
                    Ok(DefinitionTypes::Vector(v))
                } else {
                    Err(Error::CantEval(Some(String::from(
                        "Can't add non-vector to vector using `+`",
                    ))))
                }
            }
            DefinitionTypes::Nil => Ok(DefinitionTypes::Nil),
        }?;

        Ok(res)
    }
}

impl ops::Sub for DefinitionTypes {
    type Output = Result<Self, Error>;

    fn sub(self, rhs: Self) -> Self::Output {
        let res = match self {
            DefinitionTypes::Symbol(_) => todo!(),
            DefinitionTypes::Keyword(_) => Err(Error::CantEval(Some(String::from(
                "Can't eval sub of keyword",
            )))),
            DefinitionTypes::String(_) => Err(Error::CantEval(Some(String::from(
                "Can't eval sub of keyword",
            )))),
            DefinitionTypes::Char(_) => Err(Error::CantEval(Some(String::from(
                "Can't eval sub of char",
            )))),
            DefinitionTypes::Bool(_) => Err(Error::CantEval(Some(String::from(
                "Can't eval sub of bool",
            )))),
            DefinitionTypes::Double(num) => match rhs {
                DefinitionTypes::Double(rhs_num) => Ok(DefinitionTypes::Double(num - rhs_num)),
                DefinitionTypes::Int(rhs_num) => Ok(DefinitionTypes::Double(
                    (num.0 - rhs_num.to_f64().ok_or_else(|| Error::IntParseError)?).into(),
                )),
                DefinitionTypes::Rational(rhs_num, rhs_den) => Ok(DefinitionTypes::Double(
                    (((num.0 * rhs_den.to_f64().ok_or_else(|| Error::IntParseError)?)
                        - rhs_num.to_f64().ok_or_else(|| Error::IntParseError)?)
                        / rhs_den.to_f64().ok_or_else(|| Error::IntParseError)?)
                    .into(),
                )),
                DefinitionTypes::Nil => Ok(DefinitionTypes::Nil),
                _ => Err(Error::CantEval(Some(String::from(
                    "Can't sub non-numeric to numeric using `+`",
                )))),
            },
            DefinitionTypes::Int(num) => match rhs {
                DefinitionTypes::Double(rhs_num) => Ok(DefinitionTypes::Double(
                    (num.to_f64().ok_or_else(|| Error::IntParseError)? - rhs_num.0).into(),
                )),
                DefinitionTypes::Int(rhs_num) => Ok(DefinitionTypes::Int(num - rhs_num)),
                DefinitionTypes::Rational(rhs_num, rhs_den) => Ok(DefinitionTypes::Rational(
                    (num * &rhs_den) - rhs_num,
                    rhs_den,
                )),
                DefinitionTypes::Nil => Ok(DefinitionTypes::Nil),
                _ => Err(Error::CantEval(Some(String::from(
                    "Can't sub non-numeric to numeric using `+`",
                )))),
            },
            DefinitionTypes::Rational(num, den) => match rhs {
                DefinitionTypes::Double(rhs_num) => Ok(DefinitionTypes::Double(
                    ((num.to_f64().ok_or_else(|| Error::IntParseError)?
                        / den.to_f64().ok_or_else(|| Error::IntParseError)?)
                        - rhs_num.0)
                        .into(),
                )),
                DefinitionTypes::Int(rhs_num) => {
                    Ok(DefinitionTypes::Rational(num - (rhs_num * &den), den))
                }
                DefinitionTypes::Rational(rhs_num, rhs_den) => Ok(DefinitionTypes::Rational(
                    (num * &rhs_den) - (rhs_num * &den),
                    rhs_den * den,
                )),
                DefinitionTypes::Nil => Ok(DefinitionTypes::Nil),
                _ => Err(Error::CantEval(Some(String::from(
                    "Can't sub non-numeric to numeric using `+`",
                )))),
            },
            DefinitionTypes::HashSet(v) => {
                if let DefinitionTypes::HashSet(rhs_v) = rhs {
                    let mut v = v.clone();
                    for k in rhs_v {
                        v.insert(k);
                    }
                    Ok(DefinitionTypes::HashSet(v))
                } else {
                    Err(Error::CantEval(Some(String::from(
                        "Can't add non-ordered-map to ordered-map using `+`",
                    ))))
                }
            }
            DefinitionTypes::OrderedSet(v) => {
                if let DefinitionTypes::OrderedSet(rhs_v) = rhs {
                    let mut v = v.clone();
                    let mut rhs_v = rhs_v.clone();
                    v.append(&mut rhs_v);
                    Ok(DefinitionTypes::OrderedSet(v))
                } else {
                    Err(Error::CantEval(Some(String::from(
                        "Can't add non-ordered-map to ordered-map using `+`",
                    ))))
                }
            }
            DefinitionTypes::HashMap(v) => {
                if let DefinitionTypes::HashMap(rhs_v) = rhs {
                    let mut v = v.clone();
                    for (k, val) in rhs_v {
                        v.insert(k, val);
                    }
                    Ok(DefinitionTypes::HashMap(v))
                } else {
                    Err(Error::CantEval(Some(String::from(
                        "Can't add non-hash-map to hash-map using `+`",
                    ))))
                }
            }
            DefinitionTypes::OrderedMap(v) => {
                if let DefinitionTypes::OrderedMap(rhs_v) = rhs {
                    let mut v = v.clone();
                    let mut rhs_v = rhs_v.clone();
                    v.append(&mut rhs_v);
                    Ok(DefinitionTypes::OrderedMap(v))
                } else {
                    Err(Error::CantEval(Some(String::from(
                        "Can't add non-ordered-map to ordered-map using `+`",
                    ))))
                }
            }
            DefinitionTypes::List(_) => todo!("eval list not implemented"),
            DefinitionTypes::Vector(v) => {
                if let DefinitionTypes::Vector(rhs_v) = rhs {
                    let mut v = v.clone();
                    let mut rhs_v = rhs_v.clone();
                    v.append(&mut rhs_v);
                    Ok(DefinitionTypes::Vector(v))
                } else {
                    Err(Error::CantEval(Some(String::from(
                        "Can't add non-vector to vector using `+`",
                    ))))
                }
            }
            DefinitionTypes::Nil => Ok(DefinitionTypes::Nil),
        }?;

        Ok(res)
    }
}
