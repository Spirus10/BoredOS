use core::fmt;
use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;


// We use a C-like enum to specify the number for each color
// repr(u8) ensures that each variant is stored as a u8
// 4 bits would be sufficient, but Rust lacks a `u4` type
// By deriving the `Copy`, `Clone`, `Debug`, `PartialEq`, and `Eq` traits
// we enable copy semantics for the type and make it printable, and comparable

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
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

// The `ColorCode` struct contains the full color byte, containing foreground
// and background color. We use repr(transparent) to ensure it has the same layout in
// memory as a `u8`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// Since the field ordering in default structs is undefined in Rust
// we use repr(C) to guarantee that the fields are layed out exactly
// like a C struct and thus guarantees the correct field ordering. 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// We use `repr(transparent)` here again to ensure that the struct
// has the same memory layout as its singular field.
// We use volatile here, as we never read from the `Buffer` after writing to it
// the compiler knows nothing about the side effect that the characters appear on screen;
// and therefore, it may optimize the write away - so we use volatile to tell
// the compiler that the write has side effects, and shouldn't be optimized away
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// The writer will always write to the last line and shift lines up when a line is full
// (or on `\n`) - we specify a static lifetime on the reference to the VGA buffer
// as the buffer will need to live for the entire program run time
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

// This is implemented to write from the bottom of the screen, and 
// push written lines upward with each newline
impl Writer {
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
                
                // We use `.write()` instead of `=` to ensure we perform a volatile write
                // guarenteeing that the compiler wont optimize it away
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

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

// Allows us to use the `write!` and `writeln!` macros
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// We use `lazy_static` because statics in Rust are initialized at compile time
// in contrast, normal variables are initialized at run time. `Color::new` would
// be solvable using `const` functions, but the problem here is that Rust's
// const evaluator is not able to convert raw pointers to references at compile time
// `lazy_static` allows us to create a static whose value(s) are computed at the time
// static is first accessed, rather than at compile time.
lazy_static! {
    // Since we need mutability, as all the write methods take `&mut self`
    // We use a spinlock, as it is a basic Mutex, with no required OS features
    // that still provides us with interior mutability
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

// Here we just yeet the std implementation and replace with our own print function
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
// end yeet

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
