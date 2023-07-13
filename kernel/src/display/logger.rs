use core::fmt::{Arguments, Write};

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::display::logger::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    let mut writter = CharWritterAdapter;
    let _ = writter.write_fmt(args);
}

struct CharWritterAdapter;

impl Write for CharWritterAdapter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        use crate::display::char_writer::CHAR_WRITER;
        CHAR_WRITER.lock().write_string(s);
        Ok(())
    }
}
