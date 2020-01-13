use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:45"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:45"]
pub mod types_h {
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:45"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:45"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/unistd.h:45"]
pub mod unistd_h {
    #[c2rust::src_loc = "274:1"]
    pub type socklen_t = __socklen_t;
    use super::types_h::__socklen_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "877:1"]
        pub fn gethostname(__name: *mut libc::c_char, __len: size_t)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:47"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/bits/socket.h:47"]
pub mod socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "178:8"]
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [libc::c_char; 14],
    }
    use super::sockaddr_h::sa_family_t;
}
#[c2rust::header_src = "/usr/include/netdb.h:49"]
pub mod netdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "565:8"]
    pub struct addrinfo {
        pub ai_flags: libc::c_int,
        pub ai_family: libc::c_int,
        pub ai_socktype: libc::c_int,
        pub ai_protocol: libc::c_int,
        pub ai_addrlen: socklen_t,
        pub ai_addr: *mut sockaddr,
        pub ai_canonname: *mut libc::c_char,
        pub ai_next: *mut addrinfo,
    }
    use super::unistd_h::socklen_t;
    use super::socket_h::sockaddr;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "660:1"]
        pub fn getaddrinfo(__name: *const libc::c_char,
                           __service: *const libc::c_char,
                           __req: *const addrinfo, __pai: *mut *mut addrinfo)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "666:1"]
        pub fn freeaddrinfo(__ai: *mut addrinfo);
        #[no_mangle]
        #[c2rust::src_loc = "675:1"]
        pub fn getnameinfo(__sa: *const sockaddr, __salen: socklen_t,
                           __host: *mut libc::c_char, __hostlen: socklen_t,
                           __serv: *mut libc::c_char, __servlen: socklen_t,
                           __flags: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:45"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:45"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:45"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "775:1"]
        pub fn perror(__s: *const libc::c_char);
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:45"]
pub mod k5_platform_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "887:1"]
        pub fn krb5int_strlcpy(dst: *mut libc::c_char,
                               src: *const libc::c_char, siz: size_t)
         -> size_t;
    }
    /* K5_PLATFORM_H */
}
#[c2rust::header_src = "/usr/include/arpa/inet.h:48"]
pub mod inet_h {
    use super::unistd_h::socklen_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn inet_ntop(__af: libc::c_int, __cp: *const libc::c_void,
                         __buf: *mut libc::c_char, __len: socklen_t)
         -> *const libc::c_char;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__off_t, __off64_t, __socklen_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::unistd_h::{socklen_t, gethostname};
pub use self::sockaddr_h::sa_family_t;
pub use self::socket_h::sockaddr;
pub use self::netdb_h::{addrinfo, getaddrinfo, freeaddrinfo, getnameinfo};
use self::string_h::{memset, strcmp};
use self::stdlib_h::exit;
use self::stdio_h::{stderr, fprintf, printf, perror};
use self::k5_platform_h::krb5int_strlcpy;
use self::inet_h::inet_ntop;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/resolve/resolve.c */
/*
 * Copyright 1995 by the Massachusetts Institute of Technology.
 * All Rights Reserved.
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
/*
 * A simple program to test the functionality of the resolver library.
 * It simply will try to get the IP address of the host, and then look
 * up the name from the address. If the resulting name does not contain the
 * domain name, then the resolve library is broken.
 *
 * Warning: It is possible to fool this program into thinking everything is
 * alright by a clever use of /etc/hosts - but this is better than nothing.
 *
 * Usage:
 *   resolve [hostname]
 *
 *   When invoked with no arguments, gethostname is used for the local host.
 *
 */
/* This program tests the resolve library and sees if it is broken... */
#[c2rust::src_loc = "54:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut ai: *mut addrinfo = 0 as *mut addrinfo; /* for safety */
    let mut hint: addrinfo =
        addrinfo{ai_flags: 0,
                 ai_family: 0,
                 ai_socktype: 0,
                 ai_protocol: 0,
                 ai_addrlen: 0,
                 ai_addr: 0 as *mut sockaddr,
                 ai_canonname: 0 as *mut libc::c_char,
                 ai_next: 0 as *mut addrinfo,};
    let mut myname: [libc::c_char; 65] = [0; 65];
    let mut namebuf: [libc::c_char; 1025] = [0; 1025];
    let mut abuf: [libc::c_char; 256] = [0; 256];
    let mut addrstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut err: libc::c_int = 0;
    let mut quiet: libc::c_int = 0 as libc::c_int;
    argc -= 1;
    argv = argv.offset(1);
    while argc != 0 {
        if !(strcmp(*argv, b"--quiet\x00" as *const u8 as *const libc::c_char)
                 == 0 as libc::c_int ||
                 strcmp(*argv, b"-q\x00" as *const u8 as *const libc::c_char)
                     == 0 as libc::c_int) {
            break ;
        }
        quiet += 1;
        argc -= 1;
        argv = argv.offset(1)
    }
    if argc >= 1 as libc::c_int {
        krb5int_strlcpy(myname.as_mut_ptr(), *argv,
                        ::std::mem::size_of::<[libc::c_char; 65]>() as
                            libc::c_ulong);
    } else if gethostname(myname.as_mut_ptr(), 64 as libc::c_int as size_t) !=
                  0 {
        perror(b"gethostname failure\x00" as *const u8 as
                   *const libc::c_char);
        exit(1 as libc::c_int);
    }
    myname[64 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    /* Look up the address... */
    if quiet == 0 {
        printf(b"Hostname:  %s\n\x00" as *const u8 as *const libc::c_char,
               myname.as_mut_ptr());
    }
    memset(&mut hint as *mut addrinfo as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<addrinfo>() as libc::c_ulong);
    hint.ai_flags = 0x2 as libc::c_int;
    err =
        getaddrinfo(myname.as_mut_ptr(), 0 as *const libc::c_char, &mut hint,
                    &mut ai);
    if err != 0 {
        fprintf(stderr,
                b"Could not look up address for hostname \'%s\' - fatal\n\x00"
                    as *const u8 as *const libc::c_char, myname.as_mut_ptr());
        exit(2 as libc::c_int);
    }
    if quiet == 0 {
        addrstr =
            inet_ntop((*ai).ai_family, (*ai).ai_addr as *const libc::c_void,
                      abuf.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 256]>() as
                          libc::c_ulong as socklen_t);
        if !addrstr.is_null() {
            printf(b"Host address: %s\n\x00" as *const u8 as
                       *const libc::c_char, addrstr);
        }
    }
    err =
        getnameinfo((*ai).ai_addr, (*ai).ai_addrlen, namebuf.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 1025]>() as
                        libc::c_ulong as socklen_t, 0 as *mut libc::c_char,
                    0 as libc::c_int as socklen_t, 8 as libc::c_int);
    if err != 0 && quiet == 0 {
        fprintf(stderr,
                b"Error looking up IP address\n\x00" as *const u8 as
                    *const libc::c_char);
    }
    printf(b"%s%s\n\x00" as *const u8 as *const libc::c_char,
           if quiet != 0 {
               b"\x00" as *const u8 as *const libc::c_char
           } else { b"FQDN: \x00" as *const u8 as *const libc::c_char },
           if err != 0 { (*ai).ai_canonname } else { namebuf.as_mut_ptr() });
    if quiet == 0 {
        printf(b"Resolve library appears to have passed the test\n\x00" as
                   *const u8 as *const libc::c_char);
    }
    freeaddrinfo(ai);
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
