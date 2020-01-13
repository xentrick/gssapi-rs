use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:39"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:39"]
pub mod types_h {
    #[c2rust::src_loc = "31:1"]
    pub type __u_char = libc::c_uchar;
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
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "162:1"]
    pub type __suseconds_t = libc::c_long;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:39"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:39"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/sys/types.h:40"]
pub mod sys_types_h {
    #[c2rust::src_loc = "33:1"]
    pub type u_char = __u_char;
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_char, __u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:40"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timeval.h:40"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:44"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:44"]
pub mod gssrpc_types_h {
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:45"]
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
    }
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth.h:46"]
pub mod auth_h {
    /* @(#)auth.h	2.3 88/08/07 4.0 RPCSRC; from 1.17 88/02/08 SMI */
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
 * auth.h, Authentication interface.
 *
 * The data structures are completely opaque to the client.  The client
 * is required to pass a AUTH * to routines that create rpc
 * "sessions".
 */
    /* maximum length of network user's name */
    /*
 * Status returned from authentication check
 */
    #[c2rust::src_loc = "55:1"]
    pub type auth_stat = libc::c_uint;
    #[c2rust::src_loc = "74:2"]
    pub const RPCSEC_GSS_CTXPROBLEM: auth_stat = 14;
    /* some unknown reason */
    /*
	 * RPCSEC_GSS errors
	 */
    #[c2rust::src_loc = "73:2"]
    pub const RPCSEC_GSS_CREDPROBLEM: auth_stat = 13;
    /* bogus response verifier */
    #[c2rust::src_loc = "69:2"]
    pub const AUTH_FAILED: auth_stat = 7;
    /* rejected due to security reasons */
    /*
	 * failed locally
	*/
    #[c2rust::src_loc = "68:2"]
    pub const AUTH_INVALIDRESP: auth_stat = 6;
    /* verifier expired or was replayed */
    #[c2rust::src_loc = "64:2"]
    pub const AUTH_TOOWEAK: auth_stat = 5;
    /* bogus verifier (seal broken) */
    #[c2rust::src_loc = "63:2"]
    pub const AUTH_REJECTEDVERF: auth_stat = 4;
    /* client should begin new session */
    #[c2rust::src_loc = "62:2"]
    pub const AUTH_BADVERF: auth_stat = 3;
    /* bogus credentials (seal broken) */
    #[c2rust::src_loc = "61:2"]
    pub const AUTH_REJECTEDCRED: auth_stat = 2;
    /*
	 * failed at remote end
	 */
    #[c2rust::src_loc = "60:2"]
    pub const AUTH_BADCRED: auth_stat = 1;
    #[c2rust::src_loc = "56:2"]
    pub const AUTH_OK: auth_stat = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "77:7"]
    pub union des_block {
        pub c: [libc::c_char; 8],
    }
    /*
 * Authentication info.  Opaque to client.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "86:8"]
    pub struct opaque_auth {
        pub oa_flavor: libc::c_int,
        pub oa_base: caddr_t,
        pub oa_length: u_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "98:16"]
    pub struct AUTH {
        pub ah_cred: opaque_auth,
        pub ah_verf: opaque_auth,
        pub ah_key: des_block,
        pub ah_ops: *mut auth_ops,
        pub ah_private: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "102:9"]
    pub struct auth_ops {
        pub ah_nextverf: Option<unsafe extern "C" fn(_: *mut AUTH) -> ()>,
        pub ah_marshal: Option<unsafe extern "C" fn(_: *mut AUTH, _: *mut XDR)
                                   -> libc::c_int>,
        pub ah_validate: Option<unsafe extern "C" fn(_: *mut AUTH,
                                                     _: *mut opaque_auth)
                                    -> libc::c_int>,
        pub ah_refresh: Option<unsafe extern "C" fn(_: *mut AUTH,
                                                    _: *mut rpc_msg)
                                   -> libc::c_int>,
        pub ah_destroy: Option<unsafe extern "C" fn(_: *mut AUTH) -> ()>,
        pub ah_wrap: Option<unsafe extern "C" fn(_: *mut AUTH, _: *mut XDR,
                                                 _: xdrproc_t, _: caddr_t)
                                -> libc::c_int>,
        pub ah_unwrap: Option<unsafe extern "C" fn(_: *mut AUTH, _: *mut XDR,
                                                   _: xdrproc_t, _: caddr_t)
                                  -> libc::c_int>,
    }
    use super::sys_types_h::{caddr_t, u_int};
    use super::xdr_h::{XDR, xdrproc_t};
    use super::rpc_msg_h::rpc_msg;
    extern "C" {
        /*
 * Authentication ops.
 * The ops and the auth handle provide the interface to the authenticators.
 *
 * AUTH	*auth;
 * XDR	*xdrs;
 * struct opaque_auth verf;
 */
        /* RENAMED: should be _null_auth if we can use reserved namespace. */
        #[no_mangle]
        #[c2rust::src_loc = "173:27"]
        pub static mut gssrpc__null_auth: opaque_auth;
        #[no_mangle]
        #[c2rust::src_loc = "194:1"]
        pub fn gssrpc_xdr_opaque_auth(_: *mut XDR, _: *mut opaque_auth)
         -> libc::c_int;
    }
    /* !defined(GSSRPC_AUTH_H) */
    /* RPCSEC_GSS */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc_msg.h:47"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/clnt.h:47"]
