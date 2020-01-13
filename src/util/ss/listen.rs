use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:10"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:10"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "154:1"]
    pub type __pid_t = libc::c_int;
    #[c2rust::src_loc = "156:1"]
    pub type __clock_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/__sigset_t.h:10"]
pub mod __sigset_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "5:9"]
    pub struct __sigset_t {
        pub __val: [libc::c_ulong; 16],
    }
}
#[c2rust::header_src = "/usr/include/bits/types/sigset_t.h:10"]
pub mod sigset_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type sigset_t = __sigset_t;
    use super::__sigset_t_h::__sigset_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:10"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    #[c2rust::src_loc = "43:1"]
    pub type _IO_lock_t = ();
    use super::types_h::{__off_t, __off64_t};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "38:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "37:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "36:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:10"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/types/__sigval_t.h:13"]
pub mod __sigval_t_h {
    #[c2rust::src_loc = "30:1"]
    pub type __sigval_t = sigval;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:7"]
    pub union sigval {
        pub sival_int: libc::c_int,
        pub sival_ptr: *mut libc::c_void,
    }
}
#[c2rust::header_src = "/usr/include/bits/setjmp.h:10"]
pub mod setjmp_h {
    #[c2rust::src_loc = "31:1"]
    pub type __jmp_buf = [libc::c_long; 8];
}
#[c2rust::header_src = "/usr/include/setjmp.h:12"]
pub mod include_setjmp_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:8"]
    pub struct __jmp_buf_tag {
        pub __jmpbuf: __jmp_buf,
        pub __mask_was_saved: libc::c_int,
        pub __saved_mask: __sigset_t,
    }
    #[c2rust::src_loc = "45:1"]
    pub type jmp_buf = [__jmp_buf_tag; 1];
    use super::setjmp_h::__jmp_buf;
    use super::__sigset_t_h::__sigset_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "58:12"]
        pub fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "67:13"]
        pub fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/ss/ss_internal.h:10"]
pub mod ss_internal_h {
    #[c2rust::src_loc = "13:1"]
    pub type pointer = *mut libc::c_void;
    #[c2rust::src_loc = "57:1"]
    pub type ss_data = _ss_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "57:16"]
    pub struct _ss_data {
        pub subsystem_name: *mut libc::c_char,
        pub subsystem_version: *mut libc::c_char,
        pub argc: libc::c_int,
        pub argv: *mut *mut libc::c_char,
        pub current_request: *const libc::c_char,
        pub info_dirs: *mut *mut libc::c_char,
        pub info_ptr: pointer,
        pub prompt: *mut libc::c_char,
        pub rqt_tables: *mut *const ss_request_table,
        pub abbrev_info: *mut ss_abbrev_info,
        pub flags: C2RustUnnamed,
        pub abort: libc::c_int,
        pub exit_status: libc::c_int,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "73:5"]
    pub struct C2RustUnnamed {
        #[bitfield(name = "escape_disabled", ty = "libc::c_uint", bits =
                   "0..=0")]
        #[bitfield(name = "abbrevs_disabled", ty = "libc::c_uint", bits =
                   "1..=1")]
        pub escape_disabled_abbrevs_disabled: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 3],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:9"]
    pub struct ss_abbrev_info {
        pub abbrevs: [ss_abbrev_list; 127],
    }
    #[c2rust::src_loc = "47:1"]
    pub type ss_abbrev_list = _ss_abbrev_list;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "47:16"]
    pub struct _ss_abbrev_list {
        pub n_abbrevs: libc::c_int,
        pub first_abbrev: *mut ss_abbrev_entry,
    }
    #[c2rust::src_loc = "41:1"]
    pub type ss_abbrev_entry = _ss_abbrev_entry;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "41:16"]
    pub struct _ss_abbrev_entry {
        pub name: *mut libc::c_char,
        pub abbrev: *mut *mut libc::c_char,
        #[bitfield(name = "beginning_of_line", ty = "libc::c_uint", bits =
                   "0..=0")]
        pub beginning_of_line: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
    }
    use super::ss_h::ss_request_table;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "98:18"]
        pub static mut _ss_table: *mut *mut ss_data;
    }
    /* _ss_internal_h */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/util/ss/ss.h:10"]
