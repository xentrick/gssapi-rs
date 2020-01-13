use ::libc;
use ::c2rust_asm_casts;
#[c2rust::header_src = "/usr/include/bits/types.h:48"]
pub mod types_h {
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
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
}
#[c2rust::header_src = "/usr/include/sys/types.h:48"]
pub mod sys_types_h {
    #[c2rust::src_loc = "34:1"]
    pub type u_short = __u_short;
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_short, __u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:48"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/sys/select.h:48"]
pub mod select_h {
    #[c2rust::src_loc = "49:1"]
    pub type __fd_mask = libc::c_long;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "59:9"]
    pub struct fd_set {
        pub fds_bits: [__fd_mask; 16],
    }
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:50"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:50"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/netinet/in.h:50"]
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
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:50"]
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
    #[c2rust::src_loc = "90:1"]
    pub type rpcprot_t = uint32_t;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth.h:50"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc_msg.h:50"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/svc.h:50"]
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
    use super::sys_types_h::u_short;
    use super::in_h::sockaddr_in;
    use super::svc_auth_h::SVCAUTH;
    use super::rpc_msg_h::rpc_msg;
    use super::xdr_h::xdrproc_t;
    use super::select_h::{fd_set, __fd_mask};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "284:15"]
        pub static mut gssrpc_svc_fdset: fd_set;
        #[no_mangle]
        #[c2rust::src_loc = "282:12"]
        pub static mut gssrpc_svc_maxfd: libc::c_int;
    }
    /* !defined(GSSRPC_SVC_H) */
    /* XXX add auth_gsapi_log_*? */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/svc_auth.h:50"]
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
    use super::svc_h::svc_req;
    use super::rpc_msg_h::rpc_msg;
    use super::auth_h::auth_stat;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "70:28"]
        pub static mut gssrpc_svc_auth_gss_ops: svc_auth_ops;
        #[no_mangle]
        #[c2rust::src_loc = "76:1"]
        pub fn gssrpc__authenticate(rqst: *mut svc_req, msg: *mut rpc_msg,
                                    no_dispatch: *mut libc::c_int)
         -> auth_stat;
    }
    /* !defined(GSSRPC_SVC_AUTH_H) */
}
#[c2rust::header_src = "/usr/include/stdlib.h:50"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/pmap_clnt.h:51"]
pub mod pmap_clnt_h {
    use super::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcprot_t};
    use super::sys_types_h::u_int;
    extern "C" {
        /* @(#)pmap_clnt.h	2.1 88/07/29 4.0 RPCSRC; from 1.11 88/02/08 SMI */
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
 * pmap_clnt.h
 * Supplies C routines to get to portmap services.
 */
        /*
 * Usage:
 *	success = pmap_set(program, version, protocol, port);
 *	success = pmap_unset(program, version);
 *	port = pmap_getport(address, program, version, protocol);
 *	head = pmap_getmaps(address);
 *	clnt_stat = pmap_rmtcall(address, program, version, procedure,
 *		xdrargs, argsp, xdrres, resp, tout, port_ptr)
 *		(works for udp only.)
 * 	clnt_stat = clnt_broadcast(program, version, procedure,
 *		xdrargs, argsp,	xdrres, resp, eachresult)
 *		(like pmap_rmtcall, except the call is broadcasted to all
 *		locally connected nets.  For each valid response received,
 *		the procedure eachresult is called.  Its form is:
 *	done = eachresult(resp, raddr)
 *		bool_t done;
 *		caddr_t resp;
 *		struct sockaddr_in raddr;
 *		where resp points to the results of the call and raddr is the
 *		address if the responder to the broadcast.
 */
        #[no_mangle]
        #[c2rust::src_loc = "66:1"]
        pub fn gssrpc_pmap_set(_: rpcprog_t, _: rpcvers_t, _: rpcprot_t,
                               _: u_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn gssrpc_pmap_unset(_: rpcprog_t, _: rpcvers_t) -> libc::c_int;
    }
    /* !defined(GSSRPC_PMAP_CLNT_H) */
}
#[c2rust::header_src = "/usr/include/string.h:53"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
use c2rust_asm_casts::AsmCastTrait;
pub use self::types_h::{__u_short, __u_int, __uint16_t, __int32_t, __uint32_t,
                        __caddr_t};
pub use self::sys_types_h::{u_short, u_int, caddr_t};
pub use self::stdint_intn_h::int32_t;
pub use self::select_h::{__fd_mask, fd_set};
pub use self::stdint_uintn_h::{uint16_t, uint32_t};
pub use self::sockaddr_h::sa_family_t;
pub use self::in_h::{in_port_t, sockaddr_in, in_addr, in_addr_t};
pub use self::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcprot_t, rpcproc_t,
                               rpc_inline_t};
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops};
pub use self::auth_h::{auth_stat, RPCSEC_GSS_CTXPROBLEM,
                       RPCSEC_GSS_CREDPROBLEM, AUTH_FAILED, AUTH_INVALIDRESP,
                       AUTH_TOOWEAK, AUTH_REJECTEDVERF, AUTH_BADVERF,
                       AUTH_REJECTEDCRED, AUTH_BADCRED, AUTH_OK, opaque_auth};
