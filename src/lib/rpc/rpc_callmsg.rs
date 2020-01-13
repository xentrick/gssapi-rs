use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:42"]
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
#[c2rust::header_src = "/usr/include/sys/types.h:42"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:42"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:44"]
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
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "253:1"]
        pub fn gssrpc_xdr_u_int(_: *mut XDR, _: *mut u_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "259:1"]
        pub fn gssrpc_xdr_enum(_: *mut XDR, _: *mut libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "263:1"]
        pub fn gssrpc_xdr_opaque(_: *mut XDR, _: caddr_t, _: u_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "298:1"]
        pub fn gssrpc_xdr_u_int32(_: *mut XDR, _: *mut uint32_t)
         -> libc::c_int;
    }
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth.h:44"]
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
    use super::sys_types_h::{caddr_t, u_int};
    use super::xdr_h::XDR;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "194:1"]
        pub fn gssrpc_xdr_opaque_auth(_: *mut XDR, _: *mut opaque_auth)
         -> libc::c_int;
    }
    /* !defined(GSSRPC_AUTH_H) */
    /* RPCSEC_GSS */
    /* GSS-API style */
    /* des style (encrypted timestamps) */
    /* short hand unix style */
    /* unix style (uid, gids) */
    /* backward compatibility */
    /* no authentication */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc_msg.h:44"]
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
#[c2rust::header_src = "/usr/include/string.h:43"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "46:14"]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/netinet/in.h:44"]
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
#[c2rust::header_src = "/usr/include/stdlib.h:44"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    }
}
pub use self::types_h::{__u_int, __int32_t, __uint32_t, __caddr_t};
pub use self::sys_types_h::{u_int, caddr_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t, rpc_inline_t};
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, gssrpc_xdr_u_int, gssrpc_xdr_enum,
                      gssrpc_xdr_opaque, gssrpc_xdr_u_int32};
pub use self::auth_h::{auth_stat, RPCSEC_GSS_CTXPROBLEM,
                       RPCSEC_GSS_CREDPROBLEM, AUTH_FAILED, AUTH_INVALIDRESP,
                       AUTH_TOOWEAK, AUTH_REJECTEDVERF, AUTH_BADVERF,
                       AUTH_REJECTEDCRED, AUTH_BADCRED, AUTH_OK, opaque_auth,
                       gssrpc_xdr_opaque_auth};
pub use self::rpc_msg_h::{rpc_msg, C2RustUnnamed, reply_body, C2RustUnnamed_0,
                          rejected_reply, C2RustUnnamed_1, C2RustUnnamed_2,
                          reject_stat, AUTH_ERROR, RPC_MISMATCH,
                          accepted_reply, C2RustUnnamed_3, C2RustUnnamed_4,
                          C2RustUnnamed_5, accept_stat, SYSTEM_ERR,
                          GARBAGE_ARGS, PROC_UNAVAIL, PROG_MISMATCH,
                          PROG_UNAVAIL, SUCCESS, reply_stat, MSG_DENIED,
                          MSG_ACCEPTED, call_body, msg_type, REPLY, CALL};
use self::string_h::memmove;
use self::in_h::{ntohl, htonl};
use self::stdlib_h::malloc;
/* @(#)rpc_msg.h	2.1 88/07/29 4.0 RPCSRC */
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
/*      @(#)rpc_msg.h 1.7 86/07/16 SMI      */
/*
 * rpc_msg.h
 * rpc message definition
 */
/*
 * Bottom up definition of an rpc message.
 * NOTE: call and reply use the same overall stuct but
 * different parts of unions within it.
 */
/*
 * Reply part of an rpc exchange
 */
/*
 * Reply to an rpc request that was accepted by the server.
 * Note: there could be an error even though the request was
 * accepted.
 */
/* and many other null cases */
/*
 * Reply to an rpc request that was rejected by the server.
 */
/* why authentication did not work */
/*
 * Body of a reply to an rpc request.
 */
/*
 * Body of an rpc request call.
 */
/* must be equal to two */
/* protocol specific - provided by client */
/*
 * The rpc message
 */
