use std::collections::{BTreeMap, HashMap};

use num_traits::ToPrimitive;

use crate::{definitions::DefinitionTypes as T, error::Error, helper::arity_exception, DATA};

pub fn get(info: &[T]) -> Result<T, Error> {
    if let Some(value) = arity_exception("get", 3, info.len()) {
        return value;
    }

    match (info.get(0), info.get(1), info.get(2)) {
        (None, _, _) => Err(Error::Reason(String::from(
            "Collection is required for get",
        ))),
        (_, None, _) => Err(Error::Reason(String::from(
            "Access index is required for get",
        ))),
        (Some(collection), Some(index), not_found) => match collection {
            T::String(s) => match (index.to_usize(), not_found) {
                (None, Some(nf)) => Ok(nf.clone()),
                (None, None) => Err(Error::IntParseError),
                (Some(idx), None) => {
                    Ok(T::Char(s.chars().nth(idx).ok_or_else(|| {
                        Error::Reason("Index out of bounds".to_string())
                    })?))
                }
                (Some(idx), Some(nf)) => {
                    if s.len() > idx {
                        Ok(T::Char(s.chars().nth(idx).ok_or_else(|| {
                            Error::Reason("Index out of bounds".to_string())
                        })?))
                    } else {
                        Ok(nf.clone())
                    }
                }
            },
            T::HashSet(hs) => {
                if hs.contains(index) {
                    Ok(index.clone())
                } else if not_found.is_some() {
                    Ok(not_found.unwrap().clone())
                } else {
                    Ok(T::Nil)
                }
            }
            T::OrderedSet(os) => {
                if os.contains(index) {
                    Ok(index.clone())
                } else if not_found.is_some() {
                    Ok(not_found.unwrap().clone())
                } else {
                    Ok(T::Nil)
                }
            }
            T::HashMap(hm) => {
                if hm.contains_key(index) {
                    Ok(hm.get(index).unwrap().clone())
                } else if not_found.is_some() {
                    Ok(not_found.unwrap().clone())
                } else {
                    Ok(T::Nil)
                }
            }
            T::OrderedMap(om) => {
                if om.contains_key(index) {
                    Ok(om.get(index).unwrap().clone())
                } else if not_found.is_some() {
                    Ok(not_found.unwrap().clone())
                } else {
                    Ok(T::Nil)
                }
            }
            T::List(_) => {
                let l = collection.to_owned().eval()?;
                if let Some(nf) = not_found {
                    get(&[l, index.to_owned(), nf.to_owned()])
                } else {
                    get(&[l, index.to_owned()])
                }
            }
            T::Vector(v) => match index {
                T::Symbol(sym) => {
                    Ok(DATA
                        .lock()
                        .map_or(Err(Error::CantEval(Some(sym.clone()))), |e| {
                            e.get(sym)
                                .map(|inner_e| inner_e.to_owned())
                                .ok_or_else(|| Error::CantEval(Some(sym.clone())))
                        })?)
                }
                T::Int(idx) => {
                    if let Some(idx) = idx.to_usize() {
                        if v.len() > idx {
                            Ok(v[idx].clone())
                        } else if not_found.is_some() {
                            Ok(not_found.unwrap().clone())
                        } else {
                            Err(Error::Reason(String::from("Index out of bounds")))
                        }
                    } else {
                        Err(Error::CantEval(Some(format!(
                            "Can't eval {} at index {}",
                            collection.print()?,
                            &idx
                        ))))
                    }
                }
                _ => Err(Error::Reason(String::from("Index out of bounds"))),
            },
            _ => Err(Error::Reason(String::from(
                "First argument must be a collection for get",
            ))),
        },
    }
}

pub fn to_vector(list: &[T]) -> Result<T, Error> {
    Ok(T::Vector(list.iter().map(|e| e.to_owned()).collect()))
}

pub fn to_hashset(list: &[T]) -> Result<T, Error> {
    Ok(T::HashSet(
        list.iter().filter_map(|e| e.clone().eval().ok()).collect(),
    ))
}

pub fn to_orderedset(list: &[T]) -> Result<T, Error> {
    Ok(T::OrderedSet(
        list.iter().filter_map(|e| e.clone().eval().ok()).collect(),
    ))
}

pub fn to_hashmap(list: &[T]) -> Result<T, Error> {
    if list.len() % 2 == 0 {
        Ok(T::HashMap(
            list.chunks(2)
                .map(|e| Ok((e[0].clone().eval()?, e[1].clone().eval()?)))
                .collect::<Result<HashMap<T, T>, Error>>()?,
        ))
    } else {
        Err(Error::Reason(String::from(
            "Hash map must be formed by pairs",
        )))
    }
}

pub fn to_orderedmap(list: &[T]) -> Result<T, Error> {
    if list.len() % 2 == 0 {
        Ok(T::OrderedMap(
            list.chunks(2)
                .map(|e| Ok((e[0].clone().eval()?, e[1].clone().eval()?)))
                .collect::<Result<BTreeMap<T, T>, Error>>()?,
        ))
    } else {
        Err(Error::Reason(String::from(
            "Sorted map must be formed by pairs",
        )))
    }
}

