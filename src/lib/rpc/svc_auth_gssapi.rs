use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:14"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stdarg.h:14"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/usr/include/bits/types.h:14"]
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
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
}
#[c2rust::header_src = "/usr/include/sys/types.h:17"]
pub mod sys_types_h {
    #[c2rust::src_loc = "34:1"]
    pub type u_short = __u_short;
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_short, __u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:17"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:17"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:17"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:17"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/netinet/in.h:17"]
pub mod in_h {
    #[c2rust::src_loc = "30:1"]
    pub type in_addr_t = uint32_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:8"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
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
    use super::stdint_uintn_h::{uint32_t, uint16_t};
    use super::sockaddr_h::sa_family_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:17"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:17"]
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
    extern "C" {
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth.h:17"]
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
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc_msg.h:17"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:17"]
pub mod gssapi_h {
    #[c2rust::src_loc = "79:1"]
    pub type gss_name_t = *mut gss_name_struct;
    #[c2rust::src_loc = "82:1"]
    pub type gss_cred_id_t = *mut gss_cred_id_struct;
    #[c2rust::src_loc = "85:1"]
    pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
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
        /* delegated_cred_handle */
        #[no_mangle]
        #[c2rust::src_loc = "467:1"]
        pub fn gss_process_context_token(_: *mut OM_uint32, _: gss_ctx_id_t,
                                         _: gss_buffer_t) -> OM_uint32;
        /* token_buffer */
        #[no_mangle]
        #[c2rust::src_loc = "474:1"]
        pub fn gss_delete_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_buffer_t) -> OM_uint32;
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
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/svc.h:17"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/svc_auth.h:17"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth_gssapi.h:21"]
pub mod auth_gssapi_h {
    #[c2rust::src_loc = "65:1"]
    pub type auth_gssapi_log_badverf_func
        =
        Option<unsafe extern "C" fn(_: gss_name_t, _: gss_name_t,
                                    _: *mut svc_req, _: *mut rpc_msg,
                                    _: caddr_t) -> ()>;
    /* auth_gssapi_log_badauth_func is IPv4-specific; this version gives the
 * transport handle so the fd can be used to get the address. */
    #[c2rust::src_loc = "59:1"]
    pub type auth_gssapi_log_badauth2_func
        =
        Option<unsafe extern "C" fn(_: OM_uint32, _: OM_uint32,
                                    _: *mut SVCXPRT, _: caddr_t) -> ()>;
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
    #[c2rust::src_loc = "38:1"]
    pub type auth_gssapi_init_arg = _auth_gssapi_init_arg;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:16"]
    pub struct _auth_gssapi_init_arg {
        pub version: uint32_t,
        pub token: gss_buffer_desc,
    }
    #[c2rust::src_loc = "32:1"]
    pub type auth_gssapi_creds = _auth_gssapi_creds;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "32:16"]
    pub struct _auth_gssapi_creds {
        pub version: uint32_t,
        pub auth_msg: libc::c_int,
        pub client_handle: gss_buffer_desc,
    }
    #[c2rust::src_loc = "43:1"]
    pub type auth_gssapi_init_res = _auth_gssapi_init_res;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "43:16"]
    pub struct _auth_gssapi_init_res {
        pub version: uint32_t,
        pub client_handle: gss_buffer_desc,
        pub gss_major: OM_uint32,
        pub gss_minor: OM_uint32,
        pub token: gss_buffer_desc,
        pub signed_isn: gss_buffer_desc,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:16"]
    pub struct _auth_gssapi_name {
        pub name: *mut libc::c_char,
        pub type_0: gss_OID,
    }
    /*
 * Yuck.  Some sys/types.h files leak symbols
 */
    #[c2rust::src_loc = "27:1"]
    pub type auth_gssapi_name = _auth_gssapi_name;
    use super::gssapi_h::{gss_name_t, OM_uint32, gss_buffer_desc, gss_OID,
                          gss_ctx_id_struct, gss_ctx_id_t,
                          gss_buffer_desc_struct, gss_buffer_t};
    use super::svc_h::{svc_req, SVCXPRT};
    use super::rpc_msg_h::rpc_msg;
    use super::sys_types_h::caddr_t;
    use super::in_h::sockaddr_in;
    use super::stdint_uintn_h::uint32_t;
    use super::xdr_h::XDR;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn gssrpc_xdr_authgssapi_creds(_: *mut XDR,
                                           _: *mut auth_gssapi_creds)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "80:1"]
        pub fn gssrpc_xdr_authgssapi_init_arg(_: *mut XDR,
                                              _: *mut auth_gssapi_init_arg)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "81:1"]
        pub fn gssrpc_xdr_authgssapi_init_res(_: *mut XDR,
                                              _: *mut auth_gssapi_init_res)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "83:1"]
        pub fn gssrpc_auth_gssapi_wrap_data(major: *mut OM_uint32,
                                            minor: *mut OM_uint32,
                                            context: gss_ctx_id_t,
                                            seq_num: uint32_t,
                                            out_xdrs: *mut XDR,
                                            xdr_func:
                                                Option<unsafe extern "C" fn()
                                                           -> libc::c_int>,
                                            xdr_ptr: caddr_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "88:1"]
        pub fn gssrpc_auth_gssapi_unwrap_data(major: *mut OM_uint32,
                                              minor: *mut OM_uint32,
                                              context: gss_ctx_id_t,
                                              seq_num: uint32_t,
                                              in_xdrs: *mut XDR,
                                              xdr_func:
                                                  Option<unsafe extern "C" fn()
                                                             -> libc::c_int>,
                                              xdr_ptr: caddr_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "110:1"]
        pub fn gssrpc_auth_gssapi_display_status(msg: *mut libc::c_char,
                                                 major: OM_uint32,
                                                 minor: OM_uint32);
        #[no_mangle]
        #[c2rust::src_loc = "114:1"]
        pub fn gssrpc_auth_gssapi_seal_seq(context: gss_ctx_id_t,
                                           seq_num: uint32_t,
                                           out_buf: gss_buffer_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "117:1"]
        pub fn gssrpc_auth_gssapi_unseal_seq(context: gss_ctx_id_t,
                                             in_buf: gss_buffer_t,
                                             seq_num: *mut uint32_t)
         -> libc::c_int;
    }
    /* !defined(GSSRPC_AUTH_GSSAPI_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:32"]
pub mod krb5_h {
    /* *
 * Used to convey an operation status.  The value 0 indicates success; any
 * other values are com_err codes.  Use krb5_get_error_message() to obtain a
 * string describing the error.
 */
    #[c2rust::src_loc = "204:1"]
    pub type krb5_error_code = krb5_int32;
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    use super::stdint_intn_h::int32_t;
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/stdio.h:14"]
pub mod stdio_h {
    use super::internal::__va_list_tag;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "347:12"]
        pub fn vprintf(_: *const libc::c_char, _: ::std::ffi::VaList)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:16"]
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
#[c2rust::header_src = "/usr/include/stdlib.h:17"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "453:1"]
        pub fn rand() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/time.h:37"]
pub mod time_h {
    use super::time_t_h::time_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "75:1"]
        pub fn time(__timer: *mut time_t) -> time_t;
    }
}
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::stddef_h::size_t;
pub use self::stdarg_h::va_list;
pub use self::types_h::{__u_short, __u_int, __uint16_t, __int32_t, __uint32_t,
                        __time_t, __caddr_t};
pub use self::sys_types_h::{u_short, u_int, caddr_t};
pub use self::time_t_h::time_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint16_t, uint32_t};
pub use self::sockaddr_h::sa_family_t;
pub use self::in_h::{in_addr_t, in_addr, in_port_t, sockaddr_in};
pub use self::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t, rpc_inline_t};
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, gssrpc_xdr_void, gssrpc_xdrmem_create,
                      gssrpc_xdr_free};
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
pub use self::gssapi_h::{gss_name_t, gss_cred_id_t, gss_ctx_id_t, gss_uint32,
                         OM_uint32, gss_OID_desc_struct, gss_OID,
                         gss_OID_set_desc_struct, gss_OID_set,
                         gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_channel_bindings_struct,
                         gss_channel_bindings_t, gss_cred_usage_t,
                         gss_name_struct, gss_cred_id_struct,
                         gss_ctx_id_struct, gss_acquire_cred,
                         gss_release_cred, gss_accept_sec_context,
                         gss_process_context_token, gss_delete_sec_context,
                         gss_import_name, gss_release_name,
                         gss_release_buffer};
pub use self::svc_h::{svc_req, SVCXPRT, xp_ops, xprt_stat, XPRT_IDLE,
                      XPRT_MOREREQS, XPRT_DIED, gssrpc_svc_sendreply};
pub use self::svc_auth_h::{SVCAUTH, svc_auth_ops, gssrpc_svc_auth_none};
pub use self::auth_gssapi_h::{auth_gssapi_log_badverf_func,
                              auth_gssapi_log_badauth2_func,
                              auth_gssapi_log_badauth_func,
                              auth_gssapi_log_miscerr_func,
                              auth_gssapi_init_arg, _auth_gssapi_init_arg,
                              auth_gssapi_creds, _auth_gssapi_creds,
                              auth_gssapi_init_res, _auth_gssapi_init_res,
                              _auth_gssapi_name, auth_gssapi_name,
                              gssrpc_xdr_authgssapi_creds,
                              gssrpc_xdr_authgssapi_init_arg,
                              gssrpc_xdr_authgssapi_init_res,
                              gssrpc_auth_gssapi_wrap_data,
                              gssrpc_auth_gssapi_unwrap_data,
                              gssrpc_auth_gssapi_display_status,
                              gssrpc_auth_gssapi_seal_seq,
                              gssrpc_auth_gssapi_unseal_seq};
