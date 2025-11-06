pub trait Logger {
    fn log(msg: String) where Self: Sized;
}
