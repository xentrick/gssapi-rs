use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:53"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:53"]
pub mod types_h {
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
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:53"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:53"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/sys/types.h:54"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "36:1"]
    pub type u_long = __u_long;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_int, __u_long, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:54"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:54"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:54"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:55"]
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
#[c2rust::header_src = "/usr/include/stdio.h:53"]
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
#[c2rust::header_src = "/usr/include/netinet/in.h:54"]
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
#[c2rust::header_src = "/usr/include/stdlib.h:54"]
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
#[c2rust::header_src = "/usr/include/string.h:59"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "46:14"]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__u_int, __u_long, __int32_t, __uint32_t, __off_t,
                        __off64_t, __caddr_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::sys_types_h::{u_int, u_long, caddr_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssrpc_types_h::rpc_inline_t;
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, XDR, xdr_ops};
use self::stdio_h::{stderr, fprintf};
use self::in_h::{ntohl, htonl};
use self::stdlib_h::{malloc, free};
use self::string_h::memmove;
#[c2rust::src_loc = "96:1"]
pub type RECSTREAM = rec_strm;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "96:16"]
pub struct rec_strm {
    pub tcp_handle: caddr_t,
    pub the_buffer: caddr_t,
    pub writeit: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub out_base: caddr_t,
    pub out_finger: caddr_t,
    pub out_boundry: caddr_t,
    pub frag_header: *mut uint32_t,
    pub frag_sent: libc::c_int,
    pub readit: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub in_size: uint32_t,
    pub in_base: caddr_t,
    pub in_finger: caddr_t,
    pub in_boundry: caddr_t,
    pub fbtbc: int32_t,
    pub last_frag: libc::c_int,
    pub sendsize: u_int,
    pub recvsize: u_int,
}
#[c2rust::src_loc = "70:24"]
static mut xdrrec_ops: xdr_ops =
    unsafe {
        {
            let mut init =
                xdr_ops{x_getlong:
                            Some(xdrrec_getlong as
                                     unsafe extern "C" fn(_: *mut XDR,
                                                          _:
                                                              *mut libc::c_long)
                                         -> libc::c_int),
                        x_putlong:
                            Some(xdrrec_putlong as
                                     unsafe extern "C" fn(_: *mut XDR,
                                                          _:
                                                              *mut libc::c_long)
                                         -> libc::c_int),
                        x_getbytes:
                            Some(xdrrec_getbytes as
                                     unsafe extern "C" fn(_: *mut XDR,
                                                          _: caddr_t,
                                                          _: u_int)
                                         -> libc::c_int),
                        x_putbytes:
                            Some(xdrrec_putbytes as
                                     unsafe extern "C" fn(_: *mut XDR,
                                                          _: caddr_t,
                                                          _: u_int)
                                         -> libc::c_int),
                        x_getpostn:
                            Some(xdrrec_getpos as
                                     unsafe extern "C" fn(_: *mut XDR)
                                         -> u_int),
                        x_setpostn:
                            Some(xdrrec_setpos as
                                     unsafe extern "C" fn(_: *mut XDR,
                                                          _: u_int)
                                         -> libc::c_int),
                        x_inline:
                            Some(xdrrec_inline as
                                     unsafe extern "C" fn(_: *mut XDR,
                                                          _: libc::c_int)
                                         -> *mut rpc_inline_t),
                        x_destroy:
                            Some(xdrrec_destroy as
                                     unsafe extern "C" fn(_: *mut XDR)
                                         -> ()),};
            init
        }
    };
/*
	 * out-goung bits
	 */
/* output buffer (points to frag header) */
/* next output position */
/* data cannot up to this address */
/* beginning of curren fragment */
/* true if buffer sent in middle of record */
/*
	 * in-coming bits
	 */
