use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:48"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:48"]
pub mod types_h {
    #[c2rust::src_loc = "32:1"]
    pub type __u_short = libc::c_ushort;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "162:1"]
    pub type __suseconds_t = libc::c_long;
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:48"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:48"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/sys/types.h:53"]
pub mod sys_types_h {
    #[c2rust::src_loc = "34:1"]
    pub type u_short = __u_short;
    use super::types_h::__u_short;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timeval.h:53"]
pub mod struct_timeval_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8:8"]
    pub struct timeval {
        pub tv_sec: __time_t,
        pub tv_usec: __suseconds_t,
    }
    use super::types_h::{__time_t, __suseconds_t};
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:53"]
pub mod thread_shared_types_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:16"]
    pub struct __pthread_internal_list {
        pub __prev: *mut __pthread_internal_list,
        pub __next: *mut __pthread_internal_list,
    }
    #[c2rust::src_loc = "82:1"]
    pub type __pthread_list_t = __pthread_internal_list;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:8"]
    pub struct __pthread_mutex_s {
        pub __lock: libc::c_int,
        pub __count: libc::c_uint,
        pub __owner: libc::c_int,
        pub __nusers: libc::c_uint,
        pub __kind: libc::c_int,
        pub __spins: libc::c_short,
        pub __elision: libc::c_short,
        pub __list: __pthread_list_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "171:8"]
    pub struct __pthread_cond_s {
        pub c2rust_unnamed: C2RustUnnamed_1,
        pub c2rust_unnamed_0: C2RustUnnamed,
        pub __g_refs: [libc::c_uint; 2],
        pub __g_size: [libc::c_uint; 2],
        pub __g1_orig_size: libc::c_uint,
        pub __wrefs: libc::c_uint,
        pub __g_signals: [libc::c_uint; 2],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "182:17"]
    pub union C2RustUnnamed {
        pub __g1_start: libc::c_ulonglong,
        pub __g1_start32: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "185:5"]
    pub struct C2RustUnnamed_0 {
        pub __low: libc::c_uint,
        pub __high: libc::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "173:17"]
    pub union C2RustUnnamed_1 {
        pub __wseq: libc::c_ulonglong,
        pub __wseq32: C2RustUnnamed_2,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "176:5"]
    pub struct C2RustUnnamed_2 {
        pub __low: libc::c_uint,
        pub __high: libc::c_uint,
    }
}
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:53"]
pub mod pthreadtypes_h {
    #[c2rust::src_loc = "27:1"]
    pub type pthread_t = libc::c_ulong;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "56:7"]
    pub union pthread_attr_t {
        pub __size: [libc::c_char; 56],
        pub __align: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "67:9"]
    pub union pthread_mutex_t {
        pub __data: __pthread_mutex_s,
        pub __size: [libc::c_char; 40],
        pub __align: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "75:9"]
    pub union pthread_cond_t {
        pub __data: __pthread_cond_s,
        pub __size: [libc::c_char; 48],
        pub __align: libc::c_longlong,
    }
    use super::thread_shared_types_h::{__pthread_mutex_s, __pthread_cond_s};
}
#[c2rust::header_src = "/usr/include/bits/socket.h:54"]
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
#[c2rust::header_src = "/usr/include/bits/socket_type.h:54"]
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
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:54"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/sys/socket.h:54"]
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
#[c2rust::header_src = "/usr/include/netinet/in.h:56"]
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
        pub __in6_u: C2RustUnnamed_3,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "214:5"]
    pub union C2RustUnnamed_3 {
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:56"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::{__uint32_t, __uint16_t, __uint8_t};
}
#[c2rust::header_src = "/usr/include/sys/time.h:55"]
pub mod time_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:8"]
    pub struct timezone {
        pub tz_minuteswest: libc::c_int,
        pub tz_dsttime: libc::c_int,
    }
    #[c2rust::src_loc = "58:1"]
    pub type __timezone_ptr_t = *mut timezone;
    use super::struct_timeval_h::timeval;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "68:1"]
        pub fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/signal.h:58"]
pub mod signal_h {
    #[c2rust::src_loc = "72:1"]
    pub type __sighandler_t
        =
        Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "88:1"]
        pub fn signal(__sig: libc::c_int, __handler: __sighandler_t)
         -> __sighandler_t;
    }
}
#[c2rust::header_src = "/usr/include/ctype.h:64"]
pub mod ctype_h {
    #[c2rust::src_loc = "46:1"]
    pub type C2RustUnnamed_4 = libc::c_uint;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed_4 = 8;
    #[c2rust::src_loc = "58:3"]
    pub const _ISpunct: C2RustUnnamed_4 = 4;
    #[c2rust::src_loc = "57:3"]
    pub const _IScntrl: C2RustUnnamed_4 = 2;
    #[c2rust::src_loc = "56:3"]
    pub const _ISblank: C2RustUnnamed_4 = 1;
    #[c2rust::src_loc = "55:3"]
    pub const _ISgraph: C2RustUnnamed_4 = 32768;
    #[c2rust::src_loc = "54:3"]
    pub const _ISprint: C2RustUnnamed_4 = 16384;
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed_4 = 8192;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed_4 = 4096;
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed_4 = 2048;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed_4 = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed_4 = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed_4 = 256;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:66"]
pub mod gssapi_h {
    #[c2rust::src_loc = "79:1"]
    pub type gss_name_t = *mut gss_name_struct;
    #[c2rust::src_loc = "82:1"]
    pub type gss_cred_id_t = *mut gss_cred_id_struct;
    #[c2rust::src_loc = "85:1"]
    pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
    /*
 * The following type must be defined as the smallest natural unsigned integer
 * supported by the platform that has at least 32 bits of precision.
 */
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    /* OM_STRING */
    /*
 * We can't use X/Open definitions, so roll our own.
 */
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "106:16"]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:16"]
    pub struct gss_OID_set_desc_struct {
        pub count: size_t,
        pub elements: gss_OID,
    }
    /* OM_STRING */
    #[c2rust::src_loc = "112:1"]
    pub type gss_OID_set = *mut gss_OID_set_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_desc = gss_buffer_desc_struct;
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "122:16"]
    pub struct gss_channel_bindings_struct {
        pub initiator_addrtype: OM_uint32,
        pub initiator_address: gss_buffer_desc,
        pub acceptor_addrtype: OM_uint32,
        pub acceptor_address: gss_buffer_desc,
        pub application_data: gss_buffer_desc,
    }
    #[c2rust::src_loc = "122:1"]
    pub type gss_channel_bindings_t = *mut gss_channel_bindings_struct;
    /*
 * For now, define a QOP-type as an OM_uint32 (pending resolution of ongoing
 * discussions).
 */
    #[c2rust::src_loc = "134:1"]
    pub type gss_qop_t = OM_uint32;
    #[c2rust::src_loc = "135:1"]
    pub type gss_cred_usage_t = libc::c_int;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        /* This is the gssapi.h prologue. */
