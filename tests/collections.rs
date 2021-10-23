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

#[test]
fn hashset() {
    let hs = read("(hash-set 1 2.3 true 2/3 (+ 1 1))").unwrap();
    assert!(
        hs.contains("#{")
            && hs.contains("}")
            && hs.contains("1")
            && hs.contains("2/3")
            && hs.contains("2.3")
            && hs.contains("2")
            && hs.contains("true")
    );
    assert_eq!(read("(get (hash-set 1 2.3 true (+ 1 1)) 2)").unwrap(), "2");
    assert_eq!(
        read("(get (hash-set 1 2.3 true) true :oh-no)").unwrap(),
        "true"
    );
    assert_eq!(
        read("(get (hash-set 1 2.3 true) 3 :oh-no)").unwrap(),
        ":oh-no"
    );
    assert_eq!(read("(get (hash-set 1 2.3 true) 3)").unwrap(), "nil");
}

#[test]
fn sorted_set() {
    let hs = read("(sorted-set 1 2.3 true 2/3 :false \\f false \"true\" nil (+ 1 1))").unwrap();
    assert_eq!(hs, "#{2/3 1 2 2.3 \\f :false false true \"true\" nil }");
    assert_eq!(
        read("(get (sorted-set 1 2.3 true (+ 1 1)) 2)").unwrap(),
        "2"
    );
    assert_eq!(
        read("(get (sorted-set 1 2.3 true) true :oh-no)").unwrap(),
        "true"
    );
    assert_eq!(
        read("(get (sorted-set 1 2.3 true) 3 :oh-no)").unwrap(),
        ":oh-no"
    );
    assert_eq!(read("(get (sorted-set 1 2.3 true) 3)").unwrap(), "nil");
}
