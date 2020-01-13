use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:2"]
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
#[c2rust::header_src = "/usr/include/sys/types.h:2"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:2"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timeval.h:2"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:2"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:2"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:2"]
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
        #[c2rust::src_loc = "298:1"]
        pub fn gssrpc_xdr_u_int32(_: *mut XDR, _: *mut uint32_t)
         -> libc::c_int;
    }
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth.h:2"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc_msg.h:2"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/clnt.h:2"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:3"]
pub mod krb5_h {
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
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "176:1"]
    pub type krb5_kvno = libc::c_uint;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
    #[c2rust::src_loc = "195:1"]
    pub type krb5_timestamp = krb5_int32;
    #[c2rust::src_loc = "197:1"]
    pub type krb5_deltat = krb5_int32;
    #[c2rust::src_loc = "204:1"]
    pub type krb5_error_code = krb5_int32;
    #[c2rust::src_loc = "206:1"]
    pub type krb5_magic = krb5_error_code;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "208:16"]
    pub struct _krb5_data {
        pub magic: krb5_magic,
        pub length: libc::c_uint,
        pub data: *mut libc::c_char,
    }
    #[c2rust::src_loc = "208:1"]
    pub type krb5_data = _krb5_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "228:16"]
    pub struct krb5_principal_data {
        pub magic: krb5_magic,
        pub realm: krb5_data,
        pub data: *mut krb5_data,
        pub length: krb5_int32,
        pub type_0: krb5_int32,
    }
    #[c2rust::src_loc = "236:1"]
    pub type krb5_principal = *mut krb5_principal_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "363:16"]
    pub struct _krb5_keyblock {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "363:1"]
    pub type krb5_keyblock = _krb5_keyblock;
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:3"]
pub mod kdb_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1990, 1991, 2016 by the Massachusetts Institute of Technology.
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
 * Copyright (C) 1998 by the FundsXpress, INC.
 *
 * All rights reserved.
 *
 * Export of this software from the United States of America may require
 * a specific license from the United States Government.  It is the
 * responsibility of any person or organization contemplating export to
 * obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of FundsXpress. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  FundsXpress makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 *
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND WITHOUT ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED
 * WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
 */
/*
 * Copyright 2009 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
    /* KDC Database interface definitions */
    /* This API is not considered as stable as the main krb5 API.
 *
 * - We may make arbitrary incompatible changes between feature
 *   releases (e.g. from 1.7 to 1.8).
 * - We will make some effort to avoid making incompatible changes for
 *   bugfix releases, but will make them if necessary.
 */
    /* This version will be incremented when incompatible changes are made to the
 * KDB API, and will be kept in sync with the libkdb major version. */
    /* Salt types */
    /* #define KRB5_KDB_SALTTYPE_V4            1 */
    /* #define KRB5_KDB_SALTTYPE_AFS3          5 */
    /* Attributes */
    /* S4U2Self OK */
    /* Creation flags */
    /* Entry get flags */
