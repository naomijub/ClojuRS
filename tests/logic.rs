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

#[test]
fn is_true() {
    assert_eq!(read("(true? true)").unwrap(), "true");
    assert_eq!(read("(true? 1 :hello \"true\" true)").unwrap(), "true");
    assert_eq!(read("(true? (= true true) (= 1 1))").unwrap(), "true");

    assert_eq!(read("(true? nil)").unwrap(), "false");
    assert_eq!(read("(true? false)").unwrap(), "false");
    assert_eq!(read("(true? 1 :hello \"true\" nil)").unwrap(), "false");
    assert_eq!(read("(true? (= false true) (= 1 1))").unwrap(), "false");
}

#[test]
fn is_false() {
    assert_eq!(read("(false? nil)").unwrap(), "true");
    assert_eq!(read("(false? false)").unwrap(), "true");
    assert_eq!(read("(false? 1 :hello \"true\" nil)").unwrap(), "false");
    assert_eq!(read("(false? (= false true) (= 1 1))").unwrap(), "false");

    assert_eq!(read("(false? true)").unwrap(), "false");
    assert_eq!(read("(false? 1 :hello \"true\" true)").unwrap(), "false");
    assert_eq!(read("(false? (= true true) (= 1 1))").unwrap(), "false");
}
