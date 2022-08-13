#[cfg(not(test))]
use crate::text_display::TEXTWRITER;

#[cfg(test)]
use crate::serial::SERIAL;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;

    #[cfg(not(test))]
    if let Some(ref mut writer) = unsafe { TEXTWRITER.as_mut() } {
        writer.write_fmt(args).unwrap();
    }
    #[cfg(test)]
    if let Some(ref mut port) = unsafe { SERIAL.as_mut() } {
        port.write_fmt(args).unwrap();
    }
}
