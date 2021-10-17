use num_bigint::{BigInt, ToBigInt};
use num_traits::{One, Zero};

use crate::{definitions::DefinitionTypes as T, error::Error, DATA};

use super::eval_list;

pub fn is_numeric(list: &[T]) -> Result<T, Error> {
    Ok(T::Bool(list.iter().enumerate().all(
        |(i, e)| match e.clone() {
            T::Symbol(symbol) if i == 0 => {
                if let Some(data) = DATA.lock().unwrap().get(&symbol) {
                    let data = data.clone();
                    if let Ok(T::Bool(b)) = is_numeric(&[data]) {
                        b
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            T::List(mut l) => {
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
            T::Double(_) | T::Int(_) | T::Rational(_, _) => true,
            _ => false,
        },
    )))
}

pub fn is_positive(list: &[T]) -> Result<T, Error> {
    Ok(T::Bool(list.iter().all(|e| match e.clone() {
        T::Symbol(symbol) => {
            if let Some(data) = DATA.lock().unwrap().get(&symbol) {
                let data = data.clone();
                if let Ok(T::Bool(b)) = is_positive(&[data]) {
                    b
                } else {
                    false
                }
            } else {
                false
            }
        }
        T::Double(num) if num.0 > 0.0f64 => true,
        T::Int(num) if num > 0.to_bigint().unwrap() => true,
        T::Rational(num, _) if num > BigInt::zero() => true,
        T::List(mut l) => {
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
    })))
}

pub fn is_negative(list: &[T]) -> Result<T, Error> {
    println!("{:?}", list);
    Ok(T::Bool(list.iter().all(|e| match e.clone() {
        T::Symbol(symbol) => {
            if let Some(data) = DATA.lock().unwrap().get(&symbol) {
                let data = data.clone();
                if let Ok(T::Bool(b)) = is_negative(&[data]) {
                    b
                } else {
                    false
                }
            } else {
                false
            }
        }
        T::Double(num) if num.0 < 0.0f64 => true,
        T::Int(num) if num < 0.to_bigint().unwrap() => true,
        T::Rational(num, _) if num < BigInt::zero() => true,
        T::List(mut l) => {
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
    })))
}

pub fn plus(list: &[T]) -> Result<T, Error> {
    list.iter()
        .try_fold(T::Int(BigInt::zero()), |acc, e| acc + e.clone())
}

pub fn sub(list: &[T]) -> Result<T, Error> {
    if let Some((first, rest)) = list.split_first() {
        rest.iter()
            .try_fold(first.to_owned(), |acc, e| acc - e.to_owned())
    } else {
        Err(Error::Reason(String::from("Couldn't parse form content")))
    }
}

pub fn mul(list: &[T]) -> Result<T, Error> {
    list.iter()
        .try_fold(T::Int(BigInt::one()), |acc, e| acc * e.clone())
}

pub fn div(list: &[T]) -> Result<T, Error> {
    if let Some((first, rest)) = list.split_first() {
        rest.iter()
            .try_fold(first.to_owned(), |acc, e| acc / e.to_owned())
    } else {
        Err(Error::Reason(String::from("Couldn't parse form content")))
    }
}
