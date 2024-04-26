#[derive(Template)]
#[template(path = "error.html")]
pub struct Error<T: Display> {
    title: Cow<'static, str>,
    message: T,
}
