use ClojuRS::{error::Error, read};

#[test]
fn vector() {
    assert_eq!(
        read("(vector 1 2.3 true 2/3 (+ 1 1))").unwrap(),
        "[1 2.3 true 2/3 2 ]"
    );
    assert_eq!(read("(get (vector 1 2.3 true (+ 1 1)) 2)").unwrap(), "true");
    assert_eq!(read("(get (vector 1 2.3 true) 2 :oh-no)").unwrap(), "true");
    assert_eq!(
        read("(get (vector 1 2.3 true) 3 :oh-no)").unwrap(),
        ":oh-no"
    );
    assert_eq!(
        read("(get (vector 1 2.3 true) 3)").err(),
        Some(Error::Reason("Index out of bounds".to_owned()))
    );
}
