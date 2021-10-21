use ClojuRS::{error::Error, read};

mod collections;
mod logic;
mod math;
mod std;

#[test]
fn inner_1() {
    assert_eq!(
        read("(= (/ 4.0 (+ 1 2 (- 4 3))) (* 1.0 (+ 2 2) 1/4))").unwrap(),
        "true"
    );
}

#[test]
fn thrown() {
    assert_eq!(
        read("(throw 1 2 3)").err(),
        Some(Error::ArityException(
            1,
            String::from("`throw` has arity of 1 but received 3")
        ))
    );
    assert_eq!(
        read("(throw 1)").err(),
        Some(Error::CantEval(Some(String::from("[Int(1)]"))))
    );

    assert_eq!(
        read("(throw \"this is an error message\")").err(),
        Some(Error::Thrown(String::from("this is an error message")))
    );
}