/* no xom.h */
/* End of gssapi.h prologue. */
/* -*- mode: c; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 by OpenVision Technologies, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
        /*
 * Determine platform-dependent configuration.
 */
        /* __cplusplus */
        /*
 * First, include stddef.h to get size_t defined.
 */
        /*
 * POSIX says that sys/types.h is where size_t is defined.
 */
        /*
 * $Id$
 */
        /*
 * First, define the three platform-dependent pointer types.
 */
        #[c2rust::src_loc = "78:8"]
        pub type gss_name_struct;
        #[c2rust::src_loc = "81:8"]
        pub type gss_cred_id_struct;
        #[c2rust::src_loc = "84:8"]
        pub type gss_ctx_id_struct;
        /* Function Prototypes */
        #[no_mangle]
        #[c2rust::src_loc = "421:1"]
        pub fn gss_acquire_cred(_: *mut OM_uint32, _: gss_name_t,
                                _: OM_uint32, _: gss_OID_set,
                                _: gss_cred_usage_t, _: *mut gss_cred_id_t,
                                _: *mut gss_OID_set, _: *mut OM_uint32)
         -> OM_uint32;
        /* time_rec */
        #[no_mangle]
        #[c2rust::src_loc = "432:1"]
        pub fn gss_release_cred(_: *mut OM_uint32, _: *mut gss_cred_id_t)
         -> OM_uint32;
        /* time_rec */
        #[no_mangle]
        #[c2rust::src_loc = "453:1"]
        pub fn gss_accept_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_cred_id_t, _: gss_buffer_t,
                                      _: gss_channel_bindings_t,
                                      _: *mut gss_name_t, _: *mut gss_OID,
                                      _: gss_buffer_t, _: *mut OM_uint32,
                                      _: *mut OM_uint32,
                                      _: *mut gss_cred_id_t) -> OM_uint32;
        /* token_buffer */
        #[no_mangle]
        #[c2rust::src_loc = "474:1"]
        pub fn gss_delete_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_buffer_t) -> OM_uint32;
        /* time_rec */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "489:1"]
        pub fn gss_get_mic(_: *mut OM_uint32, _: gss_ctx_id_t, _: gss_qop_t,
                           _: gss_buffer_t, _: gss_buffer_t) -> OM_uint32;
        /* output_message_buffer */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "520:1"]
        pub fn gss_unwrap(_: *mut OM_uint32, _: gss_ctx_id_t, _: gss_buffer_t,
                          _: gss_buffer_t, _: *mut libc::c_int,
                          _: *mut gss_qop_t) -> OM_uint32;
        /* name_equal */
        #[no_mangle]
        #[c2rust::src_loc = "554:1"]
        pub fn gss_display_name(_: *mut OM_uint32, _: gss_name_t,
                                _: gss_buffer_t, _: *mut gss_OID)
         -> OM_uint32;
        /* output_name_type */
        #[no_mangle]
        #[c2rust::src_loc = "562:1"]
        pub fn gss_import_name(_: *mut OM_uint32, _: gss_buffer_t, _: gss_OID,
                               _: *mut gss_name_t) -> OM_uint32;
        /* output_name */
        #[no_mangle]
        #[c2rust::src_loc = "569:1"]
        pub fn gss_release_name(_: *mut OM_uint32, _: *mut gss_name_t)
         -> OM_uint32;
        /* input_name */
        #[no_mangle]
        #[c2rust::src_loc = "574:1"]
        pub fn gss_release_buffer(_: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
        /* cred_usage */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "659:1"]
        pub fn gss_export_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_buffer_t) -> OM_uint32;
        /* interprocess_token */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "666:1"]
        pub fn gss_import_sec_context(_: *mut OM_uint32, _: gss_buffer_t,
                                      _: *mut gss_ctx_id_t) -> OM_uint32;
        /* oid */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "707:1"]
        pub fn gss_oid_to_str(_: *mut OM_uint32, _: gss_OID, _: gss_buffer_t)
         -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src = "/usr/include/stdio.h:48"]
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
        #[c2rust::src_loc = "246:14"]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut FILE;
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
#[c2rust::header_src = "/usr/include/pthread.h:57"]
pub mod pthread_h {
    use super::pthreadtypes_h::{pthread_t, pthread_attr_t, pthread_mutex_t,
                                pthread_cond_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "234:1"]
        pub fn pthread_create(__newthread: *mut pthread_t,
                              __attr: *const pthread_attr_t,
                              __start_routine:
                                  Option<unsafe extern "C" fn(_:
                                                                  *mut libc::c_void)
                                             -> *mut libc::c_void>,
                              __arg: *mut libc::c_void) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "272:1"]
        pub fn pthread_detach(__th: pthread_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "763:1"]
        pub fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "781:1"]
        pub fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1003:1"]
        pub fn pthread_cond_broadcast(__cond: *mut pthread_cond_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1011:1"]
        pub fn pthread_cond_wait(__cond: *mut pthread_cond_t,
                                 __mutex: *mut pthread_mutex_t)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:61"]
pub mod unistd_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "444:1"]
        pub fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:63"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "104:1"]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_generic.h:66"]