pub use self::rpc_msg_h::{rpc_msg, C2RustUnnamed, reply_body, C2RustUnnamed_0,
                          rejected_reply, C2RustUnnamed_1, C2RustUnnamed_2,
                          reject_stat, AUTH_ERROR, RPC_MISMATCH,
                          accepted_reply, C2RustUnnamed_3, C2RustUnnamed_4,
                          C2RustUnnamed_5, accept_stat, SYSTEM_ERR,
                          GARBAGE_ARGS, PROC_UNAVAIL, PROG_MISMATCH,
                          PROG_UNAVAIL, SUCCESS, reply_stat, MSG_DENIED,
                          MSG_ACCEPTED, call_body, msg_type, REPLY, CALL};
pub use self::svc_h::{svc_req, SVCXPRT, xp_ops, xprt_stat, XPRT_IDLE,
                      XPRT_MOREREQS, XPRT_DIED, gssrpc_svc_fdset,
                      gssrpc_svc_maxfd};
pub use self::svc_auth_h::{SVCAUTH, svc_auth_ops, gssrpc_svc_auth_gss_ops,
                           gssrpc__authenticate};
use self::stdlib_h::{malloc, free};
use self::pmap_clnt_h::{gssrpc_pmap_set, gssrpc_pmap_unset};
use self::string_h::memset;
extern "C" {
    #[no_mangle]
    #[c2rust::src_loc = "58:12"]
    pub static mut gssrpc_svc_fdset_init: libc::c_int;
}
/* this size is excessive */
/*
 * The services list
 * Each entry represents a set of procedures (an rpc program).
 * The dispatch routine takes request structs and runs the
 * apropriate procedure.
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "79:15"]
pub struct svc_callout {
    pub sc_next: *mut svc_callout,
    pub sc_prog: rpcprog_t,
    pub sc_vers: rpcprog_t,
    pub sc_dispatch: Option<unsafe extern "C" fn() -> ()>,
}
/* @(#)svc.c	2.4 88/08/11 4.0 RPCSRC; from 1.44 88/02/08 SMI */
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
 * svc.c, Server-side remote procedure call interface.
 *
 * There are two sets of procedures here.  The xprt routines are
 * for handling transport handles.  The svc routines handle the
 * list of service routines.
 */
#[c2rust::src_loc = "57:18"]
static mut xports: *mut *mut SVCXPRT =
    0 as *const *mut SVCXPRT as *mut *mut SVCXPRT;
#[c2rust::src_loc = "84:4"]
static mut svc_head: *mut svc_callout =
    0 as *const svc_callout as *mut svc_callout;
/* ***************  SVCXPRT related stuff **************** */
/*
 * Activate a transport handle.
 */
#[no_mangle]
#[c2rust::src_loc = "96:1"]
pub unsafe extern "C" fn gssrpc_xprt_register(mut xprt: *mut SVCXPRT) {
    let mut sock: libc::c_int = (*xprt).xp_sock;
    if gssrpc_svc_fdset_init == 0 as libc::c_int {
        let mut __d0: libc::c_int = 0;
        let mut __d1: libc::c_int = 0;
        let fresh0 = &mut __d0;
        let fresh1;
        let fresh2 = &mut __d1;
        let fresh3;
        let fresh4 =
            (::std::mem::size_of::<fd_set>() as
                 libc::c_ulong).wrapping_div(::std::mem::size_of::<__fd_mask>()
                                                 as libc::c_ulong);
        let fresh5 =
            &mut *gssrpc_svc_fdset.fds_bits.as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as isize)
                as *mut __fd_mask;
        asm!("cld; rep; stosq" : "={cx}" (fresh1), "={di}" (fresh3) : "{ax}"
             (0 as libc::c_int), "0"
             (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh4)), "1"
             (c2rust_asm_casts::AsmCast::cast_in(fresh2, fresh5)) : "memory" :
             "volatile");
        c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh4, fresh1);
        c2rust_asm_casts::AsmCast::cast_out(fresh2, fresh5, fresh3);
        gssrpc_svc_fdset_init += 1
    }
    if xports.is_null() {
        xports =
            malloc((1024 as libc::c_int as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut SVCXPRT>()
                                                        as libc::c_ulong)) as
                *mut *mut SVCXPRT;
        memset(xports as *mut libc::c_void, 0 as libc::c_int,
               (1024 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut SVCXPRT>()
                                                    as libc::c_ulong));
    }
    if sock < 1024 as libc::c_int {
        let ref mut fresh6 = *xports.offset(sock as isize);
        *fresh6 = xprt;
        gssrpc_svc_fdset.fds_bits[(sock /
                                       (8 as libc::c_int *
                                            ::std::mem::size_of::<__fd_mask>()
                                                as libc::c_ulong as
                                                libc::c_int)) as usize] |=
            ((1 as libc::c_ulong) <<
                 sock %
                     (8 as libc::c_int *
                          ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                              as libc::c_int)) as __fd_mask;
        if sock > gssrpc_svc_maxfd { gssrpc_svc_maxfd = sock }
    };
    /* def FD_SETSIZE */
}
/*
 * De-activate a transport handle.
 */
