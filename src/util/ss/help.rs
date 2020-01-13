use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/include/bits/types.h:8"]
pub mod types_h {
    #[c2rust::src_loc = "154:1"]
    pub type __pid_t = libc::c_int;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/ss/ss_internal.h:14"]
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
        #[no_mangle]
        #[c2rust::src_loc = "91:1"]
        pub fn ss_page_stdin();
    }
    /* _ss_internal_h */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/util/ss/ss.h:14"]
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
        #[c2rust::src_loc = "49:1"]
        pub fn ss_list_requests(_: libc::c_int, _: *const *const libc::c_char,
                                _: libc::c_int, _: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "58:1"]
        pub fn ss_perror(_: libc::c_int, _: libc::c_long,
                         _: *const libc::c_char);
    }
    /* _ss_h */
}
#[c2rust::header_src = "/usr/include/dirent.h:97"]
pub mod dirent_h {
    #[c2rust::src_loc = "127:1"]
    pub type DIR = __dirstream;
    extern "C" {
        #[c2rust::src_loc = "127:16"]
        pub type __dirstream;
        #[no_mangle]
        #[c2rust::src_loc = "134:1"]
        pub fn opendir(__name: *const libc::c_char) -> *mut DIR;
        #[no_mangle]
        #[c2rust::src_loc = "149:1"]
        pub fn closedir(__dirp: *mut DIR) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:10"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:11"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "195:1"]
        pub fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/sys/wait.h:13"]
pub mod wait_h {
    use super::types_h::__pid_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "88:1"]
        pub fn wait(__stat_loc: *mut libc::c_int) -> __pid_t;
    }
}
#[c2rust::header_src = "/usr/include/string.h:14"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "132:14"]
        pub fn strncat(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "124:14"]
        pub fn strncpy(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:14"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:14"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:14"]
pub mod unistd_h {
    use super::types_h::__pid_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "534:1"]
        pub fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "756:1"]
        pub fn fork() -> __pid_t;
    }
}
pub use self::types_h::__pid_t;
pub use self::ss_internal_h::{pointer, ss_data, _ss_data, C2RustUnnamed,
                              ss_abbrev_info, ss_abbrev_list, _ss_abbrev_list,
                              ss_abbrev_entry, _ss_abbrev_entry, _ss_table,
                              ss_page_stdin};
pub use self::ss_h::{_ss_request_entry, ss_request_entry, _ss_request_table,
                     ss_request_table, ss_list_requests, ss_perror};
pub use self::dirent_h::{DIR, __dirstream, opendir, closedir};
use self::errno_h::__errno_location;
use self::fcntl_h::open;
use self::wait_h::wait;
use self::string_h::{strdup, strlen, strcmp, strncat, strncpy};
use self::stdlib_h::realloc;
use self::stdio_h::snprintf;
use self::unistd_h::{close, dup2, fork};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1987, 1988 by MIT Student Information Processing Board
 *
 * For copyright info, see copyright.h.
 */
