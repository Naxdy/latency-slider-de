use once_cell::sync::OnceCell;

#[macro_export]
macro_rules! ensure_hooks {
    ($($f:expr),*) => {{
        let mut is_successful = true;
        $(
            if $f.get_offset_in_memory().is_none() && is_successful {
                skyline::error::show_error(
                    420,
                    "Latency Slider DE failed to load.\0",
                    format!("Error: Failed to find {} in memory.\n\n{}\n\n{}\n\n{}\0",
                    $f.location_name,
                    "This may be the result of an incompatible mod being loaded, or SSBU being updated.",
                    "If you are unsure, head over to the issues page at\nhttps://github.com/Naxdy/latency-slider-de/issues",
                    "Latency Slider DE will NOT be enabled now, however you can continue playing normally."
                    ).as_str()
                );

                is_successful = false;
            }
        )*

        is_successful
    }};
}

///
/// Searches for a byte pattern in the text region of the process memory, outputs
/// the start address of the first match, or `None` in case of no match.
///
fn byte_search(needle: &[u8]) -> Option<usize> {
    let search_space = unsafe {
        let start = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const u8;
        let end = skyline::hooks::getRegionAddress(skyline::hooks::Region::Rodata) as *const u8;

        let length = end.offset_from(start) as usize;

        std::slice::from_raw_parts(start, length)
    };

    search_space.windows(needle.len()).position(|w| w == needle)
}

pub struct SSBUMemoryLocation<'a> {
    ///
    /// A memory signature used to search for the given location in memory.
    ///
    signature: &'a [u8],

    ///
    /// Offset to the actual location (in bytes) that we are interested in hooking,
    /// relative to the start of the `signature`.
    /// If the first byte of `signature` is the exact location we want, this will be `0`.
    ///
    start_offset: isize,

    ///
    /// A human-readable function / location name, used to display an error message to
    /// the user if the location cannot be found for hooking.
    ///
    pub location_name: &'a str,

    cached_offset: OnceCell<Option<usize>>,
}

impl SSBUMemoryLocation<'_> {
    pub fn get_offset_in_memory(&self) -> Option<usize> {
        *self.cached_offset.get_or_init(|| unsafe {
            let r = byte_search(self.signature)
                .map(|e| (e as *const u8).offset(self.start_offset) as usize);

            if let Some(r) = r {
                println!(
                    "[latency-slider-de] Found {} at {r:#09x?}",
                    self.location_name
                );
            }

            r
        })
    }
}

pub static LOC_SET_ONLINE_LATENCY: SSBUMemoryLocation = SSBUMemoryLocation {
    signature: &[
        0xfd, 0x7b, 0x42, 0xa9, 0xf4, 0x4f, 0x41, 0xa9, 0xe8, 0x07, 0x43, 0xfc, 0xc0, 0x03, 0x5f,
        0xd6, 0x60, 0x1e, 0x44, 0x38, 0xf6, 0xff, 0xff, 0x17,
    ],
    start_offset: 0,
    location_name: "set_online_latency",
    cached_offset: OnceCell::new(),
};

pub static LOC_SET_ROOM_ID: SSBUMemoryLocation = SSBUMemoryLocation {
    signature: &[
        0x81, 0x54, 0x01, 0x90, 0x21, 0x2c, 0x3a, 0x91, 0xe8, 0xc3, 0x00, 0x91, 0xe0, 0x03, 0x14,
        0xaa, 0x20, 0xb9, 0x7b, 0x94, 0x08, 0x18, 0x83, 0x52, 0x03, 0x03, 0x08, 0x8b, 0xa1, 0x57,
        0x01, 0x90, 0x21, 0x3c, 0x16, 0x91, 0xe0, 0xc3, 0x00, 0x91, 0xe2, 0x03, 0x00, 0x32, 0x75,
        0xc7, 0x7b, 0x94, 0xe0, 0x1f, 0x40, 0xf9, 0xa8, 0xcd, 0x01, 0xd0, 0x08, 0xa1, 0x26, 0x91,
        0xe8, 0x7f, 0x03, 0xa9, 0x40, 0x00, 0x00, 0xb4, 0x9f, 0x99, 0x82, 0x94,
    ],
    start_offset: 44,
    location_name: "set_room_id",
    cached_offset: OnceCell::new(),
};