#[no_mangle]
#[c2rust::src_loc = "130:1"]
pub unsafe extern "C" fn gssrpc_xprt_unregister(mut xprt: *mut SVCXPRT) {
    let mut sock: libc::c_int = (*xprt).xp_sock;
    if sock < 1024 as libc::c_int && *xports.offset(sock as isize) == xprt {
        let ref mut fresh7 = *xports.offset(sock as isize);
        *fresh7 = 0 as *mut SVCXPRT;
        gssrpc_svc_fdset.fds_bits[(sock /
                                       (8 as libc::c_int *
                                            ::std::mem::size_of::<__fd_mask>()
                                                as libc::c_ulong as
                                                libc::c_int)) as usize] &=
            !(((1 as libc::c_ulong) <<
                   sock %
                       (8 as libc::c_int *
                            ::std::mem::size_of::<__fd_mask>() as
                                libc::c_ulong as libc::c_int)) as __fd_mask)
    }
    /* def FD_SETSIZE */
    if gssrpc_svc_maxfd <= sock {
        while gssrpc_svc_maxfd > 0 as libc::c_int &&
                  (*xports.offset(gssrpc_svc_maxfd as isize)).is_null() {
            gssrpc_svc_maxfd -= 1
        }
    };
}
/* ********************** CALLOUT list related stuff ************* */
/*
 * Add a service program to the callout list.
 * The dispatch routine will be called when a rpc request for this
 * program number comes in.
 */
#[no_mangle]
#[c2rust::src_loc = "160:1"]
pub unsafe extern "C" fn gssrpc_svc_register(mut xprt: *mut SVCXPRT,
                                             mut prog: rpcprog_t,
                                             mut vers: rpcvers_t,
                                             mut dispatch:
                                                 Option<unsafe extern "C" fn()
                                                            -> ()>,
                                             mut protocol: libc::c_int)
 -> libc::c_int {
    let mut prev: *mut svc_callout =
        0 as *mut svc_callout; /* he is registering another xptr */
    let mut s: *mut svc_callout = 0 as *mut svc_callout;
    s = svc_find(prog, vers, &mut prev);
    if !s.is_null() {
        if !((*s).sc_dispatch == dispatch) { return 0 as libc::c_int }
    } else {
        s =
            malloc(::std::mem::size_of::<svc_callout>() as libc::c_ulong) as
                *mut svc_callout;
        if s.is_null() { return 0 as libc::c_int }
        (*s).sc_prog = prog;
        (*s).sc_vers = vers;
        (*s).sc_dispatch = dispatch;
        (*s).sc_next = svc_head;
        svc_head = s
    }
    /* now register the information with the local binder service */
    if protocol != 0 {
        return gssrpc_pmap_set(prog, vers, protocol as rpcprot_t,
                               (*xprt).xp_port as u_int)
    }
    return 1 as libc::c_int;
}
/*
 * Remove a service program from the callout list.
 */
#[no_mangle]
#[c2rust::src_loc = "196:1"]
pub unsafe extern "C" fn gssrpc_svc_unregister(mut prog: rpcprog_t,
                                               mut vers: rpcvers_t) {
    let mut prev: *mut svc_callout = 0 as *mut svc_callout;
    let mut s: *mut svc_callout = 0 as *mut svc_callout;
    s = svc_find(prog, vers, &mut prev);
    if s.is_null() { return }
    if prev.is_null() {
        svc_head = (*s).sc_next
    } else { (*prev).sc_next = (*s).sc_next }
    (*s).sc_next = 0 as *mut svc_callout;
    free(s as *mut libc::c_char as *mut libc::c_void);
    /* now unregister the information with the local binder service */
    gssrpc_pmap_unset(prog, vers);
}
/*
 * Search the callout list for a program number, return the callout
 * struct.
 */
#[c2rust::src_loc = "221:1"]
unsafe extern "C" fn svc_find(mut prog: rpcprog_t, mut vers: rpcvers_t,
                              mut prev: *mut *mut svc_callout)
 -> *mut svc_callout {
    let mut s: *mut svc_callout = 0 as *mut svc_callout;
    let mut p: *mut svc_callout = 0 as *mut svc_callout;
    p = 0 as *mut svc_callout;
    s = svc_head;
    while !s.is_null() {
        if (*s).sc_prog == prog && (*s).sc_vers == vers { break ; }
        p = s;
        s = (*s).sc_next
    }
    *prev = p;
    return s;
}
/* ******************* REPLY GENERATION ROUTINES  ************ */
/*
 * Send a reply to an rpc request
 */
