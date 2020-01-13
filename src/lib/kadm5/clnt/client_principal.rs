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
    #[c2rust::src_loc = "81:1"]
    pub type xdr_op = libc::c_uint;
    #[c2rust::src_loc = "84:2"]
    pub const XDR_FREE: xdr_op = 2;
    #[c2rust::src_loc = "83:2"]
    pub const XDR_DECODE: xdr_op = 1;
    #[c2rust::src_loc = "82:2"]
    pub const XDR_ENCODE: xdr_op = 0;
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
    /* this strange form is necessary since - is a unary operator, not a sign
   indicator */
    /* this strange form is necessary since - is a unary operator, not a sign
   indicator */
    /*
 * end wordsize.h
 */
    /*
 * begin "base-defs.h"
 */
    /*
 * Basic definitions for Kerberos V5 library
 */
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
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
    /* *
 * Used to convey an operation status.  The value 0 indicates success; any
 * other values are com_err codes.  Use krb5_get_error_message() to obtain a
 * string describing the error.
 */
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
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "363:16"]
    pub struct _krb5_keyblock {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /*
 * begin "encryption.h"
 */
    /* * Exposed contents of a key. */
    #[c2rust::src_loc = "363:1"]
    pub type krb5_keyblock = _krb5_keyblock;
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
        /* *
 * Free the contents of a krb5_keyblock structure.
 *
 * @param [in] context          Library context
 * @param [in] key              Keyblock to be freed
 *
 * This function frees the contents of @a key, but not the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4721:1"]
        pub fn krb5_free_keyblock_contents(context: krb5_context,
                                           key: *mut krb5_keyblock);
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
    #[c2rust::src_loc = "156:16"]
    pub struct krb5_string_attr_st {
        pub key: *mut libc::c_char,
        pub value: *mut libc::c_char,
    }
    #[c2rust::src_loc = "156:1"]
    pub type krb5_string_attr = krb5_string_attr_st;
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
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:9"]
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
    #[c2rust::src_loc = "190:1"]
    pub type kadm5_principal_ent_rec = _kadm5_principal_ent_t;
    #[c2rust::src_loc = "190:1"]
    pub type kadm5_principal_ent_t = *mut _kadm5_principal_ent_t;
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
                        krb5_flags, krb5_kvno, krb5_int16, krb5_enctype,
                        krb5_int32, krb5_keyblock};
    use super::kdb_h::{krb5_tl_data, krb5_key_data, krb5_key_salt_tuple,
                       krb5_keysalt};
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
    /* -*- mode: c; c-file-style: "bsd"; indent-tabs-mode: t -*- */
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
    #[c2rust::src_loc = "144:8"]
    pub struct gprinc_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub rec: kadm5_principal_ent_rec,
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
    #[c2rust::src_loc = "60:8"]
    pub struct gprincs_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub princs: *mut *mut libc::c_char,
        pub count: libc::c_int,
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
    #[c2rust::src_loc = "241:8"]
    pub struct getpkeys_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub key_data: *mut kadm5_key_data,
        pub n_key_data: libc::c_int,
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
    #[c2rust::src_loc = "205:8"]
    pub struct purgekeys_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub keepkvno: libc::c_int,
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
    #[c2rust::src_loc = "212:8"]
    pub struct gstrings_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
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
    use super::krb5_h::{krb5_ui_4, krb5_principal, krb5_boolean,
                        krb5_keyblock, krb5_kvno};
    use super::admin_h::{kadm5_ret_t, kadm5_principal_ent_rec,
                         kadm5_key_data};
    use super::kdb_h::{krb5_key_salt_tuple, krb5_string_attr};
    use super::clnt_h::{CLIENT, clnt_stat};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "252:1"]
        pub fn create_principal_2(_: *mut cprinc_arg, _: *mut generic_ret,
                                  _: *mut CLIENT) -> clnt_stat;
        #[no_mangle]
        #[c2rust::src_loc = "257:1"]
        pub fn delete_principal_2(_: *mut dprinc_arg, _: *mut generic_ret,
                                  _: *mut CLIENT) -> clnt_stat;
        #[no_mangle]
        #[c2rust::src_loc = "262:1"]
        pub fn modify_principal_2(_: *mut mprinc_arg, _: *mut generic_ret,
                                  _: *mut CLIENT) -> clnt_stat;
        #[no_mangle]
        #[c2rust::src_loc = "267:1"]
        pub fn rename_principal_2(_: *mut rprinc_arg, _: *mut generic_ret,
                                  _: *mut CLIENT) -> clnt_stat;
        #[no_mangle]
        #[c2rust::src_loc = "272:1"]
        pub fn get_principal_2(_: *mut gprinc_arg, _: *mut gprinc_ret,
                               _: *mut CLIENT) -> clnt_stat;
        #[no_mangle]
        #[c2rust::src_loc = "276:1"]
        pub fn chpass_principal_2(_: *mut chpass_arg, _: *mut generic_ret,
                                  _: *mut CLIENT) -> clnt_stat;
        #[no_mangle]
        #[c2rust::src_loc = "281:1"]
        pub fn chrand_principal_2(_: *mut chrand_arg, _: *mut chrand_ret,
                                  _: *mut CLIENT) -> clnt_stat;
        #[no_mangle]
        #[c2rust::src_loc = "307:1"]
        pub fn get_princs_2(_: *mut gprincs_arg, _: *mut gprincs_ret,
                            _: *mut CLIENT) -> clnt_stat;
        #[no_mangle]
        #[c2rust::src_loc = "314:1"]
        pub fn setkey_principal_2(_: *mut setkey_arg, _: *mut generic_ret,
                                  _: *mut CLIENT) -> clnt_stat;
        /* 17 was SETV4KEY_PRINCIPAL (removed in 1.18). */
        #[no_mangle]
        #[c2rust::src_loc = "322:1"]
        pub fn create_principal3_2(_: *mut cprinc3_arg, _: *mut generic_ret,
                                   _: *mut CLIENT) -> clnt_stat;
        #[no_mangle]
        #[c2rust::src_loc = "327:1"]
        pub fn chpass_principal3_2(_: *mut chpass3_arg, _: *mut generic_ret,
                                   _: *mut CLIENT) -> clnt_stat;
        #[no_mangle]
        #[c2rust::src_loc = "332:1"]
        pub fn chrand_principal3_2(_: *mut chrand3_arg, _: *mut chrand_ret,
                                   _: *mut CLIENT) -> clnt_stat;
        #[no_mangle]
        #[c2rust::src_loc = "337:1"]
        pub fn setkey_principal3_2(_: *mut setkey3_arg, _: *mut generic_ret,
                                   _: *mut CLIENT) -> clnt_stat;
        #[no_mangle]
        #[c2rust::src_loc = "342:1"]
        pub fn purgekeys_2(_: *mut purgekeys_arg, _: *mut generic_ret,
                           _: *mut CLIENT) -> clnt_stat;
        #[no_mangle]
        #[c2rust::src_loc = "346:1"]
        pub fn get_strings_2(_: *mut gstrings_arg, _: *mut gstrings_ret,
                             _: *mut CLIENT) -> clnt_stat;
        #[no_mangle]
        #[c2rust::src_loc = "350:1"]
        pub fn set_string_2(_: *mut sstring_arg, _: *mut generic_ret,
                            _: *mut CLIENT) -> clnt_stat;
        #[no_mangle]
        #[c2rust::src_loc = "354:1"]
        pub fn setkey_principal4_2(_: *mut setkey4_arg, _: *mut generic_ret,
                                   _: *mut CLIENT) -> clnt_stat;
        #[no_mangle]
        #[c2rust::src_loc = "359:1"]
        pub fn get_principal_keys_2(_: *mut getpkeys_arg,
                                    _: *mut getpkeys_ret, _: *mut CLIENT)
         -> clnt_stat;
    }
    /* __KADM_RPC_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/kadm5/clnt/client_internal.h:16"]
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
#[c2rust::header_src = "/usr/include/stdlib.h:8"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/string.h:14"]
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
                       krb5_ui_4, krb5_boolean, krb5_kvno, krb5_enctype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal, krb5_context,
                       _krb5_keyblock, krb5_keyblock, _krb5_context,
                       krb5_free_keyblock_contents};
pub use self::kdb_h::{_krb5_tl_data, krb5_tl_data, krb5_string_attr_st,
                      krb5_string_attr, _krb5_key_data, krb5_key_data,
                      _krb5_keysalt, krb5_keysalt, __krb5_key_salt_tuple,
                      krb5_key_salt_tuple};
pub use self::admin_h::{kadm5_ret_t, _kadm5_principal_ent_t,
                        kadm5_principal_ent_rec, kadm5_principal_ent_t,
                        _kadm5_config_params, kadm5_config_params,
                        _kadm5_key_data, kadm5_key_data};
pub use self::kadm_rpc_h::{generic_ret, cprinc_arg, cprinc3_arg, dprinc_arg,
                           mprinc_arg, rprinc_arg, gprinc_ret, gprinc_arg,
                           chpass_arg, chpass3_arg, chrand_ret, chrand_arg,
                           chrand3_arg, setkey_arg, setkey3_arg, setkey4_arg,
                           gprincs_ret, gprincs_arg, getpkeys_ret,
                           getpkeys_arg, purgekeys_arg, gstrings_ret,
                           gstrings_arg, sstring_arg, create_principal_2,
                           delete_principal_2, modify_principal_2,
                           rename_principal_2, get_principal_2,
                           chpass_principal_2, chrand_principal_2,
                           get_princs_2, setkey_principal_2,
                           create_principal3_2, chpass_principal3_2,
                           chrand_principal3_2, setkey_principal3_2,
                           purgekeys_2, get_strings_2, set_string_2,
                           setkey_principal4_2, get_principal_keys_2};
pub use self::client_internal_h::{kadm5_server_handle_t,
                                  _kadm5_server_handle_t};
use self::stdlib_h::free;
use self::string_h::{memcpy, memset};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved
 *
 * $Header$
 */
