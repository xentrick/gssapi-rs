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
#[c2rust::header_src = "/usr/include/sys/types.h:52"]
pub mod sys_types_h {
    #[c2rust::src_loc = "34:1"]
    pub type u_short = __u_short;
    use super::types_h::__u_short;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timeval.h:52"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:52"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/socket.h:52"]
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
#[c2rust::header_src = "/usr/include/bits/socket_type.h:52"]
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
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:52"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/netinet/in.h:52"]
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
#[c2rust::header_src = "/usr/include/sys/time.h:52"]
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
#[c2rust::header_src = "/usr/include/ctype.h:58"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:60"]
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
    #[c2rust::src_loc = "850:1"]
    pub type gss_const_OID = *const gss_OID_desc;
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
        /*
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {10, (void *)"\x2a\x86\x48\x86\xf7\x12"
 *              "\x01\x02\x01\x04"}, corresponding to an
 * object-identifier value of {iso(1) member-body(2)
 * Unites States(840) mit(113554) infosys(1) gssapi(2)
 * generic(1) service_name(4)}.  The constant
 * GSS_C_NT_HOSTBASED_SERVICE should be initialized
 * to point to that gss_OID_desc.
 */
        /*
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {6, (void *)"\x2b\x06\01\x05\x06\x03"},
 * corresponding to an object identifier value of
 * {1(iso), 3(org), 6(dod), 1(internet), 5(security),
 * 6(nametypes), 3(gss-anonymous-name)}.  The constant
 * and GSS_C_NT_ANONYMOUS should be initialized to point
 * to that gss_OID_desc.
 */
        /*
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {6, (void *)"\x2b\x06\x01\x05\x06\x04"},
 * corresponding to an object-identifier value of
 * {1(iso), 3(org), 6(dod), 1(internet), 5(security),
 * 6(nametypes), 4(gss-api-exported-name)}.  The constant
 * GSS_C_NT_EXPORT_NAME should be initialized to point
 * to that gss_OID_desc.
 */
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
        /* context_handle */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "673:1"]
        pub fn gss_release_oid(_: *mut OM_uint32, _: *mut gss_OID)
         -> OM_uint32;
        /* oid */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "707:1"]
        pub fn gss_oid_to_str(_: *mut OM_uint32, _: gss_OID, _: gss_buffer_t)
         -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_ext.h:61"]
pub mod gssapi_ext_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "134:16"]
    pub struct gss_buffer_set_desc_struct {
        pub count: size_t,
        pub elements: *mut gss_buffer_desc,
    }
    #[c2rust::src_loc = "134:1"]
    pub type gss_buffer_set_t = *mut gss_buffer_set_desc_struct;
    use super::stddef_h::size_t;
    use super::gssapi_h::{gss_buffer_desc, OM_uint32, gss_name_struct,
                          gss_name_t, gss_OID, gss_buffer_desc_struct,
                          gss_buffer_t, gss_OID_desc, gss_const_OID};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "444:1"]
        pub fn gss_inquire_name(_: *mut OM_uint32, _: gss_name_t,
                                _: *mut libc::c_int, _: *mut gss_OID,
                                _: *mut gss_buffer_set_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "453:1"]
        pub fn gss_get_name_attribute(_: *mut OM_uint32, _: gss_name_t,
                                      _: gss_buffer_t, _: *mut libc::c_int,
                                      _: *mut libc::c_int, _: gss_buffer_t,
                                      _: gss_buffer_t, _: *mut libc::c_int)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "57:1"]
        pub fn gss_localname(minor: *mut OM_uint32, name: gss_name_t,
                             mech_type: gss_const_OID,
                             localname: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "150:1"]
        pub fn gss_release_buffer_set(_: *mut OM_uint32,
                                      _: *mut gss_buffer_set_t) -> OM_uint32;
    }
    /* GSSAPI_EXT_H_ */
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
#[c2rust::header_src = "/usr/include/sys/socket.h:52"]
pub mod sys_socket_h {
    use super::socket_h::{sockaddr, socklen_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "102:1"]
        pub fn socket(__domain: libc::c_int, __type: libc::c_int,
                      __protocol: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "112:1"]
        pub fn bind(__fd: libc::c_int, __addr: *const sockaddr,
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
        pub fn accept(__fd: libc::c_int, __addr: *mut sockaddr,
                      __addr_len: *mut socklen_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:52"]
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
#[c2rust::header_src = "/usr/include/unistd.h:55"]
pub mod unistd_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:57"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_generic.h:60"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_krb5.h:61"]
pub mod gssapi_krb5_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID, OM_uint32};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "84:33"]
        pub static gss_mech_iakerb: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "179:1"]
        pub fn krb5_gss_register_acceptor_identity(_: *const libc::c_char)
         -> OM_uint32;
    }
    /* _GSSAPI_KRB5_H_ */
    /* __cplusplus */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/appl/gss-sample/gss-misc.h:62"]
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
        pub fn display_status(msg: *mut libc::c_char, maj_stat: OM_uint32,
                              min_stat: OM_uint32);
        #[no_mangle]
        #[c2rust::src_loc = "39:1"]
        pub fn display_ctx_flags(flags: OM_uint32);
        #[no_mangle]
        #[c2rust::src_loc = "40:1"]
        pub fn print_token(tok: gss_buffer_t);
        #[no_mangle]
        #[c2rust::src_loc = "54:21"]
        pub static mut empty_token: gss_buffer_t;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__u_short, __uint16_t, __uint32_t, __off_t, __off64_t,
                        __time_t, __suseconds_t, __socklen_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::sys_types_h::u_short;
