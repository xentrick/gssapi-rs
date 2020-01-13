use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:28"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:28"]
pub mod types_h {
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "147:1"]
    pub type __gid_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/sys/types.h:28"]
pub mod sys_types_h {
    #[c2rust::src_loc = "79:1"]
    pub type uid_t = __uid_t;
    use super::types_h::__uid_t;
}
#[c2rust::header_src = "/usr/include/pwd.h:31"]
pub mod pwd_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct passwd {
        pub pw_name: *mut libc::c_char,
        pub pw_passwd: *mut libc::c_char,
        pub pw_uid: __uid_t,
        pub pw_gid: __gid_t,
        pub pw_gecos: *mut libc::c_char,
        pub pw_dir: *mut libc::c_char,
        pub pw_shell: *mut libc::c_char,
    }
    use super::types_h::{__uid_t, __gid_t};
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "145:1"]
        pub fn getpwuid_r(__uid: __uid_t, __resultbuf: *mut passwd,
                          __buffer: *mut libc::c_char, __buflen: size_t,
                          __result: *mut *mut passwd) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:28"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:28"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:28"]
pub mod unistd_h {
    use super::types_h::__uid_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "675:1"]
        pub fn getuid() -> __uid_t;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__uid_t, __gid_t};
pub use self::sys_types_h::uid_t;
pub use self::pwd_h::{passwd, getpwuid_r};
use self::stdio_h::printf;
use self::stdlib_h::exit;
use self::unistd_h::getuid;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/misc/test_getpw.c */
/*
 * Copyright (C) 2005 by the Massachusetts Institute of Technology.
 * All rights reserved.
 *
 * Export of this software from the United States of America may
 *   require a specific license from the United States Government.
 *   It is the responsibility of any person or organization contemplating
 *   export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of M.I.T. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 * M.I.T. makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 */
#[c2rust::src_loc = "35:1"]
unsafe fn main_0() -> libc::c_int {
    let mut my_uid: uid_t = 0;
    let mut pwd: *mut passwd = 0 as *mut passwd;
    let mut pwx: passwd =
        passwd{pw_name: 0 as *mut libc::c_char,
               pw_passwd: 0 as *mut libc::c_char,
               pw_uid: 0,
               pw_gid: 0,
               pw_gecos: 0 as *mut libc::c_char,
               pw_dir: 0 as *mut libc::c_char,
               pw_shell: 0 as *mut libc::c_char,};
    let mut pwbuf: [libc::c_char; 8192] = [0; 8192];
    let mut x: libc::c_int = 0;
    my_uid = getuid();
    printf(b"my uid: %ld\n\x00" as *const u8 as *const libc::c_char,
           my_uid as libc::c_long);
    x =
        if getpwuid_r(my_uid, &mut pwx, pwbuf.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 8192]>() as
                          libc::c_ulong, &mut pwd) == 0 as libc::c_int {
            if pwd.is_null() { -(1 as libc::c_int) } else { 0 as libc::c_int }
        } else { -(1 as libc::c_int) };
    printf(b"k5_getpwuid_r returns %d\n\x00" as *const u8 as
               *const libc::c_char, x);
    if x != 0 as libc::c_int { exit(1 as libc::c_int); }
    printf(b"    username is \'%s\'\n\x00" as *const u8 as
               *const libc::c_char, (*pwd).pw_name);
    exit(0 as libc::c_int);
}
#[main]
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }
