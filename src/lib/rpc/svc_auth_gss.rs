use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:37"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:37"]
pub mod types_h {
    #[c2rust::src_loc = "31:1"]
    pub type __u_char = libc::c_uchar;
    #[c2rust::src_loc = "32:1"]
    pub type __u_short = libc::c_ushort;
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
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
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:37"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:37"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/sys/types.h:37"]
pub mod sys_types_h {
    #[c2rust::src_loc = "33:1"]
    pub type u_char = __u_char;
    #[c2rust::src_loc = "34:1"]
    pub type u_short = __u_short;
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_char, __u_short, __u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:37"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:37"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:38"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/netinet/in.h:38"]
pub mod in_h {
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:8"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    #[c2rust::src_loc = "30:1"]
    pub type in_addr_t = uint32_t;
    use super::stdint_uintn_h::{uint16_t, uint32_t};
    use super::sockaddr_h::sa_family_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "378:1"]
        pub fn htonl(__hostlong: uint32_t) -> uint32_t;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:38"]
pub mod gssrpc_types_h {
    /* @(#)types.h	2.3 88/08/15 4.0 RPCSRC */
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
 *     * Neither the name of the “Oracle America, Inc.” nor the names of
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
/*      @(#)types.h 1.18 87/07/24 SMI      */
    /*
 * Rpc additions to <sys/types.h>
 */
    /*
 * Try to get MAXHOSTNAMELEN from somewhere.
 */
    /* #include <netdb.h> */
    /* Get htonl(), ntohl(), etc. */
    /* Define if we need to fake up some BSD type aliases. */
    /* Allow application to override. */
    /* #undef GSSRPC__BSD_TYPEALIASES */
    #[c2rust::src_loc = "88:1"]
    pub type rpcprog_t = uint32_t;
    #[c2rust::src_loc = "89:1"]
    pub type rpcvers_t = uint32_t;
    #[c2rust::src_loc = "91:1"]
    pub type rpcproc_t = uint32_t;
    #[c2rust::src_loc = "93:1"]
    pub type rpc_inline_t = int32_t;
    use super::stdint_uintn_h::uint32_t;
    use super::stdint_intn_h::int32_t;
    /* !defined(GSSRPC_TYPES_H) */
    /*
 * The below should probably be internal-only, but seem to be
 * traditionally exported in RPC implementations.
 */
    /* XXX namespace */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:38"]
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
        #[no_mangle]
        #[c2rust::src_loc = "251:1"]
        pub fn gssrpc_xdr_void(_: *mut XDR, _: *mut libc::c_void)
         -> libc::c_int;
        /* XDR using memory buffers */
        #[no_mangle]
        #[c2rust::src_loc = "315:1"]
        pub fn gssrpc_xdrmem_create(_: *mut XDR, _: caddr_t, _: u_int,
                                    _: xdr_op);
        /* free memory buffers for xdr */
        #[no_mangle]
        #[c2rust::src_loc = "335:1"]
        pub fn gssrpc_xdr_free(_: xdrproc_t, _: *mut libc::c_void);
    }
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth.h:38"]
pub mod auth_h {
    #[c2rust::src_loc = "55:1"]
    pub type auth_stat = libc::c_uint;
    #[c2rust::src_loc = "74:2"]
    pub const RPCSEC_GSS_CTXPROBLEM: auth_stat = 14;
    #[c2rust::src_loc = "73:2"]
    pub const RPCSEC_GSS_CREDPROBLEM: auth_stat = 13;
    #[c2rust::src_loc = "69:2"]
    pub const AUTH_FAILED: auth_stat = 7;
    #[c2rust::src_loc = "68:2"]
    pub const AUTH_INVALIDRESP: auth_stat = 6;
    #[c2rust::src_loc = "64:2"]
    pub const AUTH_TOOWEAK: auth_stat = 5;
    #[c2rust::src_loc = "63:2"]
    pub const AUTH_REJECTEDVERF: auth_stat = 4;
    #[c2rust::src_loc = "62:2"]
    pub const AUTH_BADVERF: auth_stat = 3;
    #[c2rust::src_loc = "61:2"]
    pub const AUTH_REJECTEDCRED: auth_stat = 2;
    #[c2rust::src_loc = "60:2"]
    pub const AUTH_BADCRED: auth_stat = 1;
    #[c2rust::src_loc = "56:2"]
    pub const AUTH_OK: auth_stat = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "86:8"]
    pub struct opaque_auth {
        pub oa_flavor: libc::c_int,
        pub oa_base: caddr_t,
        pub oa_length: u_int,
    }
    use super::sys_types_h::{caddr_t, u_int};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "173:27"]
        pub static mut gssrpc__null_auth: opaque_auth;
    }
    /* !defined(GSSRPC_AUTH_H) */
    /* RPCSEC_GSS */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc_msg.h:38"]
pub mod rpc_msg_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "150:8"]
    pub struct rpc_msg {
        pub rm_xid: uint32_t,
        pub rm_direction: msg_type,
        pub ru: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "153:2"]
    pub union C2RustUnnamed {
        pub RM_cmb: call_body,
        pub RM_rmb: reply_body,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "125:8"]
    pub struct reply_body {
        pub rp_stat: reply_stat,
        pub ru: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "127:2"]
    pub union C2RustUnnamed_0 {
        pub RP_ar: accepted_reply,
        pub RP_dr: rejected_reply,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "109:8"]
    pub struct rejected_reply {
        pub rj_stat: reject_stat,
        pub ru: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "111:2"]
    pub union C2RustUnnamed_1 {
        pub RJ_versions: C2RustUnnamed_2,
        pub RJ_why: auth_stat,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:3"]
    pub struct C2RustUnnamed_2 {
        pub low: rpcvers_t,
        pub high: rpcvers_t,
    }
    #[c2rust::src_loc = "74:1"]
    pub type reject_stat = libc::c_uint;
    #[c2rust::src_loc = "76:2"]
    pub const AUTH_ERROR: reject_stat = 1;
    #[c2rust::src_loc = "75:2"]
    pub const RPC_MISMATCH: reject_stat = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "88:8"]
    pub struct accepted_reply {
        pub ar_verf: opaque_auth,
        pub ar_stat: accept_stat,
        pub ru: C2RustUnnamed_3,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "91:2"]
    pub union C2RustUnnamed_3 {
        pub AR_versions: C2RustUnnamed_5,
        pub AR_results: C2RustUnnamed_4,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "96:3"]
    pub struct C2RustUnnamed_4 {
        pub where_0: caddr_t,
        pub proc_0: xdrproc_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:3"]
    pub struct C2RustUnnamed_5 {
        pub low: rpcvers_t,
        pub high: rpcvers_t,
    }
    #[c2rust::src_loc = "65:1"]
    pub type accept_stat = libc::c_uint;
    #[c2rust::src_loc = "71:2"]
    pub const SYSTEM_ERR: accept_stat = 5;
    #[c2rust::src_loc = "70:2"]
    pub const GARBAGE_ARGS: accept_stat = 4;
    #[c2rust::src_loc = "69:2"]
    pub const PROC_UNAVAIL: accept_stat = 3;
    #[c2rust::src_loc = "68:2"]
    pub const PROG_MISMATCH: accept_stat = 2;
    #[c2rust::src_loc = "67:2"]
    pub const PROG_UNAVAIL: accept_stat = 1;
    #[c2rust::src_loc = "66:2"]
    pub const SUCCESS: accept_stat = 0;
    #[c2rust::src_loc = "60:1"]
    pub type reply_stat = libc::c_uint;
    #[c2rust::src_loc = "62:2"]
    pub const MSG_DENIED: reply_stat = 1;
    #[c2rust::src_loc = "61:2"]
    pub const MSG_ACCEPTED: reply_stat = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "138:8"]
    pub struct call_body {
        pub cb_rpcvers: rpcvers_t,
        pub cb_prog: rpcprog_t,
        pub cb_vers: rpcvers_t,
        pub cb_proc: rpcproc_t,
        pub cb_cred: opaque_auth,
        pub cb_verf: opaque_auth,
    }
    #[c2rust::src_loc = "55:1"]
    pub type msg_type = libc::c_uint;
    #[c2rust::src_loc = "57:2"]
    pub const REPLY: msg_type = 1;
    #[c2rust::src_loc = "56:2"]
    pub const CALL: msg_type = 0;
    use super::stdint_uintn_h::uint32_t;
    use super::auth_h::{auth_stat, opaque_auth};
    use super::gssrpc_types_h::{rpcvers_t, rpcprog_t, rpcproc_t};
    use super::sys_types_h::caddr_t;
    use super::xdr_h::xdrproc_t;
    /* !defined(GSSRPC_RPC_MSG_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:38"]
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
    pub type gss_OID = *mut gss_OID_desc_struct;
    /* OM_STRING */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:16"]
    pub struct gss_OID_set_desc_struct {
        pub count: size_t,
        pub elements: gss_OID,
    }
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
        /* message_token */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "499:1"]
        pub fn gss_verify_mic(_: *mut OM_uint32, _: gss_ctx_id_t,
                              _: gss_buffer_t, _: gss_buffer_t,
                              _: *mut gss_qop_t) -> OM_uint32;
        /* name_equal */
        #[no_mangle]
        #[c2rust::src_loc = "554:1"]
        pub fn gss_display_name(_: *mut OM_uint32, _: gss_name_t,
                                _: gss_buffer_t, _: *mut gss_OID)
         -> OM_uint32;
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
        /* mech_types */
        /*
 * The following routines are obsolete variants of gss_get_mic, gss_wrap,
 * gss_verify_mic and gss_unwrap.  They should be provided by GSSAPI V2
 * implementations for backwards compatibility with V1 applications.  Distinct
 * entrypoints (as opposed to #defines) should be provided, to allow GSSAPI
 * V1 applications to link against GSSAPI V2 implementations.
 */
        #[no_mangle]
        #[c2rust::src_loc = "734:1"]
        pub fn gss_sign(_: *mut OM_uint32, _: gss_ctx_id_t, _: libc::c_int,
                        _: gss_buffer_t, _: gss_buffer_t) -> OM_uint32;
        /* exported_name */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "777:1"]
        pub fn gss_duplicate_name(_: *mut OM_uint32, _: gss_name_t,
                                  _: *mut gss_name_t) -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth_gss.h:38"]
