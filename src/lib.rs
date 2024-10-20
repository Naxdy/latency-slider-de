#![feature(restricted_std)]

#[cfg(feature = "css_ui")]
use css_ui::IS_CSS;

use offsets::{LOC_SET_ONLINE_LATENCY, LOC_SET_ROOM_ID, LOC_UPDATE_ROOM};
use skyline::hooks::InlineCtx;

#[cfg(feature = "css_ui")]
mod css_ui;
mod offsets;

#[skyline::from_offset(0x37a1f10)]
unsafe fn set_text_string(pane: u64, string: *const u8);

static mut CURRENT_PANE_HANDLE: usize = 0;
static mut CURRENT_ARENA_ID: String = String::new();
static mut CURRENT_INPUT_BUFFER: isize = 4;
static mut MOST_RECENT_AUTO: isize = -1;
static mut STEALTH_MODE: bool = false;

const MAX_INPUT_BUFFER: isize = 25;
const MIN_INPUT_BUFFER: isize = -1;

struct DpadInputState {
    left_released: bool,
    right_released: bool,
    up_released: bool,
    down_released: bool,
}

static mut DPAD: DpadInputState = DpadInputState {
    left_released: true,
    right_released: true,
    up_released: true,
    down_released: true,
};

unsafe fn handle_user_input(#[cfg(feature = "css_ui")] on_css: bool) {
    if ninput::any::is_press(ninput::Buttons::RIGHT) && DPAD.right_released {
        CURRENT_INPUT_BUFFER += 1;
        DPAD.right_released = false;
    } else if ninput::any::is_press(ninput::Buttons::LEFT) && DPAD.left_released {
        CURRENT_INPUT_BUFFER -= 1;
        DPAD.left_released = false;
    }

    if ninput::any::is_press(ninput::Buttons::UP) && DPAD.up_released {
        STEALTH_MODE = true;
        DPAD.up_released = false;
    } else if ninput::any::is_press(ninput::Buttons::DOWN) && DPAD.down_released {
        STEALTH_MODE = false;
        DPAD.down_released = false;

        #[cfg(feature = "css_ui")]
        {
            // kinda hacky for elite smash text
            if on_css {
                IS_CSS = true;
            }
        }
    }

    // Clear button states (ninput is a shit input library lol)
    if !ninput::any::is_press(ninput::Buttons::RIGHT) {
        DPAD.right_released = true;
    }
    if !ninput::any::is_press(ninput::Buttons::LEFT) {
        DPAD.left_released = true;
    }
    if !ninput::any::is_press(ninput::Buttons::UP) {
        DPAD.up_released = true;
    }
    if !ninput::any::is_press(ninput::Buttons::DOWN) {
        DPAD.down_released = true;
    }

    CURRENT_INPUT_BUFFER = CURRENT_INPUT_BUFFER.clamp(MIN_INPUT_BUFFER, MAX_INPUT_BUFFER);
}

#[skyline::hook(offset = LOC_UPDATE_ROOM.get_offset_in_memory().unwrap(), inline)]
unsafe fn non_hdr_update_room_hook(_: &skyline::hooks::InlineCtx) {
    #[cfg(feature = "css_ui")]
    handle_user_input(false);

    #[cfg(not(feature = "css_ui"))]
    handle_user_input();

    #[cfg(feature = "css_ui")]
    {
        if IS_CSS {
            IS_CSS = false;
        }
    }

    if STEALTH_MODE {
        set_text_string(
            CURRENT_PANE_HANDLE as u64,
            format!("ID: {}\0", CURRENT_ARENA_ID).as_ptr(),
        );
    } else if CURRENT_INPUT_BUFFER == -1 {
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

#[skyline::hook(offset = LOC_SET_ROOM_ID.get_offset_in_memory().unwrap(), inline)]
unsafe fn non_hdr_set_room_id(ctx: &skyline::hooks::InlineCtx) {
    let panel = *((*((*ctx.registers[0].x.as_ref() + 8) as *const u64) + 0x10) as *const u64);
    CURRENT_PANE_HANDLE = panel as usize;
    CURRENT_ARENA_ID = dbg!(String::from_utf16(std::slice::from_raw_parts(
        *ctx.registers[3].x.as_ref() as *const u16,
        5
    ))
    .unwrap());
}

#[skyline::hook(offset = LOC_SET_ONLINE_LATENCY.get_offset_in_memory().unwrap(), inline)]
unsafe fn non_hdr_set_online_latency(ctx: &InlineCtx) {
    let auto = *(*ctx.registers[19].x.as_ref() as *mut u8);

    MOST_RECENT_AUTO = auto as isize;
    if CURRENT_INPUT_BUFFER != -1 {
        *(*ctx.registers[19].x.as_ref() as *mut u8) = CURRENT_INPUT_BUFFER as u8;
    }
}

#[skyline::main(name = "latency-slider-de")]
pub fn main() {
    // make sure that all hooks are findable
    // and don't crash the game if they're not
    if ensure_hooks!(LOC_UPDATE_ROOM, LOC_SET_ROOM_ID, LOC_SET_ONLINE_LATENCY) {
        skyline::install_hooks!(
            non_hdr_set_room_id,
            non_hdr_update_room_hook,
            non_hdr_set_online_latency,
        );
    }

    #[cfg(feature = "css_ui")]
    {
        use css_ui::hook_css_funcs;

        hook_css_funcs();
    }
}
