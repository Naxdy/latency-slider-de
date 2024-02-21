#![feature(restricted_std)]

use skyline::hooks::InlineCtx;
use skyline::nn::ui2d::{Layout, Pane};
use smash::ui2d::{SmashPane, SmashTextBox};

#[skyline::from_offset(0x37a1ef0)]
unsafe fn set_text_string(pane: u64, string: *const u8);

static mut CURRENT_PANE_HANDLE: usize = 0;
static mut CURRENT_ARENA_ID: String = String::new();
static mut CURRENT_INPUT_BUFFER: isize = 4;
static mut MOST_RECENT_AUTO: isize = -1;
static mut STEALTH_MODE: bool = false;
static mut ORIG_VIP_TEXT: String = String::new();

const MAX_INPUT_BUFFER: isize = 25;
const MIN_INPUT_BUFFER: isize = -1;

static mut DPAD_RIGHT_RELEASED: bool = true;
static mut DPAD_LEFT_RELEASED: bool = true;
static mut DPAD_UP_RELEASED: bool = true;
static mut DPAD_DOWN_RELEASED: bool = true;

unsafe fn handle_user_input() {
    if ninput::any::is_press(ninput::Buttons::RIGHT) && DPAD_RIGHT_RELEASED {
        CURRENT_INPUT_BUFFER += 1;
        DPAD_RIGHT_RELEASED = false;
    } else if ninput::any::is_press(ninput::Buttons::LEFT) && DPAD_LEFT_RELEASED {
        CURRENT_INPUT_BUFFER -= 1;
        DPAD_LEFT_RELEASED = false;
    }

    if ninput::any::is_press(ninput::Buttons::UP) && DPAD_UP_RELEASED {
        STEALTH_MODE = true;
        DPAD_UP_RELEASED = false;
    } else if ninput::any::is_press(ninput::Buttons::DOWN) && DPAD_DOWN_RELEASED {
        STEALTH_MODE = false;
        DPAD_DOWN_RELEASED = false;
    }

    // Clear button states (ninput is a shit input library lol)
    if !ninput::any::is_press(ninput::Buttons::RIGHT) {
        DPAD_RIGHT_RELEASED = true;
    }
    if !ninput::any::is_press(ninput::Buttons::LEFT) {
        DPAD_LEFT_RELEASED = true;
    }
    if !ninput::any::is_press(ninput::Buttons::UP) {
        DPAD_UP_RELEASED = true;
    }
    if !ninput::any::is_press(ninput::Buttons::DOWN) {
        DPAD_DOWN_RELEASED = true;
    }

    CURRENT_INPUT_BUFFER = CURRENT_INPUT_BUFFER.clamp(MIN_INPUT_BUFFER, MAX_INPUT_BUFFER);
}

#[skyline::hook(offset = 0x18881d0, inline)]
unsafe fn non_hdr_update_room_hook(_: &skyline::hooks::InlineCtx) {
    handle_user_input();

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

#[skyline::hook(offset = 0x004b620)]
unsafe fn handle_draw_hook(layout: *mut Layout, draw_info: u64, cmd_buffer: u64) {
    // let layout_name = skyline::from_c_str((*layout).layout_name);
    let root_pane = &mut *(*layout).root_pane;

    draw_ui(&root_pane);

    call_original!(layout, draw_info, cmd_buffer);
}

#[skyline::hook(offset = 0x1a12f40)]
unsafe fn update_css_hook(arg: u64) {
    handle_user_input();

    call_original!(arg)
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

unsafe fn draw_ui(root_pane: &Pane) {
    let vip_pane_00 = root_pane.find_pane_by_name_recursive("txt_vip_title_00");
    let vip_pane_01 = root_pane.find_pane_by_name_recursive("txt_vip_title_01");

    if ORIG_VIP_TEXT.is_empty() || ORIG_VIP_TEXT.len() <= 0 {
        match (vip_pane_00, vip_pane_01) {
            (Some(x), _) | (_, Some(x)) => {
                // get from raw using x.as_textbox().text_buf and x.as_textbox().text_buf_len
                ORIG_VIP_TEXT = String::from_utf16(std::slice::from_raw_parts(
                    x.as_textbox().text_buf as *mut u16,
                    x.as_textbox().text_buf_len as usize,
                ))
                .unwrap();
            }
            _ => (),
        }
    } else {
        match (vip_pane_00, vip_pane_01) {
            (Some(x), Some(y)) => {
                let s = if !STEALTH_MODE {
                    if CURRENT_INPUT_BUFFER != -1 {
                        format!("Input Latency: {}", CURRENT_INPUT_BUFFER)
                    } else {
                        if MOST_RECENT_AUTO == -1 {
                            format!("Input Latency: Auto")
                        } else {
                            format!("Input Latency: Auto ({})", MOST_RECENT_AUTO)
                        }
                    }
                } else {
                    ORIG_VIP_TEXT.clone()
                };

                for e in [x, y] {
                    e.as_textbox().set_text_string(s.as_str());
                }
            }
            _ => (),
        }
    }
}

#[skyline::main(name = "latency-slider-de")]
pub fn main() {
    skyline::install_hooks!(
        non_hdr_set_room_id,
        non_hdr_update_room_hook,
        non_hdr_set_online_latency,
        update_css_hook,
        handle_draw_hook
    );
}