pub mod ss_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "22:27"]
    pub struct _ss_request_entry {
        pub command_names: *const *const libc::c_char,
        pub function: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _:
                                                      *const *const libc::c_char,
                                                  _: libc::c_int,
                                                  _: *mut libc::c_void)
                                 -> ()>,
        pub info_string: *const libc::c_char,
        pub flags: libc::c_int,
    }
    #[c2rust::src_loc = "22:1"]
    pub type ss_request_entry = _ss_request_entry;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "29:27"]
    pub struct _ss_request_table {
        pub version: libc::c_int,
        pub requests: *const ss_request_entry,
    }
    #[c2rust::src_loc = "29:1"]
    pub type ss_request_table = _ss_request_table;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "53:1"]
        pub fn ss_error(_: libc::c_int, _: libc::c_long,
                        _: *const libc::c_char, _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "70:1"]
        pub fn ss_execute_line(_: libc::c_int, _: *mut libc::c_char)
         -> libc::c_int;
    }
    /* _ss_h */
}
#[c2rust::header_src = "/usr/include/bits/sigaction.h:13"]
pub mod sigaction_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:8"]
    pub struct sigaction {
        pub __sigaction_handler: C2RustUnnamed_0,
        pub sa_mask: __sigset_t,
        pub sa_flags: libc::c_int,
        pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:5"]
    pub union C2RustUnnamed_0 {
        pub sa_handler: __sighandler_t,
        pub sa_sigaction: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *mut siginfo_t,
                                                      _: *mut libc::c_void)
                                     -> ()>,
    }
    use super::__sigset_t_h::__sigset_t;
    use super::signal_h::__sighandler_t;
    use super::siginfo_t_h::siginfo_t;
}
#[c2rust::header_src = "/usr/include/bits/types/siginfo_t.h:13"]
pub mod siginfo_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:9"]
    pub struct siginfo_t {
        pub si_signo: libc::c_int,
        pub si_errno: libc::c_int,
        pub si_code: libc::c_int,
        pub __pad0: libc::c_int,
        pub _sifields: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "51:5"]
    pub union C2RustUnnamed_1 {
        pub _pad: [libc::c_int; 28],
        pub _kill: C2RustUnnamed_10,
        pub _timer: C2RustUnnamed_9,
        pub _rt: C2RustUnnamed_8,
        pub _sigchld: C2RustUnnamed_7,
        pub _sigfault: C2RustUnnamed_4,
        pub _sigpoll: C2RustUnnamed_3,
        pub _sigsys: C2RustUnnamed_2,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "116:2"]
    pub struct C2RustUnnamed_2 {
        pub _call_addr: *mut libc::c_void,
        pub _syscall: libc::c_int,
        pub _arch: libc::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "108:2"]
    pub struct C2RustUnnamed_3 {
        pub si_band: libc::c_long,
        pub si_fd: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "89:2"]
    pub struct C2RustUnnamed_4 {
        pub si_addr: *mut libc::c_void,
        pub si_addr_lsb: libc::c_short,
        pub _bounds: C2RustUnnamed_5,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "94:6"]
    pub union C2RustUnnamed_5 {
        pub _addr_bnd: C2RustUnnamed_6,
        pub _pkey: __uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "97:3"]
    pub struct C2RustUnnamed_6 {
        pub _lower: *mut libc::c_void,
        pub _upper: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "79:2"]
    pub struct C2RustUnnamed_7 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
        pub si_status: libc::c_int,
        pub si_utime: __clock_t,
        pub si_stime: __clock_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "71:2"]
    pub struct C2RustUnnamed_8 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
        pub si_sigval: __sigval_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:2"]
    pub struct C2RustUnnamed_9 {
        pub si_tid: libc::c_int,
        pub si_overrun: libc::c_int,
        pub si_sigval: __sigval_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "56:2"]
    pub struct C2RustUnnamed_10 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
    }
    use super::types_h::{__uint32_t, __pid_t, __uid_t, __clock_t};
    use super::__sigval_t_h::__sigval_t;
}
#[c2rust::header_src = "/usr/include/signal.h:13"]
pub mod signal_h {
    #[c2rust::src_loc = "72:1"]
    pub type __sighandler_t
        =
        Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
    use super::sigaction_h::sigaction;
    use super::sigset_t_h::sigset_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "240:1"]
        pub fn sigaction(__sig: libc::c_int, __act: *const sigaction,
                         __oact: *mut sigaction) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "229:1"]
        pub fn sigprocmask(__how: libc::c_int, __set: *const sigset_t,
                           __oset: *mut sigset_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "202:1"]
        pub fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "196:1"]
        pub fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:10"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:10"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "138:14"]
        pub static mut stdout: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "522:1"]
        pub fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:10"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "225:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/editline/readline.h:18"]
pub mod readline_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "147:1"]
        pub fn readline(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "151:1"]
        pub fn add_history(_: *const libc::c_char) -> libc::c_int;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint32_t, __uid_t, __off_t, __off64_t, __pid_t,
                        __clock_t};
