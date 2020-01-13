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
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "162:1"]
    pub type __suseconds_t = libc::c_long;
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
#[c2rust::header_src = "/usr/include/bits/types/struct_timeval.h:8"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:8"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:8"]
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
    /* This is for rpc/netdb.h */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:8"]
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
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth.h:8"]
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
    /* not to exceed MAX_AUTH_BYTES */
    /*
 * Auth handle, interface to client side authenticators.
 */
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc_msg.h:8"]
pub mod rpc_msg_h {
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/clnt.h:8"]
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
    /*
 * Error info.
 */
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
    /*
 * Client rpc handle.
 * Created by individual implementations, see e.g. rpc_udp.c.
 * Client is responsible for initializing auth, see e.g. auth_none.c.
 */
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
    use super::auth_h::{auth_stat, AUTH};
    use super::stdint_intn_h::int32_t;
    use super::gssrpc_types_h::{rpcvers_t, rpcproc_t};
    use super::xdr_h::xdrproc_t;
    use super::struct_timeval_h::timeval;
    /* !defined(GSSRPC_CLNT_H) */
    /* a more reasonable packet size */
    /* rpc imposed limit on udp msg size */
    /* string */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:8"]
pub mod gssapi_h {
    #[c2rust::src_loc = "82:1"]
    pub type gss_cred_id_t = *mut gss_cred_id_struct;
    extern "C" {
        #[c2rust::src_loc = "81:8"]
        pub type gss_cred_id_struct;
    }
    /* _GSSAPI_H_ */
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
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
    #[c2rust::src_loc = "176:1"]
    pub type krb5_kvno = libc::c_uint;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    /* This may change, later on */
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
    /* *
 * Represents a timestamp in seconds since the POSIX epoch.  This legacy type
 * is used frequently in the ABI, but cannot represent timestamps after 2038 as
 * a positive number.  Code which uses this type should cast values of it to
 * uint32_t so that negative values are treated as timestamps between 2038 and
 * 2106 on platforms with 64-bit time_t.
 */
    #[c2rust::src_loc = "195:1"]
    pub type krb5_timestamp = krb5_int32;
    #[c2rust::src_loc = "197:1"]
    pub type krb5_deltat = krb5_int32;
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    extern "C" {
        /* per Kerberos v5 protocol spec */
        /* not yet in the spec... */
        /* macros to determine if a type is a local type */
        /*
 * end "hostaddr.h"
 */
        #[c2rust::src_loc = "350:8"]
        pub type _krb5_context;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:9"]
pub mod kdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "147:16"]
    pub struct _krb5_tl_data {
        pub tl_data_next: *mut _krb5_tl_data,
        pub tl_data_type: krb5_int16,
        pub tl_data_length: krb5_ui_2,
        pub tl_data_contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "147:1"]
    pub type krb5_tl_data = _krb5_tl_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    use super::krb5_h::{krb5_int16, krb5_ui_2, krb5_octet, krb5_enctype,
                        krb5_int32};
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:9"]
pub mod admin_h {
    #[c2rust::src_loc = "70:1"]
    pub type kadm5_policy_t = *mut libc::c_char;
    #[c2rust::src_loc = "71:1"]
    pub type kadm5_ret_t = libc::c_long;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "215:16"]
    pub struct _kadm5_policy_ent_t {
        pub policy: *mut libc::c_char,
        pub pw_min_life: libc::c_long,
        pub pw_max_life: libc::c_long,
        pub pw_min_length: libc::c_long,
        pub pw_min_classes: libc::c_long,
        pub pw_history_num: libc::c_long,
        pub policy_refcnt: libc::c_long,
        pub pw_max_fail: krb5_kvno,
        pub pw_failcnt_interval: krb5_deltat,
        pub pw_lockout_duration: krb5_deltat,
        pub attributes: krb5_flags,
        pub max_life: krb5_deltat,
        pub max_renewable_life: krb5_deltat,
        pub allowed_keysalts: *mut libc::c_char,
        pub n_tl_data: krb5_int16,
        pub tl_data: *mut krb5_tl_data,
    }
    #[c2rust::src_loc = "215:1"]
    pub type kadm5_policy_ent_rec = _kadm5_policy_ent_t;
    #[c2rust::src_loc = "215:1"]
    pub type kadm5_policy_ent_t = *mut _kadm5_policy_ent_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "241:16"]
    pub struct _kadm5_config_params {
        pub mask: libc::c_long,
        pub realm: *mut libc::c_char,
        pub kadmind_port: libc::c_int,
        pub kpasswd_port: libc::c_int,
        pub admin_server: *mut libc::c_char,
        pub dbname: *mut libc::c_char,
        pub acl_file: *mut libc::c_char,
        pub dict_file: *mut libc::c_char,
        pub mkey_from_kbd: libc::c_int,
        pub stash_file: *mut libc::c_char,
        pub mkey_name: *mut libc::c_char,
        pub enctype: krb5_enctype,
        pub max_life: krb5_deltat,
        pub max_rlife: krb5_deltat,
        pub expiration: krb5_timestamp,
        pub flags: krb5_flags,
        pub keysalts: *mut krb5_key_salt_tuple,
        pub num_keysalts: krb5_int32,
        pub kvno: krb5_kvno,
        pub iprop_enabled: libc::c_int,
        pub iprop_ulogsize: uint32_t,
        pub iprop_poll_time: krb5_deltat,
        pub iprop_logfile: *mut libc::c_char,
        pub iprop_port: libc::c_int,
        pub iprop_resync_timeout: libc::c_int,
        pub kadmind_listen: *mut libc::c_char,
        pub kpasswd_listen: *mut libc::c_char,
        pub iprop_listen: *mut libc::c_char,
    }
    #[c2rust::src_loc = "241:1"]
    pub type kadm5_config_params = _kadm5_config_params;
    use super::krb5_h::{krb5_kvno, krb5_deltat, krb5_flags, krb5_int16,
                        krb5_enctype, krb5_timestamp, krb5_int32};
    use super::kdb_h::{krb5_tl_data, krb5_key_salt_tuple};
    use super::stdint_uintn_h::uint32_t;
    /* __KADM5_ADMIN_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/kadm_rpc.h:10"]
pub mod kadm_rpc_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:8"]
    pub struct generic_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "151:8"]
    pub struct cpol_arg {
        pub api_version: krb5_ui_4,
        pub rec: kadm5_policy_ent_rec,
        pub mask: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "158:8"]
    pub struct dpol_arg {
        pub api_version: krb5_ui_4,
        pub name: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "164:8"]
    pub struct mpol_arg {
        pub api_version: krb5_ui_4,
        pub rec: kadm5_policy_ent_rec,
        pub mask: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "177:8"]
    pub struct gpol_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub rec: kadm5_policy_ent_rec,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "171:8"]
    pub struct gpol_arg {
        pub api_version: krb5_ui_4,
        pub name: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "190:8"]
    pub struct gpols_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub pols: *mut *mut libc::c_char,
        pub count: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "184:8"]
    pub struct gpols_arg {
        pub api_version: krb5_ui_4,
        pub exp: *mut libc::c_char,
    }
    use super::krb5_h::krb5_ui_4;
    use super::admin_h::{kadm5_ret_t, kadm5_policy_ent_rec};
    use super::clnt_h::{CLIENT, clnt_stat};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "286:1"]
        pub fn create_policy_2(_: *mut cpol_arg, _: *mut generic_ret,
                               _: *mut CLIENT) -> clnt_stat;
        #[no_mangle]
        #[c2rust::src_loc = "290:1"]
        pub fn delete_policy_2(_: *mut dpol_arg, _: *mut generic_ret,
                               _: *mut CLIENT) -> clnt_stat;
        #[no_mangle]
        #[c2rust::src_loc = "294:1"]
        pub fn modify_policy_2(_: *mut mpol_arg, _: *mut generic_ret,
                               _: *mut CLIENT) -> clnt_stat;
        #[no_mangle]
        #[c2rust::src_loc = "298:1"]
        pub fn get_policy_2(_: *mut gpol_arg, _: *mut gpol_ret,
                            _: *mut CLIENT) -> clnt_stat;
        #[no_mangle]
        #[c2rust::src_loc = "311:1"]
        pub fn get_pols_2(_: *mut gpols_arg, _: *mut gpols_ret,
                          _: *mut CLIENT) -> clnt_stat;
    }
    /* __KADM_RPC_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/kadm5/clnt/client_internal.h:11"]
pub mod client_internal_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved
 *
 * $Header$
 *
 * $Log$
 * Revision 1.1  1996/07/24 22:22:43  tlyu
 *      * Makefile.in, configure.in: break out client lib into a
 *              subdirectory
 *
 * Revision 1.11  1996/07/22 20:35:46  marc
 * this commit includes all the changes on the OV_9510_INTEGRATION and
 * OV_MERGE branches.  This includes, but is not limited to, the new openvision
 * admin system, and major changes to gssapi to add functionality, and bring
 * the implementation in line with rfc1964.  before committing, the
 * code was built and tested for netbsd and solaris.
 *
 * Revision 1.10.4.1  1996/07/18 03:08:37  marc
 * merged in changes from OV_9510_BP to OV_9510_FINAL1
 *
 * Revision 1.10.2.1  1996/06/20  02:16:46  marc
 * File added to the repository on a branch
 *
 * Revision 1.10  1996/06/06  20:09:16  bjaspan
 * add destroy_cache, for kadm5_init_with_creds
 *
 * Revision 1.9  1996/05/30 21:04:42  bjaspan
 * add lhandle to handle
 *
 * Revision 1.8  1996/05/28 20:33:49  bjaspan
 * rework kadm5_config
 *
 * Revision 1.7  1996/05/17 21:36:59  bjaspan
 * rename to kadm5, begin implementing version 2
 *
 * Revision 1.6  1996/05/16 21:45:07  bjaspan
 * add context
 *
 * Revision 1.5  1996/05/08 21:10:23  bjaspan
 * marc's changes
 *
 * Revision 1.4  1996/01/16  20:54:30  grier
 * secure/3570 use krb5_ui_4 not unsigned int
 *
 * Revision 1.3  1995/11/14  17:48:57  grier
 * long to int
 *
 * Revision 1.2  1994/08/16  18:53:47  jik
 * Versioning stuff.
 *
 * Revision 1.1  1994/08/09  21:14:38  jik
 * Initial revision
 *
 */
    /*
 * This header file is used internally by the Admin API client
 * libraries.  IF YOU THINK YOU NEED TO USE THIS FILE FOR ANYTHING,
 * YOU'RE ALMOST CERTAINLY WRONG.
 */
    #[c2rust::src_loc = "68:1"]
    pub type kadm5_server_handle_t = *mut _kadm5_server_handle_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "68:16"]
    pub struct _kadm5_server_handle_t {
        pub magic_number: krb5_ui_4,
        pub struct_version: krb5_ui_4,
        pub api_version: krb5_ui_4,
        pub cache_name: *mut libc::c_char,
        pub destroy_cache: libc::c_int,
        pub clnt: *mut CLIENT,
        pub client_socket: libc::c_int,
        pub context: krb5_context,
        pub cred: gss_cred_id_t,
        pub params: kadm5_config_params,
        pub lhandle: *mut _kadm5_server_handle_t,
    }
    use super::krb5_h::{krb5_ui_4, krb5_context};
    use super::clnt_h::CLIENT;
    use super::gssapi_h::gss_cred_id_t;
    use super::admin_h::kadm5_config_params;
    /* __KADM5_CLIENT_INTERNAL_H__ */
}
#[c2rust::header_src = "/usr/include/string.h:13"]
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
    }
}
pub use self::types_h::{__u_int, __uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t, __time_t, __suseconds_t, __caddr_t};
pub use self::sys_types_h::{u_int, caddr_t};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::struct_timeval_h::timeval;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t, rpc_inline_t};
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops};
pub use self::auth_h::{auth_stat, RPCSEC_GSS_CTXPROBLEM,
                       RPCSEC_GSS_CREDPROBLEM, AUTH_FAILED, AUTH_INVALIDRESP,
                       AUTH_TOOWEAK, AUTH_REJECTEDVERF, AUTH_BADVERF,
                       AUTH_REJECTEDCRED, AUTH_BADCRED, AUTH_OK, des_block,
                       opaque_auth, AUTH, auth_ops};
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
                       C2RustUnnamed_8, CLIENT, clnt_ops};