pub use self::krb5_h::{krb5_error_code, krb5_int32};
use self::stdio_h::vprintf;
use self::string_h::{memcpy, memset, strlen};
use self::stdlib_h::{rand, malloc, free, abort, exit};
use self::time_h::time;
#[c2rust::src_loc = "79:1"]
pub type svc_auth_gssapi_data = _svc_auth_gssapi_data;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "79:16"]
pub struct _svc_auth_gssapi_data {
    pub established: libc::c_int,
    pub context: gss_ctx_id_t,
    pub client_name: gss_name_t,
    pub server_name: gss_name_t,
    pub server_creds: gss_cred_id_t,
    pub expiration: uint32_t,
    pub seq_num: uint32_t,
    pub key: uint32_t,
    pub svcauth: SVCAUTH,
    pub prev_verf: gss_buffer_desc,
}
#[c2rust::src_loc = "138:1"]
pub type client_list = _client_list;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "138:16"]
pub struct _client_list {
    pub client: *mut svc_auth_gssapi_data,
    pub next: *mut _client_list,
}
/* kludge to free verifiers on next call */
/* seconds until an context with no */
/* expiration time is expired */
#[no_mangle]
#[c2rust::src_loc = "49:5"]
pub static mut gssrpc_svc_debug_gssapi: libc::c_int = 0 as libc::c_int;
/* lib/rpc/gssrpcint.h */
/*
 * Copyright (C) 2008 by the Massachusetts Institute of Technology.
 * All rights reserved.
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
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn gssrpcint_printf(mut format: *const libc::c_char,
                                          mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    vprintf(format, ap.as_va_list());
}
#[no_mangle]
#[c2rust::src_loc = "113:21"]
pub static mut gssrpc_svc_auth_gssapi_ops: svc_auth_ops =
    unsafe {
        {
            let mut init =
                svc_auth_ops{svc_ah_wrap:
                                 Some(svc_auth_gssapi_wrap as
                                          unsafe extern "C" fn(_:
                                                                   *mut SVCAUTH,
                                                               _: *mut XDR,
                                                               _: xdrproc_t,
                                                               _: caddr_t)
                                              -> libc::c_int),
                             svc_ah_unwrap:
                                 Some(svc_auth_gssapi_unwrap as
                                          unsafe extern "C" fn(_:
                                                                   *mut SVCAUTH,
                                                               _: *mut XDR,
                                                               _: xdrproc_t,
                                                               _: caddr_t)
                                              -> libc::c_int),
                             svc_ah_destroy:
                                 Some(svc_auth_gssapi_destroy as
                                          unsafe extern "C" fn(_:
                                                                   *mut SVCAUTH)
                                              -> libc::c_int),};
            init
        }
    };
/*
 * Globals!  Eeek!  Run for the hills!
 */
#[c2rust::src_loc = "122:23"]
static mut server_creds_list: *mut gss_cred_id_t =
    0 as *const gss_cred_id_t as *mut gss_cred_id_t;
#[c2rust::src_loc = "123:20"]
static mut server_name_list: *mut gss_name_t =
    0 as *const gss_name_t as *mut gss_name_t;
#[c2rust::src_loc = "124:12"]
static mut server_creds_count: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "126:37"]
static mut log_badauth: auth_gssapi_log_badauth_func = None;
#[c2rust::src_loc = "127:16"]
static mut log_badauth_data: caddr_t = 0 as *const libc::c_char as caddr_t;
#[c2rust::src_loc = "128:38"]
static mut log_badauth2: auth_gssapi_log_badauth2_func = None;
#[c2rust::src_loc = "129:16"]
static mut log_badauth2_data: caddr_t = 0 as *const libc::c_char as caddr_t;
#[c2rust::src_loc = "130:37"]
static mut log_badverf: auth_gssapi_log_badverf_func = None;
#[c2rust::src_loc = "131:16"]
static mut log_badverf_data: caddr_t = 0 as *const libc::c_char as caddr_t;
#[c2rust::src_loc = "132:37"]
static mut log_miscerr: auth_gssapi_log_miscerr_func = None;
#[c2rust::src_loc = "133:16"]
static mut log_miscerr_data: caddr_t = 0 as *const libc::c_char as caddr_t;
#[c2rust::src_loc = "143:21"]
static mut clients: *mut client_list =
    0 as *const client_list as *mut client_list;
