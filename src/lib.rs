#![feature(restricted_std)]

use skyline::hooks::InlineCtx;

#[skyline::from_offset(0x37a1ef0)]
unsafe fn set_text_string(pane: u64, string: *const u8);

static mut CURRENT_PANE_HANDLE: usize = 0;
static mut CURRENT_ARENA_ID: String = String::new();
static mut CURRENT_INPUT_BUFFER: isize = 4;
static mut MOST_RECENT_AUTO: isize = -1;
static mut STEALTH_MODE: bool = false;

const MAX_INPUT_BUFFER: isize = 25;
const MIN_INPUT_BUFFER: isize = -1;

#[skyline::hook(offset = 0x18881d0, inline)]
unsafe fn non_hdr_update_room_hook(_: &skyline::hooks::InlineCtx) {
    static mut CURRENT_COUNTER: usize = 0;
    if ninput::any::is_press(ninput::Buttons::RIGHT) {
        if CURRENT_COUNTER == 0 {
            CURRENT_INPUT_BUFFER += 1;
        }
        CURRENT_COUNTER = (CURRENT_COUNTER + 1) % 10;
    } else if ninput::any::is_press(ninput::Buttons::LEFT) {
        if CURRENT_COUNTER == 0 {
            CURRENT_INPUT_BUFFER -= 1;
        }
        CURRENT_COUNTER = (CURRENT_COUNTER + 1) % 10;
    } else {
        CURRENT_COUNTER = 0;
    }

    if ninput::any::is_press(ninput::Buttons::UP) {
        STEALTH_MODE = true;
    } else if ninput::any::is_press(ninput::Buttons::DOWN) {
        STEALTH_MODE = false;
    }

    CURRENT_INPUT_BUFFER = CURRENT_INPUT_BUFFER.clamp(MIN_INPUT_BUFFER, MAX_INPUT_BUFFER);

    if STEALTH_MODE {
        set_text_string(
            CURRENT_PANE_HANDLE as u64,
            format!("ID: {}", CURRENT_ARENA_ID).as_ptr(),
        );
        return;
    }

    if CURRENT_INPUT_BUFFER == -1 {
        if MOST_RECENT_AUTO == -1 {
            set_text_string(
                CURRENT_PANE_HANDLE as u64,
                format!("ID: {}\nInput Latency: Auto\0", CURRENT_ARENA_ID).as_ptr(),
            );
        } else {
            set_text_string(
                CURRENT_PANE_HANDLE as u64,
                format!(
                    "ID: {}\nInput Latency: Auto ({})\0",
                    CURRENT_ARENA_ID, MOST_RECENT_AUTO
                )
                .as_ptr(),
            )
        }
    } else {
        set_text_string(
            CURRENT_PANE_HANDLE as u64,
            format!(
                "ID: {}\nInput Latency: {}\0",
                CURRENT_ARENA_ID, CURRENT_INPUT_BUFFER
            )
            .as_ptr(),
        );
    }
}

#[skyline::hook(offset = 0x1887afc, inline)]
unsafe fn non_hdr_set_room_id(ctx: &skyline::hooks::InlineCtx) {
    let panel = *((*((*ctx.registers[0].x.as_ref() + 8) as *const u64) + 0x10) as *const u64);
    CURRENT_PANE_HANDLE = panel as usize;
    CURRENT_ARENA_ID = dbg!(String::from_utf16(std::slice::from_raw_parts(
        *ctx.registers[3].x.as_ref() as *const u16,
        5
    ))
    .unwrap());
}

#[skyline::hook(offset = 0x16ccc58, inline)]
unsafe fn non_hdr_set_online_latency(ctx: &InlineCtx) {
    let auto = *(*ctx.registers[19].x.as_ref() as *mut u8);

    MOST_RECENT_AUTO = auto as isize;
    if CURRENT_INPUT_BUFFER != -1 {
        *(*ctx.registers[19].x.as_ref() as *mut u8) = CURRENT_INPUT_BUFFER as u8;
    }
}

#[skyline::main(name = "latency-slider-de")]
pub fn main() {
    skyline::install_hooks!(
        non_hdr_set_room_id,
        non_hdr_update_room_hook,
        non_hdr_set_online_latency,
    );
}