pub use self::gssapi_h::{gss_cred_id_t, gss_cred_id_struct};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_kvno, krb5_enctype, krb5_flags,
                       krb5_timestamp, krb5_deltat, krb5_context,
                       _krb5_context};
pub use self::kdb_h::{_krb5_tl_data, krb5_tl_data, __krb5_key_salt_tuple,
                      krb5_key_salt_tuple};
pub use self::admin_h::{kadm5_policy_t, kadm5_ret_t, _kadm5_policy_ent_t,
                        kadm5_policy_ent_rec, kadm5_policy_ent_t,
                        _kadm5_config_params, kadm5_config_params};
pub use self::kadm_rpc_h::{generic_ret, cpol_arg, dpol_arg, mpol_arg,
                           gpol_ret, gpol_arg, gpols_ret, gpols_arg,
                           create_policy_2, delete_policy_2, modify_policy_2,
                           get_policy_2, get_pols_2};
pub use self::client_internal_h::{kadm5_server_handle_t,
                                  _kadm5_server_handle_t};
use self::string_h::{memcpy, memset};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved
 *
 * $Header$
 */
#[no_mangle]
#[c2rust::src_loc = "16:1"]
pub unsafe extern "C" fn kadm5_create_policy(mut server_handle:
                                                 *mut libc::c_void,
                                             mut policy: kadm5_policy_ent_t,
                                             mut mask: libc::c_long)
 -> kadm5_ret_t {
    let mut arg: cpol_arg =
        cpol_arg{api_version: 0,
                 rec:
                     kadm5_policy_ent_rec{policy: 0 as *mut libc::c_char,
                                          pw_min_life: 0,
                                          pw_max_life: 0,
                                          pw_min_length: 0,
                                          pw_min_classes: 0,
                                          pw_history_num: 0,
                                          policy_refcnt: 0,
                                          pw_max_fail: 0,
                                          pw_failcnt_interval: 0,
                                          pw_lockout_duration: 0,
                                          attributes: 0,
                                          max_life: 0,
                                          max_renewable_life: 0,
                                          allowed_keysalts:
                                              0 as *mut libc::c_char,
                                          n_tl_data: 0,
                                          tl_data: 0 as *mut krb5_tl_data,},
                 mask: 0,};
    let mut r: generic_ret =
        {
            let mut init =
                generic_ret{api_version: 0 as libc::c_int as krb5_ui_4,
                            code: 0 as libc::c_int as kadm5_ret_t,};
            init
        };
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    let mut srvr: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787556 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787558 as libc::c_long
    }
    let mut srvr_0: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if (*srvr_0).clnt.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).cache_name.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).lhandle.is_null() { return 43787551 as libc::c_long }
    if policy.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    arg.mask = mask;
    arg.api_version = (*handle).api_version;
    memcpy(&mut arg.rec as *mut kadm5_policy_ent_rec as *mut libc::c_void,
           policy as *const libc::c_void,
           ::std::mem::size_of::<kadm5_policy_ent_rec>() as libc::c_ulong);
    if create_policy_2(&mut arg, &mut r, (*handle).clnt) as u64 != 0 {
        return 43787528 as libc::c_long
    }
    return r.code;
}
#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn kadm5_delete_policy(mut server_handle:
                                                 *mut libc::c_void,
                                             mut name: *mut libc::c_char)
 -> kadm5_ret_t {
    let mut arg: dpol_arg =
        dpol_arg{api_version: 0, name: 0 as *mut libc::c_char,};
    let mut r: generic_ret =
        {
            let mut init =
                generic_ret{api_version: 0 as libc::c_int as krb5_ui_4,
                            code: 0 as libc::c_int as kadm5_ret_t,};
            init
        };
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    let mut srvr: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787556 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787558 as libc::c_long
    }
    let mut srvr_0: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if (*srvr_0).clnt.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).cache_name.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).lhandle.is_null() { return 43787551 as libc::c_long }
    if name.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    arg.name = name;
    arg.api_version = (*handle).api_version;
    if delete_policy_2(&mut arg, &mut r, (*handle).clnt) as u64 != 0 {
        return 43787528 as libc::c_long
    }
    return r.code;
}
#[no_mangle]
#[c2rust::src_loc = "57:1"]
pub unsafe extern "C" fn kadm5_modify_policy(mut server_handle:
                                                 *mut libc::c_void,
                                             mut policy: kadm5_policy_ent_t,
                                             mut mask: libc::c_long)
 -> kadm5_ret_t {
    let mut arg: mpol_arg =
        mpol_arg{api_version: 0,
                 rec:
                     kadm5_policy_ent_rec{policy: 0 as *mut libc::c_char,
                                          pw_min_life: 0,
                                          pw_max_life: 0,
                                          pw_min_length: 0,
                                          pw_min_classes: 0,
                                          pw_history_num: 0,
                                          policy_refcnt: 0,
                                          pw_max_fail: 0,
                                          pw_failcnt_interval: 0,
                                          pw_lockout_duration: 0,
                                          attributes: 0,
                                          max_life: 0,
                                          max_renewable_life: 0,
                                          allowed_keysalts:
                                              0 as *mut libc::c_char,
                                          n_tl_data: 0,
                                          tl_data: 0 as *mut krb5_tl_data,},
                 mask: 0,};
    let mut r: generic_ret =
        {
            let mut init =
                generic_ret{api_version: 0 as libc::c_int as krb5_ui_4,
                            code: 0 as libc::c_int as kadm5_ret_t,};
            init
        };
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    let mut srvr: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787556 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787558 as libc::c_long
    }
    let mut srvr_0: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if (*srvr_0).clnt.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).cache_name.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).lhandle.is_null() { return 43787551 as libc::c_long }
    if policy.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    arg.mask = mask;
    arg.api_version = (*handle).api_version;
    memcpy(&mut arg.rec as *mut kadm5_policy_ent_rec as *mut libc::c_void,
           policy as *const libc::c_void,
           ::std::mem::size_of::<kadm5_policy_ent_rec>() as libc::c_ulong);
    if modify_policy_2(&mut arg, &mut r, (*handle).clnt) as u64 != 0 {
        return 43787528 as libc::c_long
    }
    return r.code;
}
#[no_mangle]
#[c2rust::src_loc = "79:1"]
pub unsafe extern "C" fn kadm5_get_policy(mut server_handle:
                                              *mut libc::c_void,
                                          mut name: *mut libc::c_char,
                                          mut ent: kadm5_policy_ent_t)
 -> kadm5_ret_t {
    let mut arg: gpol_arg =
        gpol_arg{api_version: 0, name: 0 as *mut libc::c_char,};
    let mut r: gpol_ret =
        gpol_ret{api_version: 0,
                 code: 0,
                 rec:
                     kadm5_policy_ent_rec{policy: 0 as *mut libc::c_char,
                                          pw_min_life: 0,
                                          pw_max_life: 0,
                                          pw_min_length: 0,
                                          pw_min_classes: 0,
                                          pw_history_num: 0,
                                          policy_refcnt: 0,
                                          pw_max_fail: 0,
                                          pw_failcnt_interval: 0,
                                          pw_lockout_duration: 0,
                                          attributes: 0,
                                          max_life: 0,
                                          max_renewable_life: 0,
                                          allowed_keysalts:
                                              0 as *mut libc::c_char,
                                          n_tl_data: 0,
                                          tl_data: 0 as *mut krb5_tl_data,},};
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    memset(ent as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<_kadm5_policy_ent_t>() as libc::c_ulong);
    let mut srvr: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787556 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787558 as libc::c_long
    }
    let mut srvr_0: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if (*srvr_0).clnt.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).cache_name.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).lhandle.is_null() { return 43787551 as libc::c_long }
    arg.name = name;
    arg.api_version = (*handle).api_version;
    if name.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    memset(&mut r as *mut gpol_ret as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<gpol_ret>() as libc::c_ulong);
    if get_policy_2(&mut arg, &mut r, (*handle).clnt) as u64 != 0 {
        return 43787528 as libc::c_long
    }
    if r.code == 0 as libc::c_int as libc::c_long {
        memcpy(ent as *mut libc::c_void,
               &mut r.rec as *mut kadm5_policy_ent_rec as *const libc::c_void,
               ::std::mem::size_of::<kadm5_policy_ent_rec>() as
                   libc::c_ulong);
    }
    return r.code;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/kadm5/admin.h */