pub mod auth_gss_h {
    /* include/gssrpc/auth_gss.h */
/*
  Copyright (c) 2000 The Regents of the University of Michigan.
  All rights reserved.

  Copyright (c) 2000 Dug Song <dugsong@UMICH.EDU>.
  All rights reserved, all wrongs reversed.

  Redistribution and use in source and binary forms, with or without
  modification, are permitted provided that the following conditions
  are met:

  1. Redistributions of source code must retain the above copyright
     notice, this list of conditions and the following disclaimer.
  2. Redistributions in binary form must reproduce the above copyright
     notice, this list of conditions and the following disclaimer in the
     documentation and/or other materials provided with the distribution.
  3. Neither the name of the University nor the names of its
     contributors may be used to endorse or promote products derived
     from this software without specific prior written permission.

  THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
  WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
  MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
  DISCLAIMED. IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
  FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
  CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
  SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
  BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
  LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
  NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
  SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

  Id: auth_gss.h,v 1.13 2002/05/08 16:54:33 andros Exp
*/
    /* RPCSEC_GSS control procedures. */
    #[c2rust::src_loc = "51:9"]
    pub type rpc_gss_proc_t = libc::c_uint;
    #[c2rust::src_loc = "55:2"]
    pub const RPCSEC_GSS_DESTROY: rpc_gss_proc_t = 3;
    #[c2rust::src_loc = "54:2"]
    pub const RPCSEC_GSS_CONTINUE_INIT: rpc_gss_proc_t = 2;
    #[c2rust::src_loc = "53:2"]
    pub const RPCSEC_GSS_INIT: rpc_gss_proc_t = 1;
    #[c2rust::src_loc = "52:2"]
    pub const RPCSEC_GSS_DATA: rpc_gss_proc_t = 0;
    /* RPCSEC_GSS services. */
    #[c2rust::src_loc = "59:9"]
    pub type rpc_gss_svc_t = libc::c_uint;
    #[c2rust::src_loc = "62:2"]
    pub const RPCSEC_GSS_SVC_PRIVACY: rpc_gss_svc_t = 3;
    #[c2rust::src_loc = "61:2"]
    pub const RPCSEC_GSS_SVC_INTEGRITY: rpc_gss_svc_t = 2;
    #[c2rust::src_loc = "60:2"]
    pub const RPCSEC_GSS_SVC_NONE: rpc_gss_svc_t = 1;
    /* RPCSEC_GSS security triple. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "68:8"]
    pub struct rpc_gss_sec {
        pub mech: gss_OID,
        pub qop: gss_qop_t,
        pub svc: rpc_gss_svc_t,
        pub cred: gss_cred_id_t,
        pub req_flags: uint32_t,
    }
    /* Sequence window */
    /* Krb 5 default mechanism
#define KRB5OID  "1.2.840.113554.1.2.2"

gss_OID_desc krb5oid = {
	20, KRB5OID
};
 */
    /*
struct rpc_gss_sec krb5mech = {
	(gss_OID)&krb5oid,
	GSS_QOP_DEFAULT,
	RPCSEC_GSS_SVC_NONE
};
*/
    /* Credentials. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "100:8"]
    pub struct rpc_gss_cred {
        pub gc_v: u_int,
        pub gc_proc: rpc_gss_proc_t,
        pub gc_seq: uint32_t,
        pub gc_svc: rpc_gss_svc_t,
        pub gc_ctx: gss_buffer_desc,
    }
    /* context handle */
    /* Context creation response. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "109:8"]
    pub struct rpc_gss_init_res {
        pub gr_ctx: gss_buffer_desc,
        pub gr_major: uint32_t,
        pub gr_minor: uint32_t,
        pub gr_win: uint32_t,
        pub gr_token: gss_buffer_desc,
    }
    use super::gssapi_h::{gss_OID, gss_qop_t, gss_cred_id_t, gss_buffer_desc,
                          gss_ctx_id_struct, gss_ctx_id_t, OM_uint32};
    use super::stdint_uintn_h::uint32_t;
    use super::sys_types_h::{u_int, caddr_t};
    use super::xdr_h::{XDR, xdrproc_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "122:1"]
        pub fn gssrpc_xdr_rpc_gss_cred(xdrs: *mut XDR, p: *mut rpc_gss_cred)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "123:1"]
        pub fn gssrpc_xdr_rpc_gss_init_args(xdrs: *mut XDR,
                                            p: *mut gss_buffer_desc)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "124:1"]
        pub fn gssrpc_xdr_rpc_gss_init_res(xdrs: *mut XDR,
                                           p: *mut rpc_gss_init_res)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "125:1"]
        pub fn gssrpc_xdr_rpc_gss_data(xdrs: *mut XDR, xdr_func: xdrproc_t,
                                       xdr_ptr: caddr_t, ctx: gss_ctx_id_t,
                                       qop: gss_qop_t, svc: rpc_gss_svc_t,
                                       seq: uint32_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "142:1"]
        pub fn gssrpc_log_debug(fmt: *const libc::c_char, _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "143:1"]
        pub fn gssrpc_log_status(m: *mut libc::c_char, major: OM_uint32,
                                 minor: OM_uint32);
    }
    /* !defined(GSSRPC_AUTH_GSS_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/svc.h:38"]
pub mod svc_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "167:8"]
    pub struct svc_req {
        pub rq_prog: rpcprog_t,
        pub rq_vers: rpcvers_t,
        pub rq_proc: rpcproc_t,
        pub rq_cred: opaque_auth,
        pub rq_clntcred: *mut libc::c_void,
        pub rq_svccred: *mut libc::c_void,
        pub rq_clntname: *mut libc::c_void,
        pub rq_xprt: *mut SVCXPRT,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "76:16"]
    pub struct SVCXPRT {
        pub xp_sock: libc::c_int,
        pub xp_port: u_short,
        pub xp_ops: *mut xp_ops,
        pub xp_addrlen: libc::c_int,
        pub xp_raddr: sockaddr_in,
        pub xp_verf: opaque_auth,
        pub xp_auth: *mut SVCAUTH,
        pub xp_p1: *mut libc::c_void,
        pub xp_p2: *mut libc::c_void,
        pub xp_laddrlen: libc::c_int,
        pub xp_laddr: sockaddr_in,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "83:9"]
    pub struct xp_ops {
        pub xp_recv: Option<unsafe extern "C" fn(_: *mut SVCXPRT,
                                                 _: *mut rpc_msg)
                                -> libc::c_int>,
        pub xp_stat: Option<unsafe extern "C" fn(_: *mut SVCXPRT)
                                -> xprt_stat>,
        pub xp_getargs: Option<unsafe extern "C" fn(_: *mut SVCXPRT,
                                                    _: xdrproc_t,
                                                    _: *mut libc::c_void)
                                   -> libc::c_int>,
        pub xp_reply: Option<unsafe extern "C" fn(_: *mut SVCXPRT,
                                                  _: *mut rpc_msg)
                                 -> libc::c_int>,
        pub xp_freeargs: Option<unsafe extern "C" fn(_: *mut SVCXPRT,
                                                     _: xdrproc_t,
                                                     _: *mut libc::c_void)
                                    -> libc::c_int>,
        pub xp_destroy: Option<unsafe extern "C" fn(_: *mut SVCXPRT) -> ()>,
    }
    #[c2rust::src_loc = "67:1"]
    pub type xprt_stat = libc::c_uint;
    #[c2rust::src_loc = "70:2"]
    pub const XPRT_IDLE: xprt_stat = 2;
    #[c2rust::src_loc = "69:2"]
    pub const XPRT_MOREREQS: xprt_stat = 1;
    #[c2rust::src_loc = "68:2"]
    pub const XPRT_DIED: xprt_stat = 0;
    use super::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t};
    use super::auth_h::opaque_auth;
    use super::sys_types_h::{u_short, caddr_t};
    use super::in_h::sockaddr_in;
    use super::svc_auth_h::SVCAUTH;
    use super::rpc_msg_h::rpc_msg;
    use super::xdr_h::xdrproc_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "258:1"]
        pub fn gssrpc_svc_sendreply(_: *mut SVCXPRT, _: xdrproc_t, _: caddr_t)
         -> libc::c_int;
    }
    /* !defined(GSSRPC_SVC_H) */
    /* XXX add auth_gsapi_log_*? */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/svc_auth.h:38"]
pub mod svc_auth_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "53:16"]
    pub struct SVCAUTH {
        pub svc_ah_ops: *mut svc_auth_ops,
        pub svc_ah_private: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "54:9"]
    pub struct svc_auth_ops {
        pub svc_ah_wrap: Option<unsafe extern "C" fn(_: *mut SVCAUTH,
                                                     _: *mut XDR,
                                                     _: xdrproc_t, _: caddr_t)
                                    -> libc::c_int>,
        pub svc_ah_unwrap: Option<unsafe extern "C" fn(_: *mut SVCAUTH,
                                                       _: *mut XDR,
                                                       _: xdrproc_t,
                                                       _: caddr_t)
                                      -> libc::c_int>,
        pub svc_ah_destroy: Option<unsafe extern "C" fn(_: *mut SVCAUTH)
                                       -> libc::c_int>,
    }
    use super::xdr_h::{XDR, xdrproc_t};
    use super::sys_types_h::caddr_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "66:16"]
        pub static mut gssrpc_svc_auth_none: SVCAUTH;
    }
    /* !defined(GSSRPC_SVC_AUTH_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth_gssapi.h:39"]
pub mod auth_gssapi_h {
    /* auth_gssapi_log_badauth_func is IPv4-specific; this version gives the
 * transport handle so the fd can be used to get the address. */
    #[c2rust::src_loc = "59:1"]
    pub type auth_gssapi_log_badauth2_func
        =
        Option<unsafe extern "C" fn(_: OM_uint32, _: OM_uint32,
                                    _: *mut SVCXPRT, _: caddr_t) -> ()>;
    #[c2rust::src_loc = "65:1"]
    pub type auth_gssapi_log_badverf_func
        =
        Option<unsafe extern "C" fn(_: gss_name_t, _: gss_name_t,
                                    _: *mut svc_req, _: *mut rpc_msg,
                                    _: caddr_t) -> ()>;
    #[c2rust::src_loc = "51:1"]
    pub type auth_gssapi_log_badauth_func
        =
        Option<unsafe extern "C" fn(_: OM_uint32, _: OM_uint32,
                                    _: *mut sockaddr_in, _: caddr_t) -> ()>;
    #[c2rust::src_loc = "72:1"]
    pub type auth_gssapi_log_miscerr_func
        =
        Option<unsafe extern "C" fn(_: *mut svc_req, _: *mut rpc_msg,
                                    _: *mut libc::c_char, _: caddr_t) -> ()>;
    use super::gssapi_h::{OM_uint32, gss_name_t};
    use super::svc_h::{SVCXPRT, svc_req};
    use super::sys_types_h::caddr_t;
    use super::rpc_msg_h::rpc_msg;
    use super::in_h::sockaddr_in;
    /* !defined(GSSRPC_AUTH_GSSAPI_H) */
}
#[c2rust::header_src = "/usr/include/stdlib.h:37"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:37"]
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
#[c2rust::header_src = "/usr/include/string.h:37"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__u_char, __u_short, __u_int, __uint16_t, __int32_t,
                        __uint32_t, __off_t, __off64_t, __caddr_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint16_t, uint32_t};
