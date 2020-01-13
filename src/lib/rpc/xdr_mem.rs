use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:48"]
pub mod types_h {
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "34:1"]
    pub type __u_long = libc::c_ulong;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
}
#[c2rust::header_src = "/usr/include/sys/types.h:48"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "36:1"]
    pub type u_long = __u_long;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_int, __u_long, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:48"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:48"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:48"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:49"]
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
#[c2rust::header_src = "/usr/include/netinet/in.h:48"]
pub mod in_h {
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "378:1"]
        pub fn htonl(__hostlong: uint32_t) -> uint32_t;
        #[no_mangle]
        #[c2rust::src_loc = "375:1"]
        pub fn ntohl(__netlong: uint32_t) -> uint32_t;
    }
}
#[c2rust::header_src = "/usr/include/string.h:52"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "46:14"]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
    }
}
pub use self::types_h::{__u_int, __u_long, __int32_t, __uint32_t, __caddr_t};
pub use self::sys_types_h::{u_int, u_long, caddr_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssrpc_types_h::rpc_inline_t;
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, XDR, xdr_ops};
use self::in_h::{htonl, ntohl};
use self::string_h::memmove;
#[c2rust::src_loc = "64:23"]
static mut xdrmem_ops: xdr_ops =
    unsafe {
        {
            let mut init =
                xdr_ops{x_getlong:
                            Some(xdrmem_getlong as
                                     unsafe extern "C" fn(_: *mut XDR,
                                                          _:
                                                              *mut libc::c_long)
                                         -> libc::c_int),
                        x_putlong:
                            Some(xdrmem_putlong as
                                     unsafe extern "C" fn(_: *mut XDR,
                                                          _:
                                                              *mut libc::c_long)
                                         -> libc::c_int),
                        x_getbytes:
                            Some(xdrmem_getbytes as
                                     unsafe extern "C" fn(_: *mut XDR,
                                                          _: caddr_t,
                                                          _: u_int)
                                         -> libc::c_int),
                        x_putbytes:
                            Some(xdrmem_putbytes as
                                     unsafe extern "C" fn(_: *mut XDR,
                                                          _: caddr_t,
                                                          _: u_int)
                                         -> libc::c_int),
                        x_getpostn:
                            Some(xdrmem_getpos as
                                     unsafe extern "C" fn(_: *mut XDR)
                                         -> u_int),
                        x_setpostn:
                            Some(xdrmem_setpos as
                                     unsafe extern "C" fn(_: *mut XDR,
                                                          _: u_int)
                                         -> libc::c_int),
                        x_inline:
                            Some(xdrmem_inline as
                                     unsafe extern "C" fn(_: *mut XDR,
                                                          _: libc::c_int)
                                         -> *mut rpc_inline_t),
                        x_destroy:
                            Some(xdrmem_destroy as
                                     unsafe extern "C" fn(_: *mut XDR)
                                         -> ()),};
            init
        }
    };
/* XDR using memory buffers */
/*
 * The procedure xdrmem_create initializes a stream descriptor for a
 * memory buffer.
 */
#[no_mangle]
#[c2rust::src_loc = "79:1"]
pub unsafe extern "C" fn gssrpc_xdrmem_create(mut xdrs: *mut XDR,
                                              mut addr: caddr_t,
                                              mut size: u_int,
                                              mut op: xdr_op) {
    (*xdrs).x_op = op;
    (*xdrs).x_ops = &mut xdrmem_ops;
    (*xdrs).x_base = addr;
    (*xdrs).x_private = (*xdrs).x_base as *mut libc::c_void;
    (*xdrs).x_handy =
        if size > 2147483647 as libc::c_int as libc::c_uint {
            2147483647 as libc::c_int as libc::c_uint
        } else { size } as libc::c_int;
    /* XXX */
}
#[c2rust::src_loc = "93:1"]
unsafe extern "C" fn xdrmem_destroy(mut xdrs: *mut XDR) { }
/* @(#)xdr_mem.c	2.1 88/07/29 4.0 RPCSRC */
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
 * xdr_mem.h, XDR implementation using memory buffers.
 *
 * If you have some data to be interpreted as external data representation
 * or to be converted to external data representation in a memory buffer,
 * then this is the package for you.
 *
 */
