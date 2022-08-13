use uart_16550::SerialPort;

pub static mut SERIAL: Option<SerialPort> = None;

pub fn init_serial_port() {
    if unsafe { SERIAL.is_none() } {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        unsafe { SERIAL = Some(serial_port) };
    }
}
