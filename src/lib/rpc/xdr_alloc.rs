use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:38"]
pub mod types_h {
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
}
#[c2rust::header_src = "/usr/include/sys/types.h:38"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:38"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:38"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:38"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:39"]
pub mod xdr_h {
    #[c2rust::src_loc = "81:1"]
    pub type xdr_op = libc::c_uint;
    #[c2rust::src_loc = "84:2"]
    pub const XDR_FREE: xdr_op = 2;
    #[c2rust::src_loc = "83:2"]
    pub const XDR_DECODE: xdr_op = 1;
    #[c2rust::src_loc = "82:2"]
    pub const XDR_ENCODE: xdr_op = 0;
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
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/lib/rpc/dyn.h:40"]
pub mod dyn_h {
    #[c2rust::src_loc = "27:1"]
    pub type DynObject = *mut _DynObject;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:16"]
    pub struct _DynObject {
        pub array: DynPtr,
        pub el_size: libc::c_int,
        pub num_el: libc::c_int,
        pub size: libc::c_int,
        pub inc: libc::c_int,
        pub debug: libc::c_int,
        pub paranoid: libc::c_int,
        pub initzero: libc::c_int,
    }
    /*
 * This file is part of libdyn.a, the C Dynamic Object library.  It
 * contains the public header file.
 *
 * There are no restrictions on this code; however, if you make any
 * changes, I request that you document them so that I do not get
 * credit or blame for your modifications.
 *
 * Written by Barr3y Jaspan, Student Information Processing Board (SIPB)
 * and MIT-Project Athena, 1989.
 *
 * 2002-07-17 Moved here from util/dyn; for old changes see dyn.c.
 *            Added macros to rename exposed symbols.  For newer changes
 *            see ChangeLog in the current directory.
 */
    /*
 * dyn.h -- header file to be included by programs linking against
 * libdyn.a.
 */
    #[c2rust::src_loc = "26:1"]
    pub type DynPtr = *mut libc::c_char;
    extern "C" {
        /*@null@*/
        /*@only@*/
        #[no_mangle]
        #[c2rust::src_loc = "60:22"]
        pub fn gssrpcint_DynCreate(el_size: libc::c_int, inc: libc::c_int)
         -> DynObject;
        #[no_mangle]
        #[c2rust::src_loc = "62:1"]
        pub fn gssrpcint_DynRelease(obj: DynObject) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "62:1"]
        pub fn gssrpcint_DynDestroy(obj: DynObject) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "65:1"]
        pub fn gssrpcint_DynInsert(obj: DynObject, idx: libc::c_int,
                                   els: *mut libc::c_void, num: libc::c_int)
         -> libc::c_int;
        /*@dependent@*/
        /*@null@*/
        #[no_mangle]
        #[c2rust::src_loc = "67:27"]
        pub fn gssrpcint_DynGet(obj: DynObject, num: libc::c_int) -> DynPtr;
        #[no_mangle]
        #[c2rust::src_loc = "72:1"]
        pub fn gssrpcint_DynSize(obj: DynObject) -> libc::c_int;
    }
    /* DO NOT ADD ANYTHING AFTER THIS #endif */
    /* _Dyn_h */
}
#[c2rust::header_src = "/usr/include/netinet/in.h:38"]
pub mod in_h {
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "378:1"]
        pub fn htonl(__hostlong: uint32_t) -> uint32_t;
    }
}
pub use self::types_h::{__u_int, __int32_t, __uint32_t, __caddr_t};
pub use self::sys_types_h::{u_int, caddr_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssrpc_types_h::rpc_inline_t;
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, XDR, xdr_ops};
pub use self::dyn_h::{DynObject, _DynObject, DynPtr, gssrpcint_DynCreate,
                      gssrpcint_DynRelease, gssrpcint_DynDestroy,
                      gssrpcint_DynInsert, gssrpcint_DynGet,
                      gssrpcint_DynSize};
use self::in_h::htonl;
#[c2rust::src_loc = "50:23"]
static mut xdralloc_ops: xdr_ops =
    unsafe {
        {
            let mut init =
                xdr_ops{x_getlong:
                            Some(xdralloc_notsup_getlong as
                                     unsafe extern "C" fn(_: *mut XDR,
                                                          _:
                                                              *mut libc::c_long)
                                         -> libc::c_int),
                        x_putlong:
                            Some(xdralloc_putlong as
                                     unsafe extern "C" fn(_: *mut XDR,
                                                          _:
                                                              *mut libc::c_long)
                                         -> libc::c_int),
                        x_getbytes:
                            Some(xdralloc_notsup_getbytes as
                                     unsafe extern "C" fn(_: *mut XDR,
                                                          _: caddr_t,
                                                          _: libc::c_uint)
                                         -> libc::c_int),
                        x_putbytes:
                            Some(xdralloc_putbytes as
                                     unsafe extern "C" fn(_: *mut XDR,
                                                          _: caddr_t,
                                                          _: libc::c_uint)
                                         -> libc::c_int),
                        x_getpostn:
                            Some(xdralloc_getpos as
                                     unsafe extern "C" fn(_: *mut XDR)
                                         -> libc::c_uint),
                        x_setpostn:
                            Some(xdralloc_notsup_setpos as
                                     unsafe extern "C" fn(_: *mut XDR,
                                                          _: libc::c_uint)
                                         -> libc::c_int),
                        x_inline:
                            Some(xdralloc_inline as
                                     unsafe extern "C" fn(_: *mut XDR,
                                                          _: libc::c_int)
                                         -> *mut rpc_inline_t),
                        x_destroy:
                            Some(xdralloc_destroy as
                                     unsafe extern "C" fn(_: *mut XDR)
                                         -> ()),};
            init
        }
    };
