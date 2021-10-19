use ClojuRS::{error::Error, read};

#[test]
fn str() {
    assert_eq!(
        read("(str :hello \"word\" 666 3/4 123 (+ 2 -1) \\4 true)").unwrap(),
        "\":hello\"word\"6663/41231\\4true\""
    );
}

#[test]
fn keyword() {
    assert_eq!(
        read("(keyword :hello \"word\" 666 3/4 123 (+ 2 -1) \\4 nil true)").unwrap(),
        "[:hello :word :666 :3/4 :123 :1 :4 :nil :true ]"
    );
    assert_eq!(
        read("(keyword [:hello \"word\" ])").err(),
        Some(Error::Thrown("Can't keywordize a collection".to_string()))
    );
}