pub use self::__sigset_t_h::__sigset_t;
pub use self::sigset_t_h::sigset_t;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::__sigval_t_h::{__sigval_t, sigval};
pub use self::setjmp_h::__jmp_buf;
pub use self::include_setjmp_h::{__jmp_buf_tag, jmp_buf, _setjmp, longjmp};
pub use self::ss_internal_h::{pointer, ss_data, _ss_data, C2RustUnnamed,
                              ss_abbrev_info, ss_abbrev_list, _ss_abbrev_list,
                              ss_abbrev_entry, _ss_abbrev_entry, _ss_table};
pub use self::ss_h::{_ss_request_entry, ss_request_entry, _ss_request_table,
                     ss_request_table, ss_error, ss_execute_line};
pub use self::sigaction_h::{sigaction, C2RustUnnamed_0};
pub use self::siginfo_t_h::{siginfo_t, C2RustUnnamed_1, C2RustUnnamed_2,
                            C2RustUnnamed_3, C2RustUnnamed_4, C2RustUnnamed_5,
                            C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
                            C2RustUnnamed_9, C2RustUnnamed_10};
pub use self::signal_h::{__sighandler_t, sigaction, sigprocmask, sigaddset,
                         sigemptyset};
use self::stdlib_h::free;
use self::stdio_h::{stdout, putc};
use self::string_h::{strchr, memcpy};
use self::readline_h::{readline, add_history};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* util/ss/listen.c */
/*
 * Copyright 1987, 1988 by MIT Student Information Processing Board
 *
 * For copyright information, see copyright.h.
 */
#[c2rust::src_loc = "26:17"]
static mut current_info: *mut ss_data = 0 as *const ss_data as *mut ss_data;
#[c2rust::src_loc = "27:16"]
static mut listen_jmpb: jmp_buf =
    [__jmp_buf_tag{__jmpbuf: [0; 8],
                   __mask_was_saved: 0,
                   __saved_mask: __sigset_t{__val: [0; 16],},}; 1];
