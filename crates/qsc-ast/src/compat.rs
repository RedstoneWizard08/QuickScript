use miette::{NamedSource, SourceCode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrappedNamedSource<T: SourceCode + 'static> {
    _inner: NamedSource<T>,
}

impl<T: SourceCode + 'static> Into<NamedSource<T>> for WrappedNamedSource<T> {
    fn into(self) -> NamedSource<T> {
        self._inner
    }
}

impl<T: SourceCode + 'static> From<NamedSource<T>> for WrappedNamedSource<T> {
    fn from(value: NamedSource<T>) -> Self {
        Self { _inner: value }
    }
}

impl Default for WrappedNamedSource<String> {
    fn default() -> Self {
        Self {
            _inner: NamedSource::new("<none>", String::new()),
        }
    }
}
