use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:13"]
pub mod types_h {
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
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
#[c2rust::header_src = "/usr/include/sys/types.h:13"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:13"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timeval.h:13"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:13"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:13"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:13"]
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
    }
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth.h:13"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc_msg.h:13"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/clnt.h:13"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop.h:13"]
pub mod iprop_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "17:9"]
    pub struct utf8str_t {
        pub utf8str_t_len: u_int,
        pub utf8str_t_val: *mut libc::c_char,
    }
    #[c2rust::src_loc = "22:1"]
    pub type kdb_sno_t = uint32_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:8"]
    pub struct kdbe_time_t {
        pub seconds: uint32_t,
        pub useconds: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "30:8"]
    pub struct kdbe_key_t {
        pub k_ver: int32_t,
        pub k_kvno: int32_t,
        pub k_enctype: C2RustUnnamed_10,
        pub k_contents: C2RustUnnamed_9,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:2"]
    pub struct C2RustUnnamed_9 {
        pub k_contents_len: u_int,
        pub k_contents_val: *mut utf8str_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:2"]
    pub struct C2RustUnnamed_10 {
        pub k_enctype_len: u_int,
        pub k_enctype_val: *mut int32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "44:8"]
    pub struct kdbe_data_t {
        pub k_magic: int32_t,
        pub k_data: utf8str_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:8"]
    pub struct kdbe_princ_t {
        pub k_realm: utf8str_t,
        pub k_components: C2RustUnnamed_11,
        pub k_nametype: int32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:2"]
    pub struct C2RustUnnamed_11 {
        pub k_components_len: u_int,
        pub k_components_val: *mut kdbe_data_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "60:8"]
    pub struct kdbe_tl_t {
        pub tl_type: int16_t,
        pub tl_data: C2RustUnnamed_12,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:2"]
    pub struct C2RustUnnamed_12 {
        pub tl_data_len: u_int,
        pub tl_data_val: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "69:9"]
    pub struct kdbe_pw_hist_t {
        pub kdbe_pw_hist_t_len: u_int,
        pub kdbe_pw_hist_t_val: *mut kdbe_key_t,
    }
    #[c2rust::src_loc = "74:1"]
    pub type kdbe_attr_type_t = libc::c_uint;
    #[c2rust::src_loc = "94:2"]
    pub const AT_PW_HIST: kdbe_attr_type_t = 19;
    #[c2rust::src_loc = "93:2"]
    pub const AT_PW_HIST_KVNO: kdbe_attr_type_t = 18;
    #[c2rust::src_loc = "92:2"]
    pub const AT_PW_POLICY_SWITCH: kdbe_attr_type_t = 17;
    #[c2rust::src_loc = "91:2"]
    pub const AT_PW_POLICY: kdbe_attr_type_t = 16;
    #[c2rust::src_loc = "90:2"]
    pub const AT_PW_LAST_CHANGE: kdbe_attr_type_t = 15;
    #[c2rust::src_loc = "89:2"]
    pub const AT_MOD_WHERE: kdbe_attr_type_t = 14;
    #[c2rust::src_loc = "88:2"]
    pub const AT_MOD_TIME: kdbe_attr_type_t = 13;
    #[c2rust::src_loc = "87:2"]
    pub const AT_MOD_PRINC: kdbe_attr_type_t = 12;
    #[c2rust::src_loc = "86:2"]
    pub const AT_LEN: kdbe_attr_type_t = 11;
    #[c2rust::src_loc = "85:2"]
    pub const AT_TL_DATA: kdbe_attr_type_t = 10;
    #[c2rust::src_loc = "84:2"]
    pub const AT_KEYDATA: kdbe_attr_type_t = 9;
    #[c2rust::src_loc = "83:2"]
    pub const AT_PRINC: kdbe_attr_type_t = 8;
    #[c2rust::src_loc = "82:2"]
    pub const AT_FAIL_AUTH_COUNT: kdbe_attr_type_t = 7;
    #[c2rust::src_loc = "81:2"]
    pub const AT_LAST_FAILED: kdbe_attr_type_t = 6;
    #[c2rust::src_loc = "80:2"]
    pub const AT_LAST_SUCCESS: kdbe_attr_type_t = 5;
    #[c2rust::src_loc = "79:2"]
    pub const AT_PW_EXP: kdbe_attr_type_t = 4;
    #[c2rust::src_loc = "78:2"]
    pub const AT_EXP: kdbe_attr_type_t = 3;
    #[c2rust::src_loc = "77:2"]
    pub const AT_MAX_RENEW_LIFE: kdbe_attr_type_t = 2;
    #[c2rust::src_loc = "76:2"]
    pub const AT_MAX_LIFE: kdbe_attr_type_t = 1;
    #[c2rust::src_loc = "75:2"]
    pub const AT_ATTRFLAGS: kdbe_attr_type_t = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "98:8"]
    pub struct kdbe_val_t {
        pub av_type: kdbe_attr_type_t,
        pub kdbe_val_t_u: C2RustUnnamed_13,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "100:2"]
    pub union C2RustUnnamed_13 {
        pub av_attrflags: uint32_t,
        pub av_max_life: uint32_t,
        pub av_max_renew_life: uint32_t,
        pub av_exp: uint32_t,
        pub av_pw_exp: uint32_t,
        pub av_last_success: uint32_t,
        pub av_last_failed: uint32_t,
        pub av_fail_auth_count: uint32_t,
        pub av_princ: kdbe_princ_t,
        pub av_keydata: C2RustUnnamed_17,
        pub av_tldata: C2RustUnnamed_16,
        pub av_len: int16_t,
        pub av_pw_last_change: uint32_t,
        pub av_mod_princ: kdbe_princ_t,
        pub av_mod_time: uint32_t,
        pub av_mod_where: utf8str_t,
        pub av_pw_policy: utf8str_t,
        pub av_pw_policy_switch: libc::c_int,
        pub av_pw_hist_kvno: uint32_t,
        pub av_pw_hist: C2RustUnnamed_15,
        pub av_extension: C2RustUnnamed_14,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "130:3"]
    pub struct C2RustUnnamed_14 {
        pub av_extension_len: u_int,
        pub av_extension_val: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "126:3"]
    pub struct C2RustUnnamed_15 {
        pub av_pw_hist_len: u_int,
        pub av_pw_hist_val: *mut kdbe_pw_hist_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "114:3"]
    pub struct C2RustUnnamed_16 {
        pub av_tldata_len: u_int,
        pub av_tldata_val: *mut kdbe_tl_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "110:3"]
    pub struct C2RustUnnamed_17 {
        pub av_keydata_len: u_int,
        pub av_keydata_val: *mut kdbe_key_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "138:9"]
    pub struct kdbe_t {
        pub kdbe_t_len: u_int,
        pub kdbe_t_val: *mut kdbe_val_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "143:8"]
    pub struct kdb_incr_update_t {
        pub kdb_princ_name: utf8str_t,
        pub kdb_entry_sno: kdb_sno_t,
        pub kdb_time: kdbe_time_t,
        pub kdb_update: kdbe_t,
        pub kdb_deleted: libc::c_int,
        pub kdb_commit: libc::c_int,
        pub kdb_kdcs_seen_by: C2RustUnnamed_19,
        pub kdb_futures: C2RustUnnamed_18,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "154:2"]
    pub struct C2RustUnnamed_18 {
        pub kdb_futures_len: u_int,
        pub kdb_futures_val: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "150:2"]
    pub struct C2RustUnnamed_19 {
        pub kdb_kdcs_seen_by_len: u_int,
        pub kdb_kdcs_seen_by_val: *mut utf8str_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "161:9"]
    pub struct kdb_ulog_t {
        pub kdb_ulog_t_len: u_int,
        pub kdb_ulog_t_val: *mut kdb_incr_update_t,
    }
    #[c2rust::src_loc = "166:1"]
    pub type update_status_t = libc::c_uint;
    #[c2rust::src_loc = "172:2"]
    pub const UPDATE_PERM_DENIED: update_status_t = 5;
    #[c2rust::src_loc = "171:2"]
    pub const UPDATE_NIL: update_status_t = 4;
    #[c2rust::src_loc = "170:2"]
    pub const UPDATE_BUSY: update_status_t = 3;
    #[c2rust::src_loc = "169:2"]
    pub const UPDATE_FULL_RESYNC_NEEDED: update_status_t = 2;
    #[c2rust::src_loc = "168:2"]
    pub const UPDATE_ERROR: update_status_t = 1;
    #[c2rust::src_loc = "167:2"]
    pub const UPDATE_OK: update_status_t = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "176:8"]
    pub struct kdb_last_t {
        pub last_sno: kdb_sno_t,
        pub last_time: kdbe_time_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "182:8"]
    pub struct kdb_incr_result_t {
        pub lastentry: kdb_last_t,
        pub updates: kdb_ulog_t,
        pub ret: update_status_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "189:8"]
    pub struct kdb_fullresync_result_t {
        pub lastentry: kdb_last_t,
        pub ret: update_status_t,
    }
    use super::sys_types_h::u_int;
    use super::stdint_uintn_h::uint32_t;
    use super::stdint_intn_h::{int32_t, int16_t};
    use super::xdr_h::XDR;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "248:1"]
        pub fn xdr_kdb_fullresync_result_t(_: *mut XDR,
                                           _: *mut kdb_fullresync_result_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "246:1"]
        pub fn xdr_kdb_last_t(_: *mut XDR, _: *mut kdb_last_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "247:1"]
        pub fn xdr_kdb_incr_result_t(_: *mut XDR, _: *mut kdb_incr_result_t)
         -> libc::c_int;
    }
    /* !_IPROP_H_RPCGEN */
    /* K&R C */
    /* K&R C */
}
#[c2rust::header_src = "/usr/include/string.h:12"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
pub use self::types_h::{__u_int, __int16_t, __int32_t, __uint32_t, __time_t,
                        __suseconds_t, __caddr_t};