/*
 * XDR routine to handle a rpc message.
 * xdr_callmsg(xdrs, cmsg)
 * 	XDR *xdrs;
 * 	struct rpc_msg *cmsg;
 */
/* @(#)rpc_callmsg.c	2.1 88/07/29 4.0 RPCSRC */
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
 * rpc_callmsg.c
 */
/*
 * XDR a call message
 */
#[no_mangle]
#[c2rust::src_loc = "49:1"]
pub unsafe extern "C" fn gssrpc_xdr_callmsg(mut xdrs: *mut XDR,
                                            mut cmsg: *mut rpc_msg)
 -> libc::c_int {
    let mut buf: *mut rpc_inline_t = 0 as *mut rpc_inline_t;
    let mut oa: *mut opaque_auth = 0 as *mut opaque_auth;
    if (*xdrs).x_op as libc::c_uint ==
           XDR_ENCODE as libc::c_int as libc::c_uint {
        if (*cmsg).ru.RM_cmb.cb_cred.oa_length >
               400 as libc::c_int as libc::c_uint {
            return 0 as libc::c_int
        }
        if (*cmsg).ru.RM_cmb.cb_verf.oa_length >
               400 as libc::c_int as libc::c_uint {
            return 0 as libc::c_int
        }
        buf =
            Some((*(*xdrs).x_ops).x_inline.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                    ((8
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                          *
                                                                                                                          4
                                                                                                                              as
                                                                                                                              libc::c_int)
                                                                                                                         as
                                                                                                                         libc::c_uint).wrapping_add((*cmsg).ru.RM_cmb.cb_cred.oa_length.wrapping_add(4
                                                                                                                                                                                                         as
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
                                                                                                                                                                                                                                                                                                      libc::c_uint)).wrapping_add((2
                                                                                                                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                                                                                                                       libc::c_int
                                                                                                                                                                                                                                                                                                                                       *
                                                                                                                                                                                                                                                                                                                                       4
                                                                                                                                                                                                                                                                                                                                           as
                                                                                                                                                                                                                                                                                                                                           libc::c_int)
                                                                                                                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                                                                                                                      libc::c_uint).wrapping_add((*cmsg).ru.RM_cmb.cb_verf.oa_length.wrapping_add(4
                                                                                                                                                                                                                                                                                                                                                                                                                      as
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
                                                                                                                        as
                                                                                                                        libc::c_int);
        if !buf.is_null() {
            let fresh0 = buf;
            buf = buf.offset(1);
            *fresh0 = htonl((*cmsg).rm_xid) as int32_t;
            let fresh1 = buf;
            buf = buf.offset(1);
            *fresh1 =
                htonl((*cmsg).rm_direction as int32_t as uint32_t) as int32_t;
            if (*cmsg).rm_direction as libc::c_uint !=
                   CALL as libc::c_int as libc::c_uint {
                return 0 as libc::c_int
            }
            let fresh2 = buf;
            buf = buf.offset(1);
            *fresh2 = htonl((*cmsg).ru.RM_cmb.cb_rpcvers) as int32_t;
            if (*cmsg).ru.RM_cmb.cb_rpcvers != 2 as libc::c_int as uint32_t {
                return 0 as libc::c_int
            }
            let fresh3 = buf;
            buf = buf.offset(1);
            *fresh3 = htonl((*cmsg).ru.RM_cmb.cb_prog) as int32_t;
            let fresh4 = buf;
            buf = buf.offset(1);
            *fresh4 = htonl((*cmsg).ru.RM_cmb.cb_vers) as int32_t;
            let fresh5 = buf;
            buf = buf.offset(1);
            *fresh5 = htonl((*cmsg).ru.RM_cmb.cb_proc) as int32_t;
            oa = &mut (*cmsg).ru.RM_cmb.cb_cred;
            let fresh6 = buf;
            buf = buf.offset(1);
            *fresh6 = htonl((*oa).oa_flavor as uint32_t) as int32_t;
            let fresh7 = buf;
            buf = buf.offset(1);
            *fresh7 = htonl((*oa).oa_length) as int32_t;
            if (*oa).oa_length != 0 {
                memmove(buf as caddr_t as *mut libc::c_void,
                        (*oa).oa_base as *const libc::c_void,
                        (*oa).oa_length as libc::c_ulong);
                buf =
                    buf.offset((*oa).oa_length.wrapping_add(4 as libc::c_int
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
                                                                                                                                                             libc::c_uint).wrapping_div(4
                                                                                                                                                                                            as
                                                                                                                                                                                            libc::c_int
                                                                                                                                                                                            as
                                                                                                                                                                                            libc::c_uint)
                                   as isize)
            }
            oa = &mut (*cmsg).ru.RM_cmb.cb_verf;
            let fresh8 = buf;
            buf = buf.offset(1);
            *fresh8 = htonl((*oa).oa_flavor as uint32_t) as int32_t;
            let fresh9 = buf;
            buf = buf.offset(1);
            *fresh9 = htonl((*oa).oa_length) as int32_t;
            if (*oa).oa_length != 0 {
                memmove(buf as caddr_t as *mut libc::c_void,
                        (*oa).oa_base as *const libc::c_void,
                        (*oa).oa_length as libc::c_ulong);
                /* no real need....
				buf += RNDUP(oa->oa_length) / BYTES_PER_XDR_UNIT;
				*/
            }
            return 1 as libc::c_int
        }
    }
    if (*xdrs).x_op as libc::c_uint ==
           XDR_DECODE as libc::c_int as libc::c_uint {
        buf =
            Some((*(*xdrs).x_ops).x_inline.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                    8
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        *
                                                                                                                        4
                                                                                                                            as
                                                                                                                            libc::c_int);
        if !buf.is_null() {
            let fresh10 = buf;
            buf = buf.offset(1);
            (*cmsg).rm_xid =
                ntohl(*fresh10 as uint32_t) as int32_t as libc::c_long as
                    uint32_t;
            let fresh11 = buf;
            buf = buf.offset(1);
            (*cmsg).rm_direction =
                ntohl(*fresh11 as uint32_t) as int32_t as msg_type;
            if (*cmsg).rm_direction as libc::c_uint !=
                   CALL as libc::c_int as libc::c_uint {
                return 0 as libc::c_int
            }
            let fresh12 = buf;
            buf = buf.offset(1);
            (*cmsg).ru.RM_cmb.cb_rpcvers =
                ntohl(*fresh12 as uint32_t) as int32_t as libc::c_long as
                    rpcvers_t;
            if (*cmsg).ru.RM_cmb.cb_rpcvers != 2 as libc::c_int as uint32_t {
                return 0 as libc::c_int
            }
            let fresh13 = buf;
            buf = buf.offset(1);
            (*cmsg).ru.RM_cmb.cb_prog =
                ntohl(*fresh13 as uint32_t) as int32_t as libc::c_long as
                    rpcprog_t;
            let fresh14 = buf;
            buf = buf.offset(1);
            (*cmsg).ru.RM_cmb.cb_vers =
                ntohl(*fresh14 as uint32_t) as int32_t as libc::c_long as
                    rpcvers_t;
            let fresh15 = buf;
            buf = buf.offset(1);
            (*cmsg).ru.RM_cmb.cb_proc =
                ntohl(*fresh15 as uint32_t) as int32_t as libc::c_long as
                    rpcproc_t;
            oa = &mut (*cmsg).ru.RM_cmb.cb_cred;
            let fresh16 = buf;
            buf = buf.offset(1);
            (*oa).oa_flavor = ntohl(*fresh16 as uint32_t) as int32_t;
            let fresh17 = buf;
            buf = buf.offset(1);
            (*oa).oa_length =
                ntohl(*fresh17 as uint32_t) as int32_t as libc::c_long as
                    u_int;
            if (*oa).oa_length != 0 {
                if (*oa).oa_length > 400 as libc::c_int as libc::c_uint {
                    return 0 as libc::c_int
                }
                if (*oa).oa_base.is_null() {
                    (*oa).oa_base =
                        malloc((*oa).oa_length as libc::c_ulong) as caddr_t
                }
                buf =
                    Some((*(*xdrs).x_ops).x_inline.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                            (*oa).oa_length.wrapping_add(4
                                                                                                                                                             as
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
                                                                                                                                                                                                                                                          libc::c_uint)
                                                                                                                                as
                                                                                                                                libc::c_int);
                if buf.is_null() {
                    if gssrpc_xdr_opaque(xdrs, (*oa).oa_base, (*oa).oa_length)
                           == 0 as libc::c_int {
                        return 0 as libc::c_int
                    }
                } else {
                    memmove((*oa).oa_base as *mut libc::c_void,
                            buf as caddr_t as *const libc::c_void,
                            (*oa).oa_length as libc::c_ulong);
                    /* no real need....
					buf += RNDUP(oa->oa_length) /
						BYTES_PER_XDR_UNIT;
					*/
                }
            }
            oa = &mut (*cmsg).ru.RM_cmb.cb_verf;
            buf =
                Some((*(*xdrs).x_ops).x_inline.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                        2
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            *
                                                                                                                            4
                                                                                                                                as
                                                                                                                                libc::c_int);
            if buf.is_null() {
                if gssrpc_xdr_enum(xdrs, &mut (*oa).oa_flavor) ==
                       0 as libc::c_int ||
                       gssrpc_xdr_u_int(xdrs, &mut (*oa).oa_length) ==
                           0 as libc::c_int {
                    return 0 as libc::c_int
                }
            } else {
                let fresh18 = buf;
                buf = buf.offset(1);
                (*oa).oa_flavor = ntohl(*fresh18 as uint32_t) as int32_t;
                let fresh19 = buf;
                buf = buf.offset(1);
                (*oa).oa_length =
                    ntohl(*fresh19 as uint32_t) as int32_t as libc::c_long as
                        u_int
            }
            if (*oa).oa_length != 0 {
                if (*oa).oa_length > 400 as libc::c_int as libc::c_uint {
                    return 0 as libc::c_int
                }
                if (*oa).oa_base.is_null() {
                    (*oa).oa_base =
                        malloc((*oa).oa_length as libc::c_ulong) as caddr_t
                }
                buf =
                    Some((*(*xdrs).x_ops).x_inline.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                            (*oa).oa_length.wrapping_add(4
                                                                                                                                                             as
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
                                                                                                                                                                                                                                                          libc::c_uint)
                                                                                                                                as
                                                                                                                                libc::c_int);
                if buf.is_null() {
                    if gssrpc_xdr_opaque(xdrs, (*oa).oa_base, (*oa).oa_length)
                           == 0 as libc::c_int {
                        return 0 as libc::c_int
                    }
                } else {
                    memmove((*oa).oa_base as *mut libc::c_void,
                            buf as caddr_t as *const libc::c_void,
                            (*oa).oa_length as libc::c_ulong);
                    /* no real need...
					buf += RNDUP(oa->oa_length) /
						BYTES_PER_XDR_UNIT;
					*/
                }
            }
            return 1 as libc::c_int
        }
    }
    if gssrpc_xdr_u_int32(xdrs, &mut (*cmsg).rm_xid) != 0 &&
           gssrpc_xdr_enum(xdrs,
                           &mut (*cmsg).rm_direction as *mut msg_type as
                               *mut libc::c_int) != 0 &&
           (*cmsg).rm_direction as libc::c_uint ==
               CALL as libc::c_int as libc::c_uint &&
           gssrpc_xdr_u_int32(xdrs, &mut (*cmsg).ru.RM_cmb.cb_rpcvers) != 0 &&
           (*cmsg).ru.RM_cmb.cb_rpcvers == 2 as libc::c_int as uint32_t &&
           gssrpc_xdr_u_int32(xdrs, &mut (*cmsg).ru.RM_cmb.cb_prog) != 0 &&
           gssrpc_xdr_u_int32(xdrs, &mut (*cmsg).ru.RM_cmb.cb_vers) != 0 &&
           gssrpc_xdr_u_int32(xdrs, &mut (*cmsg).ru.RM_cmb.cb_proc) != 0 &&
           gssrpc_xdr_opaque_auth(xdrs, &mut (*cmsg).ru.RM_cmb.cb_cred) != 0 {
        return gssrpc_xdr_opaque_auth(xdrs, &mut (*cmsg).ru.RM_cmb.cb_verf)
    }
    return 0 as libc::c_int;
}