pub use self::sys_types_h::{u_char, u_short, u_int, caddr_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::sockaddr_h::sa_family_t;
pub use self::in_h::{in_port_t, sockaddr_in, in_addr, in_addr_t, htonl};
pub use self::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t, rpc_inline_t};
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, gssrpc_xdr_void, gssrpc_xdrmem_create,
                      gssrpc_xdr_free};
pub use self::auth_h::{auth_stat, RPCSEC_GSS_CTXPROBLEM,
                       RPCSEC_GSS_CREDPROBLEM, AUTH_FAILED, AUTH_INVALIDRESP,
                       AUTH_TOOWEAK, AUTH_REJECTEDVERF, AUTH_BADVERF,
                       AUTH_REJECTEDCRED, AUTH_BADCRED, AUTH_OK, opaque_auth,
                       gssrpc__null_auth};
pub use self::rpc_msg_h::{rpc_msg, C2RustUnnamed, reply_body, C2RustUnnamed_0,
                          rejected_reply, C2RustUnnamed_1, C2RustUnnamed_2,
                          reject_stat, AUTH_ERROR, RPC_MISMATCH,
                          accepted_reply, C2RustUnnamed_3, C2RustUnnamed_4,
                          C2RustUnnamed_5, accept_stat, SYSTEM_ERR,
                          GARBAGE_ARGS, PROC_UNAVAIL, PROG_MISMATCH,
                          PROG_UNAVAIL, SUCCESS, reply_stat, MSG_DENIED,
                          MSG_ACCEPTED, call_body, msg_type, REPLY, CALL};
pub use self::gssapi_h::{gss_name_t, gss_cred_id_t, gss_ctx_id_t, gss_uint32,
                         OM_uint32, gss_OID_desc_struct, gss_OID,
                         gss_OID_set_desc_struct, gss_OID_set,
                         gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_channel_bindings_struct,
                         gss_channel_bindings_t, gss_qop_t, gss_cred_usage_t,
                         gss_name_struct, gss_cred_id_struct,
                         gss_ctx_id_struct, gss_acquire_cred,
                         gss_release_cred, gss_accept_sec_context,
                         gss_delete_sec_context, gss_get_mic, gss_verify_mic,
                         gss_display_name, gss_release_name,
                         gss_release_buffer, gss_sign, gss_duplicate_name};
pub use self::auth_gss_h::{rpc_gss_proc_t, RPCSEC_GSS_DESTROY,
                           RPCSEC_GSS_CONTINUE_INIT, RPCSEC_GSS_INIT,
                           RPCSEC_GSS_DATA, rpc_gss_svc_t,
                           RPCSEC_GSS_SVC_PRIVACY, RPCSEC_GSS_SVC_INTEGRITY,
                           RPCSEC_GSS_SVC_NONE, rpc_gss_sec, rpc_gss_cred,
                           rpc_gss_init_res, gssrpc_xdr_rpc_gss_cred,
                           gssrpc_xdr_rpc_gss_init_args,
                           gssrpc_xdr_rpc_gss_init_res,
                           gssrpc_xdr_rpc_gss_data, gssrpc_log_debug,
                           gssrpc_log_status};
pub use self::svc_h::{svc_req, SVCXPRT, xp_ops, xprt_stat, XPRT_IDLE,
                      XPRT_MOREREQS, XPRT_DIED, gssrpc_svc_sendreply};
