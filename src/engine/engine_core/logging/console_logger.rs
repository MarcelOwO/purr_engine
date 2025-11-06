use crate::engine_core::logging::logger::Logger;
pub(crate) struct ConsoleLogger {}

impl ConsoleLogger {
    pub(crate) fn new() -> Self {
        Self{}
    }
}

impl Logger for ConsoleLogger {


    fn log(msg: String) {
        println!("{}", msg)
    }
}