/* Invoke log_badauth callbacks for an authentication failure. */
#[c2rust::src_loc = "147:1"]
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
#[no_mangle]
#[c2rust::src_loc = "156:1"]
pub unsafe extern "C" fn gssrpc__svcauth_gssapi(mut rqst: *mut svc_req,
                                                mut msg: *mut rpc_msg,
                                                mut no_dispatch:
                                                    *mut libc::c_int)
 -> auth_stat {
    let mut current_block: u64;
    let mut xdrs: XDR =
        XDR{x_op: XDR_ENCODE,
            x_ops: 0 as *mut xdr_ops,
            x_public: 0 as *mut libc::c_char,
            x_private: 0 as *mut libc::c_void,
            x_base: 0 as *mut libc::c_char,
            x_handy: 0,};
    let mut creds: auth_gssapi_creds =
        auth_gssapi_creds{version: 0,
                          auth_msg: 0,
                          client_handle:
                              gss_buffer_desc{length: 0,
                                              value:
                                                  0 as *mut libc::c_void,},};
    let mut call_arg: auth_gssapi_init_arg =
        auth_gssapi_init_arg{version: 0,
                             token:
                                 gss_buffer_desc{length: 0,
                                                 value:
                                                     0 as
                                                         *mut libc::c_void,},};
    let mut call_res: auth_gssapi_init_res =
        auth_gssapi_init_res{version: 0,
                             client_handle:
                                 gss_buffer_desc{length: 0,
                                                 value:
                                                     0 as *mut libc::c_void,},
                             gss_major: 0,
                             gss_minor: 0,
                             token:
                                 gss_buffer_desc{length: 0,
                                                 value:
                                                     0 as *mut libc::c_void,},
                             signed_isn:
                                 gss_buffer_desc{length: 0,
                                                 value:
                                                     0 as
                                                         *mut libc::c_void,},};
    let mut output_token: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut in_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut out_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut server_creds: gss_cred_id_t = 0 as *mut gss_cred_id_struct;
    let mut bindings: gss_channel_bindings_struct =
        gss_channel_bindings_struct{initiator_addrtype: 0,
                                    initiator_address:
                                        gss_buffer_desc{length: 0,
                                                        value:
                                                            0 as
                                                                *mut libc::c_void,},
                                    acceptor_addrtype: 0,
                                    acceptor_address:
                                        gss_buffer_desc{length: 0,
                                                        value:
                                                            0 as
                                                                *mut libc::c_void,},
                                    application_data:
                                        gss_buffer_desc{length: 0,
                                                        value:
                                                            0 as
                                                                *mut libc::c_void,},};
    let mut bindp: *mut gss_channel_bindings_struct =
        0 as *mut gss_channel_bindings_struct;
    let mut gssstat: OM_uint32 = 0;
    let mut minor_stat: OM_uint32 = 0;
    let mut time_rec: OM_uint32 = 0;
    let mut cred: *mut opaque_auth = 0 as *mut opaque_auth;
    let mut verf: *mut opaque_auth = 0 as *mut opaque_auth;
    let mut client_data: *mut svc_auth_gssapi_data =
        0 as *mut svc_auth_gssapi_data;
    let mut i: libc::c_int = 0;
    let mut ret: auth_stat = AUTH_OK;
    let mut ret_flags: OM_uint32 = 0;
    let mut seq_num: uint32_t = 0;
    if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"svcauth_gssapi: starting\n\x00" as *const u8 as
                             *const libc::c_char);
    }
    /* clean up expired entries */
    clean_client();
    /* use AUTH_NONE until there is a client_handle */
    (*(*rqst).rq_xprt).xp_auth = &mut gssrpc_svc_auth_none;
    memset(&mut call_res as *mut auth_gssapi_init_res as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<auth_gssapi_init_res>() as libc::c_ulong);
    creds.client_handle.length = 0 as libc::c_int as size_t;
    creds.client_handle.value = 0 as *mut libc::c_void;
    cred = &mut (*msg).ru.RM_cmb.cb_cred;
    verf = &mut (*msg).ru.RM_cmb.cb_verf;
    if (*cred).oa_length == 0 as libc::c_int as libc::c_uint {
        if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"svcauth_gssapi: empty creds, failing\n\x00" as
                                 *const u8 as *const libc::c_char);
        }
        if log_miscerr.is_some() {
            Some(log_miscerr.expect("non-null function pointer")).expect("non-null function pointer")(rqst,
                                                                                                      msg,
                                                                                                      b"empty client credentials\x00"
                                                                                                          as
                                                                                                          *const u8
                                                                                                          as
                                                                                                          *const libc::c_char
                                                                                                          as
                                                                                                          *mut libc::c_char,
                                                                                                      log_miscerr_data);
        }
        ret = AUTH_BADCRED
    } else {
        if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"svcauth_gssapi: decoding credentials\n\x00" as
                                 *const u8 as *const libc::c_char);
        }
        gssrpc_xdrmem_create(&mut xdrs, (*cred).oa_base, (*cred).oa_length,
                             XDR_DECODE);
        memset(&mut creds as *mut auth_gssapi_creds as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<auth_gssapi_creds>() as libc::c_ulong);
        if gssrpc_xdr_authgssapi_creds(&mut xdrs, &mut creds) == 0 {
            if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
                gssrpcint_printf(b"svcauth_gssapi: failed decoding creds\n\x00"
                                     as *const u8 as *const libc::c_char);
            }
            if log_miscerr.is_some() {
                Some(log_miscerr.expect("non-null function pointer")).expect("non-null function pointer")(rqst,
                                                                                                          msg,
                                                                                                          b"protocol error in client credentials\x00"
                                                                                                              as
                                                                                                              *const u8
                                                                                                              as
                                                                                                              *const libc::c_char
                                                                                                              as
                                                                                                              *mut libc::c_char,
                                                                                                          log_miscerr_data);
            }
            gssrpc_xdr_free(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                    *mut XDR,
                                                                                _:
                                                                                    *mut auth_gssapi_creds)
                                                               ->
                                                                   libc::c_int>,
                                                    xdrproc_t>(Some(gssrpc_xdr_authgssapi_creds
                                                                        as
                                                                        unsafe extern "C" fn(_:
                                                                                                 *mut XDR,
                                                                                             _:
                                                                                                 *mut auth_gssapi_creds)
                                                                            ->
                                                                                libc::c_int)),
                            &mut creds as *mut auth_gssapi_creds as
                                *mut libc::c_void);
            if (*xdrs.x_ops).x_destroy.is_some() {
                Some((*xdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut xdrs);
            }
            ret = AUTH_BADCRED
        } else {
            if (*xdrs.x_ops).x_destroy.is_some() {
                Some((*xdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut xdrs);
            }
            if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
                gssrpcint_printf(b"svcauth_gssapi: got credentials, version %d, client_handle len %d\n\x00"
                                     as *const u8 as *const libc::c_char,
                                 creds.version,
                                 creds.client_handle.length as libc::c_int);
            }
            if creds.version != 2 as libc::c_int as libc::c_uint {
                if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
                    gssrpcint_printf(b"svcauth_gssapi: bad credential version\n\x00"
                                         as *const u8 as *const libc::c_char);
                }
                if log_miscerr.is_some() {
                    Some(log_miscerr.expect("non-null function pointer")).expect("non-null function pointer")(rqst,
                                                                                                              msg,
                                                                                                              b"unsupported client credentials version\x00"
                                                                                                                  as
                                                                                                                  *const u8
                                                                                                                  as
                                                                                                                  *const libc::c_char
                                                                                                                  as
                                                                                                                  *mut libc::c_char,
                                                                                                              log_miscerr_data);
                }
                ret = AUTH_BADCRED
            } else {
                if gssrpc_svc_debug_gssapi != 0 {
                    if creds.auth_msg != 0 &&
                           (*rqst).rq_proc == 0 as libc::c_int as libc::c_uint
                       {
                        if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
                            gssrpcint_printf(b"svcauth_gssapi: GSSAPI_EXIT, cleaning up\n\x00"
                                                 as *const u8 as
                                                 *const libc::c_char);
                        }
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
                                             0 as caddr_t);
                        gssrpc_xdr_free(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                *mut XDR,
                                                                                            _:
                                                                                                *mut auth_gssapi_creds)
                                                                           ->
                                                                               libc::c_int>,
                                                                xdrproc_t>(Some(gssrpc_xdr_authgssapi_creds
                                                                                    as
                                                                                    unsafe extern "C" fn(_:
                                                                                                             *mut XDR,
                                                                                                         _:
                                                                                                             *mut auth_gssapi_creds)
                                                                                        ->
                                                                                            libc::c_int)),
                                        &mut creds as *mut auth_gssapi_creds
                                            as *mut libc::c_void);
                        cleanup();
                        exit(0 as libc::c_int);
                    }
                }
                /*
      * If this is an auth_msg and proc is GSSAPI_INIT, then create a
      * client handle for this client.  Otherwise, look up the
      * existing handle.
      */
                if creds.auth_msg != 0 &&
                       (*rqst).rq_proc == 1 as libc::c_int as libc::c_uint {
                    if creds.client_handle.length !=
                           0 as libc::c_int as libc::c_ulong {
                        if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
                            gssrpcint_printf(b"svcauth_gssapi: non-empty handle on GSSAPI_INIT\n\x00"
                                                 as *const u8 as
                                                 *const libc::c_char);
                        }
                        if log_miscerr.is_some() {
                            Some(log_miscerr.expect("non-null function pointer")).expect("non-null function pointer")(rqst,
                                                                                                                      msg,
                                                                                                                      b"protocol error in client handle\x00"
                                                                                                                          as
                                                                                                                          *const u8
                                                                                                                          as
                                                                                                                          *const libc::c_char
                                                                                                                          as
                                                                                                                          *mut libc::c_char,
                                                                                                                      log_miscerr_data);
                        }
                        ret = AUTH_FAILED;
                        current_block = 15532589080399555063;
                    } else {
                        if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
                            gssrpcint_printf(b"svcauth_gssapi: GSSAPI_INIT, creating client.\n\x00"
                                                 as *const u8 as
                                                 *const libc::c_char);
                        }
                        client_data = create_client();
                        if client_data.is_null() {
                            if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
                                gssrpcint_printf(b"svcauth_gssapi: create_client failed\n\x00"
                                                     as *const u8 as
                                                     *const libc::c_char);
                            }
                            if log_miscerr.is_some() {
                                Some(log_miscerr.expect("non-null function pointer")).expect("non-null function pointer")(rqst,
                                                                                                                          msg,
                                                                                                                          b"internal error creating client record\x00"
                                                                                                                              as
                                                                                                                              *const u8
                                                                                                                              as
                                                                                                                              *const libc::c_char
                                                                                                                              as
                                                                                                                              *mut libc::c_char,
                                                                                                                          log_miscerr_data);
                            }
                            ret = AUTH_FAILED;
                            current_block = 15532589080399555063;
                        } else { current_block = 9199578309995299736; }
                    }
                } else if creds.client_handle.length ==
                              0 as libc::c_int as libc::c_ulong {
                    if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
                        gssrpcint_printf(b"svcauth_gssapi: expected non-empty creds\n\x00"
                                             as *const u8 as
                                             *const libc::c_char);
                    }
                    if log_miscerr.is_some() {
                        Some(log_miscerr.expect("non-null function pointer")).expect("non-null function pointer")(rqst,
                                                                                                                  msg,
                                                                                                                  b"protocol error in client credentials\x00"
                                                                                                                      as
                                                                                                                      *const u8
                                                                                                                      as
                                                                                                                      *const libc::c_char
                                                                                                                      as
                                                                                                                      *mut libc::c_char,
                                                                                                                  log_miscerr_data);
                    }
                    ret = AUTH_FAILED;
                    current_block = 15532589080399555063;
                } else {
                    if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
                        gssrpcint_printf(b"svcauth_gssapi: incoming client_handle %d, len %d\n\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         *(creds.client_handle.value as
                                               *mut uint32_t),
                                         creds.client_handle.length as
                                             libc::c_int);
                    }
                    client_data = get_client(&mut creds.client_handle);
                    if client_data.is_null() {
                        if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
                            gssrpcint_printf(b"svcauth_gssapi: client_handle lookup failed\n\x00"
                                                 as *const u8 as
                                                 *const libc::c_char);
                        }
                        if log_miscerr.is_some() {
                            Some(log_miscerr.expect("non-null function pointer")).expect("non-null function pointer")(rqst,
                                                                                                                      msg,
                                                                                                                      b"invalid client handle received\x00"
                                                                                                                          as
                                                                                                                          *const u8
                                                                                                                          as
                                                                                                                          *const libc::c_char
                                                                                                                          as
                                                                                                                          *mut libc::c_char,
                                                                                                                      log_miscerr_data);
                        }
                        ret = AUTH_BADCRED;
                        current_block = 15532589080399555063;
                    } else {
                        if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
                            gssrpcint_printf(b"svcauth_gssapi: client_handle lookup succeeded\n\x00"
                                                 as *const u8 as
                                                 *const libc::c_char);
                        }
                        current_block = 9199578309995299736;
                    }
                }
                match current_block {
                    15532589080399555063 => { }
                    _ => {
                        /* any response we send will use client_handle, so set it now */
                        call_res.client_handle.length =
                            ::std::mem::size_of::<uint32_t>() as
                                libc::c_ulong;
                        call_res.client_handle.value =
                            &mut (*client_data).key as *mut uint32_t as
                                *mut libc::c_char as *mut libc::c_void;
                        /* mark this call as using AUTH_GSSAPI via client_data's SVCAUTH */
                        (*(*rqst).rq_xprt).xp_auth =
                            &mut (*client_data).svcauth;
                        if (*client_data).established == 0 as libc::c_int {
                            if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
                                gssrpcint_printf(b"svcauth_gssapi: context is not established\n\x00"
                                                     as *const u8 as
                                                     *const libc::c_char);
                            }
                            if creds.auth_msg == 0 as libc::c_int {
                                if gssrpc_svc_debug_gssapi >=
                                       99 as libc::c_int {
                                    gssrpcint_printf(b"svcauth_gssapi: expected auth_msg TRUE\n\x00"
                                                         as *const u8 as
                                                         *const libc::c_char);
                                }
                                if log_miscerr.is_some() {
                                    Some(log_miscerr.expect("non-null function pointer")).expect("non-null function pointer")(rqst,
                                                                                                                              msg,
                                                                                                                              b"protocol error on incomplete connection\x00"
                                                                                                                                  as
                                                                                                                                  *const u8
                                                                                                                                  as
                                                                                                                                  *const libc::c_char
                                                                                                                                  as
                                                                                                                                  *mut libc::c_char,
                                                                                                                              log_miscerr_data);
                                }
                                ret = AUTH_REJECTEDCRED;
                                current_block = 15532589080399555063;
                            } else if (*rqst).rq_proc !=
                                          1 as libc::c_int as libc::c_uint &&
                                          (*rqst).rq_proc !=
                                              2 as libc::c_int as libc::c_uint
                             {
                                if gssrpc_svc_debug_gssapi >=
                                       99 as libc::c_int {
                                    gssrpcint_printf(b"svcauth_gssapi: unacceptable procedure %d\n\x00"
                                                         as *const u8 as
                                                         *const libc::c_char,
                                                     (*rqst).rq_proc);
                                }
                                if log_miscerr.is_some() {
                                    Some(log_miscerr.expect("non-null function pointer")).expect("non-null function pointer")(rqst,
                                                                                                                              msg,
                                                                                                                              b"protocol error on incomplete connection\x00"
                                                                                                                                  as
                                                                                                                                  *const u8
                                                                                                                                  as
                                                                                                                                  *const libc::c_char
                                                                                                                                  as
                                                                                                                                  *mut libc::c_char,
                                                                                                                              log_miscerr_data);
                                }
                                ret = AUTH_FAILED;
                                current_block = 15532589080399555063;
                            } else {
                                /*
	   * If the context is not established, then only GSSAPI_INIT
	   * and _CONTINUE requests are valid.
	   */
                                /* call is for us, deserialize arguments */
                                memset(&mut call_arg as
                                           *mut auth_gssapi_init_arg as
                                           *mut libc::c_void,
                                       0 as libc::c_int,
                                       ::std::mem::size_of::<auth_gssapi_init_arg>()
                                           as libc::c_ulong);
                                if Some((*(*(*rqst).rq_xprt).xp_ops).xp_getargs.expect("non-null function pointer")).expect("non-null function pointer")((*rqst).rq_xprt,
                                                                                                                                                         ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                                                                                 *mut XDR,
                                                                                                                                                                                                             _:
                                                                                                                                                                                                                 *mut auth_gssapi_init_arg)
                                                                                                                                                                                            ->
                                                                                                                                                                                                libc::c_int>,
                                                                                                                                                                                 xdrproc_t>(Some(gssrpc_xdr_authgssapi_init_arg
                                                                                                                                                                                                     as
                                                                                                                                                                                                     unsafe extern "C" fn(_:
                                                                                                                                                                                                                              *mut XDR,
                                                                                                                                                                                                                          _:
                                                                                                                                                                                                                              *mut auth_gssapi_init_arg)
                                                                                                                                                                                                         ->
                                                                                                                                                                                                             libc::c_int)),
                                                                                                                                                         &mut call_arg
                                                                                                                                                             as
                                                                                                                                                             *mut auth_gssapi_init_arg
                                                                                                                                                             as
                                                                                                                                                             *mut libc::c_void)
                                       == 0 {
                                    if gssrpc_svc_debug_gssapi >=
                                           99 as libc::c_int {
                                        gssrpcint_printf(b"svcauth_gssapi: cannot decode args\n\x00"
                                                             as *const u8 as
                                                             *const libc::c_char);
                                    }
                                    if log_miscerr.is_some() {
                                        Some(log_miscerr.expect("non-null function pointer")).expect("non-null function pointer")(rqst,
                                                                                                                                  msg,
                                                                                                                                  b"protocol error in procedure arguments\x00"
                                                                                                                                      as
                                                                                                                                      *const u8
                                                                                                                                      as
                                                                                                                                      *const libc::c_char
                                                                                                                                      as
                                                                                                                                      *mut libc::c_char,
                                                                                                                                  log_miscerr_data);
                                    }
                                    ret = AUTH_BADCRED;
                                    current_block = 15532589080399555063;
                                } else {
                                    /*
	   * Process the call arg version number.
	   *
	   * Set the krb5_gss backwards-compatibility mode based on client
	   * version.  This controls whether the AP_REP message is
	   * encrypted with the session key (version 2+, correct) or the
	   * session subkey (version 1, incorrect).  This function can
	   * never fail, so we don't bother checking its return value.
	   */
                                    match call_arg.version {
                                        1 | 2 => {
                                            if log_miscerr.is_some() {
                                                Some(log_miscerr.expect("non-null function pointer")).expect("non-null function pointer")(rqst,
                                                                                                                                          msg,
                                                                                                                                          b"Warning: Accepted old RPC protocol request\x00"
                                                                                                                                              as
                                                                                                                                              *const u8
                                                                                                                                              as
                                                                                                                                              *const libc::c_char
                                                                                                                                              as
                                                                                                                                              *mut libc::c_char,
                                                                                                                                          log_miscerr_data);
                                            }
                                            call_res.version =
                                                1 as libc::c_int as uint32_t;
                                            current_block =
                                                12696043255897098083;
                                        }
                                        3 | 4 => {
                                            /* 3 and 4 are essentially the same, don't bother warning */
                                            call_res.version =
                                                call_arg.version;
                                            current_block =
                                                12696043255897098083;
                                        }
                                        _ => {
                                            if gssrpc_svc_debug_gssapi >=
                                                   99 as libc::c_int {
                                                gssrpcint_printf(b"svcauth_gssapi: bad GSSAPI_INIT version\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
                                            }
                                            if log_miscerr.is_some() {
                                                Some(log_miscerr.expect("non-null function pointer")).expect("non-null function pointer")(rqst,
                                                                                                                                          msg,
                                                                                                                                          b"unsupported GSSAPI_INIT version\x00"
                                                                                                                                              as
                                                                                                                                              *const u8
                                                                                                                                              as
                                                                                                                                              *const libc::c_char
                                                                                                                                              as
                                                                                                                                              *mut libc::c_char,
                                                                                                                                          log_miscerr_data);
                                            }
                                            ret = AUTH_BADCRED;
                                            current_block =
                                                15532589080399555063;
                                        }
                                    }
                                    match current_block {
                                        15532589080399555063 => { }
                                        _ => {
                                            if call_arg.version >=
                                                   3 as libc::c_int as
                                                       libc::c_uint {
                                                memset(&mut bindings as
                                                           *mut gss_channel_bindings_struct
                                                           as
                                                           *mut libc::c_void,
                                                       0 as libc::c_int,
                                                       ::std::mem::size_of::<gss_channel_bindings_struct>()
                                                           as libc::c_ulong);
                                                bindings.application_data.length
                                                    =
                                                    0 as libc::c_int as
                                                        size_t;
                                                bindings.initiator_addrtype =
                                                    2 as libc::c_int as
                                                        OM_uint32;
                                                bindings.initiator_address.length
                                                    =
                                                    4 as libc::c_int as
                                                        size_t;
                                                bindings.initiator_address.value
                                                    =
                                                    &mut (*(*rqst).rq_xprt).xp_raddr.sin_addr.s_addr
                                                        as *mut in_addr_t as
                                                        *mut libc::c_void;
                                                if (*(*rqst).rq_xprt).xp_laddrlen
                                                       > 0 as libc::c_int {
                                                    bindings.acceptor_addrtype
                                                        =
                                                        2 as libc::c_int as
                                                            OM_uint32;
                                                    bindings.acceptor_address.length
                                                        =
                                                        4 as libc::c_int as
                                                            size_t;
                                                    bindings.acceptor_address.value
                                                        =
                                                        &mut (*(*rqst).rq_xprt).xp_laddr.sin_addr.s_addr
                                                            as *mut in_addr_t
                                                            as
                                                            *mut libc::c_void;
                                                    bindp = &mut bindings;
                                                    current_block =
                                                        15321816652064063775;
                                                } else {
                                                    if log_miscerr.is_some() {
                                                        Some(log_miscerr.expect("non-null function pointer")).expect("non-null function pointer")(rqst,
                                                                                                                                                  msg,
                                                                                                                                                  b"cannot get local address\x00"
                                                                                                                                                      as
                                                                                                                                                      *const u8
                                                                                                                                                      as
                                                                                                                                                      *const libc::c_char
                                                                                                                                                      as
                                                                                                                                                      *mut libc::c_char,
                                                                                                                                                  log_miscerr_data);
                                                    }
                                                    ret = AUTH_FAILED;
                                                    current_block =
                                                        15532589080399555063;
                                                }
                                            } else {
                                                bindp =
                                                    0 as
                                                        gss_channel_bindings_t;
                                                current_block =
                                                    15321816652064063775;
                                            }
                                            match current_block {
                                                15532589080399555063 => { }
                                                _ => {
                                                    /*
	   * If the client's server_creds is already set, use it.
	   * Otherwise, try each credential in server_creds_list until
	   * one of them succeedes, then set the client server_creds
	   * to that.  If all fail, the client's server_creds isn't
	   * set (which is fine, because the client will be gc'ed
	   * anyway).
	   *
	   * If accept_sec_context returns something other than
	   * success and GSS_S_FAILURE, then assume different
	   * credentials won't help and stop looping.
	   *
	   * Note that there are really two cases here: (1) the client
	   * has a server_creds already, and (2) it does not.  They
	   * are both written in the same loop so that there is only
	   * one textual call to gss_accept_sec_context; in fact, in
	   * case (1), the loop is executed exactly once.
	   */
                                                    i = 0 as libc::c_int;
                                                    while i <
                                                              server_creds_count
                                                          {
                                                        if !(*client_data).server_creds.is_null()
                                                           {
                                                            if gssrpc_svc_debug_gssapi
                                                                   >=
                                                                   99 as
                                                                       libc::c_int
                                                               {
                                                                gssrpcint_printf(b"svcauth_gssapi: using\'s clients server_creds\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char);
                                                            }
                                                            server_creds =
                                                                (*client_data).server_creds
                                                        } else {
                                                            if gssrpc_svc_debug_gssapi
                                                                   >=
                                                                   99 as
                                                                       libc::c_int
                                                               {
                                                                gssrpcint_printf(b"svcauth_gssapi: trying creds %d\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 i);
                                                            }
                                                            server_creds =
                                                                *server_creds_list.offset(i
                                                                                              as
                                                                                              isize)
                                                        }
                                                        /* Free previous output_token from loop */
                                                        if i !=
                                                               0 as
                                                                   libc::c_int
                                                           {
                                                            gss_release_buffer(&mut minor_stat,
                                                                               &mut output_token);
                                                        }
                                                        call_res.gss_major =
                                                            gss_accept_sec_context(&mut call_res.gss_minor,
                                                                                   &mut (*client_data).context,
                                                                                   server_creds,
                                                                                   &mut call_arg.token,
                                                                                   bindp,
                                                                                   &mut (*client_data).client_name,
                                                                                   0
                                                                                       as
                                                                                       *mut gss_OID,
                                                                                   &mut output_token,
                                                                                   &mut ret_flags,
                                                                                   &mut time_rec,
                                                                                   0
                                                                                       as
                                                                                       *mut gss_cred_id_t);
                                                        if server_creds ==
                                                               (*client_data).server_creds
                                                           {
                                                            break ;
                                                        }
                                                        if gssrpc_svc_debug_gssapi
                                                               >=
                                                               99 as
                                                                   libc::c_int
                                                           {
                                                            gssrpcint_printf(b"accept_sec_context returned 0x%x 0x%x not-us=%#x\n\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char,
                                                                             call_res.gss_major,
                                                                             call_res.gss_minor,
                                                                             -(1765328349
                                                                                   as
                                                                                   libc::c_long)
                                                                                 as
                                                                                 libc::c_int);
                                                        }
                                                        if call_res.gss_major
                                                               ==
                                                               0 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint
                                                               ||
                                                               call_res.gss_major
                                                                   ==
                                                                   ((1 as
                                                                         libc::c_int)
                                                                        <<
                                                                        0 as
                                                                            libc::c_int
                                                                            +
                                                                            0
                                                                                as
                                                                                libc::c_int)
                                                                       as
                                                                       libc::c_uint
                                                           {
                                                            /* server_creds was right, set it! */
                                                            if gssrpc_svc_debug_gssapi
                                                                   >=
                                                                   99 as
                                                                       libc::c_int
                                                               {
                                                                gssrpcint_printf(b"svcauth_gssapi: creds are correct, storing\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char);
                                                            }
                                                            (*client_data).server_creds
                                                                =
                                                                server_creds;
                                                            (*client_data).server_name
                                                                =
                                                                *server_name_list.offset(i
                                                                                             as
                                                                                             isize);
                                                            break ;
                                                        } else {
                                                            if call_res.gss_major
                                                                   !=
                                                                   (13 as
                                                                        libc::c_ulong
                                                                        as
                                                                        OM_uint32)
                                                                       <<
                                                                       16 as
                                                                           libc::c_int
                                                                   ||
                                                                   call_res.gss_minor
                                                                       as
                                                                       krb5_error_code
                                                                       !=
                                                                       -(1765328349
                                                                             as
                                                                             libc::c_long)
                                                                           as
                                                                           krb5_error_code
                                                               {
                                                                break ;
                                                            }
                                                            i += 1
                                                        }
                                                    }
                                                    gssstat =
                                                        call_res.gss_major;
                                                    minor_stat =
                                                        call_res.gss_minor;
                                                    /* done with call args */
                                                    gssrpc_xdr_free(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                            *mut XDR,
                                                                                                                        _:
                                                                                                                            *mut auth_gssapi_init_arg)
                                                                                                       ->
                                                                                                           libc::c_int>,
                                                                                            xdrproc_t>(Some(gssrpc_xdr_authgssapi_init_arg
                                                                                                                as
                                                                                                                unsafe extern "C" fn(_:
                                                                                                                                         *mut XDR,
                                                                                                                                     _:
                                                                                                                                         *mut auth_gssapi_init_arg)
                                                                                                                    ->
                                                                                                                        libc::c_int)),
                                                                    &mut call_arg
                                                                        as
                                                                        *mut auth_gssapi_init_arg
                                                                        as
                                                                        *mut libc::c_void);
                                                    if gssrpc_svc_debug_gssapi
                                                           >=
                                                           99 as libc::c_int {
                                                        gssrpcint_printf(b"svcauth_gssapi: accept_sec_context returned %#x %#x\n\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         call_res.gss_major,
                                                                         call_res.gss_minor);
                                                    }
                                                    if call_res.gss_major !=
                                                           0 as libc::c_int as
                                                               libc::c_uint &&
                                                           call_res.gss_major
                                                               !=
                                                               ((1 as
                                                                     libc::c_int)
                                                                    <<
                                                                    0 as
                                                                        libc::c_int
                                                                        +
                                                                        0 as
                                                                            libc::c_int)
                                                                   as
                                                                   libc::c_uint
                                                       {
                                                        if gssrpc_svc_debug_gssapi
                                                               != 0 {
                                                            gssrpc_auth_gssapi_display_status(b"accepting context\x00"
                                                                                                  as
                                                                                                  *const u8
                                                                                                  as
                                                                                                  *const libc::c_char
                                                                                                  as
                                                                                                  *mut libc::c_char,
                                                                                              call_res.gss_major,
                                                                                              call_res.gss_minor);
                                                        }
                                                        badauth(call_res.gss_major,
                                                                call_res.gss_minor,
                                                                (*rqst).rq_xprt);
                                                        gss_release_buffer(&mut minor_stat,
                                                                           &mut output_token);
                                                        gssrpc_svc_sendreply((*rqst).rq_xprt,
                                                                             ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                     *mut XDR,
                                                                                                                                 _:
                                                                                                                                     *mut auth_gssapi_init_res)
                                                                                                                ->
                                                                                                                    libc::c_int>,
                                                                                                     xdrproc_t>(Some(gssrpc_xdr_authgssapi_init_res
                                                                                                                         as
                                                                                                                         unsafe extern "C" fn(_:
                                                                                                                                                  *mut XDR,
                                                                                                                                              _:
                                                                                                                                                  *mut auth_gssapi_init_res)
                                                                                                                             ->
                                                                                                                                 libc::c_int)),
                                                                             &mut call_res
                                                                                 as
                                                                                 *mut auth_gssapi_init_res
                                                                                 as
                                                                                 caddr_t);
                                                        *no_dispatch =
                                                            1 as libc::c_int;
                                                        ret = AUTH_OK;
                                                        current_block =
                                                            15532589080399555063;
                                                    } else {
                                                        if output_token.length
                                                               !=
                                                               0 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong
                                                           {
                                                            if gssrpc_svc_debug_gssapi
                                                                   >=
                                                                   99 as
                                                                       libc::c_int
                                                               {
                                                                gssrpcint_printf(b"svcauth_gssapi: got new output token\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char);
                                                            }
                                                            call_res.token.length
                                                                =
                                                                output_token.length;
                                                            call_res.token.value
                                                                =
                                                                output_token.value
                                                        }
                                                        if gssstat ==
                                                               0 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint
                                                           {
                                                            (*client_data).seq_num
                                                                =
                                                                rand() as
                                                                    uint32_t;
                                                            client_expire(client_data,
                                                                          ((if time_rec
                                                                                   ==
                                                                                   0xffffffff
                                                                                       as
                                                                                       libc::c_ulong
                                                                                       as
                                                                                       OM_uint32
                                                                               {
                                                                                (60
                                                                                     as
                                                                                     libc::c_int
                                                                                     *
                                                                                     60
                                                                                         as
                                                                                         libc::c_int
                                                                                     *
                                                                                     24
                                                                                         as
                                                                                         libc::c_int)
                                                                                    as
                                                                                    libc::c_uint
                                                                            } else {
                                                                                time_rec
                                                                            })
                                                                               as
                                                                               libc::c_long
                                                                               +
                                                                               time(0
                                                                                        as
                                                                                        *mut time_t))
                                                                              as
                                                                              uint32_t);
                                                            if gssrpc_svc_debug_gssapi
                                                                   >=
                                                                   99 as
                                                                       libc::c_int
                                                               {
                                                                gssrpcint_printf(b"svcauth_gssapi: context established, isn %d\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 (*client_data).seq_num);
                                                            }
                                                            if gssrpc_auth_gssapi_seal_seq((*client_data).context,
                                                                                           (*client_data).seq_num,
                                                                                           &mut call_res.signed_isn)
                                                                   ==
                                                                   0 as
                                                                       libc::c_int
                                                               {
                                                                ret =
                                                                    AUTH_FAILED;
                                                                if log_miscerr.is_some()
                                                                   {
                                                                    Some(log_miscerr.expect("non-null function pointer")).expect("non-null function pointer")(rqst,
                                                                                                                                                              msg,
                                                                                                                                                              b"internal error sealing sequence number\x00"
                                                                                                                                                                  as
                                                                                                                                                                  *const u8
                                                                                                                                                                  as
                                                                                                                                                                  *const libc::c_char
                                                                                                                                                                  as
                                                                                                                                                                  *mut libc::c_char,
                                                                                                                                                              log_miscerr_data);
                                                                }
                                                                gss_release_buffer(&mut minor_stat,
                                                                                   &mut output_token);
                                                                current_block
                                                                    =
                                                                    15532589080399555063;
                                                            } else {
                                                                current_block
                                                                    =
                                                                    4367030874028593650;
                                                            }
                                                        } else {
                                                            current_block =
                                                                4367030874028593650;
                                                        }
                                                        match current_block {
                                                            15532589080399555063
                                                            => {
                                                            }
                                                            _ => {
                                                                if gssrpc_svc_debug_gssapi
                                                                       >=
                                                                       99 as
                                                                           libc::c_int
                                                                   {
                                                                    gssrpcint_printf(b"svcauth_gssapi: sending reply\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char);
                                                                }
                                                                gssrpc_svc_sendreply((*rqst).rq_xprt,
                                                                                     ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                             *mut XDR,
                                                                                                                                         _:
                                                                                                                                             *mut auth_gssapi_init_res)
                                                                                                                        ->
                                                                                                                            libc::c_int>,
                                                                                                             xdrproc_t>(Some(gssrpc_xdr_authgssapi_init_res
                                                                                                                                 as
                                                                                                                                 unsafe extern "C" fn(_:
                                                                                                                                                          *mut XDR,
                                                                                                                                                      _:
                                                                                                                                                          *mut auth_gssapi_init_res)
                                                                                                                                     ->
                                                                                                                                         libc::c_int)),
                                                                                     &mut call_res
                                                                                         as
                                                                                         *mut auth_gssapi_init_res
                                                                                         as
                                                                                         caddr_t);
                                                                *no_dispatch =
                                                                    1 as
                                                                        libc::c_int;
                                                                /*
	   * If appropriate, set established to TRUE *after* sending
	   * response (otherwise, the client will receive the final
	   * token encrypted)
	   */
                                                                if gssstat ==
                                                                       0 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint
                                                                   {
                                                                    gss_release_buffer(&mut minor_stat,
                                                                                       &mut call_res.signed_isn);
                                                                    (*client_data).established
                                                                        =
                                                                        1 as
                                                                            libc::c_int
                                                                }
                                                                gss_release_buffer(&mut minor_stat,
                                                                                   &mut output_token);
                                                                current_block
                                                                    =
                                                                    14693168353395310628;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
                                gssrpcint_printf(b"svcauth_gssapi: context is established\n\x00"
                                                     as *const u8 as
                                                     *const libc::c_char);
                            }
                            /* check the verifier */
                            if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
                                gssrpcint_printf(b"svcauth_gssapi: checking verifier, len %d\n\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 (*verf).oa_length);
                            }
                            in_buf.length = (*verf).oa_length as size_t;
                            in_buf.value =
                                (*verf).oa_base as *mut libc::c_void;
                            if gssrpc_auth_gssapi_unseal_seq((*client_data).context,
                                                             &mut in_buf,
                                                             &mut seq_num) ==
                                   0 as libc::c_int {
                                ret = AUTH_BADVERF;
                                if log_miscerr.is_some() {
                                    Some(log_miscerr.expect("non-null function pointer")).expect("non-null function pointer")(rqst,
                                                                                                                              msg,
                                                                                                                              b"internal error unsealing sequence number\x00"
                                                                                                                                  as
                                                                                                                                  *const u8
                                                                                                                                  as
                                                                                                                                  *const libc::c_char
                                                                                                                                  as
                                                                                                                                  *mut libc::c_char,
                                                                                                                              log_miscerr_data);
                                }
                                current_block = 15532589080399555063;
                            } else if seq_num !=
                                          (*client_data).seq_num.wrapping_add(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint)
                             {
                                if gssrpc_svc_debug_gssapi >=
                                       99 as libc::c_int {
                                    gssrpcint_printf(b"svcauth_gssapi: expected isn %d, got %d\n\x00"
                                                         as *const u8 as
                                                         *const libc::c_char,
                                                     (*client_data).seq_num.wrapping_add(1
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_uint),
                                                     seq_num);
                                }
                                if log_badverf.is_some() {
                                    Some(log_badverf.expect("non-null function pointer")).expect("non-null function pointer")((*client_data).client_name,
                                                                                                                              (*client_data).server_name,
                                                                                                                              rqst,
                                                                                                                              msg,
                                                                                                                              log_badverf_data);
                                }
                                ret = AUTH_REJECTEDVERF;
                                current_block = 15532589080399555063;
                            } else {
                                (*client_data).seq_num =
                                    (*client_data).seq_num.wrapping_add(1);
                                if gssrpc_svc_debug_gssapi >=
                                       99 as libc::c_int {
                                    gssrpcint_printf(b"svcauth_gssapi: seq_num %d okay\n\x00"
                                                         as *const u8 as
                                                         *const libc::c_char,
                                                     seq_num);
                                }
                                /* free previous response verifier, if any */
                                if (*client_data).prev_verf.length !=
                                       0 as libc::c_int as libc::c_ulong {
                                    gss_release_buffer(&mut minor_stat,
                                                       &mut (*client_data).prev_verf);
                                    (*client_data).prev_verf.length =
                                        0 as libc::c_int as size_t
                                }
                                /* prepare response verifier */
                                seq_num =
                                    (*client_data).seq_num.wrapping_add(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint);
                                if gssrpc_auth_gssapi_seal_seq((*client_data).context,
                                                               seq_num,
                                                               &mut out_buf)
                                       == 0 as libc::c_int {
                                    ret = AUTH_FAILED;
                                    if log_miscerr.is_some() {
                                        Some(log_miscerr.expect("non-null function pointer")).expect("non-null function pointer")(rqst,
                                                                                                                                  msg,
                                                                                                                                  b"internal error sealing sequence number\x00"
                                                                                                                                      as
                                                                                                                                      *const u8
                                                                                                                                      as
                                                                                                                                      *const libc::c_char
                                                                                                                                      as
                                                                                                                                      *mut libc::c_char,
                                                                                                                                  log_miscerr_data);
                                    }
                                    current_block = 15532589080399555063;
                                } else {
                                    (*client_data).seq_num =
                                        (*client_data).seq_num.wrapping_add(1);
                                    if gssrpc_svc_debug_gssapi >=
                                           99 as libc::c_int {
                                        gssrpcint_printf(b"svcauth_gssapi; response seq_num %d\n\x00"
                                                             as *const u8 as
                                                             *const libc::c_char,
                                                         seq_num);
                                    }
                                    (*(*rqst).rq_xprt).xp_verf.oa_flavor =
                                        300001 as libc::c_int;
                                    (*(*rqst).rq_xprt).xp_verf.oa_base =
                                        out_buf.value as caddr_t;
                                    (*(*rqst).rq_xprt).xp_verf.oa_length =
                                        out_buf.length as u_int;
                                    /* save verifier so it can be freed next time */
                                    (*client_data).prev_verf.value =
                                        out_buf.value;
                                    (*client_data).prev_verf.length =
                                        out_buf.length;
                                    /*
	   * Message is authentic.  If auth_msg if true, process the
	   * call; otherwise, return AUTH_OK so it will be dispatched
	   * to the application server.
	   */
                                    if creds.auth_msg == 1 as libc::c_int {
                                        /*
		* If process_token fails, then the token probably came
		* from an attacker.  No response (error or otherwise)
		* should be returned to the client, since it won't be
		* accepting one.
		*/
                                        match (*rqst).rq_proc {
                                            3 => {
                                                if gssrpc_svc_debug_gssapi >=
                                                       99 as libc::c_int {
                                                    gssrpcint_printf(b"svcauth_gssapi: GSSAPI_MSG, getting args\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char);
                                                }
                                                memset(&mut call_arg as
                                                           *mut auth_gssapi_init_arg
                                                           as
                                                           *mut libc::c_void,
                                                       0 as libc::c_int,
                                                       ::std::mem::size_of::<auth_gssapi_init_arg>()
                                                           as libc::c_ulong);
                                                if Some((*(*(*rqst).rq_xprt).xp_ops).xp_getargs.expect("non-null function pointer")).expect("non-null function pointer")((*rqst).rq_xprt,
                                                                                                                                                                         ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                                                                                                 *mut XDR,
                                                                                                                                                                                                                             _:
                                                                                                                                                                                                                                 *mut auth_gssapi_init_arg)
                                                                                                                                                                                                            ->
                                                                                                                                                                                                                libc::c_int>,
                                                                                                                                                                                                 xdrproc_t>(Some(gssrpc_xdr_authgssapi_init_arg
                                                                                                                                                                                                                     as
                                                                                                                                                                                                                     unsafe extern "C" fn(_:
                                                                                                                                                                                                                                              *mut XDR,
                                                                                                                                                                                                                                          _:
                                                                                                                                                                                                                                              *mut auth_gssapi_init_arg)
                                                                                                                                                                                                                         ->
                                                                                                                                                                                                                             libc::c_int)),
                                                                                                                                                                         &mut call_arg
                                                                                                                                                                             as
                                                                                                                                                                             *mut auth_gssapi_init_arg
                                                                                                                                                                             as
                                                                                                                                                                             *mut libc::c_void)
                                                       == 0 {
                                                    if gssrpc_svc_debug_gssapi
                                                           >=
                                                           99 as libc::c_int {
                                                        gssrpcint_printf(b"svcauth_gssapi: cannot decode args\n\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char);
                                                    }
                                                    if log_miscerr.is_some() {
                                                        Some(log_miscerr.expect("non-null function pointer")).expect("non-null function pointer")(rqst,
                                                                                                                                                  msg,
                                                                                                                                                  b"protocol error in call arguments\x00"
                                                                                                                                                      as
                                                                                                                                                      *const u8
                                                                                                                                                      as
                                                                                                                                                      *const libc::c_char
                                                                                                                                                      as
                                                                                                                                                      *mut libc::c_char,
                                                                                                                                                  log_miscerr_data);
                                                    }
                                                    gssrpc_xdr_free(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                            *mut XDR,
                                                                                                                        _:
                                                                                                                            *mut auth_gssapi_init_arg)
                                                                                                       ->
                                                                                                           libc::c_int>,
                                                                                            xdrproc_t>(Some(gssrpc_xdr_authgssapi_init_arg
                                                                                                                as
                                                                                                                unsafe extern "C" fn(_:
                                                                                                                                         *mut XDR,
                                                                                                                                     _:
                                                                                                                                         *mut auth_gssapi_init_arg)
                                                                                                                    ->
                                                                                                                        libc::c_int)),
                                                                    &mut call_arg
                                                                        as
                                                                        *mut auth_gssapi_init_arg
                                                                        as
                                                                        *mut libc::c_void);
                                                    ret = AUTH_BADCRED;
                                                    current_block =
                                                        15532589080399555063;
                                                } else {
                                                    if gssrpc_svc_debug_gssapi
                                                           >=
                                                           99 as libc::c_int {
                                                        gssrpcint_printf(b"svcauth_gssapi: processing token\n\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char);
                                                    }
                                                    gssstat =
                                                        gss_process_context_token(&mut minor_stat,
                                                                                  (*client_data).context,
                                                                                  &mut call_arg.token);
                                                    /* done with call args */
                                                    gssrpc_xdr_free(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                            *mut XDR,
                                                                                                                        _:
                                                                                                                            *mut auth_gssapi_init_arg)
                                                                                                       ->
                                                                                                           libc::c_int>,
                                                                                            xdrproc_t>(Some(gssrpc_xdr_authgssapi_init_arg
                                                                                                                as
                                                                                                                unsafe extern "C" fn(_:
                                                                                                                                         *mut XDR,
                                                                                                                                     _:
                                                                                                                                         *mut auth_gssapi_init_arg)
                                                                                                                    ->
                                                                                                                        libc::c_int)),
                                                                    &mut call_arg
                                                                        as
                                                                        *mut auth_gssapi_init_arg
                                                                        as
                                                                        *mut libc::c_void);
                                                    if gssstat !=
                                                           0 as libc::c_int as
                                                               libc::c_uint {
                                                        if gssrpc_svc_debug_gssapi
                                                               != 0 {
                                                            gssrpc_auth_gssapi_display_status(b"processing token\x00"
                                                                                                  as
                                                                                                  *const u8
                                                                                                  as
                                                                                                  *const libc::c_char
                                                                                                  as
                                                                                                  *mut libc::c_char,
                                                                                              gssstat,
                                                                                              minor_stat);
                                                        }
                                                        ret = AUTH_FAILED;
                                                        current_block =
                                                            15532589080399555063;
                                                    } else {
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
                                                                             0
                                                                                 as
                                                                                 caddr_t);
                                                        *no_dispatch =
                                                            1 as libc::c_int;
                                                        current_block =
                                                            14693168353395310628;
                                                    }
                                                }
                                            }
                                            4 => {
                                                if gssrpc_svc_debug_gssapi >=
                                                       99 as libc::c_int {
                                                    gssrpcint_printf(b"svcauth_gssapi: GSSAPI_DESTROY\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char);
                                                }
                                                if gssrpc_svc_debug_gssapi >=
                                                       99 as libc::c_int {
                                                    gssrpcint_printf(b"svcauth_gssapi: sending reply\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char);
                                                }
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
                                                                     0 as
                                                                         caddr_t);
                                                *no_dispatch =
                                                    1 as libc::c_int;
                                                destroy_client(client_data);
                                                (*(*rqst).rq_xprt).xp_auth =
                                                    0 as *mut SVCAUTH;
                                                current_block =
                                                    14693168353395310628;
                                            }
                                            _ => {
                                                if gssrpc_svc_debug_gssapi >=
                                                       99 as libc::c_int {
                                                    gssrpcint_printf(b"svcauth_gssapi: unacceptable procedure %d\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     (*rqst).rq_proc);
                                                }
                                                if log_miscerr.is_some() {
                                                    Some(log_miscerr.expect("non-null function pointer")).expect("non-null function pointer")(rqst,
                                                                                                                                              msg,
                                                                                                                                              b"invalid call procedure number\x00"
                                                                                                                                                  as
                                                                                                                                                  *const u8
                                                                                                                                                  as
                                                                                                                                                  *const libc::c_char
                                                                                                                                                  as
                                                                                                                                                  *mut libc::c_char,
                                                                                                                                              log_miscerr_data);
                                                }
                                                ret = AUTH_FAILED;
                                                current_block =
                                                    15532589080399555063;
                                            }
                                        }
                                    } else {
                                        /* set credentials for app server; comment in svc.c */
	       /* seems to imply this is incorrect, but I don't see */
	       /* any problem with it... */
                                        (*rqst).rq_clntcred =
                                            (*client_data).client_name as
                                                *mut libc::c_char as
                                                *mut libc::c_void;
                                        (*rqst).rq_svccred =
                                            (*client_data).context as
                                                *mut libc::c_char as
                                                *mut libc::c_void;
                                        current_block = 14693168353395310628;
                                    }
                                }
                            }
                        }
                        match current_block {
                            15532589080399555063 => { }
                            _ => {
                                if creds.client_handle.length !=
                                       0 as libc::c_int as libc::c_ulong {
                                    if gssrpc_svc_debug_gssapi >=
                                           99 as libc::c_int {
                                        gssrpcint_printf(b"svcauth_gssapi: freeing client_handle len %d\n\x00"
                                                             as *const u8 as
                                                             *const libc::c_char,
                                                         creds.client_handle.length
                                                             as libc::c_int);
                                    }
                                    gssrpc_xdr_free(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                            *mut XDR,
                                                                                                        _:
                                                                                                            *mut auth_gssapi_creds)
                                                                                       ->
                                                                                           libc::c_int>,
                                                                            xdrproc_t>(Some(gssrpc_xdr_authgssapi_creds
                                                                                                as
                                                                                                unsafe extern "C" fn(_:
                                                                                                                         *mut XDR,
                                                                                                                     _:
                                                                                                                         *mut auth_gssapi_creds)
                                                                                                    ->
                                                                                                        libc::c_int)),
                                                    &mut creds as
                                                        *mut auth_gssapi_creds
                                                        as *mut libc::c_void);
                                }
                                if gssrpc_svc_debug_gssapi >=
                                       99 as libc::c_int {
                                    gssrpcint_printf(b"\n\x00" as *const u8 as
                                                         *const libc::c_char);
                                }
                                return AUTH_OK
                            }
                        }
                    }
                }
            }
        }
    }
    if creds.client_handle.length != 0 as libc::c_int as libc::c_ulong {
        if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"svcauth_gssapi: freeing client_handle len %d\n\x00"
                                 as *const u8 as *const libc::c_char,
                             creds.client_handle.length as libc::c_int);
        }
        gssrpc_xdr_free(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut auth_gssapi_creds)
                                                           -> libc::c_int>,
                                                xdrproc_t>(Some(gssrpc_xdr_authgssapi_creds
                                                                    as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut XDR,
                                                                                         _:
                                                                                             *mut auth_gssapi_creds)
                                                                        ->
                                                                            libc::c_int)),
                        &mut creds as *mut auth_gssapi_creds as
                            *mut libc::c_void);
    }
    if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"\n\x00" as *const u8 as *const libc::c_char);
    }
    return ret;
}
#[c2rust::src_loc = "661:1"]
unsafe extern "C" fn cleanup() {
    let mut c: *mut client_list = 0 as *mut client_list;
    let mut c2: *mut client_list = 0 as *mut client_list;
    if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"cleanup_and_exit: starting\n\x00" as *const u8 as
                             *const libc::c_char);
    }
    c = clients;
    while !c.is_null() {
        c2 = c;
        c = (*c).next;
        destroy_client((*c2).client);
        free(c2 as *mut libc::c_void);
    }
    exit(0 as libc::c_int);
}
/*
 * Function: create_client
 *
 * Purpose: Creates an new client_data structure and stores it in the
 * database.
 *
 * Returns: the new client_data structure, or NULL on failure.
 *
 * Effects:
 *
 * A new client_data is created and stored in the hash table and
 * b-tree.  A new key that is unique in the current database is
 * chosen; this key should be used as the client's client_handle.
 */