/*
 * Copyright 2001, 2008 by the Massachusetts Institute of Technology.
 * All Rights Reserved.
 *
 * Export of this software from the United States of America may
 *   require a specific license from the United States Government.
 *   It is the responsibility of any person or organization contemplating
 *   export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of M.I.T. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 * M.I.T. makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 */
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved
 *
 * $Header$
 */
/*
 * This API is not considered as stable as the main krb5 API.
 *
 * - We may make arbitrary incompatible changes between feature
 *   releases (e.g. from 1.7 to 1.8).
 * - We will make some effort to avoid making incompatible changes for
 *   bugfix releases, but will make them if necessary.
 */
/*
 * Successful return code
 */
/*
 * Field masks
 */
/* kadm5_principal_ent_t */
/* version 2 masks */
/* Novell */
/* all but KEY_DATA, TL_DATA, LOAD */
/* kadm5_policy_ent_t */
/* kadm5_config_params */
/*#define KADM5_CONFIG_ADMIN_KEYTAB       0x00000080*/
/*
 * permission bits
 */
/*
 * API versioning constants
 */
/* version 2 fields */
/* no longer used */
/* version 3 fields */
/* version 4 fields */
/*
 * Data structure returned by kadm5_get_config_params()
 */
/* Novell */ /* ABI change? */
/* Deprecated except for db2 backwards compatibility.  Don't add
       new uses except as fallbacks for parameters that should be
       specified in the database module section of the config
       file.  */
