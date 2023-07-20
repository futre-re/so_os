use crate::task::executor::Executor;
use crate::task::keyboard::ScancodeStream;
use crate::task::{keyboard, Task};
use crate::vga_buffer::{self, Writer, WRITER};
use crate::{gdt, hlt_loop};
use crate::{print, println};
use futures_util::stream::StreamExt;
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};

// pub fn delet_byte() {
//     let color_code = self.color_code;
//     let row = BUFFER_HEIGHT - 1;
//     let col = self.column_position - 1;
//     if col >= 2 {
//         self.buffer.chars[row][col].write(ScreenChar {
//             ascii_character: b' ',
//             color_code,
//         });
//         self.column_position -= 1;
//     } else {
//     }
// }

pub async fn shell() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);
    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                key_task(key);
                // match key {
                //     DecodedKey::Unicode(character) => match DecodedKey::Unicode(character) {
                //         DecodedKey::Unicode('\n') => print!("{}>>", '\n'),
                //         DecodedKey::Unicode('\u{8}') => print!("{}", '\u{8}'),
                //         _ => print!("{}", character),
                //     },
                //     DecodedKey::RawKey(key) => print!("{:?}", key),
                // }
            }
        }
    }
}
fn key_task(key: DecodedKey) {
    match key {
        DecodedKey::Unicode(character) => match DecodedKey::Unicode(character) {
            DecodedKey::Unicode('\n') => print!("{}>>", '\n'),
            DecodedKey::Unicode('\u{8}') => print!("{}", '\u{8}'),
            _ => print!("{}", character),
        },
        pc_keyboard::DecodedKey::RawKey(KeyCode::ArrowLeft) => {
            print!("{}", '\u{128}');
        }
        pc_keyboard::DecodedKey::RawKey(KeyCode::ArrowRight) => {
            print!("{}", '\u{129}');
        }
        DecodedKey::RawKey(key) => {
            print!("{:?}", key)
        }
    }
}
