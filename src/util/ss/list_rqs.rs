use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:8"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:8"]
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
#[c2rust::header_src = "/usr/include/bits/types/__sigset_t.h:8"]
pub mod __sigset_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "5:9"]
    pub struct __sigset_t {
        pub __val: [libc::c_ulong; 16],
    }
}
#[c2rust::header_src = "/usr/include/bits/types/sigset_t.h:8"]
pub mod sigset_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type sigset_t = __sigset_t;
    use super::__sigset_t_h::__sigset_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:8"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:8"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/types/__sigval_t.h:9"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/ss/ss_internal.h:8"]
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
        pub flags: C2RustUnnamed_10,
        pub abort: libc::c_int,
        pub exit_status: libc::c_int,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "73:5"]
    pub struct C2RustUnnamed_10 {
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
        #[c2rust::src_loc = "92:1"]
        pub fn ss_pager_create() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "98:18"]
        pub static mut _ss_table: *mut *mut ss_data;
    }
    /* _ss_internal_h */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/util/ss/ss.h:8"]
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
    /* _ss_h */
}
#[c2rust::header_src = "/usr/include/bits/sigaction.h:9"]
pub mod sigaction_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:8"]
    pub struct sigaction {
        pub __sigaction_handler: C2RustUnnamed,
        pub sa_mask: __sigset_t,
        pub sa_flags: libc::c_int,
        pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:5"]
    pub union C2RustUnnamed {
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
#[c2rust::header_src = "/usr/include/bits/types/siginfo_t.h:9"]
pub mod siginfo_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:9"]
    pub struct siginfo_t {
        pub si_signo: libc::c_int,
        pub si_errno: libc::c_int,
        pub si_code: libc::c_int,
        pub __pad0: libc::c_int,
        pub _sifields: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "51:5"]
    pub union C2RustUnnamed_0 {
        pub _pad: [libc::c_int; 28],
        pub _kill: C2RustUnnamed_9,
        pub _timer: C2RustUnnamed_8,
        pub _rt: C2RustUnnamed_7,
        pub _sigchld: C2RustUnnamed_6,
        pub _sigfault: C2RustUnnamed_3,
        pub _sigpoll: C2RustUnnamed_2,
        pub _sigsys: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "116:2"]
    pub struct C2RustUnnamed_1 {
        pub _call_addr: *mut libc::c_void,
        pub _syscall: libc::c_int,
        pub _arch: libc::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "108:2"]
    pub struct C2RustUnnamed_2 {
        pub si_band: libc::c_long,
        pub si_fd: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "89:2"]
    pub struct C2RustUnnamed_3 {
        pub si_addr: *mut libc::c_void,
        pub si_addr_lsb: libc::c_short,
        pub _bounds: C2RustUnnamed_4,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "94:6"]
    pub union C2RustUnnamed_4 {
        pub _addr_bnd: C2RustUnnamed_5,
        pub _pkey: __uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "97:3"]
    pub struct C2RustUnnamed_5 {
        pub _lower: *mut libc::c_void,
        pub _upper: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "79:2"]
    pub struct C2RustUnnamed_6 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
        pub si_status: libc::c_int,
        pub si_utime: __clock_t,
        pub si_stime: __clock_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "71:2"]
    pub struct C2RustUnnamed_7 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
        pub si_sigval: __sigval_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:2"]
    pub struct C2RustUnnamed_8 {
        pub si_tid: libc::c_int,
        pub si_overrun: libc::c_int,
        pub si_sigval: __sigval_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "56:2"]
    pub struct C2RustUnnamed_9 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
    }
    use super::types_h::{__uint32_t, __pid_t, __uid_t, __clock_t};
    use super::__sigval_t_h::__sigval_t;
}
#[c2rust::header_src = "/usr/include/signal.h:9"]
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
        #[c2rust::src_loc = "196:1"]
        pub fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "202:1"]
        pub fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:8"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "124:14"]
        pub fn strncpy(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "132:14"]
        pub fn strncat(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:8"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "279:1"]
        pub fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char)
         -> *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "626:1"]
        pub fn fputs(__s: *const libc::c_char, __stream: *mut FILE)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/sys/wait.h:11"]
