use definitions::DefinitionTypes;
use error::Error;
use funtions::Func;
use helper::MaybeReplaceExt;
use parser::{parse, tokenize};

use im::{hashmap, HashMap as Hamt};

use crate::funtions::{math::{sub, plus}, meaning_of_life};

pub(crate) mod definitions;
pub mod error;
pub mod funtions;
pub(crate) mod helper;
pub(crate) mod parser;

fn main() {
    let env = &mut Env::new();
    loop {
        println!("Crs > ");
        let expr = slurp_expr();
        match read(&expr, env) {
            Ok(resp) => println!("{}", resp),
            Err(err) => println!("{:?}", err),
        }
    }
}

fn slurp_expr() -> String {
    let mut expr = String::new();

    std::io::stdin()
        .read_line(&mut expr)
        .expect("Failed to read line");

    expr
}

fn read(list: &str, env: &mut Env) -> Result<String, Error> {
    let clean = String::from(list.maybe_replace("#{", "@").trim_start());
    let mut tokens = tokenize(&clean);
    let parsed = parse(tokens.next(), &mut tokens)?;
    parsed.print(env)
}

pub struct Env {
    data: Hamt<String, DefinitionTypes>,
    func: Hamt<String, Func>,
}

impl Env {
    fn new() -> Self {
        let funcs: Hamt<String, Func> = hashmap! {
            String::from("+") => plus as Func,
            String::from("-") => sub as Func,
            String::from("meaning-of-life") => meaning_of_life as Func,
        };
        Self {
            data: Hamt::new(),
            func: funcs,
        }
    }
}
