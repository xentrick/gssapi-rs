use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:6"]
pub mod types_h {
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
}
#[c2rust::header_src = "/usr/include/sys/types.h:6"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:6"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:6"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:6"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:6"]
pub mod gssrpc_types_h {
    #[c2rust::src_loc = "93:1"]
    pub type rpc_inline_t = int32_t;
    use super::stdint_intn_h::int32_t;
    /* !defined(GSSRPC_TYPES_H) */
    /*
 * The below should probably be internal-only, but seem to be
 * traditionally exported in RPC implementations.
 */
    /* XXX namespace */
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:6"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:6"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:6"]
pub mod xdr_h {
    /* @(#)xdr.h	2.2 88/07/29 4.0 RPCSRC */
/*
 * Copyright (c) 2010, Oracle America, Inc.
 *
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
 *
 *     * Neither the name of the "Oracle America, Inc." nor the names of
 *       its contributors may be used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
 * IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
 * TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/*      @(#)xdr.h 1.19 87/04/22 SMI      */
    /*
 * xdr.h, External Data Representation Serialization Routines.
 */
    /* for FILE */
    /*
 * XDR provides a conventional way for converting between C data
 * types and an external bit-string representation.  Library supplied
 * routines provide for the conversion on built-in C data types.  These
 * routines and utility routines defined here are used to help implement
 * a type encode/decode routine for each user-defined type.
 *
 * Each data type provides a single procedure which takes two arguments:
 *
 *	bool_t
 *	xdrproc(xdrs, argresp)
 *		XDR *xdrs;
 *		<type> *argresp;
 *
 * xdrs is an instance of a XDR handle, to which or from which the data
 * type is to be converted.  argresp is a pointer to the structure to be
 * converted.  The XDR handle contains an operation field which indicates
 * which of the operations (ENCODE, DECODE * or FREE) is to be performed.
 *
 * XDR_DECODE may allocate space if the pointer argresp is null.  This
 * data can be freed with the XDR_FREE operation.
 *
 * We write only one procedure per data type to make it easy
 * to keep the encode and decode procedures for a data type consistent.
 * In many cases the same code performs all operations on a user defined type,
 * because all the hard work is done in the component type routines.
 * decode as a series of calls on the nested data types.
 */
    /*
 * Xdr operations.  XDR_ENCODE causes the type to be encoded into the
 * stream.  XDR_DECODE causes the type to be extracted from the stream.
 * XDR_FREE can be used to release the space allocated by an XDR_DECODE
 * request.
 */
    #[c2rust::src_loc = "81:1"]
    pub type xdr_op = libc::c_uint;
    #[c2rust::src_loc = "84:2"]
    pub const XDR_FREE: xdr_op = 2;
    #[c2rust::src_loc = "83:2"]
    pub const XDR_DECODE: xdr_op = 1;
    #[c2rust::src_loc = "82:2"]
    pub const XDR_ENCODE: xdr_op = 0;
    /*
 * The XDR handle.
 * Contains operation which is being applied to the stream,
 * an operations vector for the paticular implementation (e.g. see xdr_mem.c),
 * and two private fields for the use of the particular impelementation.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "113:16"]
    pub struct XDR {
        pub x_op: xdr_op,
        pub x_ops: *mut xdr_ops,
        pub x_public: caddr_t,
        pub x_private: *mut libc::c_void,
        pub x_base: caddr_t,
        pub x_handy: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "115:9"]
    pub struct xdr_ops {
        pub x_getlong: Option<unsafe extern "C" fn(_: *mut XDR,
                                                   _: *mut libc::c_long)
                                  -> libc::c_int>,
        pub x_putlong: Option<unsafe extern "C" fn(_: *mut XDR,
                                                   _: *mut libc::c_long)
                                  -> libc::c_int>,
        pub x_getbytes: Option<unsafe extern "C" fn(_: *mut XDR, _: caddr_t,
                                                    _: u_int) -> libc::c_int>,
        pub x_putbytes: Option<unsafe extern "C" fn(_: *mut XDR, _: caddr_t,
                                                    _: u_int) -> libc::c_int>,
        pub x_getpostn: Option<unsafe extern "C" fn(_: *mut XDR) -> u_int>,
        pub x_setpostn: Option<unsafe extern "C" fn(_: *mut XDR, _: u_int)
                                   -> libc::c_int>,
        pub x_inline: Option<unsafe extern "C" fn(_: *mut XDR, _: libc::c_int)
                                 -> *mut rpc_inline_t>,
        pub x_destroy: Option<unsafe extern "C" fn(_: *mut XDR) -> ()>,
    }
    use super::sys_types_h::{caddr_t, u_int};
    use super::gssrpc_types_h::rpc_inline_t;
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "258:1"]
        pub fn gssrpc_xdr_bool(_: *mut XDR, _: *mut libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "262:1"]
        pub fn gssrpc_xdr_bytes(_: *mut XDR, _: *mut *mut libc::c_char,
                                _: *mut u_int, _: u_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "298:1"]
        pub fn gssrpc_xdr_u_int32(_: *mut XDR, _: *mut uint32_t)
         -> libc::c_int;
        /*
 * These are the public routines for the various implementations of
 * xdr streams.
 */
        /* XDR allocating memory buffer */
        #[no_mangle]
        #[c2rust::src_loc = "306:1"]
        pub fn gssrpc_xdralloc_create(_: *mut XDR, _: xdr_op);
        /* get buffer from xdralloc */
        #[no_mangle]
        #[c2rust::src_loc = "312:1"]
        pub fn gssrpc_xdralloc_getdata(_: *mut XDR) -> caddr_t;
        /* XDR using memory buffers */
        #[no_mangle]
        #[c2rust::src_loc = "315:1"]
        pub fn gssrpc_xdrmem_create(_: *mut XDR, _: caddr_t, _: u_int,
                                    _: xdr_op);
    }
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:6"]
pub mod gssapi_h {
    #[c2rust::src_loc = "85:1"]
    pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
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
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_desc = gss_buffer_desc_struct;
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "84:8"]
        pub type gss_ctx_id_struct;
        /* qop_state */
        #[no_mangle]
        #[c2rust::src_loc = "530:1"]
        pub fn gss_display_status(_: *mut OM_uint32, _: OM_uint32,
                                  _: libc::c_int, _: gss_OID,
                                  _: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
        /* input_name */
        #[no_mangle]
        #[c2rust::src_loc = "574:1"]
        pub fn gss_release_buffer(_: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
        /* qop_state */
        #[no_mangle]
        #[c2rust::src_loc = "750:1"]
        pub fn gss_seal(_: *mut OM_uint32, _: gss_ctx_id_t, _: libc::c_int,
                        _: libc::c_int, _: gss_buffer_t, _: *mut libc::c_int,
                        _: gss_buffer_t) -> OM_uint32;
        /* output_message_buffer */
        #[no_mangle]
        #[c2rust::src_loc = "760:1"]
        pub fn gss_unseal(_: *mut OM_uint32, _: gss_ctx_id_t, _: gss_buffer_t,
                          _: gss_buffer_t, _: *mut libc::c_int,
                          _: *mut libc::c_int) -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth_gssapi.h:10"]
pub mod auth_gssapi_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "32:16"]
    pub struct _auth_gssapi_creds {
        pub version: uint32_t,
        pub auth_msg: libc::c_int,
        pub client_handle: gss_buffer_desc,
    }
    #[c2rust::src_loc = "32:1"]
    pub type auth_gssapi_creds = _auth_gssapi_creds;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:16"]
    pub struct _auth_gssapi_init_arg {
        pub version: uint32_t,
        pub token: gss_buffer_desc,
    }
    #[c2rust::src_loc = "38:1"]
    pub type auth_gssapi_init_arg = _auth_gssapi_init_arg;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "43:16"]
    pub struct _auth_gssapi_init_res {
        pub version: uint32_t,
        pub client_handle: gss_buffer_desc,
        pub gss_major: OM_uint32,
        pub gss_minor: OM_uint32,
        pub token: gss_buffer_desc,
        pub signed_isn: gss_buffer_desc,
    }
    #[c2rust::src_loc = "43:1"]
    pub type auth_gssapi_init_res = _auth_gssapi_init_res;
    use super::stdint_uintn_h::uint32_t;
    use super::gssapi_h::{gss_buffer_desc, OM_uint32};
    /* !defined(GSSRPC_AUTH_GSSAPI_H) */
}
#[c2rust::header_src = "/usr/include/stdlib.h:6"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/netinet/in.h:6"]
pub mod in_h {
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "375:1"]
        pub fn ntohl(__netlong: uint32_t) -> uint32_t;
        #[no_mangle]
        #[c2rust::src_loc = "378:1"]
        pub fn htonl(__hostlong: uint32_t) -> uint32_t;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:6"]
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
        #[c2rust::src_loc = "522:1"]
        pub fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "626:1"]
        pub fn fputs(__s: *const libc::c_char, __stream: *mut FILE)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "652:15"]
        pub fn fwrite(_: *const libc::c_void, _: libc::c_ulong,
                      _: libc::c_ulong, _: *mut FILE) -> libc::c_ulong;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/rpc/gssrpcint.h:12"]
