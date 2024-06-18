//runars escpos thingy :-D
//just contains a bunch of escpos commands, really
//you still have to know how to use them, cause I don't lmao


/// List of commands for the TM-T88III printer:
/// HT
/// LF
/// FF (in page mode)
/// CR
/// CAN
/// DLE EOT
/// DLE ENQ
/// DLE DC4 (fn = 1)
/// DLE DC4 (fn = 8)
/// ESC FF
/// ESC SP
/// ESC !
/// ESC $
/// ESC %
/// ESC &
/// ESC *
/// ESC -
/// ESC 2
/// ESC 3
/// ESC =
/// ESC ?
/// ESC @
/// ESC D
/// ESC E
/// ESC G
/// ESC J
/// ESC L
/// ESC M
/// ESC R
/// ESC S
/// ESC T
/// ESC V
/// ESC W
/// ESC \
/// ESC a
/// ESC c 3
/// ESC c 4
/// ESC c 5
/// ESC d
/// ESC p
/// ESC t
/// ESC {
pub struct CommandBuilder {
    commands: Vec<u8>,
}

impl CommandBuilder {

    /// Creates a new, empty 'CommandBuilder'.
    /// 
    /// Returns a 'CommandBuilder' with no commands.
    pub fn new() -> Self {
        CommandBuilder {
            commands: Vec::new(),
        }
    }

    /// Initializes the printer.
    /// 
    /// This command resets the printer to its default state, clearing the buffer
    /// and setting standard modes
    /// 
    /// Returns a mutable reference to the builder for chaining.
    pub fn init_printer(&mut self) -> &mut Self {
        self.commands.extend_from_slice(b"\x1B\x40");
        self
    }

    pub fn print_and_line_feed(&mut self) -> &mut Self {
        self.commands.extend_from_slice(b"\x0A");
        self
    }

    pub fn print_and_return_to_std(&mut self) -> &mut  Self {
        self.commands.extend_from_slice(b"\x0C");
        self
    }

    pub fn print_and_carriage_return(&mut self) -> &mut  Self {
        self.commands.extend_from_slice(b"\x0D");
        self
    }

    pub fn print_page_mode(&mut self) -> &mut  Self {
        self.commands.extend_from_slice(b"\x1B\x0C");
        self
    }

    pub fn print_and_feed(&mut self, n: u8) -> &mut  Self {
        self.commands.extend(&vec![0x1B, 0x4A, n]);
        self
    }

    pub fn print_and_feed_n(&mut self, n: u8) -> &mut  Self {
        self.commands.extend(&vec![0x1B, 0x64, n]);
        self
    }

    pub fn select_default_line_spacing(&mut self) -> &mut  Self {
        self.commands.extend_from_slice(b"\x1B\x32");
        self
    }

    pub fn select_line_spacing(&mut self, n: u8) -> &mut  Self {
        self.commands.extend(&vec![0x1B, 0x33, n]);
        self
    }

    pub fn cancel_print_data(&mut self) -> &mut  Self {
        self.commands.extend_from_slice(b"\x18");
        self
    }

    pub fn set_right_char_spacing(&mut self, n: u8) -> &mut  Self {
        self.commands.extend(&vec![0x1B, 0x20, n]);
        self
    }

    pub fn select_print_mode(&mut self, n: u8) -> &mut  Self {
        self.commands.extend(&vec![0x1B, 0x21, n]);
        self
    }

    pub fn select_user_defined_char_set(&mut self, n: u8) -> &mut  Self {
        self.commands.extend(&vec![0x1B, 0x25, n]);
        self
    }

    pub fn underline_mode(&mut self, n: u8) -> &mut  Self {
        self.commands.extend(&vec![0x1B, 0x2D, n]);
        self
    }

    pub fn emphasized_mode(&mut self, n: u8) -> &mut  Self {
        self.commands.extend(&vec![0x1B, 0x45, n]);
        self
    }

    pub fn double_strike_mode(&mut self) -> &mut  Self {
        self.commands.extend_from_slice(b"\x1B\x47");
        self
    }

    pub fn select_char_font(&mut self, n: u8) -> &mut  Self {
        self.commands.extend(&vec![0x1B, 0x4D, n]);
        self
    }

    pub fn select_international_char_set(&mut self, n: u8) -> &mut  Self {
        self.commands.extend(&vec![0x1B, 0x52, n]);
        self
    }

    pub fn rotate_90_degrees(&mut self, n: u8) -> &mut  Self {
        self.commands.extend(&vec![0x1B, 0x56, n]);
        self
    }

    pub fn upside_down_mode(&mut self, n: u8) -> &mut  Self {
        self.commands.extend(&vec![0x1B, 0x7B, n]);
        self
    }

    pub fn select_char_size(&mut self, n: u8) -> &mut  Self {
        self.commands.extend(&vec![0x1D, 0x21, n]);
        self
    }

    pub fn reverse_printing(&mut self, n: u8) -> &mut  Self {
        self.commands.extend(&vec![0x1D, 0x42, n]);
        self
    }

    pub fn smooth_mode(&mut self, n: u8) -> &mut  Self {
        self.commands.extend(&vec![0x1D, 0x62, n]);
        self
    }

    pub fn panel_button_mode(&mut self, n: u8) -> &mut  Self {
        self.commands.extend(&vec![0x1B, 0x63, 0x35, n]);
        self
    }

    pub fn select_paper_sensor(&mut self, n: u8) -> &mut  Self {
        self.commands.extend(&vec![0x1B, 0x63, 0x33, n]);
        self
    }

    pub fn select_paper_sensor_stop(&mut self, n: u8) -> &mut  Self {
        self.commands.extend(&vec![0x1B, 0x63, 0x34, n]);
        self
    }

    pub fn horizontal_tab(&mut self) -> &mut  Self {
        self.commands.extend_from_slice(b"\x09");
        self
    }

    pub fn build(self) -> Vec<u8> {
        self.commands
    }
}