pub fn assoc(info: &[T]) -> Result<T, Error> {
    if let Some(value) = arity_exception("assoc", 3, info.len()) {
        return value;
    }
    match (info.get(0), info.get(1), info.get(2)) {
        (None, _, _) => Err(Error::Reason(String::from(
            "Collection is required for assoc",
        ))),
        (_, None, _) => Err(Error::Reason(String::from(
            "Access index/key is required for assoc",
        ))),
        (Some(collection), Some(key), Some(value)) => match collection.clone().eval()? {
            T::Vector(v) => {
                if let T::Int(index) = key {
                    let idx = index.to_usize();
                    match idx {
                        Some(i) if i > v.len() => Err(Error::Reason(
                            "Index must be inside vector's bound + 1".to_owned(),
                        )),
                        Some(i) if i == v.len() => {
                            let mut v = v.clone();
                            v.push(value.clone());
                            Ok(T::Vector(v))
                        }
                        _ => {
                            let mut v = v.clone();
                            v[idx.ok_or(Error::IntParseError)?] = value.clone();
                            Ok(T::Vector(v))
                        }
                    }
                } else {
                    Err(Error::Reason(String::from("Index must be of type int")))
                }
            }
            T::HashMap(hm) => {
                let mut hm = hm.clone();
                hm.entry(key.clone())
                    .and_modify(|e| *e = value.clone())
                    .or_insert_with(|| value.clone());
                Ok(T::HashMap(hm))
            }
            T::OrderedMap(om) => {
                let mut om = om.clone();
                om.entry(key.clone())
                    .and_modify(|e| *e = value.clone())
                    .or_insert_with(|| value.clone());
                Ok(T::OrderedMap(om))
            }
            _ => Err(Error::Reason("Assoc not available for type".to_owned())),
        },
        (Some(_), Some(_), None) => Err(Error::ArityException(
            3,
            format!("`assoc` has arity of 3 but received {}", 2),
        )),
    }
}

pub fn dissoc(info: &[T]) -> Result<T, Error> {
    if let Some(value) = arity_exception("dissoc", 2, info.len()) {
        return value;
    }
    match (info.get(0), info.get(1)) {
        (None, _) => Err(Error::Reason(String::from(
            "Collection is required for dissoc",
        ))),
        (_, None) => Err(Error::Reason(String::from(
            "Access index/key is required for dissoc",
        ))),
        (Some(collection), Some(key)) => match collection.clone().eval()? {
            T::HashMap(hm) => {
                let mut hm = hm.clone();
                if hm.contains_key(key) {
                    hm.remove(key);
                }
                Ok(T::HashMap(hm))
            }
            T::OrderedMap(om) => {
                let mut om = om.clone();
                if om.contains_key(key) {
                    om.remove(key);
                }
                Ok(T::OrderedMap(om))
            }
            _ => Err(Error::Reason("Dissoc not available for type".to_owned())),
        },
    }
}

pub fn contains(info: &[T]) -> Result<T, Error> {
    if let Some(value) = arity_exception("contains?", 2, info.len()) {
        return value;
    }

    match (info.get(0), info.get(1)) {
        (None, _) => Err(Error::Reason(String::from(
            "Collection is required for contains?",
        ))),
        (_, None) => Err(Error::Reason(String::from(
            "Access index/key is required for contains?",
        ))),
        (Some(collection), Some(index)) => match collection {
            T::String(s) => {
                let evaluated_index = match index {
                    T::String(s) => s.to_owned(),
                    T::Char(c) => String::from(c.to_owned()),
                    _ => index.print()?,
                };

                Ok(T::Bool(s.contains(&evaluated_index)))
            }
            T::HashSet(hs) => {
                if hs.contains(index) {
                    Ok(T::Bool(true))
                } else {
                    Ok(T::Bool(false))
                }
            }
            T::OrderedSet(os) => {
                if os.contains(index) {
                    Ok(T::Bool(true))
                } else {
                    Ok(T::Bool(false))
                }
            }
            T::HashMap(hm) => {
                if hm.contains_key(index) {
                    Ok(T::Bool(true))
                } else {
                    Ok(T::Bool(false))
                }
            }
            T::OrderedMap(om) => {
                if om.contains_key(index) {
                    Ok(T::Bool(true))
                } else {
                    Ok(T::Bool(false))
                }
            }
            T::List(_) => {
                let l = collection.to_owned().eval()?;
                contains(&[l, index.clone()])
            }
            T::Vector(v) => match index {
                T::Int(idx) => {
                    if let Some(idx) = idx.to_usize() {
                        if v.len() > idx {
                            Ok(T::Bool(true))
                        } else {
                            Ok(T::Bool(false))
                        }
                    } else {
                        Err(Error::IntParseError)
                    }
                }
                _ => Err(Error::Reason(String::from("Index must be an integer"))),
            },
            _ => Err(Error::Reason(String::from(
                "First argument must be a collection or a string for contains?",
            ))),
        },
    }
}