#[c2rust::src_loc = "57:1"]
unsafe extern "C" fn listen_int_handler(mut signo: libc::c_int) {
    putc('\n' as i32, stdout); /* fgets is not signal-safe */
    longjmp(listen_jmpb.as_mut_ptr(), 1 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "64:1"]
pub unsafe extern "C" fn ss_listen(mut sci_idx: libc::c_int) -> libc::c_int {
    let mut current_block: u64;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut info: *mut ss_data = 0 as *mut ss_data;
    let mut input: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut code: libc::c_int = 0;
    let mut old_jmpb: jmp_buf =
        [__jmp_buf_tag{__jmpbuf: [0; 8],
                       __mask_was_saved: 0,
                       __saved_mask: __sigset_t{__val: [0; 16],},}; 1];
    let mut old_info: *mut ss_data = current_info;
    let mut isig: sigaction =
        sigaction{__sigaction_handler: C2RustUnnamed_0{sa_handler: None,},
                  sa_mask: __sigset_t{__val: [0; 16],},
                  sa_flags: 0,
                  sa_restorer: None,};
    let mut csig: sigaction =
        sigaction{__sigaction_handler: C2RustUnnamed_0{sa_handler: None,},
                  sa_mask: __sigset_t{__val: [0; 16],},
                  sa_flags: 0,
                  sa_restorer: None,};
    let mut nsig: sigaction =
        sigaction{__sigaction_handler: C2RustUnnamed_0{sa_handler: None,},
                  sa_mask: __sigset_t{__val: [0; 16],},
                  sa_flags: 0,
                  sa_restorer: None,};
    let mut osig: sigaction =
        sigaction{__sigaction_handler: C2RustUnnamed_0{sa_handler: None,},
                  sa_mask: __sigset_t{__val: [0; 16],},
                  sa_flags: 0,
                  sa_restorer: None,};
    let mut nmask: sigset_t = __sigset_t{__val: [0; 16],};
    let mut omask: sigset_t = __sigset_t{__val: [0; 16],};
    info = *_ss_table.offset(sci_idx as isize);
    current_info = info;
    (*info).abort = 0 as libc::c_int;
    csig.__sigaction_handler.sa_handler =
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> ()>,
                                __sighandler_t>(None);
    sigemptyset(&mut nmask);
    sigaddset(&mut nmask, 2 as libc::c_int);
    sigprocmask(0 as libc::c_int, &mut nmask, &mut omask);
    memcpy(old_jmpb.as_mut_ptr() as *mut libc::c_void,
           listen_jmpb.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<jmp_buf>() as libc::c_ulong);
    nsig.__sigaction_handler.sa_handler =
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> ()>,
                                __sighandler_t>(Some(::std::mem::transmute::<unsafe extern "C" fn(_:
                                                                                                      libc::c_int)
                                                                                 ->
                                                                                     (),
                                                                             unsafe extern "C" fn()
                                                                                 ->
                                                                                     ()>(listen_int_handler)));
    sigemptyset(&mut nsig.sa_mask);
    nsig.sa_flags = 0 as libc::c_int;
    sigaction(2 as libc::c_int, &mut nsig, &mut isig);
    _setjmp(listen_jmpb.as_mut_ptr());
    sigprocmask(2 as libc::c_int, &mut omask, 0 as *mut sigset_t);
    loop  {
        if !((*info).abort == 0) {
            current_block = 5529461102203738653;
            break ;
        }
        nsig.__sigaction_handler.sa_handler =
            ::std::mem::transmute::<Option<unsafe extern "C" fn() -> ()>,
                                    __sighandler_t>(Some(::std::mem::transmute::<unsafe extern "C" fn(_:
                                                                                                          libc::c_int)
                                                                                     ->
                                                                                         (),
                                                                                 unsafe extern "C" fn()
                                                                                     ->
                                                                                         ()>(listen_int_handler)));
        osig = csig;
        sigaction(18 as libc::c_int, &mut nsig, &mut csig);
        if ::std::mem::transmute::<__sighandler_t,
                                   Option<unsafe extern "C" fn()
                                              ->
                                                  ()>>(csig.__sigaction_handler.sa_handler)
               ==
               ::std::mem::transmute::<Option<unsafe extern "C" fn() -> ()>,
                                       Option<unsafe extern "C" fn()
                                                  ->
                                                      ()>>(Some(::std::mem::transmute::<unsafe extern "C" fn(_:
                                                                                                                 libc::c_int)
                                                                                            ->
                                                                                                (),
                                                                                        unsafe extern "C" fn()
                                                                                            ->
                                                                                                ()>(listen_int_handler)))
           {
            csig = osig
        }
        input = readline((*current_info).prompt);
        if input.is_null() {
            code = 748806 as libc::c_long as libc::c_int;
            current_block = 14963423777312756909;
            break ;
        } else {
            add_history(input);
            sigaction(18 as libc::c_int, &mut csig, 0 as *mut sigaction);
            code = ss_execute_line(sci_idx, input);
            if code as libc::c_long == 748804 as libc::c_long {
                let mut c: *mut libc::c_char = input;
                while *c as libc::c_int == ' ' as i32 ||
                          *c as libc::c_int == '\t' as i32 {
                    c = c.offset(1)
                }
                cp = strchr(c, ' ' as i32);
                if !cp.is_null() { *cp = '\u{0}' as i32 as libc::c_char }
                cp = strchr(c, '\t' as i32);
                if !cp.is_null() { *cp = '\u{0}' as i32 as libc::c_char }
                ss_error(sci_idx, 0 as libc::c_int as libc::c_long,
                         b"Unknown request \"%s\".  Type \"?\" for a request list.\x00"
                             as *const u8 as *const libc::c_char, c);
            }
            free(input as *mut libc::c_void);
        }
    }
    match current_block {
        5529461102203738653 => { code = 0 as libc::c_int }
        _ => { }
    }
    sigaction(2 as libc::c_int, &mut isig, 0 as *mut sigaction);
    memcpy(listen_jmpb.as_mut_ptr() as *mut libc::c_void,
           old_jmpb.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<jmp_buf>() as libc::c_ulong);
    current_info = old_info;
    return code;
}
#[no_mangle]
#[c2rust::src_loc = "169:1"]
pub unsafe extern "C" fn ss_abort_subsystem(mut sci_idx: libc::c_int,
                                            mut code: libc::c_int) {
    (**_ss_table.offset(sci_idx as isize)).abort = 1 as libc::c_int;
    (**_ss_table.offset(sci_idx as isize)).exit_status = code;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1987, 1988 by MIT Student Information Processing Board
 *
 * For copyright information, see mit-sipb-copyright.h.
 */
/* whatever */
/* foo */
/* NULL */
/* 0 */
/* DEFAULT VALUES */
/* SS_RP_V1 */
/* call for unknown command */
#[no_mangle]
#[c2rust::src_loc = "178:1"]
pub unsafe extern "C" fn ss_quit(mut argc: libc::c_int,
                                 mut argv: *const *const libc::c_char,
                                 mut sci_idx: libc::c_int,
                                 mut infop: pointer) {
    ss_abort_subsystem(sci_idx, 0 as libc::c_int);
}