#[c2rust::src_loc = "98:1"]
unsafe extern "C" fn xdrmem_getlong(mut xdrs: *mut XDR,
                                    mut lp: *mut libc::c_long)
 -> libc::c_int {
    if (*xdrs).x_handy < 4 as libc::c_int {
        return 0 as libc::c_int
    } else { (*xdrs).x_handy -= 4 as libc::c_int }
    *lp =
        ntohl(*((*xdrs).x_private as *mut uint32_t)) as int32_t as
            libc::c_long;
    (*xdrs).x_private =
        ((*xdrs).x_private as
             *mut libc::c_char).offset(4 as libc::c_int as isize) as
            *mut libc::c_void;
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "111:1"]
unsafe extern "C" fn xdrmem_putlong(mut xdrs: *mut XDR,
                                    mut lp: *mut libc::c_long)
 -> libc::c_int {
    if (*xdrs).x_handy < 4 as libc::c_int {
        return 0 as libc::c_int
    } else { (*xdrs).x_handy -= 4 as libc::c_int }
    *((*xdrs).x_private as *mut int32_t) = htonl(*lp as uint32_t) as int32_t;
    (*xdrs).x_private =
        ((*xdrs).x_private as
             *mut libc::c_char).offset(4 as libc::c_int as isize) as
            *mut libc::c_void;
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "124:1"]
unsafe extern "C" fn xdrmem_getbytes(mut xdrs: *mut XDR, mut addr: caddr_t,
                                     mut len: u_int) -> libc::c_int {
    if ((*xdrs).x_handy as u_int) < len {
        return 0 as libc::c_int
    } else {
        (*xdrs).x_handy =
            ((*xdrs).x_handy as libc::c_uint).wrapping_sub(len) as libc::c_int
                as libc::c_int
    }
    memmove(addr as *mut libc::c_void, (*xdrs).x_private,
            len as libc::c_ulong);
    (*xdrs).x_private =
        ((*xdrs).x_private as *mut libc::c_char).offset(len as isize) as
            *mut libc::c_void;
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "137:1"]
unsafe extern "C" fn xdrmem_putbytes(mut xdrs: *mut XDR, mut addr: caddr_t,
                                     mut len: u_int) -> libc::c_int {
    if ((*xdrs).x_handy as u_int) < len {
        return 0 as libc::c_int
    } else {
        (*xdrs).x_handy =
            ((*xdrs).x_handy as libc::c_uint).wrapping_sub(len) as libc::c_int
                as libc::c_int
    }
    memmove((*xdrs).x_private, addr as *const libc::c_void,
            len as libc::c_ulong);
    (*xdrs).x_private =
        ((*xdrs).x_private as *mut libc::c_char).offset(len as isize) as
            *mut libc::c_void;
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "150:1"]
unsafe extern "C" fn xdrmem_getpos(mut xdrs: *mut XDR) -> u_int {
    /*
 * 11/3/95 - JRG - Rather than recast everything for 64 bit, just convert
 * pointers to longs, then cast to int.
 */
    return ((*xdrs).x_private as
                u_long).wrapping_sub((*xdrs).x_base as u_long) as u_int;
}
#[c2rust::src_loc = "160:1"]
unsafe extern "C" fn xdrmem_setpos(mut xdrs: *mut XDR, mut pos: u_int)
 -> libc::c_int {
    let mut newaddr: caddr_t = (*xdrs).x_base.offset(pos as isize);
    let mut lastaddr: caddr_t =
        ((*xdrs).x_private as
             *mut libc::c_char).offset((*xdrs).x_handy as isize);
    if newaddr as libc::c_long > lastaddr as libc::c_long {
        return 0 as libc::c_int
    }
    (*xdrs).x_private = newaddr as *mut libc::c_void;
    (*xdrs).x_handy =
        (lastaddr as libc::c_long - newaddr as libc::c_long) as libc::c_int;
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "173:1"]
unsafe extern "C" fn xdrmem_inline(mut xdrs: *mut XDR, mut len: libc::c_int)
 -> *mut rpc_inline_t {
    let mut buf: *mut rpc_inline_t = 0 as *mut rpc_inline_t;
    if len >= 0 as libc::c_int && (*xdrs).x_handy >= len {
        (*xdrs).x_handy -= len;
        buf = (*xdrs).x_private as *mut rpc_inline_t;
        (*xdrs).x_private =
            ((*xdrs).x_private as *mut libc::c_char).offset(len as isize) as
                *mut libc::c_void
    }
    return buf;
}
