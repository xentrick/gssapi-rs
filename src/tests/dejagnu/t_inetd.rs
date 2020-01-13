use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:39"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:39"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
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
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/types/__sigset_t.h:39"]
pub mod __sigset_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "5:9"]
    pub struct __sigset_t {
        pub __val: [libc::c_ulong; 16],
    }
}
#[c2rust::header_src = "/usr/include/bits/types/sigset_t.h:39"]
pub mod sigset_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type sigset_t = __sigset_t;
    use super::__sigset_t_h::__sigset_t;
}
#[c2rust::header_src = "/usr/include/bits/socket.h:43"]
pub mod socket_h {
    #[c2rust::src_loc = "33:1"]
    pub type socklen_t = __socklen_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "178:8"]
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [libc::c_char; 14],
    }
    use super::types_h::__socklen_t;
    use super::sockaddr_h::sa_family_t;
}
#[c2rust::header_src = "/usr/include/bits/socket_type.h:43"]
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
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:43"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/sys/socket.h:43"]
pub mod sys_socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "79:9"]
    pub union __SOCKADDR_ARG {
        pub __sockaddr__: *mut sockaddr,
        pub __sockaddr_at__: *mut sockaddr_at,
        pub __sockaddr_ax25__: *mut sockaddr_ax25,
        pub __sockaddr_dl__: *mut sockaddr_dl,
        pub __sockaddr_eon__: *mut sockaddr_eon,
        pub __sockaddr_in__: *mut sockaddr_in,
        pub __sockaddr_in6__: *mut sockaddr_in6,
        pub __sockaddr_inarp__: *mut sockaddr_inarp,
        pub __sockaddr_ipx__: *mut sockaddr_ipx,
        pub __sockaddr_iso__: *mut sockaddr_iso,
        pub __sockaddr_ns__: *mut sockaddr_ns,
        pub __sockaddr_un__: *mut sockaddr_un,
        pub __sockaddr_x25__: *mut sockaddr_x25,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "83:9"]
    pub union __CONST_SOCKADDR_ARG {
        pub __sockaddr__: *const sockaddr,
        pub __sockaddr_at__: *const sockaddr_at,
        pub __sockaddr_ax25__: *const sockaddr_ax25,
        pub __sockaddr_dl__: *const sockaddr_dl,
        pub __sockaddr_eon__: *const sockaddr_eon,
        pub __sockaddr_in__: *const sockaddr_in,
        pub __sockaddr_in6__: *const sockaddr_in6,
        pub __sockaddr_inarp__: *const sockaddr_inarp,
        pub __sockaddr_ipx__: *const sockaddr_ipx,
        pub __sockaddr_iso__: *const sockaddr_iso,
        pub __sockaddr_ns__: *const sockaddr_ns,
        pub __sockaddr_un__: *const sockaddr_un,
        pub __sockaddr_x25__: *const sockaddr_x25,
    }
    use super::socket_h::{sockaddr, socklen_t};
    use super::in_h::{sockaddr_in, sockaddr_in6};
    extern "C" {
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_x25;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_un;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_ns;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_iso;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_ipx;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_inarp;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_eon;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_dl;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_ax25;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_at;
        #[no_mangle]
        #[c2rust::src_loc = "102:1"]
        pub fn socket(__domain: libc::c_int, __type: libc::c_int,
                      __protocol: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "112:1"]
        pub fn bind(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG,
                    __len: socklen_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "215:1"]
        pub fn setsockopt(__fd: libc::c_int, __level: libc::c_int,
                          __optname: libc::c_int,
                          __optval: *const libc::c_void, __optlen: socklen_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "222:1"]
        pub fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "232:1"]
        pub fn accept(__fd: libc::c_int, __addr: __SOCKADDR_ARG,
                      __addr_len: *mut socklen_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/netinet/in.h:44"]
pub mod in_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "253:8"]
    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: in_port_t,
        pub sin6_flowinfo: uint32_t,
        pub sin6_addr: in6_addr,
        pub sin6_scope_id: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "212:8"]
    pub struct in6_addr {
        pub __in6_u: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "214:5"]
    pub union C2RustUnnamed {
        pub __u6_addr8: [uint8_t; 16],
        pub __u6_addr16: [uint16_t; 8],
        pub __u6_addr32: [uint32_t; 4],
    }
    #[c2rust::src_loc = "119:1"]
    pub type in_port_t = uint16_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "238:8"]
    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [libc::c_uchar; 8],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:8"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    #[c2rust::src_loc = "30:1"]
    pub type in_addr_t = uint32_t;
    use super::sockaddr_h::sa_family_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t, uint16_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "380:1"]
        pub fn htons(__hostshort: uint16_t) -> uint16_t;
    }
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:44"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::{__uint32_t, __uint16_t, __uint8_t};
}
#[c2rust::header_src = "/usr/include/bits/types/__sigval_t.h:46"]
pub mod __sigval_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:7"]
    pub union sigval {
        pub sival_int: libc::c_int,
        pub sival_ptr: *mut libc::c_void,
    }
    #[c2rust::src_loc = "30:1"]
    pub type __sigval_t = sigval;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:47"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:47"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/types/siginfo_t.h:51"]
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
#[c2rust::header_src = "/usr/include/signal.h:51"]
pub mod signal_h {
    #[c2rust::src_loc = "72:1"]
    pub type __sighandler_t
        =
        Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
    use super::sigset_t_h::sigset_t;
    use super::sigaction_h::sigaction;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "196:1"]
        pub fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "240:1"]
        pub fn sigaction(__sig: libc::c_int, __act: *const sigaction,
                         __oact: *mut sigaction) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/sigaction.h:51"]