pub use self::sys_types_h::{u_int, caddr_t};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::struct_timeval_h::timeval;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t, rpc_inline_t};
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, gssrpc_xdr_void};
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
pub use self::iprop_h::{utf8str_t, kdb_sno_t, kdbe_time_t, kdbe_key_t,
                        C2RustUnnamed_9, C2RustUnnamed_10, kdbe_data_t,
                        kdbe_princ_t, C2RustUnnamed_11, kdbe_tl_t,
                        C2RustUnnamed_12, kdbe_pw_hist_t, kdbe_attr_type_t,
                        AT_PW_HIST, AT_PW_HIST_KVNO, AT_PW_POLICY_SWITCH,
                        AT_PW_POLICY, AT_PW_LAST_CHANGE, AT_MOD_WHERE,
                        AT_MOD_TIME, AT_MOD_PRINC, AT_LEN, AT_TL_DATA,
                        AT_KEYDATA, AT_PRINC, AT_FAIL_AUTH_COUNT,
                        AT_LAST_FAILED, AT_LAST_SUCCESS, AT_PW_EXP, AT_EXP,
                        AT_MAX_RENEW_LIFE, AT_MAX_LIFE, AT_ATTRFLAGS,
                        kdbe_val_t, C2RustUnnamed_13, C2RustUnnamed_14,
                        C2RustUnnamed_15, C2RustUnnamed_16, C2RustUnnamed_17,
                        kdbe_t, kdb_incr_update_t, C2RustUnnamed_18,
                        C2RustUnnamed_19, kdb_ulog_t, update_status_t,
                        UPDATE_PERM_DENIED, UPDATE_NIL, UPDATE_BUSY,
                        UPDATE_FULL_RESYNC_NEEDED, UPDATE_ERROR, UPDATE_OK,
                        kdb_last_t, kdb_incr_result_t,
                        kdb_fullresync_result_t, xdr_kdb_fullresync_result_t,
                        xdr_kdb_last_t, xdr_kdb_incr_result_t};