pub mod clnt_h {
    /* @(#)clnt.h	2.1 88/07/29 4.0 RPCSRC; from 1.31 88/02/08 SMI*/
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
 * clnt.h - Client side remote procedure call interface.
 */
    /*
 * Rpc calls return an enum clnt_stat.  This should be looked at more,
 * since each implementation is required to live with this (implementation
 * independent) list of errors.
 */
    #[c2rust::src_loc = "48:1"]
    pub type clnt_stat = libc::c_uint;
    /* remote program is not registered */
    /*
	 * unspecified error
	 */
    #[c2rust::src_loc = "83:2"]
    pub const RPC_FAILED: clnt_stat = 16;
    /* the pmapper failed in its call */
    #[c2rust::src_loc = "79:2"]
    pub const RPC_PROGNOTREGISTERED: clnt_stat = 15;
    /* unknown protocol */
    /*
	 * _ create errors
	 */
    #[c2rust::src_loc = "78:2"]
    pub const RPC_PMAPFAILURE: clnt_stat = 14;
    /* unknown host name */
    #[c2rust::src_loc = "73:2"]
    pub const RPC_UNKNOWNPROTO: clnt_stat = 17;
    /* generic "other problem" */
    /*
	 * callrpc & clnt_create errors
	 */
    #[c2rust::src_loc = "72:2"]
    pub const RPC_UNKNOWNHOST: clnt_stat = 13;
    /* decode arguments error */
    #[c2rust::src_loc = "67:2"]
    pub const RPC_SYSTEMERROR: clnt_stat = 12;
    /* procedure unavailable */
    #[c2rust::src_loc = "66:2"]
    pub const RPC_CANTDECODEARGS: clnt_stat = 11;
    /* program version mismatched */
    #[c2rust::src_loc = "65:2"]
    pub const RPC_PROCUNAVAIL: clnt_stat = 10;
    /* program not available */
    #[c2rust::src_loc = "64:2"]
    pub const RPC_PROGVERSMISMATCH: clnt_stat = 9;
    /* authentication error */
    #[c2rust::src_loc = "63:2"]
    pub const RPC_PROGUNAVAIL: clnt_stat = 8;
    /* rpc versions not compatible */
    #[c2rust::src_loc = "62:2"]
    pub const RPC_AUTHERROR: clnt_stat = 7;
    /* call timed out */
    /*
	 * remote errors
	 */
    #[c2rust::src_loc = "61:2"]
    pub const RPC_VERSMISMATCH: clnt_stat = 6;
    /* failure in receiving result */
    #[c2rust::src_loc = "57:2"]
    pub const RPC_TIMEDOUT: clnt_stat = 5;
    /* failure in sending call */
    #[c2rust::src_loc = "56:2"]
    pub const RPC_CANTRECV: clnt_stat = 4;
    /* can't decode results */
    #[c2rust::src_loc = "55:2"]
    pub const RPC_CANTSEND: clnt_stat = 3;
    /* can't encode arguments */
    #[c2rust::src_loc = "54:2"]
    pub const RPC_CANTDECODERES: clnt_stat = 2;
    /* call succeeded */
    /*
	 * local errors
	 */
    #[c2rust::src_loc = "53:2"]
    pub const RPC_CANTENCODEARGS: clnt_stat = 1;
    #[c2rust::src_loc = "49:2"]
    pub const RPC_SUCCESS: clnt_stat = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "90:8"]
    pub struct rpc_err {
        pub re_status: clnt_stat,
        pub ru: C2RustUnnamed_6,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:2"]
    pub union C2RustUnnamed_6 {
        pub RE_errno: libc::c_int,
        pub RE_why: auth_stat,
        pub RE_vers: C2RustUnnamed_8,
        pub RE_lb: C2RustUnnamed_7,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "99:3"]
    pub struct C2RustUnnamed_7 {
        pub s1: int32_t,
        pub s2: int32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "95:3"]
    pub struct C2RustUnnamed_8 {
        pub low: rpcvers_t,
        pub high: rpcvers_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "116:16"]
    pub struct CLIENT {
        pub cl_auth: *mut AUTH,
        pub cl_ops: *mut clnt_ops,
        pub cl_private: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:9"]
    pub struct clnt_ops {
        pub cl_call: Option<unsafe extern "C" fn(_: *mut CLIENT, _: rpcproc_t,
                                                 _: xdrproc_t,
                                                 _: *mut libc::c_void,
                                                 _: xdrproc_t,
                                                 _: *mut libc::c_void,
                                                 _: timeval) -> clnt_stat>,
        pub cl_abort: Option<unsafe extern "C" fn(_: *mut CLIENT) -> ()>,
        pub cl_geterr: Option<unsafe extern "C" fn(_: *mut CLIENT,
                                                   _: *mut rpc_err) -> ()>,
        pub cl_freeres: Option<unsafe extern "C" fn(_: *mut CLIENT,
                                                    _: xdrproc_t,
                                                    _: *mut libc::c_void)
                                   -> libc::c_int>,
        pub cl_destroy: Option<unsafe extern "C" fn(_: *mut CLIENT) -> ()>,
        pub cl_control: Option<unsafe extern "C" fn(_: *mut CLIENT,
                                                    _: libc::c_int,
                                                    _: *mut libc::c_void)
                                   -> libc::c_int>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "327:8"]
    pub struct gssrpc_rpc_createrr {
        pub cf_stat: clnt_stat,
        pub cf_error: rpc_err,
    }
    use super::auth_h::{auth_stat, AUTH};
    use super::stdint_intn_h::int32_t;
    use super::gssrpc_types_h::{rpcvers_t, rpcproc_t};
    use super::xdr_h::xdrproc_t;
    use super::struct_timeval_h::timeval;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "332:29"]
        pub static mut gssrpc_rpc_createrr: gssrpc_rpc_createrr;
        #[no_mangle]
        #[c2rust::src_loc = "322:1"]
        pub fn gssrpc_clnt_sperror(_: *mut CLIENT, _: *mut libc::c_char)
         -> *mut libc::c_char;
    }
    /* !defined(GSSRPC_CLNT_H) */
    /* a more reasonable packet size */
    /* rpc imposed limit on udp msg size */
    /* string */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:47"]
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
        /* cred_handle */
        #[no_mangle]
        #[c2rust::src_loc = "437:1"]
        pub fn gss_init_sec_context(_: *mut OM_uint32, _: gss_cred_id_t,
                                    _: *mut gss_ctx_id_t, _: gss_name_t,
                                    _: gss_OID, _: OM_uint32, _: OM_uint32,
                                    _: gss_channel_bindings_t,
                                    _: gss_buffer_t, _: *mut gss_OID,
                                    _: gss_buffer_t, _: *mut OM_uint32,
                                    _: *mut OM_uint32) -> OM_uint32;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth_gss.h:47"]
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
    #[c2rust::src_loc = "59:9"]
    pub type rpc_gss_svc_t = libc::c_uint;
    #[c2rust::src_loc = "62:2"]
    pub const RPCSEC_GSS_SVC_PRIVACY: rpc_gss_svc_t = 3;
    #[c2rust::src_loc = "61:2"]
    pub const RPCSEC_GSS_SVC_INTEGRITY: rpc_gss_svc_t = 2;
    #[c2rust::src_loc = "60:2"]
    pub const RPCSEC_GSS_SVC_NONE: rpc_gss_svc_t = 1;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "77:8"]
    pub struct authgss_private_data {
        pub pd_ctx: gss_ctx_id_t,
        pub pd_ctx_hndl: gss_buffer_desc,
        pub pd_seq_win: uint32_t,
    }
    /* RPCSEC_GSS services. */
    /* RPCSEC_GSS security triple. */
    /* mechanism */
    /* quality of protection */
    /* service */
    /* cred handle */
    /* req flags for init_sec_context */
    /* Private data required for kernel implementation */
    /* Session context handle */
    /* Credentials context handle */
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
    use super::gssapi_h::{gss_OID, gss_qop_t, gss_cred_id_t, gss_ctx_id_t,
                          gss_buffer_desc, gss_ctx_id_struct, OM_uint32};
    use super::stdint_uintn_h::uint32_t;
    use super::sys_types_h::{u_int, caddr_t};
    use super::xdr_h::{XDR, xdrproc_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "125:1"]
        pub fn gssrpc_xdr_rpc_gss_data(xdrs: *mut XDR, xdr_func: xdrproc_t,
                                       xdr_ptr: caddr_t, ctx: gss_ctx_id_t,
                                       qop: gss_qop_t, svc: rpc_gss_svc_t,
                                       seq: uint32_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "124:1"]
        pub fn gssrpc_xdr_rpc_gss_init_res(xdrs: *mut XDR,
                                           p: *mut rpc_gss_init_res)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "123:1"]
        pub fn gssrpc_xdr_rpc_gss_init_args(xdrs: *mut XDR,
                                            p: *mut gss_buffer_desc)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "122:1"]
        pub fn gssrpc_xdr_rpc_gss_cred(xdrs: *mut XDR, p: *mut rpc_gss_cred)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "143:1"]
        pub fn gssrpc_log_status(m: *mut libc::c_char, major: OM_uint32,
                                 minor: OM_uint32);
        #[no_mangle]
        #[c2rust::src_loc = "142:1"]
        pub fn gssrpc_log_debug(fmt: *const libc::c_char, _: ...);
    }
    /* !defined(GSSRPC_AUTH_GSS_H) */
}
#[c2rust::header_src = "/usr/include/stdio.h:39"]
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
#[c2rust::header_src = "/usr/include/stdlib.h:40"]
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
#[c2rust::header_src = "/usr/include/string.h:42"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/netinet/in.h:44"]
pub mod in_h {
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "378:1"]
        pub fn htonl(__hostlong: uint32_t) -> uint32_t;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_generic.h:55"]