#[c2rust::src_loc = "692:1"]
unsafe extern "C" fn create_client() -> *mut svc_auth_gssapi_data {
    let mut c: *mut client_list = 0 as *mut client_list;
    let mut client_data: *mut svc_auth_gssapi_data =
        0 as *mut svc_auth_gssapi_data;
    static mut client_key: libc::c_int = 1 as libc::c_int;
    if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"svcauth_gssapi: empty creds, creating\n\x00" as
                             *const u8 as *const libc::c_char);
    }
    client_data =
        malloc(::std::mem::size_of::<svc_auth_gssapi_data>() as libc::c_ulong)
            as *mut svc_auth_gssapi_data;
    if client_data.is_null() { return 0 as *mut svc_auth_gssapi_data }
    memset(client_data as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<svc_auth_gssapi_data>() as libc::c_ulong);
    if gssrpc_svc_debug_gssapi >= 2 as libc::c_int {
        gssrpcint_printf(b"create_client: new client_data = %p\n\x00" as
                             *const u8 as *const libc::c_char,
                         client_data as *mut libc::c_void);
    }
    /* set up client data structure */
    (*client_data).established = 0 as libc::c_int;
    (*client_data).context = 0 as gss_ctx_id_t;
    (*client_data).expiration =
        (time(0 as *mut time_t) +
             (60 as libc::c_int * 15 as libc::c_int) as libc::c_long) as
            uint32_t;
    /* set up psycho-recursive SVCAUTH hack */
    (*client_data).svcauth.svc_ah_ops = &mut gssrpc_svc_auth_gssapi_ops;
    (*client_data).svcauth.svc_ah_private =
        client_data as caddr_t as *mut libc::c_void;
    let fresh0 = client_key;
    client_key = client_key + 1;
    (*client_data).key = fresh0 as uint32_t;
    c =
        malloc(::std::mem::size_of::<client_list>() as libc::c_ulong) as
            *mut client_list;
    if c.is_null() { return 0 as *mut svc_auth_gssapi_data }
    (*c).client = client_data;
    (*c).next = 0 as *mut _client_list;
    if clients.is_null() {
        clients = c
    } else { (*c).next = clients; clients = c }
    if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"svcauth_gssapi: new handle %d\n\x00" as *const u8
                             as *const libc::c_char, (*client_data).key);
    }
    if gssrpc_svc_debug_gssapi >= 2 as libc::c_int {
        gssrpcint_printf(b"create_client: done\n\x00" as *const u8 as
                             *const libc::c_char);
    }
    return client_data;
}
/*
 * Function: client_expire
 *
 * Purpose: change the expiration time of a client in the database
 *
 * Arguments:
 *
 * 	client_data	(r) the client_data to expire
 * 	exp		(r) the new expiration time
 *
 * Effects:
 *
 * client_data->expiration = exp
 *
 * This function used to remove client_data from the database, change
 * its expiration time, and re-add it, which was necessary because the
 * database was sorted by expiration time so a simple modification
 * would break the rep invariant.  Now the database is an unsorted
 * linked list, so it doesn't matter.
 */
