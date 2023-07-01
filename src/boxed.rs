#[non_exhaustive]
#[must_use]
pub enum FuncBox<T, R> {
    BoxedFunc(Box<dyn BoxedFunc<T, R>>),
    FnOnce(Box<dyn FnOnce(T) -> R>),
}

impl<T, R> FuncBox<T, R> {
    pub fn apply(self, value: T) -> R {
        match self {
            FuncBox::BoxedFunc(f) => f.apply(value),
            FuncBox::FnOnce(f) => f(value)
        }
    }
}

#[macro_export]
macro_rules! func_box {
    ( $f:expr ) => {
        FuncBox::from(Box::new($f))
    };
}

#[macro_export]
macro_rules! typed_box {
    ($t:ty, $v:expr) => {
        Box::new($v) as Box<$t>
    };
}

impl<T: 'static, R: 'static> FuncBox<T, R> {
    pub fn before<S: 'static>(self, before: fn(S) -> T) -> FuncBox<S, R> {
        FuncBox::FnOnce(Box::new(move |value| self.apply(before(value))))
    }
}

impl<T, R, F: BoxedFunc<T, R> + 'static> From<Box<F>> for FuncBox<T, R> {
    fn from(f: Box<F>) -> Self {
        Self::BoxedFunc(f)
    }
}

impl<T, R> From<Box<dyn FnOnce(T) -> R>> for FuncBox<T, R> {
    fn from(f: Box<dyn FnOnce(T) -> R>) -> Self {
        Self::FnOnce(f)
    }
}

pub trait BoxedFunc<T, R> {
    fn apply(self: Box<Self>, value: T) -> R;
}

impl<T, R, F> BoxedFunc<T, R> for F where F: FnOnce(T) -> R {
    fn apply(self: Box<Self>, value: T) -> R {
        self(value)
    }
}
