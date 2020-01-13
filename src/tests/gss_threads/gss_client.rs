use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:47"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:47"]
pub mod types_h {
    #[c2rust::src_loc = "32:1"]
    pub type __u_short = libc::c_ushort;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "145:1"]
    pub type __dev_t = libc::c_ulong;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "147:1"]
    pub type __gid_t = libc::c_uint;
    #[c2rust::src_loc = "148:1"]
    pub type __ino_t = libc::c_ulong;
    #[c2rust::src_loc = "150:1"]
    pub type __mode_t = libc::c_uint;
    #[c2rust::src_loc = "151:1"]
    pub type __nlink_t = libc::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "174:1"]
    pub type __blksize_t = libc::c_long;
    #[c2rust::src_loc = "179:1"]
    pub type __blkcnt_t = libc::c_long;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
    #[c2rust::src_loc = "196:1"]
    pub type __syscall_slong_t = libc::c_long;
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:47"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/sys/types.h:47"]
pub mod sys_types_h {
    #[c2rust::src_loc = "34:1"]
    pub type u_short = __u_short;
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::{__u_short, __ssize_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:47"]
pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "9:8"]
    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }
    use super::types_h::{__time_t, __syscall_slong_t};
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:47"]
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
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:47"]
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
#[c2rust::header_src = "/usr/include/bits/stat.h:47"]
pub mod stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
    pub struct stat {
        pub st_dev: __dev_t,
        pub st_ino: __ino_t,
        pub st_nlink: __nlink_t,
        pub st_mode: __mode_t,
        pub st_uid: __uid_t,
        pub st_gid: __gid_t,
        pub __pad0: libc::c_int,
        pub st_rdev: __dev_t,
        pub st_size: __off_t,
        pub st_blksize: __blksize_t,
        pub st_blocks: __blkcnt_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        pub __glibc_reserved: [__syscall_slong_t; 3],
    }
    use super::types_h::{__dev_t, __ino_t, __nlink_t, __mode_t, __uid_t,
                         __gid_t, __off_t, __blksize_t, __blkcnt_t,
                         __syscall_slong_t};
    use super::struct_timespec_h::timespec;
}
#[c2rust::header_src = "/usr/include/unistd.h:47"]
pub mod unistd_h {
    #[c2rust::src_loc = "274:1"]
    pub type socklen_t = __socklen_t;
    use super::types_h::__socklen_t;
    use super::stddef_h::size_t;
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "360:1"]
        pub fn read(__fd: libc::c_int, __buf: *mut libc::c_void,
                    __nbytes: size_t) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "444:1"]
        pub fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    }
}
#[c2rust::header_src = "/usr/include/ctype.h:53"]
pub mod ctype_h {
    #[c2rust::src_loc = "46:1"]
    pub type C2RustUnnamed_3 = libc::c_uint;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed_3 = 8;
    #[c2rust::src_loc = "58:3"]
    pub const _ISpunct: C2RustUnnamed_3 = 4;
    #[c2rust::src_loc = "57:3"]
    pub const _IScntrl: C2RustUnnamed_3 = 2;
    #[c2rust::src_loc = "56:3"]
    pub const _ISblank: C2RustUnnamed_3 = 1;
    #[c2rust::src_loc = "55:3"]
    pub const _ISgraph: C2RustUnnamed_3 = 32768;
    #[c2rust::src_loc = "54:3"]
    pub const _ISprint: C2RustUnnamed_3 = 16384;
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed_3 = 8192;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed_3 = 4096;
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed_3 = 2048;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed_3 = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed_3 = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed_3 = 256;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    }
}
#[c2rust::header_src = "/usr/include/bits/socket_type.h:55"]
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
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:55"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/bits/socket.h:55"]
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
#[c2rust::header_src = "/usr/include/sys/socket.h:55"]
pub mod sys_socket_h {
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
    use super::socket_h::sockaddr;
    use super::in_h::{sockaddr_in, sockaddr_in6};
    use super::unistd_h::socklen_t;
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
        #[c2rust::src_loc = "126:1"]
        pub fn connect(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG,
                       __len: socklen_t) -> libc::c_int;
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
        pub __in6_u: C2RustUnnamed_4,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "214:5"]
    pub union C2RustUnnamed_4 {
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
#[c2rust::header_src = "/usr/include/netdb.h:57"]
pub mod netdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "98:8"]
    pub struct hostent {
        pub h_name: *mut libc::c_char,
        pub h_aliases: *mut *mut libc::c_char,
        pub h_addrtype: libc::c_int,
        pub h_length: libc::c_int,
        pub h_addr_list: *mut *mut libc::c_char,
    }
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "142:1"]
        pub fn gethostbyname(__name: *const libc::c_char) -> *mut hostent;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:63"]
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
        /* cred_handle */
        #[no_mangle]
        #[c2rust::src_loc = "437:1"]
        pub fn gss_init_sec_context(_: *mut OM_uint32, _: gss_cred_id_t,
                                    _: *mut gss_ctx_id_t, _: gss_name_t,
                                    _: gss_OID, _: OM_uint32, _: OM_uint32,
                                    _: gss_channel_bindings_t,
                                    _: gss_buffer_t, _: *mut gss_OID,
                                    _: gss_buffer_t, _: *mut OM_uint32,
                                    _: *mut OM_uint32) -> OM_uint32;
        /* token_buffer */
        #[no_mangle]
        #[c2rust::src_loc = "474:1"]
        pub fn gss_delete_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_buffer_t) -> OM_uint32;
        /* message_token */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "499:1"]
        pub fn gss_verify_mic(_: *mut OM_uint32, _: gss_ctx_id_t,
                              _: gss_buffer_t, _: gss_buffer_t,
                              _: *mut gss_qop_t) -> OM_uint32;
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "508:1"]
        pub fn gss_wrap(_: *mut OM_uint32, _: gss_ctx_id_t, _: libc::c_int,
                        _: gss_qop_t, _: gss_buffer_t, _: *mut libc::c_int,
                        _: gss_buffer_t) -> OM_uint32;
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
        /* buffer */
        #[no_mangle]
        #[c2rust::src_loc = "579:1"]
        pub fn gss_release_oid_set(_: *mut OM_uint32, _: *mut gss_OID_set)
         -> OM_uint32;
        /* mechanisms */
        /* Last argument new for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "594:1"]
        pub fn gss_inquire_context(_: *mut OM_uint32, _: gss_ctx_id_t,
                                   _: *mut gss_name_t, _: *mut gss_name_t,
                                   _: *mut OM_uint32, _: *mut gss_OID,
                                   _: *mut OM_uint32, _: *mut libc::c_int,
                                   _: *mut libc::c_int) -> OM_uint32;
        /* context_handle */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "673:1"]
        pub fn gss_release_oid(_: *mut OM_uint32, _: *mut gss_OID)
         -> OM_uint32;
        /* present */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "700:1"]
        pub fn gss_str_to_oid(_: *mut OM_uint32, _: gss_buffer_t,
                              _: *mut gss_OID) -> OM_uint32;
        /* oid */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "707:1"]
        pub fn gss_oid_to_str(_: *mut OM_uint32, _: gss_OID, _: gss_buffer_t)
         -> OM_uint32;
        /* oid_str */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "714:1"]
        pub fn gss_inquire_names_for_mech(_: *mut OM_uint32, _: gss_OID,
                                          _: *mut gss_OID_set) -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src = "/usr/include/string.h:47"]
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
#[c2rust::header_src = "/usr/include/stdlib.h:47"]
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
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "775:1"]
        pub fn perror(__s: *const libc::c_char);
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:47"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "195:1"]
        pub fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/pthread.h:47"]
