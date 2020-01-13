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
#[c2rust::header_src = "/usr/include/stdio.h:47"]
pub mod stdio_h {
    #[c2rust::src_loc = "77:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
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
        #[c2rust::src_loc = "775:1"]
        pub fn perror(__s: *const libc::c_char);
    }
}
#[c2rust::header_src = "/usr/include/sys/types.h:48"]
pub mod sys_types_h {
    #[c2rust::src_loc = "34:1"]
    pub type u_short = __u_short;
    use super::types_h::__u_short;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:48"]
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
#[c2rust::header_src = "/usr/include/unistd.h:55"]
pub mod unistd_h {
    #[c2rust::src_loc = "274:1"]
    pub type socklen_t = __socklen_t;
    use super::types_h::__socklen_t;
    use super::stddef_h::size_t;
    use super::stdio_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "360:1"]
        pub fn read(__fd: libc::c_int, __buf: *mut libc::c_void,
                    __nbytes: size_t) -> ssize_t;
    }
}
#[c2rust::header_src = "/usr/include/ctype.h:56"]
pub mod ctype_h {
    #[c2rust::src_loc = "46:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed = 8;
    #[c2rust::src_loc = "58:3"]
    pub const _ISpunct: C2RustUnnamed = 4;
    #[c2rust::src_loc = "57:3"]
    pub const _IScntrl: C2RustUnnamed = 2;
    #[c2rust::src_loc = "56:3"]
    pub const _ISblank: C2RustUnnamed = 1;
    #[c2rust::src_loc = "55:3"]
    pub const _ISgraph: C2RustUnnamed = 32768;
    #[c2rust::src_loc = "54:3"]
    pub const _ISprint: C2RustUnnamed = 16384;
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed = 8192;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed = 4096;
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed = 2048;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed = 256;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    }
}
#[c2rust::header_src = "/usr/include/bits/socket_type.h:58"]
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
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:58"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/bits/socket.h:58"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:59"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/netinet/in.h:59"]
pub mod in_h {
    #[c2rust::src_loc = "30:1"]
    pub type in_addr_t = uint32_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:8"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
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
    use super::stdint_uintn_h::{uint32_t, uint16_t};
    use super::sockaddr_h::sa_family_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "380:1"]
        pub fn htons(__hostshort: uint16_t) -> uint16_t;
    }
}
#[c2rust::header_src = "/usr/include/netdb.h:60"]
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
#[c2rust::header_src = "/usr/include/bits/stat.h:62"]
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
    pub type gss_OID_desc = gss_OID_desc_struct;
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
    pub type gss_OID_set_desc = gss_OID_set_desc_struct;
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
        /* cred_usage_stored */
        #[no_mangle]
        #[c2rust::src_loc = "816:1"]
        pub fn gss_set_neg_mechs(_: *mut OM_uint32, _: gss_cred_id_t,
                                 _: gss_OID_set) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "437:1"]
        pub fn gss_init_sec_context(_: *mut OM_uint32, _: gss_cred_id_t,
                                    _: *mut gss_ctx_id_t, _: gss_name_t,
                                    _: gss_OID, _: OM_uint32, _: OM_uint32,
                                    _: gss_channel_bindings_t,
                                    _: gss_buffer_t, _: *mut gss_OID,
                                    _: gss_buffer_t, _: *mut OM_uint32,
                                    _: *mut OM_uint32) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "432:1"]
        pub fn gss_release_cred(_: *mut OM_uint32, _: *mut gss_cred_id_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "421:1"]
        pub fn gss_acquire_cred(_: *mut OM_uint32, _: gss_name_t,
                                _: OM_uint32, _: gss_OID_set,
                                _: gss_cred_usage_t, _: *mut gss_cred_id_t,
                                _: *mut gss_OID_set, _: *mut OM_uint32)
         -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src = "/usr/include/stdlib.h:48"]
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
#[c2rust::header_src = "/usr/include/string.h:49"]
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
#[c2rust::header_src = "/usr/include/assert.h:54"]
pub mod assert_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn __assert_fail(__assertion: *const libc::c_char,
                             __file: *const libc::c_char,
                             __line: libc::c_uint,
                             __function: *const libc::c_char) -> !;
    }
}
#[c2rust::header_src = "/usr/include/sys/socket.h:58"]
pub mod sys_socket_h {
    use super::socket_h::sockaddr;
    use super::unistd_h::socklen_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "102:1"]
        pub fn socket(__domain: libc::c_int, __type: libc::c_int,
                      __protocol: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "126:1"]
        pub fn connect(__fd: libc::c_int, __addr: *const sockaddr,
                       __len: socklen_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/sys/stat.h:62"]
pub mod sys_stat_h {
    use super::stat_h::stat;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "210:1"]
        pub fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:63"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "195:1"]
        pub fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_generic.h:66"]
pub mod gssapi_generic_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "50:27"]
        pub static mut gss_nt_user_name: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "54:27"]
        pub static mut gss_nt_service_name: gss_OID;
    }
    /* _GSSAPI_GENERIC_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_ext.h:67"]
pub mod gssapi_ext_h {
    use super::gssapi_h::{OM_uint32, gss_name_struct, gss_name_t,
                          gss_buffer_desc_struct, gss_buffer_t,
                          gss_OID_set_desc_struct, gss_OID_set,
                          gss_cred_usage_t, gss_cred_id_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "104:1"]
        pub fn gss_acquire_cred_with_password(_: *mut OM_uint32,
                                              _: gss_name_t, _: gss_buffer_t,
                                              _: OM_uint32, _: gss_OID_set,
                                              _: gss_cred_usage_t,
                                              _: *mut gss_cred_id_t,
                                              _: *mut gss_OID_set,
                                              _: *mut OM_uint32) -> OM_uint32;
    }
    /* GSSAPI_EXT_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/appl/gss-sample/gss-misc.h:69"]
pub mod gss_misc_h {
    use super::FILE_h::FILE;
    use super::gssapi_h::{gss_buffer_desc_struct, gss_buffer_t, OM_uint32};
    extern "C" {
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
 * $Id$
 */
        #[no_mangle]
        #[c2rust::src_loc = "34:14"]
        pub static mut display_file: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "36:1"]
        pub fn send_token(s: libc::c_int, flags: libc::c_int,
                          tok: gss_buffer_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn recv_token(s: libc::c_int, flags: *mut libc::c_int,
                          tok: gss_buffer_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "38:1"]
        pub fn display_status(msg_0: *mut libc::c_char, maj_stat: OM_uint32,
                              min_stat_0: OM_uint32);
        #[no_mangle]
        #[c2rust::src_loc = "39:1"]
        pub fn display_ctx_flags(flags: OM_uint32);
        #[no_mangle]
        #[c2rust::src_loc = "54:21"]
        pub static mut empty_token: gss_buffer_t;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__u_short, __uint16_t, __uint32_t, __dev_t, __uid_t,
                        __gid_t, __ino_t, __mode_t, __nlink_t, __off_t,
                        __off64_t, __time_t, __blksize_t, __blkcnt_t,
                        __ssize_t, __syscall_slong_t, __socklen_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stdio_h::{ssize_t, stdout, stderr, fprintf, printf, perror};
pub use self::sys_types_h::u_short;
pub use self::struct_timespec_h::timespec;
pub use self::unistd_h::{socklen_t, close, read};
pub use self::ctype_h::{C2RustUnnamed, _ISalnum, _ISpunct, _IScntrl, _ISblank,
                        _ISgraph, _ISprint, _ISspace, _ISxdigit, _ISdigit,
                        _ISalpha, _ISlower, _ISupper, __ctype_b_loc};
pub use self::socket_type_h::{__socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC,
                              SOCK_PACKET, SOCK_DCCP, SOCK_SEQPACKET,
                              SOCK_RDM, SOCK_RAW, SOCK_DGRAM, SOCK_STREAM};
pub use self::sockaddr_h::sa_family_t;
pub use self::socket_h::sockaddr;
pub use self::stdint_uintn_h::{uint16_t, uint32_t};
pub use self::in_h::{in_addr_t, in_addr, in_port_t, sockaddr_in, htons};
pub use self::netdb_h::{hostent, gethostbyname};
pub use self::stat_h::stat;
pub use self::gssapi_h::{gss_name_t, gss_cred_id_t, gss_ctx_id_t, gss_uint32,
                         OM_uint32, gss_OID_desc_struct, gss_OID_desc,
                         gss_OID, gss_OID_set_desc_struct, gss_OID_set_desc,
                         gss_OID_set, gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_channel_bindings_struct,
                         gss_channel_bindings_t, gss_qop_t, gss_cred_usage_t,
                         gss_name_struct, gss_cred_id_struct,
                         gss_ctx_id_struct, gss_delete_sec_context,
                         gss_verify_mic, gss_wrap, gss_display_name,
                         gss_import_name, gss_release_name,
                         gss_release_buffer, gss_release_oid_set,
                         gss_inquire_context, gss_release_oid, gss_str_to_oid,
                         gss_oid_to_str, gss_inquire_names_for_mech,
                         gss_set_neg_mechs, gss_init_sec_context,
                         gss_release_cred, gss_acquire_cred};
use self::stdlib_h::{atoi, malloc, free, exit};
use self::string_h::{memcpy, strcmp, strlen};
use self::assert_h::__assert_fail;
use self::sys_socket_h::{socket, connect};
use self::sys_stat_h::fstat;
use self::fcntl_h::open;
use self::gssapi_generic_h::{gss_nt_user_name, gss_nt_service_name};
use self::gssapi_ext_h::gss_acquire_cred_with_password;
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
 * Copyright (C) 2003, 2004, 2005 by the Massachusetts Institute of Technology.
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
#[c2rust::src_loc = "72:12"]
static mut verbose: libc::c_int = 1 as libc::c_int;
#[c2rust::src_loc = "73:12"]
static mut spnego: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "74:21"]
static mut gss_spnego_mechanism_oid_desc: gss_OID_desc =
    {
        let mut init =
            gss_OID_desc_struct{length: 6 as libc::c_int as OM_uint32,
                                elements:
                                    b"+\x06\x01\x05\x05\x02\x00" as *const u8
                                        as *const libc::c_char as
                                        *mut libc::c_void,};
        init
    };
