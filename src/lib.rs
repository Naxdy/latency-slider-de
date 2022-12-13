// use skyline::hooks::InlineCtx;

// #[skyline::from_offset(0x37a1270)]
// unsafe fn set_text_string(pane: u64, string: *const u8);

// unsafe fn get_pane_by_name(arg: u64, arg2: *const u8) -> [u64; 4] {
//     let func_addr = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x37752e0);
//     let callable: extern "C" fn(u64, *const u8, ...) -> [u64; 4] = std::mem::transmute(func_addr);
//     callable(arg, arg2)
// }

// unsafe fn set_room_text(arg: u64, string: String) {
//     let func_addr = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x3778c50);
//     let callable: extern "C" fn(u64, *const u8, usize, *const u16, ...) = std::mem::transmute(func_addr);
//     callable(arg, b"mnu_online_room_inside_room_id\0".as_ptr(), 1, string.encode_utf16().collect::<Vec<u16>>().as_ptr())
// }

// static mut CURRENT_PANE_HANDLE: usize = 0;
// static mut CURRENT_ARENA_ID: String = String::new();
// static mut CURRENT_INPUT_BUFFER: isize = 4;
// static mut MOST_RECENT_AUTO: isize = -1;

// const MAX_INPUT_BUFFER: isize = 25;
// const MIN_INPUT_BUFFER: isize = -1;

// #[skyline::hook(offset = 0x1887700, inline)]
// unsafe fn non_hdr_update_room_hook(_: &skyline::hooks::InlineCtx) {
//     static mut CURRENT_COUNTER: usize = 0;
//     if ninput::any::is_press(ninput::Buttons::RIGHT) {
//         if CURRENT_COUNTER == 0 {
//             CURRENT_INPUT_BUFFER += 1;
//         }
//         CURRENT_COUNTER = (CURRENT_COUNTER + 1) % 10;
//     } else if ninput::any::is_press(ninput::Buttons::LEFT) {
//         if CURRENT_COUNTER == 0 {
//             CURRENT_INPUT_BUFFER -= 1;
//         }
//         CURRENT_COUNTER = (CURRENT_COUNTER + 1) % 10;
//     } else {
//         CURRENT_COUNTER = 0;
//     }

//     CURRENT_INPUT_BUFFER = CURRENT_INPUT_BUFFER.clamp(MIN_INPUT_BUFFER, MAX_INPUT_BUFFER);
//     if CURRENT_INPUT_BUFFER == -1 {
//         if MOST_RECENT_AUTO == -1 {
//             set_text_string(
//                 CURRENT_PANE_HANDLE as u64,
//                 format!("ROOM ID: {}\nInput Latency: Auto", CURRENT_ARENA_ID).as_ptr(),
//             );
//         } else {
//             set_text_string(
//                 CURRENT_PANE_HANDLE as u64,
//                 format!("ROOM ID: {}\nInput Latency: Auto ({})", CURRENT_ARENA_ID, MOST_RECENT_AUTO).as_ptr()
//             )
//         }
//     } else {
//         set_text_string(CURRENT_PANE_HANDLE as u64, format!("{}\nInput Latency: {}\0", CURRENT_ARENA_ID, CURRENT_INPUT_BUFFER).as_ptr());
//     }
// }

// #[skyline::hook(offset = 0x188702c, inline)]
// unsafe fn non_hdr_set_room_id(ctx: &skyline::hooks::InlineCtx) {
//     let panel = *((*((*ctx.registers[0].x.as_ref() + 8) as *const u64) + 0x10) as *const u64);
//     CURRENT_PANE_HANDLE = panel as usize;
//     CURRENT_ARENA_ID = dbg!(String::from_utf16(std::slice::from_raw_parts(*ctx.registers[3].x.as_ref() as *const u16, 5)).unwrap());
// }

// static mut PANE: u64 = 0;

// #[skyline::hook(offset = 0x1a12460)]
// unsafe fn non_hdr_update_css2(arg: u64) {
//     static mut CURRENT_COUNTER: usize = 0;
//     if ninput::any::is_press(ninput::Buttons::X) {
//         if CURRENT_COUNTER == 0 {
//             CURRENT_INPUT_BUFFER += 1;
//         }
//         CURRENT_COUNTER = (CURRENT_COUNTER + 1) % 10;
//     } else if ninput::any::is_press(ninput::Buttons::Y) {
//         if CURRENT_COUNTER == 0 {
//             CURRENT_INPUT_BUFFER -= 1;
//         }
//         CURRENT_COUNTER = (CURRENT_COUNTER + 1) % 10;
//     } else {
//         CURRENT_COUNTER = 0;
//     }

//     CURRENT_INPUT_BUFFER = CURRENT_INPUT_BUFFER.clamp(MIN_INPUT_BUFFER, MAX_INPUT_BUFFER);
//     set_text_string(*((*((arg + 0xe58) as *const u64) + 0x10) as *const u64), format!("Input Latency: {}\0", CURRENT_INPUT_BUFFER).as_ptr());
//     set_text_string(*((*((arg + 0xe68) as *const u64) + 0x10) as *const u64), format!("Input Latency: {}\0", CURRENT_INPUT_BUFFER).as_ptr());
//     call_original!(arg)
// }

// static mut IS_USABLE: bool = false;

// #[skyline::hook(offset = 0x16cdb08, inline)]
// unsafe fn non_hdr_set_online_latency(ctx: &InlineCtx) {
//     let auto = *(*ctx.registers[19].x.as_ref() as *mut u8);
//     if IS_USABLE {
//         MOST_RECENT_AUTO = auto as isize;
//         if CURRENT_INPUT_BUFFER != -1 {
//             *(*ctx.registers[19].x.as_ref() as *mut u8) = CURRENT_INPUT_BUFFER as u8;
//         }
//     }
// }

// #[skyline::hook(offset = 0x22d91f4, inline)]
// unsafe fn online_melee_any_scene_create(_: &InlineCtx) {
//     IS_USABLE = false;
// }

// #[skyline::hook(offset = 0x22d9124, inline)]
// unsafe fn bg_matchmaking_seq(_: &InlineCtx) {
//     IS_USABLE = false;
// }

// #[skyline::hook(offset = 0x22d9054, inline)]
// unsafe fn arena_seq(_: &InlineCtx) {
//     IS_USABLE = true;
// }

// #[skyline::hook(offset = 0x23599b0, inline)]
// unsafe fn main_menu(_: &InlineCtx) {
//     IS_USABLE = false;
// }

// extern "C" {
//     fn update_room_hook();
// }

// #[skyline::main(name = "arena-latency-slider")]
// pub fn main() {
//     if unsafe { (update_room_hook as *const ()).is_null() } {
//         skyline::install_hooks!(non_hdr_set_room_id, non_hdr_update_room_hook, non_hdr_set_online_latency, online_melee_any_scene_create, bg_matchmaking_seq, arena_seq, main_menu);
//     }
// }