pub mod pthread_h {
    use super::pthreadtypes_h::{pthread_cond_t, pthread_mutex_t, pthread_t,
                                pthread_attr_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "1011:1"]
        pub fn pthread_cond_wait(__cond: *mut pthread_cond_t,
                                 __mutex: *mut pthread_mutex_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1003:1"]
        pub fn pthread_cond_broadcast(__cond: *mut pthread_cond_t)
         -> libc::c_int;
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
    }
}
#[c2rust::header_src = "/usr/include/sys/stat.h:58"]
pub mod sys_stat_h {
    use super::stat_h::stat;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "210:1"]
        pub fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_generic.h:63"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/tests/gss-threads/gss-misc.h:64"]
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
        pub fn display_status(msg_0: *mut libc::c_char, maj_stat: OM_uint32,
                              min_stat_0: OM_uint32);
        #[no_mangle]
        #[c2rust::src_loc = "34:1"]
        pub fn display_ctx_flags(flags: OM_uint32);
        #[no_mangle]
        #[c2rust::src_loc = "49:21"]
        pub static mut empty_token: gss_buffer_t;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__u_short, __uint8_t, __uint16_t, __uint32_t, __dev_t,
                        __uid_t, __gid_t, __ino_t, __mode_t, __nlink_t,
                        __off_t, __off64_t, __time_t, __blksize_t, __blkcnt_t,
                        __ssize_t, __syscall_slong_t, __socklen_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::sys_types_h::{u_short, ssize_t};
pub use self::struct_timespec_h::timespec;
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s,
                                      __pthread_cond_s, C2RustUnnamed,
                                      C2RustUnnamed_0, C2RustUnnamed_1,
                                      C2RustUnnamed_2};
pub use self::pthreadtypes_h::{pthread_t, pthread_attr_t, pthread_mutex_t,
                               pthread_cond_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stat_h::stat;
pub use self::unistd_h::{socklen_t, read, close, sleep};
pub use self::ctype_h::{C2RustUnnamed_3, _ISalnum, _ISpunct, _IScntrl,
                        _ISblank, _ISgraph, _ISprint, _ISspace, _ISxdigit,
                        _ISdigit, _ISalpha, _ISlower, _ISupper,
                        __ctype_b_loc};
pub use self::socket_type_h::{__socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC,
                              SOCK_PACKET, SOCK_DCCP, SOCK_SEQPACKET,
                              SOCK_RDM, SOCK_RAW, SOCK_DGRAM, SOCK_STREAM};
pub use self::sockaddr_h::sa_family_t;
pub use self::socket_h::sockaddr;
pub use self::sys_socket_h::{__CONST_SOCKADDR_ARG, sockaddr_x25, sockaddr_un,
                             sockaddr_ns, sockaddr_iso, sockaddr_ipx,
                             sockaddr_inarp, sockaddr_eon, sockaddr_dl,
                             sockaddr_ax25, sockaddr_at, socket, connect};
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed_4, in_port_t,
                     sockaddr_in, in_addr, in_addr_t, htons};
pub use self::netdb_h::{hostent, gethostbyname};
pub use self::gssapi_h::{gss_name_t, gss_cred_id_t, gss_ctx_id_t, gss_uint32,
                         OM_uint32, gss_OID_desc_struct, gss_OID,
                         gss_OID_set_desc_struct, gss_OID_set,
                         gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_channel_bindings_struct,
                         gss_channel_bindings_t, gss_qop_t, gss_name_struct,
                         gss_cred_id_struct, gss_ctx_id_struct,
                         gss_init_sec_context, gss_delete_sec_context,
                         gss_verify_mic, gss_wrap, gss_display_name,
                         gss_import_name, gss_release_name,
                         gss_release_buffer, gss_release_oid_set,
                         gss_inquire_context, gss_release_oid, gss_str_to_oid,
                         gss_oid_to_str, gss_inquire_names_for_mech};