pub mod gssapi_generic_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "54:27"]
        pub static mut gss_nt_service_name: gss_OID;
    }
    /* _GSSAPI_GENERIC_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/tests/gss-threads/gss-misc.h:67"]
pub mod gss_misc_h {
    use super::FILE_h::FILE;
    use super::gssapi_h::{gss_buffer_desc_struct, gss_buffer_t, OM_uint32};
    extern "C" {
        /*
 * Copyright 1994 by OpenVision Technologies, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
        #[no_mangle]
        #[c2rust::src_loc = "29:14"]
        pub static mut display_file: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "31:1"]
        pub fn send_token(s: libc::c_int, flags: libc::c_int,
                          tok: gss_buffer_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "32:1"]
        pub fn recv_token(s: libc::c_int, flags: *mut libc::c_int,
                          tok: gss_buffer_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "33:1"]
        pub fn display_status(msg: *mut libc::c_char, maj_stat: OM_uint32,
                              min_stat: OM_uint32);
        #[no_mangle]
        #[c2rust::src_loc = "34:1"]
        pub fn display_ctx_flags(flags: OM_uint32);
        #[no_mangle]
        #[c2rust::src_loc = "35:1"]
        pub fn print_token(tok: gss_buffer_t);
        #[no_mangle]
        #[c2rust::src_loc = "49:21"]
        pub static mut empty_token: gss_buffer_t;
    }
}
#[c2rust::header_src = "/usr/include/string.h:68"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__u_short, __uint8_t, __uint16_t, __uint32_t, __off_t,
                        __off64_t, __time_t, __suseconds_t, __socklen_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::sys_types_h::u_short;
pub use self::struct_timeval_h::timeval;
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s,
                                      __pthread_cond_s, C2RustUnnamed,
                                      C2RustUnnamed_0, C2RustUnnamed_1,
                                      C2RustUnnamed_2};
pub use self::pthreadtypes_h::{pthread_t, pthread_attr_t, pthread_mutex_t,
                               pthread_cond_t};
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
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed_3, in_port_t,
                     sockaddr_in, in_addr, in_addr_t, htons};
pub use self::stdint_uintn_h::{uint32_t, uint16_t, uint8_t};
pub use self::time_h::{timezone, __timezone_ptr_t, gettimeofday};
pub use self::signal_h::{__sighandler_t, signal};
pub use self::ctype_h::{C2RustUnnamed_4, _ISalnum, _ISpunct, _IScntrl,
                        _ISblank, _ISgraph, _ISprint, _ISspace, _ISxdigit,
                        _ISdigit, _ISalpha, _ISlower, _ISupper,
                        __ctype_b_loc};
pub use self::gssapi_h::{gss_name_t, gss_cred_id_t, gss_ctx_id_t, gss_uint32,
                         OM_uint32, gss_OID_desc_struct, gss_OID,
                         gss_OID_set_desc_struct, gss_OID_set,
                         gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_channel_bindings_struct,
                         gss_channel_bindings_t, gss_qop_t, gss_cred_usage_t,
                         gss_name_struct, gss_cred_id_struct,
                         gss_ctx_id_struct, gss_acquire_cred,
                         gss_release_cred, gss_accept_sec_context,
                         gss_delete_sec_context, gss_get_mic, gss_unwrap,
                         gss_display_name, gss_import_name, gss_release_name,
                         gss_release_buffer, gss_export_sec_context,
                         gss_import_sec_context, gss_oid_to_str};
use self::stdio_h::{stdout, stderr, fflush, fopen, fprintf, printf, perror};
use self::pthread_h::{pthread_create, pthread_detach, pthread_mutex_lock,
                      pthread_mutex_unlock, pthread_cond_broadcast,
                      pthread_cond_wait};
use self::unistd_h::{sleep, close};
use self::stdlib_h::{atoi, malloc, free, exit};
use self::gssapi_generic_h::gss_nt_service_name;
use self::gss_misc_h::{display_file, send_token, recv_token, display_status,
                       display_ctx_flags, print_token, empty_token};
use self::string_h::{memcpy, strcmp, strlen};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "662:8"]
pub struct _work_plan {
    pub s: libc::c_int,
    pub server_creds: gss_cred_id_t,
    pub export: libc::c_int,
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1994 by OpenVision Technologies, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
/*
 * Copyright (C) 2004,2008 by the Massachusetts Institute of Technology.
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
#[c2rust::src_loc = "76:1"]
unsafe extern "C" fn usage() {
    fprintf(stderr,
            b"Usage: gss-server [-port port] [-verbose] [-once]\x00" as
                *const u8 as *const libc::c_char);
    fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"       [-inetd] [-export] [-logfile file] service_name\n\x00" as
                *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "89:7"]
pub static mut logfile: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
#[c2rust::src_loc = "91:5"]
pub static mut verbose: libc::c_int = 0 as libc::c_int;
/*
 * Function: server_acquire_creds
 *
 * Purpose: imports a service name and acquires credentials for it
 *
 * Arguments:
 *
 *      service_name    (r) the ASCII service name
 *      server_creds    (w) the GSS-API service credentials
 *
 * Returns: 0 on success, -1 on failure
 *
 * Effects:
 *
 * The service name is imported with gss_import_name, and service
 * credentials are acquired with gss_acquire_cred.  If either opertion
 * fails, an error message is displayed and -1 is returned; otherwise,
 * 0 is returned.
 */