pub static LOC_UPDATE_ROOM: SSBUMemoryLocation = SSBUMemoryLocation {
    signature: &[
        0xff, 0x03, 0x01, 0xd1, 0xf6, 0x57, 0x01, 0xa9, 0xf4, 0x4f, 0x02, 0xa9, 0xfd, 0x7b, 0x03,
        0xa9, 0xfd, 0xc3, 0x00, 0x91, 0x08, 0xcc, 0x41, 0xb9, 0xf3, 0x03, 0x00, 0xaa, 0xa8, 0x04,
        0x00, 0x35, 0x68, 0xf6, 0x40, 0xf9, 0x08, 0x41, 0x40, 0xf9, 0xa9, 0x64, 0x83, 0x52, 0xea,
        0x03, 0x00, 0x32, 0x0a, 0x69, 0x29, 0x38, 0x28, 0xd5, 0x01, 0xf0, 0x08, 0xed, 0x43, 0xf9,
        0xa8, 0x03, 0x00, 0xb4, 0x15, 0x05, 0x40, 0xf9, 0xb4, 0x22, 0x03, 0x91, 0xe0, 0x03, 0x14,
        0xaa,
    ],
    start_offset: 0,
    location_name: "update_room",
    cached_offset: OnceCell::new(),
};

#[cfg(feature = "css_ui")]
pub mod css_ui {
    use once_cell::sync::OnceCell;

    use super::SSBUMemoryLocation;

    pub static LOC_DRAW: SSBUMemoryLocation = SSBUMemoryLocation {
        signature: &[
            0x08, 0x0c, 0x40, 0xf9, 0xc8, 0x03, 0x00, 0xb4, 0xff, 0x83, 0x01, 0xd1, 0xf5, 0x1b,
            0x00, 0xf9, 0xf4, 0x4f, 0x04, 0xa9, 0xfd, 0x7b, 0x05, 0xa9, 0xfd, 0x43, 0x01, 0x91,
            0xf4, 0x03, 0x00, 0xaa,
        ],
        start_offset: 0,
        location_name: "draw",
        cached_offset: OnceCell::new(),
    };

    pub static LOC_INGAME_SCENE: SSBUMemoryLocation = SSBUMemoryLocation {
        signature: &[
            0xff, 0x83, 0x01, 0xd1, 0xfa, 0x67, 0x01, 0xa9, 0xf8, 0x5f, 0x02, 0xa9, 0xf6, 0x57,
            0x03, 0xa9, 0xf4, 0x4f, 0x04, 0xa9, 0xfd, 0x7b, 0x05, 0xa9, 0xfd, 0x43, 0x01, 0x91,
            0xf5, 0x03, 0x01, 0xaa, 0xf3, 0x03, 0x00, 0xaa, 0x16, 0x40, 0x01, 0x91, 0x01, 0x37,
            0x80, 0x52, 0xe0, 0x03, 0x1c, 0x32,
        ],
        start_offset: 48,
        location_name: "ingame_scene",
        cached_offset: OnceCell::new(),
    };

    pub static LOC_ONLINE_MELEE_ANY_SCENE: SSBUMemoryLocation = SSBUMemoryLocation {
        signature: &[
            0xe8, 0x69, 0x01, 0xb0, 0x08, 0x01, 0x00, 0x91, 0x1f, 0x40, 0x00, 0x39, 0x08, 0x7c,
            0x00, 0xa9, 0x1f, 0x40, 0x01, 0x39, 0x1f, 0x54, 0x00, 0xb9, 0x1f, 0x2c, 0x00, 0xf9,
            0xfd, 0x7b, 0x41, 0xa9, 0xff, 0x83, 0x00, 0x91, 0xc0, 0x03, 0x5f, 0xd6,
        ],
        start_offset: 12,
        location_name: "online_melee_any_scene",
        cached_offset: OnceCell::new(),
    };

    pub static LOC_MAIN_MENU_SCENE: SSBUMemoryLocation = SSBUMemoryLocation {
        signature: &[
            0x48, 0x6a, 0x01, 0xb0, 0x08, 0xe1, 0x18, 0x91, 0x7f, 0x42, 0x00, 0x39, 0x68, 0x7e,
            0x00, 0xa9, 0xa8, 0x7e, 0x01, 0xb0, 0x7f, 0x42, 0x01, 0x39, 0xe1, 0x03, 0x00, 0x32,
            0x7f, 0x56, 0x00, 0xb9, 0x7f, 0x2e, 0x00, 0xf9, 0x00, 0xf9, 0x43, 0xf9, 0xc7, 0x43,
            0x3e, 0x94, 0xe0, 0x03, 0x13, 0xaa, 0xfd, 0x7b, 0x42, 0xa9, 0xf3, 0x0b, 0x40, 0xf9,
            0xff, 0xc3, 0x00, 0x91, 0xc0, 0x03, 0x5f, 0xd6,
        ],
        start_offset: 12,
        location_name: "main_menu_scene",
        cached_offset: OnceCell::new(),
    };