pub mod gssrpcint_h {
    extern "C" {
        /* lib/rpc/gssrpcint.h */
/*
 * Copyright (C) 2008 by the Massachusetts Institute of Technology.
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
        #[no_mangle]
        #[c2rust::src_loc = "29:1"]
        pub fn gssrpcint_printf(format: *const libc::c_char, _: ...);
    }
    /* __GSSRPCINT_H__ */
}
pub use self::types_h::{__u_int, __int32_t, __uint32_t, __off_t, __off64_t,
                        __caddr_t};
pub use self::sys_types_h::{u_int, caddr_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssrpc_types_h::rpc_inline_t;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, XDR, xdr_ops,
                      gssrpc_xdr_bool, gssrpc_xdr_bytes, gssrpc_xdr_u_int32,
                      gssrpc_xdralloc_create, gssrpc_xdralloc_getdata,
                      gssrpc_xdrmem_create};
pub use self::gssapi_h::{gss_ctx_id_t, gss_uint32, OM_uint32,
                         gss_OID_desc_struct, gss_OID, gss_buffer_desc_struct,
                         gss_buffer_desc, gss_buffer_t, gss_ctx_id_struct,
                         gss_display_status, gss_release_buffer, gss_seal,
                         gss_unseal};
pub use self::auth_gssapi_h::{_auth_gssapi_creds, auth_gssapi_creds,
                              _auth_gssapi_init_arg, auth_gssapi_init_arg,
                              _auth_gssapi_init_res, auth_gssapi_init_res};
use self::stdlib_h::free;
use self::in_h::{ntohl, htonl};
use self::stdio_h::{stderr, fprintf, putc, fputs, fwrite};
use self::gssrpcint_h::gssrpcint_printf;
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved.
 *
 */
#[no_mangle]
#[c2rust::src_loc = "19:5"]
pub static mut gssrpc_misc_debug_gssapi: libc::c_int = 0 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "34:1"]
pub unsafe extern "C" fn gssrpc_xdr_gss_buf(mut xdrs: *mut XDR,
                                            mut buf: gss_buffer_t)
 -> libc::c_int {
    /*
      * On decode, xdr_bytes will only allocate buf->value if the
      * length read in is < maxsize (last arg).  This is dumb, because
      * the whole point of allocating memory is so that I don't *have*
      * to know the maximum length.  -1 effectively disables this
      * braindamage.
      */
    let mut result: libc::c_int = 0;
    /* Fix type mismatches between APIs.  */
    let mut length: libc::c_uint =
        (*buf).length as libc::c_uint; /* assumption */
    let mut cp: *mut libc::c_char = (*buf).value as *mut libc::c_char;
    result =
        gssrpc_xdr_bytes(xdrs, &mut cp, &mut length,
                         if (*xdrs).x_op as libc::c_uint ==
                                XDR_DECODE as libc::c_int as libc::c_uint &&
                                (*buf).value.is_null() {
                             -(1 as libc::c_int) as libc::c_uint
                         } else { (*buf).length as libc::c_uint });
    (*buf).value = cp as *mut libc::c_void;
    (*buf).length = length as size_t;
    return result;
}
#[no_mangle]
#[c2rust::src_loc = "57:1"]
pub unsafe extern "C" fn gssrpc_xdr_authgssapi_creds(mut xdrs: *mut XDR,
                                                     mut creds:
                                                         *mut auth_gssapi_creds)
 -> libc::c_int {
    if gssrpc_xdr_u_int32(xdrs, &mut (*creds).version) == 0 ||
           gssrpc_xdr_bool(xdrs, &mut (*creds).auth_msg) == 0 ||
           gssrpc_xdr_gss_buf(xdrs, &mut (*creds).client_handle) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "68:1"]