/* Name canonicalization requested */
    /* Include authorization data generated by backend */
    /* Is AS-REQ (client referrals only) */
    /* Map cross-realm principals */
    /* Protocol transition */
    /* Constrained delegation */
    /* User-to-user */
    /* Cross-realm */
    /* Issuing referral */
    /* KDB iteration flags */
    /* String attribute names recognized by krb5 */
    /*
 * Note --- these structures cannot be modified without changing the
 * database version number in libkdb.a, but should be expandable by
 * adding new tl_data types.
 */
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
    /* String attributes (currently stored inside tl-data) map C string keys to
 * values.  They can be set via kadmin and consumed by KDC plugins. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "156:16"]
    pub struct krb5_string_attr_st {
        pub key: *mut libc::c_char,
        pub value: *mut libc::c_char,
    }
    #[c2rust::src_loc = "156:1"]
    pub type krb5_string_attr = krb5_string_attr_st;
    /*
 * If this ever changes up the version number and make the arrays be as
 * big as necessary.
 *
 * Currently the first type is the enctype and the second is the salt type.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "167:16"]
    pub struct _krb5_key_data {
        pub key_data_ver: krb5_int16,
        pub key_data_kvno: krb5_ui_2,
        pub key_data_type: [krb5_int16; 2],
        pub key_data_length: [krb5_ui_2; 2],
        pub key_data_contents: [*mut krb5_octet; 2],
    }
    #[c2rust::src_loc = "167:1"]
    pub type krb5_key_data = _krb5_key_data;
    /* Array of pointers */
    /* # of array elements */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "177:16"]
    pub struct _krb5_keysalt {
        pub type_0: krb5_int16,
        pub data: krb5_data,
    }
    #[c2rust::src_loc = "177:1"]
    pub type krb5_keysalt = _krb5_keysalt;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    use super::krb5_h::{krb5_int16, krb5_ui_2, krb5_octet, krb5_data,
                        krb5_enctype, krb5_int32};
    /* Length, data */
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:3"]
pub mod admin_h {
    #[c2rust::src_loc = "71:1"]
    pub type kadm5_ret_t = libc::c_long;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "190:16"]
    pub struct _kadm5_principal_ent_t {
        pub principal: krb5_principal,
        pub princ_expire_time: krb5_timestamp,
        pub last_pwd_change: krb5_timestamp,
        pub pw_expiration: krb5_timestamp,
        pub max_life: krb5_deltat,
        pub mod_name: krb5_principal,
        pub mod_date: krb5_timestamp,
        pub attributes: krb5_flags,
        pub kvno: krb5_kvno,
        pub mkvno: krb5_kvno,
        pub policy: *mut libc::c_char,
        pub aux_attributes: libc::c_long,
        pub max_renewable_life: krb5_deltat,
        pub last_success: krb5_timestamp,
        pub last_failed: krb5_timestamp,
        pub fail_auth_count: krb5_kvno,
        pub n_key_data: krb5_int16,
        pub n_tl_data: krb5_int16,
        pub tl_data: *mut krb5_tl_data,
        pub key_data: *mut krb5_key_data,
    }
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
    #[c2rust::src_loc = "190:1"]
    pub type kadm5_principal_ent_rec = _kadm5_principal_ent_t;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "284:16"]
    pub struct _kadm5_key_data {
        pub kvno: krb5_kvno,
        pub key: krb5_keyblock,
        pub salt: krb5_keysalt,
    }
    #[c2rust::src_loc = "284:1"]
    pub type kadm5_key_data = _kadm5_key_data;
    use super::krb5_h::{krb5_principal, krb5_timestamp, krb5_deltat,
                        krb5_flags, krb5_kvno, krb5_int16, krb5_keyblock};
    use super::kdb_h::{krb5_tl_data, krb5_key_data, krb5_keysalt};
    /* version 2 fields */
    /* no longer used */
    /* version 3 fields */
    /* version 4 fields */
    /* __KADM5_ADMIN_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/kadm_rpc.h:3"]
pub mod kadm_rpc_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:8"]
    pub struct cprinc_arg {
        pub api_version: krb5_ui_4,
        pub rec: kadm5_principal_ent_rec,
        pub mask: libc::c_long,
        pub passwd: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "18:8"]
    pub struct cprinc3_arg {
        pub api_version: krb5_ui_4,
        pub rec: kadm5_principal_ent_rec,
        pub mask: libc::c_long,
        pub n_ks_tuple: libc::c_int,
        pub ks_tuple: *mut krb5_key_salt_tuple,
        pub passwd: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:8"]
    pub struct generic_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "34:8"]
    pub struct dprinc_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:8"]
    pub struct mprinc_arg {
        pub api_version: krb5_ui_4,
        pub rec: kadm5_principal_ent_rec,
        pub mask: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "47:8"]
    pub struct rprinc_arg {
        pub api_version: krb5_ui_4,
        pub src: krb5_principal,
        pub dest: krb5_principal,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "54:8"]
    pub struct gprincs_arg {
        pub api_version: krb5_ui_4,
        pub exp: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "60:8"]
    pub struct gprincs_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub princs: *mut *mut libc::c_char,
        pub count: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "68:8"]
    pub struct chpass_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub pass: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "75:8"]
    pub struct chpass3_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub keepold: krb5_boolean,
        pub n_ks_tuple: libc::c_int,
        pub ks_tuple: *mut krb5_key_salt_tuple,
        pub pass: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "85:8"]
    pub struct setkey_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub keyblocks: *mut krb5_keyblock,
        pub n_keys: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "93:8"]
    pub struct setkey3_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub keepold: krb5_boolean,
        pub n_ks_tuple: libc::c_int,
        pub ks_tuple: *mut krb5_key_salt_tuple,
        pub keyblocks: *mut krb5_keyblock,
        pub n_keys: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "104:8"]
    pub struct setkey4_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub keepold: krb5_boolean,
        pub key_data: *mut kadm5_key_data,
        pub n_key_data: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "113:8"]
    pub struct chrand_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "119:8"]
    pub struct chrand3_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub keepold: krb5_boolean,
        pub n_ks_tuple: libc::c_int,
        pub ks_tuple: *mut krb5_key_salt_tuple,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "128:8"]
    pub struct chrand_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub key: krb5_keyblock,
        pub keys: *mut krb5_keyblock,
        pub n_keys: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "137:8"]
    pub struct gprinc_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub mask: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "144:8"]
    pub struct gprinc_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub rec: kadm5_principal_ent_rec,
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
    #[c2rust::src_loc = "171:8"]
    pub struct gpol_arg {
        pub api_version: krb5_ui_4,
        pub name: *mut libc::c_char,
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
    #[c2rust::src_loc = "184:8"]
    pub struct gpols_arg {
        pub api_version: krb5_ui_4,
        pub exp: *mut libc::c_char,
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
    #[c2rust::src_loc = "198:8"]
    pub struct getprivs_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub privs: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "205:8"]
    pub struct purgekeys_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub keepkvno: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "212:8"]
    pub struct gstrings_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "218:8"]
    pub struct gstrings_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub strings: *mut krb5_string_attr,
        pub count: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "226:8"]
    pub struct sstring_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub key: *mut libc::c_char,
        pub value: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "234:8"]
    pub struct getpkeys_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub kvno: krb5_kvno,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "241:8"]
    pub struct getpkeys_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub key_data: *mut kadm5_key_data,
        pub n_key_data: libc::c_int,
    }
    use super::krb5_h::{krb5_ui_4, krb5_principal, krb5_boolean,
                        krb5_keyblock, krb5_kvno};
    use super::admin_h::{kadm5_principal_ent_rec, kadm5_ret_t, kadm5_key_data,
                         kadm5_policy_ent_rec};
    use super::kdb_h::{krb5_key_salt_tuple, krb5_string_attr};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "366:1"]
        pub fn xdr_generic_ret() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "364:1"]
        pub fn xdr_cprinc_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "367:1"]
        pub fn xdr_dprinc_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "368:1"]
        pub fn xdr_mprinc_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "369:1"]
        pub fn xdr_rprinc_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "381:1"]
        pub fn xdr_gprinc_ret() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "380:1"]
        pub fn xdr_gprinc_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn xdr_chpass_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "379:1"]
        pub fn xdr_chrand_ret() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "377:1"]
        pub fn xdr_chrand_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "391:1"]
        pub fn xdr_cpol_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "392:1"]
        pub fn xdr_dpol_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "393:1"]
        pub fn xdr_mpol_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "395:1"]
        pub fn xdr_gpol_ret() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "394:1"]
        pub fn xdr_gpol_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "398:1"]
        pub fn xdr_getprivs_ret() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "371:1"]
        pub fn xdr_gprincs_ret() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "370:1"]
        pub fn xdr_gprincs_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "397:1"]
        pub fn xdr_gpols_ret() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "396:1"]
        pub fn xdr_gpols_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "374:1"]
        pub fn xdr_setkey_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "365:1"]
        pub fn xdr_cprinc3_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "373:1"]
        pub fn xdr_chpass3_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "378:1"]
        pub fn xdr_chrand3_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "375:1"]
        pub fn xdr_setkey3_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "399:1"]
        pub fn xdr_purgekeys_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "405:1"]
        pub fn xdr_getpkeys_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "401:1"]
        pub fn xdr_gstrings_ret() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "400:1"]
        pub fn xdr_gstrings_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "406:1"]
        pub fn xdr_getpkeys_ret() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "402:1"]
        pub fn xdr_sstring_arg() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "376:1"]
        pub fn xdr_setkey4_arg() -> libc::c_int;
    }
    /* __KADM_RPC_H__ */
}
pub use self::types_h::{__u_int, __uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t, __time_t, __suseconds_t, __caddr_t};
pub use self::sys_types_h::{u_int, caddr_t};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::struct_timeval_h::timeval;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t, rpc_inline_t};
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, gssrpc_xdr_u_int32};
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
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_kvno, krb5_enctype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal, _krb5_keyblock,
                       krb5_keyblock};