use self::string_h::memset;
/* -*- mode: c; c-file-style: "bsd"; indent-tabs-mode: t -*- */
/*
 * Copyright 2004 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
/*
 * Please do not edit this file.
 * It was generated using rpcgen.
 */
/* for memset */
/* Default timeout can be changed using clnt_control() */
#[c2rust::src_loc = "16:23"]
static mut TIMEOUT: timeval =
    {
        let mut init =
            timeval{tv_sec: 25 as libc::c_int as __time_t,
                    tv_usec: 0 as libc::c_int as __suseconds_t,};
        init
    };
#[no_mangle]
#[c2rust::src_loc = "18:1"]
pub unsafe extern "C" fn iprop_null_1(mut argp: *mut libc::c_void,
                                      mut clnt: *mut CLIENT)
 -> *mut libc::c_void {
    static mut clnt_res: libc::c_char = 0;
    memset(&mut clnt_res as *mut libc::c_char as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<libc::c_char>() as libc::c_ulong);
    if Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
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
                                                                                                               argp
                                                                                                                   as
                                                                                                                   caddr_t
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
                                                                                                               &mut clnt_res
                                                                                                                   as
                                                                                                                   *mut libc::c_char
                                                                                                                   as
                                                                                                                   *mut libc::c_void,
                                                                                                               TIMEOUT)
           as libc::c_uint != RPC_SUCCESS as libc::c_int as libc::c_uint {
        return 0 as *mut libc::c_void
    }
    return &mut clnt_res as *mut libc::c_char as *mut libc::c_void;
}
#[no_mangle]
#[c2rust::src_loc = "33:1"]
pub unsafe extern "C" fn iprop_get_updates_1(mut argp: *mut kdb_last_t,
                                             mut clnt: *mut CLIENT)
 -> *mut kdb_incr_result_t {
    static mut clnt_res: kdb_incr_result_t =
        kdb_incr_result_t{lastentry:
                              kdb_last_t{last_sno: 0,
                                         last_time:
                                             kdbe_time_t{seconds: 0,
                                                         useconds: 0,},},
                          updates:
                              kdb_ulog_t{kdb_ulog_t_len: 0,
                                         kdb_ulog_t_val:
                                             0 as *const kdb_incr_update_t as
                                                 *mut kdb_incr_update_t,},
                          ret: UPDATE_OK,};
    memset(&mut clnt_res as *mut kdb_incr_result_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<kdb_incr_result_t>() as libc::c_ulong);
    if Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                               1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   rpcproc_t,
                                                                                                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                                       *mut XDR,
                                                                                                                                                                   _:
                                                                                                                                                                       *mut kdb_last_t)
                                                                                                                                                  ->
                                                                                                                                                      libc::c_int>,
                                                                                                                                       xdrproc_t>(Some(xdr_kdb_last_t
                                                                                                                                                           as
                                                                                                                                                           unsafe extern "C" fn(_:
                                                                                                                                                                                    *mut XDR,
                                                                                                                                                                                _:
                                                                                                                                                                                    *mut kdb_last_t)
                                                                                                                                                               ->
                                                                                                                                                                   libc::c_int)),
                                                                                                               argp
                                                                                                                   as
                                                                                                                   caddr_t
                                                                                                                   as
                                                                                                                   *mut libc::c_void,
                                                                                                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                                       *mut XDR,
                                                                                                                                                                   _:
                                                                                                                                                                       *mut kdb_incr_result_t)
                                                                                                                                                  ->
                                                                                                                                                      libc::c_int>,
                                                                                                                                       xdrproc_t>(Some(xdr_kdb_incr_result_t
                                                                                                                                                           as
                                                                                                                                                           unsafe extern "C" fn(_:
                                                                                                                                                                                    *mut XDR,
                                                                                                                                                                                _:
                                                                                                                                                                                    *mut kdb_incr_result_t)
                                                                                                                                                               ->
                                                                                                                                                                   libc::c_int)),
                                                                                                               &mut clnt_res
                                                                                                                   as
                                                                                                                   *mut kdb_incr_result_t
                                                                                                                   as
                                                                                                                   caddr_t
                                                                                                                   as
                                                                                                                   *mut libc::c_void,
                                                                                                               TIMEOUT)
           as libc::c_uint != RPC_SUCCESS as libc::c_int as libc::c_uint {
        return 0 as *mut kdb_incr_result_t
    }
    return &mut clnt_res;
}
/*
 * Please do not edit this file.
 * It was generated using rpcgen.
 */