#[no_mangle]
#[c2rust::src_loc = "245:1"]
pub unsafe extern "C" fn gssrpc_svc_sendreply(mut xprt: *mut SVCXPRT,
                                              mut xdr_results: xdrproc_t,
                                              mut xdr_location: caddr_t)
 -> libc::c_int {
    let mut rply: rpc_msg =
        rpc_msg{rm_xid: 0,
                rm_direction: CALL,
                ru:
                    C2RustUnnamed{RM_cmb:
                                      call_body{cb_rpcvers: 0,
                                                cb_prog: 0,
                                                cb_vers: 0,
                                                cb_proc: 0,
                                                cb_cred:
                                                    opaque_auth{oa_flavor: 0,
                                                                oa_base:
                                                                    0 as
                                                                        *mut libc::c_char,
                                                                oa_length:
                                                                    0,},
                                                cb_verf:
                                                    opaque_auth{oa_flavor: 0,
                                                                oa_base:
                                                                    0 as
                                                                        *mut libc::c_char,
                                                                oa_length:
                                                                    0,},},},};
    rply.rm_direction = REPLY;
    rply.ru.RM_rmb.rp_stat = MSG_ACCEPTED;
    rply.ru.RM_rmb.ru.RP_ar.ar_verf = (*xprt).xp_verf;
    rply.ru.RM_rmb.ru.RP_ar.ar_stat = SUCCESS;
    rply.ru.RM_rmb.ru.RP_ar.ru.AR_results.where_0 = xdr_location;
    rply.ru.RM_rmb.ru.RP_ar.ru.AR_results.proc_0 = xdr_results;
    return Some((*(*xprt).xp_ops).xp_reply.expect("non-null function pointer")).expect("non-null function pointer")(xprt,
                                                                                                                    &mut rply);
}
/*
 * No procedure error reply
 */
#[no_mangle]
#[c2rust::src_loc = "265:1"]
pub unsafe extern "C" fn gssrpc_svcerr_noproc(mut xprt: *mut SVCXPRT) {
    let mut rply: rpc_msg =
        rpc_msg{rm_xid: 0,
                rm_direction: CALL,
                ru:
                    C2RustUnnamed{RM_cmb:
                                      call_body{cb_rpcvers: 0,
                                                cb_prog: 0,
                                                cb_vers: 0,
                                                cb_proc: 0,
                                                cb_cred:
                                                    opaque_auth{oa_flavor: 0,
                                                                oa_base:
                                                                    0 as
                                                                        *mut libc::c_char,
                                                                oa_length:
                                                                    0,},
                                                cb_verf:
                                                    opaque_auth{oa_flavor: 0,
                                                                oa_base:
                                                                    0 as
                                                                        *mut libc::c_char,
                                                                oa_length:
                                                                    0,},},},};
    rply.rm_direction = REPLY;
    rply.ru.RM_rmb.rp_stat = MSG_ACCEPTED;
    rply.ru.RM_rmb.ru.RP_ar.ar_verf = (*xprt).xp_verf;
    rply.ru.RM_rmb.ru.RP_ar.ar_stat = PROC_UNAVAIL;
    Some((*(*xprt).xp_ops).xp_reply.expect("non-null function pointer")).expect("non-null function pointer")(xprt,
                                                                                                             &mut rply);
}
/*
 * Can't decode args error reply
 */
#[no_mangle]
#[c2rust::src_loc = "280:1"]
pub unsafe extern "C" fn gssrpc_svcerr_decode(mut xprt: *mut SVCXPRT) {
    let mut rply: rpc_msg =
        rpc_msg{rm_xid: 0,
                rm_direction: CALL,
                ru:
                    C2RustUnnamed{RM_cmb:
                                      call_body{cb_rpcvers: 0,
                                                cb_prog: 0,
                                                cb_vers: 0,
                                                cb_proc: 0,
                                                cb_cred:
                                                    opaque_auth{oa_flavor: 0,
                                                                oa_base:
                                                                    0 as
                                                                        *mut libc::c_char,
                                                                oa_length:
                                                                    0,},
                                                cb_verf:
                                                    opaque_auth{oa_flavor: 0,
                                                                oa_base:
                                                                    0 as
                                                                        *mut libc::c_char,
                                                                oa_length:
                                                                    0,},},},};
    rply.rm_direction = REPLY;
    rply.ru.RM_rmb.rp_stat = MSG_ACCEPTED;
    rply.ru.RM_rmb.ru.RP_ar.ar_verf = (*xprt).xp_verf;
    rply.ru.RM_rmb.ru.RP_ar.ar_stat = GARBAGE_ARGS;
    Some((*(*xprt).xp_ops).xp_reply.expect("non-null function pointer")).expect("non-null function pointer")(xprt,
                                                                                                             &mut rply);
}
/*
 * Some system error
 */
#[no_mangle]
#[c2rust::src_loc = "295:1"]
pub unsafe extern "C" fn gssrpc_svcerr_systemerr(mut xprt: *mut SVCXPRT) {
    let mut rply: rpc_msg =
        rpc_msg{rm_xid: 0,
                rm_direction: CALL,
                ru:
                    C2RustUnnamed{RM_cmb:
                                      call_body{cb_rpcvers: 0,
                                                cb_prog: 0,
                                                cb_vers: 0,
                                                cb_proc: 0,
                                                cb_cred:
                                                    opaque_auth{oa_flavor: 0,
                                                                oa_base:
                                                                    0 as
                                                                        *mut libc::c_char,
                                                                oa_length:
                                                                    0,},
                                                cb_verf:
                                                    opaque_auth{oa_flavor: 0,
                                                                oa_base:
                                                                    0 as
                                                                        *mut libc::c_char,
                                                                oa_length:
                                                                    0,},},},};
    rply.rm_direction = REPLY;
    rply.ru.RM_rmb.rp_stat = MSG_ACCEPTED;
    rply.ru.RM_rmb.ru.RP_ar.ar_verf = (*xprt).xp_verf;
    rply.ru.RM_rmb.ru.RP_ar.ar_stat = SYSTEM_ERR;
    Some((*(*xprt).xp_ops).xp_reply.expect("non-null function pointer")).expect("non-null function pointer")(xprt,
                                                                                                             &mut rply);
}
/*
 * Authentication error reply
 */
