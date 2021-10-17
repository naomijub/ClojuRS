use ClojuRS::read;

mod logic;
mod math;

#[test]
fn inner_1() {
    assert_eq!(
        read("(= (/ 4.0 (+ 1 2 (- 4 3))) (* 1.0 (+ 2 2) 1/4))").unwrap(),
        "true"
    );
}
