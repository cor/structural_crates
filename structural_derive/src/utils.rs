use proc_macro2::Span;

pub(crate) trait SpanExt {
    fn combine_span(self, other: Span) -> Span;
}

impl SpanExt for Span {
    fn combine_span(self, other: Span) -> Span {
        self.join(other).unwrap_or(self)
    }
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn remove_raw_prefix(mut s: String) -> String {
    if s.starts_with("r#") {
        s.drain(..2);
    }
    s
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) trait StrExt: AsRef<str> {
    fn surrounded_with(&self, before: &str, after: &str) -> String {
        let this = self.as_ref();
        let mut s = String::with_capacity(this.len() + before.len() + after.len());
        s.push_str(before);
        s.push_str(this);
        s.push_str(after);
        s
    }
}

impl<T> StrExt for T where T: AsRef<str> {}
