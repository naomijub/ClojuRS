use num_bigint::{BigInt, ToBigInt};
use num_traits::Zero;

use crate::{definitions::DefinitionTypes, error::Error, Env};

pub fn is_numeric(
    list: &mut Vec<DefinitionTypes>,
    env: &mut Env,
) -> Result<DefinitionTypes, Error> {
    Ok(DefinitionTypes::Bool(list.iter().enumerate().all(
        |(i, e)| match e.clone() {
            DefinitionTypes::Symbol(symbol) if i == 0 => {
                env.func.contains_key(&symbol)
                    || if let Some(data) = env.data.get(&symbol) {
                        let data = data.clone();
                        if let Ok(DefinitionTypes::Bool(b)) = is_numeric(&mut vec![data], env) {
                            b
                        } else {
                            false
                        }
                    } else {
                        false
                    }
            }
            DefinitionTypes::List(mut l) => {
                if let Ok(DefinitionTypes::Bool(b)) = is_numeric(&mut l, env) {
                    b
                } else {
                    false
                }
            }
            DefinitionTypes::Double(_)
            | DefinitionTypes::Int(_)
            | DefinitionTypes::Rational(_, _) => true,
            _ => false,
        },
    )))
}

pub fn is_positive(
    list: &mut Vec<DefinitionTypes>,
    env: &mut Env,
) -> Result<DefinitionTypes, Error> {
    Ok(DefinitionTypes::Bool(list.iter().all(
        |e| match e.clone() {
            DefinitionTypes::Symbol(symbol) => {
                if let Some(data) = env.data.get(&symbol) {
                    let data = data.clone();
                    if let Ok(DefinitionTypes::Bool(b)) = is_positive(&mut vec![data], env) {
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
            _ => false,
        },
    )))
}

pub fn is_negative(
    list: &mut Vec<DefinitionTypes>,
    env: &mut Env,
) -> Result<DefinitionTypes, Error> {
    Ok(DefinitionTypes::Bool(list.iter().all(
        |e| match e.clone() {
            DefinitionTypes::Symbol(symbol) => {
                if let Some(data) = env.data.get(&symbol) {
                    let data = data.clone();
                    if let Ok(DefinitionTypes::Bool(b)) = is_negative(&mut vec![data], env) {
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
