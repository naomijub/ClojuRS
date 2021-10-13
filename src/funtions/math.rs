use num_bigint::{BigInt, ToBigInt};
use num_traits::Zero;

use crate::{definitions::DefinitionTypes, error::Error, DATA};

use super::eval_list;

pub fn is_numeric(list: &[DefinitionTypes]) -> Result<DefinitionTypes, Error> {
    Ok(DefinitionTypes::Bool(list.iter().enumerate().all(
        |(i, e)| match e.clone() {
            DefinitionTypes::Symbol(symbol) if i == 0 => {
                if let Some(data) = DATA.lock().unwrap().get(&symbol) {
                    let data = data.clone();
                    if let Ok(DefinitionTypes::Bool(b)) = is_numeric(&[data]) {
                        b
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            DefinitionTypes::List(mut l) => {
                let eval = eval_list(&mut l);
                eval.clone().map(|op| op.parse::<BigInt>()).is_ok()
                    || eval.clone().map(|op| op.parse::<f64>()).is_ok()
                    || eval
                        .map(|op| {
                            op.split('/').all(|num| {
                                num.parse::<BigInt>().is_ok() || num.parse::<f64>().is_ok()
                            })
                        })
                        .unwrap_or(false)
            }
            DefinitionTypes::Double(_)
            | DefinitionTypes::Int(_)
            | DefinitionTypes::Rational(_, _) => true,
            _ => false,
        },
    )))
}

pub fn is_positive(list: &[DefinitionTypes]) -> Result<DefinitionTypes, Error> {
    Ok(DefinitionTypes::Bool(list.iter().all(
        |e| match e.clone() {
            DefinitionTypes::Symbol(symbol) => {
                if let Some(data) = DATA.lock().unwrap().get(&symbol) {
                    let data = data.clone();
                    if let Ok(DefinitionTypes::Bool(b)) = is_positive(&[data]) {
                        b
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            DefinitionTypes::Double(num) if num.0 > 0.0f64 => true,
            DefinitionTypes::Int(num) if num > 0.to_bigint().unwrap() => true,
            DefinitionTypes::Rational(num, _) if num > BigInt::zero() => true,
            DefinitionTypes::List(mut l) => {
                let eval = eval_list(&mut l).unwrap_or_default();

                if eval.contains('/')
                    && eval
                        .split('/')
                        .all(|num| num.parse::<BigInt>().is_ok() || num.parse::<f64>().is_ok())
                {
                    let split = eval.split('/');
                    split
                        .clone()
                        .filter_map(|num| num.parse::<BigInt>().ok())
                        .all(|num| num > BigInt::zero())
                        || split
                            .filter_map(|num| num.parse::<f64>().ok())
                            .all(|num| num > 0.0)
                } else if eval.parse::<BigInt>().is_ok() {
                    eval.parse::<BigInt>().unwrap() > BigInt::zero()
                } else if eval.parse::<f64>().is_ok() {
                    eval.parse::<f64>().unwrap() > 0.0
                } else {
                    false
                }
            }
            _ => false,
        },
    )))
}

pub fn is_negative(list: &[DefinitionTypes]) -> Result<DefinitionTypes, Error> {
    println!("{:?}", list);
    Ok(DefinitionTypes::Bool(list.iter().all(
        |e| match e.clone() {
            DefinitionTypes::Symbol(symbol) => {
                if let Some(data) = DATA.lock().unwrap().get(&symbol) {
                    let data = data.clone();
                    if let Ok(DefinitionTypes::Bool(b)) = is_negative(&[data]) {
                        b
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            DefinitionTypes::Double(num) if num.0 < 0.0f64 => true,
            DefinitionTypes::Int(num) if num < 0.to_bigint().unwrap() => true,
            DefinitionTypes::Rational(num, _) if num < BigInt::zero() => true,
            DefinitionTypes::List(mut l) => {
                let eval = eval_list(&mut l).unwrap_or_default();

                if eval.contains('/')
                    && eval
                        .split('/')
                        .all(|num| num.parse::<BigInt>().is_ok() || num.parse::<f64>().is_ok())
                {
                    let split = eval.split('/');
                    split
                        .clone()
                        .filter_map(|num| num.parse::<BigInt>().ok())
                        .any(|num| num < BigInt::zero())
                        || split
                            .filter_map(|num| num.parse::<f64>().ok())
                            .any(|num| num < 0.0)
                } else if eval.parse::<BigInt>().is_ok() {
                    eval.parse::<BigInt>().unwrap() < BigInt::zero()
                } else if eval.parse::<f64>().is_ok() {
                    eval.parse::<f64>().unwrap() < 0.0
                } else {
                    false
                }
            }
            _ => false,
        },
    )))
}

pub fn plus(list: &[DefinitionTypes]) -> Result<DefinitionTypes, Error> {
    list.iter()
        .try_fold(DefinitionTypes::Int(BigInt::zero()), |acc, e| {
            acc + e.clone()
        })
}

pub fn sub(list: &[DefinitionTypes]) -> Result<DefinitionTypes, Error> {
    if let Some((first, rest)) = list.split_first() {
        rest.iter()
            .try_fold(first.to_owned(), |acc, e| acc - e.to_owned())
    } else {
        Err(Error::Reason(String::from("Couldn't parse form content")))
    }
}
