use crate::{definitions::DefinitionTypes as T, error::Error};

pub fn eq(list: &[T]) -> Result<T, Error> {
    Ok(T::Bool(list.windows(2).all(|e| {
        e[0].clone().eval().unwrap_or(T::Nil) == e[1].clone().eval().unwrap_or(T::Bool(true))
    })))
}

pub fn ge(list: &[T]) -> Result<T, Error> {
    Ok(T::Bool(list.windows(2).all(|e| {
        e[0].clone().eval().unwrap_or(T::Nil) >= e[1].clone().eval().unwrap_or(T::Bool(true))
    })))
}

pub fn greater(list: &[T]) -> Result<T, Error> {
    Ok(T::Bool(list.windows(2).all(|e| {
        e[0].clone().eval().unwrap_or(T::Nil) > e[1].clone().eval().unwrap_or(T::Bool(true))
    })))
}

pub fn le(list: &[T]) -> Result<T, Error> {
    Ok(T::Bool(list.windows(2).all(|e| {
        e[0].clone().eval().unwrap_or(T::Nil) <= e[1].clone().eval().unwrap_or(T::Bool(true))
    })))
}

pub fn lesser(list: &[T]) -> Result<T, Error> {
    Ok(T::Bool(list.windows(2).all(|e| {
        e[0].clone().eval().unwrap_or(T::Nil) < e[1].clone().eval().unwrap_or(T::Bool(true))
    })))
}