#[no_mangle]
#[c2rust::src_loc = "310:1"]
pub unsafe extern "C" fn gssrpc_svcerr_auth(mut xprt: *mut SVCXPRT,
                                            mut why: auth_stat) {
    let mut rply: rpc_msg =
        rpc_msg{rm_xid: 0,
                rm_direction: CALL,
                ru:
                    C2RustUnnamed{RM_cmb:
                                      call_body{cb_rpcvers: 0,
                                                cb_prog: 0,
                                                cb_vers: 0,
                                                cb_proc: 0,
                                                cb_cred:
                                                    opaque_auth{oa_flavor: 0,
                                                                oa_base:
                                                                    0 as
                                                                        *mut libc::c_char,
                                                                oa_length:
                                                                    0,},
                                                cb_verf:
                                                    opaque_auth{oa_flavor: 0,
                                                                oa_base:
                                                                    0 as
                                                                        *mut libc::c_char,
                                                                oa_length:
                                                                    0,},},},};
    rply.rm_direction = REPLY;
    rply.ru.RM_rmb.rp_stat = MSG_DENIED;
    rply.ru.RM_rmb.ru.RP_dr.rj_stat = AUTH_ERROR;
    rply.ru.RM_rmb.ru.RP_dr.ru.RJ_why = why;
    Some((*(*xprt).xp_ops).xp_reply.expect("non-null function pointer")).expect("non-null function pointer")(xprt,
                                                                                                             &mut rply);
}
/*
 * Auth too weak error reply
 */
#[no_mangle]
#[c2rust::src_loc = "327:1"]
pub unsafe extern "C" fn gssrpc_svcerr_weakauth(mut xprt: *mut SVCXPRT) {
    gssrpc_svcerr_auth(xprt, AUTH_TOOWEAK);
}
/*
 * Program unavailable error reply
 */
#[no_mangle]
#[c2rust::src_loc = "337:1"]
pub unsafe extern "C" fn gssrpc_svcerr_noprog(mut xprt: *mut SVCXPRT) {
    let mut rply: rpc_msg =
        rpc_msg{rm_xid: 0,
                rm_direction: CALL,
                ru:
                    C2RustUnnamed{RM_cmb:
                                      call_body{cb_rpcvers: 0,
                                                cb_prog: 0,
                                                cb_vers: 0,
                                                cb_proc: 0,
                                                cb_cred:
                                                    opaque_auth{oa_flavor: 0,
                                                                oa_base:
                                                                    0 as
                                                                        *mut libc::c_char,
                                                                oa_length:
                                                                    0,},
                                                cb_verf:
                                                    opaque_auth{oa_flavor: 0,
                                                                oa_base:
                                                                    0 as
                                                                        *mut libc::c_char,
                                                                oa_length:
                                                                    0,},},},};
    rply.rm_direction = REPLY;
    rply.ru.RM_rmb.rp_stat = MSG_ACCEPTED;
    rply.ru.RM_rmb.ru.RP_ar.ar_verf = (*xprt).xp_verf;
    rply.ru.RM_rmb.ru.RP_ar.ar_stat = PROG_UNAVAIL;
    Some((*(*xprt).xp_ops).xp_reply.expect("non-null function pointer")).expect("non-null function pointer")(xprt,
                                                                                                             &mut rply);
}
/*
 * Program version mismatch error reply
 */
#[no_mangle]
#[c2rust::src_loc = "352:1"]
pub unsafe extern "C" fn gssrpc_svcerr_progvers(mut xprt: *mut SVCXPRT,
                                                mut low_vers: rpcvers_t,
                                                mut high_vers: rpcvers_t) {
    let mut rply: rpc_msg =
        rpc_msg{rm_xid: 0,
                rm_direction: CALL,
                ru:
                    C2RustUnnamed{RM_cmb:
                                      call_body{cb_rpcvers: 0,
                                                cb_prog: 0,
                                                cb_vers: 0,
                                                cb_proc: 0,
                                                cb_cred:
                                                    opaque_auth{oa_flavor: 0,
                                                                oa_base:
                                                                    0 as
                                                                        *mut libc::c_char,
                                                                oa_length:
                                                                    0,},
                                                cb_verf:
                                                    opaque_auth{oa_flavor: 0,
                                                                oa_base:
                                                                    0 as
                                                                        *mut libc::c_char,
                                                                oa_length:
                                                                    0,},},},};
    rply.rm_direction = REPLY;
    rply.ru.RM_rmb.rp_stat = MSG_ACCEPTED;
    rply.ru.RM_rmb.ru.RP_ar.ar_verf = (*xprt).xp_verf;
    rply.ru.RM_rmb.ru.RP_ar.ar_stat = PROG_MISMATCH;
    rply.ru.RM_rmb.ru.RP_ar.ru.AR_versions.low = low_vers;
    rply.ru.RM_rmb.ru.RP_ar.ru.AR_versions.high = high_vers;
    Some((*(*xprt).xp_ops).xp_reply.expect("non-null function pointer")).expect("non-null function pointer")(xprt,
                                                                                                             &mut rply);
}
/* ******************* SERVER INPUT STUFF ******************* */
/*
 * Get server side input from some transport.
 *
 * Statement of authentication parameters management:
 * This function owns and manages all authentication parameters, specifically
 * the "raw" parameters (msg.rm_call.cb_cred and msg.rm_call.cb_verf) and
 * the "cooked" credentials (rqst->rq_clntcred).
 * However, this function does not know the structure of the cooked
 * credentials, so it make the following assumptions:
 *   a) the structure is contiguous (no pointers), and
 *   b) the cred structure size does not exceed RQCRED_SIZE bytes.
 * In all events, all three parameters are freed upon exit from this routine.
 * The storage is trivially management on the call stack in user land, but
 * is mallocated in kernel land.
 */