pub mod wait_h {
    use super::types_h::__pid_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "88:1"]
        pub fn wait(__stat_loc: *mut libc::c_int) -> __pid_t;
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
pub use self::ss_internal_h::{pointer, ss_data, _ss_data, C2RustUnnamed_10,
                              ss_abbrev_info, ss_abbrev_list, _ss_abbrev_list,
                              ss_abbrev_entry, _ss_abbrev_entry,
                              ss_pager_create, _ss_table};
pub use self::ss_h::{_ss_request_entry, ss_request_entry, _ss_request_table,
                     ss_request_table};
pub use self::sigaction_h::{sigaction, C2RustUnnamed};
pub use self::siginfo_t_h::{siginfo_t, C2RustUnnamed_0, C2RustUnnamed_1,
                            C2RustUnnamed_2, C2RustUnnamed_3, C2RustUnnamed_4,
                            C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7,
                            C2RustUnnamed_8, C2RustUnnamed_9};
pub use self::signal_h::{__sighandler_t, sigaction, sigprocmask, sigemptyset,
                         sigaddset};
use self::string_h::{strncpy, strncat, strlen};
use self::stdio_h::{fclose, fdopen, fprintf, fputs};
use self::wait_h::wait;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1987, 1988 by MIT Student Information Processing Board
 *
 * For copyright information, see copyright.h.
 */
/* "lint returns a value which is sometimes ignored" */
/* !lint */
/* lint */
#[c2rust::src_loc = "19:19"]
static mut twentyfive_spaces: [libc::c_char; 26] =
    [32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
     32, 32, 32, 32, 32, 32, 32, 0];
