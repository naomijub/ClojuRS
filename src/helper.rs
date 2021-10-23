use std::{borrow::Cow, cmp::Ordering};

use num_traits::{FromPrimitive, ToPrimitive};
use ordered_float::OrderedFloat;

use crate::definitions::DefinitionTypes;
use crate::DATA;

pub trait MaybeReplaceExt<'a> {
    fn maybe_replace(self, find: &str, replacement: &str) -> Cow<'a, str>;
}

impl<'a> MaybeReplaceExt<'a> for &'a str {
    fn maybe_replace(self, find: &str, replacement: &str) -> Cow<'a, str> {
        if self.contains(find) {
            self.replace(find, replacement).into()
        } else {
            self.into()
        }
    }
}

impl<'a> MaybeReplaceExt<'a> for Cow<'a, str> {
    fn maybe_replace(self, find: &str, replacement: &str) -> Cow<'a, str> {
        if self.contains(find) {
            self.replace(find, replacement).into()
        } else {
            self
        }
    }
}

pub(crate) fn cmp(s: &DefinitionTypes, other: &DefinitionTypes) -> Ordering {
    partial_cmp(s, other).expect("Can't order seq/collections")
}

pub(crate) fn partial_cmp(s: &DefinitionTypes, other: &DefinitionTypes) -> Option<Ordering> {
    match (s, other) {
        (DefinitionTypes::Keyword(k1), DefinitionTypes::Keyword(k2)) => Some(k1.cmp(k2)),
        (DefinitionTypes::Keyword(_), DefinitionTypes::String(_)) => Some(Ordering::Less),
        (DefinitionTypes::Keyword(_), DefinitionTypes::Char(_)) => Some(Ordering::Greater),
        (DefinitionTypes::Keyword(_), DefinitionTypes::Bool(_)) => Some(Ordering::Less),
        (DefinitionTypes::Keyword(_), DefinitionTypes::Double(_)) => Some(Ordering::Greater),
        (DefinitionTypes::Keyword(_), DefinitionTypes::Int(_)) => Some(Ordering::Greater),
        (DefinitionTypes::Keyword(_), DefinitionTypes::Rational(_, _)) => Some(Ordering::Greater),
        (DefinitionTypes::String(_), DefinitionTypes::Keyword(_)) => Some(Ordering::Greater),
        (DefinitionTypes::String(s1), DefinitionTypes::String(s2)) => Some(s1.cmp(s2)),
        (DefinitionTypes::String(_), DefinitionTypes::Char(_)) => Some(Ordering::Greater),
        (DefinitionTypes::String(_), DefinitionTypes::Bool(_)) => Some(Ordering::Greater),
        (DefinitionTypes::String(_), DefinitionTypes::Double(_)) => Some(Ordering::Greater),
        (DefinitionTypes::String(_), DefinitionTypes::Int(_)) => Some(Ordering::Greater),
        (DefinitionTypes::String(_), DefinitionTypes::Rational(_, _)) => Some(Ordering::Greater),
        (DefinitionTypes::Char(_), DefinitionTypes::Keyword(_)) => Some(Ordering::Less),
        (DefinitionTypes::Char(_), DefinitionTypes::String(_)) => Some(Ordering::Less),
        (DefinitionTypes::Char(c1), DefinitionTypes::Char(c2)) => Some(c1.cmp(c2)),
        (DefinitionTypes::Char(_), DefinitionTypes::Bool(_)) => Some(Ordering::Less),
        (DefinitionTypes::Char(_), DefinitionTypes::Double(_)) => Some(Ordering::Greater),
        (DefinitionTypes::Char(_), DefinitionTypes::Int(_)) => Some(Ordering::Greater),
        (DefinitionTypes::Char(_), DefinitionTypes::Rational(_, _)) => Some(Ordering::Greater),
        (DefinitionTypes::Bool(_), DefinitionTypes::Keyword(_)) => Some(Ordering::Greater),
        (DefinitionTypes::Bool(_), DefinitionTypes::String(_)) => Some(Ordering::Less),
        (DefinitionTypes::Bool(_), DefinitionTypes::Char(_)) => Some(Ordering::Greater),
        (DefinitionTypes::Bool(b1), DefinitionTypes::Bool(b2)) => Some(b1.cmp(b2)),
        (DefinitionTypes::Bool(_), DefinitionTypes::Double(_)) => Some(Ordering::Greater),
        (DefinitionTypes::Bool(_), DefinitionTypes::Int(_)) => Some(Ordering::Greater),
        (DefinitionTypes::Bool(_), DefinitionTypes::Rational(_, _)) => Some(Ordering::Greater),
        (DefinitionTypes::Double(_), DefinitionTypes::Keyword(_)) => Some(Ordering::Less),
        (DefinitionTypes::Double(_), DefinitionTypes::String(_)) => Some(Ordering::Less),
        (DefinitionTypes::Double(_), DefinitionTypes::Char(_)) => Some(Ordering::Less),
        (DefinitionTypes::Double(_), DefinitionTypes::Bool(_)) => Some(Ordering::Less),
        (DefinitionTypes::Double(n1), DefinitionTypes::Double(n2)) => Some(n1.cmp(n2)),
        (DefinitionTypes::Double(n1), DefinitionTypes::Int(n2)) => {
            let n2_f64 = n2.to_f64().map(OrderedFloat::from_f64)?;
            n2_f64.map(|e| n1.cmp(&e))
        }
        (DefinitionTypes::Double(n1), DefinitionTypes::Rational(num, den)) => {
            let q = num.to_f64()? / den.to_f64()?;
            let ord_q = OrderedFloat::from_f64(q);
            ord_q.map(|e| n1.cmp(&e))
        }
        (DefinitionTypes::Int(_), DefinitionTypes::Keyword(_)) => Some(Ordering::Less),
        (DefinitionTypes::Int(_), DefinitionTypes::String(_)) => Some(Ordering::Less),
        (DefinitionTypes::Int(_), DefinitionTypes::Char(_)) => Some(Ordering::Less),
        (DefinitionTypes::Int(_), DefinitionTypes::Bool(_)) => Some(Ordering::Less),
        (DefinitionTypes::Int(n2), DefinitionTypes::Double(n1)) => {
            Some(OrderedFloat::<f64>::from_f64(n2.to_f64()?)?.cmp(n1))
        }
        (DefinitionTypes::Int(n1), DefinitionTypes::Int(n2)) => Some(n1.cmp(n2)),
        (DefinitionTypes::Int(n1), DefinitionTypes::Rational(num, den)) => {
            let q = num.to_f64()? / den.to_f64()?;
            let ord_q = OrderedFloat::from_f64(q)?;
            Some(OrderedFloat::<f64>::from_f64(n1.to_f64()?)?.cmp(&ord_q))
        }
        (DefinitionTypes::Rational(_, _), DefinitionTypes::Keyword(_)) => Some(Ordering::Less),
        (DefinitionTypes::Rational(_, _), DefinitionTypes::String(_)) => Some(Ordering::Less),
        (DefinitionTypes::Rational(_, _), DefinitionTypes::Char(_)) => Some(Ordering::Less),
        (DefinitionTypes::Rational(_, _), DefinitionTypes::Bool(_)) => Some(Ordering::Less),
        (DefinitionTypes::Rational(num, den), DefinitionTypes::Double(n)) => {
            let q = num.to_f64()? / den.to_f64()?;
            let ord_q = OrderedFloat::from_f64(q)?;
            Some(n.cmp(&ord_q))
        }
        (DefinitionTypes::Rational(num, den), DefinitionTypes::Int(n)) => {
            let q = num.to_f64()? / den.to_f64()?;
            let ord_q: OrderedFloat<f64> = OrderedFloat::from_f64(q)?;
            Some(ord_q.cmp(&OrderedFloat::<f64>::from_f64(n.to_f64()?)?))
        }
        (DefinitionTypes::Rational(num, den), DefinitionTypes::Rational(num2, den2)) => {
            let q = num.to_f64()? / den.to_f64()?;
            let ord_q: OrderedFloat<f64> = OrderedFloat::from_f64(q)?;

            let q2 = num2.to_f64()? / den2.to_f64()?;
            let ord_q2 = OrderedFloat::from_f64(q2)?;
            Some(ord_q.cmp(&ord_q2))
        }
        (DefinitionTypes::Symbol(sym), o) => DATA
            .lock()
            .map(|hm| cmp(hm.get(sym).unwrap_or(&DefinitionTypes::Nil), o))
            .ok(),
        (s2, DefinitionTypes::Symbol(sym)) => DATA
            .lock()
            .map(|hm| cmp(s2, hm.get(sym).unwrap_or(&DefinitionTypes::Nil)))
            .ok(),
        (DefinitionTypes::List(_), o) => Some(cmp(&s.clone().eval().ok()?, o)),
        (s2, DefinitionTypes::List(_)) => Some(cmp(s2, &other.clone().eval().ok()?)),
        (DefinitionTypes::Nil, _) => Some(Ordering::Greater),
        (_, DefinitionTypes::Nil) => Some(Ordering::Less),
        _ => None,
    }
}