/*
 * The procedure xdralloc_create initializes a stream descriptor for a
 * memory buffer.
 */
#[no_mangle]
#[c2rust::src_loc = "65:1"]
pub unsafe extern "C" fn gssrpc_xdralloc_create(mut xdrs: *mut XDR,
                                                mut op: xdr_op) {
    (*xdrs).x_op = op;
    (*xdrs).x_ops = &mut xdralloc_ops;
    (*xdrs).x_private =
        gssrpcint_DynCreate(::std::mem::size_of::<libc::c_char>() as
                                libc::c_ulong as libc::c_int,
                            -(4 as libc::c_int)) as caddr_t as
            *mut libc::c_void;
    /* not allowed to fail */
}
/* get buffer from xdralloc */
#[no_mangle]
#[c2rust::src_loc = "73:1"]
pub unsafe extern "C" fn gssrpc_xdralloc_getdata(mut xdrs: *mut XDR)
 -> caddr_t {
    return gssrpcint_DynGet((*xdrs).x_private as DynObject, 0 as libc::c_int);
}
/*
 * These are the public routines for the various implementations of
 * xdr streams.
 */
/* XDR allocating memory buffer */
/* destroy xdralloc, save buf */
#[no_mangle]
#[c2rust::src_loc = "78:1"]
pub unsafe extern "C" fn gssrpc_xdralloc_release(mut xdrs: *mut XDR) {
    gssrpcint_DynRelease((*xdrs).x_private as DynObject);
}
#[c2rust::src_loc = "83:1"]
unsafe extern "C" fn xdralloc_destroy(mut xdrs: *mut XDR) {
    gssrpcint_DynDestroy((*xdrs).x_private as DynObject);
}
#[c2rust::src_loc = "88:1"]
unsafe extern "C" fn xdralloc_notsup_getlong(mut xdrs: *mut XDR,
                                             mut lp: *mut libc::c_long)
 -> libc::c_int {
    return 0 as libc::c_int;
}
/* lib/rpc/xdr_alloc.c */
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
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved.
 */
#[c2rust::src_loc = "95:1"]
unsafe extern "C" fn xdralloc_putlong(mut xdrs: *mut XDR,
                                      mut lp: *mut libc::c_long)
 -> libc::c_int {
    let mut l: libc::c_int =
        htonl(*lp as uint32_t) as libc::c_int; /* XXX need bounds checking */
    /* XXX assumes sizeof(int)==4 */
    if gssrpcint_DynInsert((*xdrs).x_private as DynObject,
                           gssrpcint_DynSize((*xdrs).x_private as DynObject),
                           &mut l as *mut libc::c_int as *mut libc::c_void,
                           ::std::mem::size_of::<libc::c_int>() as
                               libc::c_ulong as libc::c_int) !=
           -(1000 as libc::c_int) {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "110:1"]
unsafe extern "C" fn xdralloc_notsup_getbytes(mut xdrs: *mut XDR,
                                              mut addr: caddr_t,
                                              mut len: libc::c_uint)
 -> libc::c_int {
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "119:1"]
unsafe extern "C" fn xdralloc_putbytes(mut xdrs: *mut XDR, mut addr: caddr_t,
                                       mut len: libc::c_uint) -> libc::c_int {
    if gssrpcint_DynInsert((*xdrs).x_private as DynObject,
                           gssrpcint_DynSize((*xdrs).x_private as DynObject),
                           addr as *mut libc::c_void, len as libc::c_int) !=
           -(1000 as libc::c_int) {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "131:1"]
unsafe extern "C" fn xdralloc_getpos(mut xdrs: *mut XDR) -> libc::c_uint {
    return gssrpcint_DynSize((*xdrs).x_private as DynObject) as libc::c_uint;
}
#[c2rust::src_loc = "136:1"]
unsafe extern "C" fn xdralloc_notsup_setpos(mut xdrs: *mut XDR,
                                            mut lp: libc::c_uint)
 -> libc::c_int {
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "145:1"]
unsafe extern "C" fn xdralloc_inline(mut xdrs: *mut XDR, mut len: libc::c_int)
 -> *mut rpc_inline_t {
    return 0 as *mut rpc_inline_t;
}
