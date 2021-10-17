use crate::DATA;

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

pub fn is_false(list: &[T]) -> Result<T, Error> {
    Ok(T::Bool(list.iter().all(|e| match e {
        T::Symbol(key) => DATA.lock().map_or(false, |m| match m.get(key) {
            Some(T::Bool(b)) => !*b,
            Some(T::Nil) => true,
            _ => false,
        }),
        T::Bool(b) => !*b,
        T::Nil => true,
        //TODO list
        _ => false,
    })))
}

pub fn is_true(list: &[T]) -> Result<T, Error> {
    Ok(T::Bool(list.iter().all(|e| match e {
        T::Symbol(key) => DATA.lock().map_or(false, |m| match m.get(key) {
            Some(T::Bool(b)) => *b,
            Some(T::Nil) => false,
            _ => true,
        }),
        T::Bool(b) => *b,
        T::Nil => false,
        //TODO list
        _ => true,
    })))
}

// fn every? and fn some? Issue 16