/* fixed size of the input buffer */
/* location of next byte to be had */
/* can read up to this location */
/* fragment bytes to be consumed */
/* XDR pseudo records for tcp */
/*
 * Create an xdr handle for xdrrec
 * xdrrec_create fills in xdrs.  Sendsize and recvsize are
 * send and recv buffer sizes (0 => use default).
 * tcp_handle is an opaque handle that is passed as the first parameter to
 * the procedures readit and writeit.  Readit and writeit are read and
 * write respectively.   They are like the system
 * calls expect that they take an opaque handle rather than an fd.
 */
#[no_mangle]
#[c2rust::src_loc = "137:1"]
pub unsafe extern "C" fn gssrpc_xdrrec_create(mut xdrs: *mut XDR,
                                              mut sendsize: u_int,
                                              mut recvsize: u_int,
                                              mut tcp_handle: caddr_t,
                                              mut readit:
                                                  Option<unsafe extern "C" fn()
                                                             -> libc::c_int>,
                                              mut writeit:
                                                  Option<unsafe extern "C" fn()
                                                             -> libc::c_int>) 
 /* like write, but pass it a tcp_handle, not sock */
 {
    let mut rstrm: *mut RECSTREAM =
        malloc(::std::mem::size_of::<RECSTREAM>() as libc::c_ulong) as
            *mut RECSTREAM;
    if rstrm.is_null() {
        fprintf(stderr,
                b"xdrrec_create: out of memory\n\x00" as *const u8 as
                    *const libc::c_char);
        /*
		 *  This is bad.  Should rework xdrrec_create to
		 *  return a handle, and in this case return NULL
		 */
        return
    }
    /*
	 * adjust sizes and allocate buffer quad byte aligned
	 */
    sendsize = fix_buf_size(sendsize);
    (*rstrm).sendsize = sendsize;
    recvsize = fix_buf_size(recvsize);
    (*rstrm).recvsize = recvsize;
    (*rstrm).the_buffer =
        malloc(sendsize.wrapping_add(recvsize).wrapping_add(4 as libc::c_int
                                                                as
                                                                libc::c_uint)
                   as libc::c_ulong) as caddr_t;
    if (*rstrm).the_buffer.is_null() {
        fprintf(stderr,
                b"xdrrec_create: out of memory\n\x00" as *const u8 as
                    *const libc::c_char);
        return
    }
    (*rstrm).out_base = (*rstrm).the_buffer;
    while ((*rstrm).out_base as
               u_long).wrapping_rem(4 as libc::c_int as libc::c_ulong) !=
              0 as libc::c_int as libc::c_ulong {
        (*rstrm).out_base = (*rstrm).out_base.offset(1)
    }
    (*rstrm).in_base = (*rstrm).out_base.offset(sendsize as isize);
    /*
	 * now the rest ...
	 */
    (*xdrs).x_ops = &mut xdrrec_ops;
    (*xdrs).x_private = rstrm as caddr_t as *mut libc::c_void;
    (*rstrm).tcp_handle = tcp_handle;
    (*rstrm).readit = readit;
    (*rstrm).writeit = writeit;
    (*rstrm).out_boundry = (*rstrm).out_base;
    (*rstrm).out_finger = (*rstrm).out_boundry;
    (*rstrm).frag_header =
        (*rstrm).out_base as *mut libc::c_void as *mut uint32_t;
    (*rstrm).out_finger =
        (*rstrm).out_finger.offset(4 as libc::c_int as isize);
    (*rstrm).out_boundry = (*rstrm).out_boundry.offset(sendsize as isize);
    (*rstrm).frag_sent = 0 as libc::c_int;
    (*rstrm).in_size = recvsize;
    (*rstrm).in_boundry = (*rstrm).in_base;
    (*rstrm).in_boundry = (*rstrm).in_boundry.offset(recvsize as isize);
    (*rstrm).in_finger = (*rstrm).in_boundry;
    (*rstrm).fbtbc = 0 as libc::c_int;
    (*rstrm).last_frag = 1 as libc::c_int;
}
/* @(#)xdr_rec.c	2.2 88/08/01 4.0 RPCSRC */
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
 * xdr_rec.c, Implements TCP/IP based XDR streams with a "record marking"
 * layer above tcp (for rpc's use).
 *
 * These routines interface XDRSTREAMS to a tcp/ip connection.
 * There is a record marking layer between the xdr stream
 * and the tcp transport level.  A record is composed on one or more
 * record fragments.  A record fragment is a thirty-two bit header followed
 * by n bytes of data, where n is contained in the header.  The header
 * is represented as a htonl(uint32_t).  Thegh order bit encodes
 * whether or not the fragment is the last fragment of the record
 * (1 => fragment is last, 0 => more fragments to follow.
 * The other 31 bits encode the byte length of the fragment.
 */
/*
 * The reoutines defined below are the xdr ops which will go into the
 * xdr handle filled in by xdrrec_create.
 */
#[c2rust::src_loc = "198:1"]
unsafe extern "C" fn xdrrec_getlong(mut xdrs: *mut XDR,
                                    mut lp: *mut libc::c_long)
 -> libc::c_int {
    let mut rstrm: *mut RECSTREAM = (*xdrs).x_private as *mut RECSTREAM;
    let mut buflp: *mut int32_t =
        (*rstrm).in_finger as *mut libc::c_void as *mut int32_t;
    let mut mylong: uint32_t = 0;
    /* first try the inline, fast case */
    if (*rstrm).fbtbc >= 4 as libc::c_int &&
           (*rstrm).in_boundry as libc::c_long - buflp as libc::c_long >=
               4 as libc::c_int as libc::c_long {
        *lp = ntohl(*buflp as uint32_t) as libc::c_long;
        (*rstrm).fbtbc -= 4 as libc::c_int;
        (*rstrm).in_finger =
            (*rstrm).in_finger.offset(4 as libc::c_int as isize)
    } else {
        if xdrrec_getbytes(xdrs, &mut mylong as *mut uint32_t as caddr_t,
                           4 as libc::c_int as u_int) == 0 {
            return 0 as libc::c_int
        }
        *lp = ntohl(mylong) as int32_t as libc::c_long
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "221:1"]
unsafe extern "C" fn xdrrec_putlong(mut xdrs: *mut XDR,
                                    mut lp: *mut libc::c_long)
 -> libc::c_int {
    let mut rstrm: *mut RECSTREAM = (*xdrs).x_private as *mut RECSTREAM;
    let mut dest_lp: *mut int32_t =
        (*rstrm).out_finger as *mut libc::c_void as *mut int32_t;
    if ((*rstrm).out_boundry.wrapping_offset_from((*rstrm).out_finger) as
            libc::c_long) < 4 as libc::c_int as libc::c_long {
        /*
		 * this case should almost never happen so the code is
		 * inefficient
		 */
        (*rstrm).frag_sent = 1 as libc::c_int;
        if flush_out(rstrm, 0 as libc::c_int) == 0 { return 0 as libc::c_int }
        dest_lp = (*rstrm).out_finger as *mut libc::c_void as *mut int32_t
    }
    (*rstrm).out_finger =
        (*rstrm).out_finger.offset(4 as libc::c_int as isize);
    *dest_lp = htonl(*lp as uint32_t) as int32_t;
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "242:1"]
unsafe extern "C" fn xdrrec_getbytes(mut xdrs: *mut XDR, mut addr: caddr_t,
                                     mut len: u_int) -> libc::c_int {
    let mut rstrm: *mut RECSTREAM = (*xdrs).x_private as *mut RECSTREAM;
    let mut current: u_int = 0;
    while len > 0 as libc::c_int as libc::c_uint {
        current = (*rstrm).fbtbc as u_int;
        if current == 0 as libc::c_int as libc::c_uint {
            if (*rstrm).last_frag != 0 { return 0 as libc::c_int }
            if set_input_fragment(rstrm) == 0 { return 0 as libc::c_int }
        } else {
            current = if len < current { len } else { current };
            if get_input_bytes(rstrm, addr, current as libc::c_int) == 0 {
                return 0 as libc::c_int
            }
            addr = addr.offset(current as isize);
            (*rstrm).fbtbc =
                ((*rstrm).fbtbc as libc::c_uint).wrapping_sub(current) as
                    int32_t as int32_t;
            len =
                (len as libc::c_uint).wrapping_sub(current) as u_int as u_int
        }
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "267:1"]
unsafe extern "C" fn xdrrec_putbytes(mut xdrs: *mut XDR, mut addr: caddr_t,
                                     mut len: u_int) -> libc::c_int {
    let mut rstrm: *mut RECSTREAM = (*xdrs).x_private as *mut RECSTREAM;
    let mut current: size_t = 0;
    while len > 0 as libc::c_int as libc::c_uint {
        current =
            ((*rstrm).out_boundry as libc::c_long -
                 (*rstrm).out_finger as libc::c_long) as size_t;
        current =
            if (len as libc::c_ulong) < current {
                len as libc::c_ulong
            } else { current };
        memmove((*rstrm).out_finger as *mut libc::c_void,
                addr as *const libc::c_void, current);
        (*rstrm).out_finger = (*rstrm).out_finger.offset(current as isize);
        addr = addr.offset(current as isize);
        len = (len as libc::c_ulong).wrapping_sub(current) as u_int as u_int;
        if (*rstrm).out_finger == (*rstrm).out_boundry {
            (*rstrm).frag_sent = 1 as libc::c_int;
            if flush_out(rstrm, 0 as libc::c_int) == 0 {
                return 0 as libc::c_int
            }
        }
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "290:1"]
unsafe extern "C" fn xdrrec_getpos(mut xdrs: *mut XDR) -> u_int {
    let mut rstrm: *mut RECSTREAM = (*xdrs).x_private as *mut RECSTREAM;
    let mut pos: libc::c_int = 0;
    match (*xdrs).x_op as libc::c_uint {
        0 => {
            pos =
                ((*rstrm).out_finger.wrapping_offset_from((*rstrm).out_base)
                     as libc::c_long - 4 as libc::c_int as libc::c_long) as
                    libc::c_int
        }
        1 => {
            pos =
                ((*rstrm).in_boundry.wrapping_offset_from((*rstrm).in_finger)
                     as libc::c_long - 4 as libc::c_int as libc::c_long) as
                    libc::c_int
        }
        _ => { pos = -(1 as libc::c_int) }
    }
    return pos as u_int;
}
#[c2rust::src_loc = "315:1"]
unsafe extern "C" fn xdrrec_setpos(mut xdrs: *mut XDR, mut pos: u_int)
 -> libc::c_int {
    let mut rstrm: *mut RECSTREAM = (*xdrs).x_private as *mut RECSTREAM;
    let mut currpos: u_int = xdrrec_getpos(xdrs);
    let mut delta: libc::c_int = currpos.wrapping_sub(pos) as libc::c_int;
    let mut newpos: caddr_t = 0 as *mut libc::c_char;
    if currpos as libc::c_int != -(1 as libc::c_int) {
        match (*xdrs).x_op as libc::c_uint {
            0 => {
                newpos = (*rstrm).out_finger.offset(-(delta as isize));
                if newpos > (*rstrm).frag_header as caddr_t &&
                       newpos < (*rstrm).out_boundry {
                    (*rstrm).out_finger = newpos;
                    return 1 as libc::c_int
                }
            }
            1 => {
                newpos = (*rstrm).in_finger.offset(-(delta as isize));
                if delta < (*rstrm).fbtbc && newpos <= (*rstrm).in_boundry &&
                       newpos >= (*rstrm).in_base {
                    (*rstrm).in_finger = newpos;
                    (*rstrm).fbtbc -= delta;
                    return 1 as libc::c_int
                }
            }
            2 | _ => { }
        }
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "352:1"]
unsafe extern "C" fn xdrrec_inline(mut xdrs: *mut XDR, mut len: libc::c_int)
 -> *mut rpc_inline_t {
    let mut rstrm: *mut RECSTREAM = (*xdrs).x_private as *mut RECSTREAM;
    let mut buf: *mut rpc_inline_t = 0 as *mut rpc_inline_t;
    if len < 0 as libc::c_int { return 0 as *mut rpc_inline_t }
    match (*xdrs).x_op as libc::c_uint {
        0 => {
            if len as libc::c_long <=
                   (*rstrm).out_boundry.wrapping_offset_from((*rstrm).out_finger)
                       as libc::c_long {
                buf =
                    (*rstrm).out_finger as *mut libc::c_void as
                        *mut rpc_inline_t;
                (*rstrm).out_finger = (*rstrm).out_finger.offset(len as isize)
            }
        }
        1 => {
            if len <= (*rstrm).fbtbc &&
                   len as libc::c_long <=
                       (*rstrm).in_boundry.wrapping_offset_from((*rstrm).in_finger)
                           as libc::c_long {
                buf =
                    (*rstrm).in_finger as *mut libc::c_void as
                        *mut rpc_inline_t;
                (*rstrm).fbtbc -= len;
                (*rstrm).in_finger = (*rstrm).in_finger.offset(len as isize)
            }
        }
        2 | _ => { }
    }
    return buf;
}
#[c2rust::src_loc = "385:1"]
unsafe extern "C" fn xdrrec_destroy(mut xdrs: *mut XDR) {
    let mut rstrm: *mut RECSTREAM = (*xdrs).x_private as *mut RECSTREAM;
    free((*rstrm).the_buffer as *mut libc::c_void);
    free(rstrm as caddr_t as *mut libc::c_void);
}
/* move to beginning of next record */
/*
 * Exported routines to manage xdr records
 */
/*
 * Before reading (deserializing from the stream, one should always call
 * this procedure to guarantee proper record alignment.
 */
#[no_mangle]
#[c2rust::src_loc = "404:1"]
pub unsafe extern "C" fn gssrpc_xdrrec_skiprecord(mut xdrs: *mut XDR)
 -> libc::c_int {
    let mut rstrm: *mut RECSTREAM = (*xdrs).x_private as *mut RECSTREAM;
    while (*rstrm).fbtbc > 0 as libc::c_int || (*rstrm).last_frag == 0 {
        if skip_input_bytes(rstrm, (*rstrm).fbtbc) == 0 {
            return 0 as libc::c_int
        }
        (*rstrm).fbtbc = 0 as libc::c_int;
        if (*rstrm).last_frag == 0 && set_input_fragment(rstrm) == 0 {
            return 0 as libc::c_int
        }
    }
    (*rstrm).last_frag = 0 as libc::c_int;
    return 1 as libc::c_int;
}
/* true if no more input */
/*
 * Look ahead fuction.
 * Returns TRUE iff there is no more input in the buffer
 * after consuming the rest of the current record.
 */
#[no_mangle]
#[c2rust::src_loc = "425:1"]
pub unsafe extern "C" fn gssrpc_xdrrec_eof(mut xdrs: *mut XDR)
 -> libc::c_int {
    let mut rstrm: *mut RECSTREAM = (*xdrs).x_private as *mut RECSTREAM;
    while (*rstrm).fbtbc > 0 as libc::c_int || (*rstrm).last_frag == 0 {
        if skip_input_bytes(rstrm, (*rstrm).fbtbc) == 0 {
            return 1 as libc::c_int
        }
        (*rstrm).fbtbc = 0 as libc::c_int;
        if (*rstrm).last_frag == 0 && set_input_fragment(rstrm) == 0 {
            return 1 as libc::c_int
        }
    }
    if (*rstrm).in_finger == (*rstrm).in_boundry { return 1 as libc::c_int }
    return 0 as libc::c_int;
}
/* make end of xdr record */
/*
 * The client must tell the package when an end-of-record has occurred.
 * The second paraemters tells whether the record should be flushed to the
 * (output) tcp stream.  (This let's the package support batched or
 * pipelined procedure calls.)  TRUE => immmediate flush to tcp connection.
 */
#[no_mangle]
#[c2rust::src_loc = "448:1"]
pub unsafe extern "C" fn gssrpc_xdrrec_endofrecord(mut xdrs: *mut XDR,
                                                   mut sendnow: libc::c_int)
 -> libc::c_int {
    let mut rstrm: *mut RECSTREAM =
        (*xdrs).x_private as *mut RECSTREAM; /* fragment length */
    let mut len: uint32_t = 0;
    if sendnow != 0 || (*rstrm).frag_sent != 0 ||
           (*rstrm).out_finger as libc::c_long +
               4 as libc::c_int as libc::c_long >=
               (*rstrm).out_boundry as libc::c_long {
        (*rstrm).frag_sent = 0 as libc::c_int;
        return flush_out(rstrm, 1 as libc::c_int)
    }
    len =
        ((*rstrm).out_finger as libc::c_long -
             (*rstrm).frag_header as libc::c_long -
             4 as libc::c_int as libc::c_long) as uint32_t;
    *(*rstrm).frag_header =
        htonl(len | ((1 as libc::c_ulong) << 31 as libc::c_int) as uint32_t);
    (*rstrm).frag_header =
        (*rstrm).out_finger as *mut libc::c_void as *mut uint32_t;
    (*rstrm).out_finger =
        (*rstrm).out_finger.offset(4 as libc::c_int as isize);
    return 1 as libc::c_int;
}
/*
 * Internal useful routines
 */
#[c2rust::src_loc = "472:1"]
unsafe extern "C" fn flush_out(mut rstrm: *mut RECSTREAM,
                               mut eor: libc::c_int) -> libc::c_int {
    let mut eormask: uint32_t =
        if eor == 1 as libc::c_int {
            ((1 as libc::c_ulong) << 31 as libc::c_int) as uint32_t
        } else { 0 as libc::c_int as libc::c_uint };
    let mut len: uint32_t =
        ((*rstrm).out_finger as
             u_long).wrapping_sub((*rstrm).frag_header as
                                      u_long).wrapping_sub(4 as libc::c_int as
                                                               libc::c_ulong)
            as uint32_t;
    *(*rstrm).frag_header = htonl(len | eormask);
    len =
        ((*rstrm).out_finger as
             u_long).wrapping_sub((*rstrm).out_base as u_long) as uint32_t;
    if ::std::mem::transmute::<_,
                               fn(_: _, _: _, _: _)
                                   ->
                                       libc::c_int>(Some((*rstrm).writeit.expect("non-null function pointer")).expect("non-null function pointer"))((*rstrm).tcp_handle,
                                                                                                                                                    (*rstrm).out_base,
                                                                                                                                                    len
                                                                                                                                                        as
                                                                                                                                                        libc::c_int)
           != len as libc::c_int {
        return 0 as libc::c_int
    }
    (*rstrm).frag_header =
        (*rstrm).out_base as *mut libc::c_void as *mut uint32_t;
    (*rstrm).out_finger = (*rstrm).out_base.offset(4 as libc::c_int as isize);
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "489:1"]
unsafe extern "C" fn fill_input_buf(mut rstrm: *mut RECSTREAM)
 -> libc::c_int {
    let mut where_0: caddr_t = 0 as *mut libc::c_char;
    let mut i: u_int = 0;
    let mut len: libc::c_int = 0;
    where_0 = (*rstrm).in_base;
    i =
        ((*rstrm).in_boundry as
             u_long).wrapping_rem(4 as libc::c_int as libc::c_ulong) as u_int;
    where_0 = where_0.offset(i as isize);
    len = (*rstrm).in_size.wrapping_sub(i) as libc::c_int;
    len =
        ::std::mem::transmute::<_,
                                fn(_: _, _: _, _: _)
                                    ->
                                        libc::c_int>(Some((*rstrm).readit.expect("non-null function pointer")).expect("non-null function pointer"))((*rstrm).tcp_handle,
                                                                                                                                                    where_0,
                                                                                                                                                    len);
    if len == -(1 as libc::c_int) { return 0 as libc::c_int }
    (*rstrm).in_finger = where_0;
    where_0 = where_0.offset(len as isize);
    (*rstrm).in_boundry = where_0;
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "508:1"]
unsafe extern "C" fn get_input_bytes(mut rstrm: *mut RECSTREAM,
                                     mut addr: caddr_t, mut len: libc::c_int)
 -> libc::c_int {
    let mut current: size_t = 0;
    while len > 0 as libc::c_int {
        current =
            ((*rstrm).in_boundry as libc::c_long -
                 (*rstrm).in_finger as libc::c_long) as size_t;
        if current == 0 as libc::c_int as libc::c_ulong {
            if fill_input_buf(rstrm) == 0 { return 0 as libc::c_int }
        } else {
            current =
                if (len as size_t) < current {
                    len as size_t
                } else { current };
            memmove(addr as *mut libc::c_void,
                    (*rstrm).in_finger as *const libc::c_void, current);
            (*rstrm).in_finger = (*rstrm).in_finger.offset(current as isize);
            addr = addr.offset(current as isize);
            len =
                (len as libc::c_ulong).wrapping_sub(current) as libc::c_int as
                    libc::c_int
        }
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "530:1"]
unsafe extern "C" fn set_input_fragment(mut rstrm: *mut RECSTREAM)
 -> libc::c_int {
    let mut header: uint32_t = 0;
    if get_input_bytes(rstrm, &mut header as *mut uint32_t as caddr_t,
                       ::std::mem::size_of::<uint32_t>() as libc::c_ulong as
                           libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    header = ntohl(header);
    (*rstrm).last_frag =
        if header & ((1 as libc::c_ulong) << 31 as libc::c_int) as uint32_t ==
               0 as libc::c_int as libc::c_uint {
            0 as libc::c_int
        } else { 1 as libc::c_int };
    (*rstrm).fbtbc =
        (header & !(((1 as libc::c_ulong) << 31 as libc::c_int) as uint32_t))
            as int32_t;
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "544:1"]
unsafe extern "C" fn skip_input_bytes(mut rstrm: *mut RECSTREAM,
                                      mut cnt: int32_t) -> libc::c_int {
    let mut current: libc::c_int = 0;
    while cnt > 0 as libc::c_int {
        current =
            ((*rstrm).in_boundry as libc::c_long -
                 (*rstrm).in_finger as libc::c_long) as libc::c_int;
        if current == 0 as libc::c_int {
            if fill_input_buf(rstrm) == 0 { return 0 as libc::c_int }
        } else {
            current = if cnt < current { cnt } else { current };
            (*rstrm).in_finger = (*rstrm).in_finger.offset(current as isize);
            cnt -= current
        }
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "564:1"]
unsafe extern "C" fn fix_buf_size(mut s: u_int) -> u_int {
    if s < 100 as libc::c_int as libc::c_uint {
        s = 4000 as libc::c_int as u_int
    }
    return s.wrapping_add(4 as libc::c_int as
                              libc::c_uint).wrapping_sub(1 as libc::c_int as
                                                             libc::c_uint).wrapping_div(4
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint).wrapping_mul(4
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_uint);
}
