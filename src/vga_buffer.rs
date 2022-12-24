const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)] // repr(u8) attribute, means that each enum variant is stored as a u8
pub enum Color {
  Black = 0,
  Blue = 1,
  Green = 2,
  Cyan = 3,
  Red = 4,
  Magenta = 5,
  Brown = 6,
  LightGray = 7,
  DarkGray = 8,
  LightBlue = 9,
  LightGreen = 10,
  LightCyan = 11,
  LightRed = 12,
  Pink = 13,
  Yellow = 14,
  White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8); // The ColorCode struct contains the full color byte, containing foreground and background color.

impl ColorCode {
  fn new(foreground: Color, background: Color) -> ColorCode {
    ColorCode((background as u8) << 4 | (foreground as u8))
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // It guarantees that the struct’s fields are laid out exactly like in a C struct and thus guarantees the correct field ordering
struct ScreenChar {
  ascii_character: u8,
  color_code: ColorCode,
}

#[repr(transparent)] // ensure that the struct has the same memory layout as its single field.
struct Buffer {
  chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT], // Text buffer.
}

pub struct Writer {
  column_position: usize, // Keeps track of the current position in the last row.
  color_code: ColorCode,  // The current foreground and background colors.
  buffer: &'static mut Buffer, // Reference to the VGA buffer.
}

impl Writer {
  /*
    We can use the Writer to modify the buffer’s characters.
    First we create amethod to write a single ASCII byte.

    If the byte is the newline byte \n, the writer does not print anything.
    Instead, it calls a new_line method, which we’ll implement later.
    Other bytes get printed to the screen in the second match case.

    When printing a byte, the writer checks if the current line is full.
    In that case, a new_line call is used to wrap the line.
    Then it writes a new ScreenChar to the buffer at the current position.
    Finally, the current column position is advanced.

    To print whole strings, we can convert them to bytes and print
    them one-by-one and use the method "write_string"
  */
  pub fn write_byte(&mut self, byte: u8) {
    match byte {
      b'\n' => self.new_line(),
      byte => {
        if self.column_position >= BUFFER_WIDTH {
          self.new_line();
        }

        let row = BUFFER_HEIGHT - 1;
        let col = self.column_position;

        let color_code = self.color_code;
        self.buffer.chars[row][col] = ScreenChar {
          ascii_character: byte,
          color_code,
        };
        self.column_position += 1;
      }
    }
  }

  fn new_line(&mut self) { /* TODO */
  }

  /*
    The VGA text buffer only supports ASCII and the additional bytes of code page 437.
    Rust strings are UTF-8 by default, so they might contain bytes that are not supported
    by the VGA text buffer. We use a match to differentiate printable ASCII bytes
    (a newline or anything in between a space character and a ~ character) and unprintable bytes.
    For unprintable bytes, we print a ■ character, which has the hex code 0xfe on the VGA hardware.
  */
  pub fn write_string(&mut self, s: &str) {
    for byte in s.bytes() {
      match byte {
        // printable ASCII byte or newline
        0x20..=0x7e | b'\n' => self.write_byte(byte),
        // not part of printable ASCII range
        _ => self.write_byte(0xfe),
      }
    }
  }
}

/*
  It first creates a new Writer that points to the VGA buffer at 0xb8000.
  The syntax for this might seem a bit strange: First, we cast the integer 0xb8000
  as a mutable raw pointer. Then we convert it to a mutable reference by
  dereferencing it (through *) and immediately borrowing it again (through &mut).
  This conversion requires an unsafe block, since the compiler can’t guarantee
  that the raw pointer is valid.

  Then it writes the byte b'H' to it. The b prefix creates a byte literal, which
  represents an ASCII character. By writing the strings "ello " and "Wörld!", we
  test our write_string method and the handling of unprintable characters.
  To see the output, we need to call the print_something function from our _start function:
*/
pub fn print_something() {
  let mut writer = Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
  };

  writer.write_byte(b'H');
  writer.write_string("ello ");
  writer.write_string("Wörld!");
}
