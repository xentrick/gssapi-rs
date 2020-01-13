use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:46"]
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
#[c2rust::header_src = "/usr/include/sys/types.h:46"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:46"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:46"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:46"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:46"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:48"]
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
#[c2rust::header_src = "/usr/include/netinet/in.h:46"]
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
#[c2rust::header_src = "/usr/include/stdio.h:47"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "218:1"]
        pub fn fflush(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "646:15"]
        pub fn fread(_: *mut libc::c_void, _: libc::c_ulong, _: libc::c_ulong,
                     _: *mut FILE) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "652:15"]
        pub fn fwrite(_: *const libc::c_void, _: libc::c_ulong,
                      _: libc::c_ulong, _: *mut FILE) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "684:1"]
        pub fn fseek(__stream: *mut FILE, __off: libc::c_long,
                     __whence: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "689:1"]
        pub fn ftell(__stream: *mut FILE) -> libc::c_long;
    }
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
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, XDR, xdr_ops};
use self::in_h::{ntohl, htonl};
use self::stdio_h::{fflush, fread, fwrite, fseek, ftell};
/*
 * Ops vector for stdio type XDR
 */
#[c2rust::src_loc = "62:23"]
static mut xdrstdio_ops: xdr_ops =
    unsafe {
        {
            let mut init =
                xdr_ops{x_getlong:
                            Some(xdrstdio_getlong as
                                     unsafe extern "C" fn(_: *mut XDR,
                                                          _:
                                                              *mut libc::c_long)
                                         -> libc::c_int),
                        x_putlong:
                            Some(xdrstdio_putlong as
                                     unsafe extern "C" fn(_: *mut XDR,
                                                          _:
                                                              *mut libc::c_long)
                                         -> libc::c_int),
                        x_getbytes:
                            Some(xdrstdio_getbytes as
                                     unsafe extern "C" fn(_: *mut XDR,
                                                          _: caddr_t,
                                                          _: u_int)
                                         -> libc::c_int),
                        x_putbytes:
                            Some(xdrstdio_putbytes as
                                     unsafe extern "C" fn(_: *mut XDR,
                                                          _: caddr_t,
                                                          _: u_int)
                                         -> libc::c_int),
                        x_getpostn:
                            Some(xdrstdio_getpos as
                                     unsafe extern "C" fn(_: *mut XDR)
                                         -> u_int),
                        x_setpostn:
                            Some(xdrstdio_setpos as
                                     unsafe extern "C" fn(_: *mut XDR,
                                                          _: u_int)
                                         -> libc::c_int),
                        x_inline:
                            Some(xdrstdio_inline as
                                     unsafe extern "C" fn(_: *mut XDR,
                                                          _: libc::c_int)
                                         -> *mut rpc_inline_t),
                        x_destroy:
                            Some(xdrstdio_destroy as
                                     unsafe extern "C" fn(_: *mut XDR)
                                         -> ()),};
            init
        }
    };
/* XDR using stdio library */
/*
 * Initialize a stdio xdr stream.
 * Sets the xdr stream handle xdrs for use on the stream file.
 * Operation flag is set to op.
 */
#[no_mangle]
#[c2rust::src_loc = "78:1"]
pub unsafe extern "C" fn gssrpc_xdrstdio_create(mut xdrs: *mut XDR,
                                                mut file: *mut FILE,
                                                mut op: xdr_op) {
    (*xdrs).x_op = op;
    (*xdrs).x_ops = &mut xdrstdio_ops;
    (*xdrs).x_private = file as caddr_t as *mut libc::c_void;
    (*xdrs).x_handy = 0 as libc::c_int;
    (*xdrs).x_base = 0 as caddr_t;
}
/*
 * Destroy a stdio xdr stream.
 * Cleans up the xdr stream handle xdrs previously set up by xdrstdio_create.
 */
#[c2rust::src_loc = "93:1"]
unsafe extern "C" fn xdrstdio_destroy(mut xdrs: *mut XDR) {
    fflush((*xdrs).x_private as *mut FILE);
    /* xx should we close the file ?? */
}
/* @(#)xdr_stdio.c	2.1 88/07/29 4.0 RPCSRC */
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
 * xdr_stdio.c, XDR implementation on standard i/o file.
 *
 * This set of routines implements a XDR on a stdio stream.
 * XDR_ENCODE serializes onto the stream, XDR_DECODE de-serializes
 * from the stream.
 */
#[c2rust::src_loc = "100:1"]
unsafe extern "C" fn xdrstdio_getlong(mut xdrs: *mut XDR,
                                      mut lp: *mut libc::c_long)
 -> libc::c_int {
    let mut tmp: uint32_t = 0;
    if fread(&mut tmp as *mut uint32_t as caddr_t as *mut libc::c_void,
             ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
             1 as libc::c_int as libc::c_ulong,
             (*xdrs).x_private as *mut FILE) !=
           1 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int
    }
    *lp = ntohl(tmp) as libc::c_long;
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "113:1"]
unsafe extern "C" fn xdrstdio_putlong(mut xdrs: *mut XDR,
                                      mut lp: *mut libc::c_long)
 -> libc::c_int {
    let mut mycopy: uint32_t = htonl(*lp as uint32_t);
    if fwrite(&mut mycopy as *mut uint32_t as caddr_t as *const libc::c_void,
              ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
              1 as libc::c_int as libc::c_ulong,
              (*xdrs).x_private as *mut FILE) !=
           1 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "123:1"]
unsafe extern "C" fn xdrstdio_getbytes(mut xdrs: *mut XDR, mut addr: caddr_t,
                                       mut len: u_int) -> libc::c_int {
    if len != 0 as libc::c_int as libc::c_uint &&
           fread(addr as *mut libc::c_void, len as size_t,
                 1 as libc::c_int as libc::c_ulong,
                 (*xdrs).x_private as *mut FILE) !=
               1 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "133:1"]
unsafe extern "C" fn xdrstdio_putbytes(mut xdrs: *mut XDR, mut addr: caddr_t,
                                       mut len: u_int) -> libc::c_int {
    if len != 0 as libc::c_int as libc::c_uint &&
           fwrite(addr as *const libc::c_void, len as size_t,
                  1 as libc::c_int as libc::c_ulong,
                  (*xdrs).x_private as *mut FILE) !=
               1 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "143:1"]
unsafe extern "C" fn xdrstdio_getpos(mut xdrs: *mut XDR) -> u_int {
    return ftell((*xdrs).x_private as *mut FILE) as u_int;
}
#[c2rust::src_loc = "150:1"]
unsafe extern "C" fn xdrstdio_setpos(mut xdrs: *mut XDR, mut pos: u_int)
 -> libc::c_int {
    return if fseek((*xdrs).x_private as *mut FILE, pos as libc::c_long,
                    0 as libc::c_int) < 0 as libc::c_int {
               0 as libc::c_int
           } else { 1 as libc::c_int };
}
#[c2rust::src_loc = "158:1"]
unsafe extern "C" fn xdrstdio_inline(mut xdrs: *mut XDR, mut len: libc::c_int)
 -> *mut rpc_inline_t {
    /*
	 * Must do some work to implement this: must insure
	 * enough data in the underlying stdio buffer,
	 * that the buffer is aligned so that we can indirect through a
	 * long *, and stuff this pointer in xdrs->x_buf.  Doing
	 * a fread or fwrite to a scratch buffer would defeat
	 * most of the gains to be had here and require storage
	 * management on this buffer, so we don't do this.
	 */
    return 0 as *mut rpc_inline_t;
}
