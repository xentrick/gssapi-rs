use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:46"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:46"]
pub mod types_h {
    #[c2rust::src_loc = "31:1"]
    pub type __u_char = libc::c_uchar;
    #[c2rust::src_loc = "32:1"]
    pub type __u_short = libc::c_ushort;
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "34:1"]
    pub type __u_long = libc::c_ulong;
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
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:46"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:46"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/sys/types.h:49"]
pub mod sys_types_h {
    #[c2rust::src_loc = "33:1"]
    pub type u_char = __u_char;
    #[c2rust::src_loc = "34:1"]
    pub type u_short = __u_short;
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "36:1"]
    pub type u_long = __u_long;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_char, __u_short, __u_int, __u_long, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:49"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:49"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:49"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:50"]
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "208:8"]
    pub struct xdr_discrim {
        pub value: libc::c_int,
        pub proc_0: xdrproc_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "289:8"]
    pub struct netobj {
        pub n_len: u_int,
        pub n_bytes: *mut libc::c_char,
    }
    use super::sys_types_h::{caddr_t, u_int};
    use super::gssrpc_types_h::rpc_inline_t;
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src = "/usr/include/stdio.h:46"]
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
    }
}
#[c2rust::header_src = "/usr/include/string.h:47"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:49"]
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
pub use self::stddef_h::size_t;
pub use self::types_h::{__u_char, __u_short, __u_int, __u_long, __int32_t,
                        __uint32_t, __off_t, __off64_t, __caddr_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::sys_types_h::{u_char, u_short, u_int, u_long, caddr_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssrpc_types_h::rpc_inline_t;
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, xdr_discrim, netobj};
use self::stdio_h::{stderr, fprintf};
use self::string_h::strlen;
use self::stdlib_h::{malloc, free};
#[c2rust::src_loc = "352:2"]
pub type sizecheck = libc::c_uint;
#[c2rust::src_loc = "352:19"]
pub const SIZEVAL: sizecheck = 0;
/*
 * for unit alignment
 */
#[c2rust::src_loc = "69:13"]
static mut xdr_zero: [libc::c_char; 4] =
    [0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char];
/* free memory buffers for xdr */
/*
 * Free a data structure using XDR
 * Not a filter, but a convenient utility nonetheless
 */
#[no_mangle]
#[c2rust::src_loc = "75:1"]
pub unsafe extern "C" fn gssrpc_xdr_free(mut proc_0: xdrproc_t,
                                         mut objp: *mut libc::c_void) {
    let mut x: XDR =
        XDR{x_op: XDR_ENCODE,
            x_ops: 0 as *mut xdr_ops,
            x_public: 0 as *mut libc::c_char,
            x_private: 0 as *mut libc::c_void,
            x_base: 0 as *mut libc::c_char,
            x_handy: 0,};
    x.x_op = XDR_FREE;
    ::std::mem::transmute::<_,
                            fn(_: _, _: _)
                                ->
                                    libc::c_int>(Some(proc_0.expect("non-null function pointer")).expect("non-null function pointer"))(&mut x,
                                                                                                                                       objp);
}
/*
 * In-line routines for fast encode/decode of primitve data types.
 * Caveat emptor: these use single memory cycles to get the
 * data from the underlying buffer, and will fail to operate
 * properly if the data is not aligned.  The standard way to use these
 * is to say:
 *	if ((buf = XDR_INLINE(xdrs, count)) == NULL)
 *		return (FALSE);
 *	<<< macro calls >>>
 * where ``count'' is the number of bytes of data occupied
 * by the primitive data types.
 *
 * N.B. and frozen for all time: each data type here uses 4 bytes
 * of external representation.
 */
/*
 * These are the "generic" xdr routines.
 */
/*
 * XDR nothing
 */
#[no_mangle]
#[c2rust::src_loc = "87:1"]
pub unsafe extern "C" fn gssrpc_xdr_void(mut xdrs: *mut XDR,
                                         mut addr: *mut libc::c_void)
 -> libc::c_int {
    return 1 as libc::c_int;
}
/*
 * XDR integers
 */
#[no_mangle]
#[c2rust::src_loc = "97:1"]
pub unsafe extern "C" fn gssrpc_xdr_int(mut xdrs: *mut XDR,
                                        mut ip: *mut libc::c_int)
 -> libc::c_int {
    let mut l: libc::c_long = 0;
    's_57:
        {
            match (*xdrs).x_op as libc::c_uint {
                0 => {
                    if *ip as libc::c_long > 0x7fffffff as libc::c_long ||
                           (*ip as libc::c_long) <
                               -(0x7fffffff as libc::c_long) -
                                   1 as libc::c_long {
                        return 0 as libc::c_int
                    }
                    l = *ip as libc::c_long;
                    return Some((*(*xdrs).x_ops).x_putlong.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                                    &mut l)
                }
                1 => {
                    if Some((*(*xdrs).x_ops).x_getlong.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                                &mut l)
                           == 0 {
                        return 0 as libc::c_int
                    }
                    if l > 2147483647 as libc::c_int as libc::c_long ||
                           l <
                               (-(2147483647 as libc::c_int) -
                                    1 as libc::c_int) as libc::c_long {
                        return 0 as libc::c_int
                    }
                    *ip = l as libc::c_int
                }
                2 => { }
                _ => { break 's_57 ; }
            }
            return 1 as libc::c_int
        }
    /*NOTREACHED*/
    return 0 as libc::c_int;
}
/*
 * XDR unsigned integers
 */
#[no_mangle]
#[c2rust::src_loc = "131:1"]
pub unsafe extern "C" fn gssrpc_xdr_u_int(mut xdrs: *mut XDR,
                                          mut up: *mut u_int) -> libc::c_int {
    let mut l: u_long = 0;
    match (*xdrs).x_op as libc::c_uint {
        0 => {
            if *up as libc::c_ulong > 0xffffffff as libc::c_ulong {
                return 0 as libc::c_int
            }
            l = *up as u_long;
            return Some((*(*xdrs).x_ops).x_putlong.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                            &mut l
                                                                                                                                as
                                                                                                                                *mut u_long
                                                                                                                                as
                                                                                                                                *mut libc::c_long)
        }
        1 => {
            if Some((*(*xdrs).x_ops).x_getlong.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                        &mut l
                                                                                                                            as
                                                                                                                            *mut u_long
                                                                                                                            as
                                                                                                                            *mut libc::c_long)
                   == 0 {
                return 0 as libc::c_int
            }
            if l as uint32_t >
                   (2147483647 as libc::c_int as
                        libc::c_uint).wrapping_mul(2 as
                                                       libc::c_uint).wrapping_add(1
                                                                                      as
                                                                                      libc::c_uint)
               {
                return 0 as libc::c_int
            }
            *up = l as u_int;
            return 1 as libc::c_int
        }
        2 => { return 1 as libc::c_int }
        _ => { }
    }
    /*NOTREACHED*/
    return 0 as libc::c_int;
}
/*
 * XDR long integers
 */
#[no_mangle]
#[c2rust::src_loc = "166:1"]
pub unsafe extern "C" fn gssrpc_xdr_long(mut xdrs: *mut XDR,
                                         mut lp: *mut libc::c_long)
 -> libc::c_int {
    match (*xdrs).x_op as libc::c_uint {
        0 => {
            if *lp > 0x7fffffff as libc::c_long ||
                   *lp < -(0x7fffffff as libc::c_long) - 1 as libc::c_long {
                return 0 as libc::c_int
            }
            return Some((*(*xdrs).x_ops).x_putlong.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                            lp)
        }
        1 => {
            return Some((*(*xdrs).x_ops).x_getlong.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                            lp)
        }
        2 => { return 1 as libc::c_int }
        _ => { }
    }
    return 0 as libc::c_int;
}
/*
 * XDR unsigned long integers
 */
#[no_mangle]
#[c2rust::src_loc = "190:1"]
pub unsafe extern "C" fn gssrpc_xdr_u_long(mut xdrs: *mut XDR,
                                           mut ulp: *mut u_long)
 -> libc::c_int {
    match (*xdrs).x_op as libc::c_uint {
        0 => {
            if *ulp > 0xffffffff as libc::c_ulong { return 0 as libc::c_int }
            return Some((*(*xdrs).x_ops).x_putlong.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                            ulp
                                                                                                                                as
                                                                                                                                *mut libc::c_long)
        }
        1 => {
            return Some((*(*xdrs).x_ops).x_getlong.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                            ulp
                                                                                                                                as
                                                                                                                                *mut libc::c_long)
        }
        2 => { return 1 as libc::c_int }
        _ => { }
    }
    return 0 as libc::c_int;
}
/*
 * XDR short integers
 */
#[no_mangle]
#[c2rust::src_loc = "214:1"]
pub unsafe extern "C" fn gssrpc_xdr_short(mut xdrs: *mut XDR,
                                          mut sp: *mut libc::c_short)
 -> libc::c_int {
    let mut l: libc::c_long = 0;
    match (*xdrs).x_op as libc::c_uint {
        0 => {
            l = *sp as libc::c_long;
            return Some((*(*xdrs).x_ops).x_putlong.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                            &mut l)
        }
        1 => {
            if Some((*(*xdrs).x_ops).x_getlong.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                        &mut l)
                   == 0 {
                return 0 as libc::c_int
            }
            if l > 32767 as libc::c_int as libc::c_long ||
                   l <
                       (-(32767 as libc::c_int) - 1 as libc::c_int) as
                           libc::c_long {
                return 0 as libc::c_int
            }
            *sp = l as libc::c_short;
            return 1 as libc::c_int
        }
        2 => { return 1 as libc::c_int }
        _ => { }
    }
    return 0 as libc::c_int;
}
/*
 * XDR unsigned short integers
 */
#[no_mangle]
#[c2rust::src_loc = "245:1"]
pub unsafe extern "C" fn gssrpc_xdr_u_short(mut xdrs: *mut XDR,
                                            mut usp: *mut u_short)
 -> libc::c_int {
    let mut l: u_long = 0;
    match (*xdrs).x_op as libc::c_uint {
        0 => {
            l = *usp as u_long;
            return Some((*(*xdrs).x_ops).x_putlong.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                            &mut l
                                                                                                                                as
                                                                                                                                *mut u_long
                                                                                                                                as
                                                                                                                                *mut libc::c_long)
        }
        1 => {
            if Some((*(*xdrs).x_ops).x_getlong.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                        &mut l
                                                                                                                            as
                                                                                                                            *mut u_long
                                                                                                                            as
                                                                                                                            *mut libc::c_long)
                   == 0 {
                return 0 as libc::c_int
            }
            *usp = l as u_short;
            return 1 as libc::c_int
        }
        2 => { return 1 as libc::c_int }
        _ => { }
    }
    return 0 as libc::c_int;
}
/*
 * XDR a char
 */
#[no_mangle]
#[c2rust::src_loc = "274:1"]
pub unsafe extern "C" fn gssrpc_xdr_char(mut xdrs: *mut XDR,
                                         mut cp: *mut libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    match (*xdrs).x_op as libc::c_uint { 0 | _ => { } }
    i = *cp as libc::c_int;
    if gssrpc_xdr_int(xdrs, &mut i) == 0 { return 0 as libc::c_int }
    *cp = i as libc::c_char;
    return 1 as libc::c_int;
}
/*
 * XDR an unsigned char
 */
#[no_mangle]
#[c2rust::src_loc = "297:1"]
pub unsafe extern "C" fn gssrpc_xdr_u_char(mut xdrs: *mut XDR,
                                           mut cp: *mut u_char)
 -> libc::c_int {
    let mut u: u_int = 0;
    match (*xdrs).x_op as libc::c_uint { 0 | _ => { } }
    u = *cp as u_int;
    if gssrpc_xdr_u_int(xdrs, &mut u) == 0 { return 0 as libc::c_int }
    *cp = u as u_char;
    return 1 as libc::c_int;
}
/*
 * XDR booleans
 */
#[no_mangle]
#[c2rust::src_loc = "320:1"]
pub unsafe extern "C" fn gssrpc_xdr_bool(mut xdrs: *mut XDR,
                                         mut bp: *mut libc::c_int)
 -> libc::c_int {
    let mut lb: libc::c_long = 0;
    match (*xdrs).x_op as libc::c_uint {
        0 => {
            lb =
                if *bp != 0 {
                    1 as libc::c_int as libc::c_long
                } else { 0 as libc::c_int as libc::c_long };
            return Some((*(*xdrs).x_ops).x_putlong.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                            &mut lb)
        }
        1 => {
            if Some((*(*xdrs).x_ops).x_getlong.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                        &mut lb)
                   == 0 {
                return 0 as libc::c_int
            }
            *bp =
                if lb == 0 as libc::c_int as libc::c_long {
                    0 as libc::c_int
                } else { 1 as libc::c_int };
            return 1 as libc::c_int
        }
        2 => { return 1 as libc::c_int }
        _ => { }
    }
    return 0 as libc::c_int;
}
/*
 * XDR enumerations
 */
#[no_mangle]
#[c2rust::src_loc = "348:1"]
pub unsafe extern "C" fn gssrpc_xdr_enum(mut xdrs: *mut XDR,
                                         mut ep: *mut libc::c_int)
 -> libc::c_int {
    match (*xdrs).x_op as libc::c_uint {
        0 | _ => { }
    } /* used to find the size of an enum */
    if ::std::mem::size_of::<sizecheck>() as libc::c_ulong ==
           ::std::mem::size_of::<libc::c_long>() as libc::c_ulong {
        return gssrpc_xdr_long(xdrs,
                               ep as *mut libc::c_void as *mut libc::c_long)
    } else if ::std::mem::size_of::<sizecheck>() as libc::c_ulong ==
                  ::std::mem::size_of::<libc::c_int>() as libc::c_ulong {
        return gssrpc_xdr_int(xdrs,
                              ep as *mut libc::c_void as *mut libc::c_int)
    } else if ::std::mem::size_of::<sizecheck>() as libc::c_ulong ==
                  ::std::mem::size_of::<libc::c_short>() as libc::c_ulong {
        return gssrpc_xdr_short(xdrs,
                                ep as *mut libc::c_void as *mut libc::c_short)
    } else { return 0 as libc::c_int };
}
/*
	 * enums are treated as ints
	 */
/*
 * XDR opaque data
 * Allows the specification of a fixed size sequence of opaque bytes.
 * cp points to the opaque object and cnt gives the byte length.
 */
#[no_mangle]
#[c2rust::src_loc = "384:1"]
pub unsafe extern "C" fn gssrpc_xdr_opaque(mut xdrs: *mut XDR,
                                           mut cp: caddr_t, mut cnt: u_int)
 -> libc::c_int {
    let mut rndup: u_int = 0;
    static mut crud: [libc::c_int; 4] = [0; 4];
    /*
	 * if no data we are done
	 */
    if cnt == 0 as libc::c_int as libc::c_uint { return 1 as libc::c_int }
    /*
	 * round byte count to full xdr units
	 */
    rndup = cnt.wrapping_rem(4 as libc::c_int as libc::c_uint);
    if rndup > 0 as libc::c_int as libc::c_uint {
        rndup = (4 as libc::c_int as libc::c_uint).wrapping_sub(rndup)
    }
    if (*xdrs).x_op as libc::c_uint ==
           XDR_DECODE as libc::c_int as libc::c_uint {
        if Some((*(*xdrs).x_ops).x_getbytes.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                     cp,
                                                                                                                     cnt)
               == 0 {
            return 0 as libc::c_int
        }
        if rndup == 0 as libc::c_int as libc::c_uint {
            return 1 as libc::c_int
        }
        return Some((*(*xdrs).x_ops).x_getbytes.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                         crud.as_mut_ptr()
                                                                                                                             as
                                                                                                                             *mut libc::c_void
                                                                                                                             as
                                                                                                                             caddr_t,
                                                                                                                         rndup)
    }
    if (*xdrs).x_op as libc::c_uint ==
           XDR_ENCODE as libc::c_int as libc::c_uint {
        if Some((*(*xdrs).x_ops).x_putbytes.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                     cp,
                                                                                                                     cnt)
               == 0 {
            return 0 as libc::c_int
        }
        if rndup == 0 as libc::c_int as libc::c_uint {
            return 1 as libc::c_int
        }
        return Some((*(*xdrs).x_ops).x_putbytes.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                         xdr_zero.as_mut_ptr(),
                                                                                                                         rndup)
    }
    if (*xdrs).x_op as libc::c_uint == XDR_FREE as libc::c_int as libc::c_uint
       {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/*
 * XDR counted bytes
 * *cpp is a pointer to the bytes, *sizep is the count.
 * If *cpp is NULL maxsize bytes are allocated
 */
#[no_mangle]
#[c2rust::src_loc = "434:1"]
pub unsafe extern "C" fn gssrpc_xdr_bytes(mut xdrs: *mut XDR,
                                          mut cpp: *mut *mut libc::c_char,
                                          mut sizep: *mut u_int,
                                          mut maxsize: u_int) -> libc::c_int {
    let mut sp: *mut libc::c_char =
        *cpp; /* sp is the actual string pointer */
    let mut nodesize: u_int = 0;
    /*
	 * first deal with the length since xdr bytes are counted
	 */
    if gssrpc_xdr_u_int(xdrs, sizep) == 0 { return 0 as libc::c_int }
    nodesize = *sizep;
    if nodesize > maxsize &&
           (*xdrs).x_op as libc::c_uint !=
               XDR_FREE as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    's_112:
        {
            /*
	 * now deal with the actual bytes
	 */
            match (*xdrs).x_op as libc::c_uint {
                1 => {
                    if nodesize == 0 as libc::c_int as libc::c_uint {
                        return 1 as libc::c_int
                    }
                    if sp.is_null() {
                        sp =
                            malloc(nodesize as libc::c_ulong) as
                                *mut libc::c_char;
                        *cpp = sp
                    }
                    if sp.is_null() {
                        fprintf(stderr,
                                b"xdr_bytes: out of memory\n\x00" as *const u8
                                    as *const libc::c_char);
                        return 0 as libc::c_int
                    }
                }
                0 => { }
                2 => {
                    if !sp.is_null() {
                        free(sp as *mut libc::c_void);
                        *cpp = 0 as *mut libc::c_char
                    }
                    return 1 as libc::c_int
                }
                _ => { break 's_112 ; }
            }
            /* fall into ... */
            return gssrpc_xdr_opaque(xdrs, sp, nodesize)
        }
    return 0 as libc::c_int;
}
/*
 * Implemented here due to commonality of the object.
 */
#[no_mangle]
#[c2rust::src_loc = "489:1"]
pub unsafe extern "C" fn gssrpc_xdr_netobj(mut xdrs: *mut XDR,
                                           mut np: *mut netobj)
 -> libc::c_int {
    return gssrpc_xdr_bytes(xdrs, &mut (*np).n_bytes, &mut (*np).n_len,
                            2048 as libc::c_int as u_int);
}
#[no_mangle]
#[c2rust::src_loc = "496:1"]
pub unsafe extern "C" fn gssrpc_xdr_int32(mut xdrs: *mut XDR,
                                          mut ip: *mut int32_t)
 -> libc::c_int {
    let mut l: libc::c_long = 0;
    match (*xdrs).x_op as libc::c_uint {
        0 => { l = *ip as libc::c_long; return gssrpc_xdr_long(xdrs, &mut l) }
        1 => {
            if gssrpc_xdr_long(xdrs, &mut l) == 0 { return 0 as libc::c_int }
            *ip = l as int32_t;
            return 1 as libc::c_int
        }
        2 => { return 1 as libc::c_int }
        _ => { }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "521:1"]
pub unsafe extern "C" fn gssrpc_xdr_u_int32(mut xdrs: *mut XDR,
                                            mut up: *mut uint32_t)
 -> libc::c_int {
    let mut ul: u_long = 0;
    match (*xdrs).x_op as libc::c_uint {
        0 => { ul = *up as u_long; return gssrpc_xdr_u_long(xdrs, &mut ul) }
        1 => {
            if gssrpc_xdr_u_long(xdrs, &mut ul) == 0 {
                return 0 as libc::c_int
            }
            *up = ul as uint32_t;
            return 1 as libc::c_int
        }
        2 => { return 1 as libc::c_int }
        _ => { }
    }
    return 0 as libc::c_int;
}
/*
 * XDR a descriminated union
 * Support routine for discriminated unions.
 * You create an array of xdrdiscrim structures, terminated with
 * an entry with a null procedure pointer.  The routine gets
 * the discriminant value and then searches the array of xdrdiscrims
 * looking for that value.  It calls the procedure given in the xdrdiscrim
 * to handle the discriminant.  If there is no specific routine a default
 * routine may be called.
 * If there is no specific or default routine an error is returned.
 */
#[no_mangle]
#[c2rust::src_loc = "557:1"]
pub unsafe extern "C" fn gssrpc_xdr_union(mut xdrs: *mut XDR,
                                          mut dscmp: *mut libc::c_int,
                                          mut unp: *mut libc::c_char,
                                          mut choices: *mut xdr_discrim,
                                          mut dfault: xdrproc_t)
 -> libc::c_int 
 /* default xdr routine */
 {
    let mut dscm: libc::c_int = 0;
    /*
	 * we deal with the discriminator;  it's an enum
	 */
    if gssrpc_xdr_enum(xdrs, dscmp) == 0 { return 0 as libc::c_int }
    dscm = *dscmp;
    /*
	 * search choices for a value that matches the discriminator.
	 * if we find one, execute the xdr routine for that value.
	 */
    while (*choices).proc_0.is_some() {
        if (*choices).value == dscm {
            return ::std::mem::transmute::<_,
                                           fn(_: _, _: _, _: _)
                                               ->
                                                   libc::c_int>(Some((*choices).proc_0.expect("non-null function pointer")).expect("non-null function pointer"))(xdrs,
                                                                                                                                                                 unp,
                                                                                                                                                                 (0
                                                                                                                                                                      as
                                                                                                                                                                      libc::c_int
                                                                                                                                                                      as
                                                                                                                                                                      u_int).wrapping_sub(1
                                                                                                                                                                                              as
                                                                                                                                                                                              libc::c_int
                                                                                                                                                                                              as
                                                                                                                                                                                              libc::c_uint))
        }
        choices = choices.offset(1)
    }
    /*
	 * no match - execute the default xdr routine if there is one
	 */
    return if dfault.is_none() {
               0 as libc::c_int
           } else {
               ::std::mem::transmute::<_,
                                       fn(_: _, _: _, _: _)
                                           ->
                                               libc::c_int>(Some(dfault.expect("non-null function pointer")).expect("non-null function pointer"))(xdrs,
                                                                                                                                                  unp,
                                                                                                                                                  (0
                                                                                                                                                       as
                                                                                                                                                       libc::c_int
                                                                                                                                                       as
                                                                                                                                                       u_int).wrapping_sub(1
                                                                                                                                                                               as
                                                                                                                                                                               libc::c_int
                                                                                                                                                                               as
                                                                                                                                                                               libc::c_uint))
           };
}
/*
 * Non-portable xdr primitives.
 * Care should be taken when moving these routines to new architectures.
 */
/*
 * XDR null terminated ASCII strings
 * xdr_string deals with "C strings" - arrays of bytes that are
 * terminated by a NULL character.  The parameter cpp references a
 * pointer to storage; If the pointer is null, then the necessary
 * storage is allocated.  The last parameter is the max allowed length
 * of the string as specified by a protocol.
 */
#[no_mangle]
#[c2rust::src_loc = "607:1"]
pub unsafe extern "C" fn gssrpc_xdr_string(mut xdrs: *mut XDR,
                                           mut cpp: *mut *mut libc::c_char,
                                           mut maxsize: u_int)
 -> libc::c_int {
    let mut sp: *mut libc::c_char =
        *cpp; /* sp is the actual string pointer */
    let mut size: u_int = 0;
    let mut nodesize: u_int = 0;
    let mut current_block_3: u64;
    /*
	 * first deal with the length since xdr strings are counted-strings
	 */
    match (*xdrs).x_op as libc::c_uint {
        2 => {
            if sp.is_null() {
                return 1 as libc::c_int
                /* already free */
            }
            current_block_3 = 1314968178206245084;
        }
        0 => { current_block_3 = 1314968178206245084; }
        1 | _ => { current_block_3 = 6937071982253665452; }
    }
    match current_block_3 {
        1314968178206245084 =>
        /* fall through... */
        {
            size = strlen(sp) as u_int
        }
        _ => { }
    }
    if gssrpc_xdr_u_int(xdrs, &mut size) == 0 { return 0 as libc::c_int }
    if size >= maxsize { return 0 as libc::c_int }
    nodesize = size.wrapping_add(1 as libc::c_int as libc::c_uint);
    's_137:
        {
            /*
	 * now deal with the actual bytes
	 */
            match (*xdrs).x_op as libc::c_uint {
                1 => {
                    if nodesize == 0 as libc::c_int as libc::c_uint {
                        return 1 as libc::c_int
                    }
                    if sp.is_null() {
                        sp =
                            malloc(nodesize as libc::c_ulong) as
                                *mut libc::c_char;
                        *cpp = sp
                    }
                    if sp.is_null() {
                        fprintf(stderr,
                                b"xdr_string: out of memory\n\x00" as
                                    *const u8 as *const libc::c_char);
                        return 0 as libc::c_int
                    }
                    *sp.offset(size as isize) =
                        0 as libc::c_int as libc::c_char
                }
                0 => { }
                2 => {
                    free(sp as *mut libc::c_void);
                    *cpp = 0 as *mut libc::c_char;
                    return 1 as libc::c_int
                }
                _ => { break 's_137 ; }
            }
            /* fall into ... */
            return gssrpc_xdr_opaque(xdrs, sp, size)
        }
    return 0 as libc::c_int;
}
/*
 * Wrapper for xdr_string that can be called directly from
 * routines like clnt_call
 */
#[no_mangle]
#[c2rust::src_loc = "670:1"]
pub unsafe extern "C" fn gssrpc_xdr_wrapstring(mut xdrs: *mut XDR,
                                               mut cpp:
                                                   *mut *mut libc::c_char)
 -> libc::c_int {
    if gssrpc_xdr_string(xdrs, cpp,
                         (0 as libc::c_int as
                              u_int).wrapping_sub(1 as libc::c_int as
                                                      libc::c_uint)) != 0 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