use self::string_h::{memcpy, strcmp, strlen};
use self::stdlib_h::{atoi, malloc, free, exit};
use self::stdio_h::{stdout, stderr, fprintf, printf, asprintf, perror};
use self::fcntl_h::open;
use self::pthread_h::{pthread_cond_wait, pthread_cond_broadcast,
                      pthread_create, pthread_detach, pthread_mutex_lock,
                      pthread_mutex_unlock};
use self::sys_stat_h::fstat;
use self::gssapi_generic_h::gss_nt_service_name;
use self::gss_misc_h::{display_file, send_token, recv_token, display_status,
                       display_ctx_flags, empty_token};
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
 * Copyright (C) 2003, 2004, 2008 by the Massachusetts Institute of Technology.
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
#[c2rust::src_loc = "68:12"]
static mut verbose: libc::c_int = 1 as libc::c_int;
#[c2rust::src_loc = "70:1"]
unsafe extern "C" fn usage() {
    fprintf(stderr,
            b"Usage: gss-client [-port port] [-mech mechanism] [-d]\n\x00" as
                *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"       [-seq] [-noreplay] [-nomutual]\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr,
            b" [-threads num]\x00" as *const u8 as *const libc::c_char);
    fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"       [-f] [-q] [-ccount count] [-mcount count]\n\x00" as
                *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"       [-v1] [-na] [-nw] [-nx] [-nm] host service msg\n\x00" as
                *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
}
/*
 * Function: get_server_info
 *
 * Purpose: Sets up a socket address for the named host and port.
 *
 * Arguments:
 *
 *      host            (r) the target host name
 *      port            (r) the target port, in host byte order
 *
 * Returns: 0 on success, or -1 on failure
 *
 * Effects:
 *
 * The host name is resolved with gethostbyname(), and "saddr" is set
 * to the desired socket address.  If an error occurs, an error
 * message is displayed and -1 is returned.
 */
#[no_mangle]
#[c2rust::src_loc = "100:20"]
pub static mut saddr: sockaddr_in =
    sockaddr_in{sin_family: 0,
                sin_port: 0,
                sin_addr: in_addr{s_addr: 0,},
                sin_zero: [0; 8],};
#[c2rust::src_loc = "101:1"]
unsafe extern "C" fn get_server_info(mut host: *mut libc::c_char,
                                     mut port_0: u_short) -> libc::c_int {
    let mut hp: *mut hostent = 0 as *mut hostent;
    hp = gethostbyname(host);
    if hp.is_null() {
        fprintf(stderr,
                b"Unknown host: %s\n\x00" as *const u8 as *const libc::c_char,
                host);
        return -(1 as libc::c_int)
    }
    saddr.sin_family = (*hp).h_addrtype as sa_family_t;
    memcpy(&mut saddr.sin_addr as *mut in_addr as *mut libc::c_void,
           *(*hp).h_addr_list.offset(0 as libc::c_int as isize) as
               *const libc::c_void,
           ::std::mem::size_of::<in_addr>() as libc::c_ulong);
    saddr.sin_port = htons(port_0);
    return 0 as libc::c_int;
}
/*
 * Function: connect_to_server
 *
 * Purpose: Opens a TCP connection to the name host and port.
 *
 * Arguments:
 *
 *      host            (r) the target host name
 *      port            (r) the target port, in host byte order
 *
 * Returns: the established socket file desciptor, or -1 on failure
 *
 * Effects:
 *
 * The host name is resolved with gethostbyname(), and the socket is
 * opened and connected.  If an error occurs, an error message is
 * displayed and -1 is returned.
 */