#[c2rust::src_loc = "112:1"]
unsafe extern "C" fn server_acquire_creds(mut service_name: *mut libc::c_char,
                                          mut server_creds:
                                              *mut gss_cred_id_t)
 -> libc::c_int {
    let mut name_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut server_name: gss_name_t = 0 as *mut gss_name_struct;
    let mut maj_stat: OM_uint32 = 0;
    let mut min_stat: OM_uint32 = 0;
    name_buf.value = service_name as *mut libc::c_void;
    name_buf.length =
        strlen(name_buf.value as
                   *const libc::c_char).wrapping_add(1 as libc::c_int as
                                                         libc::c_ulong);
    maj_stat =
        gss_import_name(&mut min_stat, &mut name_buf, gss_nt_service_name,
                        &mut server_name);
    if maj_stat != 0 as libc::c_int as libc::c_uint {
        display_status(b"importing name\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char, maj_stat,
                       min_stat);
        return -(1 as libc::c_int)
    }
    maj_stat =
        gss_acquire_cred(&mut min_stat, server_name,
                         0 as libc::c_int as OM_uint32, 0 as gss_OID_set,
                         2 as libc::c_int, server_creds,
                         0 as *mut gss_OID_set, 0 as *mut OM_uint32);
    if maj_stat != 0 as libc::c_int as libc::c_uint {
        display_status(b"acquiring credentials\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char, maj_stat,
                       min_stat);
        return -(1 as libc::c_int)
    }
    gss_release_name(&mut min_stat, &mut server_name);
    return 0 as libc::c_int;
}
/*
 * Function: server_establish_context
 *
 * Purpose: establishses a GSS-API context as a specified service with
 * an incoming client, and returns the context handle and associated
 * client name
 *
 * Arguments:
 *
 *      s               (r) an established TCP connection to the client
 *      service_creds   (r) server credentials, from gss_acquire_cred
 *      context         (w) the established GSS-API context
 *      client_name     (w) the client's ASCII name
 *
 * Returns: 0 on success, -1 on failure
 *
 * Effects:
 *
 * Any valid client request is accepted.  If a context is established,
 * its handle is returned in context and the client name is returned
 * in client_name and 0 is returned.  If unsuccessful, an error
 * message is displayed and -1 is returned.
 */