#[no_mangle]
#[c2rust::src_loc = "48:1"]
pub unsafe extern "C" fn iprop_full_resync_1(mut argp: *mut libc::c_void,
                                             mut clnt: *mut CLIENT)
 -> *mut kdb_fullresync_result_t {
    static mut clnt_res: kdb_fullresync_result_t =
        kdb_fullresync_result_t{lastentry:
                                    kdb_last_t{last_sno: 0,
                                               last_time:
                                                   kdbe_time_t{seconds: 0,
                                                               useconds:
                                                                   0,},},
                                ret: UPDATE_OK,};
    memset(&mut clnt_res as *mut kdb_fullresync_result_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<kdb_fullresync_result_t>() as libc::c_ulong);
    if Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                               2
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
                                                                                                               argp
                                                                                                                   as
                                                                                                                   caddr_t
                                                                                                                   as
                                                                                                                   *mut libc::c_void,
                                                                                                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                                       *mut XDR,
                                                                                                                                                                   _:
                                                                                                                                                                       *mut kdb_fullresync_result_t)
                                                                                                                                                  ->
                                                                                                                                                      libc::c_int>,
                                                                                                                                       xdrproc_t>(Some(xdr_kdb_fullresync_result_t
                                                                                                                                                           as
                                                                                                                                                           unsafe extern "C" fn(_:
                                                                                                                                                                                    *mut XDR,
                                                                                                                                                                                _:
                                                                                                                                                                                    *mut kdb_fullresync_result_t)
                                                                                                                                                               ->
                                                                                                                                                                   libc::c_int)),
                                                                                                               &mut clnt_res
                                                                                                                   as
                                                                                                                   *mut kdb_fullresync_result_t
                                                                                                                   as
                                                                                                                   caddr_t
                                                                                                                   as
                                                                                                                   *mut libc::c_void,
                                                                                                               TIMEOUT)
           as libc::c_uint != RPC_SUCCESS as libc::c_int as libc::c_uint {
        return 0 as *mut kdb_fullresync_result_t
    }
    return &mut clnt_res;
}
