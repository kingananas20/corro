use crate::common::separate_cargo_output;
use std::borrow::Cow;

pub(super) trait Output {
    fn success(&self) -> bool;
    fn stdout(&self) -> &str;
    fn stderr(&self) -> &str;
    fn output(&self) -> Cow<'_, str> {
        if self.success() {
            let combi = format!("{}\n{}", self.stderr(), self.stdout());
            let (_, other) = separate_cargo_output(&combi);
            Cow::Owned(other.to_owned())
        } else {
            Cow::Borrowed(self.stderr())
        }
    }
}

pub(super) trait WithCode<'wc> {
    fn with_code(&mut self, code: impl Into<Cow<'wc, str>>);
}