#[c2rust::src_loc = "164:1"]
unsafe extern "C" fn server_establish_context(mut s: libc::c_int,
                                              mut server_creds: gss_cred_id_t,
                                              mut context: *mut gss_ctx_id_t,
                                              mut client_name: gss_buffer_t,
                                              mut ret_flags: *mut OM_uint32)
 -> libc::c_int {
    let mut send_tok: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut recv_tok: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut oid_name: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut client: gss_name_t = 0 as *mut gss_name_struct;
    let mut doid: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut maj_stat: OM_uint32 = 0;
    let mut min_stat: OM_uint32 = 0;
    let mut acc_sec_min_stat: OM_uint32 = 0;
    let mut token_flags: libc::c_int = 0;
    if recv_token(s, &mut token_flags, &mut recv_tok) < 0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    if !recv_tok.value.is_null() {
        free(recv_tok.value);
        recv_tok.value = 0 as *mut libc::c_void
    }
    if token_flags & (1 as libc::c_int) << 0 as libc::c_int == 0 {
        if !logfile.is_null() {
            fprintf(logfile,
                    b"Expected NOOP token, got %d token instead\n\x00" as
                        *const u8 as *const libc::c_char, token_flags);
        }
        return -(1 as libc::c_int)
    }
    *context = 0 as gss_ctx_id_t;
    if token_flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        loop  {
            if recv_token(s, &mut token_flags, &mut recv_tok) <
                   0 as libc::c_int {
                return -(1 as libc::c_int)
            }
            if verbose != 0 && !logfile.is_null() {
                fprintf(logfile,
                        b"Received token (size=%d): \n\x00" as *const u8 as
                            *const libc::c_char,
                        recv_tok.length as libc::c_int);
                print_token(&mut recv_tok);
            }
            maj_stat =
                gss_accept_sec_context(&mut acc_sec_min_stat, context,
                                       server_creds, &mut recv_tok,
                                       0 as gss_channel_bindings_t,
                                       &mut client, &mut doid, &mut send_tok,
                                       ret_flags, 0 as *mut OM_uint32,
                                       0 as *mut gss_cred_id_t);
            if !recv_tok.value.is_null() {
                free(recv_tok.value);
                recv_tok.value = 0 as *mut libc::c_void
            }
            if send_tok.length != 0 as libc::c_int as libc::c_ulong {
                if verbose != 0 && !logfile.is_null() {
                    fprintf(logfile,
                            b"Sending accept_sec_context token (size=%d):\n\x00"
                                as *const u8 as *const libc::c_char,
                            send_tok.length as libc::c_int);
                    print_token(&mut send_tok);
                }
                if send_token(s, (1 as libc::c_int) << 1 as libc::c_int,
                              &mut send_tok) < 0 as libc::c_int {
                    if !logfile.is_null() {
                        fprintf(logfile,
                                b"failure sending token\n\x00" as *const u8 as
                                    *const libc::c_char);
                    }
                    return -(1 as libc::c_int)
                }
                gss_release_buffer(&mut min_stat, &mut send_tok);
            }
            if maj_stat != 0 as libc::c_int as libc::c_uint &&
                   maj_stat !=
                       ((1 as libc::c_int) <<
                            0 as libc::c_int + 0 as libc::c_int) as
                           libc::c_uint {
                display_status(b"accepting context\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                               maj_stat, acc_sec_min_stat);
                if !(*context).is_null() {
                    gss_delete_sec_context(&mut min_stat, context,
                                           0 as gss_buffer_t);
                }
                return -(1 as libc::c_int)
            }
            if verbose != 0 && !logfile.is_null() {
                if maj_stat ==
                       ((1 as libc::c_int) <<
                            0 as libc::c_int + 0 as libc::c_int) as
                           libc::c_uint {
                    fprintf(logfile,
                            b"continue needed...\n\x00" as *const u8 as
                                *const libc::c_char);
                } else {
                    fprintf(logfile,
                            b"\n\x00" as *const u8 as *const libc::c_char);
                }
                fflush(logfile);
            }
            if !(maj_stat ==
                     ((1 as libc::c_int) <<
                          0 as libc::c_int + 0 as libc::c_int) as
                         libc::c_uint) {
                break ;
            }
        }
        /* display the flags */
        display_ctx_flags(*ret_flags);
        if verbose != 0 && !logfile.is_null() {
            maj_stat = gss_oid_to_str(&mut min_stat, doid, &mut oid_name);
            if maj_stat != 0 as libc::c_int as libc::c_uint {
                display_status(b"converting oid->string\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                               maj_stat, min_stat);
                return -(1 as libc::c_int)
            }
            fprintf(logfile,
                    b"Accepted connection using mechanism OID %.*s.\n\x00" as
                        *const u8 as *const libc::c_char,
                    oid_name.length as libc::c_int,
                    oid_name.value as *mut libc::c_char);
            gss_release_buffer(&mut min_stat, &mut oid_name);
        }
        maj_stat =
            gss_display_name(&mut min_stat, client, client_name, &mut doid);
        if maj_stat != 0 as libc::c_int as libc::c_uint {
            display_status(b"displaying name\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,
                           maj_stat, min_stat);
            return -(1 as libc::c_int)
        }
        maj_stat = gss_release_name(&mut min_stat, &mut client);
        if maj_stat != 0 as libc::c_int as libc::c_uint {
            display_status(b"releasing name\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,
                           maj_stat, min_stat);
            return -(1 as libc::c_int)
        }
    } else {
        *ret_flags = 0 as libc::c_int as OM_uint32;
        (*client_name).length = *ret_flags as size_t;
        if !logfile.is_null() {
            fprintf(logfile,
                    b"Accepted unauthenticated connection.\n\x00" as *const u8
                        as *const libc::c_char);
        }
    }
    return 0 as libc::c_int;
}
/*
 * Function: create_socket
 *
 * Purpose: Opens a listening TCP socket.
 *
 * Arguments:
 *
 *      port            (r) the port number on which to listen
 *
 * Returns: the listening socket file descriptor, or -1 on failure
 *
 * Effects:
 *
 * A listening socket on the specified port and created and returned.
 * On error, an error message is displayed and -1 is returned.
 */
#[c2rust::src_loc = "300:1"]
unsafe extern "C" fn create_socket(mut port: u_short) -> libc::c_int {
    let mut saddr: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut s: libc::c_int = 0;
    let mut on: libc::c_int = 1 as libc::c_int;
    saddr.sin_family = 2 as libc::c_int as sa_family_t;
    saddr.sin_port = htons(port);
    saddr.sin_addr.s_addr = 0 as libc::c_int as in_addr_t;
    s =
        socket(2 as libc::c_int, SOCK_STREAM as libc::c_int,
               0 as libc::c_int);
    if s < 0 as libc::c_int {
        perror(b"creating socket\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    /* Let the socket be reused right away. */
    setsockopt(s, 1 as libc::c_int, 2 as libc::c_int,
               &mut on as *mut libc::c_int as *mut libc::c_char as
                   *const libc::c_void,
               ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                   socklen_t);
    if bind(s,
            __CONST_SOCKADDR_ARG{__sockaddr__:
                                     &mut saddr as *mut sockaddr_in as
                                         *mut sockaddr,},
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                socklen_t) < 0 as libc::c_int {
        perror(b"binding socket\x00" as *const u8 as *const libc::c_char);
        close(s);
        return -(1 as libc::c_int)
    }
    if listen(s, 5 as libc::c_int) < 0 as libc::c_int {
        perror(b"listening on socket\x00" as *const u8 as
                   *const libc::c_char);
        close(s);
        return -(1 as libc::c_int)
    }
    return s;
}
#[c2rust::src_loc = "330:1"]
unsafe extern "C" fn timeval_subtract(mut tv1: *mut timeval,
                                      mut tv2: *mut timeval)
 -> libc::c_float {
    return ((*tv1).tv_sec - (*tv2).tv_sec) as libc::c_float +
               ((*tv1).tv_usec - (*tv2).tv_usec) as libc::c_float /
                   1000000 as libc::c_int as libc::c_float;
}
/*
 * Yes, yes, this isn't the best place for doing this test.
 * DO NOT REMOVE THIS UNTIL A BETTER TEST HAS BEEN WRITTEN, THOUGH.
 *                                      -TYT
 */
#[c2rust::src_loc = "342:1"]
unsafe extern "C" fn test_import_export_context(mut context:
                                                    *mut gss_ctx_id_t)
 -> libc::c_int {
    let mut min_stat: OM_uint32 = 0;
    let mut maj_stat: OM_uint32 = 0;
    let mut context_token: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut copied_token: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut tm1: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut tm2: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    /* Attempt to save and then restore the context. */
    gettimeofday(&mut tm1, 0 as *mut timezone);
    maj_stat =
        gss_export_sec_context(&mut min_stat, context, &mut context_token);
    if maj_stat != 0 as libc::c_int as libc::c_uint {
        display_status(b"exporting context\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char, maj_stat,
                       min_stat);
        return 1 as libc::c_int
    }
    gettimeofday(&mut tm2, 0 as *mut timezone);
    if verbose != 0 && !logfile.is_null() {
        fprintf(logfile,
                b"Exported context: %d bytes, %7.4f seconds\n\x00" as
                    *const u8 as *const libc::c_char,
                context_token.length as libc::c_int,
                timeval_subtract(&mut tm2, &mut tm1) as libc::c_double);
    }
    copied_token.length = context_token.length;
    copied_token.value = malloc(context_token.length);
    if copied_token.value.is_null() {
        if !logfile.is_null() {
            fprintf(logfile,
                    b"Couldn\'t allocate memory to copy context token.\n\x00"
                        as *const u8 as *const libc::c_char);
        }
        return 1 as libc::c_int
    }
    memcpy(copied_token.value, context_token.value, copied_token.length);
    maj_stat =
        gss_import_sec_context(&mut min_stat, &mut copied_token, context);
    if maj_stat != 0 as libc::c_int as libc::c_uint {
        display_status(b"importing context\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char, maj_stat,
                       min_stat);
        return 1 as libc::c_int
    }
    free(copied_token.value);
    gettimeofday(&mut tm1, 0 as *mut timezone);
    if verbose != 0 && !logfile.is_null() {
        fprintf(logfile,
                b"Importing context: %7.4f seconds\n\x00" as *const u8 as
                    *const libc::c_char,
                timeval_subtract(&mut tm1, &mut tm2) as libc::c_double);
    }
    gss_release_buffer(&mut min_stat, &mut context_token);
    return 0 as libc::c_int;
}
/*
 * Function: sign_server
 *
 * Purpose: Performs the "sign" service.
 *
 * Arguments:
 *
 *      s               (r) a TCP socket on which a connection has been
 *                      accept()ed
 *      service_name    (r) the ASCII name of the GSS-API service to
 *                      establish a context as
 *      export          (r) whether to test context exporting
 *
 * Returns: -1 on error
 *
 * Effects:
 *
 * sign_server establishes a context, and performs a single sign request.
 *
 * A sign request is a single GSS-API sealed token.  The token is
 * unsealed and a signature block, produced with gss_sign, is returned
 * to the sender.  The context is the destroyed and the connection
 * closed.
 *
 * If any error occurs, -1 is returned.
 */
#[c2rust::src_loc = "412:1"]
unsafe extern "C" fn sign_server(mut s: libc::c_int,
                                 mut server_creds: gss_cred_id_t,
                                 mut export: libc::c_int) -> libc::c_int {
    let mut client_name: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut xmit_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut msg_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut context: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut maj_stat: OM_uint32 = 0;
    let mut min_stat: OM_uint32 = 0;
    let mut ret_flags: OM_uint32 = 0;
    let mut i: libc::c_int = 0;
    let mut conf_state: libc::c_int = 0;
    let mut token_flags: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Establish a context with the client */
    if server_establish_context(s, server_creds, &mut context,
                                &mut client_name, &mut ret_flags) <
           0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    if context.is_null() {
        printf(b"Accepted unauthenticated connection.\n\x00" as *const u8 as
                   *const libc::c_char);
    } else {
        printf(b"Accepted connection: \"%.*s\"\n\x00" as *const u8 as
                   *const libc::c_char, client_name.length as libc::c_int,
               client_name.value as *mut libc::c_char);
        gss_release_buffer(&mut min_stat, &mut client_name);
        if export != 0 {
            i = 0 as libc::c_int;
            while i < 3 as libc::c_int {
                if test_import_export_context(&mut context) != 0 {
                    return -(1 as libc::c_int)
                }
                i += 1
            }
        }
    }
    loop  {
        /* Receive the message token */
        if recv_token(s, &mut token_flags, &mut xmit_buf) < 0 as libc::c_int {
            return -(1 as libc::c_int)
        }
        if token_flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
            if !logfile.is_null() {
                fprintf(logfile,
                        b"NOOP token\n\x00" as *const u8 as
                            *const libc::c_char);
            }
            if !xmit_buf.value.is_null() {
                free(xmit_buf.value);
                xmit_buf.value = 0 as *mut libc::c_void
            }
            break ;
        } else {
            if verbose != 0 && !logfile.is_null() {
                fprintf(logfile,
                        b"Message token (flags=%d):\n\x00" as *const u8 as
                            *const libc::c_char, token_flags);
                print_token(&mut xmit_buf);
            }
            if context.is_null() &&
                   token_flags &
                       ((1 as libc::c_int) << 5 as libc::c_int |
                            (1 as libc::c_int) << 6 as libc::c_int |
                            (1 as libc::c_int) << 7 as libc::c_int) != 0 {
                if !logfile.is_null() {
                    fprintf(logfile,
                            b"Unauthenticated client requested authenticated services!\n\x00"
                                as *const u8 as *const libc::c_char);
                }
                if !xmit_buf.value.is_null() {
                    free(xmit_buf.value);
                    xmit_buf.value = 0 as *mut libc::c_void
                }
                return -(1 as libc::c_int)
            }
            if token_flags & (1 as libc::c_int) << 5 as libc::c_int != 0 {
                maj_stat =
                    gss_unwrap(&mut min_stat, context, &mut xmit_buf,
                               &mut msg_buf, &mut conf_state,
                               0 as *mut gss_qop_t);
                if maj_stat != 0 as libc::c_int as libc::c_uint {
                    display_status(b"unsealing message\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char, maj_stat, min_stat);
                    if !xmit_buf.value.is_null() {
                        free(xmit_buf.value);
                        xmit_buf.value = 0 as *mut libc::c_void
                    }
                    return -(1 as libc::c_int)
                } else {
                    if conf_state == 0 &&
                           token_flags &
                               (1 as libc::c_int) << 6 as libc::c_int != 0 {
                        fprintf(stderr,
                                b"Warning!  Message not encrypted.\n\x00" as
                                    *const u8 as *const libc::c_char);
                    }
                }
                if !xmit_buf.value.is_null() {
                    free(xmit_buf.value);
                    xmit_buf.value = 0 as *mut libc::c_void
                }
            } else { msg_buf = xmit_buf }
            if !logfile.is_null() {
                fprintf(logfile,
                        b"Received message: \x00" as *const u8 as
                            *const libc::c_char);
                cp = msg_buf.value as *mut libc::c_char;
                if *(*__ctype_b_loc()).offset(*cp.offset(0 as libc::c_int as
                                                             isize) as
                                                  libc::c_uchar as libc::c_int
                                                  as isize) as libc::c_int &
                       _ISprint as libc::c_int as libc::c_ushort as
                           libc::c_int != 0 &&
                       *(*__ctype_b_loc()).offset(*cp.offset(1 as libc::c_int
                                                                 as isize) as
                                                      libc::c_uchar as
                                                      libc::c_int as isize) as
                           libc::c_int &
                           _ISprint as libc::c_int as libc::c_ushort as
                               libc::c_int != 0 {
                    fprintf(logfile,
                            b"\"%.*s\"\n\x00" as *const u8 as
                                *const libc::c_char,
                            msg_buf.length as libc::c_int,
                            msg_buf.value as *mut libc::c_char);
                } else {
                    fprintf(logfile,
                            b"\n\x00" as *const u8 as *const libc::c_char);
                    print_token(&mut msg_buf);
                }
            }
            if token_flags & (1 as libc::c_int) << 7 as libc::c_int != 0 {
                /* loop will break if NOOP received */
                /* Produce a signature block for the message. */
                maj_stat =
                    gss_get_mic(&mut min_stat, context,
                                0 as libc::c_int as gss_qop_t, &mut msg_buf,
                                &mut xmit_buf);
                if maj_stat != 0 as libc::c_int as libc::c_uint {
                    display_status(b"signing message\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char, maj_stat, min_stat);
                    return -(1 as libc::c_int)
                }
                if !msg_buf.value.is_null() {
                    free(msg_buf.value);
                    msg_buf.value = 0 as *mut libc::c_void
                }
                /* Send the signature block to the client. */
                if send_token(s, (1 as libc::c_int) << 3 as libc::c_int,
                              &mut xmit_buf) < 0 as libc::c_int {
                    return -(1 as libc::c_int)
                }
                if !xmit_buf.value.is_null() {
                    free(xmit_buf.value);
                    xmit_buf.value = 0 as *mut libc::c_void
                }
            } else {
                if !msg_buf.value.is_null() {
                    free(msg_buf.value);
                    msg_buf.value = 0 as *mut libc::c_void
                }
                if send_token(s, (1 as libc::c_int) << 0 as libc::c_int,
                              empty_token) < 0 as libc::c_int {
                    return -(1 as libc::c_int)
                }
            }
        }
    }
    if !context.is_null() {
        /* Delete context. */
        maj_stat =
            gss_delete_sec_context(&mut min_stat, &mut context,
                                   0 as gss_buffer_t);
        if maj_stat != 0 as libc::c_int as libc::c_uint {
            display_status(b"deleting context\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,
                           maj_stat, min_stat);
            return -(1 as libc::c_int)
        }
    }
    if !logfile.is_null() { fflush(logfile); }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "557:12"]
static mut max_threads: libc::c_int = 1 as libc::c_int;
/* assume pthread */
#[c2rust::src_loc = "617:24"]
static mut counter_mutex: pthread_mutex_t =
    pthread_mutex_t{__data:
                        {
                            let mut init =
                                __pthread_mutex_s{__lock: 0 as libc::c_int,
                                                  __count:
                                                      0 as libc::c_int as
                                                          libc::c_uint,
                                                  __owner: 0 as libc::c_int,
                                                  __nusers:
                                                      0 as libc::c_int as
                                                          libc::c_uint,
                                                  __kind: 0 as libc::c_int,
                                                  __spins:
                                                      0 as libc::c_int as
                                                          libc::c_short,
                                                  __elision:
                                                      0 as libc::c_int as
                                                          libc::c_short,
                                                  __list:
                                                      {
                                                          let mut init =
                                                              __pthread_internal_list{__prev:
                                                                                          0
                                                                                              as
                                                                                              *const __pthread_internal_list
                                                                                              as
                                                                                              *mut __pthread_internal_list,
                                                                                      __next:
                                                                                          0
                                                                                              as
                                                                                              *const __pthread_internal_list
                                                                                              as
                                                                                              *mut __pthread_internal_list,};
                                                          init
                                                      },};
                            init
                        },};
#[c2rust::src_loc = "618:23"]
static mut counter_cond: pthread_cond_t =
    pthread_cond_t{__data:
                       {
                           let mut init =
                               __pthread_cond_s{c2rust_unnamed:
                                                    C2RustUnnamed_1{__wseq:
                                                                        0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_ulonglong,},
                                                c2rust_unnamed_0:
                                                    C2RustUnnamed{__g1_start:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_ulonglong,},
                                                __g_refs:
                                                    [0 as libc::c_int as
                                                         libc::c_uint,
                                                     0 as libc::c_int as
                                                         libc::c_uint],
                                                __g_size:
                                                    [0 as libc::c_int as
                                                         libc::c_uint,
                                                     0 as libc::c_int as
                                                         libc::c_uint],
                                                __g1_orig_size:
                                                    0 as libc::c_int as
                                                        libc::c_uint,
                                                __wrefs:
                                                    0 as libc::c_int as
                                                        libc::c_uint,
                                                __g_signals:
                                                    [0 as libc::c_int as
                                                         libc::c_uint,
                                                     0 as libc::c_int as
                                                         libc::c_uint],};
                           init
                       },};
#[no_mangle]
#[c2rust::src_loc = "619:5"]
pub static mut counter: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "621:1"]
unsafe extern "C" fn wait_and_increment_thread_counter() -> libc::c_int {
    let mut err: libc::c_int = 0;
    err = pthread_mutex_lock(&mut counter_mutex);
    if err != 0 {
        perror(b"pthread_mutex_lock\x00" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    if counter == max_threads {
        err = pthread_cond_wait(&mut counter_cond, &mut counter_mutex);
        if err != 0 {
            pthread_mutex_unlock(&mut counter_mutex);
            perror(b"pthread_cond_wait\x00" as *const u8 as
                       *const libc::c_char);
            return 0 as libc::c_int
        }
    }
    counter += 1;
    pthread_mutex_unlock(&mut counter_mutex);
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "644:1"]
unsafe extern "C" fn decrement_and_signal_thread_counter() {
    let mut err: libc::c_int = 0;
    err = pthread_mutex_lock(&mut counter_mutex);
    if err != 0 {
        perror(b"pthread_mutex_lock\x00" as *const u8 as *const libc::c_char);
        return
    }
    if counter == max_threads { pthread_cond_broadcast(&mut counter_cond); }
    counter -= 1;
    pthread_mutex_unlock(&mut counter_mutex);
}
#[c2rust::src_loc = "668:1"]
unsafe extern "C" fn worker_bee(mut param: *mut libc::c_void)
 -> *mut libc::c_void {
    let mut work: *mut _work_plan = param as *mut _work_plan;
    /* This return value is not checked, because there's not really anything to
     * do if it fails. */
    sign_server((*work).s, (*work).server_creds, (*work).export);
    close((*work).s);
    free(work as *mut libc::c_void);
    if max_threads > 1 as libc::c_int {
        decrement_and_signal_thread_counter();
    }
    return 0 as *mut libc::c_void;
}
#[c2rust::src_loc = "686:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut service_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut server_creds: gss_cred_id_t = 0 as *mut gss_cred_id_struct;
    let mut min_stat: OM_uint32 = 0;
    let mut port: u_short = 4444 as libc::c_int as u_short;
    let mut once: libc::c_int = 0 as libc::c_int;
    let mut do_inetd: libc::c_int = 0 as libc::c_int;
    let mut export: libc::c_int = 0 as libc::c_int;
    signal(13 as libc::c_int,
           ::std::mem::transmute::<libc::intptr_t,
                                   __sighandler_t>(1 as libc::c_int as
                                                       libc::intptr_t));
    logfile = stdout;
    display_file = stdout;
    argc -= 1;
    argv = argv.offset(1);
    while argc != 0 {
        if strcmp(*argv, b"-port\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
            argc -= 1;
            argv = argv.offset(1);
            if argc == 0 { usage(); }
            port = atoi(*argv) as u_short
        } else if strcmp(*argv,
                         b"-threads\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            argc -= 1;
            argv = argv.offset(1);
            if argc == 0 { usage(); }
            max_threads = atoi(*argv)
        } else if strcmp(*argv,
                         b"-verbose\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            verbose = 1 as libc::c_int
        } else if strcmp(*argv,
                         b"-once\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            once = 1 as libc::c_int
        } else if strcmp(*argv,
                         b"-inetd\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            do_inetd = 1 as libc::c_int
        } else if strcmp(*argv,
                         b"-export\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            export = 1 as libc::c_int
        } else {
            if !(strcmp(*argv,
                        b"-logfile\x00" as *const u8 as *const libc::c_char)
                     == 0 as libc::c_int) {
                break ;
            }
            argc -= 1;
            argv = argv.offset(1);
            if argc == 0 { usage(); }
            /*
             * Gross hack, but it makes it unnecessary to add an extra argument
             * to disable logging, and makes the code more efficient because it
             * doesn't actually write data to /dev/null.
             */
            if strcmp(*argv,
                      b"/dev/null\x00" as *const u8 as *const libc::c_char) ==
                   0 {
                display_file = 0 as *mut FILE;
                logfile = display_file
            } else {
                logfile =
                    fopen(*argv,
                          b"a\x00" as *const u8 as *const libc::c_char);
                display_file = logfile;
                if logfile.is_null() {
                    perror(*argv);
                    exit(1 as libc::c_int);
                }
            }
        }
        argc -= 1;
        argv = argv.offset(1)
    }
    if argc != 1 as libc::c_int { usage(); }
    if *(*argv).offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
       {
        usage();
    }
    service_name = *argv;
    if server_acquire_creds(service_name, &mut server_creds) <
           0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    if do_inetd != 0 {
        close(1 as libc::c_int);
        close(2 as libc::c_int);
        sign_server(0 as libc::c_int, server_creds, export);
        close(0 as libc::c_int);
    } else {
        let mut stmp: libc::c_int = 0;
        stmp = create_socket(port);
        if stmp >= 0 as libc::c_int {
            if listen(stmp,
                      (if max_threads == 1 as libc::c_int {
                           0 as libc::c_int
                       } else { max_threads })) < 0 as libc::c_int {
                perror(b"listening on socket\x00" as *const u8 as
                           *const libc::c_char);
            }
            loop  {
                let mut work: *mut _work_plan =
                    malloc(::std::mem::size_of::<_work_plan>() as
                               libc::c_ulong) as *mut _work_plan;
                if work.is_null() {
                    fprintf(stderr,
                            b"fatal error: out of memory\x00" as *const u8 as
                                *const libc::c_char);
                    break ;
                } else {
                    /* Accept a TCP connection */
                    (*work).s =
                        accept(stmp,
                               __SOCKADDR_ARG{__sockaddr__:
                                                  0 as *mut libc::c_void as
                                                      *mut sockaddr,},
                               0 as *mut socklen_t);
                    if (*work).s < 0 as libc::c_int {
                        perror(b"accepting connection\x00" as *const u8 as
                                   *const libc::c_char);
                    } else {
                        (*work).server_creds = server_creds;
                        (*work).export = export;
                        if max_threads == 1 as libc::c_int {
                            worker_bee(work as *mut libc::c_void);
                        } else if wait_and_increment_thread_counter() != 0 {
                            let mut err: libc::c_int = 0;
                            let mut thr: pthread_t = 0;
                            err =
                                pthread_create(&mut thr,
                                               0 as *const pthread_attr_t,
                                               Some(worker_bee as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut libc::c_void)
                                                            ->
                                                                *mut libc::c_void),
                                               work as *mut libc::c_void);
                            if err != 0 {
                                perror(b"pthread_create\x00" as *const u8 as
                                           *const libc::c_char);
                                close((*work).s);
                                free(work as *mut libc::c_void);
                            }
                            pthread_detach(thr);
                        } else {
                            fprintf(stderr,
                                    b"fatal error incrementing thread counter\x00"
                                        as *const u8 as *const libc::c_char);
                            close((*work).s);
                            free(work as *mut libc::c_void);
                            break ;
                        }
                    }
                    if !(once == 0) { break ; }
                }
            }
            close(stmp);
        }
    }
    gss_release_cred(&mut min_stat, &mut server_creds);
    if max_threads > 1 as libc::c_int {
        loop  { sleep(999999 as libc::c_int as libc::c_uint); }
    }
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