    pub static LOC_MELEE_NORMAL_SEQUENCE_SCENE: SSBUMemoryLocation = SSBUMemoryLocation {
        signature: &[
            0xc8, 0x6e, 0x01, 0xf0, 0x08, 0xc1, 0x0b, 0x91, 0x60, 0x42, 0x01, 0x91, 0x7f, 0x42,
            0x00, 0x39, 0x68, 0x7e, 0x00, 0xa9, 0x64, 0x1f, 0x51, 0x94, 0x28, 0x6e, 0x01, 0xd0,
            0x08, 0xc1, 0x01, 0x91, 0xe9, 0x6e, 0x01, 0x90, 0x29, 0xe1, 0x29, 0x91, 0x7f, 0xc2,
            0x01, 0x79, 0x7f, 0xe6, 0x00, 0xb9, 0x68, 0x2a, 0x00, 0xf9, 0x69, 0x02, 0x00, 0xf9,
            0xe0, 0x03, 0x13, 0xaa, 0x7f, 0x76, 0x00, 0xf9, 0xfd, 0x7b, 0x42, 0xa9, 0xf3, 0x0b,
            0x40, 0xf9, 0xff, 0xc3, 0x00, 0x91, 0xc0, 0x03, 0x5f, 0xd6,
        ],
        start_offset: 20,
        location_name: "melee_normal_sequence_scene",
        cached_offset: OnceCell::new(),
    };

    pub static LOC_DISPLAY_CSS: SSBUMemoryLocation = SSBUMemoryLocation {
        signature: &[
            0xa0, 0x83, 0x53, 0xf8, 0x34, 0x00, 0x00, 0x94, 0xff, 0x03, 0x0e, 0x91, 0xe9, 0x23,
            0x43, 0x6d, 0xeb, 0x2b, 0x42, 0x6d, 0xed, 0x33, 0x41, 0x6d, 0xfd, 0x7b, 0x49, 0xa9,
            0xf4, 0x4f, 0x48, 0xa9, 0xf6, 0x57, 0x47, 0xa9, 0xf8, 0x5f, 0x46, 0xa9, 0xfa, 0x67,
            0x45, 0xa9, 0xfc, 0x6f, 0x44, 0xa9, 0xef, 0x3b, 0xca, 0x6c, 0xc0, 0x03, 0x5f, 0xd6,
        ],
        start_offset: 4,
        location_name: "display_css",
        cached_offset: OnceCell::new(),
    };

    pub static LOC_UPDATE_CSS: SSBUMemoryLocation = SSBUMemoryLocation {
        signature: &[
            0xea, 0x0f, 0x18, 0xfc, 0xe9, 0x23, 0x01, 0x6d, 0xfc, 0x6f, 0x02, 0xa9, 0xfa, 0x67,
            0x03, 0xa9, 0xf8, 0x5f, 0x04, 0xa9, 0xf6, 0x57, 0x05, 0xa9, 0xf4, 0x4f, 0x06, 0xa9,
            0xfd, 0x7b, 0x07, 0xa9, 0xfd, 0xc3, 0x01, 0x91, 0xff, 0x83, 0x0b, 0xd1, 0x08, 0x3c,
            0x41, 0xb9, 0xf3, 0x03, 0x00, 0xaa, 0x08, 0x01, 0x00, 0x35, 0x68, 0xce, 0x40, 0xf9,
            0x08, 0x01, 0x40, 0xf9, 0x00, 0x01, 0x40, 0xf9, 0xe1, 0x03, 0x00, 0x32, 0xef, 0x89,
            0x75, 0x94, 0x60, 0xfa, 0x46, 0xf9, 0x1d, 0xf2, 0x62, 0x94,
        ],
        start_offset: 0,
        location_name: "update_css",
        cached_offset: OnceCell::new(),
    };
}

// TODO: not findable atm, probably need to change search algo.
// this and `find_pane_by_name_recursive` are currently the only hardcoded offsets
// pub static LOC_SET_TEXT_STRING: SSBUMemoryLocation = SSBUMemoryLocation {
//     signature: &[
//         0xfc, 0x0f, 0x1d, 0xf8, 0xf4, 0x4f, 0x01, 0xa9, 0xfd, 0x7b, 0x02, 0xa9, 0xfd, 0x83, 0x00,
//         0x91, 0xff, 0x07, 0x40, 0xd1, 0xf4, 0x03, 0x01, 0xaa, 0xf3, 0x03, 0x00, 0xaa, 0xe0, 0x03,
//         0x00, 0x91, 0xe2, 0x03, 0x14, 0x32, 0xe1, 0x03, 0x1f, 0x2a, 0x76, 0x76, 0x08, 0x94, 0xf4,
//         0x04, 0x00, 0xb4, 0x8b, 0x02, 0x40, 0x39, 0xeb, 0x04, 0x00, 0x34, 0xe8, 0x03, 0x00, 0x32,
//     ],
//     start_offset: 0,
//     location_name: "set_text_string",
//     cached_offset: OnceCell::new(),
// };