std::arch::global_asm!(r#"
.text
.def	@feat.00;
.scl	3;
.type	0;
.endef
.globl	@feat.00
.set @feat.00, 0
.file	"4bu6oet6mwhg1uu1"
.def	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h9a28d8fc34dcbba9E;
.scl	3;
.type	32;
.endef
.section	.text,"xr",one_only,_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h9a28d8fc34dcbba9E
.p2align	4, 0x90
_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h9a28d8fc34dcbba9E:
.Lfunc_begin0:
.cv_func_id 0
.cv_file	1 "/rustc/7eef946fc0e0eff40e588eab77b09b287accbec3\\library\\core\\src\\fmt\\mod.rs" "04BE1DED0EE4855FB268BF0A3873BD2DC8E6B351" 2
.cv_loc	0 1 2373 0
movq	%rdx, %r8
.Ltmp0:
movq	(%rcx), %rax
.Ltmp1:
.cv_file	2 "/rustc/7eef946fc0e0eff40e588eab77b09b287accbec3\\library\\alloc\\src\\raw_vec.rs" "4BD4DCBAD924653D81DCFB3CE21035DBF43F7FA0" 2
.cv_inline_site_id 1 within 0 inlined_at 1 2373 0
.cv_file	3 "/rustc/7eef946fc0e0eff40e588eab77b09b287accbec3\\library\\alloc\\src\\string.rs" "2691127EC288C540ED3F44447ADB8F67AD9CFA8F" 2
.cv_inline_site_id 2 within 1 inlined_at 3 2283 0
.cv_inline_site_id 3 within 2 inlined_at 3 2460 0
.cv_file	4 "/rustc/7eef946fc0e0eff40e588eab77b09b287accbec3\\library\\alloc\\src\\vec\\mod.rs" "82D9359E8BF399C45AF7A5E366D5D132CAED5FC7" 2
.cv_inline_site_id 4 within 3 inlined_at 4 2641 0
.cv_inline_site_id 5 within 4 inlined_at 4 1237 0
.cv_loc	5 2 224 0
movq	(%rax), %rcx
.Ltmp2:
.cv_loc	3 4 2641 0
movq	16(%rax), %rdx
.Ltmp3:
.cv_loc	1 3 2283 0
jmp	_ZN40_$LT$str$u20$as$u20$core..fmt..Debug$GT$3fmt17h3f4c7f2eaa722ae6E
.Ltmp4:
.Lfunc_end0:

.def	_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hf6378e006fcd7e96E;
.scl	3;
.type	32;
.endef
.section	.text,"xr",one_only,_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hf6378e006fcd7e96E
.p2align	4, 0x90
_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hf6378e006fcd7e96E:
.Lfunc_begin1:
.cv_func_id 6
.cv_loc	6 1 2373 0
movq	%rdx, %r8
.Ltmp5:
movq	(%rcx), %rax
movq	8(%rcx), %rdx
.Ltmp6:
movq	%rax, %rcx
.Ltmp7:
jmp	_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17h81cb3c15bb17a55eE
.Ltmp8:
.Lfunc_end1:

.def	_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h2571fb4fd1fdb246E;
.scl	3;
.type	32;
.endef
.section	.text,"xr",one_only,_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h2571fb4fd1fdb246E
.p2align	4, 0x90
_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h2571fb4fd1fdb246E:
.Lfunc_begin2:
.cv_func_id 7
.cv_file	5 "/rustc/7eef946fc0e0eff40e588eab77b09b287accbec3\\library\\core\\src\\ptr\\mod.rs" "915A2787D4B080290F8FC1BDA15EFBDA3325D28E" 2
.cv_inline_site_id 8 within 7 inlined_at 5 490 0
.cv_inline_site_id 9 within 8 inlined_at 5 490 0
.cv_inline_site_id 10 within 9 inlined_at 5 490 0
.cv_inline_site_id 11 within 10 inlined_at 2 478 0
.cv_loc	11 2 241 0
movq	8(%rcx), %rdx
testq	%rdx, %rdx
je	.LBB2_1
.Ltmp9:
.cv_file	6 "/rustc/7eef946fc0e0eff40e588eab77b09b287accbec3\\library\\core\\src\\alloc\\layout.rs" "D948BB75B6A59615709415F37CE2AD3FB6E56727" 2
.cv_inline_site_id 12 within 11 inlined_at 2 247 0
.cv_inline_site_id 13 within 12 inlined_at 6 428 0
.cv_loc	13 6 438 0
movq	%rdx, %r8
notq	%r8
shrq	$63, %r8
.Ltmp10:
.cv_loc	11 2 248 0
movq	(%rcx), %rcx
.Ltmp11:
.cv_file	7 "/rustc/7eef946fc0e0eff40e588eab77b09b287accbec3\\library\\alloc\\src\\alloc.rs" "297D9A62799F00C2B15E0BD660026207F247E838" 2
.cv_inline_site_id 14 within 10 inlined_at 2 479 0
.cv_inline_site_id 15 within 14 inlined_at 7 254 0
.cv_loc	15 7 117 0
jmp	__rust_dealloc
.Ltmp12:
.LBB2_1:
.cv_loc	7 5 490 0
retq
.Ltmp13:
.Lfunc_end2:

.def	_ZN4core3ptr50drop_in_place$LT$alloc..string..FromUtf16Error$GT$17h53c7078b33f3951aE;
.scl	3;
.type	32;
.endef
.section	.text,"xr",one_only,_ZN4core3ptr50drop_in_place$LT$alloc..string..FromUtf16Error$GT$17h53c7078b33f3951aE
.p2align	4, 0x90
_ZN4core3ptr50drop_in_place$LT$alloc..string..FromUtf16Error$GT$17h53c7078b33f3951aE:
.Lfunc_begin3:
.cv_func_id 16
.cv_loc	16 5 490 0
retq
.Ltmp14:
.Lfunc_end3:

.def	_ZN5alloc3fmt6format17h96a775549426137dE;
.scl	3;
.type	32;
.endef
.section	.text,"xr",one_only,_ZN5alloc3fmt6format17h96a775549426137dE
.p2align	4, 0x90
_ZN5alloc3fmt6format17h96a775549426137dE:
.Lfunc_begin4:
.cv_func_id 17
.cv_file	8 "/rustc/7eef946fc0e0eff40e588eab77b09b287accbec3\\library\\alloc\\src\\fmt.rs" "21CF05066DEBDA5469D34A5028210557D3D200B8" 2
.cv_loc	17 8 608 0
.seh_proc _ZN5alloc3fmt6format17h96a775549426137dE
pushq	%r14
.seh_pushreg %r14
pushq	%rsi
.seh_pushreg %rsi
pushq	%rdi
.seh_pushreg %rdi
pushq	%rbx
.seh_pushreg %rbx
subq	$88, %rsp
.seh_stackalloc 88
.seh_endprologue
movq	%rcx, %rsi
.Ltmp15:
.cv_inline_site_id 18 within 17 inlined_at 8 616 0
.cv_loc	18 1 517 0
movq	8(%rdx), %rcx
movq	40(%rdx), %rax
.cv_loc	18 1 518 0
cmpq	$1, %rcx
je	.LBB4_4
.Ltmp16:
testq	%rcx, %rcx
jne	.LBB4_5
.Ltmp17:
testq	%rax, %rax
jne	.LBB4_5
.Ltmp18:
movl	$1, %ebx
leaq	__unnamed_1(%rip), %r14
jmp	.LBB4_11
.Ltmp19:
.LBB4_4:
.cv_loc	18 1 519 0
testq	%rax, %rax
je	.LBB4_6
.Ltmp20:
.LBB4_5:
.cv_inline_site_id 19 within 17 inlined_at 8 616 0
.cv_file	9 "/rustc/7eef946fc0e0eff40e588eab77b09b287accbec3\\library\\core\\src\\option.rs" "E1E7EE049774EABDA40D9EC456B40CBC18A33322" 2
.cv_inline_site_id 20 within 19 inlined_at 9 1019 0
.cv_loc	20 8 616 0
movups	(%rdx), %xmm0
movups	16(%rdx), %xmm1
movups	32(%rdx), %xmm2
movaps	%xmm2, 64(%rsp)
movaps	%xmm1, 48(%rsp)
movaps	%xmm0, 32(%rsp)
leaq	32(%rsp), %rdx
.Ltmp21:
movq	%rsi, %rcx
callq	_ZN5alloc3fmt6format12format_inner17h56b631a31dae1e60E
jmp	.LBB4_13
.Ltmp22:
.LBB4_6:
.cv_loc	18 1 519 0
movq	(%rdx), %rax
.Ltmp23:
movq	(%rax), %r14
movq	8(%rax), %rdi
.Ltmp24:
.cv_inline_site_id 21 within 19 inlined_at 9 1018 0
.cv_file	10 "/rustc/7eef946fc0e0eff40e588eab77b09b287accbec3\\library\\core\\src\\ops\\function.rs" "4E475DAD0DB467D3FB8373803220ACA147B5D950" 2
.cv_inline_site_id 22 within 21 inlined_at 10 510 0
.cv_file	11 "/rustc/7eef946fc0e0eff40e588eab77b09b287accbec3\\library\\alloc\\src\\str.rs" "09543A411F65DFB52ADDD2D656BA69400990864C" 2
.cv_inline_site_id 23 within 22 inlined_at 11 209 0
.cv_file	12 "/rustc/7eef946fc0e0eff40e588eab77b09b287accbec3\\library\\alloc\\src\\slice.rs" "8E85AA4650AC9C1A17506A714F3535F74714D563" 2
.cv_inline_site_id 24 within 23 inlined_at 12 786 0
.cv_inline_site_id 25 within 24 inlined_at 12 411 0
.cv_inline_site_id 26 within 25 inlined_at 12 436 0
.cv_inline_site_id 27 within 26 inlined_at 12 106 0
.cv_inline_site_id 28 within 27 inlined_at 12 157 0
.cv_inline_site_id 29 within 28 inlined_at 4 673 0
.cv_inline_site_id 30 within 29 inlined_at 2 131 0
.cv_loc	30 2 171 0
testq	%rdi, %rdi
je	.LBB4_10
.Ltmp25:
.cv_loc	30 2 176 0
js	.LBB4_14
.Ltmp26:
.cv_inline_site_id 31 within 30 inlined_at 2 185 0
.cv_inline_site_id 32 within 31 inlined_at 7 241 0
.cv_inline_site_id 33 within 32 inlined_at 7 181 0
.cv_loc	33 7 99 0
movl	$1, %edx
.Ltmp27:
movq	%rdi, %rcx
callq	__rust_alloc
.Ltmp28:
.cv_loc	30 2 188 0
testq	%rax, %rax
.Ltmp29:
je	.LBB4_15
.Ltmp30:
movq	%rax, %rbx
jmp	.LBB4_12
.Ltmp31:
.LBB4_10:
movl	$1, %ebx
.Ltmp32:
.LBB4_11:
xorl	%edi, %edi
.Ltmp33:
.LBB4_12:
.cv_file	13 "/rustc/7eef946fc0e0eff40e588eab77b09b287accbec3\\library\\core\\src\\intrinsics.rs" "3851DB81C4CD34DCD514C9EBD20F33737A0D625F" 2
.cv_inline_site_id 34 within 27 inlined_at 12 162 0
.cv_file	14 "/rustc/7eef946fc0e0eff40e588eab77b09b287accbec3\\library\\core\\src\\ptr\\const_ptr.rs" "B3C5B6FD7E473E096AA65EC74D4255A7BD5C2079" 2
.cv_inline_site_id 35 within 34 inlined_at 14 1264 0
.cv_loc	35 13 2429 0
movq	%rbx, %rcx
movq	%r14, %rdx
movq	%rdi, %r8
callq	memcpy
.Ltmp34:
.cv_inline_site_id 36 within 22 inlined_at 11 209 0
.cv_loc	36 3 846 0
movq	%rbx, (%rsi)
movq	%rdi, 8(%rsi)
movq	%rdi, 16(%rsi)
.Ltmp35:
.LBB4_13:
.cv_loc	17 8 617 0
movq	%rsi, %rax
addq	$88, %rsp
popq	%rbx
popq	%rdi
popq	%rsi
popq	%r14
retq
.Ltmp36:
.LBB4_14:
.cv_loc	30 2 178 0
callq	_ZN5alloc7raw_vec17capacity_overflow17h7bf6bddb54e60ed2E
.Ltmp37:
ud2
.Ltmp38:
.LBB4_15:
.cv_loc	30 2 190 0
movl	$1, %edx
movq	%rdi, %rcx
callq	_ZN5alloc5alloc18handle_alloc_error17h62e6b21eb2b33df0E
.Ltmp39:
ud2
.Ltmp40:
.Lfunc_end4:
.seh_endproc

.def	_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17hd07e1a8f740041e6E;
.scl	3;
.type	32;
.endef
.section	.text,"xr",one_only,_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17hd07e1a8f740041e6E
.p2align	4, 0x90
_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17hd07e1a8f740041e6E:
.Lfunc_begin5:
.cv_func_id 37
.cv_loc	37 3 2274 0
.cv_inline_site_id 38 within 37 inlined_at 3 2275 0
.cv_inline_site_id 39 within 38 inlined_at 3 2460 0
.cv_inline_site_id 40 within 39 inlined_at 4 2641 0
.cv_inline_site_id 41 within 40 inlined_at 4 1237 0
.cv_loc	41 2 224 0
movq	%rdx, %r8
.Ltmp41:
movq	(%rcx), %rax
.Ltmp42:
.cv_loc	39 4 2641 0
movq	16(%rcx), %rdx
.Ltmp43:
.cv_loc	37 3 2275 0
movq	%rax, %rcx
.Ltmp44:
jmp	_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17h81cb3c15bb17a55eE
.Ltmp45:
.Lfunc_end5:

.def	_ZN20arena_latency_slider15set_text_string17hc4197c5838b7032fE;
.scl	3;
.type	32;
.endef
.section	.text,"xr",one_only,_ZN20arena_latency_slider15set_text_string17hc4197c5838b7032fE
.p2align	4, 0x90
_ZN20arena_latency_slider15set_text_string17hc4197c5838b7032fE:
.Lfunc_begin6:
.cv_func_id 42
.cv_file	15 "C:\\Users\\blujay\\Documents\\Development\\arena-latency-slider\\src\\lib.rs" "0AEE9BD85D3FB6DE2F8233EEF529F6D1726D0EAA" 2
.cv_loc	42 15 3 0
.seh_proc _ZN20arena_latency_slider15set_text_string17hc4197c5838b7032fE
pushq	%rsi
.seh_pushreg %rsi
pushq	%rdi
.seh_pushreg %rdi
subq	$40, %rsp
.seh_stackalloc 40
.seh_endprologue
movq	%rdx, %rsi
movq	%rcx, %rdi
.Ltmp46:
xorl	%ecx, %ecx
.Ltmp47:
callq	getRegionAddress
.Ltmp48:
.cv_inline_site_id 43 within 42 inlined_at 15 3 0
.cv_loc	43 14 461 0
addq	$58331760, %rax
.Ltmp49:
.cv_loc	42 15 3 0
movq	%rdi, %rcx
movq	%rsi, %rdx
addq	$40, %rsp
popq	%rdi
.Ltmp50:
popq	%rsi
.Ltmp51:
rex64 jmpq	*%rax
.Ltmp52:
.Lfunc_end6:
.seh_endproc

.def	non_hdr_update_room_hook;
.scl	2;
.type	32;
.endef
.section	.text,"xr",one_only,non_hdr_update_room_hook
.globl	non_hdr_update_room_hook
.p2align	4, 0x90
non_hdr_update_room_hook:
.Lfunc_begin7:
.cv_func_id 44
.cv_loc	44 15 27 0
.seh_proc non_hdr_update_room_hook
pushq	%rsi
.seh_pushreg %rsi
pushq	%rdi
.seh_pushreg %rdi
pushq	%rbx
.seh_pushreg %rbx
subq	$144, %rsp
.seh_stackalloc 144
.seh_endprologue
.cv_loc	44 15 29 0
movl	$16384, %ecx
.Ltmp53:
callq	_ZN6ninput3any8is_press17h6ad3afc071954a5aE
testb	%al, %al
je	.LBB7_1
.Ltmp54:
.cv_loc	44 15 30 0
movq	_ZN20arena_latency_slider24non_hdr_update_room_hook15CURRENT_COUNTER17h701e90c953f8ab41E.0(%rip), %rcx
testq	%rcx, %rcx
jne	.LBB7_5
.Ltmp55:
.cv_loc	44 15 31 0
incq	_ZN20arena_latency_slider20CURRENT_INPUT_BUFFER17hcd1c017fa6563844E(%rip)
jmp	.LBB7_5
.Ltmp56:
.LBB7_1:
.cv_loc	44 15 34 0
movl	$4096, %ecx
callq	_ZN6ninput3any8is_press17h6ad3afc071954a5aE
testb	%al, %al
je	.LBB7_2
.Ltmp57:
.cv_loc	44 15 35 0
movq	_ZN20arena_latency_slider24non_hdr_update_room_hook15CURRENT_COUNTER17h701e90c953f8ab41E.0(%rip), %rcx
testq	%rcx, %rcx
jne	.LBB7_5
.Ltmp58:
.cv_loc	44 15 36 0
decq	_ZN20arena_latency_slider20CURRENT_INPUT_BUFFER17hcd1c017fa6563844E(%rip)
.Ltmp59:
.LBB7_5:
incq	%rcx
movabsq	$-3689348814741910323, %rdx
movq	%rcx, %rax
mulq	%rdx
shrq	$2, %rdx
andq	$-2, %rdx
leaq	(%rdx,%rdx,4), %rax
subq	%rax, %rcx
jmp	.LBB7_6
.Ltmp60:
.LBB7_2:
xorl	%ecx, %ecx
.Ltmp61:
.LBB7_6:
.cv_loc	44 15 43 0
movq	%rcx, _ZN20arena_latency_slider24non_hdr_update_room_hook15CURRENT_COUNTER17h701e90c953f8ab41E.0(%rip)
movq	_ZN20arena_latency_slider20CURRENT_INPUT_BUFFER17hcd1c017fa6563844E(%rip), %rax
.Ltmp62:
cmpq	$25, %rax
movl	$25, %ecx
cmovlq	%rax, %rcx
movq	$-1, %rax
.Ltmp63:
.cv_file	16 "/rustc/7eef946fc0e0eff40e588eab77b09b287accbec3\\library\\core\\src\\cmp.rs" "203F09B80BB70254523209A3310BAC5FC58C6DFA" 2
.cv_inline_site_id 45 within 44 inlined_at 15 43 0
.cv_loc	45 16 861 0
testq	%rcx, %rcx
cmovnsq	%rcx, %rax
.Ltmp64:
.cv_loc	44 15 43 0
movq	%rax, _ZN20arena_latency_slider20CURRENT_INPUT_BUFFER17hcd1c017fa6563844E(%rip)
.cv_loc	44 15 44 0
js	.LBB7_7
.Ltmp65:
.cv_loc	44 15 57 0
movq	_ZN20arena_latency_slider19CURRENT_PANE_HANDLE17h2bb43f3630faa000E.0(%rip), %rsi
leaq	_ZN20arena_latency_slider16CURRENT_ARENA_ID17h90041e33cf1e16dfE(%rip), %rax
movq	%rax, 40(%rsp)
leaq	_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17hd07e1a8f740041e6E(%rip), %rax
movq	%rax, 48(%rsp)
leaq	_ZN20arena_latency_slider20CURRENT_INPUT_BUFFER17hcd1c017fa6563844E(%rip), %rax
movq	%rax, 56(%rsp)
leaq	_ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$isize$GT$3fmt17ha46d39a4aa14b182E(%rip), %rax
movq	%rax, 64(%rsp)
.Ltmp66:
.cv_inline_site_id 46 within 44 inlined_at 15 57 0
.cv_inline_site_id 47 within 46 inlined_at 8 616 0
.cv_inline_site_id 48 within 47 inlined_at 9 1019 0
.cv_loc	48 8 616 0
leaq	__unnamed_2(%rip), %rax
.Ltmp67:
jmp	.LBB7_12
.Ltmp68:
.LBB7_7:
.cv_loc	44 15 45 0
cmpq	$-1, _ZN20arena_latency_slider16MOST_RECENT_AUTO17hb25b258bf2ddaad6E(%rip)
movq	_ZN20arena_latency_slider19CURRENT_PANE_HANDLE17h2bb43f3630faa000E.0(%rip), %rsi
.cv_loc	44 15 53 0
leaq	_ZN20arena_latency_slider16CURRENT_ARENA_ID17h90041e33cf1e16dfE(%rip), %rax
.cv_loc	44 15 45 0
je	.LBB7_8
.Ltmp69:
.cv_loc	44 15 53 0
movq	%rax, 40(%rsp)
leaq	_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17hd07e1a8f740041e6E(%rip), %rax
movq	%rax, 48(%rsp)
leaq	_ZN20arena_latency_slider16MOST_RECENT_AUTO17hb25b258bf2ddaad6E(%rip), %rax
movq	%rax, 56(%rsp)
leaq	_ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$isize$GT$3fmt17ha46d39a4aa14b182E(%rip), %rax
movq	%rax, 64(%rsp)
.Ltmp70:
.cv_inline_site_id 49 within 44 inlined_at 15 53 0
.cv_inline_site_id 50 within 49 inlined_at 8 616 0
.cv_inline_site_id 51 within 50 inlined_at 9 1019 0
.cv_loc	51 8 616 0
leaq	__unnamed_3(%rip), %rax
.Ltmp71:
.LBB7_12:
movq	%rax, 96(%rsp)
movq	$3, 104(%rsp)
movq	$0, 112(%rsp)
leaq	40(%rsp), %rax
movq	%rax, 128(%rsp)
movq	$2, 136(%rsp)
leaq	72(%rsp), %rcx
leaq	96(%rsp), %rdx
callq	_ZN5alloc3fmt6format12format_inner17h56b631a31dae1e60E
movq	72(%rsp), %rdi
movq	80(%rsp), %rbx
.Ltmp72:
.LBB7_13:
xorl	%ecx, %ecx
callq	getRegionAddress
.Ltmp73:
addq	$58331760, %rax
movq	%rsi, %rcx
movq	%rdi, %rdx
callq	*%rax
.Ltmp74:
testq	%rbx, %rbx
je	.LBB7_15
.Ltmp75:
movq	%rbx, %r8
notq	%r8
shrq	$63, %r8
.Ltmp76:
movq	%rdi, %rcx
movq	%rbx, %rdx
callq	__rust_dealloc
.Ltmp77:
.LBB7_15:
.cv_loc	44 15 59 0
nop
addq	$144, %rsp
popq	%rbx
popq	%rdi
popq	%rsi
retq
.Ltmp78:
.LBB7_8:
.cv_loc	44 15 48 0
movq	%rax, 72(%rsp)
leaq	_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17hd07e1a8f740041e6E(%rip), %rax
movq	%rax, 80(%rsp)
.Ltmp79:
.cv_inline_site_id 52 within 44 inlined_at 15 48 0
.cv_inline_site_id 53 within 52 inlined_at 8 616 0
.cv_inline_site_id 54 within 53 inlined_at 9 1019 0
.cv_loc	54 8 616 0
leaq	__unnamed_4(%rip), %rax
.Ltmp80:
movq	%rax, 96(%rsp)
movq	$2, 104(%rsp)
movq	$0, 112(%rsp)
leaq	72(%rsp), %rax
.Ltmp81:
movq	%rax, 128(%rsp)
movq	$1, 136(%rsp)
leaq	40(%rsp), %rcx
leaq	96(%rsp), %rdx
callq	_ZN5alloc3fmt6format12format_inner17h56b631a31dae1e60E
.Ltmp82:
.cv_loc	44 15 48 0
movq	40(%rsp), %rdi
movq	48(%rsp), %rbx
jmp	.LBB7_13
.Ltmp83:
.Lfunc_end7:
.seh_endproc

.def	non_hdr_set_room_id;
.scl	2;
.type	32;
.endef
.section	.text,"xr",one_only,non_hdr_set_room_id
.globl	non_hdr_set_room_id
.p2align	4, 0x90
non_hdr_set_room_id:
.Lfunc_begin8:
.cv_func_id 55
.cv_loc	55 15 62 0
.seh_proc non_hdr_set_room_id
pushq	%rsi
.seh_pushreg %rsi
subq	$240, %rsp
.seh_stackalloc 240
.seh_endprologue
.cv_loc	55 15 63 0
movq	(%rcx), %rax
movq	8(%rax), %rax
movq	16(%rax), %rax
.Ltmp84:
.cv_loc	55 15 64 0
movq	%rax, _ZN20arena_latency_slider19CURRENT_PANE_HANDLE17h2bb43f3630faa000E.0(%rip)
.cv_loc	55 15 65 0
movq	24(%rcx), %rdx
leaq	48(%rsp), %rsi
movl	$5, %r8d
movq	%rsi, %rcx
.Ltmp85:
callq	_ZN5alloc6string6String10from_utf1617h640651b6adfd3677E
.Ltmp86:
.cv_file	17 "/rustc/7eef946fc0e0eff40e588eab77b09b287accbec3\\library\\core\\src\\result.rs" "D1647999EA10DAE04C462A739945296947C79CD4" 2
.cv_inline_site_id 56 within 55 inlined_at 15 65 0
.cv_loc	56 17 1111 0
cmpq	$0, 48(%rsp)
je	.LBB8_4
.Ltmp87:
.cv_loc	56 17 1112 0
movq	64(%rsp), %rax
movq	%rax, 224(%rsp)
movups	48(%rsp), %xmm0
movaps	%xmm0, 208(%rsp)
.Ltmp88:
.cv_loc	55 15 65 0
movq	%rax, 144(%rsp)
movaps	%xmm0, 128(%rsp)
leaq	128(%rsp), %rax
.Ltmp89:
.cv_loc	55 15 65 0
movq	%rax, 120(%rsp)
.Ltmp90:
.cv_loc	55 15 65 0
leaq	__unnamed_5(%rip), %rax
.Ltmp91:
movq	%rax, 48(%rsp)
leaq	_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hf6378e006fcd7e96E(%rip), %rax
.Ltmp92:
movq	%rax, 56(%rsp)
leaq	__unnamed_6(%rip), %rcx
.Ltmp93:
movq	%rcx, 64(%rsp)
leaq	_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h884ab426c1b89a2bE(%rip), %rcx
.Ltmp94:
movq	%rcx, 72(%rsp)
leaq	__unnamed_7(%rip), %rcx
.Ltmp95:
movq	%rcx, 80(%rsp)
movq	%rax, 88(%rsp)
leaq	120(%rsp), %rax
movq	%rax, 96(%rsp)
leaq	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h9a28d8fc34dcbba9E(%rip), %rax
movq	%rax, 104(%rsp)
.Ltmp96:
.cv_inline_site_id 57 within 55 inlined_at 15 65 0
.cv_loc	57 1 420 0
leaq	__unnamed_8(%rip), %rax
.Ltmp97:
movq	%rax, 160(%rsp)
movq	$5, 168(%rsp)
leaq	__unnamed_9(%rip), %rax
.Ltmp98:
movq	%rax, 176(%rsp)
movq	$4, 184(%rsp)
movq	%rsi, 192(%rsp)
movq	$4, 200(%rsp)
leaq	160(%rsp), %rcx
.Ltmp99:
.cv_loc	55 15 65 0
callq	_ZN3std2io5stdio7_eprint17hbba5eb7776851f0aE
.Ltmp100:
movaps	128(%rsp), %xmm0
movaps	%xmm0, 48(%rsp)
movq	144(%rsp), %rax
movq	%rax, 64(%rsp)
.Ltmp101:
.cv_inline_site_id 58 within 55 inlined_at 15 65 0
.cv_inline_site_id 59 within 58 inlined_at 5 490 0
.cv_inline_site_id 60 within 59 inlined_at 5 490 0
.cv_inline_site_id 61 within 60 inlined_at 5 490 0
.cv_inline_site_id 62 within 61 inlined_at 2 478 0
.cv_loc	62 2 241 0
movq	_ZN20arena_latency_slider16CURRENT_ARENA_ID17h90041e33cf1e16dfE+8(%rip), %rdx
.Ltmp102:
testq	%rdx, %rdx
je	.LBB8_3
.Ltmp103:
.cv_inline_site_id 63 within 62 inlined_at 2 247 0
.cv_inline_site_id 64 within 63 inlined_at 6 428 0
.cv_loc	64 6 438 0
movq	%rdx, %r8
notq	%r8
shrq	$63, %r8
.Ltmp104:
.cv_loc	62 2 248 0
movq	_ZN20arena_latency_slider16CURRENT_ARENA_ID17h90041e33cf1e16dfE(%rip), %rcx
.Ltmp105:
.cv_inline_site_id 65 within 61 inlined_at 2 479 0
.cv_inline_site_id 66 within 65 inlined_at 7 254 0
.cv_loc	66 7 117 0
callq	__rust_dealloc
.Ltmp106:
.LBB8_3:
.cv_loc	55 15 65 0
movq	64(%rsp), %rax
movq	%rax, _ZN20arena_latency_slider16CURRENT_ARENA_ID17h90041e33cf1e16dfE+16(%rip)
movaps	48(%rsp), %xmm0
movups	%xmm0, _ZN20arena_latency_slider16CURRENT_ARENA_ID17h90041e33cf1e16dfE(%rip)
.Ltmp107:
.cv_loc	55 15 66 0
addq	$240, %rsp
popq	%rsi
retq
.Ltmp108:
.LBB8_4:
.cv_loc	56 17 1113 0
leaq	__unnamed_10(%rip), %rax
movq	%rax, 32(%rsp)
leaq	__unnamed_11(%rip), %rcx
leaq	__unnamed_12(%rip), %r9
leaq	160(%rsp), %r8
movl	$43, %edx
callq	_ZN4core6result13unwrap_failed17hda7e378885ba4782E
ud2
.Ltmp109:
.Lfunc_end8:
.seh_endproc

.def	non_hdr_update_css2;
.scl	2;
.type	32;
.endef
.section	.text,"xr",one_only,non_hdr_update_css2
.globl	non_hdr_update_css2
.p2align	4, 0x90
non_hdr_update_css2:
.Lfunc_begin9:
.cv_func_id 67
.cv_loc	67 15 71 0
.seh_proc non_hdr_update_css2
pushq	%r15
.seh_pushreg %r15
pushq	%r14
.seh_pushreg %r14
pushq	%r13
.seh_pushreg %r13
pushq	%r12
.seh_pushreg %r12
pushq	%rsi
.seh_pushreg %rsi
pushq	%rdi
.seh_pushreg %rdi
pushq	%rbx
.seh_pushreg %rbx
subq	$128, %rsp
.seh_stackalloc 128
.seh_endprologue
movq	%rcx, %rsi
.Ltmp110:
.cv_loc	67 15 73 0
movl	$4, %ecx
.Ltmp111:
callq	_ZN6ninput3any8is_press17h6ad3afc071954a5aE
testb	%al, %al
je	.LBB9_1
.Ltmp112:
.cv_loc	67 15 74 0
movq	_ZN20arena_latency_slider19non_hdr_update_css215CURRENT_COUNTER17ha6ae7ad1c78de36cE.0(%rip), %rcx
testq	%rcx, %rcx
jne	.LBB9_6
.Ltmp113:
movl	$1, %eax
jmp	.LBB9_5
.Ltmp114:
.LBB9_1:
.cv_loc	67 15 78 0
movl	$8, %ecx
callq	_ZN6ninput3any8is_press17h6ad3afc071954a5aE
testb	%al, %al
je	.LBB9_2
.Ltmp115:
.cv_loc	67 15 79 0
movq	_ZN20arena_latency_slider19non_hdr_update_css215CURRENT_COUNTER17ha6ae7ad1c78de36cE.0(%rip), %rcx
testq	%rcx, %rcx
jne	.LBB9_6
.Ltmp116:
movq	$-1, %rax
.Ltmp117:
.LBB9_5:
addq	%rax, _ZN20arena_latency_slider20CURRENT_INPUT_BUFFER17hcd1c017fa6563844E(%rip)
.Ltmp118:
.LBB9_6:
incq	%rcx
movabsq	$-3689348814741910323, %rdx
movq	%rcx, %rax
mulq	%rdx
shrq	$2, %rdx
andq	$-2, %rdx
leaq	(%rdx,%rdx,4), %rax
subq	%rax, %rcx
jmp	.LBB9_7
.Ltmp119:
.LBB9_2:
xorl	%ecx, %ecx
.Ltmp120:
.LBB9_7:
.cv_loc	67 15 87 0
movq	%rcx, _ZN20arena_latency_slider19non_hdr_update_css215CURRENT_COUNTER17ha6ae7ad1c78de36cE.0(%rip)
leaq	_ZN20arena_latency_slider20CURRENT_INPUT_BUFFER17hcd1c017fa6563844E(%rip), %r14
movq	_ZN20arena_latency_slider20CURRENT_INPUT_BUFFER17hcd1c017fa6563844E(%rip), %rax
.Ltmp121:
cmpq	$25, %rax
movl	$25, %ecx
cmovlq	%rax, %rcx
.Ltmp122:
.cv_inline_site_id 68 within 67 inlined_at 15 87 0
.cv_loc	68 16 861 0
testq	%rcx, %rcx
movq	$-1, %rax
.Ltmp123:
cmovnsq	%rcx, %rax
.Ltmp124:
.cv_loc	67 15 87 0
movq	%rax, _ZN20arena_latency_slider20CURRENT_INPUT_BUFFER17hcd1c017fa6563844E(%rip)
.cv_loc	67 15 88 0
movq	3672(%rsi), %rax
movq	16(%rax), %rdi
movq	%r14, 40(%rsp)
leaq	_ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$isize$GT$3fmt17ha46d39a4aa14b182E(%rip), %r15
movq	%r15, 48(%rsp)
.Ltmp125:
.cv_inline_site_id 69 within 67 inlined_at 15 88 0
.cv_loc	69 1 398 0
leaq	__unnamed_13(%rip), %r12
.Ltmp126:
movq	%r12, 56(%rsp)
movq	$2, 64(%rsp)
movq	$0, 72(%rsp)
leaq	40(%rsp), %r13
movq	%r13, 88(%rsp)
movq	$1, 96(%rsp)
leaq	104(%rsp), %rbx
leaq	56(%rsp), %rdx
.Ltmp127:
.cv_loc	67 15 88 0
movq	%rbx, %rcx
callq	_ZN5alloc3fmt6format17h96a775549426137dE
.Ltmp128:
.cv_inline_site_id 70 within 67 inlined_at 15 88 0
.cv_inline_site_id 71 within 70 inlined_at 3 2460 0
.cv_inline_site_id 72 within 71 inlined_at 4 2641 0
.cv_inline_site_id 73 within 72 inlined_at 4 1237 0
.cv_loc	73 2 224 0
movq	104(%rsp), %rdx
.Ltmp129:
.cv_loc	67 15 88 0
movq	%rdi, %rcx
callq	_ZN20arena_latency_slider15set_text_string17hc4197c5838b7032fE
movq	%rbx, %rcx
callq	_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h2571fb4fd1fdb246E
.cv_loc	67 15 89 0
movq	3688(%rsi), %rax
movq	16(%rax), %rsi
.Ltmp130:
movq	%r14, 40(%rsp)
movq	%r15, 48(%rsp)
.Ltmp131:
.cv_inline_site_id 74 within 67 inlined_at 15 89 0
.cv_loc	74 1 398 0
movq	%r12, 56(%rsp)
movq	$2, 64(%rsp)
movq	$0, 72(%rsp)
movq	%r13, 88(%rsp)
movq	$1, 96(%rsp)
leaq	104(%rsp), %rdi
leaq	56(%rsp), %rdx
.Ltmp132:
.cv_loc	67 15 89 0
movq	%rdi, %rcx
callq	_ZN5alloc3fmt6format17h96a775549426137dE
.Ltmp133:
.cv_inline_site_id 75 within 67 inlined_at 15 89 0
.cv_inline_site_id 76 within 75 inlined_at 3 2460 0
.cv_inline_site_id 77 within 76 inlined_at 4 2641 0
.cv_inline_site_id 78 within 77 inlined_at 4 1237 0
.cv_loc	78 2 224 0
movq	104(%rsp), %rdx
.Ltmp134:
.cv_loc	67 15 89 0
movq	%rsi, %rcx
callq	_ZN20arena_latency_slider15set_text_string17hc4197c5838b7032fE
movq	%rdi, %rcx
callq	_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h2571fb4fd1fdb246E
.cv_loc	67 15 90 0
ud2
.Ltmp135:
.Lfunc_end9:
.seh_endproc

.def	non_hdr_set_online_latency;
.scl	2;
.type	32;
.endef
.section	.text,"xr",one_only,non_hdr_set_online_latency
.globl	non_hdr_set_online_latency
.p2align	4, 0x90
non_hdr_set_online_latency:
.Lfunc_begin10:
.cv_func_id 79
.cv_loc	79 15 98 0
cmpb	$0, _ZN20arena_latency_slider9IS_USABLE17h1375d313c434b7e9E.0(%rip)
je	.LBB10_3
.Ltmp136:
.cv_loc	79 15 97 0
movq	152(%rcx), %rax
movzbl	(%rax), %ecx
.Ltmp137:
.cv_loc	79 15 99 0
movq	%rcx, _ZN20arena_latency_slider16MOST_RECENT_AUTO17hb25b258bf2ddaad6E(%rip)
.cv_loc	79 15 100 0
movq	_ZN20arena_latency_slider20CURRENT_INPUT_BUFFER17hcd1c017fa6563844E(%rip), %rcx
cmpq	$-1, %rcx
je	.LBB10_3
.Ltmp138:
.cv_loc	79 15 101 0
movb	%cl, (%rax)
.Ltmp139:
.LBB10_3:
.cv_loc	79 15 104 0
retq
.Ltmp140:
.Lfunc_end10:

.def	bg_matchmaking_seq;
.scl	2;
.type	32;
.endef
.section	.text,"xr",one_only,bg_matchmaking_seq
.globl	bg_matchmaking_seq
.p2align	4, 0x90
bg_matchmaking_seq:
.Lfunc_begin11:
.cv_func_id 80
.cv_loc	80 15 113 0
movb	$0, _ZN20arena_latency_slider9IS_USABLE17h1375d313c434b7e9E.0(%rip)
.cv_loc	80 15 114 0
retq
.Ltmp141:
.Lfunc_end11:

.def	arena_seq;
.scl	2;
.type	32;
.endef
.section	.text,"xr",one_only,arena_seq
.globl	arena_seq
.p2align	4, 0x90
arena_seq:
.Lfunc_begin12:
.cv_func_id 81
.cv_loc	81 15 118 0
movb	$1, _ZN20arena_latency_slider9IS_USABLE17h1375d313c434b7e9E.0(%rip)
.cv_loc	81 15 119 0
retq
.Ltmp142:
.Lfunc_end12:

.def	__pthread_mutex_lock;
.scl	2;
.type	32;
.endef
.section	.text,"xr",one_only,__pthread_mutex_lock
.globl	__pthread_mutex_lock
.p2align	4, 0x90
__pthread_mutex_lock:
.Lfunc_begin13:
.cv_func_id 82
.cv_loc	82 15 130 0
jmp	pthread_mutex_lock
.Ltmp143:
.Lfunc_end13:

.def	__pthread_key_create;
.scl	2;
.type	32;
.endef
.section	.text,"xr",one_only,__pthread_key_create
.globl	__pthread_key_create
.p2align	4, 0x90
__pthread_key_create:
.Lfunc_begin14:
.cv_func_id 83
.cv_loc	83 15 130 0
jmp	pthread_key_create
.Ltmp144:
.Lfunc_end14:

.def	__pthread_key_delete;
.scl	2;
.type	32;
.endef
.section	.text,"xr",one_only,__pthread_key_delete
.globl	__pthread_key_delete
.p2align	4, 0x90
__pthread_key_delete:
.Lfunc_begin15:
.cv_func_id 84
.cv_loc	84 15 130 0
jmp	pthread_key_delete
.Ltmp145:
.Lfunc_end15:

.def	__custom_fini;
.scl	2;
.type	32;
.endef
.section	.text,"xr",one_only,__custom_fini
.globl	__custom_fini
.p2align	4, 0x90
__custom_fini:
.Lfunc_begin16:
.cv_func_id 85
.cv_file	18 "C:\\Users\\blujay\\.cargo\\registry\\src\\github.com-1ecc6299db9ec823\\skyline-0.2.1\\src\\build.rs" "A59FE857414C2225BF32B934411ABDFD34E0A4DB" 2
.cv_loc	85 18 82 0
retq
.Ltmp146:
.Lfunc_end16:

.section	.rdata,"dr",one_only,__unnamed_1
.p2align	3
__unnamed_1:

.section	.rdata,"dr",one_only,__unnamed_11
__unnamed_11:
.ascii	"called `Result::unwrap()` on an `Err` value"

.section	.rdata,"dr",one_only,__unnamed_12
.p2align	3
__unnamed_12:
.quad	_ZN4core3ptr50drop_in_place$LT$alloc..string..FromUtf16Error$GT$17h53c7078b33f3951aE
.asciz	"\000\000\000\000\000\000\000\000\001\000\000\000\000\000\000"
.quad	_ZN66_$LT$alloc..string..FromUtf16Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h92e84c897117e011E

.section	.bss,"bw",one_only,_ZN20arena_latency_slider19CURRENT_PANE_HANDLE17h2bb43f3630faa000E.0
.p2align	3
_ZN20arena_latency_slider19CURRENT_PANE_HANDLE17h2bb43f3630faa000E.0:
.quad	0

.section	.data,"dw",one_only,_ZN20arena_latency_slider16CURRENT_ARENA_ID17h90041e33cf1e16dfE
.p2align	3
_ZN20arena_latency_slider16CURRENT_ARENA_ID17h90041e33cf1e16dfE:
.asciz	"\001\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000"

.section	.data,"dw",one_only,_ZN20arena_latency_slider20CURRENT_INPUT_BUFFER17hcd1c017fa6563844E
.p2align	3
_ZN20arena_latency_slider20CURRENT_INPUT_BUFFER17hcd1c017fa6563844E:
.asciz	"\004\000\000\000\000\000\000"

.section	.data,"dw",one_only,_ZN20arena_latency_slider16MOST_RECENT_AUTO17hb25b258bf2ddaad6E
.p2align	3
_ZN20arena_latency_slider16MOST_RECENT_AUTO17hb25b258bf2ddaad6E:
.zero	8,255

.section	.bss,"bw",one_only,_ZN20arena_latency_slider9IS_USABLE17h1375d313c434b7e9E.0
_ZN20arena_latency_slider9IS_USABLE17h1375d313c434b7e9E.0:
.byte	0

.section	.rdata,"dr",one_only,__unnamed_14
__unnamed_14:
.ascii	"\nInput Latency: "

.section	.rdata,"dr",one_only,__unnamed_15
__unnamed_15:
.zero	1

.section	.rdata,"dr",one_only,__unnamed_2
.p2align	3
__unnamed_2:
.quad	__unnamed_1
.zero	8
.quad	__unnamed_14
.asciz	"\020\000\000\000\000\000\000"
.quad	__unnamed_15
.asciz	"\001\000\000\000\000\000\000"

.section	.rdata,"dr",one_only,__unnamed_16
__unnamed_16:
.ascii	"ROOM ID: "

.section	.rdata,"dr",one_only,__unnamed_17
__unnamed_17:
.ascii	"\nInput Latency: Auto ("

.section	.rdata,"dr",one_only,__unnamed_18
__unnamed_18:
.byte	41

.section	.rdata,"dr",one_only,__unnamed_3
.p2align	3
__unnamed_3:
.quad	__unnamed_16
.asciz	"\t\000\000\000\000\000\000"
.quad	__unnamed_17
.asciz	"\026\000\000\000\000\000\000"
.quad	__unnamed_18
.asciz	"\001\000\000\000\000\000\000"

.section	.rdata,"dr",one_only,__unnamed_19
__unnamed_19:
.ascii	"\nInput Latency: Auto"

.section	.rdata,"dr",one_only,__unnamed_4
.p2align	3
__unnamed_4:
.quad	__unnamed_16
.asciz	"\t\000\000\000\000\000\000"
.quad	__unnamed_19
.asciz	"\024\000\000\000\000\000\000"

.section	.bss,"bw",one_only,_ZN20arena_latency_slider24non_hdr_update_room_hook15CURRENT_COUNTER17h701e90c953f8ab41E.0
.p2align	3
_ZN20arena_latency_slider24non_hdr_update_room_hook15CURRENT_COUNTER17h701e90c953f8ab41E.0:
.quad	0

.section	.rdata,"dr",one_only,__unnamed_20
__unnamed_20:
.ascii	"src\\lib.rs"

.section	.rdata,"dr",one_only,__unnamed_10
.p2align	3
__unnamed_10:
.quad	__unnamed_20
.asciz	"\n\000\000\000\000\000\000\000A\000\000\000{{\000\000"

.section	.rdata,"dr",one_only,__unnamed_21
__unnamed_21:
.byte	91

.section	.rdata,"dr",one_only,__unnamed_22
__unnamed_22:
.byte	58

.section	.rdata,"dr",one_only,__unnamed_23
__unnamed_23:
.ascii	"] "

.section	.rdata,"dr",one_only,__unnamed_24
__unnamed_24:
.ascii	" = "

.section	.rdata,"dr",one_only,__unnamed_25
__unnamed_25:
.byte	10

.section	.rdata,"dr",one_only,__unnamed_8
.p2align	3
__unnamed_8:
.quad	__unnamed_21
.asciz	"\001\000\000\000\000\000\000"
.quad	__unnamed_22
.asciz	"\001\000\000\000\000\000\000"
.quad	__unnamed_23
.asciz	"\002\000\000\000\000\000\000"
.quad	__unnamed_24
.asciz	"\003\000\000\000\000\000\000"
.quad	__unnamed_25
.asciz	"\001\000\000\000\000\000\000"

.section	.rdata,"dr",one_only,__unnamed_5
.p2align	3
__unnamed_5:
.quad	__unnamed_20
.asciz	"\n\000\000\000\000\000\000"

.section	.rdata,"dr",one_only,__unnamed_6
.p2align	2
__unnamed_6:
.asciz	"A\000\000"

.section	.rdata,"dr",one_only,__unnamed_26
__unnamed_26:
.ascii	"String::from_utf16(std::slice::from_raw_parts(*ctx.registers[3].x.as_ref() as\n                *const u16, 5)).unwrap()"

.section	.rdata,"dr",one_only,__unnamed_7
.p2align	3
__unnamed_7:
.quad	__unnamed_26
.asciz	"v\000\000\000\000\000\000"

.section	.rdata,"dr",one_only,__unnamed_9
.p2align	3
__unnamed_9:
.asciz	"\000\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000 \000\000\000\000\000\000\000\003\000\000\000\000\000\000\000\001\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000 \000\000\000\000\000\000\000\003\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000 \000\000\000\000\000\000\000\003\000\000\000\000\000\000\000\003\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000 \000\000\000\004\000\000\000\003\000\000\000\000\000\000"

.section	.rdata,"dr",one_only,__unnamed_27
__unnamed_27:
.ascii	"Input Latency: "

.section	.rdata,"dr",one_only,__unnamed_13
.p2align	3
__unnamed_13:
.quad	__unnamed_27
.asciz	"\017\000\000\000\000\000\000"
.quad	__unnamed_15
.asciz	"\001\000\000\000\000\000\000"

.section	.bss,"bw",one_only,_ZN20arena_latency_slider19non_hdr_update_css215CURRENT_COUNTER17ha6ae7ad1c78de36cE.0
.p2align	3
_ZN20arena_latency_slider19non_hdr_update_css215CURRENT_COUNTER17ha6ae7ad1c78de36cE.0:
.quad	0

.section	.rodata.module_name,"dr"
_ZN20arena_latency_slider13__MODULE_NAME17ha529bc1e5e79fa04E:
.asciz	"\000\000\000\000\024\000\000\000arena-latency-slider"

.section	.debug$S,"dr"
.p2align	2
.long	4
.long	241
.long	.Ltmp148-.Ltmp147
.Ltmp147:
.short	.Ltmp150-.Ltmp149
.Ltmp149:
.short	4353
.long	0
.byte	0
.p2align	2
.Ltmp150:
.short	.Ltmp152-.Ltmp151
.Ltmp151:
.short	4412
.long	21
.short	208
.short	1
.short	67
.short	0
.short	0
.short	15004
.short	0
.short	0
.short	0
.asciz	"clang LLVM (rustc version 1.67.0-nightly (7eef946fc 2022-11-06))"
.p2align	2
.Ltmp152:
.Ltmp148:
.p2align	2
.long	246
.long	.Ltmp154-.Ltmp153
.Ltmp153:
.long	0


.long	4197
.cv_filechecksumoffset	3
.long	2282


.long	4205
.cv_filechecksumoffset	3
.long	2459


.long	4213
.cv_filechecksumoffset	4
.long	2640


.long	4216
.cv_filechecksumoffset	4
.long	1234


.long	4219
.cv_filechecksumoffset	2
.long	223


.long	4222
.cv_filechecksumoffset	5
.long	490


.long	4225
.cv_filechecksumoffset	5
.long	490


.long	4227
.cv_filechecksumoffset	2
.long	477


.long	4265
.cv_filechecksumoffset	2
.long	240


.long	4293
.cv_filechecksumoffset	6
.long	426


.long	4297
.cv_filechecksumoffset	6
.long	431


.long	4302
.cv_filechecksumoffset	7
.long	250


.long	4306
.cv_filechecksumoffset	7
.long	116


.long	4417
.cv_filechecksumoffset	1
.long	516


.long	4427
.cv_filechecksumoffset	9
.long	1010


.long	4431
.cv_filechecksumoffset	8
.long	616


.long	4435
.cv_filechecksumoffset	10
.long	510


.long	4437
.cv_filechecksumoffset	11
.long	208


.long	4441
.cv_filechecksumoffset	12
.long	785


.long	4443
.cv_filechecksumoffset	12
.long	407


.long	4446
.cv_filechecksumoffset	12
.long	431


.long	4448
.cv_filechecksumoffset	12
.long	105


.long	4450
.cv_filechecksumoffset	12
.long	156


.long	4453
.cv_filechecksumoffset	4
.long	672


.long	4455
.cv_filechecksumoffset	2
.long	130


.long	4461
.cv_filechecksumoffset	2
.long	169


.long	4496
.cv_filechecksumoffset	7
.long	240


.long	4499
.cv_filechecksumoffset	7
.long	176


.long	4502
.cv_filechecksumoffset	7
.long	98


.long	4506
.cv_filechecksumoffset	14
.long	1259


.long	4508
.cv_filechecksumoffset	13
.long	2412


.long	4511
.cv_filechecksumoffset	3
.long	845


.long	4205
.cv_filechecksumoffset	3
.long	2459


.long	4514
.cv_filechecksumoffset	14
.long	456


.long	4518
.cv_filechecksumoffset	16
.long	854


.long	4522
.cv_filechecksumoffset	8
.long	608


.long	4556
.cv_filechecksumoffset	17
.long	1107


.long	4563
.cv_filechecksumoffset	1
.long	414


.long	4565
.cv_filechecksumoffset	5
.long	490


.long	4568
.cv_filechecksumoffset	1
.long	394


.long	4205
.cv_filechecksumoffset	3
.long	2459
.Ltmp154:
.p2align	2
.section	.debug$S,"dr",associative,_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h9a28d8fc34dcbba9E
.p2align	2
.long	4
.long	241
.long	.Ltmp156-.Ltmp155
.Ltmp155:
.short	.Ltmp158-.Ltmp157
.Ltmp157:
.short	4422
.long	0
.long	0
.long	0
.long	.Lfunc_end0-_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h9a28d8fc34dcbba9E
.long	0
.long	0
.long	4573
.secrel32	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h9a28d8fc34dcbba9E
.secidx	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h9a28d8fc34dcbba9E
.byte	0
.asciz	"core::fmt::impl$59::fmt<alloc::string::String>"
.p2align	2
.Ltmp158:
.short	.Ltmp160-.Ltmp159
.Ltmp159:
.short	4114
.long	0
.long	0
.long	0
.long	0
.long	0
.short	0
.long	1048576
.p2align	2
.Ltmp160:
.short	.Ltmp162-.Ltmp161
.Ltmp161:
.short	4414
.long	4570
.short	1
.asciz	"self"
.p2align	2
.Ltmp162:
.cv_def_range	 .Lfunc_begin0 .Ltmp2, reg, 330
.short	.Ltmp164-.Ltmp163
.Ltmp163:
.short	4414
.long	4101
.short	1
.asciz	"f"
.p2align	2
.Ltmp164:
.cv_def_range	 .Lfunc_begin0 .Ltmp3, reg, 331
.cv_def_range	 .Ltmp3 .Ltmp4, reg, 336
.short	.Ltmp166-.Ltmp165
.Ltmp165:
.short	4429
.long	0
.long	0
.long	4197
.cv_inline_linetable	1 3 2282 .Lfunc_begin0 .Lfunc_end0
.p2align	2
.Ltmp166:
.short	.Ltmp168-.Ltmp167
.Ltmp167:
.short	4414
.long	4099
.short	1
.asciz	"self"
.p2align	2
.Ltmp168:
.cv_def_range	 .Ltmp1 .Ltmp4, reg, 328
.short	.Ltmp170-.Ltmp169
.Ltmp169:
.short	4414
.long	4101
.short	1
.asciz	"f"
.p2align	2
.Ltmp170:
.cv_def_range	 .Ltmp1 .Ltmp3, reg, 331
.cv_def_range	 .Ltmp3 .Ltmp4, reg, 336
.short	.Ltmp172-.Ltmp171
.Ltmp171:
.short	4429
.long	0
.long	0
.long	4205
.cv_inline_linetable	2 3 2459 .Lfunc_begin0 .Lfunc_end0
.p2align	2
.Ltmp172:
.short	.Ltmp174-.Ltmp173
.Ltmp173:
.short	4414
.long	4099
.short	0
.asciz	"self"
.p2align	2
.Ltmp174:
.cv_def_range	 .Ltmp1 .Ltmp4, reg, 328
.short	.Ltmp176-.Ltmp175
.Ltmp175:
.short	4429
.long	0
.long	0
.long	4213
.cv_inline_linetable	3 4 2640 .Lfunc_begin0 .Lfunc_end0
.p2align	2
.Ltmp176:
.short	.Ltmp178-.Ltmp177
.Ltmp177:
.short	4414
.long	4208
.short	1
.asciz	"self"
.p2align	2
.Ltmp178:
.cv_def_range	 .Ltmp1 .Ltmp4, reg, 328
.short	.Ltmp180-.Ltmp179
.Ltmp179:
.short	4429
.long	0
.long	0
.long	4216
.cv_inline_linetable	4 4 1234 .Lfunc_begin0 .Lfunc_end0
.p2align	2
.Ltmp180:
.short	.Ltmp182-.Ltmp181
.Ltmp181:
.short	4414
.long	4208
.short	0
.asciz	"self"
.p2align	2
.Ltmp182:
.cv_def_range	 .Ltmp1 .Ltmp4, reg, 328
.short	.Ltmp184-.Ltmp183
.Ltmp183:
.short	4429
.long	0
.long	0
.long	4219
.cv_inline_linetable	5 2 223 .Lfunc_begin0 .Lfunc_end0
.p2align	2
.Ltmp184:
.short	.Ltmp186-.Ltmp185
.Ltmp185:
.short	4414
.long	4217
.short	0
.asciz	"self"
.p2align	2
.Ltmp186:
.cv_def_range	 .Ltmp1 .Ltmp4, reg, 328
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4431
.Ltmp156:
.p2align	2
.cv_linetable	0, _ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h9a28d8fc34dcbba9E, .Lfunc_end0
.section	.debug$S,"dr",associative,_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hf6378e006fcd7e96E
.p2align	2
.long	4
.long	241
.long	.Ltmp188-.Ltmp187
.Ltmp187:
.short	.Ltmp190-.Ltmp189
.Ltmp189:
.short	4422
.long	0
.long	0
.long	0
.long	.Lfunc_end1-_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hf6378e006fcd7e96E
.long	0
.long	0
.long	4577
.secrel32	_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hf6378e006fcd7e96E
.secidx	_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hf6378e006fcd7e96E
.byte	0
.asciz	"core::fmt::impl$61::fmt<str$>"
.p2align	2
.Ltmp190:
.short	.Ltmp192-.Ltmp191
.Ltmp191:
.short	4114
.long	0
.long	0
.long	0
.long	0
.long	0
.short	0
.long	1048576
.p2align	2
.Ltmp192:
.short	.Ltmp194-.Ltmp193
.Ltmp193:
.short	4414
.long	4314
.short	1
.asciz	"self"
.p2align	2
.Ltmp194:
.cv_def_range	 .Lfunc_begin1 .Ltmp7, reg, 330
.short	.Ltmp196-.Ltmp195
.Ltmp195:
.short	4414
.long	4101
.short	1
.asciz	"f"
.p2align	2
.Ltmp196:
.cv_def_range	 .Lfunc_begin1 .Ltmp6, reg, 331
.cv_def_range	 .Ltmp6 .Ltmp8, reg, 336
.short	2
.short	4431
.Ltmp188:
.p2align	2
.cv_linetable	6, _ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hf6378e006fcd7e96E, .Lfunc_end1
.section	.debug$S,"dr",associative,_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h2571fb4fd1fdb246E
.p2align	2
.long	4
.long	241
.long	.Ltmp198-.Ltmp197
.Ltmp197:
.short	.Ltmp200-.Ltmp199
.Ltmp199:
.short	4422
.long	0
.long	0
.long	0
.long	.Lfunc_end2-_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h2571fb4fd1fdb246E
.long	0
.long	0
.long	4565
.secrel32	_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h2571fb4fd1fdb246E
.secidx	_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h2571fb4fd1fdb246E
.byte	0
.asciz	"core::ptr::drop_in_place<alloc::string::String>"
.p2align	2
.Ltmp200:
.short	.Ltmp202-.Ltmp201
.Ltmp201:
.short	4114
.long	0
.long	0
.long	0
.long	0
.long	0
.short	0
.long	1048592
.p2align	2
.Ltmp202:
.short	.Ltmp204-.Ltmp203
.Ltmp203:
.short	4414
.long	4099
.short	1
.byte	0
.p2align	2
.Ltmp204:
.cv_def_range	 .Lfunc_begin2 .Ltmp11 .Ltmp12 .Lfunc_end2, reg, 330
.short	.Ltmp206-.Ltmp205
.Ltmp205:
.short	4429
.long	0
.long	0
.long	4222
.cv_inline_linetable	8 5 490 .Lfunc_begin2 .Lfunc_end2
.p2align	2
.Ltmp206:
.short	.Ltmp208-.Ltmp207
.Ltmp207:
.short	4414
.long	4208
.short	1
.byte	0
.p2align	2
.Ltmp208:
.cv_def_range	 .Lfunc_begin2 .Ltmp11, reg, 330
.short	.Ltmp210-.Ltmp209
.Ltmp209:
.short	4429
.long	0
.long	0
.long	4225
.cv_inline_linetable	9 5 490 .Lfunc_begin2 .Lfunc_end2
.p2align	2
.Ltmp210:
.short	.Ltmp212-.Ltmp211
.Ltmp211:
.short	4414
.long	4217
.short	1
.byte	0
.p2align	2
.Ltmp212:
.cv_def_range	 .Lfunc_begin2 .Ltmp11, reg, 330
.short	.Ltmp214-.Ltmp213
.Ltmp213:
.short	4429
.long	0
.long	0
.long	4227
.cv_inline_linetable	10 2 477 .Lfunc_begin2 .Lfunc_end2
.p2align	2
.Ltmp214:
.short	.Ltmp216-.Ltmp215
.Ltmp215:
.short	4414
.long	4217
.short	1
.asciz	"self"
.p2align	2
.Ltmp216:
.cv_def_range	 .Lfunc_begin2 .Ltmp11, reg, 330
.short	.Ltmp218-.Ltmp217
.Ltmp217:
.short	4414
.long	4257
.short	0
.asciz	"layout"
.p2align	2
.Ltmp218:
.cv_def_range	 .Ltmp10 .Ltmp12, subfield_reg, 336, 8
.cv_def_range	 .Ltmp11 .Ltmp12, subfield_reg, 331, 0
.short	.Ltmp220-.Ltmp219
.Ltmp219:
.short	4414
.long	4193
.short	0
.asciz	"ptr"
.p2align	2
.Ltmp220:
.cv_def_range	 .Ltmp11 .Ltmp12, reg, 330
.short	.Ltmp222-.Ltmp221
.Ltmp221:
.short	4429
.long	0
.long	0
.long	4265
.cv_inline_linetable	11 2 240 .Lfunc_begin2 .Lfunc_end2
.p2align	2
.Ltmp222:
.short	.Ltmp224-.Ltmp223
.Ltmp223:
.short	4414
.long	4217
.short	1
.asciz	"self"
.p2align	2
.Ltmp224:
.cv_def_range	 .Lfunc_begin2 .Ltmp11, reg, 330
.short	.Ltmp226-.Ltmp225
.Ltmp225:
.short	4414
.long	4257
.short	0
.asciz	"layout"
.p2align	2
.Ltmp226:
.cv_def_range	 .Ltmp10 .Ltmp12, subfield_reg, 336, 8
.cv_def_range	 .Ltmp10 .Ltmp12, subfield_reg, 331, 0
.short	.Ltmp228-.Ltmp227
.Ltmp227:
.short	4429
.long	0
.long	0
.long	4293
.cv_inline_linetable	12 6 426 .Lfunc_begin2 .Lfunc_end2
.p2align	2
.Ltmp228:
.short	.Ltmp230-.Ltmp229
.Ltmp229:
.short	4414
.long	35
.short	0
.asciz	"n"
.p2align	2
.Ltmp230:
.cv_def_range	 .Ltmp9 .Ltmp12, reg, 331
.short	.Ltmp232-.Ltmp231
.Ltmp231:
.short	4429
.long	0
.long	0
.long	4297
.cv_inline_linetable	13 6 431 .Lfunc_begin2 .Lfunc_end2
.p2align	2
.Ltmp232:
.short	.Ltmp234-.Ltmp233
.Ltmp233:
.short	4414
.long	35
.short	257
.asciz	"element_size"
.p2align	2
.Ltmp234:
.short	.Ltmp236-.Ltmp235
.Ltmp235:
.short	4414
.long	4263
.short	257
.asciz	"align"
.p2align	2
.Ltmp236:
.short	.Ltmp238-.Ltmp237
.Ltmp237:
.short	4414
.long	35
.short	1
.asciz	"n"
.p2align	2
.Ltmp238:
.cv_def_range	 .Ltmp9 .Ltmp12, reg, 331
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	.Ltmp240-.Ltmp239
.Ltmp239:
.short	4429
.long	0
.long	0
.long	4302
.cv_inline_linetable	14 7 250 .Lfunc_begin2 .Lfunc_end2
.p2align	2
.Ltmp240:
.short	.Ltmp242-.Ltmp241
.Ltmp241:
.short	4414
.long	4299
.short	257
.asciz	"self"
.p2align	2
.Ltmp242:
.short	.Ltmp244-.Ltmp243
.Ltmp243:
.short	4414
.long	4193
.short	1
.asciz	"ptr"
.p2align	2
.Ltmp244:
.cv_def_range	 .Ltmp11 .Ltmp12, reg, 330
.short	.Ltmp246-.Ltmp245
.Ltmp245:
.short	4414
.long	4257
.short	0
.asciz	"layout"
.p2align	2
.Ltmp246:
.cv_def_range	 .Ltmp10 .Ltmp12, subfield_reg, 336, 8
.cv_def_range	 .Ltmp11 .Ltmp12, subfield_reg, 331, 0
.short	.Ltmp248-.Ltmp247
.Ltmp247:
.short	4429
.long	0
.long	0
.long	4306
.cv_inline_linetable	15 7 116 .Lfunc_begin2 .Lfunc_end2
.p2align	2
.Ltmp248:
.short	.Ltmp250-.Ltmp249
.Ltmp249:
.short	4414
.long	4257
.short	0
.asciz	"layout"
.p2align	2
.Ltmp250:
.cv_def_range	 .Ltmp10 .Ltmp12, subfield_reg, 336, 8
.cv_def_range	 .Ltmp11 .Ltmp12, subfield_reg, 331, 0
.short	.Ltmp252-.Ltmp251
.Ltmp251:
.short	4414
.long	1568
.short	0
.asciz	"ptr"
.p2align	2
.Ltmp252:
.cv_def_range	 .Ltmp11 .Ltmp12, reg, 330
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4431
.Ltmp198:
.p2align	2
.cv_linetable	7, _ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h2571fb4fd1fdb246E, .Lfunc_end2
.section	.debug$S,"dr",associative,_ZN4core3ptr50drop_in_place$LT$alloc..string..FromUtf16Error$GT$17h53c7078b33f3951aE
.p2align	2
.long	4
.long	241
.long	.Ltmp254-.Ltmp253
.Ltmp253:
.short	.Ltmp256-.Ltmp255
.Ltmp255:
.short	4422
.long	0
.long	0
.long	0
.long	.Lfunc_end3-_ZN4core3ptr50drop_in_place$LT$alloc..string..FromUtf16Error$GT$17h53c7078b33f3951aE
.long	0
.long	0
.long	4581
.secrel32	_ZN4core3ptr50drop_in_place$LT$alloc..string..FromUtf16Error$GT$17h53c7078b33f3951aE
.secidx	_ZN4core3ptr50drop_in_place$LT$alloc..string..FromUtf16Error$GT$17h53c7078b33f3951aE
.byte	0
.asciz	"core::ptr::drop_in_place<alloc::string::FromUtf16Error>"
.p2align	2
.Ltmp256:
.short	.Ltmp258-.Ltmp257
.Ltmp257:
.short	4114
.long	0
.long	0
.long	0
.long	0
.long	0
.short	0
.long	1048608
.p2align	2
.Ltmp258:
.short	.Ltmp260-.Ltmp259
.Ltmp259:
.short	4414
.long	4578
.short	257
.byte	0
.p2align	2
.Ltmp260:
.short	2
.short	4431
.Ltmp254:
.p2align	2
.cv_linetable	16, _ZN4core3ptr50drop_in_place$LT$alloc..string..FromUtf16Error$GT$17h53c7078b33f3951aE, .Lfunc_end3
.section	.debug$S,"dr",associative,_ZN5alloc3fmt6format17h96a775549426137dE
.p2align	2
.long	4
.long	241
.long	.Ltmp262-.Ltmp261
.Ltmp261:
.short	.Ltmp264-.Ltmp263
.Ltmp263:
.short	4422
.long	0
.long	0
.long	0
.long	.Lfunc_end4-_ZN5alloc3fmt6format17h96a775549426137dE
.long	0
.long	0
.long	4522
.secrel32	_ZN5alloc3fmt6format17h96a775549426137dE
.secidx	_ZN5alloc3fmt6format17h96a775549426137dE
.byte	0
.asciz	"alloc::fmt::format"
.p2align	2
.Ltmp264:
.short	.Ltmp266-.Ltmp265
.Ltmp265:
.short	4114
.long	88
.long	0
.long	0
.long	32
.long	0
.short	0
.long	1130544
.p2align	2
.Ltmp266:
.short	.Ltmp268-.Ltmp267
.Ltmp267:
.short	4414
.long	4312
.short	1
.asciz	"args"
.p2align	2
.Ltmp268:
.cv_def_range	 .Lfunc_begin4 .Ltmp21 .Ltmp22 .Ltmp27 .Ltmp31 .Ltmp33 .Ltmp36 .Ltmp37, reg_rel, 331, 0, 0
.short	.Ltmp270-.Ltmp269
.Ltmp269:
.short	4429
.long	0
.long	0
.long	4417
.cv_inline_linetable	18 1 516 .Lfunc_begin4 .Lfunc_end4
.p2align	2
.Ltmp270:
.short	.Ltmp272-.Ltmp271
.Ltmp271:
.short	4414
.long	4395
.short	1
.asciz	"self"
.p2align	2
.Ltmp272:
.cv_def_range	 .Ltmp15 .Ltmp20 .Ltmp22 .Ltmp25, reg, 331
.short	.Ltmp274-.Ltmp273
.Ltmp273:
.short	4414
.long	4314
.short	0
.asciz	"s"
.p2align	2
.Ltmp274:
.cv_def_range	 .Ltmp23 .Ltmp25, reg, 328
.short	2
.short	4430
.short	.Ltmp276-.Ltmp275
.Ltmp275:
.short	4429
.long	0
.long	0
.long	4427
.cv_inline_linetable	19 9 1010 .Lfunc_begin4 .Lfunc_end4
.p2align	2
.Ltmp276:
.short	.Ltmp278-.Ltmp277
.Ltmp277:
.short	4414
.long	4425
.short	1
.asciz	"default"
.p2align	2
.Ltmp278:
.cv_def_range	 .Ltmp20 .Ltmp21 .Ltmp24 .Ltmp27 .Ltmp31 .Ltmp32 .Ltmp36 .Ltmp37, reg, 331
.short	.Ltmp280-.Ltmp279
.Ltmp279:
.short	4414
.long	4400
.short	0
.asciz	"self"
.p2align	2
.Ltmp280:
.cv_def_range	 .Ltmp24 .Ltmp32 .Ltmp36 .Lfunc_end4, subfield_reg, 342, 0
.cv_def_range	 .Ltmp24 .Ltmp32 .Ltmp36 .Lfunc_end4, subfield_reg, 333, 8
.short	.Ltmp282-.Ltmp281
.Ltmp281:
.short	4414
.long	4203
.short	0
.asciz	"t"
.p2align	2
.Ltmp282:
.cv_def_range	 .Ltmp24 .Ltmp32 .Ltmp36 .Lfunc_end4, subfield_reg, 342, 0
.cv_def_range	 .Ltmp24 .Ltmp32 .Ltmp36 .Lfunc_end4, subfield_reg, 333, 8
.short	.Ltmp284-.Ltmp283
.Ltmp283:
.short	4429
.long	0
.long	0
.long	4431
.cv_inline_linetable	20 8 616 .Lfunc_begin4 .Lfunc_end4
.p2align	2
.Ltmp284:
.short	.Ltmp286-.Ltmp285
.Ltmp285:
.short	4414
.long	4312
.short	256
.asciz	"args"
.p2align	2
.Ltmp286:
.short	2
.short	4430
.short	.Ltmp288-.Ltmp287
.Ltmp287:
.short	4429
.long	0
.long	0
.long	4435
.cv_inline_linetable	21 10 510 .Lfunc_begin4 .Lfunc_end4
.p2align	2
.Ltmp288:
.short	.Ltmp290-.Ltmp289
.Ltmp289:
.short	4414
.long	4583
.short	1
.byte	0
.p2align	2
.Ltmp290:
.cv_def_range	 .Ltmp24 .Ltmp32 .Ltmp36 .Lfunc_end4, subfield_reg, 342, 0
.cv_def_range	 .Ltmp24 .Ltmp32 .Ltmp36 .Lfunc_end4, subfield_reg, 333, 8
.short	.Ltmp292-.Ltmp291
.Ltmp291:
.short	4429
.long	0
.long	0
.long	4437
.cv_inline_linetable	22 11 208 .Lfunc_begin4 .Lfunc_end4
.p2align	2
.Ltmp292:
.short	.Ltmp294-.Ltmp293
.Ltmp293:
.short	4414
.long	4203
.short	0
.asciz	"self"
.p2align	2
.Ltmp294:
.cv_def_range	 .Ltmp24 .Ltmp32 .Ltmp36 .Lfunc_end4, subfield_reg, 342, 0
.cv_def_range	 .Ltmp24 .Ltmp32 .Ltmp36 .Lfunc_end4, subfield_reg, 333, 8
.short	.Ltmp296-.Ltmp295
.Ltmp295:
.short	4429
.long	0
.long	0
.long	4441
.cv_inline_linetable	23 12 785 .Lfunc_begin4 .Lfunc_end4
.p2align	2
.Ltmp296:
.short	.Ltmp298-.Ltmp297
.Ltmp297:
.short	4414
.long	4211
.short	0
.asciz	"self"
.p2align	2
.Ltmp298:
.cv_def_range	 .Ltmp24 .Ltmp32 .Ltmp36 .Lfunc_end4, subfield_reg, 342, 0
.cv_def_range	 .Ltmp24 .Ltmp32 .Ltmp36 .Lfunc_end4, subfield_reg, 333, 8
.short	.Ltmp300-.Ltmp299
.Ltmp299:
.short	4429
.long	0
.long	0
.long	4443
.cv_inline_linetable	24 12 407 .Lfunc_begin4 .Lfunc_end4
.p2align	2
.Ltmp300:
.short	.Ltmp302-.Ltmp301
.Ltmp301:
.short	4414
.long	4211
.short	0
.asciz	"self"
.p2align	2
.Ltmp302:
.cv_def_range	 .Ltmp24 .Ltmp32 .Ltmp36 .Lfunc_end4, subfield_reg, 342, 0
.cv_def_range	 .Ltmp24 .Ltmp32 .Ltmp36 .Lfunc_end4, subfield_reg, 333, 8
.short	.Ltmp304-.Ltmp303
.Ltmp303:
.short	4429
.long	0
.long	0
.long	4446
.cv_inline_linetable	25 12 431 .Lfunc_begin4 .Lfunc_end4
.p2align	2
.Ltmp304:
.short	.Ltmp306-.Ltmp305
.Ltmp305:
.short	4414
.long	4211
.short	0
.asciz	"self"
.p2align	2
.Ltmp306:
.cv_def_range	 .Ltmp24 .Ltmp32 .Ltmp36 .Lfunc_end4, subfield_reg, 342, 0
.cv_def_range	 .Ltmp24 .Ltmp32 .Ltmp36 .Lfunc_end4, subfield_reg, 333, 8
.short	.Ltmp308-.Ltmp307
.Ltmp307:
.short	4429
.long	0
.long	0
.long	4448
.cv_inline_linetable	26 12 105 .Lfunc_begin4 .Lfunc_end4
.p2align	2
.Ltmp308:
.short	.Ltmp310-.Ltmp309
.Ltmp309:
.short	4414
.long	4211
.short	0
.asciz	"s"
.p2align	2
.Ltmp310:
.cv_def_range	 .Ltmp24 .Ltmp32 .Ltmp36 .Lfunc_end4, subfield_reg, 342, 0
.cv_def_range	 .Ltmp24 .Ltmp32 .Ltmp36 .Lfunc_end4, subfield_reg, 333, 8
.short	.Ltmp312-.Ltmp311
.Ltmp311:
.short	4429
.long	0
.long	0
.long	4450
.cv_inline_linetable	27 12 156 .Lfunc_begin4 .Lfunc_end4
.p2align	2
.Ltmp312:
.short	.Ltmp314-.Ltmp313
.Ltmp313:
.short	4414
.long	4135
.short	0
.asciz	"v"
.p2align	2
.Ltmp314:
.cv_def_range	 .Ltmp33 .Ltmp35, subfield_reg, 329, 0
.short	.Ltmp316-.Ltmp315
.Ltmp315:
.short	4414
.long	4211
.short	0
.asciz	"s"
.p2align	2
.Ltmp316:
.cv_def_range	 .Ltmp24 .Ltmp32 .Ltmp36 .Lfunc_end4, subfield_reg, 342, 0
.cv_def_range	 .Ltmp24 .Ltmp32 .Ltmp36 .Lfunc_end4, subfield_reg, 333, 8
.short	.Ltmp318-.Ltmp317
.Ltmp317:
.short	4429
.long	0
.long	0
.long	4453
.cv_inline_linetable	28 4 672 .Lfunc_begin4 .Lfunc_end4
.p2align	2
.Ltmp318:
.short	.Ltmp320-.Ltmp319
.Ltmp319:
.short	4414
.long	35
.short	0
.asciz	"capacity"
.p2align	2
.Ltmp320:
.cv_def_range	 .Ltmp24 .Ltmp30 .Ltmp36 .Lfunc_end4, reg, 333
.short	.Ltmp322-.Ltmp321
.Ltmp321:
.short	4429
.long	0
.long	0
.long	4455
.cv_inline_linetable	29 2 130 .Lfunc_begin4 .Lfunc_end4
.p2align	2
.Ltmp322:
.short	.Ltmp324-.Ltmp323
.Ltmp323:
.short	4414
.long	35
.short	0
.asciz	"capacity"
.p2align	2
.Ltmp324:
.cv_def_range	 .Ltmp24 .Ltmp30 .Ltmp36 .Lfunc_end4, reg, 333
.short	.Ltmp326-.Ltmp325
.Ltmp325:
.short	4429
.long	0
.long	0
.long	4461
.cv_inline_linetable	30 2 169 .Lfunc_begin4 .Lfunc_end4
.p2align	2
.Ltmp326:
.short	.Ltmp328-.Ltmp327
.Ltmp327:
.short	4414
.long	35
.short	1
.asciz	"capacity"
.p2align	2
.Ltmp328:
.cv_def_range	 .Ltmp24 .Ltmp30 .Ltmp36 .Lfunc_end4, reg, 333
.short	.Ltmp330-.Ltmp329
.Ltmp329:
.short	4414
.long	4457
.short	257
.asciz	"init"
.p2align	2
.Ltmp330:
.short	.Ltmp332-.Ltmp331
.Ltmp331:
.short	4414
.long	4257
.short	0
.asciz	"layout"
.p2align	2
.Ltmp332:
.cv_def_range	 .Ltmp26 .Ltmp30 .Ltmp38 .Lfunc_end4, subfield_reg, 333, 0
.short	.Ltmp334-.Ltmp333
.Ltmp333:
.short	4414
.long	4468
.short	0
.asciz	"result"
.p2align	2
.Ltmp334:
.cv_def_range	 .Ltmp28 .Ltmp30 .Ltmp38 .Ltmp39, subfield_reg, 328, 0
.short	.Ltmp336-.Ltmp335
.Ltmp335:
.short	4429
.long	0
.long	0
.long	4496
.cv_inline_linetable	31 7 240 .Lfunc_begin4 .Lfunc_end4
.p2align	2
.Ltmp336:
.short	.Ltmp338-.Ltmp337
.Ltmp337:
.short	4414
.long	4299
.short	257
.asciz	"self"
.p2align	2
.Ltmp338:
.short	.Ltmp340-.Ltmp339
.Ltmp339:
.short	4414
.long	4257
.short	0
.asciz	"layout"
.p2align	2
.Ltmp340:
.cv_def_range	 .Ltmp26 .Ltmp30, subfield_reg, 333, 0
.short	.Ltmp342-.Ltmp341
.Ltmp341:
.short	4429
.long	0
.long	0
.long	4499
.cv_inline_linetable	32 7 176 .Lfunc_begin4 .Lfunc_end4
.p2align	2
.Ltmp342:
.short	.Ltmp344-.Ltmp343
.Ltmp343:
.short	4414
.long	4299
.short	257
.asciz	"self"
.p2align	2
.Ltmp344:
.short	.Ltmp346-.Ltmp345
.Ltmp345:
.short	4414
.long	48
.short	257
.asciz	"zeroed"
.p2align	2
.Ltmp346:
.short	.Ltmp348-.Ltmp347
.Ltmp347:
.short	4414
.long	4257
.short	0
.asciz	"layout"
.p2align	2
.Ltmp348:
.cv_def_range	 .Ltmp26 .Ltmp30, subfield_reg, 333, 0
.short	.Ltmp350-.Ltmp349
.Ltmp349:
.short	4414
.long	35
.short	0
.asciz	"size"
.p2align	2
.Ltmp350:
.cv_def_range	 .Ltmp26 .Ltmp30, reg, 333
.short	.Ltmp352-.Ltmp351
.Ltmp351:
.short	4429
.long	0
.long	0
.long	4502
.cv_inline_linetable	33 7 98 .Lfunc_begin4 .Lfunc_end4
.p2align	2
.Ltmp352:
.short	.Ltmp354-.Ltmp353
.Ltmp353:
.short	4414
.long	4257
.short	0
.asciz	"layout"
.p2align	2
.Ltmp354:
.cv_def_range	 .Ltmp26 .Ltmp30, subfield_reg, 333, 0
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	.Ltmp356-.Ltmp355
.Ltmp355:
.short	4429
.long	0
.long	0
.long	4506
.cv_inline_linetable	34 14 1259 .Lfunc_begin4 .Lfunc_end4
.p2align	2
.Ltmp356:
.short	.Ltmp358-.Ltmp357
.Ltmp357:
.short	4414
.long	1568
.short	256
.asciz	"self"
.p2align	2
.Ltmp358:
.short	.Ltmp360-.Ltmp359
.Ltmp359:
.short	4414
.long	35
.short	256
.asciz	"count"
.p2align	2
.Ltmp360:
.short	.Ltmp362-.Ltmp361
.Ltmp361:
.short	4414
.long	1568
.short	0
.asciz	"dest"
.p2align	2
.Ltmp362:
.cv_def_range	 .Ltmp33 .Ltmp35, reg, 329
.short	.Ltmp364-.Ltmp363
.Ltmp363:
.short	4429
.long	0
.long	0
.long	4508
.cv_inline_linetable	35 13 2412 .Lfunc_begin4 .Lfunc_end4
.p2align	2
.Ltmp364:
.short	.Ltmp366-.Ltmp365
.Ltmp365:
.short	4414
.long	1568
.short	256
.asciz	"src"
.p2align	2
.Ltmp366:
.short	.Ltmp368-.Ltmp367
.Ltmp367:
.short	4414
.long	35
.short	256
.asciz	"count"
.p2align	2
.Ltmp368:
.short	.Ltmp370-.Ltmp369
.Ltmp369:
.short	4414
.long	1568
.short	0
.asciz	"dst"
.p2align	2
.Ltmp370:
.cv_def_range	 .Ltmp33 .Ltmp35, reg, 329
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	.Ltmp372-.Ltmp371
.Ltmp371:
.short	4429
.long	0
.long	0
.long	4511
.cv_inline_linetable	36 3 845 .Lfunc_begin4 .Lfunc_end4
.p2align	2
.Ltmp372:
.short	.Ltmp374-.Ltmp373
.Ltmp373:
.short	4414
.long	4135
.short	0
.asciz	"bytes"
.p2align	2
.Ltmp374:
.cv_def_range	 .Ltmp33 .Ltmp35, subfield_reg, 329, 0
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4431
.Ltmp262:
.p2align	2
.cv_linetable	17, _ZN5alloc3fmt6format17h96a775549426137dE, .Lfunc_end4
.section	.debug$S,"dr",associative,_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17hd07e1a8f740041e6E
.p2align	2
.long	4
.long	241
.long	.Ltmp376-.Ltmp375
.Ltmp375:
.short	.Ltmp378-.Ltmp377
.Ltmp377:
.short	4422
.long	0
.long	0
.long	0
.long	.Lfunc_end5-_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17hd07e1a8f740041e6E
.long	0
.long	0
.long	4586
.secrel32	_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17hd07e1a8f740041e6E
.secidx	_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17hd07e1a8f740041e6E
.byte	0
.asciz	"alloc::string::impl$22::fmt"
.p2align	2
.Ltmp378:
.short	.Ltmp380-.Ltmp379
.Ltmp379:
.short	4114
.long	0
.long	0
.long	0
.long	0
.long	0
.short	0
.long	1048608
.p2align	2
.Ltmp380:
.short	.Ltmp382-.Ltmp381
.Ltmp381:
.short	4414
.long	4099
.short	1
.asciz	"self"
.p2align	2
.Ltmp382:
.cv_def_range	 .Lfunc_begin5 .Ltmp44, reg, 330
.short	.Ltmp384-.Ltmp383
.Ltmp383:
.short	4414
.long	4101
.short	1
.asciz	"f"
.p2align	2
.Ltmp384:
.cv_def_range	 .Lfunc_begin5 .Ltmp43, reg, 331
.cv_def_range	 .Ltmp43 .Ltmp45, reg, 336
.short	.Ltmp386-.Ltmp385
.Ltmp385:
.short	4429
.long	0
.long	0
.long	4205
.cv_inline_linetable	38 3 2459 .Lfunc_begin5 .Lfunc_end5
.p2align	2
.Ltmp386:
.short	.Ltmp388-.Ltmp387
.Ltmp387:
.short	4414
.long	4099
.short	0
.asciz	"self"
.p2align	2
.Ltmp388:
.cv_def_range	 .Lfunc_begin5 .Ltmp44, reg, 330
.short	.Ltmp390-.Ltmp389
.Ltmp389:
.short	4429
.long	0
.long	0
.long	4213
.cv_inline_linetable	39 4 2640 .Lfunc_begin5 .Lfunc_end5
.p2align	2
.Ltmp390:
.short	.Ltmp392-.Ltmp391
.Ltmp391:
.short	4414
.long	4208
.short	1
.asciz	"self"
.p2align	2
.Ltmp392:
.cv_def_range	 .Lfunc_begin5 .Ltmp44, reg, 330
.short	.Ltmp394-.Ltmp393
.Ltmp393:
.short	4429
.long	0
.long	0
.long	4216
.cv_inline_linetable	40 4 1234 .Lfunc_begin5 .Lfunc_end5
.p2align	2
.Ltmp394:
.short	.Ltmp396-.Ltmp395
.Ltmp395:
.short	4414
.long	4208
.short	0
.asciz	"self"
.p2align	2
.Ltmp396:
.cv_def_range	 .Lfunc_begin5 .Ltmp44, reg, 330
.short	.Ltmp398-.Ltmp397
.Ltmp397:
.short	4429
.long	0
.long	0
.long	4219
.cv_inline_linetable	41 2 223 .Lfunc_begin5 .Lfunc_end5
.p2align	2
.Ltmp398:
.short	.Ltmp400-.Ltmp399
.Ltmp399:
.short	4414
.long	4217
.short	0
.asciz	"self"
.p2align	2
.Ltmp400:
.cv_def_range	 .Lfunc_begin5 .Ltmp44, reg, 330
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4431
.Ltmp376:
.p2align	2
.cv_linetable	37, _ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17hd07e1a8f740041e6E, .Lfunc_end5
.section	.debug$S,"dr",associative,_ZN20arena_latency_slider15set_text_string17hc4197c5838b7032fE
.p2align	2
.long	4
.long	241
.long	.Ltmp402-.Ltmp401
.Ltmp401:
.short	.Ltmp404-.Ltmp403
.Ltmp403:
.short	4422
.long	0
.long	0
.long	0
.long	.Lfunc_end6-_ZN20arena_latency_slider15set_text_string17hc4197c5838b7032fE
.long	0
.long	0
.long	4590
.secrel32	_ZN20arena_latency_slider15set_text_string17hc4197c5838b7032fE
.secidx	_ZN20arena_latency_slider15set_text_string17hc4197c5838b7032fE
.byte	0
.asciz	"arena_latency_slider::set_text_string"
.p2align	2
.Ltmp404:
.short	.Ltmp406-.Ltmp405
.Ltmp405:
.short	4114
.long	40
.long	0
.long	0
.long	16
.long	0
.short	0
.long	1130496
.p2align	2
.Ltmp406:
.short	.Ltmp408-.Ltmp407
.Ltmp407:
.short	4414
.long	35
.short	1
.asciz	"pane"
.p2align	2
.Ltmp408:
.cv_def_range	 .Lfunc_begin6 .Ltmp47 .Ltmp50 .Ltmp52, reg, 330
.cv_def_range	 .Ltmp47 .Ltmp50, reg, 333
.short	.Ltmp410-.Ltmp409
.Ltmp409:
.short	4414
.long	1568
.short	1
.asciz	"string"
.p2align	2
.Ltmp410:
.cv_def_range	 .Lfunc_begin6 .Ltmp48 .Ltmp51 .Ltmp52, reg, 331
.cv_def_range	 .Ltmp48 .Ltmp51, reg, 332
.short	.Ltmp412-.Ltmp411
.Ltmp411:
.short	4355
.long	0
.long	0
.long	.Ltmp52-.Ltmp49
.secrel32	.Ltmp49
.secidx	.Lfunc_begin6
.byte	0
.p2align	2
.Ltmp412:
.short	.Ltmp414-.Ltmp413
.Ltmp413:
.short	4414
.long	4591
.short	0
.asciz	"inner"
.p2align	2
.Ltmp414:
.cv_def_range	 .Ltmp49 .Ltmp52, reg, 328
.short	2
.short	6
.short	.Ltmp416-.Ltmp415
.Ltmp415:
.short	4429
.long	0
.long	0
.long	4514
.cv_inline_linetable	43 14 456 .Lfunc_begin6 .Lfunc_end6
.p2align	2
.Ltmp416:
.short	.Ltmp418-.Ltmp417
.Ltmp417:
.short	4414
.long	1568
.short	1
.asciz	"self"
.p2align	2
.Ltmp418:
.cv_def_range	 .Ltmp48 .Ltmp49, reg, 328
.short	.Ltmp420-.Ltmp419
.Ltmp419:
.short	4414
.long	19
.short	257
.asciz	"count"
.p2align	2
.Ltmp420:
.short	2
.short	4430
.short	2
.short	4431
.Ltmp402:
.p2align	2
.cv_linetable	42, _ZN20arena_latency_slider15set_text_string17hc4197c5838b7032fE, .Lfunc_end6
.section	.debug$S,"dr",associative,non_hdr_update_room_hook
.p2align	2
.long	4
.long	241
.long	.Ltmp422-.Ltmp421
.Ltmp421:
.short	.Ltmp424-.Ltmp423
.Ltmp423:
.short	4423
.long	0
.long	0
.long	0
.long	.Lfunc_end7-non_hdr_update_room_hook
.long	0
.long	0
.long	4618
.secrel32	non_hdr_update_room_hook
.secidx	non_hdr_update_room_hook
.byte	0
.asciz	"arena_latency_slider::non_hdr_update_room_hook"
.p2align	2
.Ltmp424:
.short	.Ltmp426-.Ltmp425
.Ltmp425:
.short	4114
.long	144
.long	0
.long	0
.long	24
.long	0
.short	0
.long	1130512
.p2align	2
.Ltmp426:
.short	.Ltmp428-.Ltmp427
.Ltmp427:
.short	4414
.long	4593
.short	1
.byte	0
.p2align	2
.Ltmp428:
.cv_def_range	 .Lfunc_begin7 .Ltmp53, reg, 330
.short	.Ltmp430-.Ltmp429
.Ltmp429:
.short	4355
.long	0
.long	0
.long	.Ltmp83-.Ltmp82
.secrel32	.Ltmp82
.secidx	.Lfunc_begin7
.byte	0
.p2align	2
.Ltmp430:
.short	.Ltmp432-.Ltmp431
.Ltmp431:
.short	4414
.long	4112
.short	0
.asciz	"res"
.p2align	2
.Ltmp432:
.cv_def_range	 .Ltmp82 .Ltmp83, frame_ptr_rel, 40
.short	2
.short	6
.short	.Ltmp434-.Ltmp433
.Ltmp433:
.short	4429
.long	0
.long	0
.long	4518
.cv_inline_linetable	45 16 854 .Lfunc_begin7 .Lfunc_end7
.p2align	2
.Ltmp434:
.short	.Ltmp436-.Ltmp435
.Ltmp435:
.short	4414
.long	19
.short	257
.asciz	"self"
.p2align	2
.Ltmp436:
.short	.Ltmp438-.Ltmp437
.Ltmp437:
.short	4414
.long	19
.short	257
.asciz	"min"
.p2align	2
.Ltmp438:
.short	.Ltmp440-.Ltmp439
.Ltmp439:
.short	4414
.long	19
.short	257
.asciz	"max"
.p2align	2
.Ltmp440:
.short	2
.short	4430
.short	.Ltmp442-.Ltmp441
.Ltmp441:
.short	4429
.long	0
.long	0
.long	4522
.cv_inline_linetable	46 8 608 .Lfunc_begin7 .Lfunc_end7
.p2align	2
.Ltmp442:
.short	.Ltmp444-.Ltmp443
.Ltmp443:
.short	4414
.long	4312
.short	1
.asciz	"args"
.p2align	2
.Ltmp444:
.cv_def_range	 .Ltmp67 .Ltmp68, subfield_reg, 328, 0
.short	.Ltmp446-.Ltmp445
.Ltmp445:
.short	4429
.long	0
.long	0
.long	4427
.cv_inline_linetable	47 9 1010 .Lfunc_begin7 .Lfunc_end7
.p2align	2
.Ltmp446:
.short	.Ltmp448-.Ltmp447
.Ltmp447:
.short	4414
.long	4425
.short	257
.asciz	"default"
.p2align	2
.Ltmp448:
.short	.Ltmp450-.Ltmp449
.Ltmp449:
.short	4429
.long	0
.long	0
.long	4431
.cv_inline_linetable	48 8 616 .Lfunc_begin7 .Lfunc_end7
.p2align	2
.Ltmp450:
.short	.Ltmp452-.Ltmp451
.Ltmp451:
.short	4414
.long	4312
.short	256
.asciz	"args"
.p2align	2
.Ltmp452:
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	.Ltmp454-.Ltmp453
.Ltmp453:
.short	4429
.long	0
.long	0
.long	4522
.cv_inline_linetable	49 8 608 .Lfunc_begin7 .Lfunc_end7
.p2align	2
.Ltmp454:
.short	.Ltmp456-.Ltmp455
.Ltmp455:
.short	4414
.long	4312
.short	1
.asciz	"args"
.p2align	2
.Ltmp456:
.cv_def_range	 .Ltmp71 .Ltmp71, subfield_reg, 328, 0
.short	.Ltmp458-.Ltmp457
.Ltmp457:
.short	4429
.long	0
.long	0
.long	4427
.cv_inline_linetable	50 9 1010 .Lfunc_begin7 .Lfunc_end7
.p2align	2
.Ltmp458:
.short	.Ltmp460-.Ltmp459
.Ltmp459:
.short	4414
.long	4425
.short	257
.asciz	"default"
.p2align	2
.Ltmp460:
.short	.Ltmp462-.Ltmp461
.Ltmp461:
.short	4429
.long	0
.long	0
.long	4431
.cv_inline_linetable	51 8 616 .Lfunc_begin7 .Lfunc_end7
.p2align	2
.Ltmp462:
.short	.Ltmp464-.Ltmp463
.Ltmp463:
.short	4414
.long	4312
.short	256
.asciz	"args"
.p2align	2
.Ltmp464:
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	.Ltmp466-.Ltmp465
.Ltmp465:
.short	4429
.long	0
.long	0
.long	4522
.cv_inline_linetable	52 8 608 .Lfunc_begin7 .Lfunc_end7
.p2align	2
.Ltmp466:
.short	.Ltmp468-.Ltmp467
.Ltmp467:
.short	4414
.long	4312
.short	1
.asciz	"args"
.p2align	2
.Ltmp468:
.cv_def_range	 .Ltmp80 .Ltmp81, subfield_reg, 328, 0
.short	.Ltmp470-.Ltmp469
.Ltmp469:
.short	4429
.long	0
.long	0
.long	4427
.cv_inline_linetable	53 9 1010 .Lfunc_begin7 .Lfunc_end7
.p2align	2
.Ltmp470:
.short	.Ltmp472-.Ltmp471
.Ltmp471:
.short	4414
.long	4425
.short	257
.asciz	"default"
.p2align	2
.Ltmp472:
.short	.Ltmp474-.Ltmp473
.Ltmp473:
.short	4429
.long	0
.long	0
.long	4431
.cv_inline_linetable	54 8 616 .Lfunc_begin7 .Lfunc_end7
.p2align	2
.Ltmp474:
.short	.Ltmp476-.Ltmp475
.Ltmp475:
.short	4414
.long	4312
.short	256
.asciz	"args"
.p2align	2
.Ltmp476:
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4431
.Ltmp422:
.p2align	2
.cv_linetable	44, non_hdr_update_room_hook, .Lfunc_end7
.section	.debug$S,"dr",associative,non_hdr_set_room_id
.p2align	2
.long	4
.long	241
.long	.Ltmp478-.Ltmp477
.Ltmp477:
.short	.Ltmp480-.Ltmp479
.Ltmp479:
.short	4423
.long	0
.long	0
.long	0
.long	.Lfunc_end8-non_hdr_set_room_id
.long	0
.long	0
.long	4619
.secrel32	non_hdr_set_room_id
.secidx	non_hdr_set_room_id
.byte	0
.asciz	"arena_latency_slider::non_hdr_set_room_id"
.p2align	2
.Ltmp480:
.short	.Ltmp482-.Ltmp481
.Ltmp481:
.short	4114
.long	240
.long	0
.long	0
.long	8
.long	0
.short	0
.long	1130512
.p2align	2
.Ltmp482:
.short	.Ltmp484-.Ltmp483
.Ltmp483:
.short	4414
.long	4593
.short	1
.asciz	"ctx"
.p2align	2
.Ltmp484:
.cv_def_range	 .Lfunc_begin8 .Ltmp85, reg, 330
.short	.Ltmp486-.Ltmp485
.Ltmp485:
.short	4414
.long	35
.short	0
.asciz	"panel"
.p2align	2
.Ltmp486:
.cv_def_range	 .Ltmp84 .Ltmp86, reg, 328
.short	.Ltmp488-.Ltmp487
.Ltmp487:
.short	4355
.long	0
.long	0
.long	.Ltmp101-.Ltmp89
.secrel32	.Ltmp89
.secidx	.Lfunc_begin8
.byte	0
.p2align	2
.Ltmp488:
.short	.Ltmp490-.Ltmp489
.Ltmp489:
.short	4414
.long	4112
.short	0
.asciz	"tmp"
.p2align	2
.Ltmp490:
.cv_def_range	 .Ltmp89 .Ltmp101, frame_ptr_rel, 128
.short	2
.short	6
.short	.Ltmp492-.Ltmp491
.Ltmp491:
.short	4429
.long	0
.long	0
.long	4556
.cv_inline_linetable	56 17 1107 .Lfunc_begin8 .Lfunc_end8
.p2align	2
.Ltmp492:
.short	.Ltmp494-.Ltmp493
.Ltmp493:
.short	4414
.long	4527
.short	1
.asciz	"self"
.p2align	2
.Ltmp494:
.cv_def_range	 .Ltmp86 .Ltmp88 .Ltmp108 .Ltmp109, frame_ptr_rel, 48
.short	.Ltmp496-.Ltmp495
.Ltmp495:
.short	4414
.long	4547
.short	0
.asciz	"e"
.p2align	2
.Ltmp496:
.cv_def_range	 .Ltmp108 .Ltmp109, frame_ptr_rel, 160
.short	2
.short	4430
.short	.Ltmp498-.Ltmp497
.Ltmp497:
.short	4429
.long	0
.long	0
.long	4563
.cv_inline_linetable	57 1 414 .Lfunc_begin8 .Lfunc_end8
.p2align	2
.Ltmp498:
.short	.Ltmp500-.Ltmp499
.Ltmp499:
.short	4414
.long	4316
.short	0
.asciz	"pieces"
.p2align	2
.Ltmp500:
.cv_def_range	 .Ltmp97 .Ltmp98, subfield_reg, 328, 0
.short	.Ltmp502-.Ltmp501
.Ltmp501:
.short	4414
.long	4326
.short	256
.asciz	"args"
.p2align	2
.Ltmp502:
.short	.Ltmp504-.Ltmp503
.Ltmp503:
.short	4414
.long	4357
.short	0
.asciz	"fmt"
.p2align	2
.Ltmp504:
.cv_def_range	 .Ltmp98 .Ltmp100, subfield_reg, 328, 0
.short	2
.short	4430
.short	.Ltmp506-.Ltmp505
.Ltmp505:
.short	4429
.long	0
.long	0
.long	4565
.cv_inline_linetable	58 5 490 .Lfunc_begin8 .Lfunc_end8
.p2align	2
.Ltmp506:
.short	.Ltmp508-.Ltmp507
.Ltmp507:
.short	4414
.long	4099
.short	257
.byte	0
.p2align	2
.Ltmp508:
.short	.Ltmp510-.Ltmp509
.Ltmp509:
.short	4429
.long	0
.long	0
.long	4222
.cv_inline_linetable	59 5 490 .Lfunc_begin8 .Lfunc_end8
.p2align	2
.Ltmp510:
.short	.Ltmp512-.Ltmp511
.Ltmp511:
.short	4414
.long	4208
.short	257
.byte	0
.p2align	2
.Ltmp512:
.short	.Ltmp514-.Ltmp513
.Ltmp513:
.short	4429
.long	0
.long	0
.long	4225
.cv_inline_linetable	60 5 490 .Lfunc_begin8 .Lfunc_end8
.p2align	2
.Ltmp514:
.short	.Ltmp516-.Ltmp515
.Ltmp515:
.short	4414
.long	4217
.short	257
.byte	0
.p2align	2
.Ltmp516:
.short	.Ltmp518-.Ltmp517
.Ltmp517:
.short	4429
.long	0
.long	0
.long	4227
.cv_inline_linetable	61 2 477 .Lfunc_begin8 .Lfunc_end8
.p2align	2
.Ltmp518:
.short	.Ltmp520-.Ltmp519
.Ltmp519:
.short	4414
.long	4217
.short	257
.asciz	"self"
.p2align	2
.Ltmp520:
.short	.Ltmp522-.Ltmp521
.Ltmp521:
.short	4414
.long	4257
.short	0
.asciz	"layout"
.p2align	2
.Ltmp522:
.cv_def_range	 .Ltmp104 .Ltmp106, subfield_reg, 336, 8
.cv_def_range	 .Ltmp105 .Ltmp106, subfield_reg, 331, 0
.short	.Ltmp524-.Ltmp523
.Ltmp523:
.short	4414
.long	4193
.short	0
.asciz	"ptr"
.p2align	2
.Ltmp524:
.cv_def_range	 .Ltmp105 .Ltmp106, reg, 330
.short	.Ltmp526-.Ltmp525
.Ltmp525:
.short	4429
.long	0
.long	0
.long	4265
.cv_inline_linetable	62 2 240 .Lfunc_begin8 .Lfunc_end8
.p2align	2
.Ltmp526:
.short	.Ltmp528-.Ltmp527
.Ltmp527:
.short	4414
.long	4217
.short	257
.asciz	"self"
.p2align	2
.Ltmp528:
.short	.Ltmp530-.Ltmp529
.Ltmp529:
.short	4414
.long	4257
.short	0
.asciz	"layout"
.p2align	2
.Ltmp530:
.cv_def_range	 .Ltmp104 .Ltmp106, subfield_reg, 336, 8
.cv_def_range	 .Ltmp104 .Ltmp106, subfield_reg, 331, 0
.short	.Ltmp532-.Ltmp531
.Ltmp531:
.short	4429
.long	0
.long	0
.long	4293
.cv_inline_linetable	63 6 426 .Lfunc_begin8 .Lfunc_end8
.p2align	2
.Ltmp532:
.short	.Ltmp534-.Ltmp533
.Ltmp533:
.short	4414
.long	35
.short	0
.asciz	"n"
.p2align	2
.Ltmp534:
.cv_def_range	 .Ltmp103 .Ltmp106, reg, 331
.short	.Ltmp536-.Ltmp535
.Ltmp535:
.short	4429
.long	0
.long	0
.long	4297
.cv_inline_linetable	64 6 431 .Lfunc_begin8 .Lfunc_end8
.p2align	2
.Ltmp536:
.short	.Ltmp538-.Ltmp537
.Ltmp537:
.short	4414
.long	35
.short	257
.asciz	"element_size"
.p2align	2
.Ltmp538:
.short	.Ltmp540-.Ltmp539
.Ltmp539:
.short	4414
.long	4263
.short	257
.asciz	"align"
.p2align	2
.Ltmp540:
.short	.Ltmp542-.Ltmp541
.Ltmp541:
.short	4414
.long	35
.short	1
.asciz	"n"
.p2align	2
.Ltmp542:
.cv_def_range	 .Ltmp103 .Ltmp106, reg, 331
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	.Ltmp544-.Ltmp543
.Ltmp543:
.short	4429
.long	0
.long	0
.long	4302
.cv_inline_linetable	65 7 250 .Lfunc_begin8 .Lfunc_end8
.p2align	2
.Ltmp544:
.short	.Ltmp546-.Ltmp545
.Ltmp545:
.short	4414
.long	4299
.short	257
.asciz	"self"
.p2align	2
.Ltmp546:
.short	.Ltmp548-.Ltmp547
.Ltmp547:
.short	4414
.long	4193
.short	1
.asciz	"ptr"
.p2align	2
.Ltmp548:
.cv_def_range	 .Ltmp105 .Ltmp106, reg, 330
.short	.Ltmp550-.Ltmp549
.Ltmp549:
.short	4414
.long	4257
.short	0
.asciz	"layout"
.p2align	2
.Ltmp550:
.cv_def_range	 .Ltmp104 .Ltmp106, subfield_reg, 336, 8
.cv_def_range	 .Ltmp105 .Ltmp106, subfield_reg, 331, 0
.short	.Ltmp552-.Ltmp551
.Ltmp551:
.short	4429
.long	0
.long	0
.long	4306
.cv_inline_linetable	66 7 116 .Lfunc_begin8 .Lfunc_end8
.p2align	2
.Ltmp552:
.short	.Ltmp554-.Ltmp553
.Ltmp553:
.short	4414
.long	4257
.short	0
.asciz	"layout"
.p2align	2
.Ltmp554:
.cv_def_range	 .Ltmp104 .Ltmp106, subfield_reg, 336, 8
.cv_def_range	 .Ltmp105 .Ltmp106, subfield_reg, 331, 0
.short	.Ltmp556-.Ltmp555
.Ltmp555:
.short	4414
.long	1568
.short	0
.asciz	"ptr"
.p2align	2
.Ltmp556:
.cv_def_range	 .Ltmp105 .Ltmp106, reg, 330
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4431
.Ltmp478:
.p2align	2
.cv_linetable	55, non_hdr_set_room_id, .Lfunc_end8
.section	.debug$S,"dr",associative,non_hdr_update_css2
.p2align	2
.long	4
.long	241
.long	.Ltmp558-.Ltmp557
.Ltmp557:
.short	.Ltmp560-.Ltmp559
.Ltmp559:
.short	4423
.long	0
.long	0
.long	0
.long	.Lfunc_end9-non_hdr_update_css2
.long	0
.long	0
.long	4621
.secrel32	non_hdr_update_css2
.secidx	non_hdr_update_css2
.byte	0
.asciz	"arena_latency_slider::non_hdr_update_css2"
.p2align	2
.Ltmp560:
.short	.Ltmp562-.Ltmp561
.Ltmp561:
.short	4114
.long	128
.long	0
.long	0
.long	56
.long	0
.short	0
.long	1130512
.p2align	2
.Ltmp562:
.short	.Ltmp564-.Ltmp563
.Ltmp563:
.short	4414
.long	35
.short	1
.asciz	"arg"
.p2align	2
.Ltmp564:
.cv_def_range	 .Lfunc_begin9 .Ltmp111, reg, 330
.cv_def_range	 .Ltmp111 .Ltmp130, reg, 332
.short	.Ltmp566-.Ltmp565
.Ltmp565:
.short	4429
.long	0
.long	0
.long	4518
.cv_inline_linetable	68 16 854 .Lfunc_begin9 .Lfunc_end9
.p2align	2
.Ltmp566:
.short	.Ltmp568-.Ltmp567
.Ltmp567:
.short	4414
.long	19
.short	1
.asciz	"self"
.p2align	2
.Ltmp568:
.cv_def_range	 .Ltmp121 .Ltmp123, reg, 328
.short	.Ltmp570-.Ltmp569
.Ltmp569:
.short	4414
.long	19
.short	257
.asciz	"min"
.p2align	2
.Ltmp570:
.short	.Ltmp572-.Ltmp571
.Ltmp571:
.short	4414
.long	19
.short	257
.asciz	"max"
.p2align	2
.Ltmp572:
.short	2
.short	4430
.short	.Ltmp574-.Ltmp573
.Ltmp573:
.short	4429
.long	0
.long	0
.long	4568
.cv_inline_linetable	69 1 394 .Lfunc_begin9 .Lfunc_end9
.p2align	2
.Ltmp574:
.short	.Ltmp576-.Ltmp575
.Ltmp575:
.short	4414
.long	4316
.short	0
.asciz	"pieces"
.p2align	2
.Ltmp576:
.cv_def_range	 .Ltmp126 .Lfunc_end9, subfield_reg, 340, 0
.short	.Ltmp578-.Ltmp577
.Ltmp577:
.short	4414
.long	4326
.short	256
.asciz	"args"
.p2align	2
.Ltmp578:
.short	2
.short	4430
.short	.Ltmp580-.Ltmp579
.Ltmp579:
.short	4429
.long	0
.long	0
.long	4205
.cv_inline_linetable	70 3 2459 .Lfunc_begin9 .Lfunc_end9
.p2align	2
.Ltmp580:
.short	.Ltmp582-.Ltmp581
.Ltmp581:
.short	4414
.long	4099
.short	257
.asciz	"self"
.p2align	2
.Ltmp582:
.short	.Ltmp584-.Ltmp583
.Ltmp583:
.short	4429
.long	0
.long	0
.long	4213
.cv_inline_linetable	71 4 2640 .Lfunc_begin9 .Lfunc_end9
.p2align	2
.Ltmp584:
.short	.Ltmp586-.Ltmp585
.Ltmp585:
.short	4414
.long	4208
.short	257
.asciz	"self"
.p2align	2
.Ltmp586:
.short	.Ltmp588-.Ltmp587
.Ltmp587:
.short	4429
.long	0
.long	0
.long	4216
.cv_inline_linetable	72 4 1234 .Lfunc_begin9 .Lfunc_end9
.p2align	2
.Ltmp588:
.short	.Ltmp590-.Ltmp589
.Ltmp589:
.short	4414
.long	4208
.short	256
.asciz	"self"
.p2align	2
.Ltmp590:
.short	.Ltmp592-.Ltmp591
.Ltmp591:
.short	4429
.long	0
.long	0
.long	4219
.cv_inline_linetable	73 2 223 .Lfunc_begin9 .Lfunc_end9
.p2align	2
.Ltmp592:
.short	.Ltmp594-.Ltmp593
.Ltmp593:
.short	4414
.long	4217
.short	256
.asciz	"self"
.p2align	2
.Ltmp594:
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	.Ltmp596-.Ltmp595
.Ltmp595:
.short	4429
.long	0
.long	0
.long	4568
.cv_inline_linetable	74 1 394 .Lfunc_begin9 .Lfunc_end9
.p2align	2
.Ltmp596:
.short	.Ltmp598-.Ltmp597
.Ltmp597:
.short	4414
.long	4316
.short	0
.asciz	"pieces"
.p2align	2
.Ltmp598:
.cv_def_range	 .Ltmp131 .Lfunc_end9, subfield_reg, 340, 0
.short	.Ltmp600-.Ltmp599
.Ltmp599:
.short	4414
.long	4326
.short	256
.asciz	"args"
.p2align	2
.Ltmp600:
.short	2
.short	4430
.short	.Ltmp602-.Ltmp601
.Ltmp601:
.short	4429
.long	0
.long	0
.long	4205
.cv_inline_linetable	75 3 2459 .Lfunc_begin9 .Lfunc_end9
.p2align	2
.Ltmp602:
.short	.Ltmp604-.Ltmp603
.Ltmp603:
.short	4414
.long	4099
.short	257
.asciz	"self"
.p2align	2
.Ltmp604:
.short	.Ltmp606-.Ltmp605
.Ltmp605:
.short	4429
.long	0
.long	0
.long	4213
.cv_inline_linetable	76 4 2640 .Lfunc_begin9 .Lfunc_end9
.p2align	2
.Ltmp606:
.short	.Ltmp608-.Ltmp607
.Ltmp607:
.short	4414
.long	4208
.short	257
.asciz	"self"
.p2align	2
.Ltmp608:
.short	.Ltmp610-.Ltmp609
.Ltmp609:
.short	4429
.long	0
.long	0
.long	4216
.cv_inline_linetable	77 4 1234 .Lfunc_begin9 .Lfunc_end9
.p2align	2
.Ltmp610:
.short	.Ltmp612-.Ltmp611
.Ltmp611:
.short	4414
.long	4208
.short	256
.asciz	"self"
.p2align	2
.Ltmp612:
.short	.Ltmp614-.Ltmp613
.Ltmp613:
.short	4429
.long	0
.long	0
.long	4219
.cv_inline_linetable	78 2 223 .Lfunc_begin9 .Lfunc_end9
.p2align	2
.Ltmp614:
.short	.Ltmp616-.Ltmp615
.Ltmp615:
.short	4414
.long	4217
.short	256
.asciz	"self"
.p2align	2
.Ltmp616:
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4430
.short	2
.short	4431
.Ltmp558:
.p2align	2
.cv_linetable	67, non_hdr_update_css2, .Lfunc_end9
.section	.debug$S,"dr",associative,non_hdr_set_online_latency
.p2align	2
.long	4
.long	241
.long	.Ltmp618-.Ltmp617
.Ltmp617:
.short	.Ltmp620-.Ltmp619
.Ltmp619:
.short	4423
.long	0
.long	0
.long	0
.long	.Lfunc_end10-non_hdr_set_online_latency
.long	0
.long	0
.long	4622
.secrel32	non_hdr_set_online_latency
.secidx	non_hdr_set_online_latency
.byte	0
.asciz	"arena_latency_slider::non_hdr_set_online_latency"
.p2align	2
.Ltmp620:
.short	.Ltmp622-.Ltmp621
.Ltmp621:
.short	4114
.long	0
.long	0
.long	0
.long	0
.long	0
.short	0
.long	1048576
.p2align	2
.Ltmp622:
.short	.Ltmp624-.Ltmp623
.Ltmp623:
.short	4414
.long	4593
.short	1
.asciz	"ctx"
.p2align	2
.Ltmp624:
.cv_def_range	 .Lfunc_begin10 .Ltmp137, reg, 330
.short	.Ltmp626-.Ltmp625
.Ltmp625:
.short	4414
.long	32
.short	256
.asciz	"auto"
.p2align	2
.Ltmp626:
.short	2
.short	4431
.Ltmp618:
.p2align	2
.cv_linetable	79, non_hdr_set_online_latency, .Lfunc_end10
.section	.debug$S,"dr",associative,bg_matchmaking_seq
.p2align	2
.long	4
.long	241
.long	.Ltmp628-.Ltmp627
.Ltmp627:
.short	.Ltmp630-.Ltmp629
.Ltmp629:
.short	4423
.long	0
.long	0
.long	0
.long	.Lfunc_end11-bg_matchmaking_seq
.long	0
.long	0
.long	4623
.secrel32	bg_matchmaking_seq
.secidx	bg_matchmaking_seq
.byte	0
.asciz	"arena_latency_slider::bg_matchmaking_seq"
.p2align	2
.Ltmp630:
.short	.Ltmp632-.Ltmp631
.Ltmp631:
.short	4114
.long	0
.long	0
.long	0
.long	0
.long	0
.short	0
.long	1048576
.p2align	2
.Ltmp632:
.short	.Ltmp634-.Ltmp633
.Ltmp633:
.short	4414
.long	4593
.short	1
.byte	0
.p2align	2
.Ltmp634:
.cv_def_range	 .Lfunc_begin11 .Lfunc_end11, reg, 330
.short	2
.short	4431
.Ltmp628:
.p2align	2
.cv_linetable	80, bg_matchmaking_seq, .Lfunc_end11
.section	.debug$S,"dr",associative,arena_seq
.p2align	2
.long	4
.long	241
.long	.Ltmp636-.Ltmp635
.Ltmp635:
.short	.Ltmp638-.Ltmp637
.Ltmp637:
.short	4423
.long	0
.long	0
.long	0
.long	.Lfunc_end12-arena_seq
.long	0
.long	0
.long	4624
.secrel32	arena_seq
.secidx	arena_seq
.byte	0
.asciz	"arena_latency_slider::arena_seq"
.p2align	2
.Ltmp638:
.short	.Ltmp640-.Ltmp639
.Ltmp639:
.short	4114
.long	0
.long	0
.long	0
.long	0
.long	0
.short	0
.long	1048576
.p2align	2
.Ltmp640:
.short	.Ltmp642-.Ltmp641
.Ltmp641:
.short	4414
.long	4593
.short	1
.byte	0
.p2align	2
.Ltmp642:
.cv_def_range	 .Lfunc_begin12 .Lfunc_end12, reg, 330
.short	2
.short	4431
.Ltmp636:
.p2align	2
.cv_linetable	81, arena_seq, .Lfunc_end12
.section	.debug$S,"dr",associative,__pthread_mutex_lock
.p2align	2
.long	4
.long	241
.long	.Ltmp644-.Ltmp643
.Ltmp643:
.short	.Ltmp646-.Ltmp645
.Ltmp645:
.short	4423
.long	0
.long	0
.long	0
.long	.Lfunc_end13-__pthread_mutex_lock
.long	0
.long	0
.long	4644
.secrel32	__pthread_mutex_lock
.secidx	__pthread_mutex_lock
.byte	0
.asciz	"arena_latency_slider::_::closure$0::_skyline_internal_pthread_mutex_lock_shim"
.p2align	2
.Ltmp646:
.short	.Ltmp648-.Ltmp647
.Ltmp647:
.short	4114
.long	0
.long	0
.long	0
.long	0
.long	0
.short	0
.long	1048576
.p2align	2
.Ltmp648:
.short	.Ltmp650-.Ltmp649
.Ltmp649:
.short	4414
.long	4627
.short	1
.asciz	"lock"
.p2align	2
.Ltmp650:
.cv_def_range	 .Lfunc_begin13 .Ltmp143, reg, 330
.short	2
.short	4431
.Ltmp644:
.p2align	2
.cv_linetable	82, __pthread_mutex_lock, .Lfunc_end13
.section	.debug$S,"dr",associative,__pthread_key_create
.p2align	2
.long	4
.long	241
.long	.Ltmp652-.Ltmp651
.Ltmp651:
.short	.Ltmp654-.Ltmp653
.Ltmp653:
.short	4423
.long	0
.long	0
.long	0
.long	.Lfunc_end14-__pthread_key_create
.long	0
.long	0
.long	4654
.secrel32	__pthread_key_create
.secidx	__pthread_key_create
.byte	0
.asciz	"arena_latency_slider::_::closure$0::_skyline_internal_pthread_key_create_shim"
.p2align	2
.Ltmp654:
.short	.Ltmp656-.Ltmp655
.Ltmp655:
.short	4114
.long	0
.long	0
.long	0
.long	0
.long	0
.short	0
.long	1048576
.p2align	2
.Ltmp656:
.short	.Ltmp658-.Ltmp657
.Ltmp657:
.short	4414
.long	1653
.short	1
.asciz	"key"
.p2align	2
.Ltmp658:
.cv_def_range	 .Lfunc_begin14 .Ltmp144, reg, 330
.short	.Ltmp660-.Ltmp659
.Ltmp659:
.short	4414
.long	4649
.short	1
.asciz	"func"
.p2align	2
.Ltmp660:
.cv_def_range	 .Lfunc_begin14 .Ltmp144, reg, 331
.short	2
.short	4431
.Ltmp652:
.p2align	2
.cv_linetable	83, __pthread_key_create, .Lfunc_end14
.section	.debug$S,"dr",associative,__pthread_key_delete
.p2align	2
.long	4
.long	241
.long	.Ltmp662-.Ltmp661
.Ltmp661:
.short	.Ltmp664-.Ltmp663
.Ltmp663:
.short	4423
.long	0
.long	0
.long	0
.long	.Lfunc_end15-__pthread_key_delete
.long	0
.long	0
.long	4657
.secrel32	__pthread_key_delete
.secidx	__pthread_key_delete
.byte	0
.asciz	"arena_latency_slider::_::closure$0::_skyline_internal_pthread_key_delete_shim"
.p2align	2
.Ltmp664:
.short	.Ltmp666-.Ltmp665
.Ltmp665:
.short	4114
.long	0
.long	0
.long	0
.long	0
.long	0
.short	0
.long	1048576
.p2align	2
.Ltmp666:
.short	.Ltmp668-.Ltmp667
.Ltmp667:
.short	4414
.long	117
.short	1
.asciz	"key"
.p2align	2
.Ltmp668:
.cv_def_range	 .Lfunc_begin15 .Ltmp145, reg, 18
.short	2
.short	4431
.Ltmp662:
.p2align	2
.cv_linetable	84, __pthread_key_delete, .Lfunc_end15
.section	.debug$S,"dr",associative,__custom_fini
.p2align	2
.long	4
.long	241
.long	.Ltmp670-.Ltmp669
.Ltmp669:
.short	.Ltmp672-.Ltmp671
.Ltmp671:
.short	4423
.long	0
.long	0
.long	0
.long	.Lfunc_end16-__custom_fini
.long	0
.long	0
.long	4659
.secrel32	__custom_fini
.secidx	__custom_fini
.byte	0
.asciz	"arena_latency_slider::__custom_fini"
.p2align	2
.Ltmp672:
.short	.Ltmp674-.Ltmp673
.Ltmp673:
.short	4114
.long	0
.long	0
.long	0
.long	0
.long	0
.short	0
.long	1048576
.p2align	2
.Ltmp674:
.short	2
.short	4431
.Ltmp670:
.p2align	2
.cv_linetable	85, __custom_fini, .Lfunc_end16
.section	.debug$S,"dr"
.long	241
.long	.Ltmp676-.Ltmp675
.Ltmp675:
.short	.Ltmp678-.Ltmp677
.Ltmp677:
.short	4364
.long	4663
.secrel32	__unnamed_12
.secidx	__unnamed_12
.asciz	"impl$<alloc::string::FromUtf16Error, core::fmt::Debug>::vtable$"
.p2align	2
.Ltmp678:
.short	.Ltmp680-.Ltmp679
.Ltmp679:
.short	4364
.long	35
.secrel32	_ZN20arena_latency_slider19CURRENT_PANE_HANDLE17h2bb43f3630faa000E.0
.secidx	_ZN20arena_latency_slider19CURRENT_PANE_HANDLE17h2bb43f3630faa000E.0
.asciz	"arena_latency_slider::CURRENT_PANE_HANDLE"
.p2align	2
.Ltmp680:
.short	.Ltmp682-.Ltmp681
.Ltmp681:
.short	4364
.long	4112
.secrel32	_ZN20arena_latency_slider16CURRENT_ARENA_ID17h90041e33cf1e16dfE
.secidx	_ZN20arena_latency_slider16CURRENT_ARENA_ID17h90041e33cf1e16dfE
.asciz	"arena_latency_slider::CURRENT_ARENA_ID"
.p2align	2
.Ltmp682:
.short	.Ltmp684-.Ltmp683
.Ltmp683:
.short	4364
.long	19
.secrel32	_ZN20arena_latency_slider20CURRENT_INPUT_BUFFER17hcd1c017fa6563844E
.secidx	_ZN20arena_latency_slider20CURRENT_INPUT_BUFFER17hcd1c017fa6563844E
.asciz	"arena_latency_slider::CURRENT_INPUT_BUFFER"
.p2align	2
.Ltmp684:
.short	.Ltmp686-.Ltmp685
.Ltmp685:
.short	4364
.long	19
.secrel32	_ZN20arena_latency_slider16MOST_RECENT_AUTO17hb25b258bf2ddaad6E
.secidx	_ZN20arena_latency_slider16MOST_RECENT_AUTO17hb25b258bf2ddaad6E
.asciz	"arena_latency_slider::MOST_RECENT_AUTO"
.p2align	2
.Ltmp686:
.short	.Ltmp688-.Ltmp687
.Ltmp687:
.short	4364
.long	35
.secrel32	_ZN20arena_latency_slider24non_hdr_update_room_hook15CURRENT_COUNTER17h701e90c953f8ab41E.0
.secidx	_ZN20arena_latency_slider24non_hdr_update_room_hook15CURRENT_COUNTER17h701e90c953f8ab41E.0
.asciz	"arena_latency_slider::non_hdr_update_room_hook::CURRENT_COUNTER"
.p2align	2
.Ltmp688:
.short	.Ltmp690-.Ltmp689
.Ltmp689:
.short	4364
.long	35
.secrel32	_ZN20arena_latency_slider19non_hdr_update_css215CURRENT_COUNTER17ha6ae7ad1c78de36cE.0
.secidx	_ZN20arena_latency_slider19non_hdr_update_css215CURRENT_COUNTER17ha6ae7ad1c78de36cE.0
.asciz	"arena_latency_slider::non_hdr_update_css2::CURRENT_COUNTER"
.p2align	2
.Ltmp690:
.short	.Ltmp692-.Ltmp691
.Ltmp691:
.short	4364
.long	4668
.secrel32	_ZN20arena_latency_slider13__MODULE_NAME17ha529bc1e5e79fa04E
.secidx	_ZN20arena_latency_slider13__MODULE_NAME17ha529bc1e5e79fa04E
.asciz	"arena_latency_slider::__MODULE_NAME"
.p2align	2
.Ltmp692:
.short	.Ltmp694-.Ltmp693
.Ltmp693:
.short	4359
.long	4124
.byte	0x00, 0x00
.asciz	"enum2$<core::result::Result<tuple$<>,core::fmt::Error> >::Variant0::NAME"
.p2align	2
.Ltmp694:
.short	.Ltmp696-.Ltmp695
.Ltmp695:
.short	4359
.long	32
.byte	0x00, 0x00
.asciz	"enum2$<core::result::Result<tuple$<>,core::fmt::Error> >::Variant0::DISCR_EXACT"
.p2align	2
.Ltmp696:
.short	.Ltmp698-.Ltmp697
.Ltmp697:
.short	4359
.long	4124
.byte	0x01, 0x00
.asciz	"enum2$<core::result::Result<tuple$<>,core::fmt::Error> >::Variant1::NAME"
.p2align	2
.Ltmp698:
.short	.Ltmp700-.Ltmp699
.Ltmp699:
.short	4359
.long	32
.byte	0x01, 0x00
.asciz	"enum2$<core::result::Result<tuple$<>,core::fmt::Error> >::Variant1::DISCR_EXACT"
.p2align	2
.Ltmp700:
.short	.Ltmp702-.Ltmp701
.Ltmp701:
.short	4359
.long	4164
.byte	0x00, 0x00
.asciz	"enum2$<core::option::Option<usize> >::Variant0::NAME"
.p2align	2
.Ltmp702:
.short	.Ltmp704-.Ltmp703
.Ltmp703:
.short	4359
.long	35
.byte	0x00, 0x00
.asciz	"enum2$<core::option::Option<usize> >::Variant0::DISCR_EXACT"
.p2align	2
.Ltmp704:
.short	.Ltmp706-.Ltmp705
.Ltmp705:
.short	4359
.long	4164
.byte	0x01, 0x00
.asciz	"enum2$<core::option::Option<usize> >::Variant1::NAME"
.p2align	2
.Ltmp706:
.short	.Ltmp708-.Ltmp707
.Ltmp707:
.short	4359
.long	35
.byte	0x01, 0x00
.asciz	"enum2$<core::option::Option<usize> >::Variant1::DISCR_EXACT"
.p2align	2
.Ltmp708:
.short	.Ltmp710-.Ltmp709
.Ltmp709:
.short	4359
.long	4236
.byte	0x00, 0x00
.asciz	"enum2$<core::option::Option<tuple$<core::ptr::non_null::NonNull<u8>,core::alloc::layout::Layout> > >::Variant0::NAME"
.p2align	2
.Ltmp710:
.short	.Ltmp712-.Ltmp711
.Ltmp711:
.short	4359
.long	35
.byte	0x00, 0x00
.asciz	"enum2$<core::option::Option<tuple$<core::ptr::non_null::NonNull<u8>,core::alloc::layout::Layout> > >::Variant0::DISCR_EXACT"
.p2align	2
.Ltmp712:
.short	.Ltmp714-.Ltmp713
.Ltmp713:
.short	4359
.long	4236
.byte	0x01, 0x00
.asciz	"enum2$<core::option::Option<tuple$<core::ptr::non_null::NonNull<u8>,core::alloc::layout::Layout> > >::Variant1::NAME"
.p2align	2
.Ltmp714:
.short	.Ltmp716-.Ltmp715
.Ltmp715:
.short	4359
.long	35
.byte	0x01, 0x00
.asciz	"enum2$<core::option::Option<tuple$<core::ptr::non_null::NonNull<u8>,core::alloc::layout::Layout> > >::Variant1::DISCR_BEGIN"
.p2align	2
.Ltmp716:
.short	.Ltmp718-.Ltmp717
.Ltmp717:
.short	4359
.long	35
.byte	0x0a, 0x80, 0x00, 0x00
.byte	0x00, 0x00, 0x00, 0x00
.byte	0x00, 0x80
.asciz	"enum2$<core::option::Option<tuple$<core::ptr::non_null::NonNull<u8>,core::alloc::layout::Layout> > >::Variant1::DISCR_END"
.p2align	2
.Ltmp718:
.short	.Ltmp720-.Ltmp719
.Ltmp719:
.short	4359
.long	4275
.byte	0x00, 0x00
.asciz	"enum2$<core::result::Result<core::alloc::layout::Layout,core::alloc::layout::LayoutError> >::Variant0::NAME"
.p2align	2
.Ltmp720:
.short	.Ltmp722-.Ltmp721
.Ltmp721:
.short	4359
.long	35
.byte	0x01, 0x00
.asciz	"enum2$<core::result::Result<core::alloc::layout::Layout,core::alloc::layout::LayoutError> >::Variant0::DISCR_BEGIN"
.p2align	2
.Ltmp722:
.short	.Ltmp724-.Ltmp723
.Ltmp723:
.short	4359
.long	35
.byte	0x0a, 0x80, 0x00, 0x00
.byte	0x00, 0x00, 0x00, 0x00
.byte	0x00, 0x80
.asciz	"enum2$<core::result::Result<core::alloc::layout::Layout,core::alloc::layout::LayoutError> >::Variant0::DISCR_END"
.p2align	2
.Ltmp724:
.short	.Ltmp726-.Ltmp725
.Ltmp725:
.short	4359
.long	4275
.byte	0x01, 0x00
.asciz	"enum2$<core::result::Result<core::alloc::layout::Layout,core::alloc::layout::LayoutError> >::Variant1::NAME"
.p2align	2
.Ltmp726:
.short	.Ltmp728-.Ltmp727
.Ltmp727:
.short	4359
.long	35
.byte	0x00, 0x00
.asciz	"enum2$<core::result::Result<core::alloc::layout::Layout,core::alloc::layout::LayoutError> >::Variant1::DISCR_EXACT"
.p2align	2
.Ltmp728:
.short	.Ltmp730-.Ltmp729
.Ltmp729:
.short	4359
.long	4329
.byte	0x00, 0x00
.asciz	"enum2$<core::option::Option<ref$<slice2$<core::fmt::rt::v1::Argument> > > >::Variant0::NAME"
.p2align	2
.Ltmp730:
.short	.Ltmp732-.Ltmp731
.Ltmp731:
.short	4359
.long	35
.byte	0x00, 0x00
.asciz	"enum2$<core::option::Option<ref$<slice2$<core::fmt::rt::v1::Argument> > > >::Variant0::DISCR_EXACT"
.p2align	2
.Ltmp732:
.short	.Ltmp734-.Ltmp733
.Ltmp733:
.short	4359
.long	4329
.byte	0x01, 0x00
.asciz	"enum2$<core::option::Option<ref$<slice2$<core::fmt::rt::v1::Argument> > > >::Variant1::NAME"
.p2align	2
.Ltmp734:
.short	.Ltmp736-.Ltmp735
.Ltmp735:
.short	4359
.long	35
.byte	0x01, 0x00
.asciz	"enum2$<core::option::Option<ref$<slice2$<core::fmt::rt::v1::Argument> > > >::Variant1::DISCR_BEGIN"
.p2align	2
.Ltmp736:
.short	.Ltmp738-.Ltmp737
.Ltmp737:
.short	4359
.long	35
.byte	0x0a, 0x80, 0xff, 0xff
.byte	0xff, 0xff, 0xff, 0xff
.byte	0xff, 0xff
.asciz	"enum2$<core::option::Option<ref$<slice2$<core::fmt::rt::v1::Argument> > > >::Variant1::DISCR_END"
.p2align	2
.Ltmp738:
.short	.Ltmp740-.Ltmp739
.Ltmp739:
.short	4359
.long	4375
.byte	0x00, 0x00
.asciz	"enum2$<core::fmt::rt::v1::Count>::Variant0::NAME"
.p2align	2
.Ltmp740:
.short	.Ltmp742-.Ltmp741
.Ltmp741:
.short	4359
.long	35
.byte	0x00, 0x00
.asciz	"enum2$<core::fmt::rt::v1::Count>::Variant0::DISCR_EXACT"
.p2align	2
.Ltmp742:
.short	.Ltmp744-.Ltmp743
.Ltmp743:
.short	4359
.long	4375
.byte	0x01, 0x00
.asciz	"enum2$<core::fmt::rt::v1::Count>::Variant1::NAME"
.p2align	2
.Ltmp744:
.short	.Ltmp746-.Ltmp745
.Ltmp745:
.short	4359
.long	35
.byte	0x01, 0x00
.asciz	"enum2$<core::fmt::rt::v1::Count>::Variant1::DISCR_EXACT"
.p2align	2
.Ltmp746:
.short	.Ltmp748-.Ltmp747
.Ltmp747:
.short	4359
.long	4375
.byte	0x02, 0x00
.asciz	"enum2$<core::fmt::rt::v1::Count>::Variant2::NAME"
.p2align	2
.Ltmp748:
.short	.Ltmp750-.Ltmp749
.Ltmp749:
.short	4359
.long	35
.byte	0x02, 0x00
.asciz	"enum2$<core::fmt::rt::v1::Count>::Variant2::DISCR_EXACT"
.p2align	2
.Ltmp750:
.short	.Ltmp752-.Ltmp751
.Ltmp751:
.short	4359
.long	4403
.byte	0x00, 0x00
.asciz	"enum2$<core::option::Option<ref$<str$> > >::Variant0::NAME"
.p2align	2
.Ltmp752:
.short	.Ltmp754-.Ltmp753
.Ltmp753:
.short	4359
.long	35
.byte	0x00, 0x00
.asciz	"enum2$<core::option::Option<ref$<str$> > >::Variant0::DISCR_EXACT"
.p2align	2
.Ltmp754:
.short	.Ltmp756-.Ltmp755
.Ltmp755:
.short	4359
.long	4403
.byte	0x01, 0x00
.asciz	"enum2$<core::option::Option<ref$<str$> > >::Variant1::NAME"
.p2align	2
.Ltmp756:
.short	.Ltmp758-.Ltmp757
.Ltmp757:
.short	4359
.long	35
.byte	0x01, 0x00
.asciz	"enum2$<core::option::Option<ref$<str$> > >::Variant1::DISCR_BEGIN"
.p2align	2
.Ltmp758:
.short	.Ltmp760-.Ltmp759
.Ltmp759:
.short	4359
.long	35
.byte	0x0a, 0x80, 0xff, 0xff
.byte	0xff, 0xff, 0xff, 0xff
.byte	0xff, 0xff
.asciz	"enum2$<core::option::Option<ref$<str$> > >::Variant1::DISCR_END"
.p2align	2
.Ltmp760:
.short	.Ltmp762-.Ltmp761
.Ltmp761:
.short	4359
.long	4471
.byte	0x00, 0x00
.asciz	"enum2$<core::result::Result<core::ptr::non_null::NonNull<slice2$<u8> >,core::alloc::AllocError> >::Variant0::NAME"
.p2align	2
.Ltmp762:
.short	.Ltmp764-.Ltmp763
.Ltmp763:
.short	4359
.long	35
.byte	0x01, 0x00
.asciz	"enum2$<core::result::Result<core::ptr::non_null::NonNull<slice2$<u8> >,core::alloc::AllocError> >::Variant0::DISCR_BEGIN"
.p2align	2
.Ltmp764:
.short	.Ltmp766-.Ltmp765
.Ltmp765:
.short	4359
.long	35
.byte	0x0a, 0x80, 0xff, 0xff
.byte	0xff, 0xff, 0xff, 0xff
.byte	0xff, 0xff
.asciz	"enum2$<core::result::Result<core::ptr::non_null::NonNull<slice2$<u8> >,core::alloc::AllocError> >::Variant0::DISCR_END"
.p2align	2
.Ltmp766:
.short	.Ltmp768-.Ltmp767
.Ltmp767:
.short	4359
.long	4471
.byte	0x01, 0x00
.asciz	"enum2$<core::result::Result<core::ptr::non_null::NonNull<slice2$<u8> >,core::alloc::AllocError> >::Variant1::NAME"
.p2align	2
.Ltmp768:
.short	.Ltmp770-.Ltmp769
.Ltmp769:
.short	4359
.long	35
.byte	0x00, 0x00
.asciz	"enum2$<core::result::Result<core::ptr::non_null::NonNull<slice2$<u8> >,core::alloc::AllocError> >::Variant1::DISCR_EXACT"
.p2align	2
.Ltmp770:
.short	.Ltmp772-.Ltmp771
.Ltmp771:
.short	4359
.long	4530
.byte	0x00, 0x00
.asciz	"enum2$<core::result::Result<alloc::string::String,alloc::string::FromUtf16Error> >::Variant0::NAME"
.p2align	2
.Ltmp772:
.short	.Ltmp774-.Ltmp773
.Ltmp773:
.short	4359
.long	35
.byte	0x01, 0x00
.asciz	"enum2$<core::result::Result<alloc::string::String,alloc::string::FromUtf16Error> >::Variant0::DISCR_BEGIN"
.p2align	2
.Ltmp774:
.short	.Ltmp776-.Ltmp775
.Ltmp775:
.short	4359
.long	35
.byte	0x0a, 0x80, 0xff, 0xff
.byte	0xff, 0xff, 0xff, 0xff
.byte	0xff, 0xff
.asciz	"enum2$<core::result::Result<alloc::string::String,alloc::string::FromUtf16Error> >::Variant0::DISCR_END"
.p2align	2
.Ltmp776:
.short	.Ltmp778-.Ltmp777
.Ltmp777:
.short	4359
.long	4530
.byte	0x01, 0x00
.asciz	"enum2$<core::result::Result<alloc::string::String,alloc::string::FromUtf16Error> >::Variant1::NAME"
.p2align	2
.Ltmp778:
.short	.Ltmp780-.Ltmp779
.Ltmp779:
.short	4359
.long	35
.byte	0x00, 0x00
.asciz	"enum2$<core::result::Result<alloc::string::String,alloc::string::FromUtf16Error> >::Variant1::DISCR_EXACT"
.p2align	2
.Ltmp780:
.Ltmp676:
.p2align	2
.long	241
.long	.Ltmp782-.Ltmp781
.Ltmp781:
.short	.Ltmp784-.Ltmp783
.Ltmp783:
.short	4360
.long	32
.asciz	"u8"
.p2align	2
.Ltmp784:
.short	.Ltmp786-.Ltmp785
.Ltmp785:
.short	4360
.long	4107
.asciz	"enum2$<core::result::Result<tuple$<>,core::fmt::Error> >"
.p2align	2
.Ltmp786:
.short	.Ltmp788-.Ltmp787
.Ltmp787:
.short	4360
.long	4112
.asciz	"alloc::string::String"
.p2align	2
.Ltmp788:
.short	.Ltmp790-.Ltmp789
.Ltmp789:
.short	4360
.long	117
.asciz	"u32"
.p2align	2
.Ltmp790:
.short	.Ltmp792-.Ltmp791
.Ltmp791:
.short	4360
.long	4120
.asciz	"core::fmt::Formatter"
.p2align	2
.Ltmp792:
.short	.Ltmp794-.Ltmp793
.Ltmp793:
.short	4360
.long	4127
.asciz	"enum2$<core::result::Result<tuple$<>,core::fmt::Error> >::Variant0"
.p2align	2
.Ltmp794:
.short	.Ltmp796-.Ltmp795
.Ltmp795:
.short	4360
.long	4131
.asciz	"enum2$<core::result::Result<tuple$<>,core::fmt::Error> >::Variant1"
.p2align	2
.Ltmp796:
.short	.Ltmp798-.Ltmp797
.Ltmp797:
.short	4360
.long	35
.asciz	"usize"
.p2align	2
.Ltmp798:
.short	.Ltmp800-.Ltmp799
.Ltmp799:
.short	4360
.long	4135
.asciz	"alloc::vec::Vec<u8,alloc::alloc::Global>"
.p2align	2
.Ltmp800:
.short	.Ltmp802-.Ltmp801
.Ltmp801:
.short	4360
.long	35
.asciz	"u64"
.p2align	2
.Ltmp802:
.short	.Ltmp804-.Ltmp803
.Ltmp803:
.short	4360
.long	4140
.asciz	"enum2$<core::option::Option<usize> >"
.p2align	2
.Ltmp804:
.short	.Ltmp806-.Ltmp805
.Ltmp805:
.short	4360
.long	4147
.asciz	"ref_mut$<dyn$<core::fmt::Write> >"
.p2align	2
.Ltmp806:
.short	.Ltmp808-.Ltmp807
.Ltmp807:
.short	4360
.long	4151
.asciz	"enum2$<core::result::Result<tuple$<>,core::fmt::Error> >::Ok"
.p2align	2
.Ltmp808:
.short	.Ltmp810-.Ltmp809
.Ltmp809:
.short	4360
.long	4155
.asciz	"enum2$<core::result::Result<tuple$<>,core::fmt::Error> >::Err"
.p2align	2
.Ltmp810:
.short	.Ltmp812-.Ltmp811
.Ltmp811:
.short	4360
.long	4160
.asciz	"alloc::raw_vec::RawVec<u8,alloc::alloc::Global>"
.p2align	2
.Ltmp812:
.short	.Ltmp814-.Ltmp813
.Ltmp813:
.short	4360
.long	4167
.asciz	"enum2$<core::option::Option<usize> >::Variant0"
.p2align	2
.Ltmp814:
.short	.Ltmp816-.Ltmp815
.Ltmp815:
.short	4360
.long	4171
.asciz	"enum2$<core::option::Option<usize> >::Variant1"
.p2align	2
.Ltmp816:
.short	.Ltmp818-.Ltmp817
.Ltmp817:
.short	4360
.long	4174
.asciz	"dyn$<core::fmt::Write>"
.p2align	2
.Ltmp818:
.short	.Ltmp820-.Ltmp819
.Ltmp819:
.short	4360
.long	4176
.asciz	"tuple$<>"
.p2align	2
.Ltmp820:
.short	.Ltmp822-.Ltmp821
.Ltmp821:
.short	4360
.long	4178
.asciz	"core::fmt::Error"
.p2align	2
.Ltmp822:
.short	.Ltmp824-.Ltmp823
.Ltmp823:
.short	4360
.long	4183
.asciz	"core::ptr::unique::Unique<u8>"
.p2align	2
.Ltmp824:
.short	.Ltmp826-.Ltmp825
.Ltmp825:
.short	4360
.long	4185
.asciz	"alloc::alloc::Global"
.p2align	2
.Ltmp826:
.short	.Ltmp828-.Ltmp827
.Ltmp827:
.short	4360
.long	4187
.asciz	"enum2$<core::option::Option<usize> >::None"
.p2align	2
.Ltmp828:
.short	.Ltmp830-.Ltmp829
.Ltmp829:
.short	4360
.long	4190
.asciz	"enum2$<core::option::Option<usize> >::Some"
.p2align	2
.Ltmp830:
.short	.Ltmp832-.Ltmp831
.Ltmp831:
.short	4360
.long	4193
.asciz	"core::ptr::non_null::NonNull<u8>"
.p2align	2
.Ltmp832:
.short	.Ltmp834-.Ltmp833
.Ltmp833:
.short	4360
.long	4195
.asciz	"core::marker::PhantomData<u8>"
.p2align	2
.Ltmp834:
.short	.Ltmp836-.Ltmp835
.Ltmp835:
.short	4360
.long	4203
.asciz	"ref$<str$>"
.p2align	2
.Ltmp836:
.short	.Ltmp838-.Ltmp837
.Ltmp837:
.short	4360
.long	4211
.asciz	"ref$<slice2$<u8> >"
.p2align	2
.Ltmp838:
.short	.Ltmp840-.Ltmp839
.Ltmp839:
.short	4360
.long	4233
.asciz	"enum2$<core::option::Option<tuple$<core::ptr::non_null::NonNull<u8>,core::alloc::layout::Layout> > >"
.p2align	2
.Ltmp840:
.short	.Ltmp842-.Ltmp841
.Ltmp841:
.short	4360
.long	4239
.asciz	"enum2$<core::option::Option<tuple$<core::ptr::non_null::NonNull<u8>,core::alloc::layout::Layout> > >::Variant0"
.p2align	2
.Ltmp842:
.short	.Ltmp844-.Ltmp843
.Ltmp843:
.short	4360
.long	4243
.asciz	"enum2$<core::option::Option<tuple$<core::ptr::non_null::NonNull<u8>,core::alloc::layout::Layout> > >::Variant1"
.p2align	2
.Ltmp844:
.short	.Ltmp846-.Ltmp845
.Ltmp845:
.short	4360
.long	4245
.asciz	"enum2$<core::option::Option<tuple$<core::ptr::non_null::NonNull<u8>,core::alloc::layout::Layout> > >::None"
.p2align	2
.Ltmp846:
.short	.Ltmp848-.Ltmp847
.Ltmp847:
.short	4360
.long	4249
.asciz	"enum2$<core::option::Option<tuple$<core::ptr::non_null::NonNull<u8>,core::alloc::layout::Layout> > >::Some"
.p2align	2
.Ltmp848:
.short	.Ltmp850-.Ltmp849
.Ltmp849:
.short	4360
.long	4253
.asciz	"tuple$<core::ptr::non_null::NonNull<u8>,core::alloc::layout::Layout>"
.p2align	2
.Ltmp850:
.short	.Ltmp852-.Ltmp851
.Ltmp851:
.short	4360
.long	4257
.asciz	"core::alloc::layout::Layout"
.p2align	2
.Ltmp852:
.short	.Ltmp854-.Ltmp853
.Ltmp853:
.short	4360
.long	4263
.asciz	"core::ptr::alignment::Alignment"
.p2align	2
.Ltmp854:
.short	.Ltmp856-.Ltmp855
.Ltmp855:
.short	4360
.long	4272
.asciz	"enum2$<core::result::Result<core::alloc::layout::Layout,core::alloc::layout::LayoutError> >"
.p2align	2
.Ltmp856:
.short	.Ltmp858-.Ltmp857
.Ltmp857:
.short	4360
.long	4278
.asciz	"enum2$<core::result::Result<core::alloc::layout::Layout,core::alloc::layout::LayoutError> >::Variant0"
.p2align	2
.Ltmp858:
.short	.Ltmp860-.Ltmp859
.Ltmp859:
.short	4360
.long	4282
.asciz	"enum2$<core::result::Result<core::alloc::layout::Layout,core::alloc::layout::LayoutError> >::Variant1"
.p2align	2
.Ltmp860:
.short	.Ltmp862-.Ltmp861
.Ltmp861:
.short	4360
.long	4285
.asciz	"enum2$<core::result::Result<core::alloc::layout::Layout,core::alloc::layout::LayoutError> >::Ok"
.p2align	2
.Ltmp862:
.short	.Ltmp864-.Ltmp863
.Ltmp863:
.short	4360
.long	4289
.asciz	"enum2$<core::result::Result<core::alloc::layout::Layout,core::alloc::layout::LayoutError> >::Err"
.p2align	2
.Ltmp864:
.short	.Ltmp866-.Ltmp865
.Ltmp865:
.short	4360
.long	4291
.asciz	"core::alloc::layout::LayoutError"
.p2align	2
.Ltmp866:
.short	.Ltmp868-.Ltmp867
.Ltmp867:
.short	4360
.long	4312
.asciz	"core::fmt::Arguments"
.p2align	2
.Ltmp868:
.short	.Ltmp870-.Ltmp869
.Ltmp869:
.short	4360
.long	4316
.asciz	"ref$<slice2$<ref$<str$> > >"
.p2align	2
.Ltmp870:
.short	.Ltmp872-.Ltmp871
.Ltmp871:
.short	4360
.long	4321
.asciz	"enum2$<core::option::Option<ref$<slice2$<core::fmt::rt::v1::Argument> > > >"
.p2align	2
.Ltmp872:
.short	.Ltmp874-.Ltmp873
.Ltmp873:
.short	4360
.long	4326
.asciz	"ref$<slice2$<core::fmt::ArgumentV1> >"
.p2align	2
.Ltmp874:
.short	.Ltmp876-.Ltmp875
.Ltmp875:
.short	4360
.long	4332
.asciz	"enum2$<core::option::Option<ref$<slice2$<core::fmt::rt::v1::Argument> > > >::Variant0"
.p2align	2
.Ltmp876:
.short	.Ltmp878-.Ltmp877
.Ltmp877:
.short	4360
.long	4336
.asciz	"enum2$<core::option::Option<ref$<slice2$<core::fmt::rt::v1::Argument> > > >::Variant1"
.p2align	2
.Ltmp878:
.short	.Ltmp880-.Ltmp879
.Ltmp879:
.short	4360
.long	4344
.asciz	"core::fmt::ArgumentV1"
.p2align	2
.Ltmp880:
.short	.Ltmp882-.Ltmp881
.Ltmp881:
.short	4360
.long	4346
.asciz	"enum2$<core::option::Option<ref$<slice2$<core::fmt::rt::v1::Argument> > > >::None"
.p2align	2
.Ltmp882:
.short	.Ltmp884-.Ltmp883
.Ltmp883:
.short	4360
.long	4350
.asciz	"enum2$<core::option::Option<ref$<slice2$<core::fmt::rt::v1::Argument> > > >::Some"
.p2align	2
.Ltmp884:
.short	.Ltmp886-.Ltmp885
.Ltmp885:
.short	4360
.long	4352
.asciz	"core::fmt::extern$0::Opaque"
.p2align	2
.Ltmp886:
.short	.Ltmp888-.Ltmp887
.Ltmp887:
.short	4360
.long	4357
.asciz	"ref$<slice2$<core::fmt::rt::v1::Argument> >"
.p2align	2
.Ltmp888:
.short	.Ltmp890-.Ltmp889
.Ltmp889:
.short	4360
.long	4361
.asciz	"core::fmt::rt::v1::Argument"
.p2align	2
.Ltmp890:
.short	.Ltmp892-.Ltmp891
.Ltmp891:
.short	4360
.long	4365
.asciz	"core::fmt::rt::v1::FormatSpec"
.p2align	2
.Ltmp892:
.short	.Ltmp894-.Ltmp893
.Ltmp893:
.short	4360
.long	4371
.asciz	"enum2$<core::fmt::rt::v1::Count>"
.p2align	2
.Ltmp894:
.short	.Ltmp896-.Ltmp895
.Ltmp895:
.short	4360
.long	4378
.asciz	"enum2$<core::fmt::rt::v1::Count>::Variant0"
.p2align	2
.Ltmp896:
.short	.Ltmp898-.Ltmp897
.Ltmp897:
.short	4360
.long	4382
.asciz	"enum2$<core::fmt::rt::v1::Count>::Variant1"
.p2align	2
.Ltmp898:
.short	.Ltmp900-.Ltmp899
.Ltmp899:
.short	4360
.long	4386
.asciz	"enum2$<core::fmt::rt::v1::Count>::Variant2"
.p2align	2
.Ltmp900:
.short	.Ltmp902-.Ltmp901
.Ltmp901:
.short	4360
.long	4388
.asciz	"enum2$<core::fmt::rt::v1::Count>::Is"
.p2align	2
.Ltmp902:
.short	.Ltmp904-.Ltmp903
.Ltmp903:
.short	4360
.long	4390
.asciz	"enum2$<core::fmt::rt::v1::Count>::Param"
.p2align	2
.Ltmp904:
.short	.Ltmp906-.Ltmp905
.Ltmp905:
.short	4360
.long	4392
.asciz	"enum2$<core::fmt::rt::v1::Count>::Implied"
.p2align	2
.Ltmp906:
.short	.Ltmp908-.Ltmp907
.Ltmp907:
.short	4360
.long	4400
.asciz	"enum2$<core::option::Option<ref$<str$> > >"
.p2align	2
.Ltmp908:
.short	.Ltmp910-.Ltmp909
.Ltmp909:
.short	4360
.long	4406
.asciz	"enum2$<core::option::Option<ref$<str$> > >::Variant0"
.p2align	2
.Ltmp910:
.short	.Ltmp912-.Ltmp911
.Ltmp911:
.short	4360
.long	4410
.asciz	"enum2$<core::option::Option<ref$<str$> > >::Variant1"
.p2align	2
.Ltmp912:
.short	.Ltmp914-.Ltmp913
.Ltmp913:
.short	4360
.long	4412
.asciz	"enum2$<core::option::Option<ref$<str$> > >::None"
.p2align	2
.Ltmp914:
.short	.Ltmp916-.Ltmp915
.Ltmp915:
.short	4360
.long	4415
.asciz	"enum2$<core::option::Option<ref$<str$> > >::Some"
.p2align	2
.Ltmp916:
.short	.Ltmp918-.Ltmp917
.Ltmp917:
.short	4360
.long	4425
.asciz	"alloc::fmt::format::closure_env$0"
.p2align	2
.Ltmp918:
.short	.Ltmp920-.Ltmp919
.Ltmp919:
.short	4360
.long	4468
.asciz	"enum2$<core::result::Result<core::ptr::non_null::NonNull<slice2$<u8> >,core::alloc::AllocError> >"
.p2align	2
.Ltmp920:
.short	.Ltmp922-.Ltmp921
.Ltmp921:
.short	4360
.long	4474
.asciz	"enum2$<core::result::Result<core::ptr::non_null::NonNull<slice2$<u8> >,core::alloc::AllocError> >::Variant0"
.p2align	2
.Ltmp922:
.short	.Ltmp924-.Ltmp923
.Ltmp923:
.short	4360
.long	4478
.asciz	"enum2$<core::result::Result<core::ptr::non_null::NonNull<slice2$<u8> >,core::alloc::AllocError> >::Variant1"
.p2align	2
.Ltmp924:
.short	.Ltmp926-.Ltmp925
.Ltmp925:
.short	4360
.long	4482
.asciz	"enum2$<core::result::Result<core::ptr::non_null::NonNull<slice2$<u8> >,core::alloc::AllocError> >::Ok"
.p2align	2
.Ltmp926:
.short	.Ltmp928-.Ltmp927
.Ltmp927:
.short	4360
.long	4486
.asciz	"enum2$<core::result::Result<core::ptr::non_null::NonNull<slice2$<u8> >,core::alloc::AllocError> >::Err"
.p2align	2
.Ltmp928:
.short	.Ltmp930-.Ltmp929
.Ltmp929:
.short	4360
.long	4490
.asciz	"core::ptr::non_null::NonNull<slice2$<u8> >"
.p2align	2
.Ltmp930:
.short	.Ltmp932-.Ltmp931
.Ltmp931:
.short	4360
.long	4492
.asciz	"core::alloc::AllocError"
.p2align	2
.Ltmp932:
.short	.Ltmp934-.Ltmp933
.Ltmp933:
.short	4360
.long	4494
.asciz	"ptr_const$<slice2$<u8> >"
.p2align	2
.Ltmp934:
.short	.Ltmp936-.Ltmp935
.Ltmp935:
.short	4360
.long	19
.asciz	"isize"
.p2align	2
.Ltmp936:
.short	.Ltmp938-.Ltmp937
.Ltmp937:
.short	4360
.long	4527
.asciz	"enum2$<core::result::Result<alloc::string::String,alloc::string::FromUtf16Error> >"
.p2align	2
.Ltmp938:
.short	.Ltmp940-.Ltmp939
.Ltmp939:
.short	4360
.long	4533
.asciz	"enum2$<core::result::Result<alloc::string::String,alloc::string::FromUtf16Error> >::Variant0"
.p2align	2
.Ltmp940:
.short	.Ltmp942-.Ltmp941
.Ltmp941:
.short	4360
.long	4537
.asciz	"enum2$<core::result::Result<alloc::string::String,alloc::string::FromUtf16Error> >::Variant1"
.p2align	2
.Ltmp942:
.short	.Ltmp944-.Ltmp943
.Ltmp943:
.short	4360
.long	4540
.asciz	"enum2$<core::result::Result<alloc::string::String,alloc::string::FromUtf16Error> >::Ok"
.p2align	2
.Ltmp944:
.short	.Ltmp946-.Ltmp945
.Ltmp945:
.short	4360
.long	4544
.asciz	"enum2$<core::result::Result<alloc::string::String,alloc::string::FromUtf16Error> >::Err"
.p2align	2
.Ltmp946:
.short	.Ltmp948-.Ltmp947
.Ltmp947:
.short	4360
.long	4547
.asciz	"alloc::string::FromUtf16Error"
.p2align	2
.Ltmp948:
.short	.Ltmp950-.Ltmp949
.Ltmp949:
.short	4360
.long	4554
.asciz	"core::panic::location::Location"
.p2align	2
.Ltmp950:
.short	.Ltmp952-.Ltmp951
.Ltmp951:
.short	4360
.long	4561
.asciz	"core::fmt::UnsafeArg"
.p2align	2
.Ltmp952:
.short	.Ltmp954-.Ltmp953
.Ltmp953:
.short	4360
.long	4583
.asciz	"tuple$<ref$<str$> >"
.p2align	2
.Ltmp954:
.short	.Ltmp956-.Ltmp955
.Ltmp955:
.short	4360
.long	4599
.asciz	"skyline::hooks::InlineCtx"
.p2align	2
.Ltmp956:
.short	.Ltmp958-.Ltmp957
.Ltmp957:
.short	4360
.long	4604
.asciz	"nnsdk::root::nn::os::CpuRegister"
.p2align	2
.Ltmp958:
.short	.Ltmp960-.Ltmp959
.Ltmp959:
.short	4360
.long	4608
.asciz	"nnsdk::root::__BindgenUnionField<u64>"
.p2align	2
.Ltmp960:
.short	.Ltmp962-.Ltmp961
.Ltmp961:
.short	4360
.long	4612
.asciz	"nnsdk::root::__BindgenUnionField<u32>"
.p2align	2
.Ltmp962:
.short	.Ltmp964-.Ltmp963
.Ltmp963:
.short	4360
.long	4614
.asciz	"core::marker::PhantomData<u64>"
.p2align	2
.Ltmp964:
.short	.Ltmp966-.Ltmp965
.Ltmp965:
.short	4360
.long	4616
.asciz	"core::marker::PhantomData<u32>"
.p2align	2
.Ltmp966:
.short	.Ltmp968-.Ltmp967
.Ltmp967:
.short	4360
.long	116
.asciz	"i32"
.p2align	2
.Ltmp968:
.short	.Ltmp970-.Ltmp969
.Ltmp969:
.short	4360
.long	19
.asciz	"i64"
.p2align	2
.Ltmp970:
.short	.Ltmp972-.Ltmp971
.Ltmp971:
.short	4360
.long	4634
.asciz	"libc::pthread_mutex_t"
.p2align	2
.Ltmp972:
.short	.Ltmp974-.Ltmp973
.Ltmp973:
.short	4360
.long	17
.asciz	"i16"
.p2align	2
.Ltmp974:
.short	.Ltmp976-.Ltmp975
.Ltmp975:
.short	4360
.long	4638
.asciz	"libc::__pthread_mutex_s"
.p2align	2
.Ltmp976:
.short	.Ltmp978-.Ltmp977
.Ltmp977:
.short	4360
.long	4642
.asciz	"libc::__pthread_list_t"
.p2align	2
.Ltmp978:
.short	.Ltmp980-.Ltmp979
.Ltmp979:
.short	4360
.long	4652
.asciz	"enum2$<libc::c_void>"
.p2align	2
.Ltmp980:
.short	.Ltmp982-.Ltmp981
.Ltmp981:
.short	4360
.long	4663
.asciz	"impl$<alloc::string::FromUtf16Error, core::fmt::Debug>::vtable_type$"
.p2align	2
.Ltmp982:
.short	.Ltmp984-.Ltmp983
.Ltmp983:
.short	4360
.long	4668
.asciz	"skyline::build::ModuleName<21>"
.p2align	2
.Ltmp984:
.Ltmp782:
.p2align	2
.cv_filechecksums
.cv_stringtable
.long	241
.long	.Ltmp986-.Ltmp985
.Ltmp985:
.short	.Ltmp988-.Ltmp987
.Ltmp987:
.short	4428
.long	4673
.p2align	2
.Ltmp988:
.Ltmp986:
.p2align	2
.section	.debug$T,"dr"
.p2align	2
.long	4
.short	0x1e
.short	0x1605
.long	0x0
.asciz	"alloc::string::impl$23"
.byte	241
.short	0x66
.short	0x1506
.short	0x0
.short	0x280
.long	0x0
.short	0x0
.asciz	"enum2$<core::result::Result<tuple$<>,core::fmt::Error> >"
.asciz	"84365149aa86c7c799948fa4b7b9ee19"
.short	0x4e
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"alloc::string::String"
.asciz	"ca630c5fd25c671a7962ec2b9e6e93c0"
.byte	243
.byte	242
.byte	241
.short	0xa
.short	0x1002
.long	0x1002
.long	0x1000c
.short	0x4a
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::fmt::Formatter"
.asciz	"8d625c6da93c5b657ed1f48baabd8824"
.short	0xa
.short	0x1002
.long	0x1004
.long	0x1000c
.short	0xe
.short	0x1201
.long	0x2
.long	0x1003
.long	0x1005
.short	0xe
.short	0x1008
.long	0x1001
.byte	0x0
.byte	0x0
.short	0x2
.long	0x1006
.short	0x7a
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::result::Result<tuple$<>,core::fmt::Error> >::Variant0"
.asciz	"c666fa097cfead46ebb20661225b1ec6"
.byte	242
.byte	241
.short	0x7a
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::result::Result<tuple$<>,core::fmt::Error> >::Variant1"
.asciz	"d34a4464136c5ef79c10ff3ea6b80e76"
.byte	242
.byte	241
.short	0x3a
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1008
.short	0x0
.asciz	"variant0"
.byte	241
.short	0x150d
.short	0x3
.long	0x1009
.short	0x0
.asciz	"variant1"
.byte	241
.short	0x150d
.short	0x3
.long	0x20
.short	0x0
.asciz	"tag"
.byte	242
.byte	241
.short	0x66
.short	0x1506
.short	0x3
.short	0x600
.long	0x100a
.short	0x1
.asciz	"enum2$<core::result::Result<tuple$<>,core::fmt::Error> >"
.asciz	"84365149aa86c7c799948fa4b7b9ee19"
.short	0x12
.short	0x1605
.long	0x0
.asciz	"\\<unknown>"
.byte	241
.short	0xe
.short	0x1606
.long	0x100b
.long	0x100c
.long	0x0
.short	0x5e
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"alloc::vec::Vec<u8,alloc::alloc::Global>"
.asciz	"b1bcab34066f2da9278f4295c681a1d2"
.short	0x12
.short	0x1203
.short	0x150d
.short	0x3
.long	0x100e
.short	0x0
.asciz	"vec"
.byte	242
.byte	241
.short	0x4e
.short	0x1505
.short	0x1
.short	0x200
.long	0x100f
.long	0x0
.long	0x0
.short	0x18
.asciz	"alloc::string::String"
.asciz	"ca630c5fd25c671a7962ec2b9e6e93c0"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1010
.long	0x100c
.long	0x0
.short	0x3a
.short	0x1203
.short	0x1502
.short	0x3
.short	0x0
.asciz	"Left"
.byte	241
.short	0x1502
.short	0x3
.short	0x1
.asciz	"Right"
.short	0x1502
.short	0x3
.short	0x2
.asciz	"Center"
.byte	243
.byte	242
.byte	241
.short	0x1502
.short	0x3
.short	0x3
.asciz	"Unknown"
.byte	242
.byte	241
.short	0x2e
.short	0x1507
.short	0x4
.short	0x0
.long	0x20
.long	0x1012
.asciz	"core::fmt::rt::v1::Alignment"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1013
.long	0x100c
.long	0x0
.short	0x52
.short	0x1506
.short	0x0
.short	0x280
.long	0x0
.short	0x0
.asciz	"enum2$<core::option::Option<usize> >"
.asciz	"314f3bdaf00b97bc584fad7211c20eea"
.short	0x56
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"ref_mut$<dyn$<core::fmt::Write> >"
.asciz	"13f1e5ef293b78fbebe09e5d8b76476"
.short	0x66
.short	0x1203
.short	0x150d
.short	0x3
.long	0x75
.short	0x30
.asciz	"flags"
.short	0x150d
.short	0x3
.long	0x7b
.short	0x34
.asciz	"fill"
.byte	241
.short	0x150d
.short	0x3
.long	0x1013
.short	0x38
.asciz	"align"
.short	0x150d
.short	0x3
.long	0x1015
.short	0x0
.asciz	"width"
.short	0x150d
.short	0x3
.long	0x1015
.short	0x10
.asciz	"precision"
.short	0x150d
.short	0x3
.long	0x1016
.short	0x20
.asciz	"buf"
.byte	242
.byte	241
.short	0x4a
.short	0x1505
.short	0x6
.short	0x200
.long	0x1017
.long	0x0
.long	0x0
.short	0x40
.asciz	"core::fmt::Formatter"
.asciz	"8d625c6da93c5b657ed1f48baabd8824"
.short	0xe
.short	0x1606
.long	0x1018
.long	0x100c
.long	0x0
.short	0x72
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::result::Result<tuple$<>,core::fmt::Error> >::Ok"
.asciz	"9c4dc112d0539a7f1f7602d6959c0623"
.short	0x1a
.short	0x1203
.short	0x1502
.short	0x3
.short	0x0
.asciz	"Ok"
.byte	243
.byte	242
.byte	241
.short	0x1502
.short	0x3
.short	0x1
.asciz	"Err"
.byte	242
.byte	241
.short	0x56
.short	0x1507
.short	0x2
.short	0x8
.long	0x75
.long	0x101b
.asciz	"enum2$<core::result::Result<tuple$<>,core::fmt::Error> >::VariantNames"
.byte	241
.short	0xe
.short	0x1606
.long	0x101c
.long	0x100c
.long	0x0
.short	0x36
.short	0x1203
.short	0x150d
.short	0x3
.long	0x101a
.short	0x0
.asciz	"value"
.short	0x150e
.short	0x3
.long	0x101c
.asciz	"NAME"
.byte	243
.byte	242
.byte	241
.short	0x150e
.short	0x3
.long	0x20
.asciz	"DISCR_EXACT"
.short	0x7a
.short	0x1505
.short	0x3
.short	0x208
.long	0x101e
.long	0x0
.long	0x0
.short	0x1
.asciz	"enum2$<core::result::Result<tuple$<>,core::fmt::Error> >::Variant0"
.asciz	"c666fa097cfead46ebb20661225b1ec6"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x101f
.long	0x100c
.long	0x0
.short	0x76
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::result::Result<tuple$<>,core::fmt::Error> >::Err"
.asciz	"ebed2cfe6003983b3ab8e1636000a8ed"
.byte	243
.byte	242
.byte	241
.short	0x36
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1021
.short	0x0
.asciz	"value"
.short	0x150e
.short	0x3
.long	0x101c
.asciz	"NAME"
.byte	243
.byte	242
.byte	241
.short	0x150e
.short	0x3
.long	0x20
.asciz	"DISCR_EXACT"
.short	0x7a
.short	0x1505
.short	0x3
.short	0x208
.long	0x1022
.long	0x0
.long	0x0
.short	0x1
.asciz	"enum2$<core::result::Result<tuple$<>,core::fmt::Error> >::Variant1"
.asciz	"d34a4464136c5ef79c10ff3ea6b80e76"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1023
.long	0x100c
.long	0x0
.short	0x66
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"alloc::raw_vec::RawVec<u8,alloc::alloc::Global>"
.asciz	"c9f8ad21d8bbc786b3e5213ab25b61ad"
.byte	241
.short	0x22
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1025
.short	0x0
.asciz	"buf"
.byte	242
.byte	241
.short	0x150d
.short	0x3
.long	0x23
.short	0x10
.asciz	"len"
.byte	242
.byte	241
.short	0x5e
.short	0x1505
.short	0x2
.short	0x200
.long	0x1026
.long	0x0
.long	0x0
.short	0x18
.asciz	"alloc::vec::Vec<u8,alloc::alloc::Global>"
.asciz	"b1bcab34066f2da9278f4295c681a1d2"
.short	0xe
.short	0x1606
.long	0x1027
.long	0x100c
.long	0x0
.short	0x66
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::option::Option<usize> >::Variant0"
.asciz	"312b60a5577c244ea9ad4d5427514098"
.byte	242
.byte	241
.short	0x66
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::option::Option<usize> >::Variant1"
.asciz	"67418d0eff3f55aa7a3d99edce9474bd"
.byte	242
.byte	241
.short	0x3a
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1029
.short	0x0
.asciz	"variant0"
.byte	241
.short	0x150d
.short	0x3
.long	0x102a
.short	0x0
.asciz	"variant1"
.byte	241
.short	0x150d
.short	0x3
.long	0x23
.short	0x0
.asciz	"tag"
.byte	242
.byte	241
.short	0x52
.short	0x1506
.short	0x3
.short	0x600
.long	0x102b
.short	0x10
.asciz	"enum2$<core::option::Option<usize> >"
.asciz	"314f3bdaf00b97bc584fad7211c20eea"
.short	0xe
.short	0x1606
.long	0x102c
.long	0x100c
.long	0x0
.short	0x4e
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"dyn$<core::fmt::Write>"
.asciz	"794c3a317bca85cf727587b991d36a6a"
.byte	242
.byte	241
.short	0xa
.short	0x1002
.long	0x102e
.long	0x1000c
.short	0xe
.short	0x1503
.long	0x23
.long	0x23
.short	0x18
.byte	0
.byte	241
.short	0xa
.short	0x1002
.long	0x1030
.long	0x1000c
.short	0x2a
.short	0x1203
.short	0x150d
.short	0x3
.long	0x102f
.short	0x0
.asciz	"pointer"
.byte	242
.byte	241
.short	0x150d
.short	0x3
.long	0x1031
.short	0x8
.asciz	"vtable"
.byte	243
.byte	242
.byte	241
.short	0x56
.short	0x1505
.short	0x2
.short	0x200
.long	0x1032
.long	0x0
.long	0x0
.short	0x10
.asciz	"ref_mut$<dyn$<core::fmt::Write> >"
.asciz	"13f1e5ef293b78fbebe09e5d8b76476"
.short	0xe
.short	0x1606
.long	0x1033
.long	0x100c
.long	0x0
.short	0x3e
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"tuple$<>"
.asciz	"65e33f3994015bcf158992dbe0323c0"
.byte	241
.short	0x12
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1035
.short	0x1
.asciz	"__0"
.byte	242
.byte	241
.short	0x72
.short	0x1505
.short	0x1
.short	0x208
.long	0x1036
.long	0x0
.long	0x0
.short	0x1
.asciz	"enum2$<core::result::Result<tuple$<>,core::fmt::Error> >::Ok"
.asciz	"9c4dc112d0539a7f1f7602d6959c0623"
.short	0xe
.short	0x1606
.long	0x1037
.long	0x100c
.long	0x0
.short	0x46
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::fmt::Error"
.asciz	"d2c2e00b8ae594e899cffd2b4083f1bc"
.short	0x12
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1039
.short	0x1
.asciz	"__0"
.byte	242
.byte	241
.short	0x76
.short	0x1505
.short	0x1
.short	0x208
.long	0x103a
.long	0x0
.long	0x0
.short	0x1
.asciz	"enum2$<core::result::Result<tuple$<>,core::fmt::Error> >::Err"
.asciz	"ebed2cfe6003983b3ab8e1636000a8ed"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x103b
.long	0x100c
.long	0x0
.short	0x56
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::ptr::unique::Unique<u8>"
.asciz	"22dcaab5991dc3bbf4c1c471db6a319b"
.byte	243
.byte	242
.byte	241
.short	0x4a
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"alloc::alloc::Global"
.asciz	"e56700c87718367d5b10fbea3a2d5bd2"
.short	0x32
.short	0x1203
.short	0x150d
.short	0x3
.long	0x103d
.short	0x0
.asciz	"ptr"
.byte	242
.byte	241
.short	0x150d
.short	0x3
.long	0x23
.short	0x8
.asciz	"cap"
.byte	242
.byte	241
.short	0x150d
.short	0x3
.long	0x103e
.short	0x0
.asciz	"alloc"
.short	0x66
.short	0x1505
.short	0x3
.short	0x200
.long	0x103f
.long	0x0
.long	0x0
.short	0x10
.asciz	"alloc::raw_vec::RawVec<u8,alloc::alloc::Global>"
.asciz	"c9f8ad21d8bbc786b3e5213ab25b61ad"
.byte	241
.short	0xe
.short	0x1606
.long	0x1040
.long	0x100c
.long	0x0
.short	0x62
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::option::Option<usize> >::None"
.asciz	"9a7e14846761dcea78c460580ad2290"
.byte	243
.byte	242
.byte	241
.short	0x1a
.short	0x1203
.short	0x1502
.short	0x3
.short	0x0
.asciz	"None"
.byte	241
.short	0x1502
.short	0x3
.short	0x1
.asciz	"Some"
.byte	241
.short	0x42
.short	0x1507
.short	0x2
.short	0x8
.long	0x75
.long	0x1043
.asciz	"enum2$<core::option::Option<usize> >::VariantNames"
.byte	241
.short	0xe
.short	0x1606
.long	0x1044
.long	0x100c
.long	0x0
.short	0x36
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1042
.short	0x0
.asciz	"value"
.short	0x150e
.short	0x3
.long	0x1044
.asciz	"NAME"
.byte	243
.byte	242
.byte	241
.short	0x150e
.short	0x3
.long	0x23
.asciz	"DISCR_EXACT"
.short	0x66
.short	0x1505
.short	0x3
.short	0x208
.long	0x1046
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::option::Option<usize> >::Variant0"
.asciz	"312b60a5577c244ea9ad4d5427514098"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1047
.long	0x100c
.long	0x0
.short	0x62
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::option::Option<usize> >::Some"
.asciz	"8a1a9388a83ae2a6334088f3070c491c"
.byte	242
.byte	241
.short	0x36
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1049
.short	0x0
.asciz	"value"
.short	0x150e
.short	0x3
.long	0x1044
.asciz	"NAME"
.byte	243
.byte	242
.byte	241
.short	0x150e
.short	0x3
.long	0x23
.asciz	"DISCR_EXACT"
.short	0x66
.short	0x1505
.short	0x3
.short	0x208
.long	0x104a
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::option::Option<usize> >::Variant1"
.asciz	"67418d0eff3f55aa7a3d99edce9474bd"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x104b
.long	0x100c
.long	0x0
.short	0x2
.short	0x1203
.short	0x4e
.short	0x1505
.short	0x0
.short	0x200
.long	0x104d
.long	0x0
.long	0x0
.short	0x0
.asciz	"dyn$<core::fmt::Write>"
.asciz	"794c3a317bca85cf727587b991d36a6a"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x104e
.long	0x100c
.long	0x0
.short	0x3e
.short	0x1505
.short	0x0
.short	0x200
.long	0x104d
.long	0x0
.long	0x0
.short	0x0
.asciz	"tuple$<>"
.asciz	"65e33f3994015bcf158992dbe0323c0"
.byte	241
.short	0xe
.short	0x1606
.long	0x1050
.long	0x100c
.long	0x0
.short	0x46
.short	0x1505
.short	0x0
.short	0x200
.long	0x104d
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::fmt::Error"
.asciz	"d2c2e00b8ae594e899cffd2b4083f1bc"
.short	0xe
.short	0x1606
.long	0x1052
.long	0x100c
.long	0x0
.short	0x56
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::ptr::non_null::NonNull<u8>"
.asciz	"bf451e066cea9900a8dede8675309c71"
.short	0x56
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::marker::PhantomData<u8>"
.asciz	"1a5a411d2d3ea72dcb2630e759230409"
.byte	243
.byte	242
.byte	241
.short	0x2a
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1054
.short	0x0
.asciz	"pointer"
.byte	242
.byte	241
.short	0x150d
.short	0x3
.long	0x1055
.short	0x0
.asciz	"_marker"
.byte	242
.byte	241
.short	0x56
.short	0x1505
.short	0x2
.short	0x200
.long	0x1056
.long	0x0
.long	0x0
.short	0x8
.asciz	"core::ptr::unique::Unique<u8>"
.asciz	"22dcaab5991dc3bbf4c1c471db6a319b"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1057
.long	0x100c
.long	0x0
.short	0x4a
.short	0x1505
.short	0x0
.short	0x200
.long	0x104d
.long	0x0
.long	0x0
.short	0x0
.asciz	"alloc::alloc::Global"
.asciz	"e56700c87718367d5b10fbea3a2d5bd2"
.short	0xe
.short	0x1606
.long	0x1059
.long	0x100c
.long	0x0
.short	0x62
.short	0x1505
.short	0x0
.short	0x208
.long	0x104d
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::option::Option<usize> >::None"
.asciz	"9a7e14846761dcea78c460580ad2290"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x105b
.long	0x100c
.long	0x0
.short	0x12
.short	0x1203
.short	0x150d
.short	0x3
.long	0x23
.short	0x8
.asciz	"__0"
.byte	242
.byte	241
.short	0x62
.short	0x1505
.short	0x1
.short	0x208
.long	0x105d
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::option::Option<usize> >::Some"
.asciz	"8a1a9388a83ae2a6334088f3070c491c"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x105e
.long	0x100c
.long	0x0
.short	0x16
.short	0x1203
.short	0x150d
.short	0x3
.long	0x620
.short	0x0
.asciz	"pointer"
.byte	242
.byte	241
.short	0x56
.short	0x1505
.short	0x1
.short	0x200
.long	0x1060
.long	0x0
.long	0x0
.short	0x8
.asciz	"core::ptr::non_null::NonNull<u8>"
.asciz	"bf451e066cea9900a8dede8675309c71"
.short	0xe
.short	0x1606
.long	0x1061
.long	0x100c
.long	0x0
.short	0x56
.short	0x1505
.short	0x0
.short	0x200
.long	0x104d
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::marker::PhantomData<u8>"
.asciz	"1a5a411d2d3ea72dcb2630e759230409"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1063
.long	0x100c
.long	0x0
.short	0xe
.short	0x1601
.long	0x1000
.long	0x1007
.asciz	"fmt"
.short	0x1e
.short	0x1605
.long	0x0
.asciz	"alloc::string::impl$39"
.byte	241
.short	0x42
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"ref$<str$>"
.asciz	"5a9563f597f3707a3b1d983c8a782f1"
.byte	243
.byte	242
.byte	241
.short	0xa
.short	0x1201
.long	0x1
.long	0x1003
.short	0xe
.short	0x1008
.long	0x1067
.byte	0x0
.byte	0x0
.short	0x1
.long	0x1068
.short	0x2a
.short	0x1203
.short	0x150d
.short	0x3
.long	0x620
.short	0x0
.asciz	"data_ptr"
.byte	241
.short	0x150d
.short	0x3
.long	0x23
.short	0x8
.asciz	"length"
.byte	243
.byte	242
.byte	241
.short	0x42
.short	0x1505
.short	0x2
.short	0x200
.long	0x106a
.long	0x0
.long	0x0
.short	0x10
.asciz	"ref$<str$>"
.asciz	"5a9563f597f3707a3b1d983c8a782f1"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x106b
.long	0x100c
.long	0x0
.short	0x12
.short	0x1601
.long	0x1066
.long	0x1069
.asciz	"deref"
.byte	242
.byte	241
.short	0x1a
.short	0x1605
.long	0x0
.asciz	"alloc::vec::impl$10"
.short	0x4a
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"ref$<slice2$<u8> >"
.asciz	"76f368a1641a25f5ea1dd03aa07d17c8"
.byte	242
.byte	241
.short	0xa
.short	0x1002
.long	0x100e
.long	0x1000c
.short	0xa
.short	0x1201
.long	0x1
.long	0x1070
.short	0xe
.short	0x1008
.long	0x106f
.byte	0x0
.byte	0x0
.short	0x1
.long	0x1071
.short	0x4a
.short	0x1505
.short	0x2
.short	0x200
.long	0x106a
.long	0x0
.long	0x0
.short	0x10
.asciz	"ref$<slice2$<u8> >"
.asciz	"76f368a1641a25f5ea1dd03aa07d17c8"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1073
.long	0x100c
.long	0x0
.short	0x12
.short	0x1601
.long	0x106e
.long	0x1072
.asciz	"deref"
.byte	242
.byte	241
.short	0x6
.short	0x1201
.long	0x0
.short	0x1a
.short	0x1009
.long	0x620
.long	0x100e
.long	0x1070
.byte	0x0
.byte	0x0
.short	0x0
.long	0x1076
.long	0x0
.short	0x12
.short	0x1602
.long	0x100e
.long	0x1077
.asciz	"as_ptr"
.byte	241
.short	0xa
.short	0x1002
.long	0x1025
.long	0x1000c
.short	0x1a
.short	0x1009
.long	0x620
.long	0x1025
.long	0x1079
.byte	0x0
.byte	0x0
.short	0x0
.long	0x1076
.long	0x0
.short	0xe
.short	0x1602
.long	0x1025
.long	0x107a
.asciz	"ptr"
.short	0x12
.short	0x1605
.long	0x0
.asciz	"core::ptr"
.byte	242
.byte	241
.short	0xe
.short	0x1008
.long	0x3
.byte	0x0
.byte	0x0
.short	0x1
.long	0x1071
.short	0x1a
.short	0x1601
.long	0x107c
.long	0x107d
.asciz	"drop_in_place"
.byte	242
.byte	241
.short	0xa
.short	0x1201
.long	0x1
.long	0x1079
.short	0xe
.short	0x1008
.long	0x3
.byte	0x0
.byte	0x0
.short	0x1
.long	0x107f
.short	0x1a
.short	0x1601
.long	0x107c
.long	0x1080
.asciz	"drop_in_place"
.byte	242
.byte	241
.short	0x1e
.short	0x1605
.long	0x0
.asciz	"alloc::raw_vec::impl$3"
.byte	241
.short	0x12
.short	0x1601
.long	0x1082
.long	0x1080
.asciz	"drop"
.byte	243
.byte	242
.byte	241
.short	0x92
.short	0x1506
.short	0x0
.short	0x280
.long	0x0
.short	0x0
.asciz	"enum2$<core::option::Option<tuple$<core::ptr::non_null::NonNull<u8>,core::alloc::layout::Layout> > >"
.asciz	"dd7849f6a135454176e4faee00166d9d"
.short	0x1a
.short	0x1009
.long	0x1084
.long	0x1025
.long	0x1079
.byte	0x0
.byte	0x1
.short	0x0
.long	0x1076
.long	0x0
.short	0xa6
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::option::Option<tuple$<core::ptr::non_null::NonNull<u8>,core::alloc::layout::Layout> > >::Variant0"
.asciz	"abbed453e4ac290dc216a76476044ce"
.byte	243
.byte	242
.byte	241
.short	0xa6
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::option::Option<tuple$<core::ptr::non_null::NonNull<u8>,core::alloc::layout::Layout> > >::Variant1"
.asciz	"1d4d958a7ae426c74faaefc2820ed8f"
.byte	243
.byte	242
.byte	241
.short	0x3a
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1086
.short	0x0
.asciz	"variant0"
.byte	241
.short	0x150d
.short	0x3
.long	0x1087
.short	0x0
.asciz	"variant1"
.byte	241
.short	0x150d
.short	0x3
.long	0x23
.short	0x10
.asciz	"tag"
.byte	242
.byte	241
.short	0x92
.short	0x1506
.short	0x3
.short	0x600
.long	0x1088
.short	0x18
.asciz	"enum2$<core::option::Option<tuple$<core::ptr::non_null::NonNull<u8>,core::alloc::layout::Layout> > >"
.asciz	"dd7849f6a135454176e4faee00166d9d"
.short	0xe
.short	0x1606
.long	0x1089
.long	0x100c
.long	0x0
.short	0xa2
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::option::Option<tuple$<core::ptr::non_null::NonNull<u8>,core::alloc::layout::Layout> > >::None"
.asciz	"caeddfad93f737e2b1af89ee22cc1226"
.byte	242
.byte	241
.short	0x82
.short	0x1507
.short	0x2
.short	0x8
.long	0x75
.long	0x1043
.asciz	"enum2$<core::option::Option<tuple$<core::ptr::non_null::NonNull<u8>,core::alloc::layout::Layout> > >::VariantNames"
.byte	241
.short	0xe
.short	0x1606
.long	0x108c
.long	0x100c
.long	0x0
.short	0x36
.short	0x1203
.short	0x150d
.short	0x3
.long	0x108b
.short	0x0
.asciz	"value"
.short	0x150e
.short	0x3
.long	0x108c
.asciz	"NAME"
.byte	243
.byte	242
.byte	241
.short	0x150e
.short	0x3
.long	0x23
.asciz	"DISCR_EXACT"
.short	0xa6
.short	0x1505
.short	0x3
.short	0x208
.long	0x108e
.long	0x0
.long	0x0
.short	0x18
.asciz	"enum2$<core::option::Option<tuple$<core::ptr::non_null::NonNull<u8>,core::alloc::layout::Layout> > >::Variant0"
.asciz	"abbed453e4ac290dc216a76476044ce"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x108f
.long	0x100c
.long	0x0
.short	0xa2
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::option::Option<tuple$<core::ptr::non_null::NonNull<u8>,core::alloc::layout::Layout> > >::Some"
.asciz	"99b7cb25290add072b72075b3be7956"
.byte	243
.byte	242
.byte	241
.short	0x4a
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1091
.short	0x0
.asciz	"value"
.short	0x150e
.short	0x3
.long	0x108c
.asciz	"NAME"
.byte	243
.byte	242
.byte	241
.short	0x150e
.short	0x3
.long	0x23
.asciz	"DISCR_BEGIN"
.short	0x150e
.short	0x3
.long	0x23
.asciz	"DISCR_END"
.byte	242
.byte	241
.short	0xa6
.short	0x1505
.short	0x4
.short	0x208
.long	0x1092
.long	0x0
.long	0x0
.short	0x18
.asciz	"enum2$<core::option::Option<tuple$<core::ptr::non_null::NonNull<u8>,core::alloc::layout::Layout> > >::Variant1"
.asciz	"1d4d958a7ae426c74faaefc2820ed8f"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1093
.long	0x100c
.long	0x0
.short	0xa2
.short	0x1505
.short	0x0
.short	0x208
.long	0x104d
.long	0x0
.long	0x0
.short	0x18
.asciz	"enum2$<core::option::Option<tuple$<core::ptr::non_null::NonNull<u8>,core::alloc::layout::Layout> > >::None"
.asciz	"caeddfad93f737e2b1af89ee22cc1226"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1095
.long	0x100c
.long	0x0
.short	0x7a
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"tuple$<core::ptr::non_null::NonNull<u8>,core::alloc::layout::Layout>"
.asciz	"33f367103f0bcbdba7bd2bbc6c46876"
.byte	241
.short	0x12
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1097
.short	0x0
.asciz	"__0"
.byte	242
.byte	241
.short	0xa2
.short	0x1505
.short	0x1
.short	0x208
.long	0x1098
.long	0x0
.long	0x0
.short	0x18
.asciz	"enum2$<core::option::Option<tuple$<core::ptr::non_null::NonNull<u8>,core::alloc::layout::Layout> > >::Some"
.asciz	"99b7cb25290add072b72075b3be7956"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1099
.long	0x100c
.long	0x0
.short	0x52
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::alloc::layout::Layout"
.asciz	"64b873e994db57cd9a1dd694281b76f6"
.byte	241
.short	0x22
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1054
.short	0x0
.asciz	"__0"
.byte	242
.byte	241
.short	0x150d
.short	0x3
.long	0x109b
.short	0x8
.asciz	"__1"
.byte	242
.byte	241
.short	0x7a
.short	0x1505
.short	0x2
.short	0x200
.long	0x109c
.long	0x0
.long	0x0
.short	0x18
.asciz	"tuple$<core::ptr::non_null::NonNull<u8>,core::alloc::layout::Layout>"
.asciz	"33f367103f0bcbdba7bd2bbc6c46876"
.byte	241
.short	0xe
.short	0x1606
.long	0x109d
.long	0x100c
.long	0x0
.short	0x56
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::ptr::alignment::Alignment"
.asciz	"1648a776d874203ea752726cf55d817a"
.byte	241
.short	0x22
.short	0x1203
.short	0x150d
.short	0x3
.long	0x23
.short	0x0
.asciz	"size"
.byte	241
.short	0x150d
.short	0x3
.long	0x109f
.short	0x8
.asciz	"align"
.short	0x52
.short	0x1505
.short	0x2
.short	0x200
.long	0x10a0
.long	0x0
.long	0x0
.short	0x10
.asciz	"core::alloc::layout::Layout"
.asciz	"64b873e994db57cd9a1dd694281b76f6"
.byte	241
.short	0xe
.short	0x1606
.long	0x10a1
.long	0x100c
.long	0x0
.short	0x646
.short	0x1203
.short	0x1502
.short	0x3
.short	0x1
.asciz	"_Align1Shl0"
.byte	242
.byte	241
.short	0x1502
.short	0x3
.short	0x2
.asciz	"_Align1Shl1"
.byte	242
.byte	241
.short	0x1502
.short	0x3
.short	0x4
.asciz	"_Align1Shl2"
.byte	242
.byte	241
.short	0x1502
.short	0x3
.short	0x8
.asciz	"_Align1Shl3"
.byte	242
.byte	241
.short	0x1502
.short	0x3
.short	0x10
.asciz	"_Align1Shl4"
.byte	242
.byte	241
.short	0x1502
.short	0x3
.short	0x20
.asciz	"_Align1Shl5"
.byte	242
.byte	241
.short	0x1502
.short	0x3
.short	0x40
.asciz	"_Align1Shl6"
.byte	242
.byte	241
.short	0x1502
.short	0x3
.short	0x80
.asciz	"_Align1Shl7"
.byte	242
.byte	241
.short	0x1502
.short	0x3
.short	0x100
.asciz	"_Align1Shl8"
.byte	242
.byte	241
.short	0x1502
.short	0x3
.short	0x200
.asciz	"_Align1Shl9"
.byte	242
.byte	241
.short	0x1502
.short	0x3
.short	0x400
.asciz	"_Align1Shl10"
.byte	241
.short	0x1502
.short	0x3
.short	0x800
.asciz	"_Align1Shl11"
.byte	241
.short	0x1502
.short	0x3
.short	0x1000
.asciz	"_Align1Shl12"
.byte	241
.short	0x1502
.short	0x3
.short	0x2000
.asciz	"_Align1Shl13"
.byte	241
.short	0x1502
.short	0x3
.short	0x4000
.asciz	"_Align1Shl14"
.byte	241
.short	0x1502
.short	0x3
.short	0x8002
.short	0x8000
.asciz	"_Align1Shl15"
.byte	243
.byte	242
.byte	241
.short	0x1502
.short	0x3
.short	0x8004
.long	0x10000
.asciz	"_Align1Shl16"
.byte	241
.short	0x1502
.short	0x3
.short	0x8004
.long	0x20000
.asciz	"_Align1Shl17"
.byte	241
.short	0x1502
.short	0x3
.short	0x8004
.long	0x40000
.asciz	"_Align1Shl18"
.byte	241
.short	0x1502
.short	0x3
.short	0x8004
.long	0x80000
.asciz	"_Align1Shl19"
.byte	241
.short	0x1502
.short	0x3
.short	0x8004
.long	0x100000
.asciz	"_Align1Shl20"
.byte	241
.short	0x1502
.short	0x3
.short	0x8004
.long	0x200000
.asciz	"_Align1Shl21"
.byte	241
.short	0x1502
.short	0x3
.short	0x8004
.long	0x400000
.asciz	"_Align1Shl22"
.byte	241
.short	0x1502
.short	0x3
.short	0x8004
.long	0x800000
.asciz	"_Align1Shl23"
.byte	241
.short	0x1502
.short	0x3
.short	0x8004
.long	0x1000000
.asciz	"_Align1Shl24"
.byte	241
.short	0x1502
.short	0x3
.short	0x8004
.long	0x2000000
.asciz	"_Align1Shl25"
.byte	241
.short	0x1502
.short	0x3
.short	0x8004
.long	0x4000000
.asciz	"_Align1Shl26"
.byte	241
.short	0x1502
.short	0x3
.short	0x8004
.long	0x8000000
.asciz	"_Align1Shl27"
.byte	241
.short	0x1502
.short	0x3
.short	0x8004
.long	0x10000000
.asciz	"_Align1Shl28"
.byte	241
.short	0x1502
.short	0x3
.short	0x8004
.long	0x20000000
.asciz	"_Align1Shl29"
.byte	241
.short	0x1502
.short	0x3
.short	0x8004
.long	0x40000000
.asciz	"_Align1Shl30"
.byte	241
.short	0x1502
.short	0x3
.short	0x8004
.long	0x80000000
.asciz	"_Align1Shl31"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x100000000
.asciz	"_Align1Shl32"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x200000000
.asciz	"_Align1Shl33"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x400000000
.asciz	"_Align1Shl34"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x800000000
.asciz	"_Align1Shl35"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x1000000000
.asciz	"_Align1Shl36"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x2000000000
.asciz	"_Align1Shl37"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x4000000000
.asciz	"_Align1Shl38"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x8000000000
.asciz	"_Align1Shl39"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x10000000000
.asciz	"_Align1Shl40"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x20000000000
.asciz	"_Align1Shl41"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x40000000000
.asciz	"_Align1Shl42"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x80000000000
.asciz	"_Align1Shl43"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x100000000000
.asciz	"_Align1Shl44"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x200000000000
.asciz	"_Align1Shl45"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x400000000000
.asciz	"_Align1Shl46"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x800000000000
.asciz	"_Align1Shl47"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x1000000000000
.asciz	"_Align1Shl48"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x2000000000000
.asciz	"_Align1Shl49"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x4000000000000
.asciz	"_Align1Shl50"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x8000000000000
.asciz	"_Align1Shl51"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x10000000000000
.asciz	"_Align1Shl52"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x20000000000000
.asciz	"_Align1Shl53"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x40000000000000
.asciz	"_Align1Shl54"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x80000000000000
.asciz	"_Align1Shl55"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x100000000000000
.asciz	"_Align1Shl56"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x200000000000000
.asciz	"_Align1Shl57"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x400000000000000
.asciz	"_Align1Shl58"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x800000000000000
.asciz	"_Align1Shl59"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x1000000000000000
.asciz	"_Align1Shl60"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x2000000000000000
.asciz	"_Align1Shl61"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x4000000000000000
.asciz	"_Align1Shl62"
.byte	241
.short	0x1502
.short	0x3
.short	0x800a
.quad	0x8000000000000000
.asciz	"_Align1Shl63"
.byte	241
.short	0x36
.short	0x1507
.short	0x40
.short	0x0
.long	0x23
.long	0x10a3
.asciz	"core::ptr::alignment::AlignmentEnum64"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x10a4
.long	0x100c
.long	0x0
.short	0x12
.short	0x1203
.short	0x150d
.short	0x3
.long	0x10a4
.short	0x0
.asciz	"__0"
.byte	242
.byte	241
.short	0x56
.short	0x1505
.short	0x1
.short	0x200
.long	0x10a6
.long	0x0
.long	0x0
.short	0x8
.asciz	"core::ptr::alignment::Alignment"
.asciz	"1648a776d874203ea752726cf55d817a"
.byte	241
.short	0xe
.short	0x1606
.long	0x10a7
.long	0x100c
.long	0x0
.short	0x1a
.short	0x1602
.long	0x1025
.long	0x1085
.asciz	"current_memory"
.byte	241
.short	0x8a
.short	0x1506
.short	0x0
.short	0x280
.long	0x0
.short	0x0
.asciz	"enum2$<core::result::Result<core::alloc::layout::Layout,core::alloc::layout::LayoutError> >"
.asciz	"a4f64b314f6b48f9fadd3b48e072b787"
.byte	241
.short	0xa
.short	0x1201
.long	0x1
.long	0x23
.short	0x1a
.short	0x1009
.long	0x10aa
.long	0x109b
.long	0x0
.byte	0x0
.byte	0x1
.short	0x1
.long	0x10ab
.long	0x0
.short	0x9a
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::result::Result<core::alloc::layout::Layout,core::alloc::layout::LayoutError> >::Variant0"
.asciz	"fb9df58d2ca89a59b83bd87cbdc449f"
.short	0x9e
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::result::Result<core::alloc::layout::Layout,core::alloc::layout::LayoutError> >::Variant1"
.asciz	"676ec6295ea9fbb73d96d29061bb336a"
.byte	243
.byte	242
.byte	241
.short	0x3a
.short	0x1203
.short	0x150d
.short	0x3
.long	0x10ad
.short	0x0
.asciz	"variant0"
.byte	241
.short	0x150d
.short	0x3
.long	0x10ae
.short	0x0
.asciz	"variant1"
.byte	241
.short	0x150d
.short	0x3
.long	0x23
.short	0x8
.asciz	"tag"
.byte	242
.byte	241
.short	0x8a
.short	0x1506
.short	0x3
.short	0x600
.long	0x10af
.short	0x10
.asciz	"enum2$<core::result::Result<core::alloc::layout::Layout,core::alloc::layout::LayoutError> >"
.asciz	"a4f64b314f6b48f9fadd3b48e072b787"
.byte	241
.short	0xe
.short	0x1606
.long	0x10b0
.long	0x100c
.long	0x0
.short	0x96
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::result::Result<core::alloc::layout::Layout,core::alloc::layout::LayoutError> >::Ok"
.asciz	"509580436af2e9abf927f5d899c946ee"
.byte	241
.short	0x7a
.short	0x1507
.short	0x2
.short	0x8
.long	0x75
.long	0x101b
.asciz	"enum2$<core::result::Result<core::alloc::layout::Layout,core::alloc::layout::LayoutError> >::VariantNames"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x10b3
.long	0x100c
.long	0x0
.short	0x4a
.short	0x1203
.short	0x150d
.short	0x3
.long	0x10b2
.short	0x0
.asciz	"value"
.short	0x150e
.short	0x3
.long	0x10b3
.asciz	"NAME"
.byte	243
.byte	242
.byte	241
.short	0x150e
.short	0x3
.long	0x23
.asciz	"DISCR_BEGIN"
.short	0x150e
.short	0x3
.long	0x23
.asciz	"DISCR_END"
.byte	242
.byte	241
.short	0x9a
.short	0x1505
.short	0x4
.short	0x208
.long	0x10b5
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::result::Result<core::alloc::layout::Layout,core::alloc::layout::LayoutError> >::Variant0"
.asciz	"fb9df58d2ca89a59b83bd87cbdc449f"
.short	0xe
.short	0x1606
.long	0x10b6
.long	0x100c
.long	0x0
.short	0x96
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::result::Result<core::alloc::layout::Layout,core::alloc::layout::LayoutError> >::Err"
.asciz	"a1a8c723b4ec1cfe826eb5805f17e26e"
.short	0x36
.short	0x1203
.short	0x150d
.short	0x3
.long	0x10b8
.short	0x0
.asciz	"value"
.short	0x150e
.short	0x3
.long	0x10b3
.asciz	"NAME"
.byte	243
.byte	242
.byte	241
.short	0x150e
.short	0x3
.long	0x23
.asciz	"DISCR_EXACT"
.short	0x9e
.short	0x1505
.short	0x3
.short	0x208
.long	0x10b9
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::result::Result<core::alloc::layout::Layout,core::alloc::layout::LayoutError> >::Variant1"
.asciz	"676ec6295ea9fbb73d96d29061bb336a"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x10ba
.long	0x100c
.long	0x0
.short	0x12
.short	0x1203
.short	0x150d
.short	0x3
.long	0x109b
.short	0x0
.asciz	"__0"
.byte	242
.byte	241
.short	0x96
.short	0x1505
.short	0x1
.short	0x208
.long	0x10bc
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::result::Result<core::alloc::layout::Layout,core::alloc::layout::LayoutError> >::Ok"
.asciz	"509580436af2e9abf927f5d899c946ee"
.byte	241
.short	0xe
.short	0x1606
.long	0x10bd
.long	0x100c
.long	0x0
.short	0x56
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::alloc::layout::LayoutError"
.asciz	"9821346e8ab492174b2ccc7fcc1ad63a"
.short	0x12
.short	0x1203
.short	0x150d
.short	0x3
.long	0x10bf
.short	0x0
.asciz	"__0"
.byte	242
.byte	241
.short	0x96
.short	0x1505
.short	0x1
.short	0x208
.long	0x10c0
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::result::Result<core::alloc::layout::Layout,core::alloc::layout::LayoutError> >::Err"
.asciz	"a1a8c723b4ec1cfe826eb5805f17e26e"
.short	0xe
.short	0x1606
.long	0x10c1
.long	0x100c
.long	0x0
.short	0x56
.short	0x1505
.short	0x0
.short	0x200
.long	0x104d
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::alloc::layout::LayoutError"
.asciz	"9821346e8ab492174b2ccc7fcc1ad63a"
.short	0xe
.short	0x1606
.long	0x10c3
.long	0x100c
.long	0x0
.short	0x12
.short	0x1602
.long	0x109b
.long	0x10ac
.asciz	"array"
.byte	242
.byte	241
.short	0x2a
.short	0x1605
.long	0x0
.asciz	"core::alloc::layout::impl$0::array"
.byte	241
.short	0x12
.short	0x1201
.long	0x3
.long	0x23
.long	0x109f
.long	0x23
.short	0xe
.short	0x1008
.long	0x10aa
.byte	0x0
.byte	0x0
.short	0x3
.long	0x10c7
.short	0x12
.short	0x1601
.long	0x10c6
.long	0x10c8
.asciz	"inner"
.byte	242
.byte	241
.short	0x1e
.short	0x1605
.long	0x0
.asciz	"alloc::alloc::impl$1"
.byte	243
.byte	242
.byte	241
.short	0xa
.short	0x1002
.long	0x103e
.long	0x1000c
.short	0x12
.short	0x1201
.long	0x3
.long	0x10cb
.long	0x1054
.long	0x109b
.short	0xe
.short	0x1008
.long	0x3
.byte	0x0
.byte	0x0
.short	0x3
.long	0x10cc
.short	0x16
.short	0x1601
.long	0x10ca
.long	0x10cd
.asciz	"deallocate"
.byte	241
.short	0x16
.short	0x1605
.long	0x0
.asciz	"alloc::alloc"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1201
.long	0x2
.long	0x620
.long	0x109b
.short	0xe
.short	0x1008
.long	0x3
.byte	0x0
.byte	0x0
.short	0x2
.long	0x10d0
.short	0x12
.short	0x1601
.long	0x10cf
.long	0x10d1
.asciz	"dealloc"
.short	0x4a
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::fmt::Arguments"
.asciz	"20207b42968cd658198dafbaa4934ef7"
.short	0x52
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"ref$<slice2$<ref$<str$> > >"
.asciz	"5c4964e97d6dfb41ee6a117a01f17f9"
.byte	242
.byte	241
.short	0x7a
.short	0x1506
.short	0x0
.short	0x280
.long	0x0
.short	0x0
.asciz	"enum2$<core::option::Option<ref$<slice2$<core::fmt::rt::v1::Argument> > > >"
.asciz	"d3074a849e7f99cc7bac1790e1bf85cb"
.byte	241
.short	0x5e
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"ref$<slice2$<core::fmt::ArgumentV1> >"
.asciz	"ed8041076b62bd5cf123c1dcc23bef03"
.byte	243
.byte	242
.byte	241
.short	0x36
.short	0x1203
.short	0x150d
.short	0x3
.long	0x10d4
.short	0x0
.asciz	"pieces"
.byte	243
.byte	242
.byte	241
.short	0x150d
.short	0x3
.long	0x10d5
.short	0x10
.asciz	"fmt"
.byte	242
.byte	241
.short	0x150d
.short	0x3
.long	0x10d6
.short	0x20
.asciz	"args"
.byte	241
.short	0x4a
.short	0x1505
.short	0x3
.short	0x200
.long	0x10d7
.long	0x0
.long	0x0
.short	0x30
.asciz	"core::fmt::Arguments"
.asciz	"20207b42968cd658198dafbaa4934ef7"
.short	0xe
.short	0x1606
.long	0x10d8
.long	0x100c
.long	0x0
.short	0xa
.short	0x1002
.long	0x1067
.long	0x1000c
.short	0x2a
.short	0x1203
.short	0x150d
.short	0x3
.long	0x10da
.short	0x0
.asciz	"data_ptr"
.byte	241
.short	0x150d
.short	0x3
.long	0x23
.short	0x8
.asciz	"length"
.byte	243
.byte	242
.byte	241
.short	0x52
.short	0x1505
.short	0x2
.short	0x200
.long	0x10db
.long	0x0
.long	0x0
.short	0x10
.asciz	"ref$<slice2$<ref$<str$> > >"
.asciz	"5c4964e97d6dfb41ee6a117a01f17f9"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x10dc
.long	0x100c
.long	0x0
.short	0x8e
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::option::Option<ref$<slice2$<core::fmt::rt::v1::Argument> > > >::Variant0"
.asciz	"c6ed0a0b4979c7a1f0c9e65b0e203013"
.byte	243
.byte	242
.byte	241
.short	0x8e
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::option::Option<ref$<slice2$<core::fmt::rt::v1::Argument> > > >::Variant1"
.asciz	"ce329723a182b9a773869b59357863e5"
.byte	243
.byte	242
.byte	241
.short	0x3a
.short	0x1203
.short	0x150d
.short	0x3
.long	0x10de
.short	0x0
.asciz	"variant0"
.byte	241
.short	0x150d
.short	0x3
.long	0x10df
.short	0x0
.asciz	"variant1"
.byte	241
.short	0x150d
.short	0x3
.long	0x23
.short	0x0
.asciz	"tag"
.byte	242
.byte	241
.short	0x7a
.short	0x1506
.short	0x3
.short	0x600
.long	0x10e0
.short	0x10
.asciz	"enum2$<core::option::Option<ref$<slice2$<core::fmt::rt::v1::Argument> > > >"
.asciz	"d3074a849e7f99cc7bac1790e1bf85cb"
.byte	241
.short	0xe
.short	0x1606
.long	0x10e1
.long	0x100c
.long	0x0
.short	0x4e
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::fmt::ArgumentV1"
.asciz	"8746d27928d0d71260dde07128f738fa"
.byte	243
.byte	242
.byte	241
.short	0xa
.short	0x1002
.long	0x10e3
.long	0x1000c
.short	0x2a
.short	0x1203
.short	0x150d
.short	0x3
.long	0x10e4
.short	0x0
.asciz	"data_ptr"
.byte	241
.short	0x150d
.short	0x3
.long	0x23
.short	0x8
.asciz	"length"
.byte	243
.byte	242
.byte	241
.short	0x5e
.short	0x1505
.short	0x2
.short	0x200
.long	0x10e5
.long	0x0
.long	0x0
.short	0x10
.asciz	"ref$<slice2$<core::fmt::ArgumentV1> >"
.asciz	"ed8041076b62bd5cf123c1dcc23bef03"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x10e6
.long	0x100c
.long	0x0
.short	0x8a
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::option::Option<ref$<slice2$<core::fmt::rt::v1::Argument> > > >::None"
.asciz	"e1702c0b20cb1a0591ef767bce336d44"
.byte	243
.byte	242
.byte	241
.short	0x6a
.short	0x1507
.short	0x2
.short	0x8
.long	0x75
.long	0x1043
.asciz	"enum2$<core::option::Option<ref$<slice2$<core::fmt::rt::v1::Argument> > > >::VariantNames"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x10e9
.long	0x100c
.long	0x0
.short	0x36
.short	0x1203
.short	0x150d
.short	0x3
.long	0x10e8
.short	0x0
.asciz	"value"
.short	0x150e
.short	0x3
.long	0x10e9
.asciz	"NAME"
.byte	243
.byte	242
.byte	241
.short	0x150e
.short	0x3
.long	0x23
.asciz	"DISCR_EXACT"
.short	0x8e
.short	0x1505
.short	0x3
.short	0x208
.long	0x10eb
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::option::Option<ref$<slice2$<core::fmt::rt::v1::Argument> > > >::Variant0"
.asciz	"c6ed0a0b4979c7a1f0c9e65b0e203013"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x10ec
.long	0x100c
.long	0x0
.short	0x8a
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::option::Option<ref$<slice2$<core::fmt::rt::v1::Argument> > > >::Some"
.asciz	"76a7d1a825fd4bb23eb8e5db58bf97cf"
.byte	243
.byte	242
.byte	241
.short	0x4a
.short	0x1203
.short	0x150d
.short	0x3
.long	0x10ee
.short	0x0
.asciz	"value"
.short	0x150e
.short	0x3
.long	0x10e9
.asciz	"NAME"
.byte	243
.byte	242
.byte	241
.short	0x150e
.short	0x3
.long	0x23
.asciz	"DISCR_BEGIN"
.short	0x150e
.short	0x3
.long	0x23
.asciz	"DISCR_END"
.byte	242
.byte	241
.short	0x8e
.short	0x1505
.short	0x4
.short	0x208
.long	0x10ef
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::option::Option<ref$<slice2$<core::fmt::rt::v1::Argument> > > >::Variant1"
.asciz	"ce329723a182b9a773869b59357863e5"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x10f0
.long	0x100c
.long	0x0
.short	0x52
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::fmt::extern$0::Opaque"
.asciz	"2fd10ed1d5c58ce56e2e545ef57346cd"
.byte	241
.short	0xa
.short	0x1002
.long	0x10f2
.long	0x1000c
.short	0xe
.short	0x1201
.long	0x2
.long	0x10f3
.long	0x1005
.short	0xe
.short	0x1008
.long	0x1001
.byte	0x0
.byte	0x0
.short	0x2
.long	0x10f4
.short	0xa
.short	0x1002
.long	0x10f5
.long	0x1000c
.short	0x26
.short	0x1203
.short	0x150d
.short	0x3
.long	0x10f3
.short	0x0
.asciz	"value"
.short	0x150d
.short	0x3
.long	0x10f6
.short	0x8
.asciz	"formatter"
.short	0x4e
.short	0x1505
.short	0x2
.short	0x200
.long	0x10f7
.long	0x0
.long	0x0
.short	0x10
.asciz	"core::fmt::ArgumentV1"
.asciz	"8746d27928d0d71260dde07128f738fa"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x10f8
.long	0x100c
.long	0x0
.short	0x8a
.short	0x1505
.short	0x0
.short	0x208
.long	0x104d
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::option::Option<ref$<slice2$<core::fmt::rt::v1::Argument> > > >::None"
.asciz	"e1702c0b20cb1a0591ef767bce336d44"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x10fa
.long	0x100c
.long	0x0
.short	0x62
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"ref$<slice2$<core::fmt::rt::v1::Argument> >"
.asciz	"16457ea002c22c61641498738cded5e1"
.byte	241
.short	0x12
.short	0x1203
.short	0x150d
.short	0x3
.long	0x10fc
.short	0x0
.asciz	"__0"
.byte	242
.byte	241
.short	0x8a
.short	0x1505
.short	0x1
.short	0x208
.long	0x10fd
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::option::Option<ref$<slice2$<core::fmt::rt::v1::Argument> > > >::Some"
.asciz	"76a7d1a825fd4bb23eb8e5db58bf97cf"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x10fe
.long	0x100c
.long	0x0
.short	0x52
.short	0x1505
.short	0x0
.short	0x200
.long	0x104d
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::fmt::extern$0::Opaque"
.asciz	"2fd10ed1d5c58ce56e2e545ef57346cd"
.byte	241
.short	0xe
.short	0x1606
.long	0x1100
.long	0x100c
.long	0x0
.short	0x52
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::fmt::rt::v1::Argument"
.asciz	"b53a963f1b91920c9f25e9593432477"
.byte	242
.byte	241
.short	0xa
.short	0x1002
.long	0x1102
.long	0x1000c
.short	0x2a
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1103
.short	0x0
.asciz	"data_ptr"
.byte	241
.short	0x150d
.short	0x3
.long	0x23
.short	0x8
.asciz	"length"
.byte	243
.byte	242
.byte	241
.short	0x62
.short	0x1505
.short	0x2
.short	0x200
.long	0x1104
.long	0x0
.long	0x0
.short	0x10
.asciz	"ref$<slice2$<core::fmt::rt::v1::Argument> >"
.asciz	"16457ea002c22c61641498738cded5e1"
.byte	241
.short	0xe
.short	0x1606
.long	0x1105
.long	0x100c
.long	0x0
.short	0x56
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::fmt::rt::v1::FormatSpec"
.asciz	"c5f426a678ae94a61bba95f7eb6988ee"
.byte	243
.byte	242
.byte	241
.short	0x2a
.short	0x1203
.short	0x150d
.short	0x3
.long	0x23
.short	0x0
.asciz	"position"
.byte	241
.short	0x150d
.short	0x3
.long	0x1107
.short	0x8
.asciz	"format"
.byte	243
.byte	242
.byte	241
.short	0x52
.short	0x1505
.short	0x2
.short	0x200
.long	0x1108
.long	0x0
.long	0x0
.short	0x38
.asciz	"core::fmt::rt::v1::Argument"
.asciz	"b53a963f1b91920c9f25e9593432477"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1109
.long	0x100c
.long	0x0
.short	0x4e
.short	0x1506
.short	0x0
.short	0x280
.long	0x0
.short	0x0
.asciz	"enum2$<core::fmt::rt::v1::Count>"
.asciz	"bc67309499349c4d8f7002281d245d95"
.short	0x56
.short	0x1203
.short	0x150d
.short	0x3
.long	0x7b
.short	0x20
.asciz	"fill"
.byte	241
.short	0x150d
.short	0x3
.long	0x1013
.short	0x28
.asciz	"align"
.short	0x150d
.short	0x3
.long	0x75
.short	0x24
.asciz	"flags"
.short	0x150d
.short	0x3
.long	0x110b
.short	0x0
.asciz	"precision"
.short	0x150d
.short	0x3
.long	0x110b
.short	0x10
.asciz	"width"
.short	0x56
.short	0x1505
.short	0x5
.short	0x200
.long	0x110c
.long	0x0
.long	0x0
.short	0x30
.asciz	"core::fmt::rt::v1::FormatSpec"
.asciz	"c5f426a678ae94a61bba95f7eb6988ee"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x110d
.long	0x100c
.long	0x0
.short	0x62
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::fmt::rt::v1::Count>::Variant0"
.asciz	"3442cfc9ba7db846404d2f92b7381f05"
.byte	242
.byte	241
.short	0x62
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::fmt::rt::v1::Count>::Variant1"
.asciz	"790f011387e7aee7e50ba1d5fc5bb380"
.byte	242
.byte	241
.short	0x62
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::fmt::rt::v1::Count>::Variant2"
.asciz	"682df3f489a38d158380073c485cb211"
.byte	242
.byte	241
.short	0x4e
.short	0x1203
.short	0x150d
.short	0x3
.long	0x110f
.short	0x0
.asciz	"variant0"
.byte	241
.short	0x150d
.short	0x3
.long	0x1110
.short	0x0
.asciz	"variant1"
.byte	241
.short	0x150d
.short	0x3
.long	0x1111
.short	0x0
.asciz	"variant2"
.byte	241
.short	0x150d
.short	0x3
.long	0x23
.short	0x0
.asciz	"tag"
.byte	242
.byte	241
.short	0x4e
.short	0x1506
.short	0x4
.short	0x600
.long	0x1112
.short	0x10
.asciz	"enum2$<core::fmt::rt::v1::Count>"
.asciz	"bc67309499349c4d8f7002281d245d95"
.short	0xe
.short	0x1606
.long	0x1113
.long	0x100c
.long	0x0
.short	0x5a
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::fmt::rt::v1::Count>::Is"
.asciz	"18c5cce2c7faa3bdc54b5c69396c039c"
.short	0x2a
.short	0x1203
.short	0x1502
.short	0x3
.short	0x0
.asciz	"Is"
.byte	243
.byte	242
.byte	241
.short	0x1502
.short	0x3
.short	0x1
.asciz	"Param"
.short	0x1502
.short	0x3
.short	0x2
.asciz	"Implied"
.byte	242
.byte	241
.short	0x3e
.short	0x1507
.short	0x3
.short	0x8
.long	0x75
.long	0x1116
.asciz	"enum2$<core::fmt::rt::v1::Count>::VariantNames"
.byte	241
.short	0xe
.short	0x1606
.long	0x1117
.long	0x100c
.long	0x0
.short	0x36
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1115
.short	0x0
.asciz	"value"
.short	0x150e
.short	0x3
.long	0x1117
.asciz	"NAME"
.byte	243
.byte	242
.byte	241
.short	0x150e
.short	0x3
.long	0x23
.asciz	"DISCR_EXACT"
.short	0x62
.short	0x1505
.short	0x3
.short	0x208
.long	0x1119
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::fmt::rt::v1::Count>::Variant0"
.asciz	"3442cfc9ba7db846404d2f92b7381f05"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x111a
.long	0x100c
.long	0x0
.short	0x5e
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::fmt::rt::v1::Count>::Param"
.asciz	"3d611d5aedacd1fc272db46ba9a51ce1"
.byte	241
.short	0x36
.short	0x1203
.short	0x150d
.short	0x3
.long	0x111c
.short	0x0
.asciz	"value"
.short	0x150e
.short	0x3
.long	0x1117
.asciz	"NAME"
.byte	243
.byte	242
.byte	241
.short	0x150e
.short	0x3
.long	0x23
.asciz	"DISCR_EXACT"
.short	0x62
.short	0x1505
.short	0x3
.short	0x208
.long	0x111d
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::fmt::rt::v1::Count>::Variant1"
.asciz	"790f011387e7aee7e50ba1d5fc5bb380"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x111e
.long	0x100c
.long	0x0
.short	0x62
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::fmt::rt::v1::Count>::Implied"
.asciz	"f81864a620094f42f590f40af682886e"
.byte	243
.byte	242
.byte	241
.short	0x36
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1120
.short	0x0
.asciz	"value"
.short	0x150e
.short	0x3
.long	0x1117
.asciz	"NAME"
.byte	243
.byte	242
.byte	241
.short	0x150e
.short	0x3
.long	0x23
.asciz	"DISCR_EXACT"
.short	0x62
.short	0x1505
.short	0x3
.short	0x208
.long	0x1121
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::fmt::rt::v1::Count>::Variant2"
.asciz	"682df3f489a38d158380073c485cb211"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1122
.long	0x100c
.long	0x0
.short	0x5a
.short	0x1505
.short	0x1
.short	0x208
.long	0x105d
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::fmt::rt::v1::Count>::Is"
.asciz	"18c5cce2c7faa3bdc54b5c69396c039c"
.short	0xe
.short	0x1606
.long	0x1124
.long	0x100c
.long	0x0
.short	0x5e
.short	0x1505
.short	0x1
.short	0x208
.long	0x105d
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::fmt::rt::v1::Count>::Param"
.asciz	"3d611d5aedacd1fc272db46ba9a51ce1"
.byte	241
.short	0xe
.short	0x1606
.long	0x1126
.long	0x100c
.long	0x0
.short	0x62
.short	0x1505
.short	0x0
.short	0x208
.long	0x104d
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::fmt::rt::v1::Count>::Implied"
.asciz	"f81864a620094f42f590f40af682886e"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1128
.long	0x100c
.long	0x0
.short	0x5a
.short	0x1506
.short	0x0
.short	0x280
.long	0x0
.short	0x0
.asciz	"enum2$<core::option::Option<ref$<str$> > >"
.asciz	"a031cf57726f805b65c0769320d00d86"
.byte	242
.byte	241
.short	0xa
.short	0x1002
.long	0x10d3
.long	0x1000c
.short	0x1a
.short	0x1009
.long	0x112a
.long	0x10d3
.long	0x112b
.byte	0x0
.byte	0x1
.short	0x0
.long	0x1076
.long	0x0
.short	0x6a
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::option::Option<ref$<str$> > >::Variant0"
.asciz	"284402b81b9da725b4c96c81260d0b3e"
.short	0x6a
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::option::Option<ref$<str$> > >::Variant1"
.asciz	"5559bb49ce15aade14b388f390468fed"
.short	0x3a
.short	0x1203
.short	0x150d
.short	0x3
.long	0x112d
.short	0x0
.asciz	"variant0"
.byte	241
.short	0x150d
.short	0x3
.long	0x112e
.short	0x0
.asciz	"variant1"
.byte	241
.short	0x150d
.short	0x3
.long	0x23
.short	0x0
.asciz	"tag"
.byte	242
.byte	241
.short	0x5a
.short	0x1506
.short	0x3
.short	0x600
.long	0x112f
.short	0x10
.asciz	"enum2$<core::option::Option<ref$<str$> > >"
.asciz	"a031cf57726f805b65c0769320d00d86"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1130
.long	0x100c
.long	0x0
.short	0x66
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::option::Option<ref$<str$> > >::None"
.asciz	"1fdc80cf97d03adfa8b4c1e689c07bcb"
.short	0x4a
.short	0x1507
.short	0x2
.short	0x8
.long	0x75
.long	0x1043
.asciz	"enum2$<core::option::Option<ref$<str$> > >::VariantNames"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1133
.long	0x100c
.long	0x0
.short	0x36
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1132
.short	0x0
.asciz	"value"
.short	0x150e
.short	0x3
.long	0x1133
.asciz	"NAME"
.byte	243
.byte	242
.byte	241
.short	0x150e
.short	0x3
.long	0x23
.asciz	"DISCR_EXACT"
.short	0x6a
.short	0x1505
.short	0x3
.short	0x208
.long	0x1135
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::option::Option<ref$<str$> > >::Variant0"
.asciz	"284402b81b9da725b4c96c81260d0b3e"
.short	0xe
.short	0x1606
.long	0x1136
.long	0x100c
.long	0x0
.short	0x66
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::option::Option<ref$<str$> > >::Some"
.asciz	"8b8b376a18d42c0ff80efa8cf56bc8bb"
.short	0x4a
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1138
.short	0x0
.asciz	"value"
.short	0x150e
.short	0x3
.long	0x1133
.asciz	"NAME"
.byte	243
.byte	242
.byte	241
.short	0x150e
.short	0x3
.long	0x23
.asciz	"DISCR_BEGIN"
.short	0x150e
.short	0x3
.long	0x23
.asciz	"DISCR_END"
.byte	242
.byte	241
.short	0x6a
.short	0x1505
.short	0x4
.short	0x208
.long	0x1139
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::option::Option<ref$<str$> > >::Variant1"
.asciz	"5559bb49ce15aade14b388f390468fed"
.short	0xe
.short	0x1606
.long	0x113a
.long	0x100c
.long	0x0
.short	0x66
.short	0x1505
.short	0x0
.short	0x208
.long	0x104d
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::option::Option<ref$<str$> > >::None"
.asciz	"1fdc80cf97d03adfa8b4c1e689c07bcb"
.short	0xe
.short	0x1606
.long	0x113c
.long	0x100c
.long	0x0
.short	0x12
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1067
.short	0x0
.asciz	"__0"
.byte	242
.byte	241
.short	0x66
.short	0x1505
.short	0x1
.short	0x208
.long	0x113e
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::option::Option<ref$<str$> > >::Some"
.asciz	"8b8b376a18d42c0ff80efa8cf56bc8bb"
.short	0xe
.short	0x1606
.long	0x113f
.long	0x100c
.long	0x0
.short	0x12
.short	0x1602
.long	0x10d3
.long	0x112c
.asciz	"as_str"
.byte	241
.short	0x5a
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"alloc::fmt::format::closure_env$0"
.asciz	"cf392acfd6e81391c38b65030e7a6914"
.byte	243
.byte	242
.byte	241
.short	0xa
.short	0x1201
.long	0x1
.long	0x1067
.short	0xe
.short	0x1008
.long	0x1002
.byte	0x0
.byte	0x0
.short	0x1
.long	0x1143
.short	0xa
.short	0x1002
.long	0x1144
.long	0x1000c
.short	0x12
.short	0x1201
.long	0x3
.long	0x112a
.long	0x1142
.long	0x1145
.short	0x1a
.short	0x1009
.long	0x1002
.long	0x112a
.long	0x0
.byte	0x0
.byte	0x1
.short	0x3
.long	0x1146
.long	0x0
.short	0x1a
.short	0x1203
.short	0x150d
.short	0x3
.long	0x112b
.short	0x0
.asciz	"_ref__args"
.byte	243
.byte	242
.byte	241
.short	0x5a
.short	0x1505
.short	0x1
.short	0x200
.long	0x1148
.long	0x0
.long	0x0
.short	0x8
.asciz	"alloc::fmt::format::closure_env$0"
.asciz	"cf392acfd6e81391c38b65030e7a6914"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1149
.long	0x100c
.long	0x0
.short	0x16
.short	0x1602
.long	0x112a
.long	0x1147
.asciz	"map_or_else"
.short	0x1a
.short	0x1605
.long	0x0
.asciz	"alloc::fmt::format"
.byte	241
.short	0xa
.short	0x1201
.long	0x1
.long	0x1142
.short	0xe
.short	0x1008
.long	0x1002
.byte	0x0
.byte	0x0
.short	0x1
.long	0x114d
.short	0x16
.short	0x1601
.long	0x114c
.long	0x114e
.asciz	"closure$0"
.byte	242
.byte	241
.short	0x22
.short	0x1605
.long	0x0
.asciz	"core::ops::function::FnOnce"
.short	0xe
.short	0x1201
.long	0x2
.long	0x1145
.long	0x1067
.short	0xe
.short	0x1008
.long	0x1002
.byte	0x0
.byte	0x0
.short	0x2
.long	0x1151
.short	0x16
.short	0x1601
.long	0x1150
.long	0x1152
.asciz	"call_once"
.byte	242
.byte	241
.short	0x1a
.short	0x1605
.long	0x0
.asciz	"alloc::str::impl$4"
.byte	241
.short	0x16
.short	0x1601
.long	0x1154
.long	0x1144
.asciz	"to_owned"
.byte	243
.byte	242
.byte	241
.short	0x1e
.short	0x1605
.long	0x0
.asciz	"alloc::slice::impl$7"
.byte	243
.byte	242
.byte	241
.short	0xa
.short	0x1201
.long	0x1
.long	0x106f
.short	0xe
.short	0x1008
.long	0x100e
.byte	0x0
.byte	0x0
.short	0x1
.long	0x1157
.short	0x16
.short	0x1601
.long	0x1156
.long	0x1158
.asciz	"to_owned"
.byte	243
.byte	242
.byte	241
.short	0x1e
.short	0x1605
.long	0x0
.asciz	"alloc::slice::impl$0"
.byte	243
.byte	242
.byte	241
.short	0x12
.short	0x1601
.long	0x115a
.long	0x1158
.asciz	"to_vec"
.byte	241
.short	0xe
.short	0x1201
.long	0x2
.long	0x106f
.long	0x103e
.short	0xe
.short	0x1008
.long	0x100e
.byte	0x0
.byte	0x0
.short	0x2
.long	0x115c
.short	0x16
.short	0x1601
.long	0x115a
.long	0x115d
.asciz	"to_vec_in"
.byte	242
.byte	241
.short	0x1a
.short	0x1605
.long	0x0
.asciz	"alloc::slice::hack"
.byte	241
.short	0x12
.short	0x1601
.long	0x115f
.long	0x115d
.asciz	"to_vec"
.byte	241
.short	0x22
.short	0x1605
.long	0x0
.asciz	"alloc::slice::hack::impl$1"
.byte	241
.short	0x12
.short	0x1601
.long	0x1161
.long	0x115d
.asciz	"to_vec"
.byte	241
.short	0xe
.short	0x1201
.long	0x2
.long	0x23
.long	0x103e
.short	0x1a
.short	0x1009
.long	0x100e
.long	0x100e
.long	0x0
.byte	0x0
.byte	0x1
.short	0x2
.long	0x1163
.long	0x0
.short	0x1e
.short	0x1602
.long	0x100e
.long	0x1164
.asciz	"with_capacity_in"
.byte	243
.byte	242
.byte	241
.short	0x1a
.short	0x1009
.long	0x1025
.long	0x1025
.long	0x0
.byte	0x0
.byte	0x1
.short	0x2
.long	0x1163
.long	0x0
.short	0x1e
.short	0x1602
.long	0x1025
.long	0x1166
.asciz	"with_capacity_in"
.byte	243
.byte	242
.byte	241
.short	0x26
.short	0x1203
.short	0x1502
.short	0x3
.short	0x0
.asciz	"Uninitialized"
.short	0x1502
.short	0x3
.short	0x1
.asciz	"Zeroed"
.byte	243
.byte	242
.byte	241
.short	0x2a
.short	0x1507
.short	0x2
.short	0x0
.long	0x20
.long	0x1168
.asciz	"alloc::raw_vec::AllocInit"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1169
.long	0x100c
.long	0x0
.short	0x12
.short	0x1201
.long	0x3
.long	0x23
.long	0x1169
.long	0x103e
.short	0x1a
.short	0x1009
.long	0x1025
.long	0x1025
.long	0x0
.byte	0x0
.byte	0x1
.short	0x3
.long	0x116b
.long	0x0
.short	0x16
.short	0x1602
.long	0x1025
.long	0x116c
.asciz	"allocate_in"
.short	0x92
.short	0x1506
.short	0x0
.short	0x280
.long	0x0
.short	0x0
.asciz	"enum2$<core::result::Result<core::ptr::non_null::NonNull<slice2$<u8> >,core::alloc::AllocError> >"
.asciz	"e855ef355975412e3b62084f1939acce"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1201
.long	0x2
.long	0x10cb
.long	0x109b
.short	0xe
.short	0x1008
.long	0x116e
.byte	0x0
.byte	0x0
.short	0x2
.long	0x116f
.short	0xa2
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::result::Result<core::ptr::non_null::NonNull<slice2$<u8> >,core::alloc::AllocError> >::Variant0"
.asciz	"e62dd7a05a0f561be08958de9849e934"
.byte	241
.short	0xa2
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::result::Result<core::ptr::non_null::NonNull<slice2$<u8> >,core::alloc::AllocError> >::Variant1"
.asciz	"a3c4cfc1c401a5b3d947c942aeeaabaf"
.byte	241
.short	0x3a
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1171
.short	0x0
.asciz	"variant0"
.byte	241
.short	0x150d
.short	0x3
.long	0x1172
.short	0x0
.asciz	"variant1"
.byte	241
.short	0x150d
.short	0x3
.long	0x23
.short	0x0
.asciz	"tag"
.byte	242
.byte	241
.short	0x92
.short	0x1506
.short	0x3
.short	0x600
.long	0x1173
.short	0x10
.asciz	"enum2$<core::result::Result<core::ptr::non_null::NonNull<slice2$<u8> >,core::alloc::AllocError> >"
.asciz	"e855ef355975412e3b62084f1939acce"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1174
.long	0x100c
.long	0x0
.short	0x9e
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::result::Result<core::ptr::non_null::NonNull<slice2$<u8> >,core::alloc::AllocError> >::Ok"
.asciz	"220993251839d669346e76f1bedbba88"
.byte	243
.byte	242
.byte	241
.short	0x7e
.short	0x1507
.short	0x2
.short	0x8
.long	0x75
.long	0x101b
.asciz	"enum2$<core::result::Result<core::ptr::non_null::NonNull<slice2$<u8> >,core::alloc::AllocError> >::VariantNames"
.short	0xe
.short	0x1606
.long	0x1177
.long	0x100c
.long	0x0
.short	0x4a
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1176
.short	0x0
.asciz	"value"
.short	0x150e
.short	0x3
.long	0x1177
.asciz	"NAME"
.byte	243
.byte	242
.byte	241
.short	0x150e
.short	0x3
.long	0x23
.asciz	"DISCR_BEGIN"
.short	0x150e
.short	0x3
.long	0x23
.asciz	"DISCR_END"
.byte	242
.byte	241
.short	0xa2
.short	0x1505
.short	0x4
.short	0x208
.long	0x1179
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::result::Result<core::ptr::non_null::NonNull<slice2$<u8> >,core::alloc::AllocError> >::Variant0"
.asciz	"e62dd7a05a0f561be08958de9849e934"
.byte	241
.short	0xe
.short	0x1606
.long	0x117a
.long	0x100c
.long	0x0
.short	0x9e
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::result::Result<core::ptr::non_null::NonNull<slice2$<u8> >,core::alloc::AllocError> >::Err"
.asciz	"e09cc008135d493fbe8533e69dd77cc8"
.byte	242
.byte	241
.short	0x36
.short	0x1203
.short	0x150d
.short	0x3
.long	0x117c
.short	0x0
.asciz	"value"
.short	0x150e
.short	0x3
.long	0x1177
.asciz	"NAME"
.byte	243
.byte	242
.byte	241
.short	0x150e
.short	0x3
.long	0x23
.asciz	"DISCR_EXACT"
.short	0xa2
.short	0x1505
.short	0x3
.short	0x208
.long	0x117d
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::result::Result<core::ptr::non_null::NonNull<slice2$<u8> >,core::alloc::AllocError> >::Variant1"
.asciz	"a3c4cfc1c401a5b3d947c942aeeaabaf"
.byte	241
.short	0xe
.short	0x1606
.long	0x117e
.long	0x100c
.long	0x0
.short	0x62
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::ptr::non_null::NonNull<slice2$<u8> >"
.asciz	"80a44ec53c75d572e8bd72c25c8373d0"
.byte	242
.byte	241
.short	0x12
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1180
.short	0x0
.asciz	"__0"
.byte	242
.byte	241
.short	0x9e
.short	0x1505
.short	0x1
.short	0x208
.long	0x1181
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::result::Result<core::ptr::non_null::NonNull<slice2$<u8> >,core::alloc::AllocError> >::Ok"
.asciz	"220993251839d669346e76f1bedbba88"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1182
.long	0x100c
.long	0x0
.short	0x4e
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::alloc::AllocError"
.asciz	"c2fb1860a3f4c9a1dd7b950c711aefd1"
.byte	241
.short	0x12
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1184
.short	0x0
.asciz	"__0"
.byte	242
.byte	241
.short	0x9e
.short	0x1505
.short	0x1
.short	0x208
.long	0x1185
.long	0x0
.long	0x0
.short	0x10
.asciz	"enum2$<core::result::Result<core::ptr::non_null::NonNull<slice2$<u8> >,core::alloc::AllocError> >::Err"
.asciz	"e09cc008135d493fbe8533e69dd77cc8"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1186
.long	0x100c
.long	0x0
.short	0x4e
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"ptr_const$<slice2$<u8> >"
.asciz	"7b54d414f2f329e57c9aa3e4ca07f4"
.byte	242
.byte	241
.short	0x16
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1188
.short	0x0
.asciz	"pointer"
.byte	242
.byte	241
.short	0x62
.short	0x1505
.short	0x1
.short	0x200
.long	0x1189
.long	0x0
.long	0x0
.short	0x10
.asciz	"core::ptr::non_null::NonNull<slice2$<u8> >"
.asciz	"80a44ec53c75d572e8bd72c25c8373d0"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x118a
.long	0x100c
.long	0x0
.short	0x4e
.short	0x1505
.short	0x0
.short	0x200
.long	0x104d
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::alloc::AllocError"
.asciz	"c2fb1860a3f4c9a1dd7b950c711aefd1"
.byte	241
.short	0xe
.short	0x1606
.long	0x118c
.long	0x100c
.long	0x0
.short	0x4e
.short	0x1505
.short	0x2
.short	0x200
.long	0x106a
.long	0x0
.long	0x0
.short	0x10
.asciz	"ptr_const$<slice2$<u8> >"
.asciz	"7b54d414f2f329e57c9aa3e4ca07f4"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x118e
.long	0x100c
.long	0x0
.short	0x16
.short	0x1601
.long	0x10ca
.long	0x1170
.asciz	"allocate"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1201
.long	0x2
.long	0x109b
.long	0x30
.short	0x1a
.short	0x1009
.long	0x116e
.long	0x103e
.long	0x10cb
.byte	0x0
.byte	0x1
.short	0x2
.long	0x1191
.long	0x0
.short	0x16
.short	0x1602
.long	0x103e
.long	0x1192
.asciz	"alloc_impl"
.byte	241
.short	0xa
.short	0x1201
.long	0x1
.long	0x109b
.short	0xe
.short	0x1008
.long	0x620
.byte	0x0
.byte	0x0
.short	0x1
.long	0x1194
.short	0x12
.short	0x1601
.long	0x10cf
.long	0x1195
.asciz	"alloc"
.byte	242
.byte	241
.short	0x26
.short	0x1605
.long	0x0
.asciz	"core::ptr::const_ptr::impl$0"
.byte	243
.byte	242
.byte	241
.short	0x12
.short	0x1201
.long	0x3
.long	0x620
.long	0x620
.long	0x23
.short	0xe
.short	0x1008
.long	0x3
.byte	0x0
.byte	0x0
.short	0x3
.long	0x1198
.short	0x22
.short	0x1601
.long	0x1197
.long	0x1199
.asciz	"copy_to_nonoverlapping"
.byte	241
.short	0x1a
.short	0x1605
.long	0x0
.asciz	"core::intrinsics"
.byte	243
.byte	242
.byte	241
.short	0x1e
.short	0x1601
.long	0x119b
.long	0x1199
.asciz	"copy_nonoverlapping"
.short	0xa
.short	0x1201
.long	0x1
.long	0x100e
.short	0x1a
.short	0x1009
.long	0x1002
.long	0x1002
.long	0x0
.byte	0x0
.byte	0x1
.short	0x1
.long	0x119d
.long	0x0
.short	0x1e
.short	0x1602
.long	0x1002
.long	0x119e
.asciz	"from_utf8_unchecked"
.short	0xe
.short	0x1201
.long	0x2
.long	0x620
.long	0x13
.short	0xe
.short	0x1008
.long	0x620
.byte	0x0
.byte	0x0
.short	0x2
.long	0x11a0
.short	0x12
.short	0x1601
.long	0x1197
.long	0x11a1
.asciz	"offset"
.byte	241
.short	0x16
.short	0x1605
.long	0x0
.asciz	"core::cmp::Ord"
.byte	241
.short	0x12
.short	0x1201
.long	0x3
.long	0x13
.long	0x13
.long	0x13
.short	0xe
.short	0x1008
.long	0x13
.byte	0x0
.byte	0x0
.short	0x3
.long	0x11a4
.short	0x12
.short	0x1601
.long	0x11a3
.long	0x11a5
.asciz	"clamp"
.byte	242
.byte	241
.short	0x12
.short	0x1605
.long	0x0
.asciz	"alloc::fmt"
.byte	241
.short	0xa
.short	0x1201
.long	0x1
.long	0x10d3
.short	0xe
.short	0x1008
.long	0x1002
.byte	0x0
.byte	0x0
.short	0x1
.long	0x11a8
.short	0x12
.short	0x1601
.long	0x11a7
.long	0x11a9
.asciz	"format"
.byte	241
.short	0x82
.short	0x1506
.short	0x0
.short	0x280
.long	0x0
.short	0x0
.asciz	"enum2$<core::result::Result<alloc::string::String,alloc::string::FromUtf16Error> >"
.asciz	"9a98389440b405d8c51abf8e17222da6"
.byte	242
.byte	241
.short	0x92
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::result::Result<alloc::string::String,alloc::string::FromUtf16Error> >::Variant0"
.asciz	"2e8d46576f61bcba588b416b73b2da42"
.short	0x92
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::result::Result<alloc::string::String,alloc::string::FromUtf16Error> >::Variant1"
.asciz	"959374b7124d98303766785c464654f3"
.short	0x3a
.short	0x1203
.short	0x150d
.short	0x3
.long	0x11ac
.short	0x0
.asciz	"variant0"
.byte	241
.short	0x150d
.short	0x3
.long	0x11ad
.short	0x0
.asciz	"variant1"
.byte	241
.short	0x150d
.short	0x3
.long	0x23
.short	0x0
.asciz	"tag"
.byte	242
.byte	241
.short	0x82
.short	0x1506
.short	0x3
.short	0x600
.long	0x11ae
.short	0x18
.asciz	"enum2$<core::result::Result<alloc::string::String,alloc::string::FromUtf16Error> >"
.asciz	"9a98389440b405d8c51abf8e17222da6"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x11af
.long	0x100c
.long	0x0
.short	0x8e
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::result::Result<alloc::string::String,alloc::string::FromUtf16Error> >::Ok"
.asciz	"a6fca7c2fc12c59158d961c6396b1606"
.byte	242
.byte	241
.short	0x72
.short	0x1507
.short	0x2
.short	0x8
.long	0x75
.long	0x101b
.asciz	"enum2$<core::result::Result<alloc::string::String,alloc::string::FromUtf16Error> >::VariantNames"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x11b2
.long	0x100c
.long	0x0
.short	0x4a
.short	0x1203
.short	0x150d
.short	0x3
.long	0x11b1
.short	0x0
.asciz	"value"
.short	0x150e
.short	0x3
.long	0x11b2
.asciz	"NAME"
.byte	243
.byte	242
.byte	241
.short	0x150e
.short	0x3
.long	0x23
.asciz	"DISCR_BEGIN"
.short	0x150e
.short	0x3
.long	0x23
.asciz	"DISCR_END"
.byte	242
.byte	241
.short	0x92
.short	0x1505
.short	0x4
.short	0x208
.long	0x11b4
.long	0x0
.long	0x0
.short	0x18
.asciz	"enum2$<core::result::Result<alloc::string::String,alloc::string::FromUtf16Error> >::Variant0"
.asciz	"2e8d46576f61bcba588b416b73b2da42"
.short	0xe
.short	0x1606
.long	0x11b5
.long	0x100c
.long	0x0
.short	0x8e
.short	0x1505
.short	0x0
.short	0x288
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"enum2$<core::result::Result<alloc::string::String,alloc::string::FromUtf16Error> >::Err"
.asciz	"7f6dc5a094a8b59df574d02def2b61c5"
.byte	241
.short	0x36
.short	0x1203
.short	0x150d
.short	0x3
.long	0x11b7
.short	0x0
.asciz	"value"
.short	0x150e
.short	0x3
.long	0x11b2
.asciz	"NAME"
.byte	243
.byte	242
.byte	241
.short	0x150e
.short	0x3
.long	0x23
.asciz	"DISCR_EXACT"
.short	0x92
.short	0x1505
.short	0x3
.short	0x208
.long	0x11b8
.long	0x0
.long	0x0
.short	0x18
.asciz	"enum2$<core::result::Result<alloc::string::String,alloc::string::FromUtf16Error> >::Variant1"
.asciz	"959374b7124d98303766785c464654f3"
.short	0xe
.short	0x1606
.long	0x11b9
.long	0x100c
.long	0x0
.short	0x12
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1002
.short	0x0
.asciz	"__0"
.byte	242
.byte	241
.short	0x8e
.short	0x1505
.short	0x1
.short	0x208
.long	0x11bb
.long	0x0
.long	0x0
.short	0x18
.asciz	"enum2$<core::result::Result<alloc::string::String,alloc::string::FromUtf16Error> >::Ok"
.asciz	"a6fca7c2fc12c59158d961c6396b1606"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x11bc
.long	0x100c
.long	0x0
.short	0x56
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"alloc::string::FromUtf16Error"
.asciz	"e35cf641a80b5ccea6bac39fa86c8306"
.byte	243
.byte	242
.byte	241
.short	0x12
.short	0x1203
.short	0x150d
.short	0x3
.long	0x11be
.short	0x0
.asciz	"__0"
.byte	242
.byte	241
.short	0x8e
.short	0x1505
.short	0x1
.short	0x208
.long	0x11bf
.long	0x0
.long	0x0
.short	0x18
.asciz	"enum2$<core::result::Result<alloc::string::String,alloc::string::FromUtf16Error> >::Err"
.asciz	"7f6dc5a094a8b59df574d02def2b61c5"
.byte	241
.short	0xe
.short	0x1606
.long	0x11c0
.long	0x100c
.long	0x0
.short	0x12
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1035
.short	0x0
.asciz	"__0"
.byte	242
.byte	241
.short	0x56
.short	0x1505
.short	0x1
.short	0x200
.long	0x11c2
.long	0x0
.long	0x0
.short	0x0
.asciz	"alloc::string::FromUtf16Error"
.asciz	"e35cf641a80b5ccea6bac39fa86c8306"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x11c3
.long	0x100c
.long	0x0
.short	0x56
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::panic::location::Location"
.asciz	"1ec8e848f91a59adafb1210038f1d31c"
.byte	241
.short	0xa
.short	0x1002
.long	0x11c5
.long	0x1000c
.short	0xe
.short	0x1201
.long	0x2
.long	0x11ab
.long	0x11c6
.short	0x1a
.short	0x1009
.long	0x1002
.long	0x11ab
.long	0x0
.byte	0x0
.byte	0x1
.short	0x2
.long	0x11c7
.long	0x0
.short	0x32
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1067
.short	0x0
.asciz	"file"
.byte	241
.short	0x150d
.short	0x3
.long	0x75
.short	0x10
.asciz	"line"
.byte	241
.short	0x150d
.short	0x3
.long	0x75
.short	0x14
.asciz	"col"
.byte	242
.byte	241
.short	0x56
.short	0x1505
.short	0x3
.short	0x200
.long	0x11c9
.long	0x0
.long	0x0
.short	0x18
.asciz	"core::panic::location::Location"
.asciz	"1ec8e848f91a59adafb1210038f1d31c"
.byte	241
.short	0xe
.short	0x1606
.long	0x11ca
.long	0x100c
.long	0x0
.short	0x12
.short	0x1602
.long	0x11ab
.long	0x11c8
.asciz	"unwrap"
.byte	241
.short	0x4a
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::fmt::UnsafeArg"
.asciz	"432f3e5179e6907a53b8d6c71f14d6ae"
.short	0x16
.short	0x1201
.long	0x4
.long	0x10d4
.long	0x10d6
.long	0x10fc
.long	0x11cd
.short	0x1a
.short	0x1009
.long	0x10d3
.long	0x10d3
.long	0x0
.byte	0x0
.byte	0x1
.short	0x4
.long	0x11ce
.long	0x0
.short	0x16
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1035
.short	0x0
.asciz	"_private"
.byte	241
.short	0x4a
.short	0x1505
.short	0x1
.short	0x200
.long	0x11d0
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::fmt::UnsafeArg"
.asciz	"432f3e5179e6907a53b8d6c71f14d6ae"
.short	0xe
.short	0x1606
.long	0x11d1
.long	0x100c
.long	0x0
.short	0x1e
.short	0x1602
.long	0x10d3
.long	0x11cf
.asciz	"new_v1_formatted"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1008
.long	0x3
.byte	0x0
.byte	0x0
.short	0x1
.long	0x1068
.short	0x1a
.short	0x1601
.long	0x107c
.long	0x11d4
.asciz	"drop_in_place"
.byte	242
.byte	241
.short	0xe
.short	0x1201
.long	0x2
.long	0x10d4
.long	0x10d6
.short	0x1a
.short	0x1009
.long	0x10d3
.long	0x10d3
.long	0x0
.byte	0x0
.byte	0x1
.short	0x2
.long	0x11d6
.long	0x0
.short	0x12
.short	0x1602
.long	0x10d3
.long	0x11d7
.asciz	"new_v1"
.byte	241
.short	0x1a
.short	0x1605
.long	0x0
.asciz	"core::fmt::impl$59"
.byte	241
.short	0xa
.short	0x1002
.long	0x1003
.long	0x1000c
.short	0xe
.short	0x1201
.long	0x2
.long	0x11da
.long	0x1005
.short	0xe
.short	0x1008
.long	0x1001
.byte	0x0
.byte	0x0
.short	0x2
.long	0x11db
.short	0xe
.short	0x1601
.long	0x11d9
.long	0x11dc
.asciz	"fmt"
.short	0x1a
.short	0x1605
.long	0x0
.asciz	"core::fmt::impl$61"
.byte	241
.short	0xe
.short	0x1201
.long	0x2
.long	0x10da
.long	0x1005
.short	0xe
.short	0x1008
.long	0x1001
.byte	0x0
.byte	0x0
.short	0x2
.long	0x11df
.short	0xe
.short	0x1601
.long	0x11de
.long	0x11e0
.asciz	"fmt"
.short	0xa
.short	0x1002
.long	0x11be
.long	0x1000c
.short	0xa
.short	0x1201
.long	0x1
.long	0x11e2
.short	0xe
.short	0x1008
.long	0x3
.byte	0x0
.byte	0x0
.short	0x1
.long	0x11e3
.short	0x1a
.short	0x1601
.long	0x107c
.long	0x11e4
.asciz	"drop_in_place"
.byte	242
.byte	241
.short	0x4a
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"tuple$<ref$<str$> >"
.asciz	"de2486a155454ce755c27b6a96cceff0"
.byte	241
.short	0x4a
.short	0x1505
.short	0x1
.short	0x200
.long	0x113e
.long	0x0
.long	0x0
.short	0x10
.asciz	"tuple$<ref$<str$> >"
.asciz	"de2486a155454ce755c27b6a96cceff0"
.byte	241
.short	0xe
.short	0x1606
.long	0x11e7
.long	0x100c
.long	0x0
.short	0x1e
.short	0x1605
.long	0x0
.asciz	"alloc::string::impl$22"
.byte	241
.short	0xe
.short	0x1601
.long	0x11e9
.long	0x1007
.asciz	"fmt"
.short	0x1e
.short	0x1605
.long	0x0
.asciz	"arena_latency_slider"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1201
.long	0x2
.long	0x23
.long	0x620
.short	0xe
.short	0x1008
.long	0x3
.byte	0x0
.byte	0x0
.short	0x2
.long	0x11ec
.short	0x1a
.short	0x1601
.long	0x11eb
.long	0x11ed
.asciz	"set_text_string"
.short	0xa
.short	0x1002
.long	0x11ed
.long	0x1000c
.short	0x4e
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"skyline::hooks::InlineCtx"
.asciz	"f1d48516466d08bde8f85ae89a2122c"
.short	0xa
.short	0x1002
.long	0x11f0
.long	0x1000c
.short	0xa
.short	0x1201
.long	0x1
.long	0x11f1
.short	0xe
.short	0x1008
.long	0x3
.byte	0x0
.byte	0x0
.short	0x1
.long	0x11f2
.short	0x56
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"nnsdk::root::nn::os::CpuRegister"
.asciz	"d218c331d2e5d15b179a30903068cb5b"
.short	0xe
.short	0x1503
.long	0x11f4
.long	0x23
.short	0xe8
.byte	0
.byte	241
.short	0x16
.short	0x1203
.short	0x150d
.short	0x3
.long	0x11f5
.short	0x0
.asciz	"registers"
.short	0x4e
.short	0x1505
.short	0x1
.short	0x200
.long	0x11f6
.long	0x0
.long	0x0
.short	0xe8
.asciz	"skyline::hooks::InlineCtx"
.asciz	"f1d48516466d08bde8f85ae89a2122c"
.short	0xe
.short	0x1606
.long	0x11f7
.long	0x100c
.long	0x0
.short	0x5e
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"nnsdk::root::__BindgenUnionField<u64>"
.asciz	"adab2eb3e7176540bb71239556b86560"
.byte	243
.byte	242
.byte	241
.short	0x5e
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"nnsdk::root::__BindgenUnionField<u32>"
.asciz	"488bf07c4c824aae42082297176fac91"
.byte	243
.byte	242
.byte	241
.short	0x46
.short	0x1203
.short	0x150d
.short	0x3
.long	0x11f9
.short	0x0
.asciz	"x"
.short	0x150d
.short	0x3
.long	0x11fa
.short	0x0
.asciz	"w"
.short	0x150d
.short	0x3
.long	0x11fa
.short	0x0
.asciz	"r"
.short	0x150d
.short	0x3
.long	0x23
.short	0x0
.asciz	"bindgen_union_field"
.byte	242
.byte	241
.short	0x56
.short	0x1505
.short	0x4
.short	0x200
.long	0x11fb
.long	0x0
.long	0x0
.short	0x8
.asciz	"nnsdk::root::nn::os::CpuRegister"
.asciz	"d218c331d2e5d15b179a30903068cb5b"
.short	0xe
.short	0x1606
.long	0x11fc
.long	0x100c
.long	0x0
.short	0x56
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::marker::PhantomData<u64>"
.asciz	"d8df59822cad903a96bc2b5bf8bb5488"
.byte	242
.byte	241
.short	0x12
.short	0x1203
.short	0x150d
.short	0x3
.long	0x11fe
.short	0x0
.asciz	"__0"
.byte	242
.byte	241
.short	0x5e
.short	0x1505
.short	0x1
.short	0x200
.long	0x11ff
.long	0x0
.long	0x0
.short	0x0
.asciz	"nnsdk::root::__BindgenUnionField<u64>"
.asciz	"adab2eb3e7176540bb71239556b86560"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1200
.long	0x100c
.long	0x0
.short	0x56
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::marker::PhantomData<u32>"
.asciz	"e245d8e69de77d96fad608e109f5023c"
.byte	242
.byte	241
.short	0x12
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1202
.short	0x0
.asciz	"__0"
.byte	242
.byte	241
.short	0x5e
.short	0x1505
.short	0x1
.short	0x200
.long	0x1203
.long	0x0
.long	0x0
.short	0x0
.asciz	"nnsdk::root::__BindgenUnionField<u32>"
.asciz	"488bf07c4c824aae42082297176fac91"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1204
.long	0x100c
.long	0x0
.short	0x56
.short	0x1505
.short	0x0
.short	0x200
.long	0x104d
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::marker::PhantomData<u64>"
.asciz	"d8df59822cad903a96bc2b5bf8bb5488"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1206
.long	0x100c
.long	0x0
.short	0x56
.short	0x1505
.short	0x0
.short	0x200
.long	0x104d
.long	0x0
.long	0x0
.short	0x0
.asciz	"core::marker::PhantomData<u32>"
.asciz	"e245d8e69de77d96fad608e109f5023c"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1208
.long	0x100c
.long	0x0
.short	0x26
.short	0x1601
.long	0x11eb
.long	0x11f3
.asciz	"non_hdr_update_room_hook"
.byte	243
.byte	242
.byte	241
.short	0x1e
.short	0x1601
.long	0x11eb
.long	0x11f3
.asciz	"non_hdr_set_room_id"
.short	0xe
.short	0x1008
.long	0x3
.byte	0x0
.byte	0x0
.short	0x1
.long	0x10ab
.short	0x1e
.short	0x1601
.long	0x11eb
.long	0x120c
.asciz	"non_hdr_update_css2"
.short	0x26
.short	0x1601
.long	0x11eb
.long	0x11f3
.asciz	"non_hdr_set_online_latency"
.byte	241
.short	0x1e
.short	0x1601
.long	0x11eb
.long	0x11f3
.asciz	"bg_matchmaking_seq"
.byte	241
.short	0x16
.short	0x1601
.long	0x11eb
.long	0x11f3
.asciz	"arena_seq"
.byte	242
.byte	241
.short	0x2a
.short	0x1605
.long	0x0
.asciz	"arena_latency_slider::_::closure$0"
.byte	241
.short	0x46
.short	0x1506
.short	0x0
.short	0x280
.long	0x0
.short	0x0
.asciz	"libc::pthread_mutex_t"
.asciz	"64f18b063f72e06d8cb529fc6d67a8d2"
.byte	243
.byte	242
.byte	241
.short	0xa
.short	0x1002
.long	0x1212
.long	0x1000c
.short	0xa
.short	0x1201
.long	0x1
.long	0x1213
.short	0xe
.short	0x1008
.long	0x74
.byte	0x0
.byte	0x0
.short	0x1
.long	0x1214
.short	0x4e
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"libc::__pthread_mutex_s"
.asciz	"3eeff4299d0d45716e63cd69c62af98d"
.byte	241
.short	0xe
.short	0x1503
.long	0x20
.long	0x23
.short	0x28
.byte	0
.byte	241
.short	0xe
.short	0x1503
.long	0x23
.long	0x23
.short	0x28
.byte	0
.byte	241
.short	0x5e
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1216
.short	0x0
.asciz	"__data"
.byte	243
.byte	242
.byte	241
.short	0x150d
.short	0x3
.long	0x1217
.short	0x0
.asciz	"__size"
.byte	243
.byte	242
.byte	241
.short	0x150d
.short	0x3
.long	0x13
.short	0x0
.asciz	"__align"
.byte	242
.byte	241
.short	0x150d
.short	0x3
.long	0x1218
.short	0x0
.asciz	"_bindgen_union_align"
.byte	241
.short	0x46
.short	0x1506
.short	0x4
.short	0x600
.long	0x1219
.short	0x28
.asciz	"libc::pthread_mutex_t"
.asciz	"64f18b063f72e06d8cb529fc6d67a8d2"
.byte	243
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x121a
.long	0x100c
.long	0x0
.short	0x4e
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"libc::__pthread_list_t"
.asciz	"14145578af177a98ee36ed1a5f4e0721"
.byte	242
.byte	241
.short	0xa2
.short	0x1203
.short	0x150d
.short	0x3
.long	0x74
.short	0x0
.asciz	"__lock"
.byte	243
.byte	242
.byte	241
.short	0x150d
.short	0x3
.long	0x75
.short	0x4
.asciz	"__count"
.byte	242
.byte	241
.short	0x150d
.short	0x3
.long	0x74
.short	0x8
.asciz	"__owner"
.byte	242
.byte	241
.short	0x150d
.short	0x3
.long	0x75
.short	0xc
.asciz	"__nusers"
.byte	241
.short	0x150d
.short	0x3
.long	0x74
.short	0x10
.asciz	"__kind"
.byte	243
.byte	242
.byte	241
.short	0x150d
.short	0x3
.long	0x11
.short	0x14
.asciz	"__spins"
.byte	242
.byte	241
.short	0x150d
.short	0x3
.long	0x11
.short	0x16
.asciz	"__elision"
.short	0x150d
.short	0x3
.long	0x121c
.short	0x18
.asciz	"__list"
.byte	243
.byte	242
.byte	241
.short	0x4e
.short	0x1505
.short	0x8
.short	0x200
.long	0x121d
.long	0x0
.long	0x0
.short	0x28
.asciz	"libc::__pthread_mutex_s"
.asciz	"3eeff4299d0d45716e63cd69c62af98d"
.byte	241
.short	0xe
.short	0x1606
.long	0x121e
.long	0x100c
.long	0x0
.short	0xa
.short	0x1002
.long	0x121c
.long	0x1000c
.short	0x2a
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1220
.short	0x0
.asciz	"__prev"
.byte	243
.byte	242
.byte	241
.short	0x150d
.short	0x3
.long	0x1220
.short	0x8
.asciz	"__next"
.byte	243
.byte	242
.byte	241
.short	0x4e
.short	0x1505
.short	0x2
.short	0x200
.long	0x1221
.long	0x0
.long	0x0
.short	0x10
.asciz	"libc::__pthread_list_t"
.asciz	"14145578af177a98ee36ed1a5f4e0721"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x1222
.long	0x100c
.long	0x0
.short	0x36
.short	0x1601
.long	0x1211
.long	0x1215
.asciz	"_skyline_internal_pthread_mutex_lock_shim"
.byte	242
.byte	241
.short	0x42
.short	0x1506
.short	0x0
.short	0x280
.long	0x0
.short	0x0
.asciz	"enum2$<libc::c_void>"
.asciz	"ee1a6c80d4af6cc85b78efaddf6f28db"
.short	0xa
.short	0x1002
.long	0x1225
.long	0x1000c
.short	0xa
.short	0x1201
.long	0x1
.long	0x1226
.short	0xe
.short	0x1008
.long	0x3
.byte	0x0
.byte	0x0
.short	0x1
.long	0x1227
.short	0xa
.short	0x1002
.long	0x1228
.long	0x1000c
.short	0xe
.short	0x1201
.long	0x2
.long	0x675
.long	0x1229
.short	0xe
.short	0x1008
.long	0x74
.byte	0x0
.byte	0x0
.short	0x2
.long	0x122a
.short	0x42
.short	0x1506
.short	0x0
.short	0x600
.long	0x104d
.short	0x0
.asciz	"enum2$<libc::c_void>"
.asciz	"ee1a6c80d4af6cc85b78efaddf6f28db"
.short	0xe
.short	0x1606
.long	0x122c
.long	0x100c
.long	0x0
.short	0x36
.short	0x1601
.long	0x1211
.long	0x122b
.asciz	"_skyline_internal_pthread_key_create_shim"
.byte	242
.byte	241
.short	0xa
.short	0x1201
.long	0x1
.long	0x75
.short	0xe
.short	0x1008
.long	0x74
.byte	0x0
.byte	0x0
.short	0x1
.long	0x122f
.short	0x36
.short	0x1601
.long	0x1211
.long	0x1230
.asciz	"_skyline_internal_pthread_key_delete_shim"
.byte	242
.byte	241
.short	0xe
.short	0x1008
.long	0x3
.byte	0x0
.byte	0x0
.short	0x0
.long	0x1076
.short	0x1a
.short	0x1601
.long	0x11eb
.long	0x1232
.asciz	"__custom_fini"
.byte	242
.byte	241
.short	0x7a
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"impl$<alloc::string::FromUtf16Error, core::fmt::Debug>::vtable_type$"
.asciz	"92e6d29a57caa79e81615296c0d5b32"
.byte	241
.short	0xa
.short	0x1002
.long	0x1035
.long	0x1000c
.short	0x4e
.short	0x1203
.short	0x150d
.short	0x3
.long	0x1235
.short	0x0
.asciz	"drop_in_place"
.short	0x150d
.short	0x3
.long	0x23
.short	0x8
.asciz	"size"
.byte	241
.short	0x150d
.short	0x3
.long	0x23
.short	0x10
.asciz	"align"
.short	0x150d
.short	0x3
.long	0x1235
.short	0x18
.asciz	"__method3"
.short	0x7a
.short	0x1505
.short	0x4
.short	0x200
.long	0x1236
.long	0x0
.long	0x0
.short	0x20
.asciz	"impl$<alloc::string::FromUtf16Error, core::fmt::Debug>::vtable_type$"
.asciz	"92e6d29a57caa79e81615296c0d5b32"
.byte	241
.short	0xe
.short	0x1606
.long	0x1237
.long	0x100c
.long	0x0
.short	0x56
.short	0x1505
.short	0x0
.short	0x280
.long	0x0
.long	0x0
.long	0x0
.short	0x0
.asciz	"skyline::build::ModuleName<21>"
.asciz	"b1fdcd01302083c51b07f60b7bc0a7b9"
.byte	242
.byte	241
.short	0xe
.short	0x1503
.long	0x20
.long	0x23
.short	0x15
.byte	0
.byte	241
.short	0x3a
.short	0x1203
.short	0x150d
.short	0x3
.long	0x75
.short	0x0
.asciz	"unk"
.byte	242
.byte	241
.short	0x150d
.short	0x3
.long	0x75
.short	0x4
.asciz	"name_length"
.byte	242
.byte	241
.short	0x150d
.short	0x3
.long	0x123a
.short	0x8
.asciz	"name"
.byte	241
.short	0x56
.short	0x1505
.short	0x3
.short	0x200
.long	0x123b
.long	0x0
.long	0x0
.short	0x1d
.asciz	"skyline::build::ModuleName<21>"
.asciz	"b1fdcd01302083c51b07f60b7bc0a7b9"
.byte	242
.byte	241
.short	0xe
.short	0x1606
.long	0x123c
.long	0x100c
.long	0x0
.short	0x42
.short	0x1605
.long	0x0
.asciz	"C:\\Users\\blujay\\Documents\\Development\\arena-latency-slider"
.byte	241
.short	0x26
.short	0x1605
.long	0x0
.asciz	"src\\lib.rs\\@\\4bu6oet6mwhg1uu1"
.byte	242
.byte	241
.short	0xa
.short	0x1605
.long	0x0
.byte	0
.byte	243
.byte	242
.byte	241
.short	0x1a
.short	0x1603
.short	0x5
.long	0x123e
.long	0x0
.long	0x123f
.long	0x1240
.long	0x0
.byte	242
.byte	241
.globl	online_melee_any_scene_create
.def	online_melee_any_scene_create;
.scl	2;
.type	32;
.endef
.set online_melee_any_scene_create, bg_matchmaking_seq
.globl	main_menu
.def	main_menu;
.scl	2;
.type	32;
.endef
.set main_menu, bg_matchmaking_seq
.globl	main
.def	main;
.scl	2;
.type	32;
.endef
.set main, __custom_fini
.globl	__custom_init
.def	__custom_init;
.scl	2;
.type	32;
.endef
.set __custom_init, __custom_fini
"#);