pub mod gssapi_generic_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "54:27"]
        pub static mut gss_nt_service_name: gss_OID;
    }
    /* _GSSAPI_GENERIC_H_ */
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__u_char, __u_int, __int32_t, __uint32_t, __off_t,
                        __off64_t, __time_t, __suseconds_t, __caddr_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::sys_types_h::{u_char, u_int, caddr_t};
pub use self::stdint_intn_h::int32_t;
pub use self::struct_timeval_h::timeval;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t, rpc_inline_t};
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, gssrpc_xdr_void, gssrpc_xdrmem_create};
pub use self::auth_h::{auth_stat, RPCSEC_GSS_CTXPROBLEM,
                       RPCSEC_GSS_CREDPROBLEM, AUTH_FAILED, AUTH_INVALIDRESP,
                       AUTH_TOOWEAK, AUTH_REJECTEDVERF, AUTH_BADVERF,
                       AUTH_REJECTEDCRED, AUTH_BADCRED, AUTH_OK, des_block,
                       opaque_auth, AUTH, auth_ops, gssrpc__null_auth,
                       gssrpc_xdr_opaque_auth};
pub use self::rpc_msg_h::{rpc_msg, C2RustUnnamed, reply_body, C2RustUnnamed_0,
                          rejected_reply, C2RustUnnamed_1, C2RustUnnamed_2,
                          reject_stat, AUTH_ERROR, RPC_MISMATCH,
                          accepted_reply, C2RustUnnamed_3, C2RustUnnamed_4,
                          C2RustUnnamed_5, accept_stat, SYSTEM_ERR,
                          GARBAGE_ARGS, PROC_UNAVAIL, PROG_MISMATCH,
                          PROG_UNAVAIL, SUCCESS, reply_stat, MSG_DENIED,
                          MSG_ACCEPTED, call_body, msg_type, REPLY, CALL};
pub use self::clnt_h::{clnt_stat, RPC_FAILED, RPC_PROGNOTREGISTERED,
                       RPC_PMAPFAILURE, RPC_UNKNOWNPROTO, RPC_UNKNOWNHOST,
                       RPC_SYSTEMERROR, RPC_CANTDECODEARGS, RPC_PROCUNAVAIL,
                       RPC_PROGVERSMISMATCH, RPC_PROGUNAVAIL, RPC_AUTHERROR,
                       RPC_VERSMISMATCH, RPC_TIMEDOUT, RPC_CANTRECV,
                       RPC_CANTSEND, RPC_CANTDECODERES, RPC_CANTENCODEARGS,
                       RPC_SUCCESS, rpc_err, C2RustUnnamed_6, C2RustUnnamed_7,
                       C2RustUnnamed_8, CLIENT, clnt_ops, gssrpc_rpc_createrr,
                       gssrpc_clnt_sperror};
