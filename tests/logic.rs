use ClojuRS::read;

#[test]
fn equals() {
    assert_eq!(read("(= 1 (+ 2 -1) (- 2 1))").unwrap(), "true");
    assert_eq!(read("(= 1 (+ 2 -1) (- 2 1) 2)").unwrap(), "false");
}

#[test]
fn greater_eq() {
    assert_eq!(read("(>= 4 3 2 1)").unwrap(), "true");
    assert_eq!(read("(>= 4 3 3 2 1 1)").unwrap(), "true");
    assert_eq!(read("(>= 3 4 3  2 1 1)").unwrap(), "false");

    assert_eq!(read("(> 4 3 2 1)").unwrap(), "true");
    assert_eq!(read("(> 4 3 3 2 1 1)").unwrap(), "false");
    assert_eq!(read("(> 3 4 3  2 1 1)").unwrap(), "false");
}

#[test]
fn lesser_eq() {
    assert_eq!(read("(<= 1 2 3 4)").unwrap(), "true");
    assert_eq!(read("(<= 1 2 2 3  4 4)").unwrap(), "true");
    assert_eq!(read("(<= 2 1 2 3)").unwrap(), "false");

    assert_eq!(read("(< 1 2 3 4)").unwrap(), "true");
    assert_eq!(read("(< 1 2 2 3  4 4)").unwrap(), "false");
    assert_eq!(read("(< 2 1 2 3)").unwrap(), "false");
}
