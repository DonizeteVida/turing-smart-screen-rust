use serialport::SerialPort;
use anyhow::Result;

#[derive(Debug)]
struct Display {
    conn: Box<dyn SerialPort>,
}

impl Display {
    fn new() -> Result<Self> {
        let mut available_ports = serialport::available_ports()?;
        let port_info = available_ports.pop().unwrap();

        let mut conn = serialport::new(port_info.port_name, 115_200).open()?;
        conn.write_request_to_send(true)?;

        Ok(Self { conn })
    }
}

fn main() -> Result<()> {
    let display = Display::new()?;
    println!("{:#?}", display);

    Ok(())
}