pub unsafe extern "C" fn gssrpc_xdr_authgssapi_init_arg(mut xdrs: *mut XDR,
                                                        mut init_arg:
                                                            *mut auth_gssapi_init_arg)
 -> libc::c_int {
    if gssrpc_xdr_u_int32(xdrs, &mut (*init_arg).version) == 0 ||
           gssrpc_xdr_gss_buf(xdrs, &mut (*init_arg).token) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "78:1"]
pub unsafe extern "C" fn gssrpc_xdr_authgssapi_init_res(mut xdrs: *mut XDR,
                                                        mut init_res:
                                                            *mut auth_gssapi_init_res)
 -> libc::c_int {
    if gssrpc_xdr_u_int32(xdrs, &mut (*init_res).version) == 0 ||
           gssrpc_xdr_gss_buf(xdrs, &mut (*init_res).client_handle) == 0 ||
           gssrpc_xdr_u_int32(xdrs, &mut (*init_res).gss_major) == 0 ||
           gssrpc_xdr_u_int32(xdrs, &mut (*init_res).gss_minor) == 0 ||
           gssrpc_xdr_gss_buf(xdrs, &mut (*init_res).token) == 0 ||
           gssrpc_xdr_gss_buf(xdrs, &mut (*init_res).signed_isn) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn gssrpc_auth_gssapi_seal_seq(mut context:
                                                         gss_ctx_id_t,
                                                     mut seq_num: uint32_t,
                                                     mut out_buf:
                                                         gss_buffer_t)
 -> libc::c_int {
    let mut in_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut gssstat: OM_uint32 = 0;
    let mut minor_stat: OM_uint32 = 0;
    let mut nl_seq_num: uint32_t = 0;
    nl_seq_num = htonl(seq_num);
    in_buf.length = ::std::mem::size_of::<uint32_t>() as libc::c_ulong;
    in_buf.value =
        &mut nl_seq_num as *mut uint32_t as *mut libc::c_char as
            *mut libc::c_void;
    gssstat =
        gss_seal(&mut minor_stat, context, 0 as libc::c_int, 0 as libc::c_int,
                 &mut in_buf, 0 as *mut libc::c_int, out_buf);
    if gssstat != 0 as libc::c_int as libc::c_uint {
        if gssrpc_misc_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"gssapi_seal_seq: failed\n\x00" as *const u8 as
                                 *const libc::c_char);
        }
        if gssrpc_misc_debug_gssapi != 0 {
            gssrpc_auth_gssapi_display_status(b"sealing sequence number\x00"
                                                  as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char, gssstat,
                                              minor_stat);
        }
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "116:1"]
pub unsafe extern "C" fn gssrpc_auth_gssapi_unseal_seq(mut context:
                                                           gss_ctx_id_t,
                                                       mut in_buf:
                                                           gss_buffer_t,
                                                       mut seq_num:
                                                           *mut uint32_t)
 -> libc::c_int {
    let mut out_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut gssstat: OM_uint32 = 0;
    let mut minor_stat: OM_uint32 = 0;
    let mut nl_seq_num: uint32_t = 0;
    gssstat =
        gss_unseal(&mut minor_stat, context, in_buf, &mut out_buf,
                   0 as *mut libc::c_int, 0 as *mut libc::c_int);
    if gssstat != 0 as libc::c_int as libc::c_uint {
        if gssrpc_misc_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"gssapi_unseal_seq: failed\n\x00" as *const u8
                                 as *const libc::c_char);
        }
        if gssrpc_misc_debug_gssapi != 0 {
            gssrpc_auth_gssapi_display_status(b"unsealing sequence number\x00"
                                                  as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char, gssstat,
                                              minor_stat);
        }
        return 0 as libc::c_int
    } else {
        if out_buf.length !=
               ::std::mem::size_of::<uint32_t>() as libc::c_ulong {
            if gssrpc_misc_debug_gssapi >= 99 as libc::c_int {
                gssrpcint_printf(b"gssapi_unseal_seq: unseal gave %d bytes\n\x00"
                                     as *const u8 as *const libc::c_char,
                                 out_buf.length as libc::c_int);
            }
            gss_release_buffer(&mut minor_stat, &mut out_buf);
            return 0 as libc::c_int
        }
    }
    nl_seq_num = *(out_buf.value as *mut uint32_t);
    *seq_num = ntohl(nl_seq_num);
    gss_release_buffer(&mut minor_stat, &mut out_buf);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "146:1"]