pub use self::kdb_h::{_krb5_tl_data, krb5_tl_data, krb5_string_attr_st,
                      krb5_string_attr, _krb5_key_data, krb5_key_data,
                      _krb5_keysalt, krb5_keysalt, __krb5_key_salt_tuple,
                      krb5_key_salt_tuple};
pub use self::admin_h::{kadm5_ret_t, _kadm5_principal_ent_t,
                        kadm5_principal_ent_rec, _kadm5_policy_ent_t,
                        kadm5_policy_ent_rec, _kadm5_key_data,
                        kadm5_key_data};
pub use self::kadm_rpc_h::{cprinc_arg, cprinc3_arg, generic_ret, dprinc_arg,
                           mprinc_arg, rprinc_arg, gprincs_arg, gprincs_ret,
                           chpass_arg, chpass3_arg, setkey_arg, setkey3_arg,
                           setkey4_arg, chrand_arg, chrand3_arg, chrand_ret,
                           gprinc_arg, gprinc_ret, cpol_arg, dpol_arg,
                           mpol_arg, gpol_arg, gpol_ret, gpols_arg, gpols_ret,
                           getprivs_ret, purgekeys_arg, gstrings_arg,
                           gstrings_ret, sstring_arg, getpkeys_arg,
                           getpkeys_ret, xdr_generic_ret, xdr_cprinc_arg,
                           xdr_dprinc_arg, xdr_mprinc_arg, xdr_rprinc_arg,
                           xdr_gprinc_ret, xdr_gprinc_arg, xdr_chpass_arg,
                           xdr_chrand_ret, xdr_chrand_arg, xdr_cpol_arg,
                           xdr_dpol_arg, xdr_mpol_arg, xdr_gpol_ret,
                           xdr_gpol_arg, xdr_getprivs_ret, xdr_gprincs_ret,
                           xdr_gprincs_arg, xdr_gpols_ret, xdr_gpols_arg,
                           xdr_setkey_arg, xdr_cprinc3_arg, xdr_chpass3_arg,
                           xdr_chrand3_arg, xdr_setkey3_arg,
                           xdr_purgekeys_arg, xdr_getpkeys_arg,
                           xdr_gstrings_ret, xdr_gstrings_arg,
                           xdr_getpkeys_ret, xdr_sstring_arg,
                           xdr_setkey4_arg};