#[c2rust::src_loc = "136:1"]
unsafe extern "C" fn connect_to_server() -> libc::c_int {
    let mut s: libc::c_int = 0;
    s =
        socket(2 as libc::c_int, SOCK_STREAM as libc::c_int,
               0 as libc::c_int);
    if s < 0 as libc::c_int {
        perror(b"creating socket\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    if connect(s,
               __CONST_SOCKADDR_ARG{__sockaddr__:
                                        &mut saddr as *mut sockaddr_in as
                                            *mut sockaddr,},
               ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                   socklen_t) < 0 as libc::c_int {
        perror(b"connecting to server\x00" as *const u8 as
                   *const libc::c_char);
        close(s);
        return -(1 as libc::c_int)
    }
    return s;
}
/*
 * Function: client_establish_context
 *
 * Purpose: establishes a GSS-API context with a specified service and
 * returns the context handle
 *
 * Arguments:
 *
 *      s               (r) an established TCP connection to the service
 *      service_name    (r) the ASCII service name of the service
 *      gss_flags       (r) GSS-API delegation flag (if any)
 *      auth_flag       (r) whether to actually do authentication
 *      v1_format       (r) whether the v1 sample protocol should be used
 *      oid             (r) OID of the mechanism to use
 *      context         (w) the established GSS-API context
 *      ret_flags       (w) the returned flags from init_sec_context
 *
 * Returns: 0 on success, -1 on failure
 *
 * Effects:
 *
 * service_name is imported as a GSS-API name and a GSS-API context is
 * established with the corresponding service; the service should be
 * listening on the TCP connection s.  The default GSS-API mechanism
 * is used, and mutual authentication and replay detection are
 * requested.
 *
 * If successful, the context handle is returned in context.  If
 * unsuccessful, the GSS-API error messages are displayed on stderr
 * and -1 is returned.
 */
#[c2rust::src_loc = "185:1"]
unsafe extern "C" fn client_establish_context(mut s: libc::c_int,
                                              mut service_name_0:
                                                  *mut libc::c_char,
                                              mut gss_flags_0: OM_uint32,
                                              mut auth_flag_0: libc::c_int,
                                              mut v1_format_0: libc::c_int,
                                              mut oid_0: gss_OID,
                                              mut gss_context:
                                                  *mut gss_ctx_id_t,
                                              mut ret_flags: *mut OM_uint32)
 -> libc::c_int {
    if auth_flag_0 != 0 {
        let mut send_tok: gss_buffer_desc =
            gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
        let mut recv_tok: gss_buffer_desc =
            gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
        let mut token_ptr: *mut gss_buffer_desc = 0 as *mut gss_buffer_desc;
        let mut target_name: gss_name_t = 0 as *mut gss_name_struct;
        let mut maj_stat: OM_uint32 = 0;
        let mut min_stat_0: OM_uint32 = 0;
        let mut init_sec_min_stat: OM_uint32 = 0;
        let mut token_flags: libc::c_int = 0;
        /*
         * Import the name into target_name.  Use send_tok to save
         * local variable space.
         */
        send_tok.value = service_name_0 as *mut libc::c_void;
        send_tok.length = strlen(service_name_0);
        maj_stat =
            gss_import_name(&mut min_stat_0, &mut send_tok,
                            gss_nt_service_name, &mut target_name);
        if maj_stat != 0 as libc::c_int as libc::c_uint {
            display_status(b"parsing name\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,
                           maj_stat, min_stat_0);
            return -(1 as libc::c_int)
        }
        if v1_format_0 == 0 {
            if send_token(s,
                          (1 as libc::c_int) << 0 as libc::c_int |
                              (1 as libc::c_int) << 4 as libc::c_int,
                          empty_token) < 0 as libc::c_int {
                gss_release_name(&mut min_stat_0, &mut target_name);
                return -(1 as libc::c_int)
            }
        }
        /*
         * Perform the context-establishement loop.
         *
         * On each pass through the loop, token_ptr points to the token
         * to send to the server (or GSS_C_NO_BUFFER on the first pass).
         * Every generated token is stored in send_tok which is then
         * transmitted to the server; every received token is stored in
         * recv_tok, which token_ptr is then set to, to be processed by
         * the next call to gss_init_sec_context.
         *
         * GSS-API guarantees that send_tok's length will be non-zero
         * if and only if the server is expecting another token from us,
         * and that gss_init_sec_context returns GSS_S_CONTINUE_NEEDED if
         * and only if the server has another token to send us.
         */
        token_ptr = 0 as gss_buffer_t;
        *gss_context = 0 as gss_ctx_id_t;
        loop  {
            maj_stat =
                gss_init_sec_context(&mut init_sec_min_stat,
                                     0 as gss_cred_id_t, gss_context,
                                     target_name, oid_0, gss_flags_0,
                                     0 as libc::c_int as OM_uint32,
                                     0 as gss_channel_bindings_t, token_ptr,
                                     0 as *mut gss_OID, &mut send_tok,
                                     ret_flags, 0 as *mut OM_uint32);
            if !token_ptr.is_null() { free(recv_tok.value); }
            if send_tok.length != 0 as libc::c_int as libc::c_ulong {
                if verbose != 0 {
                    printf(b"Sending init_sec_context token (size=%d)...\x00"
                               as *const u8 as *const libc::c_char,
                           send_tok.length as libc::c_int);
                }
                if send_token(s,
                              (if v1_format_0 != 0 {
                                   0 as libc::c_int
                               } else {
                                   ((1 as libc::c_int)) << 1 as libc::c_int
                               }), &mut send_tok) < 0 as libc::c_int {
                    gss_release_buffer(&mut min_stat_0, &mut send_tok);
                    gss_release_name(&mut min_stat_0, &mut target_name);
                    if !(*gss_context).is_null() {
                        gss_delete_sec_context(&mut min_stat_0, gss_context,
                                               0 as gss_buffer_t);
                        *gss_context = 0 as gss_ctx_id_t
                    }
                    return -(1 as libc::c_int)
                }
            }
            gss_release_buffer(&mut min_stat_0, &mut send_tok);
            if maj_stat != 0 as libc::c_int as libc::c_uint &&
                   maj_stat !=
                       ((1 as libc::c_int) <<
                            0 as libc::c_int + 0 as libc::c_int) as
                           libc::c_uint {
                display_status(b"initializing context\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                               maj_stat, init_sec_min_stat);
                gss_release_name(&mut min_stat_0, &mut target_name);
                if !(*gss_context).is_null() {
                    gss_delete_sec_context(&mut min_stat_0, gss_context,
                                           0 as gss_buffer_t);
                }
                return -(1 as libc::c_int)
            }
            if maj_stat ==
                   ((1 as libc::c_int) << 0 as libc::c_int + 0 as libc::c_int)
                       as libc::c_uint {
                if verbose != 0 {
                    printf(b"continue needed...\x00" as *const u8 as
                               *const libc::c_char);
                }
                if recv_token(s, &mut token_flags, &mut recv_tok) <
                       0 as libc::c_int {
                    gss_release_name(&mut min_stat_0, &mut target_name);
                    return -(1 as libc::c_int)
                }
                token_ptr = &mut recv_tok
            }
            if verbose != 0 {
                printf(b"\n\x00" as *const u8 as *const libc::c_char);
            }
            if !(maj_stat ==
                     ((1 as libc::c_int) <<
                          0 as libc::c_int + 0 as libc::c_int) as
                         libc::c_uint) {
                break ;
            }
        }
        gss_release_name(&mut min_stat_0, &mut target_name);
    } else if send_token(s, (1 as libc::c_int) << 0 as libc::c_int,
                         empty_token) < 0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "298:1"]
unsafe extern "C" fn read_file(mut file_name: *mut libc::c_char,
                               mut in_buf: gss_buffer_t) {
    let mut fd: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut stat_buf: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    fd = open(file_name, 0 as libc::c_int, 0 as libc::c_int);
    if fd < 0 as libc::c_int {
        perror(b"open\x00" as *const u8 as *const libc::c_char);
        fprintf(stderr,
                b"Couldn\'t open file %s\n\x00" as *const u8 as
                    *const libc::c_char, file_name);
        exit(2 as libc::c_int);
    }
    if fstat(fd, &mut stat_buf) < 0 as libc::c_int {
        perror(b"fstat\x00" as *const u8 as *const libc::c_char);
        exit(3 as libc::c_int);
    }
    (*in_buf).length = stat_buf.st_size as size_t;
    if (*in_buf).length == 0 as libc::c_int as libc::c_ulong {
        (*in_buf).value = 0 as *mut libc::c_void;
        return
    }
    (*in_buf).value = malloc((*in_buf).length);
    if (*in_buf).value.is_null() {
        fprintf(stderr,
                b"Couldn\'t allocate %d byte buffer for reading file\n\x00" as
                    *const u8 as *const libc::c_char,
                (*in_buf).length as libc::c_int);
        exit(4 as libc::c_int);
    }
    /* This code used to check for incomplete reads, but you can't get
     * an incomplete read on any file for which fstat() is meaningful. */
    count = read(fd, (*in_buf).value, (*in_buf).length) as libc::c_int;
    if count < 0 as libc::c_int {
        perror(b"read\x00" as *const u8 as *const libc::c_char);
        exit(5 as libc::c_int);
    }
    if (count as size_t) < (*in_buf).length {
        fprintf(stderr,
                b"Warning, only read in %d bytes, expected %d\n\x00" as
                    *const u8 as *const libc::c_char, count,
                (*in_buf).length as libc::c_int);
    };
}
/*
 * Function: call_server
 *
 * Purpose: Call the "sign" service.
 *
 * Arguments:
 *
 *      host            (r) the host providing the service
 *      port            (r) the port to connect to on host
 *      service_name    (r) the GSS-API service name to authenticate to
 *      gss_flags       (r) GSS-API delegation flag (if any)
 *      auth_flag       (r) whether to do authentication
 *      wrap_flag       (r) whether to do message wrapping at all
 *      encrypt_flag    (r) whether to do encryption while wrapping
 *      mic_flag        (r) whether to request a MIC from the server
 *      msg             (r) the message to have "signed"
 *      use_file        (r) whether to treat msg as an input file name
 *      mcount          (r) the number of times to send the message
 *
 * Returns: 0 on success, -1 on failure
 *
 * Effects:
 *
 * call_server opens a TCP connection to <host:port> and establishes a
 * GSS-API context with service_name over the connection.  It then
 * seals msg in a GSS-API token with gss_wrap, sends it to the server,
 * reads back a GSS-API signature block for msg from the server, and
 * verifies it with gss_verify.  -1 is returned if any step fails,
 * otherwise 0 is returned.
 */
#[c2rust::src_loc = "372:1"]
unsafe extern "C" fn call_server(mut host: *mut libc::c_char,
                                 mut port_0: u_short, mut oid_0: gss_OID,
                                 mut service_name_0: *mut libc::c_char,
                                 mut gss_flags_0: OM_uint32,
                                 mut auth_flag_0: libc::c_int,
                                 mut wrap_flag_0: libc::c_int,
                                 mut encrypt_flag_0: libc::c_int,
                                 mut mic_flag_0: libc::c_int,
                                 mut v1_format_0: libc::c_int,
                                 mut msg_0: *mut libc::c_char,
                                 mut use_file_0: libc::c_int,
                                 mut mcount_0: size_t) -> libc::c_int {
    let mut context: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut in_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut out_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut sname: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut tname: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut oid_name: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut s: libc::c_int = 0;
    let mut state: libc::c_int = 0;
    let mut is_local: libc::c_int = 0;
    let mut is_open: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut token_flags: libc::c_int = 0;
    let mut ret_flags: OM_uint32 = 0;
    let mut maj_stat: OM_uint32 = 0;
    let mut min_stat_0: OM_uint32 = 0;
    let mut lifetime: OM_uint32 = 0;
    let mut context_flags: OM_uint32 = 0;
    let mut src_name: gss_name_t = 0 as *mut gss_name_struct;
    let mut targ_name: gss_name_t = 0 as *mut gss_name_struct;
    let mut mechanism_0: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut name_type: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut qop_state: gss_qop_t = 0;
    let mut mech_names: gss_OID_set = 0 as *mut gss_OID_set_desc_struct;
    let mut i: size_t = 0;
    /* Open connection. */
    s = connect_to_server();
    if s < 0 as libc::c_int { return -(1 as libc::c_int) }
    /* Establish context. */
    if client_establish_context(s, service_name_0, gss_flags_0, auth_flag_0,
                                v1_format_0, oid_0, &mut context,
                                &mut ret_flags) < 0 as libc::c_int {
        close(s);
        return -(1 as libc::c_int)
    }
    if auth_flag_0 != 0 && verbose != 0 {
        /* Display the flags. */
        display_ctx_flags(ret_flags);
        /* Get context information. */
        maj_stat =
            gss_inquire_context(&mut min_stat_0, context, &mut src_name,
                                &mut targ_name, &mut lifetime,
                                &mut mechanism_0, &mut context_flags,
                                &mut is_local, &mut is_open);
        if maj_stat != 0 as libc::c_int as libc::c_uint {
            display_status(b"inquiring context\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,
                           maj_stat, min_stat_0);
            return -(1 as libc::c_int)
        }
        maj_stat =
            gss_display_name(&mut min_stat_0, src_name, &mut sname,
                             &mut name_type);
        if maj_stat != 0 as libc::c_int as libc::c_uint {
            display_status(b"displaying source name\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,
                           maj_stat, min_stat_0);
            return -(1 as libc::c_int)
        }
        maj_stat =
            gss_display_name(&mut min_stat_0, targ_name, &mut tname,
                             0 as *mut gss_OID);
        if maj_stat != 0 as libc::c_int as libc::c_uint {
            display_status(b"displaying target name\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,
                           maj_stat, min_stat_0);
            return -(1 as libc::c_int)
        }
        printf(b"\"%.*s\" to \"%.*s\", lifetime %d, flags %x, %s, %s\n\x00" as
                   *const u8 as *const libc::c_char,
               sname.length as libc::c_int, sname.value as *mut libc::c_char,
               tname.length as libc::c_int, tname.value as *mut libc::c_char,
               lifetime, context_flags,
               if is_local != 0 {
                   b"locally initiated\x00" as *const u8 as
                       *const libc::c_char
               } else {
                   b"remotely initiated\x00" as *const u8 as
                       *const libc::c_char
               },
               if is_open != 0 {
                   b"open\x00" as *const u8 as *const libc::c_char
               } else { b"closed\x00" as *const u8 as *const libc::c_char });
        gss_release_name(&mut min_stat_0, &mut src_name);
        gss_release_name(&mut min_stat_0, &mut targ_name);
        gss_release_buffer(&mut min_stat_0, &mut sname);
        gss_release_buffer(&mut min_stat_0, &mut tname);
        maj_stat = gss_oid_to_str(&mut min_stat_0, name_type, &mut oid_name);
        if maj_stat != 0 as libc::c_int as libc::c_uint {
            display_status(b"converting oid->string\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,
                           maj_stat, min_stat_0);
            return -(1 as libc::c_int)
        }
        printf(b"Name type of source name is %.*s.\n\x00" as *const u8 as
                   *const libc::c_char, oid_name.length as libc::c_int,
               oid_name.value as *mut libc::c_char);
        gss_release_buffer(&mut min_stat_0, &mut oid_name);
        /* Now get the names supported by the mechanism. */
        maj_stat =
            gss_inquire_names_for_mech(&mut min_stat_0, mechanism_0,
                                       &mut mech_names);
        if maj_stat != 0 as libc::c_int as libc::c_uint {
            display_status(b"inquiring mech names\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,
                           maj_stat, min_stat_0);
            return -(1 as libc::c_int)
        }
        maj_stat =
            gss_oid_to_str(&mut min_stat_0, mechanism_0, &mut oid_name);
        if maj_stat != 0 as libc::c_int as libc::c_uint {
            display_status(b"converting oid->string\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,
                           maj_stat, min_stat_0);
            return -(1 as libc::c_int)
        }
        printf(b"Mechanism %.*s supports %d names\n\x00" as *const u8 as
                   *const libc::c_char, oid_name.length as libc::c_int,
               oid_name.value as *mut libc::c_char,
               (*mech_names).count as libc::c_int);
        gss_release_buffer(&mut min_stat_0, &mut oid_name);
        i = 0 as libc::c_int as size_t;
        while i < (*mech_names).count {
            maj_stat =
                gss_oid_to_str(&mut min_stat_0,
                               &mut *(*mech_names).elements.offset(i as
                                                                       isize),
                               &mut oid_name);
            if maj_stat != 0 as libc::c_int as libc::c_uint {
                display_status(b"converting oid->string\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                               maj_stat, min_stat_0);
                return -(1 as libc::c_int)
            }
            printf(b"  %d: %.*s\n\x00" as *const u8 as *const libc::c_char,
                   i as libc::c_int, oid_name.length as libc::c_int,
                   oid_name.value as *mut libc::c_char);
            gss_release_buffer(&mut min_stat_0, &mut oid_name);
            i = i.wrapping_add(1)
        }
        gss_release_oid_set(&mut min_stat_0, &mut mech_names);
    }
    if use_file_0 != 0 {
        read_file(msg_0, &mut in_buf);
    } else {
        /* Seal the message. */
        in_buf.value = msg_0 as *mut libc::c_void;
        in_buf.length = strlen(msg_0)
    }
    i = 0 as libc::c_int as size_t;
    while i < mcount_0 {
        if wrap_flag_0 != 0 {
            maj_stat =
                gss_wrap(&mut min_stat_0, context, encrypt_flag_0,
                         0 as libc::c_int as gss_qop_t, &mut in_buf,
                         &mut state, &mut out_buf);
            if maj_stat != 0 as libc::c_int as libc::c_uint {
                display_status(b"wrapping message\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                               maj_stat, min_stat_0);
                close(s);
                gss_delete_sec_context(&mut min_stat_0, &mut context,
                                       0 as gss_buffer_t);
                return -(1 as libc::c_int)
            } else {
                if encrypt_flag_0 != 0 && state == 0 {
                    fprintf(stderr,
                            b"Warning!  Message not encrypted.\n\x00" as
                                *const u8 as *const libc::c_char);
                }
            }
        } else { out_buf = in_buf }
        /* Send to server. */
        flags = 0 as libc::c_int;
        if v1_format_0 == 0 {
            flags =
                (1 as libc::c_int) << 2 as libc::c_int |
                    (if wrap_flag_0 != 0 {
                         ((1 as libc::c_int)) << 5 as libc::c_int
                     } else { 0 as libc::c_int }) |
                    (if encrypt_flag_0 != 0 {
                         ((1 as libc::c_int)) << 6 as libc::c_int
                     } else { 0 as libc::c_int }) |
                    (if mic_flag_0 != 0 {
                         ((1 as libc::c_int)) << 7 as libc::c_int
                     } else { 0 as libc::c_int })
        }
        if send_token(s, flags, &mut out_buf) < 0 as libc::c_int {
            close(s);
            gss_delete_sec_context(&mut min_stat_0, &mut context,
                                   0 as gss_buffer_t);
            return -(1 as libc::c_int)
        }
        if out_buf.value != in_buf.value {
            gss_release_buffer(&mut min_stat_0, &mut out_buf);
        }
        /* Read signature block into out_buf. */
        if recv_token(s, &mut token_flags, &mut out_buf) < 0 as libc::c_int {
            close(s);
            gss_delete_sec_context(&mut min_stat_0, &mut context,
                                   0 as gss_buffer_t);
            return -(1 as libc::c_int)
        }
        if mic_flag_0 != 0 {
            /* Verify signature block. */
            maj_stat =
                gss_verify_mic(&mut min_stat_0, context, &mut in_buf,
                               &mut out_buf, &mut qop_state);
            if maj_stat != 0 as libc::c_int as libc::c_uint {
                display_status(b"verifying signature\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                               maj_stat, min_stat_0);
                close(s);
                gss_delete_sec_context(&mut min_stat_0, &mut context,
                                       0 as gss_buffer_t);
                return -(1 as libc::c_int)
            }
            if verbose != 0 {
                printf(b"Signature verified.\n\x00" as *const u8 as
                           *const libc::c_char);
            }
        } else if verbose != 0 {
            printf(b"Response received.\n\x00" as *const u8 as
                       *const libc::c_char);
        }
        free(out_buf.value);
        i = i.wrapping_add(1)
    }
    if use_file_0 != 0 { free(in_buf.value); }
    /* Send NOOP. */
    if v1_format_0 == 0 {
        send_token(s, (1 as libc::c_int) << 0 as libc::c_int, empty_token);
    }
    if auth_flag_0 != 0 {
        /* Delete context. */
        maj_stat =
            gss_delete_sec_context(&mut min_stat_0, &mut context,
                                   &mut out_buf);
        if maj_stat != 0 as libc::c_int as libc::c_uint {
            display_status(b"deleting context\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,
                           maj_stat, min_stat_0);
            close(s);
            gss_delete_sec_context(&mut min_stat_0, &mut context,
                                   0 as gss_buffer_t);
            return -(1 as libc::c_int)
        }
        gss_release_buffer(&mut min_stat_0, &mut out_buf);
    }
    close(s);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "567:1"]
unsafe extern "C" fn parse_oid(mut mechanism_0: *mut libc::c_char,
                               mut oid_0: *mut gss_OID) {
    let mut mechstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tok: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut maj_stat: OM_uint32 = 0;
    let mut min_stat_0: OM_uint32 = 0;
    if *(*__ctype_b_loc()).offset(*mechanism_0.offset(0 as libc::c_int as
                                                          isize) as
                                      libc::c_uchar as libc::c_int as isize)
           as libc::c_int &
           _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        if asprintf(&mut mechstr as *mut *mut libc::c_char,
                    b"{ %s }\x00" as *const u8 as *const libc::c_char,
                    mechanism_0) < 0 as libc::c_int {
            fprintf(stderr,
                    b"Couldn\'t allocate mechanism scratch!\n\x00" as
                        *const u8 as *const libc::c_char);
            return
        }
        cp = mechstr;
        while *cp != 0 {
            if *cp as libc::c_int == '.' as i32 {
                *cp = ' ' as i32 as libc::c_char
            }
            cp = cp.offset(1)
        }
        tok.value = mechstr as *mut libc::c_void
    } else { tok.value = mechanism_0 as *mut libc::c_void }
    tok.length = strlen(tok.value as *const libc::c_char);
    maj_stat = gss_str_to_oid(&mut min_stat_0, &mut tok, oid_0);
    if maj_stat != 0 as libc::c_int as libc::c_uint {
        display_status(b"str_to_oid\x00" as *const u8 as *const libc::c_char
                           as *mut libc::c_char, maj_stat, min_stat_0);
        return
    }
    if !mechstr.is_null() { free(mechstr as *mut libc::c_void); };
}
#[c2rust::src_loc = "597:12"]
static mut max_threads: libc::c_int = 1 as libc::c_int;
/* assume pthread */
#[c2rust::src_loc = "657:24"]
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
#[c2rust::src_loc = "658:23"]
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
#[c2rust::src_loc = "659:5"]
pub static mut counter: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "661:1"]
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
#[c2rust::src_loc = "684:1"]
unsafe extern "C" fn decrement_and_signal_thread_counter() {
    let mut err: libc::c_int = 0;
    sleep(1 as libc::c_int as libc::c_uint);
    err = pthread_mutex_lock(&mut counter_mutex);
    if err != 0 {
        perror(b"pthread_mutex_lock\x00" as *const u8 as *const libc::c_char);
        return
    }
    if counter == max_threads { pthread_cond_broadcast(&mut counter_cond); }
    counter -= 1;
    pthread_mutex_unlock(&mut counter_mutex);
}
#[c2rust::src_loc = "703:14"]
static mut service_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "703:29"]
static mut server_host: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "703:43"]
static mut msg: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "704:14"]
static mut mechanism: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "705:16"]
static mut port: u_short = 4444 as libc::c_int as u_short;
#[c2rust::src_loc = "706:12"]
static mut use_file: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "707:18"]
static mut gss_flags: OM_uint32 =
    (2 as libc::c_int | 4 as libc::c_int) as OM_uint32;