#[c2rust::src_loc = "21:19"]
static mut NL: [libc::c_char; 2] = [10, 0];
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
#[c2rust::src_loc = "23:1"]
pub unsafe extern "C" fn ss_list_requests(mut argc: libc::c_int,
                                          mut argv:
                                              *const *const libc::c_char,
                                          mut sci_idx: libc::c_int,
                                          mut info_ptr: *mut libc::c_void) {
    let mut entry: *const ss_request_entry =
        0 as *const ss_request_entry; /* FD_CLOEXEC set */
    let mut name: *const *const libc::c_char =
        0 as *const *const libc::c_char;
    let mut spacing: libc::c_int = 0;
    let mut table: *mut *const ss_request_table =
        0 as *mut *const ss_request_table;
    let mut buffer: [libc::c_char; 8192] = [0; 8192];
    let mut output: *mut FILE = 0 as *mut FILE;
    let mut fd: libc::c_int = 0;
    let mut nsig: sigaction =
        sigaction{__sigaction_handler: C2RustUnnamed{sa_handler: None,},
                  sa_mask: __sigset_t{__val: [0; 16],},
                  sa_flags: 0,
                  sa_restorer: None,};
    let mut osig: sigaction =
        sigaction{__sigaction_handler: C2RustUnnamed{sa_handler: None,},
                  sa_mask: __sigset_t{__val: [0; 16],},
                  sa_flags: 0,
                  sa_restorer: None,};
    let mut nmask: sigset_t = __sigset_t{__val: [0; 16],};
    let mut omask: sigset_t = __sigset_t{__val: [0; 16],};
    let mut waitb: libc::c_int = 0;
    sigemptyset(&mut nmask);
    sigaddset(&mut nmask, 2 as libc::c_int);
    sigprocmask(0 as libc::c_int, &mut nmask, &mut omask);
    nsig.__sigaction_handler.sa_handler =
        ::std::mem::transmute::<libc::intptr_t,
                                __sighandler_t>(1 as libc::c_int as
                                                    libc::intptr_t);
    sigemptyset(&mut nsig.sa_mask);
    nsig.sa_flags = 0 as libc::c_int;
    sigaction(2 as libc::c_int, &mut nsig, &mut osig);
    fd = ss_pager_create();
    output = fdopen(fd, b"w\x00" as *const u8 as *const libc::c_char);
    sigprocmask(2 as libc::c_int, &mut omask, 0 as *mut sigset_t);
    fprintf(output,
            b"Available %s requests:\n\n\x00" as *const u8 as
                *const libc::c_char,
            (**_ss_table.offset(sci_idx as isize)).subsystem_name);
    table = (**_ss_table.offset(sci_idx as isize)).rqt_tables;
    while !(*table).is_null() {
        entry = (**table).requests;
        while !(*entry).command_names.is_null() {
            spacing = -(2 as libc::c_int);
            buffer[0 as libc::c_int as usize] =
                '\u{0}' as i32 as libc::c_char;
            if !((*entry).flags & 0x1 as libc::c_int != 0) {
                buffer[(::std::mem::size_of::<[libc::c_char; 8192]>() as
                            libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                            libc::c_ulong) as
                           usize] = '\u{0}' as i32 as libc::c_char;
                name = (*entry).command_names;
                while !(*name).is_null() {
                    let mut len: libc::c_int = strlen(*name) as libc::c_int;
                    strncat(buffer.as_mut_ptr(), *name,
                            (::std::mem::size_of::<[libc::c_char; 8192]>() as
                                 libc::c_ulong).wrapping_sub(1 as libc::c_int
                                                                 as
                                                                 libc::c_ulong).wrapping_sub(strlen(buffer.as_mut_ptr())));
                    spacing += len + 2 as libc::c_int;
                    if !(*name.offset(1 as libc::c_int as isize)).is_null() {
                        strncat(buffer.as_mut_ptr(),
                                b", \x00" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 8192]>()
                                     as
                                     libc::c_ulong).wrapping_sub(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong).wrapping_sub(strlen(buffer.as_mut_ptr())));
                    }
                    name = name.offset(1)
                }
                if spacing > 23 as libc::c_int {
                    strncat(buffer.as_mut_ptr(), NL.as_ptr(),
                            (::std::mem::size_of::<[libc::c_char; 8192]>() as
                                 libc::c_ulong).wrapping_sub(1 as libc::c_int
                                                                 as
                                                                 libc::c_ulong).wrapping_sub(strlen(buffer.as_mut_ptr())));
                    fputs(buffer.as_mut_ptr(), output);
                    spacing = 0 as libc::c_int;
                    buffer[0 as libc::c_int as usize] =
                        '\u{0}' as i32 as libc::c_char
                }
                strncat(buffer.as_mut_ptr(), twentyfive_spaces.as_ptr(),
                        (::std::mem::size_of::<[libc::c_char; 8192]>() as
                             libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                             libc::c_ulong).wrapping_sub((25
                                                                                              as
                                                                                              libc::c_int
                                                                                              -
                                                                                              spacing)
                                                                                             as
                                                                                             libc::c_ulong));
                strncpy(buffer.as_mut_ptr().offset(25 as libc::c_int as
                                                       isize),
                        (*entry).info_string,
                        (::std::mem::size_of::<[libc::c_char; 8192]>() as
                             libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                             libc::c_ulong).wrapping_sub(25
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong));
                strncat(buffer.as_mut_ptr(), NL.as_ptr(),
                        (::std::mem::size_of::<[libc::c_char; 8192]>() as
                             libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                             libc::c_ulong).wrapping_sub(strlen(buffer.as_mut_ptr())));
                fputs(buffer.as_mut_ptr(), output);
            }
            entry = entry.offset(1)
        }
        table = table.offset(1)
    }
    fclose(output);
    wait(&mut waitb);
    sigaction(2 as libc::c_int, &mut osig, 0 as *mut sigaction);
}