pub use self::svc_auth_h::{SVCAUTH, svc_auth_ops, gssrpc_svc_auth_none};
pub use self::auth_gssapi_h::{auth_gssapi_log_badauth2_func,
                              auth_gssapi_log_badverf_func,
                              auth_gssapi_log_badauth_func,
                              auth_gssapi_log_miscerr_func};
use self::stdlib_h::{malloc, calloc, free};
use self::stdio_h::{stderr, fprintf};
use self::string_h::{memset, memcpy};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "92:8"]
pub struct svc_rpc_gss_data {
    pub established: libc::c_int,
    pub cred: gss_cred_id_t,
    pub ctx: gss_ctx_id_t,
    pub sec: rpc_gss_sec,
    pub cname: gss_buffer_desc,
    pub seq: u_int,
    pub win: u_int,
    pub seqlast: u_int,
    pub seqmask: uint32_t,
    pub client_name: gss_name_t,
    pub checksum: gss_buffer_desc,
}
/* lib/rpc/svc_auth_gss.c */
/*
  Copyright (c) 2000 The Regents of the University of Michigan.
  All rights reserved.

  Copyright (c) 2000 Dug Song <dugsong@UMICH.EDU>.
  All rights reserved, all wrongs reversed.

  Redistribution and use in source and binary forms, with or without
  modification, are permitted provided that the following conditions
  are met:

  1. Redistributions of source code must retain the above copyright
     notice, this list of conditions and the following disclaimer.
  2. Redistributions in binary form must reproduce the above copyright
     notice, this list of conditions and the following disclaimer in the
     documentation and/or other materials provided with the distribution.
  3. Neither the name of the University nor the names of its
     contributors may be used to endorse or promote products derived
     from this software without specific prior written permission.

  THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
  WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
  MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
  DISCLAIMED. IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
  FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
  CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
  SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
  BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
  LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
  NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
  SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

  Id: svc_auth_gss.c,v 1.28 2002/10/15 21:29:36 kwc Exp
 */
#[no_mangle]
#[c2rust::src_loc = "49:5"]
pub static mut gssrpc_svc_debug_gss: libc::c_int = 0 as libc::c_int;
/* SPKM */
#[c2rust::src_loc = "68:37"]
static mut log_badauth: auth_gssapi_log_badauth_func = None;
#[c2rust::src_loc = "69:16"]
static mut log_badauth_data: caddr_t = 0 as *const libc::c_char as caddr_t;
#[c2rust::src_loc = "70:38"]
static mut log_badauth2: auth_gssapi_log_badauth2_func = None;
#[c2rust::src_loc = "71:16"]
static mut log_badauth2_data: caddr_t = 0 as *const libc::c_char as caddr_t;
#[c2rust::src_loc = "72:37"]
static mut log_badverf: auth_gssapi_log_badverf_func = None;
#[c2rust::src_loc = "73:16"]
static mut log_badverf_data: caddr_t = 0 as *const libc::c_char as caddr_t;
#[c2rust::src_loc = "74:37"]
static mut log_miscerr: auth_gssapi_log_miscerr_func = None;
#[c2rust::src_loc = "75:16"]
static mut log_miscerr_data: caddr_t = 0 as *const libc::c_char as caddr_t;
#[no_mangle]
#[c2rust::src_loc = "86:21"]
pub static mut gssrpc_svc_auth_gss_ops: svc_auth_ops =
    unsafe {
        {
            let mut init =
                svc_auth_ops{svc_ah_wrap:
                                 Some(svcauth_gss_wrap as
                                          unsafe extern "C" fn(_:
                                                                   *mut SVCAUTH,
                                                               _: *mut XDR,
                                                               _: xdrproc_t,
                                                               _: caddr_t)
                                              -> libc::c_int),
                             svc_ah_unwrap:
                                 Some(svcauth_gss_unwrap as
                                          unsafe extern "C" fn(_:
                                                                   *mut SVCAUTH,
                                                               _: *mut XDR,
                                                               _: xdrproc_t,
                                                               _: caddr_t)
                                              -> libc::c_int),
                             svc_ah_destroy:
                                 Some(svcauth_gss_destroy as
                                          unsafe extern "C" fn(_:
                                                                   *mut SVCAUTH)
                                              -> libc::c_int),};
            init
        }
    };
/* so we can free it */
/* Global server credentials. */
#[c2rust::src_loc = "110:19"]
static mut svcauth_gss_name: gss_name_t =
    0 as *const gss_name_struct as gss_name_t;
/*
 * Approved way of setting server principal
 */