pub use self::gssapi_h::{gss_name_t, gss_cred_id_t, gss_ctx_id_t, gss_uint32,
                         OM_uint32, gss_OID_desc_struct, gss_OID,
                         gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_channel_bindings_struct,
                         gss_channel_bindings_t, gss_qop_t, gss_name_struct,
                         gss_cred_id_struct, gss_ctx_id_struct,
                         gss_init_sec_context, gss_delete_sec_context,
                         gss_get_mic, gss_verify_mic, gss_import_name,
                         gss_release_name, gss_release_buffer,
                         gss_duplicate_name};
pub use self::auth_gss_h::{rpc_gss_proc_t, RPCSEC_GSS_DESTROY,
                           RPCSEC_GSS_CONTINUE_INIT, RPCSEC_GSS_INIT,
                           RPCSEC_GSS_DATA, rpc_gss_svc_t,
                           RPCSEC_GSS_SVC_PRIVACY, RPCSEC_GSS_SVC_INTEGRITY,
                           RPCSEC_GSS_SVC_NONE, rpc_gss_sec,
                           authgss_private_data, rpc_gss_cred,
                           rpc_gss_init_res, gssrpc_xdr_rpc_gss_data,
                           gssrpc_xdr_rpc_gss_init_res,
                           gssrpc_xdr_rpc_gss_init_args,
                           gssrpc_xdr_rpc_gss_cred, gssrpc_log_status,
                           gssrpc_log_debug};
use self::stdio_h::{stderr, fprintf};
use self::stdlib_h::{malloc, calloc, free};
use self::string_h::{memcpy, memset, strlen};
use self::in_h::htonl;
use self::gssapi_generic_h::gss_nt_service_name;
/*DEBUG*/
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "143:8"]
pub struct rpc_gss_data {
    pub established: libc::c_int,
    pub inprogress: libc::c_int,
    pub gc_wire_verf: gss_buffer_desc,
    pub clnt: *mut CLIENT,
    pub name: gss_name_t,
    pub sec: rpc_gss_sec,
    pub ctx: gss_ctx_id_t,
    pub gc: rpc_gss_cred,
    pub win: uint32_t,
}
/* lib/rpc/auth_gss.c */
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

  Id: auth_gss.c,v 1.35 2002/10/15 21:25:25 kwc Exp
*/
/* RPCSEC_GSS client routines. */
#[no_mangle]
#[c2rust::src_loc = "59:5"]
pub static mut gssrpc_auth_debug_gss: libc::c_int = 0 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "60:5"]
pub static mut gssrpc_misc_debug_gss: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "82:24"]
static mut authgss_ops: auth_ops =
    unsafe {
        {
            let mut init =
                auth_ops{ah_nextverf:
                             Some(authgss_nextverf as
                                      unsafe extern "C" fn(_: *mut AUTH)
                                          -> ()),
                         ah_marshal:
                             Some(authgss_marshal as
                                      unsafe extern "C" fn(_: *mut AUTH,
                                                           _: *mut XDR)
                                          -> libc::c_int),
                         ah_validate:
                             Some(authgss_validate as
                                      unsafe extern "C" fn(_: *mut AUTH,
                                                           _:
                                                               *mut opaque_auth)
                                          -> libc::c_int),
                         ah_refresh:
                             Some(authgss_refresh as
                                      unsafe extern "C" fn(_: *mut AUTH,
                                                           _: *mut rpc_msg)
                                          -> libc::c_int),
                         ah_destroy:
                             Some(authgss_destroy as
                                      unsafe extern "C" fn(_: *mut AUTH)
                                          -> ()),
                         ah_wrap:
                             Some(authgss_wrap as
                                      unsafe extern "C" fn(_: *mut AUTH,
                                                           _: *mut XDR,
                                                           _: xdrproc_t,
                                                           _: caddr_t)
                                          -> libc::c_int),
                         ah_unwrap:
                             Some(authgss_unwrap as
                                      unsafe extern "C" fn(_: *mut AUTH,
                                                           _: *mut XDR,
                                                           _: xdrproc_t,
                                                           _: caddr_t)
                                          -> libc::c_int),};
            init
        }
    };
/* sequence window */
#[c2rust::src_loc = "158:23"]
static mut AUTH_TIMEOUT: timeval =
    {
        let mut init =
            timeval{tv_sec: 25 as libc::c_int as __time_t,
                    tv_usec: 0 as libc::c_int as __suseconds_t,};
        init
    };