pub use self::struct_timeval_h::timeval;
pub use self::stdint_uintn_h::{uint16_t, uint32_t};
pub use self::socket_h::{socklen_t, sockaddr};
pub use self::socket_type_h::{__socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC,
                              SOCK_PACKET, SOCK_DCCP, SOCK_SEQPACKET,
                              SOCK_RDM, SOCK_RAW, SOCK_DGRAM, SOCK_STREAM};
pub use self::sockaddr_h::sa_family_t;
pub use self::in_h::{in_addr_t, in_addr, in_port_t, sockaddr_in, htons};
pub use self::time_h::{timezone, __timezone_ptr_t, gettimeofday};
pub use self::ctype_h::{C2RustUnnamed, _ISalnum, _ISpunct, _IScntrl, _ISblank,
                        _ISgraph, _ISprint, _ISspace, _ISxdigit, _ISdigit,
                        _ISalpha, _ISlower, _ISupper, __ctype_b_loc};
pub use self::gssapi_h::{gss_name_t, gss_cred_id_t, gss_ctx_id_t, gss_uint32,
                         OM_uint32, gss_OID_desc_struct, gss_OID_desc,
                         gss_OID, gss_OID_set_desc_struct, gss_OID_set_desc,
                         gss_OID_set, gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_channel_bindings_struct,
                         gss_channel_bindings_t, gss_qop_t, gss_cred_usage_t,
                         gss_const_OID, gss_name_struct, gss_cred_id_struct,
                         gss_ctx_id_struct, gss_acquire_cred,
                         gss_release_cred, gss_accept_sec_context,
                         gss_delete_sec_context, gss_get_mic, gss_unwrap,
                         gss_display_name, gss_import_name, gss_release_name,
                         gss_release_buffer, gss_export_sec_context,
                         gss_import_sec_context, gss_release_oid,
                         gss_oid_to_str};
pub use self::gssapi_ext_h::{gss_buffer_set_desc_struct, gss_buffer_set_t,
                             gss_inquire_name, gss_get_name_attribute,
                             gss_localname, gss_release_buffer_set};
use self::stdio_h::{stdout, stderr, fflush, fopen, fprintf, printf, perror};
use self::sys_socket_h::{socket, bind, setsockopt, listen, accept};
use self::string_h::{memcpy, strcmp, strlen};
use self::unistd_h::close;
use self::stdlib_h::{atoi, malloc, free, exit};
use self::gssapi_generic_h::gss_nt_service_name;
use self::gssapi_krb5_h::{gss_mech_iakerb,
                          krb5_gss_register_acceptor_identity};
use self::gss_misc_h::{display_file, send_token, recv_token, display_status,
                       display_ctx_flags, print_token, empty_token};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "634:8"]
