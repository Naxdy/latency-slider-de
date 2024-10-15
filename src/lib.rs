#![feature(restricted_std)]

use std::path::Path;

use offsets::{
    LOC_DISPLAY_CSS, LOC_DRAW, LOC_INGAME_SCENE, LOC_MAIN_MENU_SCENE,
    LOC_MELEE_NORMAL_SEQUENCE_SCENE, LOC_ONLINE_MELEE_ANY_SCENE, LOC_SET_ONLINE_LATENCY,
    LOC_SET_ROOM_ID, LOC_UPDATE_CSS, LOC_UPDATE_ROOM,
};
use skyline::hooks::InlineCtx;
use skyline::nn::ui2d::{Layout, Pane};
use smash::ui2d::SmashPane;

mod offsets;

const TRAINING_MODPACK_LOCATION: &str =
    "sd:/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/libtraining_modpack.nro";

const DONTANNOYME_LOCATION: &str = "sd:/dontannoyme.txt";

#[skyline::from_offset(0x37a1f10)]
unsafe fn set_text_string(pane: u64, string: *const u8);

static mut CURRENT_PANE_HANDLE: usize = 0;
static mut CURRENT_ARENA_ID: String = String::new();
static mut CURRENT_INPUT_BUFFER: isize = 4;
static mut MOST_RECENT_AUTO: isize = -1;
static mut STEALTH_MODE: bool = false;
static mut ORIG_VIP_TEXT: String = String::new();
static mut IS_CSS: bool = false;

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

unsafe fn handle_user_input(on_css: bool) {
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

        // kinda hacky for elite smash text
        if on_css {
            IS_CSS = true;
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
    handle_user_input(false);

    if IS_CSS {
        IS_CSS = false;
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

#[skyline::hook(offset = LOC_DRAW.get_offset_in_memory().unwrap())]
unsafe fn handle_draw_hook(layout: *mut Layout, draw_info: u64, cmd_buffer: u64) {
    if IS_CSS {
        let root_pane = &mut *(*layout).root_pane;

        // TODO: Reevaluate functionality after update to 19.0.0
        draw_ui(root_pane);
    }

    call_original!(layout, draw_info, cmd_buffer);
}

#[skyline::hook(offset = LOC_UPDATE_CSS.get_offset_in_memory().unwrap())]
unsafe fn update_css_hook(arg: u64) {
    handle_user_input(true);

    call_original!(arg)
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

#[skyline::hook(offset = LOC_DISPLAY_CSS.get_offset_in_memory().unwrap(), inline)]
unsafe fn display_css_hook(_: &InlineCtx) {
    if !STEALTH_MODE {
        IS_CSS = true;
    }
}

#[skyline::hook(offset = LOC_MELEE_NORMAL_SEQUENCE_SCENE.get_offset_in_memory().unwrap(), inline)]
unsafe fn melee_normal_sequence_scene_hook(_: &InlineCtx) {
    IS_CSS = false;
}

#[skyline::hook(offset = LOC_MAIN_MENU_SCENE.get_offset_in_memory().unwrap(), inline)]
unsafe fn main_menu_scene_hook(_: &InlineCtx) {
    IS_CSS = false;
}

#[skyline::hook(offset = LOC_ONLINE_MELEE_ANY_SCENE.get_offset_in_memory().unwrap(), inline)]
unsafe fn online_melee_any_scene_hook(_: &InlineCtx) {
    IS_CSS = false;
}

#[skyline::hook(offset = LOC_INGAME_SCENE.get_offset_in_memory().unwrap(), inline)]
unsafe fn ingame_scene_hook(_: &InlineCtx) {
    IS_CSS = false;
}

unsafe fn draw_ui(root_pane: &Pane) {
    let vip_pane_00 = root_pane.find_pane_by_name_recursive("txt_vip_title_00");
    let vip_pane_01 = root_pane.find_pane_by_name_recursive("txt_vip_title_01");

    if ORIG_VIP_TEXT.is_empty() {
        match (vip_pane_00, vip_pane_01) {
            (Some(x), _) | (_, Some(x)) => {
                ORIG_VIP_TEXT = dbg!(String::from_utf16(std::slice::from_raw_parts(
                    x.as_textbox().text_buf as *mut u16,
                    x.as_textbox().text_buf_len as usize,
                ))
                .unwrap());
            }
            _ => (),
        }
    } else if let (Some(x), Some(y)) = (vip_pane_00, vip_pane_01) {
        let s = if !STEALTH_MODE {
            if CURRENT_INPUT_BUFFER != -1 {
                format!("Input Latency: {}\0", CURRENT_INPUT_BUFFER)
            } else if MOST_RECENT_AUTO == -1 {
                "Input Latency: Auto\0".to_string()
            } else {
                format!("Input Latency: Auto ({})\0", MOST_RECENT_AUTO)
            }
        } else {
            ORIG_VIP_TEXT.clone()
        };

        [x, y]
            .into_iter()
            .for_each(|e| set_text_string(e as *mut Pane as u64, s.as_str().as_ptr()));
    }
}

#[skyline::main(name = "latency-slider-de")]
pub fn main() {
    // make sure that all hooks are findable
    // and don't crash the game if they're not
    if ensure_hooks!(
        LOC_UPDATE_ROOM,
        LOC_UPDATE_CSS,
        LOC_SET_ROOM_ID,
        LOC_DISPLAY_CSS,
        LOC_SET_ONLINE_LATENCY,
        LOC_INGAME_SCENE,
        LOC_ONLINE_MELEE_ANY_SCENE,
        LOC_MAIN_MENU_SCENE,
        LOC_MELEE_NORMAL_SEQUENCE_SCENE
    ) {
        skyline::install_hooks!(
            non_hdr_set_room_id,
            non_hdr_update_room_hook,
            non_hdr_set_online_latency,
            update_css_hook,
            display_css_hook,
            melee_normal_sequence_scene_hook,
            main_menu_scene_hook,
            online_melee_any_scene_hook,
            ingame_scene_hook
        );
    }

    // only hook draw if training mode modpack doesn't exist
    if Path::new(TRAINING_MODPACK_LOCATION).exists() {
        if !Path::new(DONTANNOYME_LOCATION).exists() {
            skyline::error::show_error(
            69420,
            "Latency Slider DE will run with reduced features.\0",
            format!("{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\0",
            "Latency Slider DE has detected the presence of the Ultimate Training Modpack in your plugins folder.",
            "Due to conflicting functionality, Latency Slider DE will NOT be able to display your desired latency on the quickplay / Elite Smash screen.",
            "The mod will remain fully functional otherwise, and you will still be able to see your desired latency in arenas, which will carry over to all online modes (including quickplay / Elite).",
            "You can also still \"blindly\" adjust your desired latency on any character select screen, even if the latency is not displayed.",
            "If you wish to see your latency on the quickplay / Elite Smash screen again, you will have to (temporarily) remove / disable the Ultimate Training Modpack.",
            "If you don't want to see this message ever again, create an empty file named \"dontannoyme.txt\" and place it in the root (the topmost) folder of your SD card."
            ).as_str()
        );
        } else {
            println!(
                "[latency-slider-de] NOT enabling draw hook; user doesn't wish to be informed."
            )
        }
    } else if ensure_hooks!(LOC_DRAW) {
        skyline::install_hooks!(handle_draw_hook);
    }
}