/* -*- mode: c; c-file-style: "bsd"; indent-tabs-mode: t -*- */
/* for memset prototype */
/* Default timeout can be changed using clnt_control() */
#[c2rust::src_loc = "13:23"]
static mut TIMEOUT: timeval =
    {
        let mut init =
            timeval{tv_sec: 25 as libc::c_int as __time_t,
                    tv_usec: 0 as libc::c_int as __suseconds_t,};
        init
    };
#[no_mangle]
#[c2rust::src_loc = "15:1"]
pub unsafe extern "C" fn create_principal_2(mut argp: *mut cprinc_arg,
                                            mut res: *mut generic_ret,
                                            mut clnt: *mut CLIENT)
 -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   1
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_cprinc_arg))),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_generic_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
#[no_mangle]
#[c2rust::src_loc = "23:1"]
pub unsafe extern "C" fn create_principal3_2(mut argp: *mut cprinc3_arg,
                                             mut res: *mut generic_ret,
                                             mut clnt: *mut CLIENT)
 -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   18
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_cprinc3_arg))),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_generic_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
#[no_mangle]
#[c2rust::src_loc = "31:1"]
pub unsafe extern "C" fn delete_principal_2(mut argp: *mut dprinc_arg,
                                            mut res: *mut generic_ret,
                                            mut clnt: *mut CLIENT)
 -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_dprinc_arg))),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_generic_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