#[c2rust::src_loc = "708:18"]
static mut min_stat: OM_uint32 = 0;
#[c2rust::src_loc = "709:16"]
static mut oid: gss_OID = 0 as *const gss_OID_desc_struct as gss_OID;
#[c2rust::src_loc = "710:12"]
static mut mcount: libc::c_int = 1 as libc::c_int;
#[c2rust::src_loc = "710:24"]
static mut ccount: libc::c_int = 1 as libc::c_int;
#[c2rust::src_loc = "711:12"]
static mut auth_flag: libc::c_int = 0;
#[c2rust::src_loc = "711:23"]
static mut wrap_flag: libc::c_int = 0;
#[c2rust::src_loc = "711:34"]
static mut encrypt_flag: libc::c_int = 0;
#[c2rust::src_loc = "711:48"]
static mut mic_flag: libc::c_int = 0;
#[c2rust::src_loc = "711:58"]
static mut v1_format: libc::c_int = 0;
#[c2rust::src_loc = "713:1"]
unsafe extern "C" fn worker_bee(mut unused: *mut libc::c_void)
 -> *mut libc::c_void {
    printf(b"worker bee!\n\x00" as *const u8 as *const libc::c_char);
    if call_server(server_host, port, oid, service_name, gss_flags, auth_flag,
                   wrap_flag, encrypt_flag, mic_flag, v1_format, msg,
                   use_file, mcount as size_t) < 0 as libc::c_int {
        if max_threads == 1 as libc::c_int { exit(6 as libc::c_int); }
    }
    if max_threads > 1 as libc::c_int {
        decrement_and_signal_thread_counter();
    }
    free(unused);
    return 0 as *mut libc::c_void;
}
#[c2rust::src_loc = "730:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    display_file = stdout;
    mic_flag = 1 as libc::c_int;
    encrypt_flag = mic_flag;
    wrap_flag = encrypt_flag;
    auth_flag = wrap_flag;
    v1_format = 0 as libc::c_int;
    /* Parse arguments. */
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
                         b"-mech\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            argc -= 1;
            argv = argv.offset(1);
            if argc == 0 { usage(); }
            mechanism = *argv
        } else if strcmp(*argv,
                         b"-threads\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            argc -= 1;
            argv = argv.offset(1);
            if argc == 0 { usage(); }
            max_threads = atoi(*argv)
        } else if strcmp(*argv, b"-d\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            gss_flags |= 1 as libc::c_int as libc::c_uint
        } else if strcmp(*argv,
                         b"-seq\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            gss_flags |= 8 as libc::c_int as libc::c_uint
        } else if strcmp(*argv,
                         b"-noreplay\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            gss_flags &= !(4 as libc::c_int) as libc::c_uint
        } else if strcmp(*argv,
                         b"-nomutual\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            gss_flags &= !(2 as libc::c_int) as libc::c_uint
        } else if strcmp(*argv, b"-f\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            use_file = 1 as libc::c_int
        } else if strcmp(*argv, b"-q\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            verbose = 0 as libc::c_int
        } else if strcmp(*argv,
                         b"-ccount\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            argc -= 1;
            argv = argv.offset(1);
            if argc == 0 { usage(); }
            ccount = atoi(*argv);
            if ccount <= 0 as libc::c_int { usage(); }
        } else if strcmp(*argv,
                         b"-mcount\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            argc -= 1;
            argv = argv.offset(1);
            if argc == 0 { usage(); }
            mcount = atoi(*argv);
            if mcount < 0 as libc::c_int { usage(); }
        } else if strcmp(*argv,
                         b"-na\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            mic_flag = 0 as libc::c_int;
            encrypt_flag = mic_flag;
            wrap_flag = encrypt_flag;
            auth_flag = wrap_flag
        } else if strcmp(*argv,
                         b"-nw\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            wrap_flag = 0 as libc::c_int
        } else if strcmp(*argv,
                         b"-nx\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            encrypt_flag = 0 as libc::c_int
        } else if strcmp(*argv,
                         b"-nm\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            mic_flag = 0 as libc::c_int
        } else {
            if !(strcmp(*argv, b"-v1\x00" as *const u8 as *const libc::c_char)
                     == 0 as libc::c_int) {
                break ;
            }
            v1_format = 1 as libc::c_int
        }
        argc -= 1;
        argv = argv.offset(1)
    }
    if argc != 3 as libc::c_int { usage(); }
    let fresh0 = argv;
    argv = argv.offset(1);
    server_host = *fresh0;
    let fresh1 = argv;
    argv = argv.offset(1);
    service_name = *fresh1;
    let fresh2 = argv;
    argv = argv.offset(1);
    msg = *fresh2;
    if !mechanism.is_null() { parse_oid(mechanism, &mut oid); }
    if get_server_info(server_host, port) < 0 as libc::c_int {
        exit(1 as libc::c_int);
    }
    if max_threads == 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < ccount { worker_bee(0 as *mut libc::c_void); i += 1 }
    } else {
        i = 0 as libc::c_int;
        while i < ccount {
            if wait_and_increment_thread_counter() != 0 {
                let mut err: libc::c_int = 0;
                let mut thr: pthread_t = 0;
                err =
                    pthread_create(&mut thr, 0 as *const pthread_attr_t,
                                   Some(worker_bee as
                                            unsafe extern "C" fn(_:
                                                                     *mut libc::c_void)
                                                -> *mut libc::c_void),
                                   malloc(12 as libc::c_int as
                                              libc::c_ulong));
                if err != 0 {
                    perror(b"pthread_create\x00" as *const u8 as
                               *const libc::c_char);
                    exit(7 as libc::c_int);
                }
                pthread_detach(thr);
            } else { exit(8 as libc::c_int); }
            i += 1
        }
    }
    if !oid.is_null() { gss_release_oid(&mut min_stat, &mut oid); }
    if max_threads > 1 as libc::c_int {
        sleep(10 as libc::c_int as libc::c_uint);
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
