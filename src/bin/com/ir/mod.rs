pub trait IRController {
    fn send(&self) -> Result<i8, &'static str>;
}