#[c2rust::src_loc = "758:1"]
unsafe extern "C" fn client_expire(mut client_data: *mut svc_auth_gssapi_data,
                                   mut exp: uint32_t) {
    (*client_data).expiration = exp;
}
/*
 * Function get_client
 *
 * Purpose: retrieve a client_data structure from the database based
 * on its client handle (key)
 *
 * Arguments:
 *
 *	client_handle	(r) the handle (key) to retrieve
 *
 * Effects:
 *
 * Searches the list and returns the client_data whose key field
 * matches the contents of client_handle, or returns NULL if none was
 * found.
 */
#[c2rust::src_loc = "781:1"]
unsafe extern "C" fn get_client(mut client_handle: gss_buffer_t)
 -> *mut svc_auth_gssapi_data {
    let mut c: *mut client_list = 0 as *mut client_list;
    let mut handle: uint32_t = 0;
    memcpy(&mut handle as *mut uint32_t as *mut libc::c_void,
           (*client_handle).value, 4 as libc::c_int as libc::c_ulong);
    if gssrpc_svc_debug_gssapi >= 2 as libc::c_int {
        gssrpcint_printf(b"get_client: looking for client %d\n\x00" as
                             *const u8 as *const libc::c_char, handle);
    }
    c = clients;
    while !c.is_null() {
        if (*(*c).client).key == handle { return (*c).client }
        c = (*c).next
    }
    if gssrpc_svc_debug_gssapi >= 2 as libc::c_int {
        gssrpcint_printf(b"get_client: client_handle lookup failed\n\x00" as
                             *const u8 as *const libc::c_char);
    }
    return 0 as *mut svc_auth_gssapi_data;
}
/*
 * Function: destroy_client
 *
 * Purpose: destroys a client entry and removes it from the database
 *
 * Arguments:
 *
 *	client_data	(r) the client to be destroyed
 *
 * Effects:
 *
 * client_data->context is deleted with gss_delete_sec_context.
 * client_data's entry in the database is destroyed.  client_data is
 * freed.
 */
