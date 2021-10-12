use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use num_bigint::BigInt;
use ordered_float::OrderedFloat;

use crate::{definitions::DefinitionTypes, error::Error};

pub(crate) fn tokenize(exp: &str) -> std::iter::Enumerate<std::str::Chars> {
    exp.chars().enumerate()
}

pub(crate) fn parse(
    c: Option<(usize, char)>,
    chars: &mut std::iter::Enumerate<std::str::Chars>,
) -> Result<DefinitionTypes, Error> {
    Ok(match c {
        Some((_, '[')) => read_vec(chars)?,
        Some((_, '(')) => read_list(chars)?,
        Some((_, '@')) => read_set(chars)?,
        Some((_, '{')) => read_map(chars)?,
        edn => parse_edn(edn, chars)?,
    })
}

pub(crate) fn parse_edn(
    c: Option<(usize, char)>,
    chars: &mut std::iter::Enumerate<std::str::Chars>,
) -> Result<DefinitionTypes, Error> {
    match c {
        Some((_, '\"')) => read_str(chars),
        Some((_, ':')) => read_key_or_nsmap(chars),
        Some((_, '-')) => Ok(read_number('-', chars)?),
        Some((_, '\\')) => Ok(read_char(chars)?),
        Some((_, b)) if b == 't' || b == 'f' || b == 'n' => Ok(read_bool_or_nil(b, chars)?),
        Some((_, n)) if n.is_numeric() => Ok(read_number(n, chars)?),
        Some((_, a)) => Ok(read_symbol(a, chars)?),
        None => Err(Error::Reason("Expression could not be parsed".to_string())),
    }
}

fn read_key_or_nsmap(
    chars: &mut std::iter::Enumerate<std::str::Chars>,
) -> Result<DefinitionTypes, Error> {
    let mut key_chars = chars.clone().take_while(|c| {
        !c.1.is_whitespace() && c.1 != ',' && c.1 != ')' && c.1 != ']' && c.1 != '}'
    });
    let c_len = key_chars.clone().count();

    Ok(match key_chars.find(|c| c.1 == '{') {
        Some(_) => {
            return Err(Error::Reason(String::from(
                "Namespace maps not yet supported",
            )))
        } //read_namespaced_map(chars)?,
        None => read_key(chars, c_len),
    })
}

fn read_key(chars: &mut std::iter::Enumerate<std::str::Chars>, c_len: usize) -> DefinitionTypes {
    let mut key = String::from(":");
    let key_chars = chars.take(c_len).map(|c| c.1).collect::<String>();
    key.push_str(&key_chars);
    DefinitionTypes::Keyword(key)
}

fn read_str(chars: &mut std::iter::Enumerate<std::str::Chars>) -> Result<DefinitionTypes, Error> {
    let result = chars.try_fold(
        (false, String::new()),
        |(last_was_escape, mut s), (_, c)| {
            if last_was_escape {
                // Supported escape characters, per https://github.com/edn-format/edn#strings
                match c {
                    't' => s.push('\t'),
                    'r' => s.push('\r'),
                    'n' => s.push('\n'),
                    '\\' => s.push('\\'),
                    '\"' => s.push('\"'),
                    _ => {
                        return Err(Err(Error::Reason(format!(
                            "Invalid escape sequence \\{}",
                            c
                        ))))
                    }
                };

                Ok((false, s))
            } else if c == '\"' {
                // Unescaped quote means we're done
                Err(Ok(s))
            } else if c == '\\' {
                Ok((true, s))
            } else {
                s.push(c);
                Ok((false, s))
            }
        },
    );

    match result {
        // An Ok means we actually finished parsing *without* seeing the end of the string, so that's
        // an error.
        Ok(_) => Err(Error::Reason("Unterminated string".to_string())),
        Err(Err(e)) => Err(e),
        Err(Ok(string)) => Ok(DefinitionTypes::String(string)),
    }
}

fn read_symbol(
    a: char,
    chars: &mut std::iter::Enumerate<std::str::Chars>,
) -> Result<DefinitionTypes, Error> {
    let c_len = chars
        .clone()
        .enumerate()
        .take_while(|&(i, c)| {
            i <= 200 && !c.1.is_whitespace() && c.1 != ')' && c.1 != '}' && c.1 != ']'
        })
        .count();
    let i = chars
        .clone()
        .next()
        .ok_or_else(|| Error::Reason("Could not identify symbol index".to_string()))?
        .0;

    if a.is_whitespace() {
        return Err(Error::Reason(format!(
            "\"{}\" could not be parsed at char count {}",
            a, i
        )));
    }

    let mut symbol = String::from(a);
    let symbol_chars = chars.take(c_len).map(|c| c.1).collect::<String>();
    symbol.push_str(&symbol_chars);
    Ok(DefinitionTypes::Symbol(symbol))
}