#[no_mangle]
#[c2rust::src_loc = "24:1"]
pub unsafe extern "C" fn kadm5_create_principal(mut server_handle:
                                                    *mut libc::c_void,
                                                mut princ:
                                                    kadm5_principal_ent_t,
                                                mut mask: libc::c_long,
                                                mut pw: *mut libc::c_char)
 -> kadm5_ret_t {
    let mut r: generic_ret =
        {
            let mut init =
                generic_ret{api_version: 0 as libc::c_int as krb5_ui_4,
                            code: 0 as libc::c_int as kadm5_ret_t,};
            init
        };
    let mut arg: cprinc_arg =
        cprinc_arg{api_version: 0,
                   rec:
                       kadm5_principal_ent_rec{principal:
                                                   0 as
                                                       *mut krb5_principal_data,
                                               princ_expire_time: 0,
                                               last_pwd_change: 0,
                                               pw_expiration: 0,
                                               max_life: 0,
                                               mod_name:
                                                   0 as
                                                       *mut krb5_principal_data,
                                               mod_date: 0,
                                               attributes: 0,
                                               kvno: 0,
                                               mkvno: 0,
                                               policy: 0 as *mut libc::c_char,
                                               aux_attributes: 0,
                                               max_renewable_life: 0,
                                               last_success: 0,
                                               last_failed: 0,
                                               fail_auth_count: 0,
                                               n_key_data: 0,
                                               n_tl_data: 0,
                                               tl_data:
                                                   0 as *mut krb5_tl_data,
                                               key_data:
                                                   0 as *mut krb5_key_data,},
                   mask: 0,
                   passwd: 0 as *mut libc::c_char,};
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
    memset(&mut arg as *mut cprinc_arg as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<cprinc_arg>() as libc::c_ulong);
    arg.mask = mask;
    arg.passwd = pw;
    arg.api_version = (*handle).api_version;
    if princ.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    memcpy(&mut arg.rec as *mut kadm5_principal_ent_rec as *mut libc::c_void,
           princ as *const libc::c_void,
           ::std::mem::size_of::<kadm5_principal_ent_rec>() as libc::c_ulong);
    arg.rec.mod_name = 0 as krb5_principal;
    if mask & 0x800 as libc::c_int as libc::c_long == 0 {
        arg.rec.policy = 0 as *mut libc::c_char
    }
    if mask & 0x20000 as libc::c_int as libc::c_long == 0 {
        arg.rec.n_key_data = 0 as libc::c_int as krb5_int16;
        arg.rec.key_data = 0 as *mut krb5_key_data
    }
    if mask & 0x40000 as libc::c_int as libc::c_long == 0 {
        arg.rec.n_tl_data = 0 as libc::c_int as krb5_int16;
        arg.rec.tl_data = 0 as *mut krb5_tl_data
    }
    if create_principal_2(&mut arg, &mut r, (*handle).clnt) as u64 != 0 {
        return 43787528 as libc::c_long
    }
    return r.code;
}
#[no_mangle]
#[c2rust::src_loc = "62:1"]
pub unsafe extern "C" fn kadm5_create_principal_3(mut server_handle:
                                                      *mut libc::c_void,
                                                  mut princ:
                                                      kadm5_principal_ent_t,
                                                  mut mask: libc::c_long,
                                                  mut n_ks_tuple: libc::c_int,
                                                  mut ks_tuple:
                                                      *mut krb5_key_salt_tuple,
                                                  mut pw: *mut libc::c_char)
 -> kadm5_ret_t {
    let mut r: generic_ret =
        {
            let mut init =
                generic_ret{api_version: 0 as libc::c_int as krb5_ui_4,
                            code: 0 as libc::c_int as kadm5_ret_t,};
            init
        };
    let mut arg: cprinc3_arg =
        cprinc3_arg{api_version: 0,
                    rec:
                        kadm5_principal_ent_rec{principal:
                                                    0 as
                                                        *mut krb5_principal_data,
                                                princ_expire_time: 0,
                                                last_pwd_change: 0,
                                                pw_expiration: 0,
                                                max_life: 0,
                                                mod_name:
                                                    0 as
                                                        *mut krb5_principal_data,
                                                mod_date: 0,
                                                attributes: 0,
                                                kvno: 0,
                                                mkvno: 0,
                                                policy:
                                                    0 as *mut libc::c_char,
                                                aux_attributes: 0,
                                                max_renewable_life: 0,
                                                last_success: 0,
                                                last_failed: 0,
                                                fail_auth_count: 0,
                                                n_key_data: 0,
                                                n_tl_data: 0,
                                                tl_data:
                                                    0 as *mut krb5_tl_data,
                                                key_data:
                                                    0 as *mut krb5_key_data,},
                    mask: 0,
                    n_ks_tuple: 0,
                    ks_tuple: 0 as *mut krb5_key_salt_tuple,
                    passwd: 0 as *mut libc::c_char,};
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
    memset(&mut arg as *mut cprinc3_arg as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<cprinc3_arg>() as libc::c_ulong);
    arg.mask = mask;
    arg.passwd = pw;
    arg.api_version = (*handle).api_version;
    arg.n_ks_tuple = n_ks_tuple;
    arg.ks_tuple = ks_tuple;
    if princ.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    memcpy(&mut arg.rec as *mut kadm5_principal_ent_rec as *mut libc::c_void,
           princ as *const libc::c_void,
           ::std::mem::size_of::<kadm5_principal_ent_rec>() as libc::c_ulong);
    arg.rec.mod_name = 0 as krb5_principal;
    if mask & 0x800 as libc::c_int as libc::c_long == 0 {
        arg.rec.policy = 0 as *mut libc::c_char
    }
    if mask & 0x20000 as libc::c_int as libc::c_long == 0 {
        arg.rec.n_key_data = 0 as libc::c_int as krb5_int16;
        arg.rec.key_data = 0 as *mut krb5_key_data
    }
    if mask & 0x40000 as libc::c_int as libc::c_long == 0 {
        arg.rec.n_tl_data = 0 as libc::c_int as krb5_int16;
        arg.rec.tl_data = 0 as *mut krb5_tl_data
    }
    if create_principal3_2(&mut arg, &mut r, (*handle).clnt) as u64 != 0 {
        return 43787528 as libc::c_long
    }
    return r.code;
}
#[no_mangle]
#[c2rust::src_loc = "104:1"]
pub unsafe extern "C" fn kadm5_delete_principal(mut server_handle:
                                                    *mut libc::c_void,
                                                mut principal: krb5_principal)
 -> kadm5_ret_t {
    let mut arg: dprinc_arg =
        dprinc_arg{api_version: 0, princ: 0 as *mut krb5_principal_data,};
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
    if principal.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    arg.princ = principal;
    arg.api_version = (*handle).api_version;
    if delete_principal_2(&mut arg, &mut r, (*handle).clnt) as u64 != 0 {
        return 43787528 as libc::c_long
    }
    return r.code;
}
#[no_mangle]
#[c2rust::src_loc = "122:1"]
pub unsafe extern "C" fn kadm5_modify_principal(mut server_handle:
                                                    *mut libc::c_void,
                                                mut princ:
                                                    kadm5_principal_ent_t,
                                                mut mask: libc::c_long)
 -> kadm5_ret_t {
    let mut arg: mprinc_arg =
        mprinc_arg{api_version: 0,
                   rec:
                       kadm5_principal_ent_rec{principal:
                                                   0 as
                                                       *mut krb5_principal_data,
                                               princ_expire_time: 0,
                                               last_pwd_change: 0,
                                               pw_expiration: 0,
                                               max_life: 0,
                                               mod_name:
                                                   0 as
                                                       *mut krb5_principal_data,
                                               mod_date: 0,
                                               attributes: 0,
                                               kvno: 0,
                                               mkvno: 0,
                                               policy: 0 as *mut libc::c_char,
                                               aux_attributes: 0,
                                               max_renewable_life: 0,
                                               last_success: 0,
                                               last_failed: 0,
                                               fail_auth_count: 0,
                                               n_key_data: 0,
                                               n_tl_data: 0,
                                               tl_data:
                                                   0 as *mut krb5_tl_data,
                                               key_data:
                                                   0 as *mut krb5_key_data,},
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
    memset(&mut arg as *mut mprinc_arg as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<mprinc_arg>() as libc::c_ulong);
    arg.mask = mask;
    arg.api_version = (*handle).api_version;
    if princ.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    memcpy(&mut arg.rec as *mut kadm5_principal_ent_rec as *mut libc::c_void,
           princ as *const libc::c_void,
           ::std::mem::size_of::<kadm5_principal_ent_rec>() as libc::c_ulong);
    if mask & 0x800 as libc::c_int as libc::c_long == 0 {
        arg.rec.policy = 0 as *mut libc::c_char
    }
    if mask & 0x20000 as libc::c_int as libc::c_long == 0 {
        arg.rec.n_key_data = 0 as libc::c_int as krb5_int16;
        arg.rec.key_data = 0 as *mut krb5_key_data
    }
    if mask & 0x40000 as libc::c_int as libc::c_long == 0 {
        arg.rec.n_tl_data = 0 as libc::c_int as krb5_int16;
        arg.rec.tl_data = 0 as *mut krb5_tl_data
    }
    arg.rec.mod_name = 0 as krb5_principal;
    if modify_principal_2(&mut arg, &mut r, (*handle).clnt) as u64 != 0 {
        return 43787528 as libc::c_long
    }
    return r.code;
}
#[no_mangle]
#[c2rust::src_loc = "156:1"]
pub unsafe extern "C" fn kadm5_get_principal(mut server_handle:
                                                 *mut libc::c_void,
                                             mut princ: krb5_principal,
                                             mut ent: kadm5_principal_ent_t,
                                             mut mask: libc::c_long)
 -> kadm5_ret_t {
    let mut arg: gprinc_arg =
        gprinc_arg{api_version: 0,
                   princ: 0 as *mut krb5_principal_data,
                   mask: 0,};
    let mut r: gprinc_ret =
        gprinc_ret{api_version: 0,
                   code: 0,
                   rec:
                       kadm5_principal_ent_rec{principal:
                                                   0 as
                                                       *mut krb5_principal_data,
                                               princ_expire_time: 0,
                                               last_pwd_change: 0,
                                               pw_expiration: 0,
                                               max_life: 0,
                                               mod_name:
                                                   0 as
                                                       *mut krb5_principal_data,
                                               mod_date: 0,
                                               attributes: 0,
                                               kvno: 0,
                                               mkvno: 0,
                                               policy: 0 as *mut libc::c_char,
                                               aux_attributes: 0,
                                               max_renewable_life: 0,
                                               last_success: 0,
                                               last_failed: 0,
                                               fail_auth_count: 0,
                                               n_key_data: 0,
                                               n_tl_data: 0,
                                               tl_data:
                                                   0 as *mut krb5_tl_data,
                                               key_data:
                                                   0 as
                                                       *mut krb5_key_data,},};
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
    if princ.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    arg.princ = princ;
    arg.mask = mask;
    arg.api_version = (*handle).api_version;
    memset(&mut r as *mut gprinc_ret as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<gprinc_ret>() as libc::c_ulong);
    if get_principal_2(&mut arg, &mut r, (*handle).clnt) as u64 != 0 {
        return 43787528 as libc::c_long
    }
    if r.code == 0 as libc::c_int as libc::c_long {
        memcpy(ent as *mut libc::c_void,
               &mut r.rec as *mut kadm5_principal_ent_rec as
                   *const libc::c_void,
               ::std::mem::size_of::<kadm5_principal_ent_rec>() as
                   libc::c_ulong);
    }
    return r.code;
}
#[no_mangle]
#[c2rust::src_loc = "181:1"]
pub unsafe extern "C" fn kadm5_get_principals(mut server_handle:
                                                  *mut libc::c_void,
                                              mut exp: *mut libc::c_char,
                                              mut princs:
                                                  *mut *mut *mut libc::c_char,
                                              mut count: *mut libc::c_int)
 -> kadm5_ret_t {
    let mut arg: gprincs_arg =
        gprincs_arg{api_version: 0, exp: 0 as *mut libc::c_char,};
    let mut r: gprincs_ret =
        gprincs_ret{api_version: 0,
                    code: 0,
                    princs: 0 as *mut *mut libc::c_char,
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
    if princs.is_null() || count.is_null() {
        return 22 as libc::c_int as kadm5_ret_t
    }
    arg.exp = exp;
    arg.api_version = (*handle).api_version;
    memset(&mut r as *mut gprincs_ret as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<gprincs_ret>() as libc::c_ulong);
    if get_princs_2(&mut arg, &mut r, (*handle).clnt) as u64 != 0 {
        return 43787528 as libc::c_long
    }
    if r.code == 0 as libc::c_int as libc::c_long {
        *count = r.count;
        *princs = r.princs
    } else {
        *count = 0 as libc::c_int;
        *princs = 0 as *mut *mut libc::c_char
    }
    return r.code;
}
#[no_mangle]
#[c2rust::src_loc = "209:1"]
pub unsafe extern "C" fn kadm5_rename_principal(mut server_handle:
                                                    *mut libc::c_void,
                                                mut source: krb5_principal,
                                                mut dest: krb5_principal)
 -> kadm5_ret_t {
    let mut arg: rprinc_arg =
        rprinc_arg{api_version: 0,
                   src: 0 as *mut krb5_principal_data,
                   dest: 0 as *mut krb5_principal_data,};
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
    arg.src = source;
    arg.dest = dest;
    arg.api_version = (*handle).api_version;
    if source.is_null() || dest.is_null() {
        return 22 as libc::c_int as kadm5_ret_t
    }
    if rename_principal_2(&mut arg, &mut r, (*handle).clnt) as u64 != 0 {
        return 43787528 as libc::c_long
    }
    return r.code;
}
#[no_mangle]
#[c2rust::src_loc = "229:1"]
pub unsafe extern "C" fn kadm5_chpass_principal(mut server_handle:
                                                    *mut libc::c_void,
                                                mut princ: krb5_principal,
                                                mut password:
                                                    *mut libc::c_char)
 -> kadm5_ret_t {
    let mut arg: chpass_arg =
        chpass_arg{api_version: 0,
                   princ: 0 as *mut krb5_principal_data,
                   pass: 0 as *mut libc::c_char,};
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
    arg.princ = princ;
    arg.pass = password;
    arg.api_version = (*handle).api_version;
    if princ.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    if chpass_principal_2(&mut arg, &mut r, (*handle).clnt) as u64 != 0 {
        return 43787528 as libc::c_long
    }
    return r.code;
}
#[no_mangle]
#[c2rust::src_loc = "250:1"]
pub unsafe extern "C" fn kadm5_chpass_principal_3(mut server_handle:
                                                      *mut libc::c_void,
                                                  mut princ: krb5_principal,
                                                  mut keepold: krb5_boolean,
                                                  mut n_ks_tuple: libc::c_int,
                                                  mut ks_tuple:
                                                      *mut krb5_key_salt_tuple,
                                                  mut password:
                                                      *mut libc::c_char)
 -> kadm5_ret_t {
    let mut arg: chpass3_arg =
        chpass3_arg{api_version: 0,
                    princ: 0 as *mut krb5_principal_data,
                    keepold: 0,
                    n_ks_tuple: 0,
                    ks_tuple: 0 as *mut krb5_key_salt_tuple,
                    pass: 0 as *mut libc::c_char,};
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
    arg.princ = princ;
    arg.pass = password;
    arg.api_version = (*handle).api_version;
    arg.keepold = keepold;
    arg.n_ks_tuple = n_ks_tuple;
    arg.ks_tuple = ks_tuple;
    if princ.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    if chpass_principal3_2(&mut arg, &mut r, (*handle).clnt) as u64 != 0 {
        return 43787528 as libc::c_long
    }
    return r.code;
}
#[no_mangle]
#[c2rust::src_loc = "276:1"]
pub unsafe extern "C" fn kadm5_setkey_principal(mut server_handle:
                                                    *mut libc::c_void,
                                                mut princ: krb5_principal,
                                                mut keyblocks:
                                                    *mut krb5_keyblock,
                                                mut n_keys: libc::c_int)
 -> kadm5_ret_t {
    let mut arg: setkey_arg =
        setkey_arg{api_version: 0,
                   princ: 0 as *mut krb5_principal_data,
                   keyblocks: 0 as *mut krb5_keyblock,
                   n_keys: 0,};
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
    arg.princ = princ;
    arg.keyblocks = keyblocks;
    arg.n_keys = n_keys;
    arg.api_version = (*handle).api_version;
    if princ.is_null() || keyblocks.is_null() {
        return 22 as libc::c_int as kadm5_ret_t
    }
    if setkey_principal_2(&mut arg, &mut r, (*handle).clnt) as u64 != 0 {
        return 43787528 as libc::c_long
    }
    return r.code;
}
#[no_mangle]
#[c2rust::src_loc = "300:1"]
pub unsafe extern "C" fn kadm5_setkey_principal_3(mut server_handle:
                                                      *mut libc::c_void,
                                                  mut princ: krb5_principal,
                                                  mut keepold: krb5_boolean,
                                                  mut n_ks_tuple: libc::c_int,
                                                  mut ks_tuple:
                                                      *mut krb5_key_salt_tuple,
                                                  mut keyblocks:
                                                      *mut krb5_keyblock,
                                                  mut n_keys: libc::c_int)
 -> kadm5_ret_t {
    let mut arg: setkey3_arg =
        setkey3_arg{api_version: 0,
                    princ: 0 as *mut krb5_principal_data,
                    keepold: 0,
                    n_ks_tuple: 0,
                    ks_tuple: 0 as *mut krb5_key_salt_tuple,
                    keyblocks: 0 as *mut krb5_keyblock,
                    n_keys: 0,};
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
    arg.princ = princ;
    arg.keyblocks = keyblocks;
    arg.n_keys = n_keys;
    arg.api_version = (*handle).api_version;
    arg.keepold = keepold;
    arg.n_ks_tuple = n_ks_tuple;
    arg.ks_tuple = ks_tuple;
    if princ.is_null() || keyblocks.is_null() {
        return 22 as libc::c_int as kadm5_ret_t
    }
    if setkey_principal3_2(&mut arg, &mut r, (*handle).clnt) as u64 != 0 {
        return 43787528 as libc::c_long
    }
    return r.code;
}
#[no_mangle]
#[c2rust::src_loc = "329:1"]
pub unsafe extern "C" fn kadm5_setkey_principal_4(mut server_handle:
                                                      *mut libc::c_void,
                                                  mut princ: krb5_principal,
                                                  mut keepold: krb5_boolean,
                                                  mut key_data:
                                                      *mut kadm5_key_data,
                                                  mut n_key_data: libc::c_int)
 -> kadm5_ret_t {
    let mut arg: setkey4_arg =
        setkey4_arg{api_version: 0,
                    princ: 0 as *mut krb5_principal_data,
                    keepold: 0,
                    key_data: 0 as *mut kadm5_key_data,
                    n_key_data: 0,};
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
    arg.api_version = (*handle).api_version;
    arg.princ = princ;
    arg.keepold = keepold;
    arg.key_data = key_data;
    arg.n_key_data = n_key_data;
    if princ.is_null() || key_data.is_null() || n_key_data == 0 as libc::c_int
       {
        return 22 as libc::c_int as kadm5_ret_t
    }
    if setkey_principal4_2(&mut arg, &mut r, (*handle).clnt) as u64 != 0 {
        return 43787528 as libc::c_long
    }
    return r.code;
}
#[no_mangle]
#[c2rust::src_loc = "355:1"]
pub unsafe extern "C" fn kadm5_randkey_principal_3(mut server_handle:
                                                       *mut libc::c_void,
                                                   mut princ: krb5_principal,
                                                   mut keepold: krb5_boolean,
                                                   mut n_ks_tuple:
                                                       libc::c_int,
                                                   mut ks_tuple:
                                                       *mut krb5_key_salt_tuple,
                                                   mut key:
                                                       *mut *mut krb5_keyblock,
                                                   mut n_keys:
                                                       *mut libc::c_int)
 -> kadm5_ret_t {
    let mut arg: chrand3_arg =
        chrand3_arg{api_version: 0,
                    princ: 0 as *mut krb5_principal_data,
                    keepold: 0,
                    n_ks_tuple: 0,
                    ks_tuple: 0 as *mut krb5_key_salt_tuple,};
    let mut r: chrand_ret =
        chrand_ret{api_version: 0,
                   code: 0,
                   key:
                       krb5_keyblock{magic: 0,
                                     enctype: 0,
                                     length: 0,
                                     contents: 0 as *mut krb5_octet,},
                   keys: 0 as *mut krb5_keyblock,
                   n_keys: 0,};
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    let mut i: libc::c_int = 0;
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
    arg.princ = princ;
    arg.api_version = (*handle).api_version;
    arg.keepold = keepold;
    arg.n_ks_tuple = n_ks_tuple;
    arg.ks_tuple = ks_tuple;
    if princ.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    memset(&mut r as *mut chrand_ret as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<chrand_ret>() as libc::c_ulong);
    if chrand_principal3_2(&mut arg, &mut r, (*handle).clnt) as u64 != 0 {
        return 43787528 as libc::c_long
    }
    if !n_keys.is_null() { *n_keys = r.n_keys }
    if !key.is_null() {
        *key = r.keys
    } else {
        i = 0 as libc::c_int;
        while i < r.n_keys {
            krb5_free_keyblock_contents((*handle).context,
                                        &mut *r.keys.offset(i as isize));
            i += 1
        }
        free(r.keys as *mut libc::c_void);
    }
    return r.code;
}
#[no_mangle]
#[c2rust::src_loc = "392:1"]
pub unsafe extern "C" fn kadm5_randkey_principal(mut server_handle:
                                                     *mut libc::c_void,
                                                 mut princ: krb5_principal,
                                                 mut key:
                                                     *mut *mut krb5_keyblock,
                                                 mut n_keys: *mut libc::c_int)
 -> kadm5_ret_t {
    let mut arg: chrand_arg =
        chrand_arg{api_version: 0, princ: 0 as *mut krb5_principal_data,};
    let mut r: chrand_ret =
        chrand_ret{api_version: 0,
                   code: 0,
                   key:
                       krb5_keyblock{magic: 0,
                                     enctype: 0,
                                     length: 0,
                                     contents: 0 as *mut krb5_octet,},
                   keys: 0 as *mut krb5_keyblock,
                   n_keys: 0,};
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    let mut i: libc::c_int = 0;
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
    arg.princ = princ;
    arg.api_version = (*handle).api_version;
    if princ.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    memset(&mut r as *mut chrand_ret as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<chrand_ret>() as libc::c_ulong);
    if chrand_principal_2(&mut arg, &mut r, (*handle).clnt) as u64 != 0 {
        return 43787528 as libc::c_long
    }
    if !n_keys.is_null() { *n_keys = r.n_keys }
    if !key.is_null() {
        *key = r.keys
    } else {
        i = 0 as libc::c_int;
        while i < r.n_keys {
            krb5_free_keyblock_contents((*handle).context,
                                        &mut *r.keys.offset(i as isize));
            i += 1
        }
        free(r.keys as *mut libc::c_void);
    }
    return r.code;
}
/* not supported on client side */
#[no_mangle]
#[c2rust::src_loc = "425:1"]
pub unsafe extern "C" fn kadm5_decrypt_key(mut server_handle:
                                               *mut libc::c_void,
                                           mut entry: kadm5_principal_ent_t,
                                           mut ktype: krb5_int32,
                                           mut stype: krb5_int32,
                                           mut kvno: krb5_int32,
                                           mut keyblock: *mut krb5_keyblock,
                                           mut keysalt: *mut krb5_keysalt,
                                           mut kvnop: *mut libc::c_int)
 -> kadm5_ret_t {
    return 22 as libc::c_int as kadm5_ret_t;
}
#[no_mangle]
#[c2rust::src_loc = "434:1"]
pub unsafe extern "C" fn kadm5_purgekeys(mut server_handle: *mut libc::c_void,
                                         mut princ: krb5_principal,
                                         mut keepkvno: libc::c_int)
 -> kadm5_ret_t {
    let mut arg: purgekeys_arg =
        purgekeys_arg{api_version: 0,
                      princ: 0 as *mut krb5_principal_data,
                      keepkvno: 0,};
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
    arg.princ = princ;
    arg.keepkvno = keepkvno;
    arg.api_version = (*handle).api_version;
    if princ.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    if purgekeys_2(&mut arg, &mut r, (*handle).clnt) as u64 != 0 {
        return 43787528 as libc::c_long
    }
    return r.code;
}
#[no_mangle]
#[c2rust::src_loc = "456:1"]
pub unsafe extern "C" fn kadm5_get_strings(mut server_handle:
                                               *mut libc::c_void,
                                           mut principal: krb5_principal,
                                           mut strings_out:
                                               *mut *mut krb5_string_attr,
                                           mut count_out: *mut libc::c_int)
 -> kadm5_ret_t {
    let mut arg: gstrings_arg =
        gstrings_arg{api_version: 0, princ: 0 as *mut krb5_principal_data,};
    let mut r: gstrings_ret =
        gstrings_ret{api_version: 0,
                     code: 0,
                     strings: 0 as *mut krb5_string_attr,
                     count: 0,};
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    *strings_out = 0 as *mut krb5_string_attr;
    *count_out = 0 as libc::c_int;
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
    if principal.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    arg.princ = principal;
    arg.api_version = (*handle).api_version;
    memset(&mut r as *mut gstrings_ret as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<gstrings_ret>() as libc::c_ulong);
    if get_strings_2(&mut arg, &mut r, (*handle).clnt) as u64 != 0 {
        return 43787528 as libc::c_long
    }
    if r.code == 0 as libc::c_int as libc::c_long {
        *strings_out = r.strings;
        *count_out = r.count
    }
    return r.code;
}
#[no_mangle]
#[c2rust::src_loc = "482:1"]
pub unsafe extern "C" fn kadm5_set_string(mut server_handle:
                                              *mut libc::c_void,
                                          mut principal: krb5_principal,
                                          mut key: *const libc::c_char,
                                          mut value: *const libc::c_char)
 -> kadm5_ret_t {
    let mut arg: sstring_arg =
        sstring_arg{api_version: 0,
                    princ: 0 as *mut krb5_principal_data,
                    key: 0 as *mut libc::c_char,
                    value: 0 as *mut libc::c_char,};
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
    if principal.is_null() || key.is_null() {
        return 22 as libc::c_int as kadm5_ret_t
    }
    arg.princ = principal;
    arg.key = key as *mut libc::c_char;
    arg.value = value as *mut libc::c_char;
    arg.api_version = (*handle).api_version;
    if set_string_2(&mut arg, &mut r, (*handle).clnt) as u64 != 0 {
        return 43787528 as libc::c_long
    }
    return r.code;
}
#[no_mangle]
#[c2rust::src_loc = "503:1"]
pub unsafe extern "C" fn kadm5_get_principal_keys(mut server_handle:
                                                      *mut libc::c_void,
                                                  mut princ: krb5_principal,
                                                  mut kvno: krb5_kvno,
                                                  mut key_data:
                                                      *mut *mut kadm5_key_data,
                                                  mut n_key_data:
                                                      *mut libc::c_int)
 -> kadm5_ret_t {
    let mut arg: getpkeys_arg =
        getpkeys_arg{api_version: 0,
                     princ: 0 as *mut krb5_principal_data,
                     kvno: 0,};
    let mut r: getpkeys_ret =
        getpkeys_ret{api_version: 0,
                     code: 0,
                     key_data: 0 as *mut kadm5_key_data,
                     n_key_data: 0,};
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
    arg.api_version = (*handle).api_version;
    arg.princ = princ;
    arg.kvno = kvno;
    if princ.is_null() || key_data.is_null() || n_key_data.is_null() {
        return 22 as libc::c_int as kadm5_ret_t
    }
    memset(&mut r as *mut getpkeys_ret as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<getpkeys_ret>() as libc::c_ulong);
    if get_principal_keys_2(&mut arg, &mut r, (*handle).clnt) as u64 != 0 {
        return 43787528 as libc::c_long
    }
    if r.code == 0 as libc::c_int as libc::c_long {
        *key_data = r.key_data;
        *n_key_data = r.n_key_data
    }
    return r.code;
}
