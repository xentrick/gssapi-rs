use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:40"]
pub mod types_h {
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
}
#[c2rust::header_src = "/usr/include/sys/types.h:40"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:40"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:40"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:41"]
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
    #[c2rust::src_loc = "105:1"]
    pub type xdrproc_t = Option<unsafe extern "C" fn() -> libc::c_int>;
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
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src = "/usr/include/stdlib.h:40"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
pub use self::types_h::{__u_int, __int32_t, __caddr_t};
pub use self::sys_types_h::{u_int, caddr_t};
pub use self::stdint_intn_h::int32_t;
pub use self::gssrpc_types_h::rpc_inline_t;
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops};
use self::stdlib_h::{malloc, free};
#[c2rust::src_loc = "142:2"]
pub type dummyfunc2
    =
    Option<unsafe extern "C" fn(_: *mut XDR, _: caddr_t, _: u_int)
               -> libc::c_int>;
#[c2rust::src_loc = "141:2"]
pub type dummyfunc1
    =
    Option<unsafe extern "C" fn(_: *mut XDR, _: *mut libc::c_long)
               -> libc::c_int>;
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
/*
 * xdr_sizeof.c
 *
 * General purpose routine to see how much space something will use
 * when serialized using XDR.
 */
/* ARGSUSED */
#[c2rust::src_loc = "45:1"]
unsafe extern "C" fn x_putlong(mut xdrs: *mut XDR,
                               mut longp: *mut libc::c_long) -> libc::c_int {
    (*xdrs).x_handy += 4 as libc::c_int;
    return 1 as libc::c_int;
}
/* ARGSUSED */
#[c2rust::src_loc = "55:1"]
unsafe extern "C" fn x_putbytes(mut xdrs: *mut XDR, mut bp: *mut libc::c_char,
                                mut len: libc::c_int) -> libc::c_int {
    (*xdrs).x_handy += len;
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "66:1"]
unsafe extern "C" fn x_getpostn(mut xdrs: *mut XDR) -> u_int {
    return (*xdrs).x_handy as u_int;
}
/* ARGSUSED */
#[c2rust::src_loc = "74:1"]
unsafe extern "C" fn x_setpostn(mut xdrs: *mut XDR, mut pos: u_int)
 -> libc::c_int {
    /* This is not allowed */
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "83:1"]
unsafe extern "C" fn x_inline(mut xdrs: *mut XDR, mut len: libc::c_int)
 -> *mut rpc_inline_t {
    if len == 0 as libc::c_int { return 0 as *mut rpc_inline_t }
    if (*xdrs).x_op as libc::c_uint !=
           XDR_ENCODE as libc::c_int as libc::c_uint {
        return 0 as *mut rpc_inline_t
    }
    if len <
           ((*xdrs).x_private as caddr_t).wrapping_offset_from((*xdrs).x_base)
               as libc::c_long as libc::c_int {
        /* x_private was already allocated */
        (*xdrs).x_handy += len;
        return (*xdrs).x_private as *mut rpc_inline_t
    } else {
        /* Free the earlier space and allocate new area */
        if !(*xdrs).x_base.is_null() {
            free((*xdrs).x_base as *mut libc::c_void);
        }
        (*xdrs).x_base = malloc(len as libc::c_ulong) as caddr_t;
        if (*xdrs).x_base.is_null() {
            (*xdrs).x_private = 0 as *mut libc::c_void;
            return 0 as *mut rpc_inline_t
        }
        (*xdrs).x_private =
            (*xdrs).x_base.offset(len as isize) as *mut libc::c_void;
        (*xdrs).x_handy += len;
        return (*xdrs).x_base as *mut libc::c_void as *mut rpc_inline_t
    };
}
#[c2rust::src_loc = "112:1"]
unsafe extern "C" fn harmless() -> libc::c_int {
    /* Always return FALSE/NULL, as the case may be */
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "119:1"]
unsafe extern "C" fn x_destroy(mut xdrs: *mut XDR) {
    (*xdrs).x_handy = 0 as libc::c_int;
    (*xdrs).x_private = 0 as *mut libc::c_void;
    if !(*xdrs).x_base.is_null() {
        free((*xdrs).x_base as *mut libc::c_void);
        (*xdrs).x_base = 0 as caddr_t
    };
}
#[no_mangle]
#[c2rust::src_loc = "132:1"]
pub unsafe extern "C" fn gssrpc_xdr_sizeof(mut func: xdrproc_t,
                                           mut data: *mut libc::c_void)
 -> libc::c_ulong {
    let mut x: XDR =
        XDR{x_op: XDR_ENCODE,
            x_ops: 0 as *mut xdr_ops,
            x_public: 0 as *mut libc::c_char,
            x_private: 0 as *mut libc::c_void,
            x_base: 0 as *mut libc::c_char,
            x_handy: 0,};
    let mut ops: xdr_ops =
        xdr_ops{x_getlong: None,
                x_putlong: None,
                x_getbytes: None,
                x_putbytes: None,
                x_getpostn: None,
                x_setpostn: None,
                x_inline: None,
                x_destroy: None,};
    let mut stat: libc::c_int = 0;
    /* to stop ANSI-C compiler from complaining */
    ops.x_putlong =
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> libc::c_int>,
                                Option<unsafe extern "C" fn(_: *mut XDR,
                                                            _:
                                                                *mut libc::c_long)
                                           ->
                                               libc::c_int>>(Some(::std::mem::transmute::<unsafe extern "C" fn(_:
                                                                                                                   *mut XDR,
                                                                                                               _:
                                                                                                                   *mut libc::c_long)
                                                                                              ->
                                                                                                  libc::c_int,
                                                                                          unsafe extern "C" fn()
                                                                                              ->
                                                                                                  libc::c_int>(x_putlong)));
    ops.x_putbytes =
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> libc::c_int>,
                                Option<unsafe extern "C" fn(_: *mut XDR,
                                                            _: caddr_t,
                                                            _: u_int)
                                           ->
                                               libc::c_int>>(Some(::std::mem::transmute::<unsafe extern "C" fn(_:
                                                                                                                   *mut XDR,
                                                                                                               _:
                                                                                                                   *mut libc::c_char,
                                                                                                               _:
                                                                                                                   libc::c_int)
                                                                                              ->
                                                                                                  libc::c_int,
                                                                                          unsafe extern "C" fn()
                                                                                              ->
                                                                                                  libc::c_int>(x_putbytes)));
    ops.x_inline =
        ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                           -> *mut rpc_inline_t>,
                                Option<unsafe extern "C" fn(_: *mut XDR,
                                                            _: libc::c_int)
                                           ->
                                               *mut rpc_inline_t>>(Some(::std::mem::transmute::<unsafe extern "C" fn(_:
                                                                                                                         *mut XDR,
                                                                                                                     _:
                                                                                                                         libc::c_int)
                                                                                                    ->
                                                                                                        *mut rpc_inline_t,
                                                                                                unsafe extern "C" fn()
                                                                                                    ->
                                                                                                        *mut rpc_inline_t>(x_inline)));
    ops.x_getpostn =
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> u_int>,
                                Option<unsafe extern "C" fn(_: *mut XDR)
                                           ->
                                               u_int>>(Some(::std::mem::transmute::<unsafe extern "C" fn(_:
                                                                                                             *mut XDR)
                                                                                        ->
                                                                                            u_int,
                                                                                    unsafe extern "C" fn()
                                                                                        ->
                                                                                            u_int>(x_getpostn)));
    ops.x_setpostn =
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> libc::c_int>,
                                Option<unsafe extern "C" fn(_: *mut XDR,
                                                            _: u_int)
                                           ->
                                               libc::c_int>>(Some(::std::mem::transmute::<unsafe extern "C" fn(_:
                                                                                                                   *mut XDR,
                                                                                                               _:
                                                                                                                   u_int)
                                                                                              ->
                                                                                                  libc::c_int,
                                                                                          unsafe extern "C" fn()
                                                                                              ->
                                                                                                  libc::c_int>(x_setpostn)));
    ops.x_destroy =
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> ()>,
                                Option<unsafe extern "C" fn(_: *mut XDR)
                                           ->
                                               ()>>(Some(::std::mem::transmute::<unsafe extern "C" fn(_:
                                                                                                          *mut XDR)
                                                                                     ->
                                                                                         (),
                                                                                 unsafe extern "C" fn()
                                                                                     ->
                                                                                         ()>(x_destroy)));
    /* the other harmless ones */
    ops.x_getlong =
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> libc::c_int>,
                                dummyfunc1>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                             ->
                                                                                 libc::c_int,
                                                                         unsafe extern "C" fn()
                                                                             ->
                                                                                 libc::c_int>(harmless)));
    ops.x_getbytes =
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> libc::c_int>,
                                dummyfunc2>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                             ->
                                                                                 libc::c_int,
                                                                         unsafe extern "C" fn()
                                                                             ->
                                                                                 libc::c_int>(harmless)));
    x.x_op = XDR_ENCODE;
    x.x_ops = &mut ops;
    x.x_handy = 0 as libc::c_int;
    x.x_private = 0 as *mut libc::c_void as caddr_t as *mut libc::c_void;
    x.x_base = 0 as caddr_t;
    stat =
        ::std::mem::transmute::<_,
                                fn(_: _, _: _)
                                    ->
                                        libc::c_int>(func.expect("non-null function pointer"))(&mut x,
                                                                                               data);
    if !x.x_base.is_null() { free(x.x_base as *mut libc::c_void); }
    return if stat == 1 as libc::c_int {
               x.x_handy as libc::c_uint
           } else { 0 as libc::c_int as libc::c_uint } as libc::c_ulong;
}
