use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:8"]
pub mod types_h {
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
}
#[c2rust::header_src = "/usr/include/sys/types.h:8"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:8"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:9"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    use super::types_h::{__uint8_t, __uint16_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:9"]
pub mod krb5_h {
    /* typedef struct _profile_t *profile_t; */
    /*
 * begin wordsize.h
 */
    /*
 * Word-size related definition.
 */
    #[c2rust::src_loc = "136:1"]
    pub type krb5_octet = uint8_t;
    #[c2rust::src_loc = "137:1"]
    pub type krb5_int16 = int16_t;
    #[c2rust::src_loc = "138:1"]
    pub type krb5_ui_2 = uint16_t;
    #[c2rust::src_loc = "176:1"]
    pub type krb5_kvno = libc::c_uint;
    use super::stdint_uintn_h::{uint8_t, uint16_t};
    use super::stdint_intn_h::int16_t;
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:10"]
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
    /* This is for rpc/netdb.h */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:10"]
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
 * This is the number of bytes per unit of external data.
 */
    /*
 * A xdrproc_t exists for each data type which is to be encoded or decoded.
 *
 * The second argument to the xdrproc_t is a pointer to an opaque pointer.
 * The opaque pointer generally points to a structure of the data type
 * to be decoded.  If this pointer is 0, then the type routines should
 * allocate dynamic storage of the appropriate size and return it.
 * bool_t	(*xdrproc_t)(XDR *, caddr_t *);
 *
 * XXX can't actually prototype it, because some take three args!!!
 */
    #[c2rust::src_loc = "105:1"]
    pub type xdrproc_t = Option<unsafe extern "C" fn() -> libc::c_int>;
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
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "252:1"]
        pub fn gssrpc_xdr_int(_: *mut XDR, _: *mut libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "253:1"]
        pub fn gssrpc_xdr_u_int(_: *mut XDR, _: *mut u_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "254:1"]
        pub fn gssrpc_xdr_long(_: *mut XDR, _: *mut libc::c_long)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "260:1"]
        pub fn gssrpc_xdr_array(_: *mut XDR, _: *mut caddr_t, _: *mut u_int,
                                _: u_int, _: u_int, _: xdrproc_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "262:1"]
        pub fn gssrpc_xdr_bytes(_: *mut XDR, _: *mut *mut libc::c_char,
                                _: *mut u_int, _: u_int) -> libc::c_int;
        /* XDR using memory buffers */
        #[no_mangle]
        #[c2rust::src_loc = "315:1"]
        pub fn gssrpc_xdrmem_create(_: *mut XDR, _: caddr_t, _: u_int,
                                    _: xdr_op);
    }
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:11"]
pub mod kdb_h {
    /*
 * If this ever changes up the version number and make the arrays be as
 * big as necessary.
 *
 * Currently the first type is the enctype and the second is the salt type.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "167:16"]
    pub struct _krb5_key_data {
        pub key_data_ver: krb5_int16,
        pub key_data_kvno: krb5_ui_2,
        pub key_data_type: [krb5_int16; 2],
        pub key_data_length: [krb5_ui_2; 2],
        pub key_data_contents: [*mut krb5_octet; 2],
    }
    #[c2rust::src_loc = "167:1"]
    pub type krb5_key_data = _krb5_key_data;
    use super::krb5_h::{krb5_int16, krb5_ui_2, krb5_octet};
    /* Array of pointers */
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/server_internal.h:11"]
pub mod server_internal_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "57:16"]
    pub struct _osa_pw_hist_t {
        pub n_key_data: libc::c_int,
        pub key_data: *mut krb5_key_data,
    }
    #[c2rust::src_loc = "57:1"]
    pub type osa_pw_hist_ent = _osa_pw_hist_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:16"]
    pub struct _osa_princ_ent_t {
        pub version: libc::c_int,
        pub policy: *mut libc::c_char,
        pub aux_attributes: libc::c_long,
        pub old_key_len: libc::c_uint,
        pub old_key_next: libc::c_uint,
        pub admin_history_kvno: krb5_kvno,
        pub old_keys: *mut osa_pw_hist_ent,
    }
    #[c2rust::src_loc = "62:1"]
    pub type osa_princ_ent_t = *mut _osa_princ_ent_t;
    use super::kdb_h::krb5_key_data;
    use super::krb5_h::krb5_kvno;
    /* __KADM5_SERVER_INTERNAL_H__ */
    /* * @}*/
}
#[c2rust::header_src = "/usr/include/stdlib.h:9"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin_xdr.h:12"]
pub mod admin_xdr_h {
    use super::xdr_h::XDR;
    use super::krb5_h::{krb5_int16, krb5_kvno, krb5_ui_2};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "21:1"]
        pub fn xdr_krb5_int16(xdrs: *mut XDR, objp: *mut krb5_int16)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "17:1"]
        pub fn xdr_krb5_kvno(xdrs: *mut XDR, objp: *mut krb5_kvno)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "14:1"]
        pub fn xdr_nullstring(xdrs: *mut XDR, objp: *mut *mut libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "22:1"]
        pub fn xdr_krb5_ui_2(xdrs: *mut XDR, objp: *mut krb5_ui_2)
         -> libc::c_int;
    }
}
pub use self::types_h::{__u_int, __uint8_t, __int16_t, __uint16_t, __int32_t,
                        __caddr_t};