#[c2rust::src_loc = "816:1"]
unsafe extern "C" fn destroy_client(mut client_data:
                                        *mut svc_auth_gssapi_data) {
    let mut current_block: u64;
    let mut gssstat: OM_uint32 = 0;
    let mut minor_stat: OM_uint32 = 0;
    let mut out_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut c: *mut client_list = 0 as *mut client_list;
    let mut c2: *mut client_list = 0 as *mut client_list;
    if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"destroy_client: destroying client_data\n\x00" as
                             *const u8 as *const libc::c_char);
    }
    if gssrpc_svc_debug_gssapi >= 2 as libc::c_int {
        gssrpcint_printf(b"destroy_client: client_data = %p\n\x00" as
                             *const u8 as *const libc::c_char,
                         client_data as *mut libc::c_void);
    }
    if gssrpc_svc_debug_gssapi >= 3 as libc::c_int {
        dump_db(b"before frees\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char);
    }
    /* destroy client struct even if error occurs */
    gssstat =
        gss_delete_sec_context(&mut minor_stat, &mut (*client_data).context,
                               &mut out_buf);
    if gssstat != 0 as libc::c_int as libc::c_uint {
        if gssrpc_svc_debug_gssapi != 0 {
            gssrpc_auth_gssapi_display_status(b"deleting context\x00" as
                                                  *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char, gssstat,
                                              minor_stat);
        }
    }
    gss_release_buffer(&mut minor_stat, &mut out_buf);
    gss_release_name(&mut minor_stat, &mut (*client_data).client_name);
    if (*client_data).prev_verf.length != 0 as libc::c_int as libc::c_ulong {
        gss_release_buffer(&mut minor_stat, &mut (*client_data).prev_verf);
    }
    if clients.is_null() {
        if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"destroy_client: called on empty database\n\x00"
                                 as *const u8 as *const libc::c_char);
        }
        abort();
    } else {
        if (*clients).client == client_data {
            c = clients;
            clients = (*clients).next;
            free(c as *mut libc::c_void);
        } else {
            c2 = clients;
            c = (*clients).next;
            loop  {
                if c.is_null() {
                    current_block = 2569451025026770673;
                    break ;
                }
                if (*c).client == client_data {
                    (*c2).next = (*c).next;
                    free(c as *mut libc::c_void);
                    current_block = 17450855096986142212;
                    break ;
                } else { c2 = c; c = (*c).next }
            }
            match current_block {
                17450855096986142212 => { }
                _ => {
                    if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
                        gssrpcint_printf(b"destroy_client: client_handle delete failed\n\x00"
                                             as *const u8 as
                                             *const libc::c_char);
                    }
                    abort();
                }
            }
        }
        if gssrpc_svc_debug_gssapi >= 2 as libc::c_int {
            gssrpcint_printf(b"destroy_client: client %d destroyed\n\x00" as
                                 *const u8 as *const libc::c_char,
                             (*client_data).key);
        }
        free(client_data as *mut libc::c_void);
        return;
    };
}
#[c2rust::src_loc = "874:1"]
unsafe extern "C" fn dump_db(mut msg: *mut libc::c_char) {
    let mut client_data: *mut svc_auth_gssapi_data =
        0 as *mut svc_auth_gssapi_data;
    let mut c: *mut client_list = 0 as *mut client_list;
    if gssrpc_svc_debug_gssapi >= 3 as libc::c_int {
        gssrpcint_printf(b"dump_db: %s:\n\x00" as *const u8 as
                             *const libc::c_char, msg);
    }
    c = clients;
    while !c.is_null() {
        client_data = (*c).client;
        if gssrpc_svc_debug_gssapi >= 3 as libc::c_int {
            gssrpcint_printf(b"\tclient_data = %p, exp = %d\n\x00" as
                                 *const u8 as *const libc::c_char,
                             client_data as *mut libc::c_void,
                             (*client_data).expiration);
        }
        c = (*c).next
    }
    if gssrpc_svc_debug_gssapi >= 3 as libc::c_int {
        gssrpcint_printf(b"\n\x00" as *const u8 as *const libc::c_char);
    };
}
#[c2rust::src_loc = "892:1"]
unsafe extern "C" fn clean_client() {
    let mut client_data: *mut svc_auth_gssapi_data =
        0 as *mut svc_auth_gssapi_data;
    let mut c: *mut client_list = 0 as *mut client_list;
    if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"clean_client: starting\n\x00" as *const u8 as
                             *const libc::c_char);
    }
    c = clients;
    while !c.is_null() {
        client_data = (*c).client;
        if gssrpc_svc_debug_gssapi >= 2 as libc::c_int {
            gssrpcint_printf(b"clean_client: client_data = %p\n\x00" as
                                 *const u8 as *const libc::c_char,
                             client_data as *mut libc::c_void);
        }
        if ((*client_data).expiration as libc::c_long) <
               time(0 as *mut time_t) {
            if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
                gssrpcint_printf(b"clean_client: client %d expired\n\x00" as
                                     *const u8 as *const libc::c_char,
                                 (*client_data).key);
            }
            destroy_client(client_data);
            c = clients
            /* start over, just to be safe */
        } else { c = (*c).next }
    }
    if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"clean_client: done\n\x00" as *const u8 as
                             *const libc::c_char);
    };
}
/*
 * Function: svcauth_gssapi_set_names
 *
 * Purpose: Sets the list of service names for which incoming
 * authentication requests should be honored.
 *
 * See functional specifications.
 */
