use std::borrow::Cow;

pub(super) trait Output {
    fn success(&self) -> bool;
    fn output(&self) -> String;
}

pub(super) trait WithCode<'wc> {
    fn with_code(&mut self, code: impl Into<Cow<'wc, str>>);
}