// TODO
fn read_number(
    n: char,
    chars: &mut std::iter::Enumerate<std::str::Chars>,
) -> Result<DefinitionTypes, Error> {
    let i = chars
        .clone()
        .next()
        .ok_or_else(|| Error::Reason("Could not identify symbol index".to_string()))?
        .0;
    let c_len = chars
        .clone()
        .take_while(|c| c.1.is_numeric() || c.1 == '.' || c.1 == '/')
        .count();
    let mut number = String::new();
    let string = chars.take(c_len).map(|c| c.1).collect::<String>();
    number.push(n);
    number.push_str(&string);

    match number {
        n if n.contains('/') && n.split('/').all(|d| d.parse::<BigInt>().is_ok()) => {
            let split = n.split('/').collect::<Vec<_>>();
            Ok(DefinitionTypes::Rational(
                split[0].parse::<BigInt>()?,
                split[1].parse::<BigInt>()?,
            ))
        }
        n if n.contains('.') => Ok(DefinitionTypes::Double(OrderedFloat::from_str(&n)?)),
        n if n.parse::<BigInt>().is_ok() => Ok(DefinitionTypes::Int(n.parse::<BigInt>().unwrap())),
        n if n.parse::<f64>().is_ok() => Ok(DefinitionTypes::Double(n.parse()?)),

        _ => Err(Error::Reason(format!(
            "{} could not be parsed at char count {}",
            number, i
        ))),
    }
}

fn read_char(chars: &mut std::iter::Enumerate<std::str::Chars>) -> Result<DefinitionTypes, Error> {
    let i = chars
        .clone()
        .next()
        .ok_or_else(|| Error::Reason("Could not identify symbol index".to_string()))?
        .0;
    let c = chars.next();
    c.ok_or(format!("{:?} could not be parsed at char count {}", c, i))
        .map(|c| c.1)
        .map(DefinitionTypes::Char)
        .map_err(Error::Reason)
}

fn read_bool_or_nil(
    c: char,
    chars: &mut std::iter::Enumerate<std::str::Chars>,
) -> Result<DefinitionTypes, Error> {
    let i = chars
        .clone()
        .next()
        .ok_or_else(|| Error::Reason("Could not identify symbol index".to_string()))?
        .0;
    match c.clone() {
        't' if {
            let val = chars.clone().take(4).map(|c| c.1).collect::<String>();
            val.eq("rue ")
                || val.eq("rue,")
                || val.eq("rue]")
                || val.eq("rue}")
                || val.eq("rue)")
                || val.eq("rue")
        } =>
        {
            let mut string = String::new();
            let t = chars.take(3).map(|c| c.1).collect::<String>();
            string.push(c);
            string.push_str(&t);
            Ok(DefinitionTypes::Bool(string.parse::<bool>()?))
        }
        'f' if {
            let val = chars.clone().take(5).map(|c| c.1).collect::<String>();
            val.eq("alse ")
                || val.eq("alse,")
                || val.eq("alse]")
                || val.eq("alse}")
                || val.eq("alse)")
                || val.eq("alse")
        } =>
        {
            let mut string = String::new();
            let f = chars.take(4).map(|c| c.1).collect::<String>();
            string.push(c);
            string.push_str(&f);
            Ok(DefinitionTypes::Bool(string.parse::<bool>()?))
        }
        'n' if {
            let val = chars.clone().take(3).map(|c| c.1).collect::<String>();
            val.eq("il ")
                || val.eq("il,")
                || val.eq("il]")
                || val.eq("il}")
                || val.eq("il)")
                || val.eq("il")
        } =>
        {
            let mut string = String::new();
            let n = chars.take(2).map(|c| c.1).collect::<String>();
            string.push(c);
            string.push_str(&n);
            match &string[..] {
                "nil" => Ok(DefinitionTypes::Nil),
                _ => Err(Error::Reason(format!(
                    "{} could not be parsed at char count {}",
                    string, i
                ))),
            }
        }
        _ => read_symbol(c, chars),
    }
}

fn read_vec(chars: &mut std::iter::Enumerate<std::str::Chars>) -> Result<DefinitionTypes, Error> {
    let i = chars
        .clone()
        .next()
        .ok_or_else(|| Error::Reason("Could not identify symbol index".to_string()))?
        .0;
    let mut res: Vec<DefinitionTypes> = vec![];
    loop {
        match chars.next() {
            Some((_, ']')) => return Ok(DefinitionTypes::Vector(res)),
            Some(c) if !c.1.is_whitespace() && c.1 != ',' => {
                res.push(parse(Some(c), chars)?);
            }
            Some(c) if c.1.is_whitespace() || c.1 == ',' => (),
            err => {
                return Err(Error::Reason(format!(
                    "{:?} could not be parsed at char count {}",
                    err, i
                )))
            }
        }
    }
}