#[no_mangle]
#[c2rust::src_loc = "39:1"]
pub unsafe extern "C" fn modify_principal_2(mut argp: *mut mprinc_arg,
                                            mut res: *mut generic_ret,
                                            mut clnt: *mut CLIENT)
 -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   3
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_mprinc_arg))),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_generic_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
#[no_mangle]
#[c2rust::src_loc = "47:1"]
pub unsafe extern "C" fn rename_principal_2(mut argp: *mut rprinc_arg,
                                            mut res: *mut generic_ret,
                                            mut clnt: *mut CLIENT)
 -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   4
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_rprinc_arg))),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_generic_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
#[no_mangle]
#[c2rust::src_loc = "55:1"]
pub unsafe extern "C" fn get_principal_2(mut argp: *mut gprinc_arg,
                                         mut res: *mut gprinc_ret,
                                         mut clnt: *mut CLIENT) -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   5
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_gprinc_arg))),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_gprinc_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
#[no_mangle]
#[c2rust::src_loc = "63:1"]
pub unsafe extern "C" fn get_princs_2(mut argp: *mut gprincs_arg,
                                      mut res: *mut gprincs_ret,
                                      mut clnt: *mut CLIENT) -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   14
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_gprincs_arg))),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_gprincs_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
#[no_mangle]
#[c2rust::src_loc = "71:1"]
pub unsafe extern "C" fn chpass_principal_2(mut argp: *mut chpass_arg,
                                            mut res: *mut generic_ret,
                                            mut clnt: *mut CLIENT)
 -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   6
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_chpass_arg))),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_generic_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
#[no_mangle]
#[c2rust::src_loc = "79:1"]
pub unsafe extern "C" fn chpass_principal3_2(mut argp: *mut chpass3_arg,
                                             mut res: *mut generic_ret,
                                             mut clnt: *mut CLIENT)
 -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   19
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_chpass3_arg))),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_generic_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
#[no_mangle]
#[c2rust::src_loc = "87:1"]
pub unsafe extern "C" fn setkey_principal_2(mut argp: *mut setkey_arg,
                                            mut res: *mut generic_ret,
                                            mut clnt: *mut CLIENT)
 -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   16
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_setkey_arg))),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_generic_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
#[no_mangle]
#[c2rust::src_loc = "95:1"]
pub unsafe extern "C" fn setkey_principal3_2(mut argp: *mut setkey3_arg,
                                             mut res: *mut generic_ret,
                                             mut clnt: *mut CLIENT)
 -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   21
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_setkey3_arg))),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_generic_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
#[no_mangle]
#[c2rust::src_loc = "103:1"]
pub unsafe extern "C" fn setkey_principal4_2(mut argp: *mut setkey4_arg,
                                             mut res: *mut generic_ret,
                                             mut clnt: *mut CLIENT)
 -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   25
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_setkey4_arg))),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_generic_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
#[no_mangle]
#[c2rust::src_loc = "111:1"]
pub unsafe extern "C" fn chrand_principal_2(mut argp: *mut chrand_arg,
                                            mut res: *mut chrand_ret,
                                            mut clnt: *mut CLIENT)
 -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   7
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_chrand_arg))),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_chrand_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
#[no_mangle]
#[c2rust::src_loc = "119:1"]
pub unsafe extern "C" fn chrand_principal3_2(mut argp: *mut chrand3_arg,
                                             mut res: *mut chrand_ret,
                                             mut clnt: *mut CLIENT)
 -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   20
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_chrand3_arg))),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_chrand_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
#[no_mangle]
#[c2rust::src_loc = "127:1"]
pub unsafe extern "C" fn create_policy_2(mut argp: *mut cpol_arg,
                                         mut res: *mut generic_ret,
                                         mut clnt: *mut CLIENT) -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   8
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_cpol_arg))),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_generic_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
#[no_mangle]
#[c2rust::src_loc = "135:1"]
pub unsafe extern "C" fn delete_policy_2(mut argp: *mut dpol_arg,
                                         mut res: *mut generic_ret,
                                         mut clnt: *mut CLIENT) -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   9
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_dpol_arg))),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_generic_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
#[no_mangle]
#[c2rust::src_loc = "143:1"]
pub unsafe extern "C" fn modify_policy_2(mut argp: *mut mpol_arg,
                                         mut res: *mut generic_ret,
                                         mut clnt: *mut CLIENT) -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   10
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_mpol_arg))),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_generic_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
#[no_mangle]
#[c2rust::src_loc = "151:1"]
pub unsafe extern "C" fn get_policy_2(mut argp: *mut gpol_arg,
                                      mut res: *mut gpol_ret,
                                      mut clnt: *mut CLIENT) -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   11
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_gpol_arg))),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_gpol_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
#[no_mangle]
#[c2rust::src_loc = "159:1"]
pub unsafe extern "C" fn get_pols_2(mut argp: *mut gpols_arg,
                                    mut res: *mut gpols_ret,
                                    mut clnt: *mut CLIENT) -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   15
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_gpols_arg))),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_gpols_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
#[no_mangle]
#[c2rust::src_loc = "167:1"]
pub unsafe extern "C" fn get_privs_2(mut argp: *mut libc::c_void,
                                     mut res: *mut getprivs_ret,
                                     mut clnt: *mut CLIENT) -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   12
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                                           *mut XDR,
                                                                                                                                                                       _:
                                                                                                                                                                           *mut uint32_t)
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(gssrpc_xdr_u_int32
                                                                                                                                                               as
                                                                                                                                                               unsafe extern "C" fn(_:
                                                                                                                                                                                        *mut XDR,
                                                                                                                                                                                    _:
                                                                                                                                                                                        *mut uint32_t)
                                                                                                                                                                   ->
                                                                                                                                                                       libc::c_int)),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_getprivs_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
