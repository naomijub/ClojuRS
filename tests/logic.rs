use ClojuRS::read;

#[test]
fn equals() {
    assert_eq!(read("(= 1 (+ 2 -1) (- 2 1))").unwrap(), "true");
    assert_eq!(read("(= 1 (+ 2 -1) (- 2 1) 2)").unwrap(), "false");
}
