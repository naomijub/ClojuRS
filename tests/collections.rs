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

#[test]
fn hashmap() {
    let hm = read("(hash-map 1 1 1.2 1.2 true true 3/4 3/4 \"hello\" \"hello\" \\c \\c :key :key [1 2 3] [1 2 3] (+ 3 1) (+ 3 1))").unwrap();
    assert!(
        hm.contains("{")
            && hm.contains("}")
            && hm.contains("1 1")
            && hm.contains("1.2 1.2")
            && hm.contains("true true")
            && hm.contains("3/4 3/4")
            && hm.contains("\"hello\" \"hello\"")
            && hm.contains("\\c \\c")
            && hm.contains(":key :key")
            && hm.contains("[1 2 3 ] [1 2 3 ]")
            && hm.contains("4 4")
    );
    assert_eq!(
        read("(hash-map 1 1 1.2 1.2 true)").err(),
        Some(Error::Reason("Hash map must be formed by pairs".to_owned()))
    );
    assert_eq!(
        read("(get (hash-map 1 2 1.2 1.3 true false) 1.2)").unwrap(),
        "1.3"
    );
    assert_eq!(
        read("(get (hash-map 1 2 1.2 1.3 true false) 1.3 :oh-no)").unwrap(),
        ":oh-no"
    );
    assert_eq!(
        read("(get (hash-map 1 1 1.2 1.2 true false) 5)").unwrap(),
        "nil"
    );
}

#[test]
fn ordered_map() {
    let sm = read("(sorted-map 1 1 1.2 1.2 true true 3/4 3/4 \"hello\" \"hello\" \\c \\c :key :key (+ 3 1) (+ 3 1))").unwrap();
    assert_eq!(
        sm,
        "{3/4 3/4 1 1 1.2 1.2 4 4 \\c \\c :key :key true true \"hello\" \"hello\" }"
    );
    assert_eq!(
        read("(sorted-map 1 1 1.2 1.2 true)").err(),
        Some(Error::Reason(
            "Sorted map must be formed by pairs".to_owned()
        ))
    );
    assert_eq!(
        read("(get (sorted-map 1 2 1.2 1.3 true false) 1.2)").unwrap(),
        "1.3"
    );
    assert_eq!(
        read("(get (sorted-map 1 2 1.2 1.3 true false) 1.3 :oh-no)").unwrap(),
        ":oh-no"
    );
    assert_eq!(
        read("(get (sorted-map 1 1 1.2 1.2 true false) 5)").unwrap(),
        "nil"
    );
    assert_eq!(
        read("(get (sorted-map 1 2 1.2 1.3 true false) 1.3 :oh-no :oh-no-no)").err(),
        Some(Error::ArityException(
            3,
            "`get` has arity of 3 but received 4".to_string()
        ))
    );
}

#[test]
fn assoc() {
    assert_eq!(
        read("(assoc)").err(),
        Some(Error::Reason("Collection is required for assoc".to_owned()))
    );
    assert_eq!(
        read("(assoc {})").err(),
        Some(Error::Reason(
            "Access index/key is required for assoc".to_owned()
        ))
    );
    assert_eq!(
        read("(assoc [] :key :value)").err(),
        Some(Error::Reason("Index must be of type int".to_owned()))
    );
    assert_eq!(
        read("(assoc {} :key)").err(),
        Some(Error::ArityException(
            3,
            "`assoc` has arity of 3 but received 2".to_owned()
        ))
    );
    assert_eq!(read("(assoc {} :key :value)").unwrap(), "{:key :value }");
    assert!(read("(assoc {:key :value :key2 :value2 } :key :new-value)")
        .unwrap()
        .contains(":key :new-value"));
    assert_eq!(
        read("(assoc (sorted-map :key :value :key2 :value2 ) :key :new-value)").unwrap(),
        "{:key :new-value :key2 :value2 }"
    );
    assert_eq!(read("(assoc [] 0 :value)").unwrap(), "[:value ]");
    assert_eq!(read("(assoc [:1 :2] 0 :value)").unwrap(), "[:value :2 ]");
    assert_eq!(read("(assoc [:1 :2] 2 :value)").unwrap(), "[:1 :2 :value ]");
    assert_eq!(
        read("(assoc (sorted-map  :key2 :value2 ) :key :new-value)").unwrap(),
        "{:key :new-value :key2 :value2 }"
    );
    assert_eq!(
        read("(assoc (sorted-map  :key2 :value2 ) :key :other-key :oh-no)").err(),
        Some(Error::ArityException(
            3,
            "`assoc` has arity of 3 but received 4".to_string()
        ))
    );
}

#[test]
fn dissoc() {
    assert_eq!(
        read("(dissoc)").err(),
        Some(Error::Reason(
            "Collection is required for dissoc".to_owned()
        ))
    );
    assert_eq!(
        read("(dissoc {})").err(),
        Some(Error::Reason(
            "Access index/key is required for dissoc".to_owned()
        ))
    );
    assert_eq!(
        read("(dissoc [] 0)").err(),
        Some(Error::Reason("Dissoc not available for type".to_owned()))
    );
    assert_eq!(read("(dissoc {} :key)").unwrap(), "{}");
    assert_eq!(read("(dissoc {:key :value} :key)").unwrap(), "{}");
    assert_eq!(
        read("(dissoc {:key :value :key2 :value2 } :key)").unwrap(),
        "{:key2 :value2 }"
    );
    assert_eq!(
        read("(dissoc (sorted-map :key :value :key2 :value2 ) :key)").unwrap(),
        "{:key2 :value2 }"
    );
    assert_eq!(
        read("(dissoc (sorted-map  :key2 :value2 ) :key :other-key)").err(),
        Some(Error::ArityException(
            2,
            "`dissoc` has arity of 2 but received 3".to_string()
        ))
    );
}
