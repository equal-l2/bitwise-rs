#![allow(non_upper_case_globals)]

use libc::c_char;
use libc::c_int as int;
use libc::c_ulonglong as ull;

extern "C" {
    fn base_scanf(buf: *const c_char, base: int, val: *mut u64) -> int;
}

/* IEC Standard */
const KiB: ull = 1 << 10;
const MiB: ull = 1 << 20;
const GiB: ull = 1 << 30;
const TiB: ull = 1 << 40;
const PiB: ull = 1 << 50;

/* SI Standard */
const KB: ull = 1000;
const MB: ull = 1000 * KB;
const GB: ull = 1000 * MB;
const TB: ull = 1000 * GB;
const PB: ull = 1000 * TB;

pub static mut g_has_color: int = 1;
pub static mut g_width: int = 0;
pub static mut g_input_avail: bool = false;
pub static mut g_input: int = 0;

const RED: *const c_char = b"\x1B[31m\0".as_ptr() as *const c_char;
const GREEN: *const c_char = b"\x1B[32m\0".as_ptr() as *const c_char;
const YEL: *const c_char = b"\x1B[33m\0".as_ptr() as *const c_char;
const BLUE: *const c_char = b"\x1B[34m\0".as_ptr() as *const c_char;
const MAGENTA: *const c_char = b"\x1B[35m\0".as_ptr() as *const c_char;
const CYAN: *const c_char = b"\x1B[36m\0".as_ptr() as *const c_char;
const WHITE: *const c_char = b"\x1B[37m\0".as_ptr() as *const c_char;
const RESET: *const c_char = b"\x1B[0m\0".as_ptr() as *const c_char;
const NOTHING: *const c_char = b"\0".as_ptr() as *const c_char;

pub static mut color_green: *const c_char = NOTHING;
pub static mut color_red: *const c_char = NOTHING;
pub static mut color_blue: *const c_char = NOTHING;
pub static mut color_magenta: *const c_char = NOTHING;
pub static mut color_cyan: *const c_char = NOTHING;
pub static mut color_white: *const c_char = NOTHING;
pub static mut color_reset: *const c_char = NOTHING;

#[no_mangle]
pub unsafe extern "C" fn init_colors() {
    if g_has_color != 0 {
        color_green = GREEN;
        color_red = RED;
        color_blue = BLUE;
        color_magenta = MAGENTA;
        color_cyan = CYAN;
        color_white = WHITE;
        color_reset = RESET;
    } else {
        color_green = NOTHING;
        color_red = NOTHING;
        color_blue = NOTHING;
        color_magenta = NOTHING;
        color_cyan = NOTHING;
        color_white = NOTHING;
        color_reset = NOTHING;
    }
}

#[no_mangle]
pub unsafe extern "C" fn init_terminal() {
    use ncurses::*;
    initscr();
    if !has_colors() {
        g_has_color = 0;
    } else {
        start_color();
        init_colors();
    }
    cbreak();
    noecho();
    nonl();
    intrflush(std::ptr::null_mut(), false);
    keypad(stdscr(), true);
    curs_set(CURSOR_VISIBILITY::CURSOR_VERY_VISIBLE);
}

#[no_mangle]
pub extern "C" fn deinit_terminal() {
    ncurses::endwin();
}

#[no_mangle]
pub unsafe extern "C" fn validate_input(ch: int, base: int) -> int {
    match base {
        2 => {
            if ch == '0' as int || ch == '1' as int {
                return 0;
            }
        }
        8 => {
            if ch >= '0' as int && ch <= '7' as int {
                return 0;
            }
        }
        16 => {
            if (ch >= '0' as int && ch <= '9' as int)
                || (ch >= 'A' as int && ch <= 'F' as int)
                || (ch >= 'a' as int && ch <= 'f' as int)
            {
                return 0;
            };
        }
        10 => {
            if libc::isdigit(ch) != 0 {
                return 0;
            }
        }
        _ => {}
    }
    return 1;
}

#[no_mangle]
pub unsafe extern "C" fn binary_scanf(buf: *const c_char, val: *mut u64) -> int {
    let mut value: u64 = 0;

    /* Skip the leading b */
    buf.add(1);

    while *buf != b'\0' as c_char {
        let c = *buf as u8;
        match c {
            b'0' => {
                value <<= 1;
            }
            b'1' => {
                value <<= 1;
                value += 1;
            }
            _ => {}
        }
        buf.add(1);
    }

    *val = value;

    return 1;
}

#[no_mangle]
pub unsafe extern "C" fn parse_input(input: *const c_char, val: *mut u64) -> int {
    let base: int = if libc::tolower(*input.offset(0) as int) == 'b' as int {
        2
    } else if *input.offset(0) == '0' as c_char {
        if *input.offset(1) == 'x' as c_char || *input.offset(1) == 'X' as c_char {
            16
        } else {
            8
        }
    } else {
        10
    };

    return base_scanf(input, base, val);
}

#[no_mangle]
pub unsafe extern "C" fn set_width_by_val(val: u64) {
    if (val & 0xFFFFFFFF00000000) != 0 {
        g_width = 64;
    } else if (val & 0xFFFF0000) != 0 {
        g_width = 32;
    } else if (val & 0xFF00) != 0 {
        g_width = 16;
    } else if (val & 0xFF) != 0 {
        g_width = 8;
    } else {
        g_width = 32;
    }
}

#[no_mangle]
pub unsafe extern "C" fn set_width(width: c_char) -> int {
    if libc::tolower(width as int) == 'b' as int {
        g_width = 8;
    } else if libc::tolower(width as int) == 'w' as int {
        g_width = 16;
    } else if libc::tolower(width as int) == 'l' as int {
        g_width = 32;
    } else if libc::tolower(width as int) == 'd' as int {
        g_width = 64;
    } else {
        return 1;
    }

    return 0;
}
