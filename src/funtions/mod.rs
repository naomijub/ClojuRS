use num_bigint::ToBigInt;

use crate::{definitions::DefinitionTypes, error::Error, STD};
pub mod math;

pub type Func = fn(&[DefinitionTypes]) -> Result<DefinitionTypes, Error>;

pub fn eval_list(list: &mut Vec<DefinitionTypes>) -> Result<String, Error> {
    if list.is_empty() {
        return Ok(String::from("()"));
    }

    let mut list = list.iter_mut();
    let next = list.next();
    if let Some(DefinitionTypes::Symbol(symbol)) = next {
        let rest: Vec<DefinitionTypes> = list.map(|e| e.clone()).collect();
        STD.get(symbol)
            .ok_or_else(|| Error::UnknownSymbol(symbol.to_string()))?(&rest)?
        .print()
    } else {
        let next = next.map(|e| e.print().unwrap_or_default());
        Err(Error::CantEval(next))
    }
}

pub fn meaning_of_life(_: &[DefinitionTypes]) -> Result<DefinitionTypes, Error> {
    Ok(DefinitionTypes::Int(
        42.to_bigint().ok_or(Error::IntParseError)?,
    ))
}