#[no_mangle]
#[c2rust::src_loc = "175:1"]
pub unsafe extern "C" fn init_2(mut argp: *mut libc::c_void,
                                mut res: *mut generic_ret,
                                mut clnt: *mut CLIENT) -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   13
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                                           *mut XDR,
                                                                                                                                                                       _:
                                                                                                                                                                           *mut uint32_t)
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(gssrpc_xdr_u_int32
                                                                                                                                                               as
                                                                                                                                                               unsafe extern "C" fn(_:
                                                                                                                                                                                        *mut XDR,
                                                                                                                                                                                    _:
                                                                                                                                                                                        *mut uint32_t)
                                                                                                                                                                   ->
                                                                                                                                                                       libc::c_int)),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_generic_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
#[no_mangle]
#[c2rust::src_loc = "183:1"]
pub unsafe extern "C" fn purgekeys_2(mut argp: *mut purgekeys_arg,
                                     mut res: *mut generic_ret,
                                     mut clnt: *mut CLIENT) -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   22
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_purgekeys_arg))),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_generic_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
#[no_mangle]
#[c2rust::src_loc = "191:1"]
pub unsafe extern "C" fn get_strings_2(mut argp: *mut gstrings_arg,
                                       mut res: *mut gstrings_ret,
                                       mut clnt: *mut CLIENT) -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   23
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_gstrings_arg))),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_gstrings_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
#[no_mangle]
#[c2rust::src_loc = "199:1"]
pub unsafe extern "C" fn set_string_2(mut argp: *mut sstring_arg,
                                      mut res: *mut generic_ret,
                                      mut clnt: *mut CLIENT) -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   24
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_sstring_arg))),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_generic_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
/* -*- mode: c; c-file-style: "bsd"; indent-tabs-mode: t -*- */
/* 17 was SETV4KEY_PRINCIPAL (removed in 1.18). */
#[no_mangle]
#[c2rust::src_loc = "207:1"]
pub unsafe extern "C" fn get_principal_keys_2(mut argp: *mut getpkeys_arg,
                                              mut res: *mut getpkeys_ret,
                                              mut clnt: *mut CLIENT)
 -> clnt_stat {
    return Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                   26
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       rpcproc_t,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_getpkeys_arg))),
                                                                                                                   argp
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int>,
                                                                                                                                           xdrproc_t>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int,
                                                                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                                                                       ->
                                                                                                                                                                                           libc::c_int>(xdr_getpkeys_ret))),
                                                                                                                   res
                                                                                                                       as
                                                                                                                       caddr_t
                                                                                                                       as
                                                                                                                       *mut libc::c_void,
                                                                                                                   TIMEOUT);
}
