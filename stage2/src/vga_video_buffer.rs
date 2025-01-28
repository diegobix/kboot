use core::
    ptr::write_volatile
;

static mut VGA_BUFFER: Option<VgaBuffer> = None;

/// Representa un color del buffer vga
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

/// Estructura que contiene cada caracter del VGA Buffer
#[derive(Clone, Copy)]
#[repr(C)]
struct PrintableChar {
    char: u8,
    colour: u8,
}

impl PrintableChar {
    /// Constructor de PrintableChar
    /// 
    /// ## Arguments
    /// * `foreground` - Color del texto.
    /// * `background` - Color del fondo.
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
            char: b' ',
            colour: 0,
        }
    }
}

// constantes de tamaño del buffer
const VGA_BUFFER_WIDTH: usize = 80;
const VGA_BUFFER_HEIGHT: usize = 25;

/// Dirección VGA Buffer 
const VGA_BUFFER_ADDR: usize = 0xb8000;

/// Representa la memoria del VGA Buffer
#[repr(transparent)]
struct Buffer([[PrintableChar; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT]);

/// Representa el VGA Buffer
pub struct VgaBuffer {
    col: usize,
    f_colour: Colour,
    b_colour: Colour,
    buffer: *mut Buffer,
}

impl VgaBuffer {

    /// Singleton
    pub fn instance() -> &'static mut Self {
        unsafe {
            if VGA_BUFFER.is_none() {
                VGA_BUFFER = Some(VgaBuffer::new(Colour::White, Colour::Black));
            }
            VGA_BUFFER.as_mut().unwrap()
        }
    }

    pub fn change_colour(&mut self, f_colour: Colour, b_colour: Colour) {
        self.f_colour = f_colour;
        self.b_colour = b_colour;
    }

    fn print_char(&mut self, char: char) {
        if char == '\n' || self.col >= VGA_BUFFER_WIDTH {
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

            let row = &mut buf.0[VGA_BUFFER_HEIGHT - 1];
            for character in row.iter_mut() {
                write_volatile(character, PrintableChar::default());
            }
        }
        self.col = 0;
    }
}

/// Imprime un string en el VGA_BUFFFER
#[macro_export]
macro_rules! vga_log {
    ($msg:expr) => {
        $crate::vga_video_buffer::VgaBuffer::instance().print_string($msg);
    };
}

/// Imprime un string con salto de linea en el VGA_BUFFFER
#[macro_export]
macro_rules! vga_logln {
    ($msg:expr) => {
        vga_log!(concat!($msg, "\n"));
    };
}