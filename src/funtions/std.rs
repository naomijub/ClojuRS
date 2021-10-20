use crate::{definitions::DefinitionTypes as T, error::Error};

pub fn str(list: &[T]) -> Result<T, Error> {
    Ok(T::String(list.iter().fold(String::new(), |acc, el| {
        acc + &el.print().unwrap_or_default()
    })))
}

pub fn println(list: &[T]) -> Result<T, Error> {
    if let Ok(T::String(str)) = str(list) {
        println!("{}", str);
    }
    Ok(T::Nil)
}

pub fn to_keyword(list: &[T]) -> Result<T, Error> {
    let keywords = list
        .iter()
        .map(|el| match el {
            T::Symbol(s) => Ok(T::Keyword(format!(":{}", s))),
            T::Keyword(_) => Ok(el.to_owned()),
            T::String(s) => Ok(T::Keyword(format!(":{}", s))),
            T::Char(s) => Ok(T::Keyword(format!(":{}", s))),
            T::Bool(s) => Ok(T::Keyword(format!(":{}", s))),
            T::Double(s) => Ok(T::Keyword(format!(":{}", s))),
            T::Int(s) => Ok(T::Keyword(format!(":{}", s))),
            T::Rational(n, d) => Ok(T::Keyword(format!(":{}/{}", n, d))),
            T::Nil => Ok(T::Keyword(":nil".to_owned())),
            T::List(_) => {
                if let Ok(T::Vector(item)) = to_keyword(&[el.clone().eval()?]) {
                    if item.len() == 1 {
                        Ok(item[0].clone())
                    } else {
                        Err(Error::Thrown(String::from("Can't keywordize a collection")))
                    }
                } else {
                    Err(Error::CantEval(el.print().ok()))
                }
            }
            _ => Err(Error::Thrown(String::from("Can't keywordize a collection"))),
        })
        .collect::<Result<Vec<T>, Error>>();
    Ok(T::Vector(keywords?))
}