fn read_list(chars: &mut std::iter::Enumerate<std::str::Chars>) -> Result<DefinitionTypes, Error> {
    let i = chars
        .clone()
        .next()
        .ok_or_else(|| Error::Reason("Could not identify symbol index".to_string()))?
        .0;
    let mut res: Vec<DefinitionTypes> = vec![];
    loop {
        match chars.next() {
            Some((_, ')')) => return Ok(DefinitionTypes::List(res)),
            Some(c) if !c.1.is_whitespace() && c.1 != ',' => {
                res.push(parse(Some(c), chars)?);
            }
            Some(c) if c.1.is_whitespace() || c.1 == ',' => (),
            err => {
                return Err(Error::Reason(format!(
                    "{:?} could not be parsed at char count {}",
                    err, i
                )))
            }
        }
    }
}

fn read_set(chars: &mut std::iter::Enumerate<std::str::Chars>) -> Result<DefinitionTypes, Error> {
    let i = chars
        .clone()
        .next()
        .ok_or_else(|| Error::Reason("Could not identify symbol index".to_string()))?
        .0;
    let mut res: HashSet<DefinitionTypes> = HashSet::new();
    loop {
        match chars.next() {
            Some((_, '}')) => return Ok(DefinitionTypes::HashSet(res)),
            Some(c) if !c.1.is_whitespace() && c.1 != ',' => {
                res.insert(parse(Some(c), chars)?);
            }
            Some(c) if c.1.is_whitespace() || c.1 == ',' => (),
            err => {
                return Err(Error::Reason(format!(
                    "{:?} could not be parsed at char count {}",
                    err, i
                )))
            }
        }
    }
}

// fn read_namespaced_map(chars: &mut std::iter::Enumerate<std::str::Chars>) -> Result<DefinitionTypes, Error> {
//     let i = chars
//         .clone()
//         .next()
//         .ok_or_else(|| Error::Reason("Could not identify symbol index".to_string()))?
//         .0;
//     use std::collections::BTreeMap;
//     let mut res: BTreeMap<String, DefinitionTypes> = BTreeMap::new();
//     let mut key: Option<DefinitionTypes> = None;
//     let mut val: Option<DefinitionTypes> = None;
//     let namespace = chars
//         .take_while(|c| c.1 != '{')
//         .map(|c| c.1)
//         .collect::<String>();

//     loop {
//         match chars.next() {
//             Some((_, '}')) => return Ok(DefinitionTypes::NamespacedMap(namespace, res)),
//             Some(c) if !c.1.is_whitespace() && c.1 != ',' => {
//                 if key.is_some() {
//                     val = Some(parse(Some(c), chars)?);
//                 } else {
//                     key = Some(parse(Some(c), chars)?);
//                 }
//             }
//             Some(c) if c.1.is_whitespace() || c.1 == ',' => (),
//             err => {
//                 return Err(Error::Reason(format!(
//                     "{:?} could not be parsed at char count {}",
//                     err, i
//                 )))
//             }
//         }

//         if key.is_some() && val.is_some() {
//             res.insert(key.unwrap().to_string(), val.unwrap());
//             key = None;
//             val = None;
//         }
//     }
// }

fn read_map(chars: &mut std::iter::Enumerate<std::str::Chars>) -> Result<DefinitionTypes, Error> {
    let i = chars
        .clone()
        .next()
        .ok_or_else(|| Error::Reason("Could not identify symbol index".to_string()))?
        .0;
    let mut res: HashMap<DefinitionTypes, DefinitionTypes> = HashMap::new();
    let mut key: Option<DefinitionTypes> = None;
    let mut val: Option<DefinitionTypes> = None;
    loop {
        match chars.next() {
            Some((_, '}')) => return Ok(DefinitionTypes::HashMap(res)),
            Some(c) if !c.1.is_whitespace() && c.1 != ',' => {
                if key.is_some() {
                    val = Some(parse(Some(c), chars)?);
                } else {
                    key = Some(parse(Some(c), chars)?);
                }
            }
            Some(c) if c.1.is_whitespace() || c.1 == ',' => (),
            err => {
                return Err(Error::Reason(format!(
                    "{:?} could not be parsed at char count {}",
                    err, i
                )))
            }
        }

        if key.is_some() && val.is_some() {
            res.insert(key.unwrap(), val.unwrap());
            key = None;
            val = None;
        }
    }
}
