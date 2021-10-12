use std::borrow::Cow;

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