#[no_mangle]
#[c2rust::src_loc = "160:1"]
pub unsafe extern "C" fn gssrpc_authgss_create(mut clnt: *mut CLIENT,
                                               mut name: gss_name_t,
                                               mut sec: *mut rpc_gss_sec)
 -> *mut AUTH {
    let mut auth: *mut AUTH = 0 as *mut AUTH;
    let mut save_auth: *mut AUTH = 0 as *mut AUTH;
    let mut gd: *mut rpc_gss_data = 0 as *mut rpc_gss_data;
    let mut min_stat: OM_uint32 = 0 as libc::c_int as OM_uint32;
    gssrpc_log_debug(b"in authgss_create()\x00" as *const u8 as
                         *const libc::c_char);
    memset(&mut gssrpc_rpc_createrr as *mut gssrpc_rpc_createrr as
               *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<gssrpc_rpc_createrr>() as libc::c_ulong);
    auth =
        calloc(::std::mem::size_of::<AUTH>() as libc::c_ulong,
               1 as libc::c_int as libc::c_ulong) as *mut AUTH;
    if auth.is_null() {
        gssrpc_rpc_createrr.cf_stat = RPC_SYSTEMERROR;
        gssrpc_rpc_createrr.cf_error.ru.RE_errno = 12 as libc::c_int;
        return 0 as *mut AUTH
    }
    gd =
        calloc(::std::mem::size_of::<rpc_gss_data>() as libc::c_ulong,
               1 as libc::c_int as libc::c_ulong) as *mut rpc_gss_data;
    if gd.is_null() {
        gssrpc_rpc_createrr.cf_stat = RPC_SYSTEMERROR;
        gssrpc_rpc_createrr.cf_error.ru.RE_errno = 12 as libc::c_int;
        free(auth as *mut libc::c_void);
        return 0 as *mut AUTH
    }
    if !name.is_null() {
        if gss_duplicate_name(&mut min_stat, name, &mut (*gd).name) !=
               0 as libc::c_int as libc::c_uint {
            gssrpc_rpc_createrr.cf_stat = RPC_SYSTEMERROR;
            gssrpc_rpc_createrr.cf_error.ru.RE_errno = 12 as libc::c_int;
            free(auth as *mut libc::c_void);
            free(gd as *mut libc::c_void);
            return 0 as *mut AUTH
        }
    } else { (*gd).name = name }
    (*gd).clnt = clnt;
    (*gd).ctx = 0 as gss_ctx_id_t;
    (*gd).sec = *sec;
    (*gd).gc.gc_v = 1 as libc::c_int as u_int;
    (*gd).gc.gc_proc = RPCSEC_GSS_INIT;
    (*gd).gc.gc_svc = (*gd).sec.svc;
    (*auth).ah_ops = &mut authgss_ops;
    (*auth).ah_private = gd as caddr_t as *mut libc::c_void;
    save_auth = (*clnt).cl_auth;
    (*clnt).cl_auth = auth;
    if authgss_refresh(auth, 0 as *mut rpc_msg) == 0 { auth = 0 as *mut AUTH }
    (*clnt).cl_auth = save_auth;
    gssrpc_log_debug(b"authgss_create returning auth 0x%08x\x00" as *const u8
                         as *const libc::c_char, auth);
    return auth;
}
#[no_mangle]
#[c2rust::src_loc = "218:1"]
pub unsafe extern "C" fn gssrpc_authgss_create_default(mut clnt: *mut CLIENT,
                                                       mut service:
                                                           *mut libc::c_char,
                                                       mut sec:
                                                           *mut rpc_gss_sec)
 -> *mut AUTH {
    let mut auth: *mut AUTH = 0 as *mut AUTH;
    let mut maj_stat: OM_uint32 = 0 as libc::c_int as OM_uint32;
    let mut min_stat: OM_uint32 = 0 as libc::c_int as OM_uint32;
    let mut sname: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut name: gss_name_t = 0 as *mut gss_name_struct;
    gssrpc_log_debug(b"in authgss_create_default()\x00" as *const u8 as
                         *const libc::c_char);
    sname.value = service as *mut libc::c_void;
    sname.length = strlen(service);
    maj_stat =
        gss_import_name(&mut min_stat, &mut sname, gss_nt_service_name,
                        &mut name);
    if maj_stat != 0 as libc::c_int as libc::c_uint {
        gssrpc_log_status(b"gss_import_name\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          maj_stat, min_stat);
        gssrpc_rpc_createrr.cf_stat = RPC_AUTHERROR;
        return 0 as *mut AUTH
    }
    auth = gssrpc_authgss_create(clnt, name, sec);
    if !name.is_null() { gss_release_name(&mut min_stat, &mut name); }
    gssrpc_log_debug(b"authgss_create_default returning auth 0x%08x\x00" as
                         *const u8 as *const libc::c_char, auth);
    return auth;
}
#[no_mangle]
#[c2rust::src_loc = "251:1"]
pub unsafe extern "C" fn gssrpc_authgss_get_private_data(mut auth: *mut AUTH,
                                                         mut pd:
                                                             *mut authgss_private_data)
 -> libc::c_int {
    let mut gd: *mut rpc_gss_data = 0 as *mut rpc_gss_data;
    gssrpc_log_debug(b"in authgss_get_private_data()\x00" as *const u8 as
                         *const libc::c_char);
    if auth.is_null() || pd.is_null() { return 0 as libc::c_int }
    gd = (*auth).ah_private as *mut rpc_gss_data;
    if gd.is_null() || (*gd).established == 0 { return 0 as libc::c_int }
    (*pd).pd_ctx = (*gd).ctx;
    (*pd).pd_ctx_hndl = (*gd).gc.gc_ctx;
    (*pd).pd_seq_win = (*gd).win;
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "273:1"]
unsafe extern "C" fn authgss_nextverf(mut auth: *mut AUTH) {
    gssrpc_log_debug(b"in authgss_nextverf()\n\x00" as *const u8 as
                         *const libc::c_char);
    /* no action necessary */
}
#[c2rust::src_loc = "280:1"]
unsafe extern "C" fn authgss_marshal(mut auth: *mut AUTH, mut xdrs: *mut XDR)
 -> libc::c_int {
    let mut tmpxdrs: XDR =
        XDR{x_op: XDR_ENCODE,
            x_ops: 0 as *mut xdr_ops,
            x_public: 0 as *mut libc::c_char,
            x_private: 0 as *mut libc::c_void,
            x_base: 0 as *mut libc::c_char,
            x_handy: 0,};
    let mut tmp: [libc::c_char; 400] = [0; 400];
    let mut gd: *mut rpc_gss_data = 0 as *mut rpc_gss_data;
    let mut rpcbuf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut checksum: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut maj_stat: OM_uint32 = 0;
    let mut min_stat: OM_uint32 = 0;
    let mut xdr_stat: libc::c_int = 0;
    gssrpc_log_debug(b"in authgss_marshal()\x00" as *const u8 as
                         *const libc::c_char);
    gd = (*auth).ah_private as *mut rpc_gss_data;
    if (*gd).established != 0 {
        (*gd).gc.gc_seq = (*gd).gc.gc_seq.wrapping_add(1)
    }
    gssrpc_xdrmem_create(&mut tmpxdrs, tmp.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 400]>() as
                             libc::c_ulong as u_int, XDR_ENCODE);
    if gssrpc_xdr_rpc_gss_cred(&mut tmpxdrs, &mut (*gd).gc) == 0 {
        if (*tmpxdrs.x_ops).x_destroy.is_some() {
            Some((*tmpxdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut tmpxdrs);
        }
        return 0 as libc::c_int
    }
    (*auth).ah_cred.oa_flavor = 6 as libc::c_int;
    (*auth).ah_cred.oa_base = tmp.as_mut_ptr();
    (*auth).ah_cred.oa_length =
        Some((*tmpxdrs.x_ops).x_getpostn.expect("non-null function pointer")).expect("non-null function pointer")(&mut tmpxdrs);
    if (*tmpxdrs.x_ops).x_destroy.is_some() {
        Some((*tmpxdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut tmpxdrs);
    }
    if gssrpc_xdr_opaque_auth(xdrs, &mut (*auth).ah_cred) == 0 {
        return 0 as libc::c_int
    }
    if (*gd).gc.gc_proc as libc::c_uint ==
           RPCSEC_GSS_INIT as libc::c_int as libc::c_uint ||
           (*gd).gc.gc_proc as libc::c_uint ==
               RPCSEC_GSS_CONTINUE_INIT as libc::c_int as libc::c_uint {
        return gssrpc_xdr_opaque_auth(xdrs, &mut gssrpc__null_auth)
    }
    /* Checksum serialized RPC header, up to and including credential. */
    rpcbuf.length =
        Some((*(*xdrs).x_ops).x_getpostn.expect("non-null function pointer")).expect("non-null function pointer")(xdrs)
            as size_t;
    Some((*(*xdrs).x_ops).x_setpostn.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                              0
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  u_int);
    rpcbuf.value =
        Some((*(*xdrs).x_ops).x_inline.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                rpcbuf.length
                                                                                                                    as
                                                                                                                    libc::c_int)
            as *mut libc::c_void;
    maj_stat =
        gss_get_mic(&mut min_stat, (*gd).ctx, (*gd).sec.qop, &mut rpcbuf,
                    &mut checksum);
    if maj_stat != 0 as libc::c_int as libc::c_uint {
        gssrpc_log_status(b"gss_get_mic\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          maj_stat, min_stat);
        if maj_stat == (12 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
           {
            (*gd).established = 0 as libc::c_int;
            authgss_destroy_context(auth);
        }
        return 0 as libc::c_int
    }
    (*auth).ah_verf.oa_flavor = 6 as libc::c_int;
    (*auth).ah_verf.oa_base = checksum.value as caddr_t;
    (*auth).ah_verf.oa_length = checksum.length as u_int;
    xdr_stat = gssrpc_xdr_opaque_auth(xdrs, &mut (*auth).ah_verf);
    gss_release_buffer(&mut min_stat, &mut checksum);
    return xdr_stat;
}
#[c2rust::src_loc = "342:1"]
unsafe extern "C" fn authgss_validate(mut auth: *mut AUTH,
                                      mut verf: *mut opaque_auth)
 -> libc::c_int {
    let mut gd: *mut rpc_gss_data = 0 as *mut rpc_gss_data;
    let mut num: uint32_t = 0;
    let mut qop_state: gss_qop_t = 0;
    let mut signbuf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut checksum: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut maj_stat: OM_uint32 = 0;
    let mut min_stat: OM_uint32 = 0;
    gssrpc_log_debug(b"in authgss_validate()\x00" as *const u8 as
                         *const libc::c_char);
    gd = (*auth).ah_private as *mut rpc_gss_data;
    if (*gd).established == 0 as libc::c_int {
        /* would like to do this only on NULL rpc - gc->established is good enough.
		 * save the on the wire verifier to validate last INIT phase packet
		 * after decode if the major status is GSS_S_COMPLETE
		 */
        (*gd).gc_wire_verf.value = malloc((*verf).oa_length as libc::c_ulong);
        if (*gd).gc_wire_verf.value.is_null() {
            fprintf(stderr,
                    b"gss_validate: out of memory\n\x00" as *const u8 as
                        *const libc::c_char);
            return 0 as libc::c_int
        }
        memcpy((*gd).gc_wire_verf.value,
               (*verf).oa_base as *const libc::c_void,
               (*verf).oa_length as libc::c_ulong);
        (*gd).gc_wire_verf.length = (*verf).oa_length as size_t;
        return 1 as libc::c_int
    }
    if (*gd).gc.gc_proc as libc::c_uint ==
           RPCSEC_GSS_INIT as libc::c_int as libc::c_uint ||
           (*gd).gc.gc_proc as libc::c_uint ==
               RPCSEC_GSS_CONTINUE_INIT as libc::c_int as libc::c_uint {
        num = htonl((*gd).win)
    } else { num = htonl((*gd).gc.gc_seq) }
    signbuf.value = &mut num as *mut uint32_t as *mut libc::c_void;
    signbuf.length = ::std::mem::size_of::<uint32_t>() as libc::c_ulong;
    checksum.value = (*verf).oa_base as *mut libc::c_void;
    checksum.length = (*verf).oa_length as size_t;
    maj_stat =
        gss_verify_mic(&mut min_stat, (*gd).ctx, &mut signbuf, &mut checksum,
                       &mut qop_state);
    if maj_stat != 0 as libc::c_int as libc::c_uint ||
           qop_state != (*gd).sec.qop {
        gssrpc_log_status(b"gss_verify_mic\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          maj_stat, min_stat);
        if maj_stat == (12 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
           {
            (*gd).established = 0 as libc::c_int;
            authgss_destroy_context(auth);
        }
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "394:1"]
unsafe extern "C" fn authgss_refresh(mut auth: *mut AUTH,
                                     mut msg: *mut rpc_msg) -> libc::c_int {
    let mut gd: *mut rpc_gss_data = 0 as *mut rpc_gss_data;
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
    let mut recv_tokenp: *mut gss_buffer_desc = 0 as *mut gss_buffer_desc;
    let mut send_token: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut maj_stat: OM_uint32 = 0;
    let mut min_stat: OM_uint32 = 0;
    let mut call_stat: OM_uint32 = 0;
    let mut ret_flags: OM_uint32 = 0;
    gssrpc_log_debug(b"in authgss_refresh()\x00" as *const u8 as
                         *const libc::c_char);
    gd = (*auth).ah_private as *mut rpc_gss_data;
    if (*gd).established != 0 || (*gd).inprogress != 0 {
        return 1 as libc::c_int
    }
    /* GSS context establishment loop. */
    memset(&mut gr as *mut rpc_gss_init_res as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<rpc_gss_init_res>() as libc::c_ulong);
    recv_tokenp = 0 as gss_buffer_t;
    loop 
         /*DEBUG*/
         {
        (*gd).inprogress = 1 as libc::c_int; /* time rec */
        maj_stat =
            gss_init_sec_context(&mut min_stat, (*gd).sec.cred,
                                 &mut (*gd).ctx, (*gd).name, (*gd).sec.mech,
                                 (*gd).sec.req_flags,
                                 0 as libc::c_int as OM_uint32,
                                 0 as gss_channel_bindings_t, recv_tokenp,
                                 0 as *mut gss_OID, &mut send_token,
                                 &mut ret_flags, 0 as *mut OM_uint32);
        gssrpc_log_status(b"gss_init_sec_context\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          maj_stat, min_stat);
        if !recv_tokenp.is_null() {
            free(gr.gr_token.value);
            gr.gr_token.value = 0 as *mut libc::c_void;
            recv_tokenp = 0 as gss_buffer_t
        }
        if maj_stat != 0 as libc::c_int as libc::c_uint &&
               maj_stat !=
                   ((1 as libc::c_int) << 0 as libc::c_int + 0 as libc::c_int)
                       as libc::c_uint {
            gssrpc_log_status(b"gss_init_sec_context (error)\x00" as *const u8
                                  as *const libc::c_char as *mut libc::c_char,
                              maj_stat, min_stat);
            break ;
        } else {
            if send_token.length != 0 as libc::c_int as libc::c_ulong {
                memset(&mut gr as *mut rpc_gss_init_res as *mut libc::c_void,
                       0 as libc::c_int,
                       ::std::mem::size_of::<rpc_gss_init_res>() as
                           libc::c_ulong);
                call_stat =
                    Some((*(*(*gd).clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")((*gd).clnt,
                                                                                                                                  0
                                                                                                                                      as
                                                                                                                                      libc::c_int
                                                                                                                                      as
                                                                                                                                      rpcproc_t,
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
                                                                                                                                  &mut send_token
                                                                                                                                      as
                                                                                                                                      *mut gss_buffer_desc
                                                                                                                                      as
                                                                                                                                      *mut libc::c_void,
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
                                                                                                                                  &mut gr
                                                                                                                                      as
                                                                                                                                      *mut rpc_gss_init_res
                                                                                                                                      as
                                                                                                                                      caddr_t
                                                                                                                                      as
                                                                                                                                      *mut libc::c_void,
                                                                                                                                  AUTH_TIMEOUT)
                        as OM_uint32;
                gss_release_buffer(&mut min_stat, &mut send_token);
                gssrpc_log_debug(b"authgss_refresh: call_stat=%d\x00" as
                                     *const u8 as *const libc::c_char,
                                 call_stat);
                gssrpc_log_debug(b"%s\x00" as *const u8 as
                                     *const libc::c_char,
                                 gssrpc_clnt_sperror((*gd).clnt,
                                                     b"authgss_refresh\x00" as
                                                         *const u8 as
                                                         *const libc::c_char
                                                         as
                                                         *mut libc::c_char));
                if call_stat != RPC_SUCCESS as libc::c_int as libc::c_uint ||
                       gr.gr_major != 0 as libc::c_int as libc::c_uint &&
                           gr.gr_major !=
                               ((1 as libc::c_int) <<
                                    0 as libc::c_int + 0 as libc::c_int) as
                                   libc::c_uint {
                    break ;
                }
                if gr.gr_ctx.length != 0 as libc::c_int as libc::c_ulong {
                    free((*gd).gc.gc_ctx.value);
                    (*gd).gc.gc_ctx = gr.gr_ctx
                }
                if gr.gr_token.length != 0 as libc::c_int as libc::c_ulong {
                    if maj_stat !=
                           ((1 as libc::c_int) <<
                                0 as libc::c_int + 0 as libc::c_int) as
                               libc::c_uint {
                        break ;
                    }
                    recv_tokenp = &mut gr.gr_token
                }
                (*gd).gc.gc_proc = RPCSEC_GSS_CONTINUE_INIT
            }
            /* GSS_S_COMPLETE => check gss header verifier, usually checked in
		 * gss_validate
		 */
            if !(maj_stat == 0 as libc::c_int as libc::c_uint) { continue ; }
            let mut bufin: gss_buffer_desc =
                gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
            let mut bufout: gss_buffer_desc =
                gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
            let mut seq: uint32_t = 0;
            let mut qop_state: gss_qop_t = 0 as libc::c_int as gss_qop_t;
            seq = htonl(gr.gr_win);
            bufin.value =
                &mut seq as *mut uint32_t as *mut u_char as *mut libc::c_void;
            bufin.length = ::std::mem::size_of::<uint32_t>() as libc::c_ulong;
            bufout.value =
                (*gd).gc_wire_verf.value as *mut u_char as *mut libc::c_void;
            bufout.length = (*gd).gc_wire_verf.length;
            gssrpc_log_debug(b"authgss_refresh: GSS_S_COMPLETE: calling verify_mic\x00"
                                 as *const u8 as *const libc::c_char);
            maj_stat =
                gss_verify_mic(&mut min_stat, (*gd).ctx, &mut bufin,
                               &mut bufout, &mut qop_state);
            free((*gd).gc_wire_verf.value);
            (*gd).gc_wire_verf.length = 0 as libc::c_int as size_t;
            (*gd).gc_wire_verf.value = 0 as *mut libc::c_void;
            if maj_stat != 0 as libc::c_int as libc::c_uint ||
                   qop_state != (*gd).sec.qop {
                gssrpc_log_status(b"gss_verify_mic\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, maj_stat, min_stat);
                if maj_stat ==
                       (12 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
                   {
                    (*gd).established = 0 as libc::c_int;
                    authgss_destroy_context(auth);
                }
                return 0 as libc::c_int
            }
            (*gd).established = 1 as libc::c_int;
            (*gd).inprogress = 0 as libc::c_int;
            (*gd).gc.gc_proc = RPCSEC_GSS_DATA;
            (*gd).gc.gc_seq = 0 as libc::c_int as uint32_t;
            (*gd).win = gr.gr_win;
            break ;
        }
    }
    gssrpc_log_status(b"authgss_refresh: at end of context negotiation\x00" as
                          *const u8 as *const libc::c_char as
                          *mut libc::c_char, maj_stat, min_stat);
    /* End context negotiation loop. */
    if (*gd).gc.gc_proc as libc::c_uint !=
           RPCSEC_GSS_DATA as libc::c_int as libc::c_uint {
        gssrpc_log_debug(b"authgss_refresh: returning ERROR (gc_proc %d)\x00"
                             as *const u8 as *const libc::c_char,
                         (*gd).gc.gc_proc as libc::c_uint);
        free(gr.gr_token.value);
        authgss_destroy(auth);
        auth = 0 as *mut AUTH;
        gssrpc_rpc_createrr.cf_stat = RPC_AUTHERROR;
        return 0 as libc::c_int
    }
    gssrpc_log_debug(b"authgss_refresh: returning SUCCESS\x00" as *const u8 as
                         *const libc::c_char);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "527:1"]
pub unsafe extern "C" fn gssrpc_authgss_service(mut auth: *mut AUTH,
                                                mut svc: libc::c_int)
 -> libc::c_int {
    let mut gd: *mut rpc_gss_data = 0 as *mut rpc_gss_data;
    gssrpc_log_debug(b"in authgss_service()\x00" as *const u8 as
                         *const libc::c_char);
    if auth.is_null() { return 0 as libc::c_int }
    gd = (*auth).ah_private as *mut rpc_gss_data;
    if gd.is_null() || (*gd).established == 0 { return 0 as libc::c_int }
    (*gd).sec.svc = svc as rpc_gss_svc_t;
    (*gd).gc.gc_svc = svc as rpc_gss_svc_t;
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "544:1"]
unsafe extern "C" fn authgss_destroy_context(mut auth: *mut AUTH) {
    let mut gd: *mut rpc_gss_data = 0 as *mut rpc_gss_data;
    let mut min_stat: OM_uint32 = 0;
    gssrpc_log_debug(b"in authgss_destroy_context()\x00" as *const u8 as
                         *const libc::c_char);
    gd = (*auth).ah_private as *mut rpc_gss_data;
    if (*gd).gc.gc_ctx.length != 0 as libc::c_int as libc::c_ulong {
        if (*gd).established != 0 {
            (*gd).gc.gc_proc = RPCSEC_GSS_DESTROY;
            Some((*(*(*gd).clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")((*gd).clnt,
                                                                                                                          0
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              rpcproc_t,
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
                                                                                                                          0
                                                                                                                              as
                                                                                                                              *mut libc::c_void,
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
                                                                                                                          0
                                                                                                                              as
                                                                                                                              *mut libc::c_void,
                                                                                                                          AUTH_TIMEOUT);
            gssrpc_log_debug(b"%s\x00" as *const u8 as *const libc::c_char,
                             gssrpc_clnt_sperror((*gd).clnt,
                                                 b"authgss_destroy_context\x00"
                                                     as *const u8 as
                                                     *const libc::c_char as
                                                     *mut libc::c_char));
        }
        free((*gd).gc.gc_ctx.value);
        /* XXX ANDROS check size of context  - should be 8 */
        memset(&mut (*gd).gc.gc_ctx as *mut gss_buffer_desc as
                   *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<gss_buffer_desc>() as libc::c_ulong);
    }
    if !(*gd).ctx.is_null() {
        gss_delete_sec_context(&mut min_stat, &mut (*gd).ctx,
                               0 as gss_buffer_t);
        (*gd).ctx = 0 as gss_ctx_id_t
    }
    (*gd).established = 0 as libc::c_int;
    gssrpc_log_debug(b"finished authgss_destroy_context()\x00" as *const u8 as
                         *const libc::c_char);
}
#[c2rust::src_loc = "576:1"]
unsafe extern "C" fn authgss_destroy(mut auth: *mut AUTH) {
    let mut gd: *mut rpc_gss_data = 0 as *mut rpc_gss_data;
    let mut min_stat: OM_uint32 = 0;
    gssrpc_log_debug(b"in authgss_destroy()\x00" as *const u8 as
                         *const libc::c_char);
    gd = (*auth).ah_private as *mut rpc_gss_data;
    authgss_destroy_context(auth);
    if !(*gd).name.is_null() {
        gss_release_name(&mut min_stat, &mut (*gd).name);
    }
    free(gd as *mut libc::c_void);
    free(auth as *mut libc::c_void);
}
#[c2rust::src_loc = "595:1"]
unsafe extern "C" fn authgss_wrap(mut auth: *mut AUTH, mut xdrs: *mut XDR,
                                  mut xdr_func: xdrproc_t,
                                  mut xdr_ptr: caddr_t) -> libc::c_int {
    let mut gd: *mut rpc_gss_data = 0 as *mut rpc_gss_data;
    gssrpc_log_debug(b"in authgss_wrap()\x00" as *const u8 as
                         *const libc::c_char);
    gd = (*auth).ah_private as *mut rpc_gss_data;
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
                                   (*gd).sec.qop, (*gd).sec.svc,
                                   (*gd).gc.gc_seq);
}
#[c2rust::src_loc = "612:1"]
unsafe extern "C" fn authgss_unwrap(mut auth: *mut AUTH, mut xdrs: *mut XDR,
                                    mut xdr_func: xdrproc_t,
                                    mut xdr_ptr: caddr_t) -> libc::c_int {
    let mut gd: *mut rpc_gss_data = 0 as *mut rpc_gss_data;
    gssrpc_log_debug(b"in authgss_unwrap()\x00" as *const u8 as
                         *const libc::c_char);
    gd = (*auth).ah_private as *mut rpc_gss_data;
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
                                   (*gd).sec.qop, (*gd).sec.svc,
                                   (*gd).gc.gc_seq);
}