pub unsafe extern "C" fn gssrpc_auth_gssapi_display_status(mut msg:
                                                               *mut libc::c_char,
                                                           mut major:
                                                               OM_uint32,
                                                           mut minor:
                                                               OM_uint32) {
    auth_gssapi_display_status_1(msg, major, 1 as libc::c_int,
                                 0 as libc::c_int);
    auth_gssapi_display_status_1(msg, minor, 2 as libc::c_int,
                                 0 as libc::c_int);
}
#[c2rust::src_loc = "155:1"]
unsafe extern "C" fn auth_gssapi_display_status_1(mut m: *mut libc::c_char,
                                                  mut code: OM_uint32,
                                                  mut type_0: libc::c_int,
                                                  mut rec: libc::c_int) {
    let mut gssstat: OM_uint32 = 0;
    let mut minor_stat: OM_uint32 = 0;
    let mut msg: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut msg_ctx: OM_uint32 = 0;
    msg_ctx = 0 as libc::c_int as OM_uint32;
    loop  {
        gssstat =
            gss_display_status(&mut minor_stat, code, type_0, 0 as gss_OID,
                               &mut msg_ctx, &mut msg);
        if gssstat != 0 as libc::c_int as libc::c_uint {
            if rec == 0 {
                auth_gssapi_display_status_1(m, gssstat, 1 as libc::c_int,
                                             1 as libc::c_int);
                auth_gssapi_display_status_1(m, minor_stat, 2 as libc::c_int,
                                             1 as libc::c_int);
            } else {
                fputs(b"GSS-API authentication error \x00" as *const u8 as
                          *const libc::c_char, stderr);
                fwrite(msg.value, msg.length,
                       1 as libc::c_int as libc::c_ulong, stderr);
                fputs(b": recursive failure!\n\x00" as *const u8 as
                          *const libc::c_char, stderr);
            }
            return
        }
        fprintf(stderr,
                b"GSS-API authentication error %s: \x00" as *const u8 as
                    *const libc::c_char, m);
        fwrite(msg.value, msg.length, 1 as libc::c_int as libc::c_ulong,
               stderr);
        putc('\n' as i32, stderr);
        if gssrpc_misc_debug_gssapi != 0 {
            gssrpcint_printf(b"GSS-API authentication error %s: %*s\n\x00" as
                                 *const u8 as *const libc::c_char, m,
                             msg.length as libc::c_int,
                             msg.value as *mut libc::c_char);
        }
        gss_release_buffer(&mut minor_stat, &mut msg);
        if msg_ctx == 0 { break ; }
    };
}
#[no_mangle]
#[c2rust::src_loc = "196:1"]
pub unsafe extern "C" fn gssrpc_auth_gssapi_wrap_data(mut major:
                                                          *mut OM_uint32,
                                                      mut minor:
                                                          *mut OM_uint32,
                                                      mut context:
                                                          gss_ctx_id_t,
                                                      mut seq_num: uint32_t,
                                                      mut out_xdrs: *mut XDR,
                                                      mut xdr_func:
                                                          Option<unsafe extern "C" fn()
                                                                     ->
                                                                         libc::c_int>,
                                                      mut xdr_ptr: caddr_t)
 -> libc::c_int {
    let mut in_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut out_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut temp_xdrs: XDR =
        XDR{x_op: XDR_ENCODE,
            x_ops: 0 as *mut xdr_ops,
            x_public: 0 as *mut libc::c_char,
            x_private: 0 as *mut libc::c_void,
            x_base: 0 as *mut libc::c_char,
            x_handy: 0,};
    let mut conf_state: libc::c_int = 0;
    let mut length: libc::c_uint = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if gssrpc_misc_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"gssapi_wrap_data: starting\n\x00" as *const u8 as
                             *const libc::c_char);
    }
    *major = 0 as libc::c_int as OM_uint32;
    *minor = 0 as libc::c_int as OM_uint32;
    gssrpc_xdralloc_create(&mut temp_xdrs, XDR_ENCODE);
    /* serialize the sequence number into local memory */
    if gssrpc_misc_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"gssapi_wrap_data: encoding seq_num %d\n\x00" as
                             *const u8 as *const libc::c_char, seq_num);
    }
    if gssrpc_xdr_u_int32(&mut temp_xdrs, &mut seq_num) == 0 {
        if gssrpc_misc_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"gssapi_wrap_data: serializing seq_num failed\n\x00"
                                 as *const u8 as *const libc::c_char);
        }
        if (*temp_xdrs.x_ops).x_destroy.is_some() {
            Some((*temp_xdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut temp_xdrs);
        }
        return 0 as libc::c_int
    }
    /* serialize the arguments into local memory */
    if ::std::mem::transmute::<_,
                               fn(_: _, _: _)
                                   ->
                                       libc::c_int>(Some(xdr_func.expect("non-null function pointer")).expect("non-null function pointer"))(&mut temp_xdrs,
                                                                                                                                            xdr_ptr)
           == 0 {
        if gssrpc_misc_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"gssapi_wrap_data: serializing arguments failed\n\x00"
                                 as *const u8 as *const libc::c_char);
        }
        if (*temp_xdrs.x_ops).x_destroy.is_some() {
            Some((*temp_xdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut temp_xdrs);
        }
        return 0 as libc::c_int
    }
    in_buf.length =
        Some((*temp_xdrs.x_ops).x_getpostn.expect("non-null function pointer")).expect("non-null function pointer")(&mut temp_xdrs)
            as size_t;
    in_buf.value =
        gssrpc_xdralloc_getdata(&mut temp_xdrs) as *mut libc::c_void;
    *major =
        gss_seal(minor, context, 1 as libc::c_int, 0 as libc::c_int,
                 &mut in_buf, &mut conf_state, &mut out_buf);
    if *major != 0 as libc::c_int as libc::c_uint {
        if (*temp_xdrs.x_ops).x_destroy.is_some() {
            Some((*temp_xdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut temp_xdrs);
        }
        return 0 as libc::c_int
    }
    if gssrpc_misc_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"gssapi_wrap_data: %d bytes data, %d bytes sealed\n\x00"
                             as *const u8 as *const libc::c_char,
                         in_buf.length as libc::c_int,
                         out_buf.length as libc::c_int);
    }
    /* write the token */
    length = out_buf.length as libc::c_uint; /* assumption */
    cp = out_buf.value as *mut libc::c_char;
    if gssrpc_xdr_bytes(out_xdrs, &mut cp, &mut length,
                        out_buf.length as u_int) == 0 {
        if gssrpc_misc_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"gssapi_wrap_data: serializing encrypted data failed\n\x00"
                                 as *const u8 as *const libc::c_char);
        }
        if (*temp_xdrs.x_ops).x_destroy.is_some() {
            Some((*temp_xdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut temp_xdrs);
        }
        return 0 as libc::c_int
    }
    out_buf.value = cp as *mut libc::c_void;
    *major = gss_release_buffer(minor, &mut out_buf);
    if gssrpc_misc_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"gssapi_wrap_data: succeeding\n\n\x00" as *const u8
                             as *const libc::c_char);
    }
    if (*temp_xdrs.x_ops).x_destroy.is_some() {
        Some((*temp_xdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut temp_xdrs);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "264:1"]
pub unsafe extern "C" fn gssrpc_auth_gssapi_unwrap_data(mut major:
                                                            *mut OM_uint32,
                                                        mut minor:
                                                            *mut OM_uint32,
                                                        mut context:
                                                            gss_ctx_id_t,
                                                        mut seq_num: uint32_t,
                                                        mut in_xdrs: *mut XDR,
                                                        mut xdr_func:
                                                            Option<unsafe extern "C" fn()
                                                                       ->
                                                                           libc::c_int>,
                                                        mut xdr_ptr: caddr_t)
 -> libc::c_int {
    let mut in_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut out_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut temp_xdrs: XDR =
        XDR{x_op: XDR_ENCODE,
            x_ops: 0 as *mut xdr_ops,
            x_public: 0 as *mut libc::c_char,
            x_private: 0 as *mut libc::c_void,
            x_base: 0 as *mut libc::c_char,
            x_handy: 0,};
    let mut verf_seq_num: uint32_t = 0;
    let mut conf: libc::c_int = 0;
    let mut qop: libc::c_int = 0;
    let mut length: libc::c_uint = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if gssrpc_misc_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"gssapi_unwrap_data: starting\n\x00" as *const u8 as
                             *const libc::c_char);
    }
    *major = 0 as libc::c_int as OM_uint32;
    *minor = 0 as libc::c_int as OM_uint32;
    in_buf.value = 0 as *mut libc::c_void;
    out_buf.value = 0 as *mut libc::c_void;
    cp = in_buf.value as *mut libc::c_char;
    if gssrpc_xdr_bytes(in_xdrs, &mut cp, &mut length,
                        -(1 as libc::c_int) as libc::c_uint) == 0 {
        if gssrpc_misc_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"gssapi_unwrap_data: deserializing encrypted data failed\n\x00"
                                 as *const u8 as *const libc::c_char);
        }
        temp_xdrs.x_op = XDR_FREE;
        gssrpc_xdr_bytes(&mut temp_xdrs, &mut cp, &mut length,
                         -(1 as libc::c_int) as libc::c_uint);
        in_buf.value = 0 as *mut libc::c_void;
        return 0 as libc::c_int
    }
    in_buf.value = cp as *mut libc::c_void;
    in_buf.length = length as size_t;
    *major =
        gss_unseal(minor, context, &mut in_buf, &mut out_buf, &mut conf,
                   &mut qop);
    free(in_buf.value);
    if *major != 0 as libc::c_int as libc::c_uint { return 0 as libc::c_int }
    if gssrpc_misc_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"gssapi_unwrap_data: %llu bytes data, %llu bytes sealed\n\x00"
                             as *const u8 as *const libc::c_char,
                         out_buf.length as libc::c_ulonglong,
                         in_buf.length as libc::c_ulonglong);
    }
    gssrpc_xdrmem_create(&mut temp_xdrs, out_buf.value as caddr_t,
                         out_buf.length as u_int, XDR_DECODE);
    /* deserialize the sequence number */
    if gssrpc_xdr_u_int32(&mut temp_xdrs, &mut verf_seq_num) == 0 {
        if gssrpc_misc_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"gssapi_unwrap_data: deserializing verf_seq_num failed\n\x00"
                                 as *const u8 as *const libc::c_char);
        }
        gss_release_buffer(minor, &mut out_buf);
        if (*temp_xdrs.x_ops).x_destroy.is_some() {
            Some((*temp_xdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut temp_xdrs);
        }
        return 0 as libc::c_int
    }
    if verf_seq_num != seq_num {
        if gssrpc_misc_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"gssapi_unwrap_data: seq %d specified, read %d\n\x00"
                                 as *const u8 as *const libc::c_char, seq_num,
                             verf_seq_num);
        }
        gss_release_buffer(minor, &mut out_buf);
        if (*temp_xdrs.x_ops).x_destroy.is_some() {
            Some((*temp_xdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut temp_xdrs);
        }
        return 0 as libc::c_int
    }
    if gssrpc_misc_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"gssapi_unwrap_data: unwrap seq_num %d okay\n\x00"
                             as *const u8 as *const libc::c_char,
                         verf_seq_num);
    }
    /* deserialize the arguments into xdr_ptr */
    if ::std::mem::transmute::<_,
                               fn(_: _, _: _)
                                   ->
                                       libc::c_int>(Some(xdr_func.expect("non-null function pointer")).expect("non-null function pointer"))(&mut temp_xdrs,
                                                                                                                                            xdr_ptr)
           == 0 {
        if gssrpc_misc_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"gssapi_unwrap_data: deserializing arguments failed\n\x00"
                                 as *const u8 as *const libc::c_char);
        }
        gss_release_buffer(minor, &mut out_buf);
        if (*temp_xdrs.x_ops).x_destroy.is_some() {
            Some((*temp_xdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut temp_xdrs);
        }
        return 0 as libc::c_int
    }
    if gssrpc_misc_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"gssapi_unwrap_data: succeeding\n\n\x00" as
                             *const u8 as *const libc::c_char);
    }
    gss_release_buffer(minor, &mut out_buf);
    if (*temp_xdrs.x_ops).x_destroy.is_some() {
        Some((*temp_xdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut temp_xdrs);
    }
    return 1 as libc::c_int;
}
