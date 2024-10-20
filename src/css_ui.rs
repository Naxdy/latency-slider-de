use std::path::Path;

use skyline::{
    hooks::InlineCtx,
    nn::ui2d::{Layout, Pane},
};
use smash::ui2d::SmashPane;

use crate::{
    ensure_hooks, handle_user_input,
    offsets::css_ui::{
        LOC_DISPLAY_CSS, LOC_DRAW, LOC_INGAME_SCENE, LOC_MAIN_MENU_SCENE,
        LOC_MELEE_NORMAL_SEQUENCE_SCENE, LOC_ONLINE_MELEE_ANY_SCENE, LOC_UPDATE_CSS,
    },
    set_text_string, CURRENT_INPUT_BUFFER, MOST_RECENT_AUTO, STEALTH_MODE,
};

static mut ORIG_VIP_TEXT: String = String::new();
pub static mut IS_CSS: bool = false;

const TRAINING_MODPACK_LOCATION: &str =
    "sd:/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/libtraining_modpack.nro";

const DONTANNOYME_LOCATION: &str = "sd:/dontannoyme.txt";

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

#[skyline::hook(offset = LOC_DISPLAY_CSS.get_offset_in_memory().unwrap(), inline)]
unsafe fn display_css_hook(_: &InlineCtx) {
    use crate::STEALTH_MODE;

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

#[skyline::hook(offset = LOC_DRAW.get_offset_in_memory().unwrap())]
unsafe fn handle_draw_hook(layout: *mut Layout, draw_info: u64, cmd_buffer: u64) {
    if IS_CSS {
        let root_pane = &mut *(*layout).root_pane;

        draw_ui(root_pane);
    }

    call_original!(layout, draw_info, cmd_buffer);
}

#[skyline::hook(offset = LOC_UPDATE_CSS.get_offset_in_memory().unwrap())]
unsafe fn update_css_hook(arg: u64) {
    handle_user_input(true);

    call_original!(arg)
}

pub fn hook_css_funcs() {
    if ensure_hooks!(
        LOC_UPDATE_CSS,
        LOC_DISPLAY_CSS,
        LOC_INGAME_SCENE,
        LOC_ONLINE_MELEE_ANY_SCENE,
        LOC_MAIN_MENU_SCENE,
        LOC_MELEE_NORMAL_SEQUENCE_SCENE
    ) {
        skyline::install_hooks!(
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
            "Latency Slider DE will run with reduced features. View details for more info.\0",
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