pub mod sigaction_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:8"]
    pub struct sigaction {
        pub __sigaction_handler: C2RustUnnamed_10,
        pub sa_mask: __sigset_t,
        pub sa_flags: libc::c_int,
        pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:5"]
    pub union C2RustUnnamed_10 {
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:57"]
pub mod com_err_h {
    /*
 * Copyright 1988, Student Information Processing Board of the
 * Massachusetts Institute of Technology.
 *
 * Copyright 1995 by Cygnus Support.
 *
 * For copyright and distribution info, see the documentation supplied
 * with this package.
 */
    /* Header file for common error description library. */
    #[c2rust::src_loc = "26:1"]
    pub type errcode_t = libc::c_long;
    extern "C" {
        /* Public interfaces */
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn com_err(_: *const libc::c_char, _: errcode_t,
                       _: *const libc::c_char, _: ...);
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src = "/usr/include/stdlib.h:39"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "104:1"]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:47"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "138:14"]
        pub static mut stdout: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "218:1"]
        pub fn fflush(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:48"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:50"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:54"]
pub mod unistd_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "534:1"]
        pub fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "563:1"]
        pub fn execv(__path: *const libc::c_char,
                     __argv: *const *mut libc::c_char) -> libc::c_int;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint8_t, __uint16_t, __uint32_t, __uid_t, __off_t,
                        __off64_t, __pid_t, __clock_t, __socklen_t};
pub use self::__sigset_t_h::__sigset_t;
pub use self::sigset_t_h::sigset_t;
pub use self::socket_h::{socklen_t, sockaddr};
pub use self::socket_type_h::{__socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC,
                              SOCK_PACKET, SOCK_DCCP, SOCK_SEQPACKET,
                              SOCK_RDM, SOCK_RAW, SOCK_DGRAM, SOCK_STREAM};
pub use self::sockaddr_h::sa_family_t;
pub use self::sys_socket_h::{__SOCKADDR_ARG, __CONST_SOCKADDR_ARG,
                             sockaddr_x25, sockaddr_un, sockaddr_ns,
                             sockaddr_iso, sockaddr_ipx, sockaddr_inarp,
                             sockaddr_eon, sockaddr_dl, sockaddr_ax25,
                             sockaddr_at, socket, bind, setsockopt, listen,
                             accept};
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed, in_port_t,
                     sockaddr_in, in_addr, in_addr_t, htons};