pub struct _work_plan {
    pub s: libc::c_int,
    pub server_creds: gss_cred_id_t,
    pub export: libc::c_int,
}
#[c2rust::src_loc = "75:1"]
unsafe extern "C" fn usage() {
    fprintf(stderr,
            b"Usage: gss-server [-port port] [-verbose] [-once]\x00" as
                *const u8 as *const libc::c_char);
    fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"       [-inetd] [-export] [-logfile file] [-keytab keytab]\n       service_name\n\x00"
                as *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
}
#[c2rust::src_loc = "89:14"]
static mut logfile: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
#[c2rust::src_loc = "91:9"]
pub static mut verbose: libc::c_int = 0 as libc::c_int;
/*
 * Function: server_acquire_creds
 *
 * Purpose: imports a service name and acquires credentials for it
 *
 * Arguments:
 *
 *      service_name    (r) the ASCII service name
 *      mech            (r) the desired mechanism (or GSS_C_NO_OID)
 *      server_creds    (w) the GSS-API service credentials
 *
 * Returns: 0 on success, -1 on failure
 *
 * Effects:
 *
 * The service name is imported with gss_import_name, and service
 * credentials are acquired with gss_acquire_cred.  If either opertion
 * fails, an error message is displayed and -1 is returned; otherwise,
 * 0 is returned.  If mech is given, credentials are acquired for the
 * specified mechanism.
 */