#[c2rust::src_loc = "77:1"]
unsafe extern "C" fn usage() {
    fprintf(stderr,
            b"Usage: gss-client [-port port] [-mech mechanism] [-spnego] [-d]\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"       [-seq] [-noreplay] [-nomutual] [-user user] [-pass pw]\x00"
                as *const u8 as *const libc::c_char);
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
#[c2rust::src_loc = "111:1"]
unsafe extern "C" fn connect_to_server(mut host: *mut libc::c_char,
                                       mut port_0: u_short) -> libc::c_int {
    let mut saddr: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut hp: *mut hostent = 0 as *mut hostent;
    let mut s: libc::c_int = 0;
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
    s =
        socket(2 as libc::c_int, SOCK_STREAM as libc::c_int,
               0 as libc::c_int);
    if s < 0 as libc::c_int {
        perror(b"creating socket\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    if connect(s, &mut saddr as *mut sockaddr_in as *mut sockaddr,
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
 *      s                   (r) an established TCP connection to the service
 *      service_name(r) the ASCII service name of the service
 *      gss_flags       (r) GSS-API delegation flag (if any)
 *      auth_flag       (r) whether to actually do authentication
 *  v1_format   (r) whether the v1 sample protocol should be used
 *      oid                 (r) OID of the mechanism to use
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
#[c2rust::src_loc = "179:1"]
unsafe extern "C" fn client_establish_context(mut s: libc::c_int,
                                              mut service_name_0:
                                                  *mut libc::c_char,
                                              mut gss_flags_0: OM_uint32,
                                              mut auth_flag_0: libc::c_int,
                                              mut v1_format_0: libc::c_int,
                                              mut oid_0: gss_OID,
                                              mut username_0:
                                                  *mut libc::c_char,
                                              mut password_0:
                                                  *mut libc::c_char,
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
        let mut cred: gss_cred_id_t = 0 as gss_cred_id_t;
        let mut gss_username: gss_name_t = 0 as gss_name_t;
        let mut mechs: gss_OID_set_desc =
            gss_OID_set_desc{count: 0,
                             elements: 0 as *mut gss_OID_desc_struct,};
        let mut mechsp: *mut gss_OID_set_desc = 0 as gss_OID_set;
        if spnego != 0 {
            mechs.elements = &mut gss_spnego_mechanism_oid_desc;
            mechs.count = 1 as libc::c_int as size_t;
            mechsp = &mut mechs
        } else if !oid_0.is_null() {
            mechs.elements = oid_0;
            mechs.count = 1 as libc::c_int as size_t;
            mechsp = &mut mechs
        } else {
            mechs.elements = 0 as gss_OID;
            mechs.count = 0 as libc::c_int as size_t
        }
        if !username_0.is_null() {
            send_tok.value = username_0 as *mut libc::c_void;
            send_tok.length = strlen(username_0);
            maj_stat =
                gss_import_name(&mut min_stat_0, &mut send_tok,
                                gss_nt_user_name, &mut gss_username);
            if maj_stat != 0 as libc::c_int as libc::c_uint {
                display_status(b"parsing client name\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                               maj_stat, min_stat_0);
                return -(1 as libc::c_int)
            }
        }
        if !password_0.is_null() {
            let mut pwbuf: gss_buffer_desc =
                gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
            pwbuf.value = password_0 as *mut libc::c_void;
            pwbuf.length = strlen(password_0);
            maj_stat =
                gss_acquire_cred_with_password(&mut min_stat_0, gss_username,
                                               &mut pwbuf,
                                               0 as libc::c_int as OM_uint32,
                                               mechsp, 1 as libc::c_int,
                                               &mut cred,
                                               0 as *mut gss_OID_set,
                                               0 as *mut OM_uint32)
        } else if !gss_username.is_null() {
            maj_stat =
                gss_acquire_cred(&mut min_stat_0, gss_username,
                                 0 as libc::c_int as OM_uint32, mechsp,
                                 1 as libc::c_int, &mut cred,
                                 0 as *mut gss_OID_set, 0 as *mut OM_uint32)
        } else { maj_stat = 0 as libc::c_int as OM_uint32 }
        if maj_stat != 0 as libc::c_int as libc::c_uint {
            display_status(b"acquiring creds\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,
                           maj_stat, min_stat_0);
            gss_release_name(&mut min_stat_0, &mut gss_username);
            return -(1 as libc::c_int)
        }
        if spnego != 0 && !oid_0.is_null() {
            let mut neg_mechs: gss_OID_set_desc =
                gss_OID_set_desc{count: 0,
                                 elements: 0 as *mut gss_OID_desc_struct,};
            neg_mechs.elements = oid_0;
            neg_mechs.count = 1 as libc::c_int as size_t;
            maj_stat =
                gss_set_neg_mechs(&mut min_stat_0, cred, &mut neg_mechs);
            if maj_stat != 0 as libc::c_int as libc::c_uint {
                display_status(b"setting neg mechs\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                               maj_stat, min_stat_0);
                gss_release_name(&mut min_stat_0, &mut gss_username);
                gss_release_cred(&mut min_stat_0, &mut cred);
                return -(1 as libc::c_int)
            }
        }
        gss_release_name(&mut min_stat_0, &mut gss_username);
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
        token_ptr = 0 as gss_buffer_t; /* time_rec */
        *gss_context = 0 as gss_ctx_id_t;
        loop  {
            maj_stat =
                gss_init_sec_context(&mut init_sec_min_stat, cred,
                                     gss_context, target_name, mechs.elements,
                                     gss_flags_0,
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
                gss_release_cred(&mut min_stat_0, &mut cred);
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
        gss_release_cred(&mut min_stat_0, &mut cred);
        gss_release_name(&mut min_stat_0, &mut target_name);
    } else if send_token(s, (1 as libc::c_int) << 0 as libc::c_int,
                         empty_token) < 0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "361:1"]
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
        exit(1 as libc::c_int);
    }
    if fstat(fd, &mut stat_buf) < 0 as libc::c_int {
        perror(b"fstat\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
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
        exit(1 as libc::c_int);
    }
    /* this code used to check for incomplete reads, but you can't get
     * an incomplete read on any file for which fstat() is meaningful */
    count = read(fd, (*in_buf).value, (*in_buf).length) as libc::c_int;
    if count < 0 as libc::c_int {
        perror(b"read\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if count < (*in_buf).length as libc::c_int {
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
 * otherwise 0 is returned.  */
#[c2rust::src_loc = "433:1"]
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
                                 mut mcount_0: libc::c_int,
                                 mut username_0: *mut libc::c_char,
                                 mut password_0: *mut libc::c_char)
 -> libc::c_int {
    let mut context: gss_ctx_id_t = 0 as gss_ctx_id_t;
    let mut in_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut out_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut s: libc::c_int = 0;
    let mut state: libc::c_int = 0;
    let mut ret_flags: OM_uint32 = 0;
    let mut maj_stat: OM_uint32 = 0;
    let mut min_stat_0: OM_uint32 = 0;
    let mut src_name: gss_name_t = 0 as *mut gss_name_struct;
    let mut targ_name: gss_name_t = 0 as *mut gss_name_struct;
    let mut sname: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut tname: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut lifetime: OM_uint32 = 0;
    let mut mechanism_0: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut name_type: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut is_local: libc::c_int = 0;
    let mut context_flags: OM_uint32 = 0;
    let mut is_open: libc::c_int = 0;
    let mut qop_state: gss_qop_t = 0;
    let mut mech_names: gss_OID_set = 0 as *mut gss_OID_set_desc_struct;
    let mut oid_name: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut i: size_t = 0;
    let mut token_flags: libc::c_int = 0;
    /* Open connection */
    s = connect_to_server(host, port_0);
    if s < 0 as libc::c_int { return -(1 as libc::c_int) }
    /* Establish context */
    if client_establish_context(s, service_name_0, gss_flags_0, auth_flag_0,
                                v1_format_0, oid_0, username_0, password_0,
                                &mut context, &mut ret_flags) <
           0 as libc::c_int {
        close(s);
        return -(1 as libc::c_int)
    }
    if auth_flag_0 != 0 && verbose != 0 {
        /* display the flags */
        display_ctx_flags(ret_flags);
        /* Get context information */
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
                             0 as *mut libc::c_void as *mut gss_OID);
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
        /* Now get the names supported by the mechanism */
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
        /* Seal the message */
        in_buf.value = msg_0 as *mut libc::c_void;
        in_buf.length = strlen(in_buf.value as *mut libc::c_char)
    }
    i = 0 as libc::c_int as size_t;
    while i < mcount_0 as size_t {
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
        /* Send to server */
        if send_token(s,
                      (if v1_format_0 != 0 {
                           0 as libc::c_int
                       } else {
                           ((1 as libc::c_int) << 2 as libc::c_int |
                                (if wrap_flag_0 != 0 {
                                     ((1 as libc::c_int)) << 5 as libc::c_int
                                 } else { 0 as libc::c_int }) |
                                (if encrypt_flag_0 != 0 {
                                     ((1 as libc::c_int)) << 6 as libc::c_int
                                 } else { 0 as libc::c_int })) |
                               (if mic_flag_0 != 0 {
                                    ((1 as libc::c_int)) << 7 as libc::c_int
                                } else { 0 as libc::c_int })
                       }), &mut out_buf) < 0 as libc::c_int {
            close(s);
            gss_delete_sec_context(&mut min_stat_0, &mut context,
                                   0 as gss_buffer_t);
            return -(1 as libc::c_int)
        }
        if out_buf.value != in_buf.value {
            gss_release_buffer(&mut min_stat_0, &mut out_buf);
        }
        /* Read signature block into out_buf */
        if recv_token(s, &mut token_flags, &mut out_buf) < 0 as libc::c_int {
            close(s);
            gss_delete_sec_context(&mut min_stat_0, &mut context,
                                   0 as gss_buffer_t);
            return -(1 as libc::c_int)
        }
        if mic_flag_0 != 0 {
            /* Verify signature block */
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
    /* Send NOOP */
    if v1_format_0 == 0 {
        send_token(s, (1 as libc::c_int) << 0 as libc::c_int, empty_token);
    }
    if auth_flag_0 != 0 {
        /* Delete context */
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
#[c2rust::src_loc = "656:1"]
unsafe extern "C" fn parse_oid(mut mechanism_0: *mut libc::c_char,
                               mut oid_0: *mut gss_OID) {
    let mut mechstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tok: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut maj_stat: OM_uint32 = 0;
    let mut min_stat_0: OM_uint32 = 0;
    let mut i: size_t = 0;
    let mut mechlen: size_t = strlen(mechanism_0);
    if *(*__ctype_b_loc()).offset(*mechanism_0.offset(0 as libc::c_int as
                                                          isize) as
                                      libc::c_int as isize) as libc::c_int &
           _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        mechstr =
            malloc(mechlen.wrapping_add(5 as libc::c_int as libc::c_ulong)) as
                *mut libc::c_char;
        if mechstr.is_null() {
            fprintf(stderr,
                    b"Couldn\'t allocate mechanism scratch!\n\x00" as
                        *const u8 as *const libc::c_char);
            return
        }
        *mechstr.offset(0 as libc::c_int as isize) =
            '{' as i32 as libc::c_char;
        *mechstr.offset(1 as libc::c_int as isize) =
            ' ' as i32 as libc::c_char;
        i = 0 as libc::c_int as size_t;
        while i < mechlen {
            *mechstr.offset(i.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                as isize) =
                if *mechanism_0.offset(i as isize) as libc::c_int ==
                       '.' as i32 {
                    ' ' as i32
                } else { *mechanism_0.offset(i as isize) as libc::c_int } as
                    libc::c_char;
            i = i.wrapping_add(1)
        }
        *mechstr.offset(mechlen.wrapping_add(2 as libc::c_int as
                                                 libc::c_ulong) as isize) =
            ' ' as i32 as libc::c_char;
        *mechstr.offset(mechlen.wrapping_add(3 as libc::c_int as
                                                 libc::c_ulong) as isize) =
            ' ' as i32 as libc::c_char;
        *mechstr.offset(mechlen.wrapping_add(4 as libc::c_int as
                                                 libc::c_ulong) as isize) =
            '\u{0}' as i32 as libc::c_char;
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
#[c2rust::src_loc = "690:12"]
static mut max_threads: libc::c_int = 1 as libc::c_int;
#[c2rust::src_loc = "750:14"]
static mut service_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "750:29"]
static mut server_host: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "750:43"]
static mut msg: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "751:14"]
static mut mechanism: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "752:16"]
static mut port: u_short = 4444 as libc::c_int as u_short;
#[c2rust::src_loc = "753:12"]
static mut use_file: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "754:18"]
static mut gss_flags: OM_uint32 =
    (2 as libc::c_int | 4 as libc::c_int) as OM_uint32;
#[c2rust::src_loc = "755:18"]
static mut min_stat: OM_uint32 = 0;
#[c2rust::src_loc = "756:16"]
static mut oid: gss_OID = 0 as *const gss_OID_desc_struct as gss_OID;
#[c2rust::src_loc = "757:12"]
static mut mcount: libc::c_int = 1 as libc::c_int;
#[c2rust::src_loc = "757:24"]
static mut ccount: libc::c_int = 1 as libc::c_int;
#[c2rust::src_loc = "758:12"]
static mut auth_flag: libc::c_int = 0;
#[c2rust::src_loc = "758:23"]
static mut wrap_flag: libc::c_int = 0;
#[c2rust::src_loc = "758:34"]
static mut encrypt_flag: libc::c_int = 0;
#[c2rust::src_loc = "758:48"]
static mut mic_flag: libc::c_int = 0;
#[c2rust::src_loc = "758:58"]
static mut v1_format: libc::c_int = 0;
#[c2rust::src_loc = "759:14"]
static mut username: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "760:14"]
static mut password: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "762:1"]
unsafe extern "C" fn worker_bee(mut unused: *mut libc::c_void) {
    if call_server(server_host, port as libc::c_int, oid, service_name,
                   gss_flags, auth_flag, wrap_flag, encrypt_flag, mic_flag,
                   v1_format, msg, use_file, mcount, username, password) <
           0 as libc::c_int {
        exit(1 as libc::c_int);
    };
}
#[c2rust::src_loc = "776:1"]
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
                         b"-user\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            argc -= 1;
            argv = argv.offset(1);
            if argc == 0 { usage(); }
            username = *argv
        } else if strcmp(*argv,
                         b"-pass\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            argc -= 1;
            argv = argv.offset(1);
            if argc == 0 { usage(); }
            password = *argv
        } else if strcmp(*argv,
                         b"-iakerb\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            mechanism =
                b"{ 1 3 6 1 5 2 5 }\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        } else if strcmp(*argv,
                         b"-spnego\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            spnego = 1 as libc::c_int
        } else if strcmp(*argv,
                         b"-krb5\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            mechanism =
                b"{ 1 2 840 113554 1 2 2 }\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        } else if strcmp(*argv,
                         b"-dce\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            gss_flags |= 0x1000 as libc::c_int as libc::c_uint
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
    if max_threads == 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < ccount { worker_bee(0 as *mut libc::c_void); i += 1 }
    } else if max_threads == 1 as libc::c_int {
    } else {
        __assert_fail(b"max_threads == 1\x00" as *const u8 as
                          *const libc::c_char,
                      b"gss-client.c\x00" as *const u8 as *const libc::c_char,
                      909 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"int main()\x00")).as_ptr());
    }
    if !oid.is_null() { gss_release_oid(&mut min_stat, &mut oid); }
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
/* boom */