#[no_mangle]
#[c2rust::src_loc = "112:1"]
pub unsafe extern "C" fn gssrpc_svcauth_gss_set_svc_name(mut name: gss_name_t)
 -> libc::c_int {
    let mut maj_stat: OM_uint32 = 0;
    let mut min_stat: OM_uint32 = 0;
    gssrpc_log_debug(b"in svcauth_gss_set_svc_name()\x00" as *const u8 as
                         *const libc::c_char);
    if !svcauth_gss_name.is_null() {
        maj_stat = gss_release_name(&mut min_stat, &mut svcauth_gss_name);
        if maj_stat != 0 as libc::c_int as libc::c_uint {
            gssrpc_log_status(b"gss_release_name\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              maj_stat, min_stat);
            return 0 as libc::c_int
        }
        svcauth_gss_name = 0 as gss_name_t
    }
    if svcauth_gss_name.is_null() { return 1 as libc::c_int }
    maj_stat = gss_duplicate_name(&mut min_stat, name, &mut svcauth_gss_name);
    if maj_stat != 0 as libc::c_int as libc::c_uint {
        gssrpc_log_status(b"gss_duplicate_name\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          maj_stat, min_stat);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "141:1"]
unsafe extern "C" fn svcauth_gss_acquire_cred(mut gd: *mut svc_rpc_gss_data)
 -> libc::c_int {
    let mut maj_stat: OM_uint32 = 0;
    let mut min_stat: OM_uint32 = 0;
    gssrpc_log_debug(b"in svcauth_gss_acquire_cred()\x00" as *const u8 as
                         *const libc::c_char);
    /* We don't need to acquire a credential if using the default name. */
    if svcauth_gss_name.is_null() { return 1 as libc::c_int }
    /* Only acquire a credential once per authentication. */
    if !(*gd).cred.is_null() { return 1 as libc::c_int }
    maj_stat =
        gss_acquire_cred(&mut min_stat, svcauth_gss_name,
                         0 as libc::c_int as OM_uint32, 0 as gss_OID_set,
                         2 as libc::c_int, &mut (*gd).cred,
                         0 as *mut gss_OID_set, 0 as *mut OM_uint32);
    if maj_stat != 0 as libc::c_int as libc::c_uint {
        gssrpc_log_status(b"gss_acquire_cred\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          maj_stat, min_stat);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* Invoke log_badauth callbacks for an authentication failure. */
#[c2rust::src_loc = "168:1"]
unsafe extern "C" fn badauth(mut maj: OM_uint32, mut minor: OM_uint32,
                             mut xprt: *mut SVCXPRT) {
    if log_badauth.is_some() {
        Some(log_badauth.expect("non-null function pointer")).expect("non-null function pointer")(maj,
                                                                                                  minor,
                                                                                                  &mut (*xprt).xp_raddr,
                                                                                                  log_badauth_data);
    }
    if log_badauth2.is_some() {
        Some(log_badauth2.expect("non-null function pointer")).expect("non-null function pointer")(maj,
                                                                                                   minor,
                                                                                                   xprt,
                                                                                                   log_badauth2_data);
    };
}
#[c2rust::src_loc = "177:1"]
unsafe extern "C" fn svcauth_gss_accept_sec_context(mut rqst: *mut svc_req,
                                                    mut gr:
                                                        *mut rpc_gss_init_res)
 -> libc::c_int {
    let mut current_block: u64;
    let mut gd: *mut svc_rpc_gss_data = 0 as *mut svc_rpc_gss_data;
    let mut gc: *mut rpc_gss_cred = 0 as *mut rpc_gss_cred;
    let mut recv_tok: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut seqbuf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut mech: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut maj_stat: OM_uint32 = 0 as libc::c_int as OM_uint32;
    let mut min_stat: OM_uint32 = 0 as libc::c_int as OM_uint32;
    let mut ret_flags: OM_uint32 = 0;
    let mut seq: OM_uint32 = 0;
    gssrpc_log_debug(b"in svcauth_gss_accept_context()\x00" as *const u8 as
                         *const libc::c_char);
    gd =
        *(&mut (*(*(*rqst).rq_xprt).xp_auth).svc_ah_private as
              *mut *mut libc::c_void as *mut *mut svc_rpc_gss_data);
    gc = (*rqst).rq_clntcred as *mut rpc_gss_cred;
    memset(gr as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<rpc_gss_init_res>() as libc::c_ulong);
    /* Deserialize arguments. */
    memset(&mut recv_tok as *mut gss_buffer_desc as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<gss_buffer_desc>() as libc::c_ulong);
    if Some((*(*(*rqst).rq_xprt).xp_ops).xp_getargs.expect("non-null function pointer")).expect("non-null function pointer")((*rqst).rq_xprt,
                                                                                                                             ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                                                     *mut XDR,
                                                                                                                                                                                 _:
                                                                                                                                                                                     *mut gss_buffer_desc)
                                                                                                                                                                ->
                                                                                                                                                                    libc::c_int>,
                                                                                                                                                     xdrproc_t>(Some(gssrpc_xdr_rpc_gss_init_args
                                                                                                                                                                         as
                                                                                                                                                                         unsafe extern "C" fn(_:
                                                                                                                                                                                                  *mut XDR,
                                                                                                                                                                                              _:
                                                                                                                                                                                                  *mut gss_buffer_desc)
                                                                                                                                                                             ->
                                                                                                                                                                                 libc::c_int)),
                                                                                                                             &mut recv_tok
                                                                                                                                 as
                                                                                                                                 *mut gss_buffer_desc
                                                                                                                                 as
                                                                                                                                 caddr_t
                                                                                                                                 as
                                                                                                                                 *mut libc::c_void)
           == 0 {
        return 0 as libc::c_int
    }
    (*gr).gr_major =
        gss_accept_sec_context(&mut (*gr).gr_minor, &mut (*gd).ctx,
                               (*gd).cred, &mut recv_tok,
                               0 as gss_channel_bindings_t,
                               &mut (*gd).client_name, &mut mech,
                               &mut (*gr).gr_token, &mut ret_flags,
                               0 as *mut OM_uint32, 0 as *mut gss_cred_id_t);
    Some((*(*(*rqst).rq_xprt).xp_ops).xp_freeargs.expect("non-null function pointer")).expect("non-null function pointer")((*rqst).rq_xprt,
                                                                                                                           ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                                                   *mut XDR,
                                                                                                                                                                               _:
                                                                                                                                                                                   *mut gss_buffer_desc)
                                                                                                                                                              ->
                                                                                                                                                                  libc::c_int>,
                                                                                                                                                   xdrproc_t>(Some(gssrpc_xdr_rpc_gss_init_args
                                                                                                                                                                       as
                                                                                                                                                                       unsafe extern "C" fn(_:
                                                                                                                                                                                                *mut XDR,
                                                                                                                                                                                            _:
                                                                                                                                                                                                *mut gss_buffer_desc)
                                                                                                                                                                           ->
                                                                                                                                                                               libc::c_int)),
                                                                                                                           &mut recv_tok
                                                                                                                               as
                                                                                                                               *mut gss_buffer_desc
                                                                                                                               as
                                                                                                                               caddr_t
                                                                                                                               as
                                                                                                                               *mut libc::c_void);
    gssrpc_log_status(b"accept_sec_context\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      (*gr).gr_major, (*gr).gr_minor);
    if (*gr).gr_major != 0 as libc::c_int as libc::c_uint &&
           (*gr).gr_major !=
               ((1 as libc::c_int) << 0 as libc::c_int + 0 as libc::c_int) as
                   libc::c_uint {
        badauth((*gr).gr_major, (*gr).gr_minor, (*rqst).rq_xprt);
        (*gd).ctx = 0 as gss_ctx_id_t
    } else {
        (*gr).gr_ctx.value =
            b"xxxx\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_void;
        (*gr).gr_ctx.length = 4 as libc::c_int as size_t;
        /* gr->gr_win = 0x00000005; ANDROS: for debugging linux kernel version...  */
        (*gr).gr_win =
            (::std::mem::size_of::<uint32_t>() as
                 libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                 libc::c_ulong) as uint32_t;
        /* Save client info. */
        (*gd).sec.mech = mech;
        (*gd).sec.qop = 0 as libc::c_int as gss_qop_t;
        (*gd).sec.svc = (*gc).gc_svc;
        (*gd).seq = (*gc).gc_seq;
        (*gd).win = (*gr).gr_win;
        if (*gr).gr_major == 0 as libc::c_int as libc::c_uint {
            maj_stat =
                gss_display_name(&mut min_stat, (*gd).client_name,
                                 &mut (*gd).cname, &mut (*gd).sec.mech);
            if maj_stat != 0 as libc::c_int as libc::c_uint {
                gssrpc_log_status(b"display_name\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, maj_stat, min_stat);
                current_block = 17889447268125949398;
            } else {
                /* DEBUG */
                seq = htonl((*gr).gr_win);
                seqbuf.value =
                    &mut seq as *mut OM_uint32 as *mut libc::c_void;
                seqbuf.length =
                    ::std::mem::size_of::<OM_uint32>() as libc::c_ulong;
                gss_release_buffer(&mut min_stat, &mut (*gd).checksum);
                maj_stat =
                    gss_sign(&mut min_stat, (*gd).ctx, 0 as libc::c_int,
                             &mut seqbuf, &mut (*gd).checksum);
                if maj_stat != 0 as libc::c_int as libc::c_uint {
                    current_block = 17889447268125949398;
                } else {
                    (*(*rqst).rq_xprt).xp_verf.oa_flavor = 6 as libc::c_int;
                    (*(*rqst).rq_xprt).xp_verf.oa_base =
                        (*gd).checksum.value as caddr_t;
                    (*(*rqst).rq_xprt).xp_verf.oa_length =
                        (*gd).checksum.length as u_int;
                    current_block = 1118134448028020070;
                }
            }
        } else { current_block = 1118134448028020070; }
        match current_block {
            17889447268125949398 => { }
            _ => { return 1 as libc::c_int }
        }
    }
    gss_release_buffer(&mut min_stat, &mut (*gr).gr_token);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "293:1"]
unsafe extern "C" fn svcauth_gss_validate(mut rqst: *mut svc_req,
                                          mut gd: *mut svc_rpc_gss_data,
                                          mut msg: *mut rpc_msg)
 -> libc::c_int {
    let mut oa: *mut opaque_auth = 0 as *mut opaque_auth;
    let mut rpcbuf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut checksum: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut maj_stat: OM_uint32 = 0;
    let mut min_stat: OM_uint32 = 0;
    let mut qop_state: OM_uint32 = 0;
    let mut rpchdr: [u_char; 128] = [0; 128];
    let mut buf: *mut int32_t = 0 as *mut int32_t;
    gssrpc_log_debug(b"in svcauth_gss_validate()\x00" as *const u8 as
                         *const libc::c_char);
    memset(rpchdr.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[u_char; 128]>() as libc::c_ulong);
    /* XXX - Reconstruct RPC header for signing (from xdr_callmsg). */
    oa = &mut (*msg).ru.RM_cmb.cb_cred;
    if (*oa).oa_length > 400 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    /* 8 XDR units from the IXDR macro calls. */
    if (::std::mem::size_of::<[u_char; 128]>() as libc::c_ulong) <
           ((8 as libc::c_int * 4 as libc::c_int) as
                libc::c_uint).wrapping_add((*oa).oa_length.wrapping_add(4 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_sub(1
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_uint).wrapping_div(4
                                                                                                                                          as
                                                                                                                                          libc::c_int
                                                                                                                                          as
                                                                                                                                          libc::c_uint).wrapping_mul(4
                                                                                                                                                                         as
                                                                                                                                                                         libc::c_int
                                                                                                                                                                         as
                                                                                                                                                                         libc::c_uint))
               as libc::c_ulong {
        return 0 as libc::c_int
    }
    buf = rpchdr.as_mut_ptr() as *mut libc::c_void as *mut int32_t;
    let fresh0 = buf;
    buf = buf.offset(1);
    *fresh0 = htonl((*msg).rm_xid) as int32_t;
    let fresh1 = buf;
    buf = buf.offset(1);
    *fresh1 = htonl((*msg).rm_direction as int32_t as uint32_t) as int32_t;
    let fresh2 = buf;
    buf = buf.offset(1);
    *fresh2 = htonl((*msg).ru.RM_cmb.cb_rpcvers) as int32_t;
    let fresh3 = buf;
    buf = buf.offset(1);
    *fresh3 = htonl((*msg).ru.RM_cmb.cb_prog) as int32_t;
    let fresh4 = buf;
    buf = buf.offset(1);
    *fresh4 = htonl((*msg).ru.RM_cmb.cb_vers) as int32_t;
    let fresh5 = buf;
    buf = buf.offset(1);
    *fresh5 = htonl((*msg).ru.RM_cmb.cb_proc) as int32_t;
    let fresh6 = buf;
    buf = buf.offset(1);
    *fresh6 = htonl((*oa).oa_flavor as uint32_t) as int32_t;
    let fresh7 = buf;
    buf = buf.offset(1);
    *fresh7 = htonl((*oa).oa_length) as int32_t;
    if (*oa).oa_length != 0 {
        memcpy(buf as caddr_t as *mut libc::c_void,
               (*oa).oa_base as *const libc::c_void,
               (*oa).oa_length as libc::c_ulong);
        buf =
            buf.offset(((*oa).oa_length.wrapping_add(4 as libc::c_int as
                                                         libc::c_uint).wrapping_sub(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint).wrapping_div(4
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_uint).wrapping_mul(4
                                                                                                                                                      as
                                                                                                                                                      libc::c_int
                                                                                                                                                      as
                                                                                                                                                      libc::c_uint)
                            as
                            libc::c_ulong).wrapping_div(::std::mem::size_of::<int32_t>()
                                                            as libc::c_ulong)
                           as isize)
    }
    rpcbuf.value = rpchdr.as_mut_ptr() as *mut libc::c_void;
    rpcbuf.length =
        (buf as *mut u_char).wrapping_offset_from(rpchdr.as_mut_ptr()) as
            libc::c_long as size_t;
    checksum.value = (*msg).ru.RM_cmb.cb_verf.oa_base as *mut libc::c_void;
    checksum.length = (*msg).ru.RM_cmb.cb_verf.oa_length as size_t;
    maj_stat =
        gss_verify_mic(&mut min_stat, (*gd).ctx, &mut rpcbuf, &mut checksum,
                       &mut qop_state);
    if maj_stat != 0 as libc::c_int as libc::c_uint {
        gssrpc_log_status(b"gss_verify_mic\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          maj_stat, min_stat);
        if log_badverf.is_some() {
            Some(log_badverf.expect("non-null function pointer")).expect("non-null function pointer")((*gd).client_name,
                                                                                                      svcauth_gss_name,
                                                                                                      rqst,
                                                                                                      msg,
                                                                                                      log_badverf_data);
        }
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "349:1"]
unsafe extern "C" fn svcauth_gss_nextverf(mut rqst: *mut svc_req,
                                          mut num: u_int) -> libc::c_int {
    let mut gd: *mut svc_rpc_gss_data = 0 as *mut svc_rpc_gss_data;
    let mut signbuf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut maj_stat: OM_uint32 = 0;
    let mut min_stat: OM_uint32 = 0;
    gssrpc_log_debug(b"in svcauth_gss_nextverf()\x00" as *const u8 as
                         *const libc::c_char);
    if (*(*rqst).rq_xprt).xp_auth.is_null() { return 0 as libc::c_int }
    gd =
        *(&mut (*(*(*rqst).rq_xprt).xp_auth).svc_ah_private as
              *mut *mut libc::c_void as *mut *mut svc_rpc_gss_data);
    gss_release_buffer(&mut min_stat, &mut (*gd).checksum);
    signbuf.value = &mut num as *mut u_int as *mut libc::c_void;
    signbuf.length = ::std::mem::size_of::<u_int>() as libc::c_ulong;
    maj_stat =
        gss_get_mic(&mut min_stat, (*gd).ctx, (*gd).sec.qop, &mut signbuf,
                    &mut (*gd).checksum);
    if maj_stat != 0 as libc::c_int as libc::c_uint {
        gssrpc_log_status(b"gss_get_mic\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          maj_stat, min_stat);
        return 0 as libc::c_int
    }
    (*(*rqst).rq_xprt).xp_verf.oa_flavor = 6 as libc::c_int;
    (*(*rqst).rq_xprt).xp_verf.oa_base = (*gd).checksum.value as caddr_t;
    (*(*rqst).rq_xprt).xp_verf.oa_length = (*gd).checksum.length as u_int;
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "382:1"]
pub unsafe extern "C" fn gssrpc__svcauth_gss(mut rqst: *mut svc_req,
                                             mut msg: *mut rpc_msg,
                                             mut no_dispatch:
                                                 *mut libc::c_int)
 -> auth_stat {
    let mut current_block: u64;
    let mut retstat: auth_stat = AUTH_OK;
    let mut xdrs: XDR =
        XDR{x_op: XDR_ENCODE,
            x_ops: 0 as *mut xdr_ops,
            x_public: 0 as *mut libc::c_char,
            x_private: 0 as *mut libc::c_void,
            x_base: 0 as *mut libc::c_char,
            x_handy: 0,};
    let mut auth: *mut SVCAUTH = 0 as *mut SVCAUTH;
    let mut gd: *mut svc_rpc_gss_data = 0 as *mut svc_rpc_gss_data;
    let mut gc: *mut rpc_gss_cred = 0 as *mut rpc_gss_cred;
    let mut gr: rpc_gss_init_res =
        rpc_gss_init_res{gr_ctx:
                             gss_buffer_desc{length: 0,
                                             value: 0 as *mut libc::c_void,},
                         gr_major: 0,
                         gr_minor: 0,
                         gr_win: 0,
                         gr_token:
                             gss_buffer_desc{length: 0,
                                             value:
                                                 0 as *mut libc::c_void,},};
    let mut call_stat: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut min_stat: OM_uint32 = 0;
    gssrpc_log_debug(b"in svcauth_gss()\x00" as *const u8 as
                         *const libc::c_char);
    /* Initialize reply. */
    (*(*rqst).rq_xprt).xp_verf = gssrpc__null_auth;
    /* Allocate and set up server auth handle. */
    if (*(*rqst).rq_xprt).xp_auth.is_null() ||
           (*(*rqst).rq_xprt).xp_auth ==
               &mut gssrpc_svc_auth_none as *mut SVCAUTH {
        auth =
            calloc(::std::mem::size_of::<SVCAUTH>() as libc::c_ulong,
                   1 as libc::c_int as libc::c_ulong) as *mut SVCAUTH;
        if auth.is_null() {
            fprintf(stderr,
                    b"svcauth_gss: out_of_memory\n\x00" as *const u8 as
                        *const libc::c_char);
            return AUTH_FAILED
        }
        gd =
            calloc(::std::mem::size_of::<svc_rpc_gss_data>() as libc::c_ulong,
                   1 as libc::c_int as libc::c_ulong) as
                *mut svc_rpc_gss_data;
        if gd.is_null() {
            fprintf(stderr,
                    b"svcauth_gss: out_of_memory\n\x00" as *const u8 as
                        *const libc::c_char);
            return AUTH_FAILED
        }
        (*auth).svc_ah_ops = &mut gssrpc_svc_auth_gss_ops;
        let ref mut fresh8 =
            *(&mut (*auth).svc_ah_private as *mut *mut libc::c_void as
                  *mut *mut svc_rpc_gss_data);
        *fresh8 = gd;
        (*(*rqst).rq_xprt).xp_auth = auth
    } else {
        gd =
            *(&mut (*(*(*rqst).rq_xprt).xp_auth).svc_ah_private as
                  *mut *mut libc::c_void as *mut *mut svc_rpc_gss_data)
    }
    gssrpc_log_debug(b"xp_auth=%p, gd=%p\x00" as *const u8 as
                         *const libc::c_char, (*(*rqst).rq_xprt).xp_auth, gd);
    /* Deserialize client credentials. */
    if (*rqst).rq_cred.oa_length <= 0 as libc::c_int as libc::c_uint {
        return AUTH_BADCRED
    }
    gc = (*rqst).rq_clntcred as *mut rpc_gss_cred;
    memset(gc as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<rpc_gss_cred>() as libc::c_ulong);
    gssrpc_log_debug(b"calling xdrmem_create()\x00" as *const u8 as
                         *const libc::c_char);
    gssrpc_log_debug(b"oa_base=%p, oa_length=%u\x00" as *const u8 as
                         *const libc::c_char, (*rqst).rq_cred.oa_base,
                     (*rqst).rq_cred.oa_length);
    gssrpc_xdrmem_create(&mut xdrs, (*rqst).rq_cred.oa_base,
                         (*rqst).rq_cred.oa_length, XDR_DECODE);
    gssrpc_log_debug(b"xdrmem_create() returned\x00" as *const u8 as
                         *const libc::c_char);
    if gssrpc_xdr_rpc_gss_cred(&mut xdrs, gc) == 0 {
        gssrpc_log_debug(b"xdr_rpc_gss_cred() failed\x00" as *const u8 as
                             *const libc::c_char);
        if (*xdrs.x_ops).x_destroy.is_some() {
            Some((*xdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut xdrs);
        }
        return AUTH_BADCRED
    }
    if (*xdrs.x_ops).x_destroy.is_some() {
        Some((*xdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut xdrs);
    }
    retstat = AUTH_FAILED;
    /* Check version. */
    if (*gc).gc_v != 1 as libc::c_int as libc::c_uint {
        retstat = AUTH_BADCRED
    } else if (*gc).gc_svc as libc::c_uint !=
                  RPCSEC_GSS_SVC_NONE as libc::c_int as libc::c_uint &&
                  (*gc).gc_svc as libc::c_uint !=
                      RPCSEC_GSS_SVC_INTEGRITY as libc::c_int as libc::c_uint
                  &&
                  (*gc).gc_svc as libc::c_uint !=
                      RPCSEC_GSS_SVC_PRIVACY as libc::c_int as libc::c_uint {
        retstat = AUTH_BADCRED
    } else {
        /* Check RPCSEC_GSS service. */
        /* Check sequence number. */
        if (*gd).established != 0 {
            if (*gc).gc_seq > 0x80000000 as libc::c_uint {
                retstat = RPCSEC_GSS_CTXPROBLEM;
                current_block = 2539039579982765382;
            } else {
                offset =
                    (*gd).seqlast.wrapping_sub((*gc).gc_seq) as libc::c_int;
                if offset < 0 as libc::c_int {
                    (*gd).seqlast = (*gc).gc_seq;
                    offset = 0 as libc::c_int - offset;
                    (*gd).seqmask <<= offset;
                    offset = 0 as libc::c_int;
                    current_block = 13763002826403452995;
                } else if offset as u_int >= (*gd).win ||
                              (*gd).seqmask &
                                  ((1 as libc::c_int) << offset) as
                                      libc::c_uint != 0 {
                    *no_dispatch = 1 as libc::c_int;
                    retstat = RPCSEC_GSS_CTXPROBLEM;
                    current_block = 2539039579982765382;
                } else { current_block = 13763002826403452995; }
                match current_block {
                    2539039579982765382 => { }
                    _ => {
                        (*gd).seq = (*gc).gc_seq;
                        (*gd).seqmask |=
                            ((1 as libc::c_int) << offset) as libc::c_uint;
                        current_block = 16415152177862271243;
                    }
                }
            }
        } else { current_block = 16415152177862271243; }
        match current_block {
            2539039579982765382 => { }
            _ => {
                if (*gd).established != 0 {
                    (*rqst).rq_clntname =
                        (*gd).client_name as *mut libc::c_char as
                            *mut libc::c_void;
                    (*rqst).rq_svccred =
                        (*gd).ctx as *mut libc::c_char as *mut libc::c_void
                }
                /* Handle RPCSEC_GSS control procedure. */
                match (*gc).gc_proc as libc::c_uint {
                    1 | 2 => {
                        if (*rqst).rq_proc != 0 as libc::c_int as rpcproc_t {
                            retstat = AUTH_FAILED; /* XXX ? */
                            current_block = 2539039579982765382; /* XXX ? */
                        } else if svcauth_gss_acquire_cred(gd) == 0 {
                            retstat = AUTH_FAILED;
                            current_block = 2539039579982765382;
                        } else if svcauth_gss_accept_sec_context(rqst,
                                                                 &mut gr) == 0
                         {
                            retstat = AUTH_REJECTEDCRED;
                            current_block = 2539039579982765382;
                        } else if svcauth_gss_nextverf(rqst, htonl(gr.gr_win))
                                      == 0 {
                            gss_release_buffer(&mut min_stat,
                                               &mut gr.gr_token);
                            retstat = AUTH_FAILED;
                            current_block = 2539039579982765382;
                        } else {
                            *no_dispatch = 1 as libc::c_int;
                            call_stat =
                                gssrpc_svc_sendreply((*rqst).rq_xprt,
                                                     ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                             *mut XDR,
                                                                                                         _:
                                                                                                             *mut rpc_gss_init_res)
                                                                                        ->
                                                                                            libc::c_int>,
                                                                             xdrproc_t>(Some(gssrpc_xdr_rpc_gss_init_res
                                                                                                 as
                                                                                                 unsafe extern "C" fn(_:
                                                                                                                          *mut XDR,
                                                                                                                      _:
                                                                                                                          *mut rpc_gss_init_res)
                                                                                                     ->
                                                                                                         libc::c_int)),
                                                     &mut gr as
                                                         *mut rpc_gss_init_res
                                                         as caddr_t);
                            gss_release_buffer(&mut min_stat,
                                               &mut gr.gr_token);
                            gss_release_buffer(&mut min_stat,
                                               &mut (*gd).checksum);
                            if call_stat == 0 {
                                retstat = AUTH_FAILED;
                                current_block = 2539039579982765382;
                            } else {
                                if gr.gr_major ==
                                       0 as libc::c_int as libc::c_uint {
                                    (*gd).established = 1 as libc::c_int
                                }
                                current_block = 8304106758420804164;
                            }
                        }
                    }
                    0 => {
                        if svcauth_gss_validate(rqst, gd, msg) == 0 {
                            retstat = RPCSEC_GSS_CREDPROBLEM;
                            current_block = 2539039579982765382;
                        } else if svcauth_gss_nextverf(rqst,
                                                       htonl((*gc).gc_seq)) ==
                                      0 {
                            retstat = AUTH_FAILED;
                            current_block = 2539039579982765382;
                        } else { current_block = 8304106758420804164; }
                    }
                    3 => {
                        if (*rqst).rq_proc != 0 as libc::c_int as rpcproc_t {
                            retstat = AUTH_FAILED;
                            current_block = 2539039579982765382;
                        } else if svcauth_gss_validate(rqst, gd, msg) == 0 {
                            retstat = RPCSEC_GSS_CREDPROBLEM;
                            current_block = 2539039579982765382;
                        } else if svcauth_gss_nextverf(rqst,
                                                       htonl((*gc).gc_seq)) ==
                                      0 {
                            retstat = AUTH_FAILED;
                            current_block = 2539039579982765382;
                        } else {
                            *no_dispatch = 1 as libc::c_int;
                            call_stat =
                                gssrpc_svc_sendreply((*rqst).rq_xprt,
                                                     ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                             *mut XDR,
                                                                                                         _:
                                                                                                             *mut libc::c_void)
                                                                                        ->
                                                                                            libc::c_int>,
                                                                             xdrproc_t>(Some(gssrpc_xdr_void
                                                                                                 as
                                                                                                 unsafe extern "C" fn(_:
                                                                                                                          *mut XDR,
                                                                                                                      _:
                                                                                                                          *mut libc::c_void)
                                                                                                     ->
                                                                                                         libc::c_int)),
                                                     0 as *mut libc::c_void as
                                                         caddr_t);
                            gssrpc_log_debug(b"sendreply in destroy: %d\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             call_stat);
                            Some((*(*(*(*rqst).rq_xprt).xp_auth).svc_ah_ops).svc_ah_destroy.expect("non-null function pointer")).expect("non-null function pointer")((*(*rqst).rq_xprt).xp_auth);
                            (*(*rqst).rq_xprt).xp_auth =
                                &mut gssrpc_svc_auth_none;
                            current_block = 8304106758420804164;
                        }
                    }
                    _ => {
                        retstat = AUTH_REJECTEDCRED;
                        current_block = 2539039579982765382;
                    }
                }
                match current_block {
                    2539039579982765382 => { }
                    _ => { retstat = AUTH_OK }
                }
            }
        }
    }
    gssrpc_xdr_free(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                            *mut XDR,
                                                                        _:
                                                                            *mut rpc_gss_cred)
                                                       -> libc::c_int>,
                                            xdrproc_t>(Some(gssrpc_xdr_rpc_gss_cred
                                                                as
                                                                unsafe extern "C" fn(_:
                                                                                         *mut XDR,
                                                                                     _:
                                                                                         *mut rpc_gss_cred)
                                                                    ->
                                                                        libc::c_int)),
                    gc as *mut libc::c_void);
    gssrpc_log_debug(b"returning %d from svcauth_gss()\x00" as *const u8 as
                         *const libc::c_char, retstat as libc::c_uint);
    return retstat;
}
#[c2rust::src_loc = "552:1"]
unsafe extern "C" fn svcauth_gss_destroy(mut auth: *mut SVCAUTH)
 -> libc::c_int {
    let mut gd: *mut svc_rpc_gss_data = 0 as *mut svc_rpc_gss_data;
    let mut min_stat: OM_uint32 = 0;
    gssrpc_log_debug(b"in svcauth_gss_destroy()\x00" as *const u8 as
                         *const libc::c_char);
    gd =
        *(&mut (*auth).svc_ah_private as *mut *mut libc::c_void as
              *mut *mut svc_rpc_gss_data);
    gss_delete_sec_context(&mut min_stat, &mut (*gd).ctx, 0 as gss_buffer_t);
    gss_release_cred(&mut min_stat, &mut (*gd).cred);
    gss_release_buffer(&mut min_stat, &mut (*gd).cname);
    gss_release_buffer(&mut min_stat, &mut (*gd).checksum);
    if !(*gd).client_name.is_null() {
        gss_release_name(&mut min_stat, &mut (*gd).client_name);
    }
    free(gd as *mut libc::c_void);
    free(auth as *mut libc::c_void);
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "576:1"]
unsafe extern "C" fn svcauth_gss_wrap(mut auth: *mut SVCAUTH,
                                      mut xdrs: *mut XDR,
                                      mut xdr_func: xdrproc_t,
                                      mut xdr_ptr: caddr_t) -> libc::c_int {
    let mut gd: *mut svc_rpc_gss_data = 0 as *mut svc_rpc_gss_data;
    gssrpc_log_debug(b"in svcauth_gss_wrap()\x00" as *const u8 as
                         *const libc::c_char);
    gd =
        *(&mut (*auth).svc_ah_private as *mut *mut libc::c_void as
              *mut *mut svc_rpc_gss_data);
    if (*gd).established == 0 ||
           (*gd).sec.svc as libc::c_uint ==
               RPCSEC_GSS_SVC_NONE as libc::c_int as libc::c_uint {
        return ::std::mem::transmute::<_,
                                       fn(_: _, _: _)
                                           ->
                                               libc::c_int>(Some(xdr_func.expect("non-null function pointer")).expect("non-null function pointer"))(xdrs,
                                                                                                                                                    xdr_ptr)
    }
    return gssrpc_xdr_rpc_gss_data(xdrs, xdr_func, xdr_ptr, (*gd).ctx,
                                   (*gd).sec.qop, (*gd).sec.svc, (*gd).seq);
}
#[c2rust::src_loc = "593:1"]
unsafe extern "C" fn svcauth_gss_unwrap(mut auth: *mut SVCAUTH,
                                        mut xdrs: *mut XDR,
                                        mut xdr_func: xdrproc_t,
                                        mut xdr_ptr: caddr_t) -> libc::c_int {
    let mut gd: *mut svc_rpc_gss_data = 0 as *mut svc_rpc_gss_data;
    gssrpc_log_debug(b"in svcauth_gss_unwrap()\x00" as *const u8 as
                         *const libc::c_char);
    gd =
        *(&mut (*auth).svc_ah_private as *mut *mut libc::c_void as
              *mut *mut svc_rpc_gss_data);
    if (*gd).established == 0 ||
           (*gd).sec.svc as libc::c_uint ==
               RPCSEC_GSS_SVC_NONE as libc::c_int as libc::c_uint {
        return ::std::mem::transmute::<_,
                                       fn(_: _, _: _)
                                           ->
                                               libc::c_int>(Some(xdr_func.expect("non-null function pointer")).expect("non-null function pointer"))(xdrs,
                                                                                                                                                    xdr_ptr)
    }
    return gssrpc_xdr_rpc_gss_data(xdrs, xdr_func, xdr_ptr, (*gd).ctx,
                                   (*gd).sec.qop, (*gd).sec.svc, (*gd).seq);
}
/* @(#)svc_auth.h	2.1 88/07/29 4.0 RPCSRC */
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
/*      @(#)svc_auth.h 1.6 86/07/16 SMI      */
/*
 * svc_auth.h, Service side of rpc authentication.
 */
/*
 * Interface to server-side authentication flavors.
 */
/*
 * Server side authenticator
 */
/* RENAMED: should be _authenticate. */
/* no authentication */
/* RENAMED: should be _svcauth_none. */
/* unix style (uid, gids) */
/* RENAMED: shoudl be _svcauth_unix. */
/* short hand unix style */
/* RENAMED: should be _svcauth_short. */
/* GSS-API style */
/* RENAMED: should be _svcauth_gssapi. */
/* RPCSEC_GSS */
/* defined(GSSRPC__IMPL) */
/*
 * Approved way of getting principal of caller
 */
#[no_mangle]
#[c2rust::src_loc = "610:1"]
pub unsafe extern "C" fn gssrpc_svcauth_gss_get_principal(mut auth:
                                                              *mut SVCAUTH)
 -> *mut libc::c_char {
    let mut gd: *mut svc_rpc_gss_data = 0 as *mut svc_rpc_gss_data;
    let mut pname: *mut libc::c_char = 0 as *mut libc::c_char;
    gd =
        *(&mut (*auth).svc_ah_private as *mut *mut libc::c_void as
              *mut *mut svc_rpc_gss_data);
    if (*gd).cname.length == 0 as libc::c_int as libc::c_ulong ||
           (*gd).cname.length >= 18446744073709551615 as libc::c_ulong {
        return 0 as *mut libc::c_char
    }
    pname =
        malloc((*gd).cname.length.wrapping_add(1 as libc::c_int as
                                                   libc::c_ulong)) as
            *mut libc::c_char;
    if pname.is_null() { return 0 as *mut libc::c_char }
    memcpy(pname as *mut libc::c_void, (*gd).cname.value, (*gd).cname.length);
    *pname.offset((*gd).cname.length as isize) =
        '\u{0}' as i32 as libc::c_char;
    return pname;
}
/*
 * Function: svcauth_gss_set_log_badauth_func
 *
 * Purpose: sets the logging function called when an invalid RPC call
 * arrives
 *
 * See functional specifications.
 */
#[no_mangle]
#[c2rust::src_loc = "638:1"]
pub unsafe extern "C" fn gssrpc_svcauth_gss_set_log_badauth_func(mut func:
                                                                     auth_gssapi_log_badauth_func,
                                                                 mut data:
                                                                     caddr_t) {
    log_badauth = func;
    log_badauth_data = data;
}
#[no_mangle]
#[c2rust::src_loc = "646:1"]
pub unsafe extern "C" fn gssrpc_svcauth_gss_set_log_badauth2_func(mut func:
                                                                      auth_gssapi_log_badauth2_func,
                                                                  mut data:
                                                                      caddr_t) {
    log_badauth2 = func;
    log_badauth2_data = data;
}
/*
 * Function: svcauth_gss_set_log_badverf_func
 *
 * Purpose: sets the logging function called when an invalid RPC call
 * arrives
 *
 * See functional specifications.
 */
#[no_mangle]
#[c2rust::src_loc = "662:1"]
pub unsafe extern "C" fn gssrpc_svcauth_gss_set_log_badverf_func(mut func:
                                                                     auth_gssapi_log_badverf_func,
                                                                 mut data:
                                                                     caddr_t) {
    log_badverf = func;
    log_badverf_data = data;
}
/*
 * Function: svcauth_gss_set_log_miscerr_func
 *
 * Purpose: sets the logging function called when a miscellaneous
 * AUTH_GSSAPI error occurs
 *
 * See functional specifications.
 */
#[no_mangle]
#[c2rust::src_loc = "678:1"]
pub unsafe extern "C" fn gssrpc_svcauth_gss_set_log_miscerr_func(mut func:
                                                                     auth_gssapi_log_miscerr_func,
                                                                 mut data:
                                                                     caddr_t) {
    log_miscerr = func;
    log_miscerr_data = data;
}