#[no_mangle]
#[c2rust::src_loc = "927:1"]
pub unsafe extern "C" fn gssrpc_svcauth_gssapi_set_names(mut names:
                                                             *mut auth_gssapi_name,
                                                         mut num: libc::c_int)
 -> libc::c_int {
    let mut current_block: u64;
    let mut gssstat: OM_uint32 = 0;
    let mut minor_stat: OM_uint32 = 0;
    let mut in_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut i: libc::c_int = 0;
    if num == 0 as libc::c_int {
        while !(*names.offset(num as isize)).name.is_null() { num += 1 }
    }
    server_creds_list = 0 as *mut gss_cred_id_t;
    server_name_list = 0 as *mut gss_name_t;
    server_creds_list =
        malloc((num as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<gss_cred_id_t>()
                                                    as libc::c_ulong)) as
            *mut gss_cred_id_t;
    if !server_creds_list.is_null() {
        server_name_list =
            malloc((num as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<gss_name_t>()
                                                        as libc::c_ulong)) as
                *mut gss_name_t;
        if !server_name_list.is_null() {
            i = 0 as libc::c_int;
            while i < num {
                let ref mut fresh1 = *server_name_list.offset(i as isize);
                *fresh1 = 0 as gss_name_t;
                let ref mut fresh2 = *server_creds_list.offset(i as isize);
                *fresh2 = 0 as gss_cred_id_t;
                i += 1
            }
            server_creds_count = num;
            i = 0 as libc::c_int;
            loop  {
                if !(i < num) { current_block = 2569451025026770673; break ; }
                in_buf.value =
                    (*names.offset(i as isize)).name as *mut libc::c_void;
                in_buf.length =
                    strlen(in_buf.value as
                               *const libc::c_char).wrapping_add(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong);
                if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
                    gssrpcint_printf(b"svcauth_gssapi_set_names: importing %s\n\x00"
                                         as *const u8 as *const libc::c_char,
                                     (*names.offset(i as isize)).name);
                }
                gssstat =
                    gss_import_name(&mut minor_stat, &mut in_buf,
                                    (*names.offset(i as isize)).type_0,
                                    &mut *server_name_list.offset(i as
                                                                      isize));
                if gssstat != 0 as libc::c_int as libc::c_uint {
                    if gssrpc_svc_debug_gssapi != 0 {
                        gssrpc_auth_gssapi_display_status(b"importing name\x00"
                                                              as *const u8 as
                                                              *const libc::c_char
                                                              as
                                                              *mut libc::c_char,
                                                          gssstat,
                                                          minor_stat);
                    }
                    current_block = 7057233496068045865;
                    break ;
                } else {
                    gssstat =
                        gss_acquire_cred(&mut minor_stat,
                                         *server_name_list.offset(i as isize),
                                         0 as libc::c_int as OM_uint32,
                                         0 as gss_OID_set, 2 as libc::c_int,
                                         &mut *server_creds_list.offset(i as
                                                                            isize),
                                         0 as *mut gss_OID_set,
                                         0 as *mut OM_uint32);
                    if gssstat != 0 as libc::c_int as libc::c_uint {
                        if gssrpc_svc_debug_gssapi != 0 {
                            gssrpc_auth_gssapi_display_status(b"acquiring credentials\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char
                                                                  as
                                                                  *mut libc::c_char,
                                                              gssstat,
                                                              minor_stat);
                        }
                        current_block = 7057233496068045865;
                        break ;
                    } else { i += 1 }
                }
            }
            match current_block {
                7057233496068045865 => { }
                _ => { return 1 as libc::c_int }
            }
        }
    }
    gssrpc_svcauth_gssapi_unset_names();
    return 0 as libc::c_int;
}
/* Function: svcauth_gssapi_unset_names
 *
 * Purpose: releases the names and credentials allocated by
 * svcauth_gssapi_set_names
 */