/*    char *            iprop_server;*/
/*
 * functions
 */
/*
 * For all initialization functions, the caller must first initialize
 * a context with kadm5_init_krb5_context which will survive as long
 * as the resulting handle.  The caller should free the context with
 * krb5_free_context.
 */
#[no_mangle]
#[c2rust::src_loc = "104:1"]
pub unsafe extern "C" fn kadm5_get_policies(mut server_handle:
                                                *mut libc::c_void,
                                            mut exp: *mut libc::c_char,
                                            mut pols:
                                                *mut *mut *mut libc::c_char,
                                            mut count: *mut libc::c_int)
 -> kadm5_ret_t {
    let mut arg: gpols_arg =
        gpols_arg{api_version: 0, exp: 0 as *mut libc::c_char,};
    let mut r: gpols_ret =
        gpols_ret{api_version: 0,
                  code: 0,
                  pols: 0 as *mut *mut libc::c_char,
                  count: 0,};
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    let mut srvr: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787556 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787558 as libc::c_long
    }
    let mut srvr_0: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if (*srvr_0).clnt.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).cache_name.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).lhandle.is_null() { return 43787551 as libc::c_long }
    if pols.is_null() || count.is_null() {
        return 22 as libc::c_int as kadm5_ret_t
    }
    arg.exp = exp;
    arg.api_version = (*handle).api_version;
    memset(&mut r as *mut gpols_ret as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<gpols_ret>() as libc::c_ulong);
    if get_pols_2(&mut arg, &mut r, (*handle).clnt) as u64 != 0 {
        return 43787528 as libc::c_long
    }
    if r.code == 0 as libc::c_int as libc::c_long {
        *count = r.count;
        *pols = r.pols
    } else { *count = 0 as libc::c_int; *pols = 0 as *mut *mut libc::c_char }
    return r.code;
}