#[no_mangle]
#[c2rust::src_loc = "387:1"]
pub unsafe extern "C" fn gssrpc_svc_getreq(mut rdfds: libc::c_int) {
    let mut readfds: fd_set = fd_set{fds_bits: [0; 16],};
    let mut i: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh8 = &mut __d0;
    let fresh9;
    let fresh10 = &mut __d1;
    let fresh11;
    let fresh12 =
        (::std::mem::size_of::<fd_set>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<__fd_mask>() as
                                             libc::c_ulong);
    let fresh13 =
        &mut *readfds.fds_bits.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut __fd_mask;
    asm!("cld; rep; stosq" : "={cx}" (fresh9), "={di}" (fresh11) : "{ax}"
         (0 as libc::c_int), "0"
         (c2rust_asm_casts::AsmCast::cast_in(fresh8, fresh12)), "1"
         (c2rust_asm_casts::AsmCast::cast_in(fresh10, fresh13)) : "memory" :
         "volatile");
    c2rust_asm_casts::AsmCast::cast_out(fresh8, fresh12, fresh9);
    c2rust_asm_casts::AsmCast::cast_out(fresh10, fresh13, fresh11);
    i = 0 as libc::c_int;
    mask = 1 as libc::c_int;
    while rdfds != 0 {
        if rdfds & mask != 0 {
            readfds.fds_bits[(i /
                                  (8 as libc::c_int *
                                       ::std::mem::size_of::<__fd_mask>() as
                                           libc::c_ulong as libc::c_int)) as
                                 usize] |=
                ((1 as libc::c_ulong) <<
                     i %
                         (8 as libc::c_int *
                              ::std::mem::size_of::<__fd_mask>() as
                                  libc::c_ulong as libc::c_int)) as __fd_mask
        }
        rdfds &= !mask;
        i += 1;
        mask <<= 1 as libc::c_int
    }
    gssrpc_svc_getreqset(&mut readfds);
    /* def FD_SETSIZE */
}
/* @(#)svc.h	2.2 88/07/29 4.0 RPCSRC; from 1.20 88/02/08 SMI */
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
 * svc.h, Server-side remote procedure call interface.
 */
/*
 * This interface must manage two items concerning remote procedure calling:
 *
 * 1) An arbitrary number of transport connections upon which rpc requests
 * are received.  The two most notable transports are TCP and UDP;  they are
 * created and registered by routines in svc_tcp.c and svc_udp.c, respectively;
 * they in turn call xprt_register and xprt_unregister.
 *
 * 2) An arbitrary number of locally registered services.  Services are
 * described by the following four data: program number, version number,
 * "service dispatch" function, a transport handle, and a boolean that
 * indicates whether or not the exported program should be registered with a
 * local binder service;  if true the program's number and version and the
 * port number from the transport handle are registered with the binder.
 * These data are registered with the rpc svc system via svc_register.
 *
 * A service's dispatch function is called whenever an rpc request comes in
 * on a transport.  The request's program and version numbers must match
 * those of the registered service.  The dispatch function is passed two
 * parameters, struct svc_req * and SVCXPRT *, defined below.
 */
/*
 * Server side transport handle
 */
/* associated port number */
/* receive incomming requests */
/* get transport status */
/* get arguments */
/* send reply */
/* free mem allocated for args */
/* destroy this struct */
/* length of remote address */
/* remote address */
/* raw response verifier */
/* auth flavor of current req */
/* private */
/* private */
/* lenght of local address */
/* local address */
/*
 *  Approved way of getting address of caller
 */
/*
 * Operations defined on an SVCXPRT handle
 *
 * SVCXPRT		*xprt;
 * struct rpc_msg	*msg;
 * xdrproc_t		 xargs;
 * caddr_t		 argsp;
 */
/*
 * Service request
 */
/* service program number */
/* service protocol version */
/* the desired procedure */
/* raw creds from the wire */
/* read only cooked client cred */
/* read only svc cred/context */
/* read only client name */
/* associated transport */
/* The request's auth flavor *should* be here, but the svc_req 	*/
	/* isn't passed around everywhere it is necessary.  The 	*/
	/* transport *is* passed around, so the auth flavor it stored 	*/
	/* there.  This means that the transport must be single 	*/
	/* threaded, but other parts of SunRPC already require that. 	*/
	/*SVCAUTH		*rq_auth;	 associated auth flavor */