pub use self::sys_types_h::{u_int, caddr_t};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_kvno};
pub use self::gssrpc_types_h::rpc_inline_t;
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, gssrpc_xdr_int, gssrpc_xdr_u_int,
                      gssrpc_xdr_long, gssrpc_xdr_array, gssrpc_xdr_bytes,
                      gssrpc_xdrmem_create};
pub use self::kdb_h::{_krb5_key_data, krb5_key_data};
pub use self::server_internal_h::{_osa_pw_hist_t, osa_pw_hist_ent,
                                  _osa_princ_ent_t, osa_princ_ent_t};
use self::stdlib_h::free;
use self::admin_xdr_h::{xdr_krb5_int16, xdr_krb5_kvno, xdr_nullstring,
                        xdr_krb5_ui_2};
/* -*- mode: c; c-file-style: "bsd"; indent-tabs-mode: t -*- */
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved
 *
 * $Header$
 */
#[no_mangle]
#[c2rust::src_loc = "17:1"]
pub unsafe extern "C" fn xdr_krb5_key_data(mut xdrs: *mut XDR,
                                           mut objp: *mut krb5_key_data)
 -> libc::c_int {
    let mut tmp: libc::c_uint = 0;
    if xdr_krb5_int16(xdrs, &mut (*objp).key_data_ver) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_ui_2(xdrs, &mut (*objp).key_data_kvno) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_int16(xdrs,
                      &mut *(*objp).key_data_type.as_mut_ptr().offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize))
           == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_int16(xdrs,
                      &mut *(*objp).key_data_type.as_mut_ptr().offset(1 as
                                                                          libc::c_int
                                                                          as
                                                                          isize))
           == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_ui_2(xdrs,
                     &mut *(*objp).key_data_length.as_mut_ptr().offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize))
           == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_ui_2(xdrs,
                     &mut *(*objp).key_data_length.as_mut_ptr().offset(1 as
                                                                           libc::c_int
                                                                           as
                                                                           isize))
           == 0 {
        return 0 as libc::c_int
    }
    tmp = (*objp).key_data_length[0 as libc::c_int as usize] as libc::c_uint;
    if gssrpc_xdr_bytes(xdrs,
                        &mut *(*objp).key_data_contents.as_mut_ptr().offset(0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)
                            as *mut *mut krb5_octet as *mut *mut libc::c_char,
                        &mut tmp, !(0 as libc::c_int) as u_int) == 0 {
        return 0 as libc::c_int
    }
    tmp = (*objp).key_data_length[1 as libc::c_int as usize] as libc::c_uint;
    if gssrpc_xdr_bytes(xdrs,
                        &mut *(*objp).key_data_contents.as_mut_ptr().offset(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)
                            as *mut *mut krb5_octet as *mut *mut libc::c_char,
                        &mut tmp, !(0 as libc::c_int) as u_int) == 0 {
        return 0 as libc::c_int
    }
    /* don't need to copy tmp out, since key_data_length will be set
       by the above encoding. */
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "51:1"]
pub unsafe extern "C" fn xdr_osa_pw_hist_ent(mut xdrs: *mut XDR,
                                             mut objp: *mut osa_pw_hist_ent)
 -> libc::c_int {
    if gssrpc_xdr_array(xdrs,
                        &mut (*objp).key_data as *mut *mut krb5_key_data as
                            *mut caddr_t,
                        &mut (*objp).n_key_data as *mut libc::c_int as
                            *mut u_int, !(0 as libc::c_int) as u_int,
                        ::std::mem::size_of::<krb5_key_data>() as
                            libc::c_ulong as u_int,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut krb5_key_data)
                                                           -> libc::c_int>,
                                                xdrproc_t>(Some(xdr_krb5_key_data
                                                                    as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut XDR,
                                                                                         _:
                                                                                             *mut krb5_key_data)
                                                                        ->
                                                                            libc::c_int)))
           == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "62:1"]
