use lazy_static::lazy_static;

// Stolen from Ultimate Training Modpack who stole it from HDR who stole it from Arcropolis
// https://github.com/HDR-Development/HewDraw-Remix/blob/dev/dynamic/src/util.rs
// https://github.com/jugeeya/UltimateTrainingModpack/blob/5b0b5490cc30af4964ed7f8d8d1ed8cfe38f6ff1/src/common/offsets.rs
pub fn byte_search<T: Eq>(needle: &[T]) -> Option<usize> {
    let text = unsafe {
        let start = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const T;
        let end = skyline::hooks::getRegionAddress(skyline::hooks::Region::Rodata) as *const T;
        let length = end.offset_from(start) as usize;
        std::slice::from_raw_parts(start, length)
    };

    text.windows(needle.len())
        .position(|window| window == needle)
}

// Wrapper around byte_search() with some additional logging
fn find_offset(name: &str, needle: &[u8]) -> Option<usize> {
    println!("Searching for {}", name);
    let offset_opt = byte_search(needle);
    match offset_opt {
        Some(offset) => {
            println!("Found offset for {} at {:#x}", name, offset);
            Some(offset)
        }
        None => {
            println!("ERROR: Cound not find offset for {}", name);
            None
        }
    }
}

macro_rules! impl_offset {
    ($fn_name:ident) => {
        paste::paste! {
            lazy_static! {
                pub static ref [<OFFSET_ $fn_name>]: usize = find_offset(stringify!($fn_name), [<NEEDLE_ $fn_name>]).expect(stringify!(Failed to find offset for $fn_name));
            }
        }
    }
}

static NEEDLE_DRAW: &[u8] = &[
    0x08, 0x0c, 0x40, 0xf9, // These comments are to prevent rustfmt from destroying
    0xc8, 0x03, 0x00, 0xb4, // this while still allowing rustfmt in general
    0xff, 0x83, 0x01, 0xd1, //
    0xf5, 0x1b, 0x00, 0xf9, //
    0xf4, 0x4f, 0x04, 0xa9, //
    0xfd, 0x7b, 0x05, 0xa9, //
    0xfd, 0x43, 0x01, 0x91, //
    0xf4, 0x03, 0x00, 0xaa,
];
impl_offset!(DRAW);
