use anyhow::{Context, Result};
use serialport::{SerialPort, SerialPortType};

#[derive(Debug)]
struct Display {
    conn: Box<dyn SerialPort>,
}

impl Display {
    fn new() -> Result<Self> {
        let available_ports = serialport::available_ports()?;
        let turing_device = available_ports
            .iter()
            .filter(|info| {
                if let SerialPortType::UsbPort(ref info) = info.port_type {
                    info.serial_number == Some("USB35INCHIPSV2".to_owned())
                } else {
                    false
                }
            })
            .collect::<Vec<_>>()
            .pop()
            .context("Turing device not found")?;

        let mut conn = serialport::new(turing_device.port_name.to_owned(), 115_200).open()?;
        conn.write_request_to_send(true)?;

        Ok(Self { conn })
    }
}

fn main() -> Result<()> {
    let display = Display::new()?;
    println!("{:#?}", display);

    Ok(())
}
