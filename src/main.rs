use anyhow::{Context, Result};
use serialport::{SerialPort, SerialPortType};

enum DisplayCommand {
    Clear = 102,
    ScreenOff = 108,
    ScreenOn = 109,
    DisplayBitmap = 197,
}

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

    fn send(&mut self, bytes: &[u8]) {
        self.conn.write_all(bytes).unwrap();

        if cfg!(debug_assertions) && false {
            println!("{:?}", bytes.to_ascii_lowercase());
        }
    }

    fn send_statefull_command(
        &mut self,
        display_command: DisplayCommand,
        x: u16,
        y: u16,
        _x: u16,
        _y: u16,
    ) {
        let mut buffer: [u8; 6] = [0; 6];

        //  X 10 bits, 8 MSB written, 2 remaining
        buffer[0] = (x >> 2) as u8;

        //  X 10 bits, 2 LSB written, 0 remaining
        //  Y 10 bits, 6 MSB written, 4 remaining
        buffer[1] = (x << 6) as u8 + (y >> 4) as u8;

        //  Y 10 bits, 4 LSB written, 0 remaining
        // _X 10 bits, 4 MSB written, 6 remaining
        buffer[2] = (y << 4) as u8 + (_x >> 6) as u8;

        // _X 10 bits, 6 LSB written, 0 remaining
        // _Y 10 bits, 2 MSB written, 8 remaining
        buffer[3] = (_x << 2) as u8 + (_y >> 8) as u8;

        // _Y 10 bits, 8 LSB written, 0 remaining
        buffer[4] = _y as u8;

        buffer[5] = display_command as u8;

        self.send(&buffer);
    }

    fn send_stateless_command(&mut self, display_command: DisplayCommand) {
        let mut buf = [0u8; 6];
        buf[5] = display_command as u8;
        self.send(&buf)
    }

    fn clear(&mut self) {
        self.send_stateless_command(DisplayCommand::Clear);
    }

    fn turn_on(&mut self) {
        self.send_stateless_command(DisplayCommand::ScreenOn)
    }

    fn turn_ff(&mut self) {
        self.send_stateless_command(DisplayCommand::ScreenOff)
    }

    fn send_rect_draw(&mut self, start_x: u16, start_y: u16, end_x: u16, end_y: u16) {
        self.send_statefull_command(
            DisplayCommand::DisplayBitmap,
            start_x,
            start_y,
            end_x,
            end_y,
        )
    }
}

fn main() -> Result<()> {
    let display = Display::new()?;
    println!("{:#?}", display);

    Ok(())
}
