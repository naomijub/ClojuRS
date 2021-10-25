use std::collections::{BTreeMap, HashMap};

use num_traits::ToPrimitive;

use crate::{definitions::DefinitionTypes as T, error::Error, DATA};

pub fn get(info: &[T]) -> Result<T, Error> {
    match (info.get(0), info.get(1), info.get(2)) {
        (None, _, _) => Err(Error::Reason(String::from(
            "Collection is required for get",
        ))),
        (_, None, _) => Err(Error::Reason(String::from(
            "Access index is required for get",
        ))),
        (Some(collection), Some(index), not_found) => match collection {
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