#[no_mangle]
#[c2rust::src_loc = "995:1"]
pub unsafe extern "C" fn gssrpc_svcauth_gssapi_unset_names() {
    let mut i: libc::c_int = 0;
    let mut minor_stat: OM_uint32 = 0;
    if !server_creds_list.is_null() {
        i = 0 as libc::c_int;
        while i < server_creds_count {
            if !(*server_creds_list.offset(i as isize)).is_null() {
                gss_release_cred(&mut minor_stat,
                                 &mut *server_creds_list.offset(i as isize));
            }
            i += 1
        }
        free(server_creds_list as *mut libc::c_void);
        server_creds_list = 0 as *mut gss_cred_id_t
    }
    if !server_name_list.is_null() {
        i = 0 as libc::c_int;
        while i < server_creds_count {
            if !(*server_name_list.offset(i as isize)).is_null() {
                gss_release_name(&mut minor_stat,
                                 &mut *server_name_list.offset(i as isize));
            }
            i += 1
        }
        free(server_name_list as *mut libc::c_void);
        server_name_list = 0 as *mut gss_name_t
    }
    server_creds_count = 0 as libc::c_int;
}
/*
 * Function: svcauth_gssapi_set_log_badauth_func
 *
 * Purpose: sets the logging function called when an invalid RPC call
 * arrives
 *
 * See functional specifications.
 */
#[no_mangle]
#[c2rust::src_loc = "1027:1"]
pub unsafe extern "C" fn gssrpc_svcauth_gssapi_set_log_badauth_func(mut func:
                                                                        auth_gssapi_log_badauth_func,
                                                                    mut data:
                                                                        caddr_t) {
    log_badauth = func;
    log_badauth_data = data;
}
#[no_mangle]
#[c2rust::src_loc = "1035:1"]
pub unsafe extern "C" fn gssrpc_svcauth_gssapi_set_log_badauth2_func(mut func:
                                                                         auth_gssapi_log_badauth2_func,
                                                                     mut data:
                                                                         caddr_t) {
    log_badauth2 = func;
    log_badauth2_data = data;
}
/*
 * Function: svcauth_gssapi_set_log_badverf_func
 *
 * Purpose: sets the logging function called when an invalid RPC call
 * arrives
 *
 * See functional specifications.
 */
#[no_mangle]
#[c2rust::src_loc = "1051:1"]
pub unsafe extern "C" fn gssrpc_svcauth_gssapi_set_log_badverf_func(mut func:
                                                                        auth_gssapi_log_badverf_func,
                                                                    mut data:
                                                                        caddr_t) {
    log_badverf = func;
    log_badverf_data = data;
}
/*
 * Function: svcauth_gssapi_set_log_miscerr_func
 *
 * Purpose: sets the logging function called when a miscellaneous
 * AUTH_GSSAPI error occurs
 *
 * See functional specifications.
 */
#[no_mangle]
#[c2rust::src_loc = "1067:1"]
pub unsafe extern "C" fn gssrpc_svcauth_gssapi_set_log_miscerr_func(mut func:
                                                                        auth_gssapi_log_miscerr_func,
                                                                    mut data:
                                                                        caddr_t) {
    log_miscerr = func;
    log_miscerr_data = data;
}
/*
 * Encrypt the serialized arguments from xdr_func applied to xdr_ptr
 * and write the result to xdrs.
 */
#[c2rust::src_loc = "1079:1"]
unsafe extern "C" fn svc_auth_gssapi_wrap(mut auth: *mut SVCAUTH,
                                          mut out_xdrs: *mut XDR,
                                          mut xdr_func:
                                              Option<unsafe extern "C" fn()
                                                         -> libc::c_int>,
                                          mut xdr_ptr: caddr_t)
 -> libc::c_int {
    let mut gssstat: OM_uint32 = 0;
    let mut minor_stat: OM_uint32 = 0;
    if (*((*auth).svc_ah_private as *mut svc_auth_gssapi_data)).established ==
           0 {
        if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"svc_gssapi_wrap: not established, noop\n\x00"
                                 as *const u8 as *const libc::c_char);
        }
        return ::std::mem::transmute::<_,
                                       fn(_: _, _: _)
                                           ->
                                               libc::c_int>(Some(xdr_func.expect("non-null function pointer")).expect("non-null function pointer"))(out_xdrs,
                                                                                                                                                    xdr_ptr)
    } else if gssrpc_auth_gssapi_wrap_data(&mut gssstat, &mut minor_stat,
                                           (*((*auth).svc_ah_private as
                                                  *mut svc_auth_gssapi_data)).context,
                                           (*((*auth).svc_ah_private as
                                                  *mut svc_auth_gssapi_data)).seq_num,
                                           out_xdrs, xdr_func, xdr_ptr) == 0 {
        if gssstat != 0 as libc::c_int as libc::c_uint {
            if gssrpc_svc_debug_gssapi != 0 {
                gssrpc_auth_gssapi_display_status(b"encrypting function arguments\x00"
                                                      as *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char,
                                                  gssstat, minor_stat);
            }
        }
        return 0 as libc::c_int
    } else { return 1 as libc::c_int };
}
#[c2rust::src_loc = "1102:1"]
unsafe extern "C" fn svc_auth_gssapi_unwrap(mut auth: *mut SVCAUTH,
                                            mut in_xdrs: *mut XDR,
                                            mut xdr_func:
                                                Option<unsafe extern "C" fn()
                                                           -> libc::c_int>,
                                            mut xdr_ptr: caddr_t)
 -> libc::c_int {
    let mut client_data: *mut svc_auth_gssapi_data =
        (*auth).svc_ah_private as *mut svc_auth_gssapi_data;
    let mut gssstat: OM_uint32 = 0;
    let mut minor_stat: OM_uint32 = 0;
    if (*client_data).established == 0 {
        if gssrpc_svc_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"svc_gssapi_unwrap: not established, noop\n\x00"
                                 as *const u8 as *const libc::c_char);
        }
        return ::std::mem::transmute::<_,
                                       fn(_: _, _: _)
                                           ->
                                               libc::c_int>(Some(xdr_func.expect("non-null function pointer")).expect("non-null function pointer"))(in_xdrs,
                                                                                                                                                    xdr_ptr
                                                                                                                                                        as
                                                                                                                                                        *mut libc::c_void
                                                                                                                                                        as
                                                                                                                                                        *mut auth_gssapi_init_arg)
    } else if gssrpc_auth_gssapi_unwrap_data(&mut gssstat, &mut minor_stat,
                                             (*client_data).context,
                                             (*client_data).seq_num.wrapping_sub(1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint),
                                             in_xdrs, xdr_func, xdr_ptr) == 0
     {
        if gssstat != 0 as libc::c_int as libc::c_uint {
            if gssrpc_svc_debug_gssapi != 0 {
                gssrpc_auth_gssapi_display_status(b"decrypting function arguments\x00"
                                                      as *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char,
                                                  gssstat, minor_stat);
            }
        }
        return 0 as libc::c_int
    } else { return 1 as libc::c_int };
}
#[c2rust::src_loc = "1126:1"]
unsafe extern "C" fn svc_auth_gssapi_destroy(mut auth: *mut SVCAUTH)
 -> libc::c_int {
    let mut client_data: *mut svc_auth_gssapi_data =
        (*auth).svc_ah_private as *mut svc_auth_gssapi_data;
    destroy_client(client_data);
    return 1 as libc::c_int;
}
