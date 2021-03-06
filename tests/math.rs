use ClojuRS::read;

#[test]
fn what_is_the_meaning_of_life() {
    assert_eq!(read("(meaning-of-life?)").unwrap(), "42");
}

#[test]
fn is_positive_number() {
    assert_eq!(read("(pos? 42)").unwrap(), "true");
    assert_eq!(read("(pos? 42.0)").unwrap(), "true");
    assert_eq!(read("(pos? 42/4)").unwrap(), "true");
    assert_eq!(read("(pos? -42)").unwrap(), "false");
    assert_eq!(read("(pos? -42.0)").unwrap(), "false");
    assert_eq!(read("(pos? -42/4)").unwrap(), "false");
    assert_eq!(read("(pos? 0)").unwrap(), "false");
    assert_eq!(read("(pos? (- 1/3 4/5))").unwrap(), "false");
    assert_eq!(read("(pos? (+ 1/3 4/5))").unwrap(), "true");
}

#[test]
fn is_negative_number() {
    assert_eq!(read("(neg? -42)").unwrap(), "true");
    assert_eq!(read("(neg? -42.0)").unwrap(), "true");
    assert_eq!(read("(neg? -42/4)").unwrap(), "true");
    assert_eq!(read("(neg? 42)").unwrap(), "false");
    assert_eq!(read("(neg? 42.0)").unwrap(), "false");
    assert_eq!(read("(neg? 42/4)").unwrap(), "false");
    assert_eq!(read("(neg? 0)").unwrap(), "false");
    assert_eq!(read("(neg? (- 1/3 4/5))").unwrap(), "true");
    assert_eq!(read("(neg? (+ 1/3 4/5))").unwrap(), "false");
}

#[test]
fn is_numeric_number() {
    assert_eq!(read("(num? -42)").unwrap(), "true");
    assert_eq!(read("(num? -42.0)").unwrap(), "true");
    assert_eq!(read("(num? -42/4)").unwrap(), "true");
    assert_eq!(read("(num? 42)").unwrap(), "true");
    assert_eq!(read("(num? 42.0)").unwrap(), "true");
    assert_eq!(read("(num? 42/4)").unwrap(), "true");
    assert_eq!(read("(num? 0)").unwrap(), "true");
    assert_eq!(read("(num? \"hello\")").unwrap(), "false");
    assert_eq!(read("(num? 'h')").unwrap(), "false");
    assert_eq!(read("(num? true)").unwrap(), "false");
    assert_eq!(read("(num? false)").unwrap(), "false");
    assert_eq!(read("(num? [1 2 3])").unwrap(), "false");
    assert_eq!(read("(num? #{{1 2 3}})").unwrap(), "false");
    assert_eq!(read("(num? {{1 2 :5 3}})").unwrap(), "false");
    assert_eq!(read("(num? :5)").unwrap(), "false");
}

#[test]
fn add_values() {
    assert_eq!(read("(+ 1 2 3)").unwrap(), "6");
    assert_eq!(read("(+ 1 2.3)").unwrap(), "3.3");
    assert_eq!(read("(+ 1 2/3)").unwrap(), "5/3");
    assert_eq!(read("(+ 1 4.5 1/2)").unwrap(), "6");
    assert_eq!(read("(+ 1 4.1 1/2)").unwrap(), "5.6");
}

#[test]
fn sub_values() {
    assert_eq!(read("(- 1 2 3)").unwrap(), "-4");
    assert!(read("(- 1 2.3)").unwrap().contains("-1.299"));
    assert_eq!(read("(- 1 2/3)").unwrap(), "1/3");
    assert_eq!(read("(- 1 4.5 1/2)").unwrap(), "-4");
    assert!(read("(- 1 4.1 1/2)").unwrap().contains("-3.5999"));
}

#[test]
fn mul_values() {
    assert_eq!(read("(* 4 2 3)").unwrap(), "24");
    assert_eq!(read("(* 4 -2 3)").unwrap(), "-24");
    assert_eq!(read("(* 2 2.3)").unwrap(), "4.6");
    assert_eq!(read("(* 2 -2.3)").unwrap(), "-4.6");
    assert_eq!(read("(* 2 2/3)").unwrap(), "4/3");
    assert_eq!(read("(* 3/4 2/3)").unwrap(), "6/12");
    assert_eq!(read("(* 1 4.5 1/2)").unwrap(), "2.25");
    assert_eq!(read("(* 1 4.1 -1/2)").unwrap(), "-2.05");
}

#[test]
fn div_values() {
    assert_eq!(read("(/ 4 2 3)").unwrap(), "2/3");
    assert_eq!(read("(/ 4 2 1)").unwrap(), "2");
    assert_eq!(read("(/ 4 -2 3)").unwrap(), "-2/3");
    assert!(read("(/ 2 2.3)").unwrap().contains("0.8695652"));
    assert!(read("(/ 2 -2.3)").unwrap().contains("-0.8695652"));
    assert_eq!(read("(/ 2 2/3)").unwrap(), "6/2");
    assert_eq!(read("(/ 3/4 2/3)").unwrap(), "9/8");
    assert!(read("(/ 1 4.5 1/2)").unwrap().contains("0.4444"));
    assert!(read("(/ 1 4.1 -1/2)").unwrap().contains("-0.487804"));
}