/*
 * Service registration
 *
 * svc_register(xprt, prog, vers, dispatch, protocol)
 *	SVCXPRT *xprt;
 *	rpcprog_t prog;
 *	rpcvers_t vers;
 *	void (*dispatch)();
 *	int protocol;  like IPPROTO_TCP or _UDP; zero means do not register
 *
 * registerrpc(prog, vers, proc, routine, inproc, outproc)
 * 	returns 0 upon success, -1 if error.
 */
/*
 * Service un-registration
 *
 * svc_unregister(prog, vers)
 *	rpcprog_t prog;
 *	rpcvers_t vers;
 */
/*
 * Transport registration.
 *
 * xprt_register(xprt)
 *	SVCXPRT *xprt;
 */
/*
 * Transport un-register
 *
 * xprt_unregister(xprt)
 *	SVCXPRT *xprt;
 */
/*
 * When the service routine is called, it must first check to see if
 * it knows about the procedure; if not, it should call svcerr_noproc
 * and return.  If so, it should deserialize its arguments via
 * SVC_GETARGS or the new SVC_GETARGS_REQ (both defined above).  If
 * the deserialization does not work, svcerr_decode should be called
 * followed by a return.  Successful decoding of the arguments should
 * be followed the execution of the procedure's code and a call to
 * svc_sendreply or the new svc_sendreply_req.
 *
 * Also, if the service refuses to execute the procedure due to too-
 * weak authentication parameters, svcerr_weakauth should be called.
 * Note: do not confuse access-control failure with weak authentication!
 *
 * NB: In pure implementations of rpc, the caller always waits for a reply
 * msg.  This message is sent when svc_sendreply is called.
 * Therefore pure service implementations should always call
 * svc_sendreply even if the function logically returns void;  use
 * xdr.h - xdr_void for the xdr routine.  HOWEVER, tcp based rpc allows
 * for the abuse of pure rpc via batched calling or pipelining.  In the
 * case of a batched call, svc_sendreply should NOT be called since
 * this would send a return message, which is what batching tries to avoid.
 * It is the service/protocol writer's responsibility to know which calls are
 * batched and which are not.  Warning: responding to batch calls may
 * deadlock the caller and server processes!
 */
/*
 * Lowest level dispatching -OR- who owns this process anyway.
 * Somebody has to wait for incoming requests and then call the correct
 * service routine.  The routine svc_run does infinite waiting; i.e.,
 * svc_run never returns.
 * Since another (co-existant) package may wish to selectively wait for
 * incoming calls or other events outside of the rpc architecture, the
 * routine svc_getreq is provided.  It must be passed readfds, the
 * "in-place" results of a select system call (see select, section 2).
 */
/*
 * Global keeper of rpc service descriptors in use
 * dynamic; must be inspected before each call to select
 */
/* RENAMED */
/* compatibility */
/* def FD_SETSIZE */
/*
 * a small program implemented by the svc_rpc implementation itself;
 * also see clnt.h for protocol numbers.
 */