pub use self::stdint_uintn_h::{uint32_t, uint16_t, uint8_t};
pub use self::__sigval_t_h::{sigval, __sigval_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::siginfo_t_h::{siginfo_t, C2RustUnnamed_0, C2RustUnnamed_1,
                            C2RustUnnamed_2, C2RustUnnamed_3, C2RustUnnamed_4,
                            C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7,
                            C2RustUnnamed_8, C2RustUnnamed_9};
pub use self::signal_h::{__sighandler_t, sigemptyset, sigaction};
pub use self::sigaction_h::{sigaction, C2RustUnnamed_10};
pub use self::com_err_h::{errcode_t, com_err};
use self::stdlib_h::{atoi, exit};
use self::stdio_h::{stdout, stderr, fflush, fprintf, printf};
use self::string_h::memset;
use self::errno_h::__errno_location;
use self::unistd_h::{close, dup2, execv};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/dejagnu/t_inetd.c */
/*
 * Copyright 1991 by the Massachusetts Institute of Technology.
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
 * A simple program to simulate starting a process from inetd.
 *
 * Unlike a proper inetd situation, environment variables are passed
 * to the client.
 *
 * usage: t_inetd port program argv0 ...
 */
#[no_mangle]
#[c2rust::src_loc = "60:7"]
pub static mut progname: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "62:1"]
unsafe extern "C" fn usage() {
    fprintf(stderr,
            b"%s: port program argv0 argv1 ...\n\x00" as *const u8 as
                *const libc::c_char, progname); /* local, foreign address */
    exit(1 as libc::c_int);
}
#[c2rust::src_loc = "68:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut port: libc::c_ushort = 0;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sock: libc::c_int = 0;
    let mut acc: libc::c_int = 0;
    let mut one: libc::c_int = 1 as libc::c_int;
    let mut l_inaddr: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut f_inaddr: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut namelen: socklen_t =
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    let mut csig: sigaction =
        sigaction{__sigaction_handler: C2RustUnnamed_10{sa_handler: None,},
                  sa_mask: __sigset_t{__val: [0; 16],},
                  sa_flags: 0,
                  sa_restorer: None,};
    progname = *argv.offset(0 as libc::c_int as isize);
    if argc <= 3 as libc::c_int { usage(); }
    if atoi(*argv.offset(1 as libc::c_int as isize)) == 0 as libc::c_int {
        usage();
    }
    port = htons(atoi(*argv.offset(1 as libc::c_int as isize)) as uint16_t);
    path = *argv.offset(2 as libc::c_int as isize);
    sock =
        socket(2 as libc::c_int, SOCK_STREAM as libc::c_int,
               0 as libc::c_int);
    if sock < 0 as libc::c_int {
        com_err(progname, *__errno_location() as errcode_t,
                b"creating socket\x00" as *const u8 as *const libc::c_char);
        exit(3 as libc::c_int);
    }
    setsockopt(sock, 1 as libc::c_int, 2 as libc::c_int,
               &mut one as *mut libc::c_int as *mut libc::c_char as
                   *const libc::c_void,
               ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                   socklen_t);
    memset(&mut l_inaddr as *mut sockaddr_in as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong);
    l_inaddr.sin_family = 2 as libc::c_int as sa_family_t;
    l_inaddr.sin_addr.s_addr = 0 as libc::c_int as in_addr_t;
    l_inaddr.sin_port = port;
    if bind(sock,
            __CONST_SOCKADDR_ARG{__sockaddr__:
                                     &mut l_inaddr as *mut sockaddr_in as
                                         *mut sockaddr,},
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                socklen_t) != 0 {
        com_err(progname, *__errno_location() as errcode_t,
                b"binding socket\x00" as *const u8 as *const libc::c_char);
        exit(3 as libc::c_int);
    }
    if listen(sock, 1 as libc::c_int) == -(1 as libc::c_int) {
        com_err(progname, *__errno_location() as errcode_t,
                b"listening\x00" as *const u8 as *const libc::c_char);
        exit(3 as libc::c_int);
    }
    printf(b"Ready!\n\x00" as *const u8 as *const libc::c_char);
    fflush(stdout);
    acc =
        accept(sock,
               __SOCKADDR_ARG{__sockaddr__:
                                  &mut f_inaddr as *mut sockaddr_in as
                                      *mut sockaddr,}, &mut namelen);
    if acc == -(1 as libc::c_int) {
        com_err(progname, *__errno_location() as errcode_t,
                b"accepting\x00" as *const u8 as *const libc::c_char);
        exit(3 as libc::c_int);
    }
    dup2(acc, 0 as libc::c_int);
    dup2(acc, 1 as libc::c_int);
    dup2(acc, 2 as libc::c_int);
    close(sock);
    sock = 0 as libc::c_int;
    /* Don't wait for a child signal... Otherwise dejagnu gets confused */
    csig.__sigaction_handler.sa_handler =
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> ()>,
                                __sighandler_t>(None);
    sigemptyset(&mut csig.sa_mask);
    csig.sa_flags = 0 as libc::c_int;
    sigaction(17 as libc::c_int, &mut csig, 0 as *mut sigaction);
    if execv(path,
             &mut *argv.offset(3 as libc::c_int as isize) as
                 *mut *mut libc::c_char as *const *mut libc::c_char) != 0 {
        fprintf(stderr,
                b"t_inetd: Could not exec %s\n\x00" as *const u8 as
                    *const libc::c_char, path);
    }
    exit(1 as libc::c_int);
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
