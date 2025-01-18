use core::{cell::{LazyCell, OnceCell, RefCell, RefMut}, fmt::{self, Write}, ptr::write_volatile};

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Colour {
    Black = 0,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightMagenta,
    Yellow,
    White,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct PrintableChar {
    char: u8,
    colour: u8,
}

impl PrintableChar {
    fn new(char: u8, foreground: u8, background: u8) -> Self {
        PrintableChar {
            char,
            colour: (background << 4) | foreground,
        }
    }
}

impl Default for PrintableChar {
    fn default() -> Self {
        Self {
            char: b'3',
            colour: 0,
        }
    }
}

const VGA_BUFFER_WIDTH: usize = 80;
const VGA_BUFFER_HEIGHT: usize = 25;

const VGA_BUFFER_ADDR: usize = 0xb8000;

#[repr(transparent)]
struct Buffer([[PrintableChar; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT]);

pub struct VgaBuffer {
    col: usize,
    f_colour: Colour,
    b_colour: Colour,
    buffer: *mut Buffer,
}

impl VgaBuffer {
    fn new(f_colour: Colour, b_colour: Colour) -> Self {
        VgaBuffer {
            col: 0,
            f_colour,
            b_colour,
            buffer: VGA_BUFFER_ADDR as *mut Buffer,
        }
    }

    pub fn change_colour(&mut self, f_colour: Colour, b_colour: Colour) {
        self.f_colour = f_colour;
        self.b_colour = b_colour;
    }

    fn print_char(&mut self, char: char) {
        if char == '\n' || self.col >= VGA_BUFFER_WIDTH {
            _print(format_args!("new line"));
            self.new_line();
            return;
        }

        let byte = char as u8;
        let row = VGA_BUFFER_HEIGHT - 1;

        let printable_char = PrintableChar::new(byte, self.f_colour as u8, self.b_colour as u8);

        unsafe {
            let dst = &mut (*self.buffer).0[row][self.col] as *mut PrintableChar;
            write_volatile(dst, printable_char);
        }
        self.col += 1;
    }

    pub fn print_string(&mut self, s: &str) {
        for c in s.chars() {
            self.print_char(c);
        }
    }

    fn new_line(&mut self) {
        unsafe {
            let buf = &mut (*self.buffer);
            buf.0.rotate_left(1);
            
            for col in 0..VGA_BUFFER_WIDTH {
                let dst = &mut buf.0[VGA_BUFFER_HEIGHT-1][col] as *mut PrintableChar;

                write_volatile(dst, PrintableChar::default());
            }
        }
        self.col = 0;
    }
}

static mut VGA_BUFFER: OnceCell<RefCell<VgaBuffer>> = OnceCell::new();

impl fmt::Write for VgaBuffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print_string(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let buf = unsafe {
        VGA_BUFFER.get_or_init(|| RefCell::new(VgaBuffer::new(Colour::White, Colour::Black)))
    };
    buf.borrow_mut().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! vga_print {
    ($($arg:tt)*) => {
        $crate::vga_buffer::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! vga_println {
    () => ($crate::vga_print!("\n"));
    ($fmt:expr, $($arg:tt)*) => (
        $crate::vga_print!(concat!($fmt, "\n"), $($arg)*)
    );
    ($arg:tt) => (
        $crate::vga_print!(concat!($arg, "\n"))
    );
}
