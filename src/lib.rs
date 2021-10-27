#![allow(non_snake_case)]
#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

use definitions::DefinitionTypes as T;
use error::Error;
use funtions::Func;
use helper::MaybeReplaceExt;
use parser::{parse, tokenize};

use im::{hashmap, HashMap as Hamt};

use crate::funtions::{
    collections::{
        assoc, contains, dissoc, get, to_hashmap, to_hashset, to_orderedmap, to_orderedset,
        to_vector,
    },
    logic::{eq, ge, greater, is_false, is_nil, is_true, le, lesser},
    math::{div, is_negative, is_numeric, is_positive, mul, plus, sub},
    meaning_of_life,
    std::{println, str, to_keyword},
    throw,
};

pub(crate) mod definitions;
pub mod error;
pub mod funtions;
pub(crate) mod helper;
pub(crate) mod parser;

lazy_static! {
    pub static ref STD: Hamt<String, Func> = hashmap! {
        String::from("+") => plus as Func,
        String::from("-") => sub as Func,
        String::from("*") => mul as Func,
        String::from("/") => div as Func,
        String::from("meaning-of-life?") => meaning_of_life as Func,
        String::from("neg?") => is_negative as Func,
        String::from("pos?") => is_positive as Func,
        String::from("num?") => is_numeric as Func,
        String::from("=") => eq as Func,
        String::from(">=") => ge as Func,
        String::from(">") => greater as Func,
        String::from("<=") => le as Func,
        String::from("<") => lesser as Func,
        String::from("true?") => is_true as Func,
        String::from("false?") => is_false as Func,
        String::from("nil?") => is_nil as Func,
        String::from("throw") => throw as Func,
        String::from("str") => str as Func,
        String::from("println!") => println as Func,
        String::from("keyword") => to_keyword as Func,
        String::from("vector") => to_vector as Func,
        String::from("get") => get as Func,
        String::from("sorted-set") => to_orderedset as Func,
        String::from("hash-set") => to_hashset as Func,
        String::from("hash-map") => to_hashmap as Func,
        String::from("sorted-map") => to_orderedmap as Func,
        String::from("assoc") => assoc as Func,
        String::from("dissoc") => dissoc as Func,
        String::from("contains?") => contains as Func,
    };
    pub static ref LOCAL: Mutex<Hamt<String, Func>> = Mutex::new(Hamt::new());
    pub static ref DATA: Mutex<Hamt<String, T>> = Mutex::new(Hamt::new());
}

pub fn read(list: &str) -> Result<String, Error> {
    let clean = String::from(list.maybe_replace("#{", "@").trim_start());
    let mut tokens = tokenize(&clean);
    let parsed = parse(tokens.next(), &mut tokens)?;
    parsed.print()
}
