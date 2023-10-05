pub trait IntoBoxExtension {
    fn into_box(self) -> Box<Self>
    where
        Self: Sized;
}

impl<T> IntoBoxExtension for T {
    fn into_box(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}