#[no_mangle]
#[c2rust::src_loc = "414:1"]
pub unsafe extern "C" fn gssrpc_svc_getreqset(mut readfds: *mut fd_set) {
    let mut xprt: *mut SVCXPRT = 0 as *mut SVCXPRT;
    let mut sock: libc::c_int = 0;
    sock = 0 as libc::c_int;
    while sock <= gssrpc_svc_maxfd {
        if (*readfds).fds_bits[(sock /
                                    (8 as libc::c_int *
                                         ::std::mem::size_of::<__fd_mask>() as
                                             libc::c_ulong as libc::c_int)) as
                                   usize] &
               ((1 as libc::c_ulong) <<
                    sock %
                        (8 as libc::c_int *
                             ::std::mem::size_of::<__fd_mask>() as
                                 libc::c_ulong as libc::c_int)) as __fd_mask
               != 0 as libc::c_int as libc::c_long {
            /* sock has input waiting */
            xprt = *xports.offset(sock as isize);
            /* now receive msgs from xprtprt (support batch calls) */
            svc_do_xprt(xprt);
        }
        sock += 1
    };
}
#[c2rust::src_loc = "446:1"]
unsafe extern "C" fn svc_do_xprt(mut xprt: *mut SVCXPRT) {
    let mut current_block: u64;
    let mut rawcred: caddr_t = 0 as *mut libc::c_char;
    let mut rawverf: caddr_t = 0 as *mut libc::c_char;
    let mut cookedcred: caddr_t = 0 as *mut libc::c_char;
    let mut msg: rpc_msg =
        rpc_msg{rm_xid: 0,
                rm_direction: CALL,
                ru:
                    C2RustUnnamed{RM_cmb:
                                      call_body{cb_rpcvers: 0,
                                                cb_prog: 0,
                                                cb_vers: 0,
                                                cb_proc: 0,
                                                cb_cred:
                                                    opaque_auth{oa_flavor: 0,
                                                                oa_base:
                                                                    0 as
                                                                        *mut libc::c_char,
                                                                oa_length:
                                                                    0,},
                                                cb_verf:
                                                    opaque_auth{oa_flavor: 0,
                                                                oa_base:
                                                                    0 as
                                                                        *mut libc::c_char,
                                                                oa_length:
                                                                    0,},},},};
    let mut r: svc_req =
        svc_req{rq_prog: 0,
                rq_vers: 0,
                rq_proc: 0,
                rq_cred:
                    opaque_auth{oa_flavor: 0,
                                oa_base: 0 as *mut libc::c_char,
                                oa_length: 0,},
                rq_clntcred: 0 as *mut libc::c_void,
                rq_svccred: 0 as *mut libc::c_void,
                rq_clntname: 0 as *mut libc::c_void,
                rq_xprt: 0 as *mut SVCXPRT,};
    let mut no_dispatch: libc::c_int = 0;
    let mut prog_found: libc::c_int = 0;
    let mut low_vers: rpcvers_t = 0;
    let mut high_vers: rpcvers_t = 0;
    let mut stat: xprt_stat = XPRT_DIED;
    rawcred = malloc(400 as libc::c_int as libc::c_ulong) as caddr_t;
    rawverf = malloc(400 as libc::c_int as libc::c_ulong) as caddr_t;
    cookedcred = malloc(1024 as libc::c_int as libc::c_ulong) as caddr_t;
    if rawcred.is_null() || rawverf.is_null() || cookedcred.is_null() {
        return
    }
    msg.ru.RM_cmb.cb_cred.oa_base = rawcred;
    msg.ru.RM_cmb.cb_verf.oa_base = rawverf;
    r.rq_clntcred = cookedcred as *mut libc::c_void;
    loop  {
        let mut s: *mut svc_callout = 0 as *mut svc_callout;
        let mut why: auth_stat = AUTH_OK;
        if !(Some((*(*xprt).xp_ops).xp_recv.expect("non-null function pointer")).expect("non-null function pointer")(xprt,
                                                                                                                     &mut msg)
                 == 0) {
            /* now find the exported program and call it */
            r.rq_xprt = xprt;
            r.rq_prog = msg.ru.RM_cmb.cb_prog;
            r.rq_vers = msg.ru.RM_cmb.cb_vers;
            r.rq_proc = msg.ru.RM_cmb.cb_proc;
            r.rq_cred = msg.ru.RM_cmb.cb_cred;
            no_dispatch = 0 as libc::c_int;
            /* first authenticate the message */
            why = gssrpc__authenticate(&mut r, &mut msg, &mut no_dispatch);
            if why as libc::c_uint != AUTH_OK as libc::c_int as libc::c_uint {
                gssrpc_svcerr_auth(xprt, why);
            } else if !(no_dispatch != 0) {
                /* now match message with a registered service*/
                prog_found = 0 as libc::c_int; /* found correct version */
                low_vers = -(1 as libc::c_long) as rpcvers_t;
                high_vers = 0 as libc::c_int as rpcvers_t;
                s = svc_head;
                loop  {
                    if s.is_null() {
                        current_block = 5529461102203738653;
                        break ;
                    }
                    if (*s).sc_prog == r.rq_prog {
                        if (*s).sc_vers == r.rq_vers {
                            ::std::mem::transmute::<_,
                                                    fn(_: _,
                                                       _:
                                                           _)>(Some((*s).sc_dispatch.expect("non-null function pointer")).expect("non-null function pointer"))(&mut r,
                                                                                                                                                               xprt);
                            current_block = 4534832402155516892;
                            break ;
                        } else {
                            prog_found = 1 as libc::c_int;
                            if (*s).sc_vers < low_vers {
                                low_vers = (*s).sc_vers
                            }
                            if (*s).sc_vers > high_vers {
                                high_vers = (*s).sc_vers
                            }
                        }
                    }
                    s = (*s).sc_next
                    /* found correct program */
                }
                match current_block {
                    4534832402155516892 => { }
                    _ => {
                        /*
		 * if we got here, the program or version
		 * is not served ...
		 */
                        if prog_found != 0 {
                            gssrpc_svcerr_progvers(xprt, low_vers, high_vers);
                        } else { gssrpc_svcerr_noprog(xprt); }
                    }
                }
            }
        }
        /* Fall through to ... */
        stat =
            Some((*(*xprt).xp_ops).xp_stat.expect("non-null function pointer")).expect("non-null function pointer")(xprt);
        if stat as libc::c_uint == XPRT_DIED as libc::c_int as libc::c_uint {
            Some((*(*xprt).xp_ops).xp_destroy.expect("non-null function pointer")).expect("non-null function pointer")(xprt);
            break ;
        } else {
            if !(*xprt).xp_auth.is_null() &&
                   (*(*xprt).xp_auth).svc_ah_ops !=
                       &mut gssrpc_svc_auth_gss_ops as *mut svc_auth_ops {
                (*xprt).xp_auth = 0 as *mut SVCAUTH
            }
            if !(stat as libc::c_uint ==
                     XPRT_MOREREQS as libc::c_int as libc::c_uint) {
                break ;
            }
        }
    }
    free(rawcred as *mut libc::c_void);
    free(rawverf as *mut libc::c_void);
    free(cookedcred as *mut libc::c_void);
}
