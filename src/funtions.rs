use crate::{Env, definitions::{DefinitionTypes, Error}};

pub type Func = fn(&[DefinitionTypes]) -> Result<DefinitionTypes, Error>;

pub fn eval_list(list: &mut Vec<DefinitionTypes>, env: &mut Env) -> Result<String, Error> {
    let mut list = list.into_iter();
    let next = list.next();
    if let Some(DefinitionTypes::Symbol(symbol)) = next {
        let rest: Vec<DefinitionTypes> = list.map(|e| e.clone()).collect();
        env.func.get(symbol).ok_or_else(|| Error::UnknownSymbol)?(&rest)?.print(env)
    } else {
        let next = next.map(|e| e.print(env).unwrap_or(String::new()));
        Err(Error::CantEval(next))
    }
}

pub fn plus(list: &[DefinitionTypes]) -> Result<DefinitionTypes, Error> {
    Ok(DefinitionTypes::Int(42))
}

pub fn meaning_of_life(_: &[DefinitionTypes]) -> Result<DefinitionTypes, Error> {
    Ok(DefinitionTypes::Int(42))
}