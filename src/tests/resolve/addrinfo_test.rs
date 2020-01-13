use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:39"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:39"]
pub mod types_h {
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:39"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:39"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/unistd.h:39"]
pub mod unistd_h {
    #[c2rust::src_loc = "274:1"]
    pub type socklen_t = __socklen_t;
    use super::types_h::__socklen_t;
}
#[c2rust::header_src = "/usr/include/bits/socket_type.h:41"]
pub mod socket_type_h {
    #[c2rust::src_loc = "24:1"]
    pub type __socket_type = libc::c_uint;
    #[c2rust::src_loc = "52:3"]
    pub const SOCK_NONBLOCK: __socket_type = 2048;
    #[c2rust::src_loc = "49:3"]
    pub const SOCK_CLOEXEC: __socket_type = 524288;
    #[c2rust::src_loc = "41:3"]
    pub const SOCK_PACKET: __socket_type = 10;
    #[c2rust::src_loc = "39:3"]
    pub const SOCK_DCCP: __socket_type = 6;
    #[c2rust::src_loc = "36:3"]
    pub const SOCK_SEQPACKET: __socket_type = 5;
    #[c2rust::src_loc = "34:3"]
    pub const SOCK_RDM: __socket_type = 4;
    #[c2rust::src_loc = "32:3"]
    pub const SOCK_RAW: __socket_type = 3;
    #[c2rust::src_loc = "29:3"]
    pub const SOCK_DGRAM: __socket_type = 2;
    #[c2rust::src_loc = "26:3"]
    pub const SOCK_STREAM: __socket_type = 1;
}
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:41"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/bits/socket.h:41"]
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
#[c2rust::header_src = "/usr/include/netinet/in.h:42"]
pub mod in_h {
    #[c2rust::src_loc = "40:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "92:5"]
    pub const IPPROTO_MAX: C2RustUnnamed = 256;
    #[c2rust::src_loc = "90:5"]
    pub const IPPROTO_RAW: C2RustUnnamed = 255;
    #[c2rust::src_loc = "88:5"]
    pub const IPPROTO_MPLS: C2RustUnnamed = 137;
    #[c2rust::src_loc = "86:5"]
    pub const IPPROTO_UDPLITE: C2RustUnnamed = 136;
    #[c2rust::src_loc = "84:5"]
    pub const IPPROTO_SCTP: C2RustUnnamed = 132;
    #[c2rust::src_loc = "82:5"]
    pub const IPPROTO_COMP: C2RustUnnamed = 108;
    #[c2rust::src_loc = "80:5"]
    pub const IPPROTO_PIM: C2RustUnnamed = 103;
    #[c2rust::src_loc = "78:5"]
    pub const IPPROTO_ENCAP: C2RustUnnamed = 98;
    #[c2rust::src_loc = "76:5"]
    pub const IPPROTO_BEETPH: C2RustUnnamed = 94;
    #[c2rust::src_loc = "74:5"]
    pub const IPPROTO_MTP: C2RustUnnamed = 92;
    #[c2rust::src_loc = "72:5"]
    pub const IPPROTO_AH: C2RustUnnamed = 51;
    #[c2rust::src_loc = "70:5"]
    pub const IPPROTO_ESP: C2RustUnnamed = 50;
    #[c2rust::src_loc = "68:5"]
    pub const IPPROTO_GRE: C2RustUnnamed = 47;
    #[c2rust::src_loc = "66:5"]
    pub const IPPROTO_RSVP: C2RustUnnamed = 46;
    #[c2rust::src_loc = "64:5"]
    pub const IPPROTO_IPV6: C2RustUnnamed = 41;
    #[c2rust::src_loc = "62:5"]
    pub const IPPROTO_DCCP: C2RustUnnamed = 33;
    #[c2rust::src_loc = "60:5"]
    pub const IPPROTO_TP: C2RustUnnamed = 29;
    #[c2rust::src_loc = "58:5"]
    pub const IPPROTO_IDP: C2RustUnnamed = 22;
    #[c2rust::src_loc = "56:5"]
    pub const IPPROTO_UDP: C2RustUnnamed = 17;
    #[c2rust::src_loc = "54:5"]
    pub const IPPROTO_PUP: C2RustUnnamed = 12;
    #[c2rust::src_loc = "52:5"]
    pub const IPPROTO_EGP: C2RustUnnamed = 8;
    #[c2rust::src_loc = "50:5"]
    pub const IPPROTO_TCP: C2RustUnnamed = 6;
    #[c2rust::src_loc = "48:5"]
    pub const IPPROTO_IPIP: C2RustUnnamed = 4;
    #[c2rust::src_loc = "46:5"]
    pub const IPPROTO_IGMP: C2RustUnnamed = 2;
    #[c2rust::src_loc = "44:5"]
    pub const IPPROTO_ICMP: C2RustUnnamed = 1;
    #[c2rust::src_loc = "42:5"]
    pub const IPPROTO_IP: C2RustUnnamed = 0;
    #[c2rust::src_loc = "99:1"]
    pub type C2RustUnnamed_0 = libc::c_uint;
    #[c2rust::src_loc = "113:5"]
    pub const IPPROTO_MH: C2RustUnnamed_0 = 135;
    #[c2rust::src_loc = "111:5"]
    pub const IPPROTO_DSTOPTS: C2RustUnnamed_0 = 60;
    #[c2rust::src_loc = "109:5"]
    pub const IPPROTO_NONE: C2RustUnnamed_0 = 59;
    #[c2rust::src_loc = "107:5"]
    pub const IPPROTO_ICMPV6: C2RustUnnamed_0 = 58;
    #[c2rust::src_loc = "105:5"]
    pub const IPPROTO_FRAGMENT: C2RustUnnamed_0 = 44;
    #[c2rust::src_loc = "103:5"]
    pub const IPPROTO_ROUTING: C2RustUnnamed_0 = 43;
    #[c2rust::src_loc = "101:5"]
    pub const IPPROTO_HOPOPTS: C2RustUnnamed_0 = 0;
}
#[c2rust::header_src = "/usr/include/netdb.h:42"]
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
        #[c2rust::src_loc = "669:1"]
        pub fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "675:1"]
        pub fn getnameinfo(__sa: *const sockaddr, __salen: socklen_t,
                           __host: *mut libc::c_char, __hostlen: socklen_t,
                           __serv: *mut libc::c_char, __servlen: socklen_t,
                           __flags: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:39"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "252:14"]
        pub fn strrchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "396:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:39"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:39"]
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
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:39"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:39"]
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
pub use self::stddef_h::size_t;
pub use self::types_h::{__off_t, __off64_t, __socklen_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::unistd_h::socklen_t;
pub use self::socket_type_h::{__socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC,
                              SOCK_PACKET, SOCK_DCCP, SOCK_SEQPACKET,
                              SOCK_RDM, SOCK_RAW, SOCK_DGRAM, SOCK_STREAM};
pub use self::sockaddr_h::sa_family_t;
pub use self::socket_h::sockaddr;
pub use self::in_h::{C2RustUnnamed, IPPROTO_MAX, IPPROTO_RAW, IPPROTO_MPLS,
                     IPPROTO_UDPLITE, IPPROTO_SCTP, IPPROTO_COMP, IPPROTO_PIM,
                     IPPROTO_ENCAP, IPPROTO_BEETPH, IPPROTO_MTP, IPPROTO_AH,
                     IPPROTO_ESP, IPPROTO_GRE, IPPROTO_RSVP, IPPROTO_IPV6,
                     IPPROTO_DCCP, IPPROTO_TP, IPPROTO_IDP, IPPROTO_UDP,
                     IPPROTO_PUP, IPPROTO_EGP, IPPROTO_TCP, IPPROTO_IPIP,
                     IPPROTO_IGMP, IPPROTO_ICMP, IPPROTO_IP, C2RustUnnamed_0,
                     IPPROTO_MH, IPPROTO_DSTOPTS, IPPROTO_NONE,
                     IPPROTO_ICMPV6, IPPROTO_FRAGMENT, IPPROTO_ROUTING,
                     IPPROTO_HOPOPTS};
pub use self::netdb_h::{addrinfo, getaddrinfo, freeaddrinfo, gai_strerror,
                        getnameinfo};
use self::string_h::{strrchr, strerror, memset};
use self::stdlib_h::exit;
use self::stdio_h::{stderr, fprintf, printf, snprintf};
use self::errno_h::__errno_location;
use self::k5_platform_h::krb5int_strlcpy;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/resolve/addrinfo-test.c */
/*
 * Copyright 2004 by the Massachusetts Institute of Technology.
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
 * A simple program to test the functionality of the getaddrinfo function.
 *
 * Usage:
 *   addrinfo-test [-t|-u|-R|-I] [-d|-s|-r] [-p port] [-P] [hostname]
 *
 *   When invoked with no arguments, NULL is used for the node name,
 *   which (at least with a non-null "port") means a socket address
 *   is desired that can be used with connect() or bind() (depending
 *   on whether "-P" is given).
 */
/* needed for IPPROTO_* on NetBSD */
#[c2rust::src_loc = "48:1"]
unsafe extern "C" fn protoname(mut p: libc::c_int) -> *const libc::c_char {
    static mut buf: [libc::c_char; 30] = [0; 30];
    if p == IPPROTO_TCP as libc::c_int {
        return b"TCP\x00" as *const u8 as *const libc::c_char
    }
    if p == IPPROTO_UDP as libc::c_int {
        return b"UDP\x00" as *const u8 as *const libc::c_char
    }
    if p == IPPROTO_ICMP as libc::c_int {
        return b"ICMP\x00" as *const u8 as *const libc::c_char
    }
    if p == IPPROTO_IPV6 as libc::c_int {
        return b"IPV6\x00" as *const u8 as *const libc::c_char
    }
    if p == IPPROTO_GRE as libc::c_int {
        return b"GRE\x00" as *const u8 as *const libc::c_char
    }
    if p == IPPROTO_NONE as libc::c_int {
        return b"NONE\x00" as *const u8 as *const libc::c_char
    }
    if p == IPPROTO_RAW as libc::c_int {
        return b"RAW\x00" as *const u8 as *const libc::c_char
    }
    if p == IPPROTO_COMP as libc::c_int {
        return b"COMP\x00" as *const u8 as *const libc::c_char
    }
    snprintf(buf.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
             b" %-2d\x00" as *const u8 as *const libc::c_char, p);
    return buf.as_mut_ptr();
}
#[c2rust::src_loc = "74:1"]
unsafe extern "C" fn socktypename(mut t: libc::c_int) -> *const libc::c_char {
    static mut buf: [libc::c_char; 30] = [0; 30];
    match t {
        2 => { return b"DGRAM\x00" as *const u8 as *const libc::c_char }
        1 => { return b"STREAM\x00" as *const u8 as *const libc::c_char }
        3 => { return b"RAW\x00" as *const u8 as *const libc::c_char }
        4 => { return b"RDM\x00" as *const u8 as *const libc::c_char }
        5 => { return b"SEQPACKET\x00" as *const u8 as *const libc::c_char }
        _ => { }
    }
    snprintf(buf.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
             b" %-2d\x00" as *const u8 as *const libc::c_char, t);
    return buf.as_mut_ptr();
}
#[c2rust::src_loc = "87:14"]
static mut whoami: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "89:1"]
unsafe extern "C" fn usage() {
    fprintf(stderr,
            b"usage:\n\t%s [ options ] [host]\noptions:\n\t-t\tspecify protocol IPPROTO_TCP\n\t-u\tspecify protocol IPPROTO_UDP\n\t-R\tspecify protocol IPPROTO_RAW\n\t-I\tspecify protocol IPPROTO_ICMP\n\n\t-d\tspecify socket type SOCK_DGRAM\n\t-s\tspecify socket type SOCK_STREAM\n\t-r\tspecify socket type SOCK_RAW\n\n\t-4\tspecify address family AF_INET\n\t-6\tspecify address family AF_INET6\n\n\t-p P\tspecify port P (service name or port number)\n\t-N\thostname is numeric, skip DNS query\n\t-n\tservice/port is numeric (sets AI_NUMERICSERV)\n\t-P\tset AI_PASSIVE\n\ndefault: protocol 0, socket type 0, address family 0, null port\n\x00"
                as *const u8 as *const libc::c_char, whoami);
    /* [ -t | -u | -R | -I ] [ -d | -s | -r ] [ -p port ] */
    exit(1 as libc::c_int);
}
#[c2rust::src_loc = "120:1"]
unsafe extern "C" fn familyname(mut f: libc::c_int) -> *const libc::c_char {
    static mut buf: [libc::c_char; 30] = [0; 30];
    match f {
        2 => { return b"AF_INET\x00" as *const u8 as *const libc::c_char }
        10 => { return b"AF_INET6\x00" as *const u8 as *const libc::c_char }
        _ => {
            snprintf(buf.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 30]>() as
                         libc::c_ulong,
                     b"AF %d\x00" as *const u8 as *const libc::c_char, f);
            return buf.as_mut_ptr()
        }
    };
}
#[c2rust::src_loc = "135:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut ap: *mut addrinfo = 0 as *mut addrinfo;
    let mut ap2: *mut addrinfo = 0 as *mut addrinfo;
    let mut err: libc::c_int = 0;
    let mut numerichost: libc::c_int = 0 as libc::c_int;
    let mut numericserv: libc::c_int = 0 as libc::c_int;
    let mut hname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut port: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hints: addrinfo =
        addrinfo{ai_flags: 0,
                 ai_family: 0,
                 ai_socktype: 0,
                 ai_protocol: 0,
                 ai_addrlen: 0,
                 ai_addr: 0 as *mut sockaddr,
                 ai_canonname: 0 as *mut libc::c_char,
                 ai_next: 0 as *mut addrinfo,};
    whoami = strrchr(*argv.offset(0 as libc::c_int as isize), '/' as i32);
    if whoami.is_null() {
        whoami = *argv.offset(0 as libc::c_int as isize)
    } else { whoami = whoami.offset(1 as libc::c_int as isize) }
    memset(&mut hints as *mut addrinfo as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<addrinfo>() as libc::c_ulong);
    hints.ai_flags = 0 as libc::c_int;
    hints.ai_socktype = 0 as libc::c_int;
    hname = 0 as *mut libc::c_char;
    hints.ai_family = 0 as libc::c_int;
    if argc == 1 as libc::c_int { usage(); }
    loop  {
        argv = argv.offset(1);
        argc -= 1;
        if !(argc > 0 as libc::c_int) { break ; }
        let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
        arg = *argv;
        if *arg as libc::c_int != '-' as i32 {
            hname = arg
        } else if *arg.offset(1 as libc::c_int as isize) as libc::c_int ==
                      0 as libc::c_int ||
                      *arg.offset(2 as libc::c_int as isize) as libc::c_int !=
                          0 as libc::c_int {
            usage();
        } else {
            match *arg.offset(1 as libc::c_int as isize) as libc::c_int {
                117 => { hints.ai_protocol = IPPROTO_UDP as libc::c_int }
                116 => { hints.ai_protocol = IPPROTO_TCP as libc::c_int }
                82 => { hints.ai_protocol = IPPROTO_RAW as libc::c_int }
                73 => { hints.ai_protocol = IPPROTO_ICMP as libc::c_int }
                100 => { hints.ai_socktype = SOCK_DGRAM as libc::c_int }
                115 => { hints.ai_socktype = SOCK_STREAM as libc::c_int }
                114 => { hints.ai_socktype = SOCK_RAW as libc::c_int }
                112 => {
                    if (*argv.offset(1 as libc::c_int as isize)).is_null() ||
                           *(*argv.offset(1 as libc::c_int as
                                              isize)).offset(0 as libc::c_int
                                                                 as isize) as
                               libc::c_int == 0 as libc::c_int ||
                           *(*argv.offset(1 as libc::c_int as
                                              isize)).offset(0 as libc::c_int
                                                                 as isize) as
                               libc::c_int == '-' as i32 {
                        usage();
                    }
                    port = *argv.offset(1 as libc::c_int as isize);
                    argc -= 1;
                    argv = argv.offset(1)
                }
                52 => { hints.ai_family = 2 as libc::c_int }
                54 => { hints.ai_family = 10 as libc::c_int }
                78 => { numerichost = 1 as libc::c_int }
                110 => { numericserv = 1 as libc::c_int }
                80 => { hints.ai_flags |= 0x1 as libc::c_int }
                _ => { usage(); }
            }
        }
    }
    if !hname.is_null() && numerichost == 0 {
        hints.ai_flags |= 0x2 as libc::c_int
    }
    if numerichost != 0 { hints.ai_flags |= 0x4 as libc::c_int }
    if numericserv != 0 { hints.ai_flags |= 0x400 as libc::c_int }
    printf(b"getaddrinfo(hostname %s, service %s,\n            hints { \x00"
               as *const u8 as *const libc::c_char,
           if !hname.is_null() {
               hname as *const libc::c_char
           } else { b"(null)\x00" as *const u8 as *const libc::c_char },
           if !port.is_null() {
               port as *const libc::c_char
           } else { b"(null)\x00" as *const u8 as *const libc::c_char });
    sep = b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if hints.ai_flags & 0x2 as libc::c_int != 0 {
        printf(b"%s%s\x00" as *const u8 as *const libc::c_char, sep,
               b"CANONNAME\x00" as *const u8 as *const libc::c_char);
        sep =
            b"|\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    if hints.ai_flags & 0x1 as libc::c_int != 0 {
        printf(b"%s%s\x00" as *const u8 as *const libc::c_char, sep,
               b"PASSIVE\x00" as *const u8 as *const libc::c_char);
        sep =
            b"|\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    if hints.ai_flags & 0x4 as libc::c_int != 0 {
        printf(b"%s%s\x00" as *const u8 as *const libc::c_char, sep,
               b"NUMERICHOST\x00" as *const u8 as *const libc::c_char);
        sep =
            b"|\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    if hints.ai_flags & 0x400 as libc::c_int != 0 {
        printf(b"%s%s\x00" as *const u8 as *const libc::c_char, sep,
               b"NUMERICSERV\x00" as *const u8 as *const libc::c_char);
        sep =
            b"|\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    if *sep.offset(0 as libc::c_int as isize) as libc::c_int ==
           0 as libc::c_int {
        printf(b"no-flags\x00" as *const u8 as *const libc::c_char);
    }
    if hints.ai_family != 0 {
        printf(b" %s\x00" as *const u8 as *const libc::c_char,
               familyname(hints.ai_family));
    }
    if hints.ai_socktype != 0 {
        printf(b" SOCK_%s\x00" as *const u8 as *const libc::c_char,
               socktypename(hints.ai_socktype));
    }
    if hints.ai_protocol != 0 {
        printf(b" IPPROTO_%s\x00" as *const u8 as *const libc::c_char,
               protoname(hints.ai_protocol));
    }
    printf(b" }):\n\x00" as *const u8 as *const libc::c_char);
    err = getaddrinfo(hname, port, &mut hints, &mut ap);
    if err != 0 {
        printf(b"\terror => %s\n\x00" as *const u8 as *const libc::c_char,
               if err == -(11 as libc::c_int) {
                   strerror(*__errno_location()) as *const libc::c_char
               } else { gai_strerror(err) });
        return 1 as libc::c_int
    }
    ap2 = ap;
    while !ap2.is_null() {
        let mut hbuf: [libc::c_char; 1025] = [0; 1025];
        let mut pbuf: [libc::c_char; 32] = [0; 32];
        /* If we don't do this, even AIX's own getnameinfo will reject
           the sockaddr structures.  The sa_len field doesn't get set
           either, on AIX, but getnameinfo won't complain.  */
        if (*(*ap2).ai_addr).sa_family as libc::c_int == 0 as libc::c_int {
            printf(b"BAD: sa_family zero! fixing...\n\x00" as *const u8 as
                       *const libc::c_char);
            (*(*ap2).ai_addr).sa_family = (*ap2).ai_family as sa_family_t
        } else if (*(*ap2).ai_addr).sa_family as libc::c_int !=
                      (*ap2).ai_family {
            printf(b"BAD: sa_family != ai_family! fixing...\n\x00" as
                       *const u8 as *const libc::c_char);
            (*(*ap2).ai_addr).sa_family = (*ap2).ai_family as sa_family_t
        }
        if getnameinfo((*ap2).ai_addr, (*ap2).ai_addrlen, hbuf.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 1025]>() as
                           libc::c_ulong as socklen_t, pbuf.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 32]>() as
                           libc::c_ulong as socklen_t,
                       1 as libc::c_int | 2 as libc::c_int) != 0 {
            krb5int_strlcpy(hbuf.as_mut_ptr(),
                            b"...\x00" as *const u8 as *const libc::c_char,
                            ::std::mem::size_of::<[libc::c_char; 1025]>() as
                                libc::c_ulong);
            krb5int_strlcpy(pbuf.as_mut_ptr(),
                            b"...\x00" as *const u8 as *const libc::c_char,
                            ::std::mem::size_of::<[libc::c_char; 32]>() as
                                libc::c_ulong);
        }
        printf(b"%p:\n\tfamily = %s\tproto = %-4s\tsocktype = %s\n\x00" as
                   *const u8 as *const libc::c_char, ap2 as *mut libc::c_void,
               familyname((*ap2).ai_family), protoname((*ap2).ai_protocol),
               socktypename((*ap2).ai_socktype));
        if !(*ap2).ai_canonname.is_null() {
            if *(*ap2).ai_canonname.offset(0 as libc::c_int as isize) != 0 {
                printf(b"\tcanonname = %s\n\x00" as *const u8 as
                           *const libc::c_char, (*ap2).ai_canonname);
            } else {
                printf(b"BAD: ai_canonname is set but empty!\n\x00" as
                           *const u8 as *const libc::c_char);
            }
        } else if ap2 == ap && hints.ai_flags & 0x2 as libc::c_int != 0 {
            printf(b"BAD: first ai_canonname is null!\n\x00" as *const u8 as
                       *const libc::c_char);
        }
        printf(b"\taddr = %-28s\tport = %s\n\x00" as *const u8 as
                   *const libc::c_char, hbuf.as_mut_ptr(), pbuf.as_mut_ptr());
        err =
            getnameinfo((*ap2).ai_addr, (*ap2).ai_addrlen, hbuf.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 1025]>() as
                            libc::c_ulong as socklen_t, pbuf.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 32]>() as
                            libc::c_ulong as socklen_t, 8 as libc::c_int);
        if err != 0 {
            printf(b"\tgetnameinfo(NI_NAMEREQD): %s\n\x00" as *const u8 as
                       *const libc::c_char,
                   if err == -(11 as libc::c_int) {
                       strerror(*__errno_location()) as *const libc::c_char
                   } else { gai_strerror(err) });
        } else {
            printf(b"\tgetnameinfo => %s, %s\n\x00" as *const u8 as
                       *const libc::c_char, hbuf.as_mut_ptr(),
                   pbuf.as_mut_ptr());
        }
        ap2 = (*ap2).ai_next
    }
    freeaddrinfo(ap);
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