/* just for O_* */
#[no_mangle]
#[c2rust::src_loc = "18:1"]
pub unsafe extern "C" fn ss_help(mut argc: libc::c_int,
                                 mut argv: *const *const libc::c_char,
                                 mut sci_idx: libc::c_int,
                                 mut info_ptr: pointer) {
    let mut current_block: u64;
    let mut buffer: [libc::c_char; 4096] = [0; 4096];
    let mut request_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut code: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut child: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut info: *mut ss_data = 0 as *mut ss_data;
    code = 0 as libc::c_int;
    request_name = (**_ss_table.offset(sci_idx as isize)).current_request;
    if code != 0 as libc::c_int {
        ss_perror(sci_idx, code as libc::c_long,
                  b"\x00" as *const u8 as *const libc::c_char);
        return
        /* no ss_abort_line, if invalid invocation */
    }
    if argc == 1 as libc::c_int {
        ss_list_requests(argc, argv, sci_idx, info_ptr);
        return
    } else {
        if argc != 2 as libc::c_int {
            /* should do something better than this */
            snprintf(buffer.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 4096]>() as
                         libc::c_ulong,
                     b"usage:\n\t%s [topic|command]\nor\t%s\n\x00" as
                         *const u8 as *const libc::c_char, request_name,
                     request_name); /* put file on stdin */
            ss_perror(sci_idx, 0 as libc::c_int as libc::c_long,
                      buffer.as_mut_ptr()); /* what can we do if it fails? */
            return
        }
    }
    info = *_ss_table.offset(sci_idx as isize);
    if (*info).info_dirs.is_null() {
        ss_perror(sci_idx, 748803 as libc::c_long,
                  0 as *mut libc::c_void as *mut libc::c_char);
        return
    }
    if (*(*info).info_dirs.offset(0 as libc::c_int as isize)).is_null() {
        ss_perror(sci_idx, 748803 as libc::c_long,
                  0 as *mut libc::c_void as *mut libc::c_char);
        return
    }
    idx = 0 as libc::c_int;
    loop  {
        if (*(*info).info_dirs.offset(idx as isize)).is_null() {
            current_block = 5689316957504528238;
            break ;
        }
        strncpy(buffer.as_mut_ptr(), *(*info).info_dirs.offset(idx as isize),
                (::std::mem::size_of::<[libc::c_char; 4096]>() as
                     libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                     libc::c_ulong));
        buffer[(::std::mem::size_of::<[libc::c_char; 4096]>() as
                    libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                    libc::c_ulong) as usize] =
            '\u{0}' as i32 as libc::c_char;
        strncat(buffer.as_mut_ptr(),
                b"/\x00" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 4096]>() as
                     libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                     libc::c_ulong).wrapping_sub(strlen(buffer.as_mut_ptr())));
        strncat(buffer.as_mut_ptr(), *argv.offset(1 as libc::c_int as isize),
                (::std::mem::size_of::<[libc::c_char; 4096]>() as
                     libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                     libc::c_ulong).wrapping_sub(strlen(buffer.as_mut_ptr())));
        strncat(buffer.as_mut_ptr(),
                b".info\x00" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 4096]>() as
                     libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                     libc::c_ulong).wrapping_sub(strlen(buffer.as_mut_ptr())));
        fd =
            open(&mut *buffer.as_mut_ptr().offset(0 as libc::c_int as isize)
                     as *mut libc::c_char, 0 as libc::c_int);
        if fd >= 0 as libc::c_int {
            current_block = 7071516361779662619;
            break ;
        }
        idx += 1
    }
    match current_block {
        5689316957504528238 => {
            fd =
                open(&mut *buffer.as_mut_ptr().offset(0 as libc::c_int as
                                                          isize) as
                         *mut libc::c_char, 0 as libc::c_int);
            if fd < 0 as libc::c_int {
                let mut buf: [libc::c_char; 4096] = [0; 4096];
                strncpy(buf.as_mut_ptr(),
                        b"No info found for \x00" as *const u8 as
                            *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 4096]>() as
                             libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                             libc::c_ulong));
                buf[(::std::mem::size_of::<[libc::c_char; 4096]>() as
                         libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                         libc::c_ulong) as
                        usize] = '\u{0}' as i32 as libc::c_char;
                strncat(buf.as_mut_ptr(),
                        *argv.offset(1 as libc::c_int as isize),
                        (::std::mem::size_of::<[libc::c_char; 4096]>() as
                             libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                             libc::c_ulong).wrapping_sub(strlen(buf.as_mut_ptr())));
                ss_perror(sci_idx, 0 as libc::c_int as libc::c_long,
                          buf.as_mut_ptr());
                return
            }
        }
        _ => { }
    }
    child = fork();
    match child {
        -1 => {
            ss_perror(sci_idx, *__errno_location() as libc::c_long,
                      b"Can\'t fork for pager\x00" as *const u8 as
                          *const libc::c_char);
            close(fd);
            return
        }
        0 => { dup2(fd, 0 as libc::c_int); ss_page_stdin(); }
        _ => { }
    }
    close(fd);
    /* do nothing if wrong pid */
    while wait(0 as *mut libc::c_void as *mut libc::c_int) != child {
    }; /* get number of non-NULL dir entries */
}
#[no_mangle]
#[c2rust::src_loc = "100:5"]
pub unsafe extern "C" fn ss_add_info_dir(mut sci_idx: libc::c_int,
                                         mut info_dir: *mut libc::c_char,
                                         mut code_ptr: *mut libc::c_int) {
    let mut info: *mut ss_data = 0 as *mut ss_data;
    let mut d: *mut DIR = 0 as *mut DIR;
    let mut n_dirs: libc::c_int = 0;
    let mut dirs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    info = *_ss_table.offset(sci_idx as isize);
    if info_dir.is_null() || *info_dir as libc::c_int == '\u{0}' as i32 {
        *code_ptr = 748803 as libc::c_long as libc::c_int;
        return
    }
    d = opendir(info_dir);
    if d.is_null() { *code_ptr = *__errno_location(); return }
    closedir(d);
    dirs = (*info).info_dirs;
    n_dirs = 0 as libc::c_int;
    while !(*dirs.offset(n_dirs as isize)).is_null() { n_dirs += 1 }
    dirs =
        realloc(dirs as *mut libc::c_char as *mut libc::c_void,
                ((n_dirs + 2 as libc::c_int) as libc::c_uint as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                     as libc::c_ulong)) as
            *mut *mut libc::c_char;
    if dirs.is_null() {
        (*info).info_dirs = 0 as *mut libc::c_void as *mut *mut libc::c_char;
        *code_ptr = *__errno_location();
        return
    }
    (*info).info_dirs = dirs;
    let ref mut fresh0 = *dirs.offset((n_dirs + 1 as libc::c_int) as isize);
    *fresh0 = 0 as *mut libc::c_void as *mut libc::c_char;
    let ref mut fresh1 = *dirs.offset(n_dirs as isize);
    *fresh1 = strdup(info_dir);
    *code_ptr = 0 as libc::c_int;
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
#[c2rust::src_loc = "136:5"]
pub unsafe extern "C" fn ss_delete_info_dir(mut sci_idx: libc::c_int,
                                            mut info_dir: *mut libc::c_char,
                                            mut code_ptr: *mut libc::c_int) {
    let mut i_d: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut info_dirs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    info_dirs = (**_ss_table.offset(sci_idx as isize)).info_dirs;
    i_d = info_dirs;
    while !(*i_d).is_null() {
        if strcmp(*i_d, info_dir) == 0 {
            while !(*i_d).is_null() {
                *i_d = *i_d.offset(1 as libc::c_int as isize);
                i_d = i_d.offset(1)
            }
            *code_ptr = 0 as libc::c_int;
            return
        }
        i_d = i_d.offset(1)
    }
    *code_ptr = 748803 as libc::c_long as libc::c_int;
}