#[c2rust::src_loc = "115:1"]
unsafe extern "C" fn server_acquire_creds(mut service_name: *mut libc::c_char,
                                          mut mech: gss_OID,
                                          mut server_creds:
                                              *mut gss_cred_id_t)
 -> libc::c_int {
    let mut name_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut server_name: gss_name_t = 0 as *mut gss_name_struct;
    let mut maj_stat: OM_uint32 = 0;
    let mut min_stat: OM_uint32 = 0;
    let mut mechlist: gss_OID_set_desc =
        gss_OID_set_desc{count: 0, elements: 0 as *mut gss_OID_desc_struct,};
    let mut mechs: gss_OID_set = 0 as gss_OID_set;
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
    if !mech.is_null() {
        mechlist.count = 1 as libc::c_int as size_t;
        mechlist.elements = mech;
        mechs = &mut mechlist
    }
    maj_stat =
        gss_acquire_cred(&mut min_stat, server_name,
                         0 as libc::c_int as OM_uint32, mechs,
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
#[c2rust::src_loc = "174:1"]
unsafe extern "C" fn server_establish_context(mut s: libc::c_int,
                                              mut server_creds: gss_cred_id_t,
                                              mut context: *mut gss_ctx_id_t,
                                              mut client_name: gss_buffer_t,
                                              mut ret_flags: *mut OM_uint32)
 -> libc::c_int {
    let mut send_tok: gss_buffer_desc =
        gss_buffer_desc{length: 0,
                        value: 0 as *mut libc::c_void,}; /* del_cred_handle */
    let mut recv_tok: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut client: gss_name_t = 0 as *mut gss_name_struct;
    let mut doid: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut maj_stat: OM_uint32 = 0;
    let mut min_stat: OM_uint32 = 0;
    let mut acc_sec_min_stat: OM_uint32 = 0;
    let mut oid_name: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
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
        enumerateAttributes(&mut min_stat, client, 1 as libc::c_int);
        showLocalIdentity(&mut min_stat, client);
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
#[c2rust::src_loc = "313:1"]
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
    /* Let the socket be reused right away */
    setsockopt(s, 1 as libc::c_int, 2 as libc::c_int,
               &mut on as *mut libc::c_int as *mut libc::c_char as
                   *const libc::c_void,
               ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                   socklen_t);
    if bind(s, &mut saddr as *mut sockaddr_in as *mut sockaddr,
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
#[c2rust::src_loc = "343:1"]
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
#[c2rust::src_loc = "355:1"]
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
    /*
     * Attempt to save and then restore the context.
     */
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
#[c2rust::src_loc = "424:1"]
unsafe extern "C" fn sign_server(mut s: libc::c_int,
                                 mut server_creds: gss_cred_id_t,
                                 mut export: libc::c_int) -> libc::c_int {
    let mut client_name: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut recv_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut unwrap_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut mic_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut msg_buf: *mut gss_buffer_desc = 0 as *mut gss_buffer_desc;
    let mut send_buf: *mut gss_buffer_desc = 0 as *mut gss_buffer_desc;
    let mut context: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut maj_stat: OM_uint32 = 0;
    let mut min_stat: OM_uint32 = 0;
    let mut i: libc::c_int = 0;
    let mut conf_state: libc::c_int = 0;
    let mut ret_flags: OM_uint32 = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token_flags: libc::c_int = 0;
    let mut send_flags: libc::c_int = 0;
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
        if recv_token(s, &mut token_flags, &mut recv_buf) < 0 as libc::c_int {
            return -(1 as libc::c_int)
        }
        if token_flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
            if !logfile.is_null() {
                fprintf(logfile,
                        b"NOOP token\n\x00" as *const u8 as
                            *const libc::c_char);
            }
            if !recv_buf.value.is_null() {
                free(recv_buf.value);
                recv_buf.value = 0 as *mut libc::c_void
            }
            break ;
        } else {
            if verbose != 0 && !logfile.is_null() {
                fprintf(logfile,
                        b"Message token (flags=%d):\n\x00" as *const u8 as
                            *const libc::c_char, token_flags);
                print_token(&mut recv_buf);
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
                if !recv_buf.value.is_null() {
                    free(recv_buf.value);
                    recv_buf.value = 0 as *mut libc::c_void
                }
                return -(1 as libc::c_int)
            }
            if token_flags & (1 as libc::c_int) << 5 as libc::c_int != 0 {
                maj_stat =
                    gss_unwrap(&mut min_stat, context, &mut recv_buf,
                               &mut unwrap_buf, &mut conf_state,
                               0 as *mut libc::c_void as *mut gss_qop_t);
                if maj_stat != 0 as libc::c_int as libc::c_uint {
                    display_status(b"unsealing message\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char, maj_stat, min_stat);
                    if !recv_buf.value.is_null() {
                        free(recv_buf.value);
                        recv_buf.value = 0 as *mut libc::c_void
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
                if !recv_buf.value.is_null() {
                    free(recv_buf.value);
                    recv_buf.value = 0 as *mut libc::c_void
                }
                msg_buf = &mut unwrap_buf
            } else {
                unwrap_buf.value = 0 as *mut libc::c_void;
                unwrap_buf.length = 0 as libc::c_int as size_t;
                msg_buf = &mut recv_buf
            }
            if !logfile.is_null() {
                fprintf(logfile,
                        b"Received message: \x00" as *const u8 as
                            *const libc::c_char);
                cp = (*msg_buf).value as *mut libc::c_char;
                if (*(*__ctype_b_loc()).offset(*cp.offset(0 as libc::c_int as
                                                              isize) as
                                                   libc::c_int as isize) as
                        libc::c_int &
                        _ISprint as libc::c_int as libc::c_ushort as
                            libc::c_int != 0 ||
                        *(*__ctype_b_loc()).offset(*cp.offset(0 as libc::c_int
                                                                  as isize) as
                                                       libc::c_int as isize)
                            as libc::c_int &
                            _ISspace as libc::c_int as libc::c_ushort as
                                libc::c_int != 0) &&
                       (*(*__ctype_b_loc()).offset(*cp.offset(1 as libc::c_int
                                                                  as isize) as
                                                       libc::c_int as isize)
                            as libc::c_int &
                            _ISprint as libc::c_int as libc::c_ushort as
                                libc::c_int != 0 ||
                            *(*__ctype_b_loc()).offset(*cp.offset(1 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                                                           as libc::c_int as
                                                           isize) as
                                libc::c_int &
                                _ISspace as libc::c_int as libc::c_ushort as
                                    libc::c_int != 0) {
                    fprintf(logfile,
                            b"\"%.*s\"\n\x00" as *const u8 as
                                *const libc::c_char,
                            (*msg_buf).length as libc::c_int,
                            (*msg_buf).value as *mut libc::c_char);
                } else {
                    fprintf(logfile,
                            b"\n\x00" as *const u8 as *const libc::c_char);
                    print_token(msg_buf);
                }
            }
            if token_flags & (1 as libc::c_int) << 7 as libc::c_int != 0 {
                /* loop will break if NOOP received */
                /* Produce a signature block for the message */
                maj_stat =
                    gss_get_mic(&mut min_stat, context,
                                0 as libc::c_int as gss_qop_t, msg_buf,
                                &mut mic_buf);
                if maj_stat != 0 as libc::c_int as libc::c_uint {
                    display_status(b"signing message\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char, maj_stat, min_stat);
                    return -(1 as libc::c_int)
                }
                send_flags = (1 as libc::c_int) << 3 as libc::c_int;
                send_buf = &mut mic_buf
            } else {
                mic_buf.value = 0 as *mut libc::c_void;
                mic_buf.length = 0 as libc::c_int as size_t;
                send_flags = (1 as libc::c_int) << 0 as libc::c_int;
                send_buf = empty_token
            }
            if !recv_buf.value.is_null() {
                free(recv_buf.value);
                recv_buf.value = 0 as *mut libc::c_void
            }
            if !unwrap_buf.value.is_null() {
                gss_release_buffer(&mut min_stat, &mut unwrap_buf);
            }
            /* Send the signature block or NOOP to the client */
            if send_token(s, send_flags, send_buf) < 0 as libc::c_int {
                return -(1 as libc::c_int)
            }
            if !mic_buf.value.is_null() {
                gss_release_buffer(&mut min_stat, &mut mic_buf);
            }
        }
    }
    if !context.is_null() {
        /* Delete context */
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
#[c2rust::src_loc = "574:12"]
static mut max_threads: libc::c_int = 1 as libc::c_int;
#[c2rust::src_loc = "641:1"]
unsafe extern "C" fn worker_bee(mut param: *mut libc::c_void) {
    let mut work: *mut _work_plan = param as *mut _work_plan;
    /* this return value is not checked, because there's
     * not really anything to do if it fails
     */
    sign_server((*work).s, (*work).server_creds, (*work).export);
    close((*work).s);
    free(work as *mut libc::c_void);
}
#[c2rust::src_loc = "659:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut service_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut server_creds: gss_cred_id_t = 0 as *mut gss_cred_id_struct;
    let mut mech: gss_OID = 0 as gss_OID;
    let mut min_stat: OM_uint32 = 0;
    let mut port: u_short = 4444 as libc::c_int as u_short;
    let mut once: libc::c_int = 0 as libc::c_int;
    let mut do_inetd: libc::c_int = 0 as libc::c_int;
    let mut export: libc::c_int = 0 as libc::c_int;
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
        } else if strcmp(*argv,
                         b"-logfile\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            argc -= 1;
            argv = argv.offset(1);
            if argc == 0 { usage(); }
            /* Gross hack, but it makes it unnecessary to add an
             * extra argument to disable logging, and makes the code
             * more efficient because it doesn't actually write data
             * to /dev/null. */
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
        } else if strcmp(*argv,
                         b"-keytab\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            argc -= 1;
            argv = argv.offset(1);
            if argc == 0 { usage(); }
            if krb5_gss_register_acceptor_identity(*argv) != 0 {
                fprintf(stderr,
                        b"failed to register keytab\n\x00" as *const u8 as
                            *const libc::c_char);
                exit(1 as libc::c_int);
            }
        } else {
            if !(strcmp(*argv,
                        b"-iakerb\x00" as *const u8 as *const libc::c_char) ==
                     0 as libc::c_int) {
                break ;
            }
            mech = gss_mech_iakerb
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
    if server_acquire_creds(service_name, mech, &mut server_creds) <
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
            fprintf(stderr,
                    b"starting...\n\x00" as *const u8 as *const libc::c_char);
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
                        accept(stmp, 0 as *mut sockaddr, 0 as *mut socklen_t);
                    if (*work).s < 0 as libc::c_int {
                        perror(b"accepting connection\x00" as *const u8 as
                                   *const libc::c_char);
                        free(work as *mut libc::c_void);
                    } else {
                        (*work).server_creds = server_creds;
                        (*work).export = export;
                        if max_threads == 1 as libc::c_int {
                            worker_bee(work as *mut libc::c_void);
                        }
                    }
                    if !(once == 0) { break ; }
                }
            }
            close(stmp);
        }
    }
    gss_release_cred(&mut min_stat, &mut server_creds);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "827:1"]
unsafe extern "C" fn dumpAttribute(mut minor: *mut OM_uint32,
                                   mut name: gss_name_t,
                                   mut attribute: gss_buffer_t,
                                   mut noisy: libc::c_int) {
    let mut major: OM_uint32 = 0;
    let mut tmp: OM_uint32 = 0;
    let mut value: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut display_value: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut authenticated: libc::c_int = 0 as libc::c_int;
    let mut complete: libc::c_int = 0 as libc::c_int;
    let mut more: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_uint = 0;
    while more != 0 as libc::c_int {
        value.value = 0 as *mut libc::c_void;
        display_value.value = 0 as *mut libc::c_void;
        major =
            gss_get_name_attribute(minor, name, attribute, &mut authenticated,
                                   &mut complete, &mut value,
                                   &mut display_value, &mut more);
        if major &
               ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                    (0o377 as libc::c_ulong as OM_uint32) <<
                        16 as libc::c_int) != 0 {
            display_status(b"gss_get_name_attribute\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,
                           major, *minor);
            break ;
        } else {
            printf(b"Attribute %.*s %s %s\n\n%.*s\n\x00" as *const u8 as
                       *const libc::c_char,
                   (*attribute).length as libc::c_int,
                   (*attribute).value as *mut libc::c_char,
                   if authenticated != 0 {
                       b"Authenticated\x00" as *const u8 as
                           *const libc::c_char
                   } else { b"\x00" as *const u8 as *const libc::c_char },
                   if complete != 0 {
                       b"Complete\x00" as *const u8 as *const libc::c_char
                   } else { b"\x00" as *const u8 as *const libc::c_char },
                   display_value.length as libc::c_int,
                   display_value.value as *mut libc::c_char);
            if noisy != 0 {
                i = 0 as libc::c_int as libc::c_uint;
                while (i as libc::c_ulong) < value.length {
                    if i.wrapping_rem(32 as libc::c_int as libc::c_uint) ==
                           0 as libc::c_int as libc::c_uint {
                        printf(b"\n\x00" as *const u8 as *const libc::c_char);
                    }
                    printf(b"%02x\x00" as *const u8 as *const libc::c_char,
                           *(value.value as
                                 *mut libc::c_char).offset(i as isize) as
                               libc::c_int & 0xff as libc::c_int);
                    i = i.wrapping_add(1)
                }
                printf(b"\n\n\x00" as *const u8 as *const libc::c_char);
            }
            gss_release_buffer(&mut tmp, &mut value);
            gss_release_buffer(&mut tmp, &mut display_value);
        }
    };
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
 * Copyright (C) 2004,2005 by the Massachusetts Institute of Technology.
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
#[c2rust::src_loc = "873:1"]
unsafe extern "C" fn enumerateAttributes(mut minor: *mut OM_uint32,
                                         mut name: gss_name_t,
                                         mut noisy: libc::c_int)
 -> OM_uint32 {
    let mut major: OM_uint32 = 0;
    let mut tmp: OM_uint32 = 0;
    let mut name_is_MN: libc::c_int = 0;
    let mut mech: gss_OID = 0 as gss_OID;
    let mut attrs: gss_buffer_set_t = 0 as gss_buffer_set_t;
    let mut i: libc::c_uint = 0;
    major =
        gss_inquire_name(minor, name, &mut name_is_MN, &mut mech, &mut attrs);
    if major &
           ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                (0o377 as libc::c_ulong as OM_uint32) << 16 as libc::c_int) !=
           0 {
        display_status(b"gss_inquire_name\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char, major,
                       *minor);
        return major
    }
    if !attrs.is_null() {
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) < (*attrs).count {
            dumpAttribute(minor, name,
                          &mut *(*attrs).elements.offset(i as isize), noisy);
            i = i.wrapping_add(1)
        }
    }
    gss_release_oid(&mut tmp, &mut mech);
    gss_release_buffer_set(&mut tmp, &mut attrs);
    return major;
}
#[c2rust::src_loc = "901:1"]
unsafe extern "C" fn showLocalIdentity(mut minor: *mut OM_uint32,
                                       mut name: gss_name_t) -> OM_uint32 {
    let mut major: OM_uint32 = 0;
    let mut buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    major =
        gss_localname(minor, name, 0 as gss_OID as gss_const_OID, &mut buf);
    if major == 0 as libc::c_int as libc::c_uint {
        printf(b"localname: %-*s\n\x00" as *const u8 as *const libc::c_char,
               buf.length as libc::c_int, buf.value as *mut libc::c_char);
    } else if major != (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
     {
        display_status(b"gss_localname\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char, major,
                       *minor);
    }
    gss_release_buffer(minor, &mut buf);
    return major;
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