pub unsafe extern "C" fn xdr_osa_princ_ent_rec(mut xdrs: *mut XDR,
                                               mut objp: osa_princ_ent_t)
 -> libc::c_int {
    let mut current_block_6: u64;
    match (*xdrs).x_op as libc::c_uint {
        0 => {
            (*objp).version = 0x12345c01 as libc::c_int;
            current_block_6 = 13144677794153224956;
        }
        2 => { current_block_6 = 13144677794153224956; }
        1 => {
            if gssrpc_xdr_int(xdrs, &mut (*objp).version) == 0 {
                return 0 as libc::c_int
            }
            if (*objp).version != 0x12345c01 as libc::c_int {
                return 0 as libc::c_int
            }
            current_block_6 = 8515828400728868193;
        }
        _ => { current_block_6 = 8515828400728868193; }
    }
    match current_block_6 {
        13144677794153224956 =>
        /* fall through */
        {
            if gssrpc_xdr_int(xdrs, &mut (*objp).version) == 0 {
                return 0 as libc::c_int
            }
        }
        _ => { }
    }
    if xdr_nullstring(xdrs, &mut (*objp).policy) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_long(xdrs, &mut (*objp).aux_attributes) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_u_int(xdrs, &mut (*objp).old_key_next) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_kvno(xdrs, &mut (*objp).admin_history_kvno) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_array(xdrs,
                        &mut (*objp).old_keys as *mut *mut osa_pw_hist_ent as
                            *mut caddr_t,
                        &mut (*objp).old_key_len as *mut libc::c_uint,
                        !(0 as libc::c_int) as u_int,
                        ::std::mem::size_of::<osa_pw_hist_ent>() as
                            libc::c_ulong as u_int,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut osa_pw_hist_ent)
                                                           -> libc::c_int>,
                                                xdrproc_t>(Some(xdr_osa_pw_hist_ent
                                                                    as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut XDR,
                                                                                         _:
                                                                                             *mut osa_pw_hist_ent)
                                                                        ->
                                                                            libc::c_int)))
           == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* XXX this ought to be in libkrb5.a, but isn't */
/*
 * *Warning*
 * *Warning*        This is going to break if we
 * *Warning*        ever go multi-threaded
 * *Warning*
 */
/*
 * Why is this (or something similar) not defined *anywhere* in krb5?
 */
/*
 * all the various mask bits or'd together
 */
#[no_mangle]
#[c2rust::src_loc = "97:1"]
pub unsafe extern "C" fn osa_free_princ_ent(mut val: osa_princ_ent_t) {
    let mut xdrs: XDR =
        XDR{x_op: XDR_ENCODE,
            x_ops: 0 as *mut xdr_ops,
            x_public: 0 as *mut libc::c_char,
            x_private: 0 as *mut libc::c_void,
            x_base: 0 as *mut libc::c_char,
            x_handy: 0,};
    gssrpc_xdrmem_create(&mut xdrs, 0 as caddr_t, 0 as libc::c_int as u_int,
                         XDR_FREE);
    xdr_osa_princ_ent_rec(&mut xdrs, val);
    free(val as *mut libc::c_void);
}
