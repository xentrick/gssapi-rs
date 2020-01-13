use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "32:1"]
    pub type __u_short = libc::c_ushort;
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "34:1"]
    pub type __u_long = libc::c_ulong;
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
    #[c2rust::src_loc = "154:1"]
    pub type __pid_t = libc::c_int;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:33"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:33"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/sys/types.h:33"]
pub mod sys_types_h {
    #[c2rust::src_loc = "34:1"]
    pub type u_short = __u_short;
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "36:1"]
    pub type u_long = __u_long;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_short, __u_int, __u_long, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:33"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:33"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:44"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/netinet/in.h:46"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:48"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:48"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth.h:48"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc_msg.h:48"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:48"]
pub mod gssapi_h {
    #[c2rust::src_loc = "79:1"]
    pub type gss_name_t = *mut gss_name_struct;
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
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_desc = gss_buffer_desc_struct;
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "78:8"]
        pub type gss_name_struct;
        #[no_mangle]
        #[c2rust::src_loc = "530:1"]
        pub fn gss_display_status(_: *mut OM_uint32, _: OM_uint32,
                                  _: libc::c_int, _: gss_OID,
                                  _: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "554:1"]
        pub fn gss_display_name(_: *mut OM_uint32, _: gss_name_t,
                                _: gss_buffer_t, _: *mut gss_OID)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "562:1"]
        pub fn gss_import_name(_: *mut OM_uint32, _: gss_buffer_t, _: gss_OID,
                               _: *mut gss_name_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "569:1"]
        pub fn gss_release_name(_: *mut OM_uint32, _: *mut gss_name_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "574:1"]
        pub fn gss_release_buffer(_: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/svc.h:48"]
pub mod svc_h {
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
    /* !defined(GSSRPC_SVC_H) */
    /* XXX add auth_gsapi_log_*? */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/svc_auth.h:48"]
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
    use super::gssapi_h::{gss_name_struct, gss_name_t};
    extern "C" {
        /*
 * Approved way of setting server principal
 */
        #[no_mangle]
        #[c2rust::src_loc = "115:1"]
        pub fn gssrpc_svcauth_gss_set_svc_name(name: gss_name_t)
         -> libc::c_int;
    }
    /* !defined(GSSRPC_SVC_AUTH_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:50"]
pub mod krb5_h {
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
    #[c2rust::src_loc = "8510:1"]
    pub type krb5_post_recv_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: krb5_error_code, _: *const krb5_data,
                                    _: *const krb5_data, _: *const krb5_data,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    /* per Kerberos v5 protocol spec */
    /* not yet in the spec... */
    /* macros to determine if a type is a local type */
    /*
 * end "hostaddr.h"
 */
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
    #[c2rust::src_loc = "8475:1"]
    pub type krb5_pre_send_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: *const krb5_data, _: *const krb5_data,
                                    _: *mut *mut krb5_data,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "8391:1"]
    pub type krb5_trace_callback
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *const krb5_trace_info,
                                    _: *mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "8387:1"]
    pub type krb5_trace_info = _krb5_trace_info;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8387:16"]
    pub struct _krb5_trace_info {
        pub message: *const libc::c_char,
    }
    #[c2rust::src_loc = "7865:1"]
    pub type krb5_prompt_type = krb5_int32;
    use super::stdint_intn_h::int32_t;
    use super::stdint_uintn_h::uint32_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
        /* This file is generated, please don't edit it directly.  */
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* General definitions for Kerberos version 5. */
/*
 * Copyright 1989, 1990, 1995, 2001, 2003, 2007, 2011 by the Massachusetts
 * Institute of Technology.  All Rights Reserved.
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
        /* * @defgroup KRB5_H krb5 library API
 * @{
 */
        /* By default, do not expose deprecated interfaces. */
        /* !KRB5_CONFIG__ */
        /* for *_MAX */
        /* from profile.h */
        #[c2rust::src_loc = "125:8"]
        pub type _profile_t;
        /* *
 * Collect entropy from the OS if possible.
 *
 * @param [in]  context         Library context
 * @param [in]  strong          Strongest available source of entropy
 * @param [out] success         1 if OS provides entropy, 0 otherwise
 *
 * If @a strong is non-zero, this function attempts to use the strongest
 * available source of entropy.  Setting this flag may cause the function to
 * block on some operating systems.  Good uses include seeding the PRNG for
 * kadmind and realm setup.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "817:1"]
        pub fn krb5_c_random_os_entropy(context_0: krb5_context,
                                        strong: libc::c_int,
                                        success: *mut libc::c_int)
         -> krb5_error_code;
        /* *
 * Free a krb5 library context.
 *
 * @param [in] context          Library context
 *
 * This function frees a @a context that was created by krb5_init_context()
 * or krb5_init_secure_context().
 */
        #[no_mangle]
        #[c2rust::src_loc = "2972:1"]
        pub fn krb5_free_context(context_0: krb5_context);
        /* *
 * Get the (possibly extended) error message for a code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 *
 * The behavior of krb5_get_error_message() is only defined the first time it
 * is called after a failed call to a krb5 function using the same context, and
 * only when the error code passed in is the same as that returned by the krb5
 * function.
 *
 * This function never returns NULL, so its result may be used unconditionally
 * as a C string.
 *
 * The string returned by this function must be freed using
 * krb5_free_error_message()
 *
 * @note Future versions may return the same string for the second
 * and following calls.
 */
        #[no_mangle]
        #[c2rust::src_loc = "8023:1"]
        pub fn krb5_get_error_message(ctx: krb5_context,
                                      code: krb5_error_code)
         -> *const libc::c_char;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:50"]
pub mod k5_int_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1208:8"]
    pub struct _krb5_context {
        pub magic: krb5_magic,
        pub in_tkt_etypes: *mut krb5_enctype,
        pub tgs_etypes: *mut krb5_enctype,
        pub os_context: _krb5_os_context,
        pub default_realm: *mut libc::c_char,
        pub profile: profile_t,
        pub dal_handle: *mut kdb5_dal_handle,
        pub clockskew: krb5_deltat,
        pub kdc_default_options: krb5_flags,
        pub library_options: krb5_flags,
        pub profile_secure: krb5_boolean,
        pub fcc_default_format: libc::c_int,
        pub prompt_types: *mut krb5_prompt_type,
        pub udp_pref_limit: libc::c_int,
        pub use_conf_ktypes: krb5_boolean,
        pub libkrb5_plugins: plugin_dir_handle,
        pub preauth_context: krb5_preauth_context,
        pub ccselect_handles: *mut *mut ccselect_module_handle,
        pub localauth_handles: *mut *mut localauth_module_handle,
        pub hostrealm_handles: *mut *mut hostrealm_module_handle,
        pub tls: *mut k5_tls_vtable_st,
        pub err: errinfo,
        pub err_fmt: *mut libc::c_char,
        pub kdblog_context: *mut _kdb_log_context,
        pub allow_weak_crypto: krb5_boolean,
        pub ignore_acceptor_hostname: krb5_boolean,
        pub enforce_ok_as_delegate: krb5_boolean,
        pub dns_canonicalize_hostname: dns_canonhost,
        pub trace_callback: krb5_trace_callback,
        pub trace_callback_data: *mut libc::c_void,
        pub kdc_send_hook: krb5_pre_send_fn,
        pub kdc_send_hook_data: *mut libc::c_void,
        pub kdc_recv_hook: krb5_post_recv_fn,
        pub kdc_recv_hook_data: *mut libc::c_void,
        pub plugins: [plugin_interface; 13],
        pub plugin_base_dir: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1137:8"]
    pub struct plugin_interface {
        pub modules: *mut *mut plugin_mapping,
        pub configured: krb5_boolean,
    }
    #[c2rust::src_loc = "1194:1"]
    pub type dns_canonhost = libc::c_uint;
    #[c2rust::src_loc = "1197:5"]
    pub const CANONHOST_FALLBACK: dns_canonhost = 2;
    #[c2rust::src_loc = "1196:5"]
    pub const CANONHOST_TRUE: dns_canonhost = 1;
    #[c2rust::src_loc = "1195:5"]
    pub const CANONHOST_FALSE: dns_canonhost = 0;
    #[c2rust::src_loc = "1203:1"]
    pub type krb5_preauth_context = *mut krb5_preauth_context_st;
    #[c2rust::src_loc = "1201:1"]
    pub type kdb5_dal_handle = _kdb5_dal_handle;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "702:16"]
    pub struct _krb5_os_context {
        pub magic: krb5_magic,
        pub time_offset: krb5_int32,
        pub usec_offset: krb5_int32,
        pub os_flags: krb5_int32,
        pub default_ccname: *mut libc::c_char,
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::kdb_log_h::_kdb_log_context;
    extern "C" {
        #[c2rust::src_loc = "1134:8"]
        pub type plugin_mapping;
        #[c2rust::src_loc = "1207:8"]
        pub type k5_tls_vtable_st;
        #[c2rust::src_loc = "1206:8"]
        pub type hostrealm_module_handle;
        #[c2rust::src_loc = "1205:8"]
        pub type localauth_module_handle;
        #[c2rust::src_loc = "1204:8"]
        pub type ccselect_module_handle;
        #[c2rust::src_loc = "1203:16"]
        pub type krb5_preauth_context_st;
        #[c2rust::src_loc = "1200:8"]
        pub type _kdb5_dal_handle;
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kdb_log.h:57"]
pub mod kdb_log_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2004 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
    /* #pragma ident        "@(#)kdb_log.h  1.3     04/02/23 SMI" */
    /*
 * DB macros
 */
    /*
 * Current DB version #
 */
    /*
 * DB log states
 */
    /*
 * DB log constants
 */
    /*
 * Default ulog file attributes
 */
    /* in seconds */
    /*
 * Max size of update entry + update header
 * We make this large since resizing can be costly.
 */
    /* Default size of principal record */
    /* 256 MB log file */
    /*
 * Prototype declarations
 */
    /* Log header magic # */
    /* Kerberos database version no. */
    /* # of updates in log */
    /* Timestamp of first update */
    /* Timestamp of last update */
    /* First serial # in the update log */
    /* Last serial # in the update log */
    /* State of update log */
    /* Block size of each element */
    /* Update entry magic # */
    /* Serial # of entry */
    /* Timestamp of update */
    /* Is the entry committed or not */
    /* Size of update entry */
    /* Address of kdb_incr_update_t */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "102:16"]
    pub struct _kdb_log_context {
        pub iproprole: iprop_role,
        pub ulog: *mut kdb_hlog_t,
        pub ulogentries: uint32_t,
        pub ulogfd: libc::c_int,
    }
    #[c2rust::src_loc = "81:1"]
    pub type kdb_hlog_t = kdb_hlog;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "81:16"]
    pub struct kdb_hlog {
        pub kdb_hmagic: uint32_t,
        pub db_version_num: uint16_t,
        pub kdb_num: uint32_t,
        pub kdb_first_time: kdbe_time_t,
        pub kdb_last_time: kdbe_time_t,
        pub kdb_first_sno: kdb_sno_t,
        pub kdb_last_sno: kdb_sno_t,
        pub kdb_state: uint16_t,
        pub kdb_block: uint16_t,
    }
    use super::iprop_hdr_h::{iprop_role, IPROP_NULL};
    use super::stdint_uintn_h::{uint32_t, uint16_t};
    use super::iprop_h::{kdbe_time_t, kdb_sno_t};
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_error_code};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "61:1"]
        pub fn ulog_map(context_0: krb5_context, logname: *const libc::c_char,
                        entries: uint32_t) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn ulog_set_role(ctx: krb5_context, role: iprop_role)
         -> krb5_error_code;
    }
    /* !_KDB_LOG_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop.h:57"]
pub mod iprop_h {
    #[c2rust::src_loc = "22:1"]
    pub type kdb_sno_t = uint32_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:8"]
    pub struct kdbe_time_t {
        pub seconds: uint32_t,
        pub useconds: uint32_t,
    }
    use super::stdint_uintn_h::uint32_t;
    /* !_IPROP_H_RPCGEN */
    /* K&R C */
    /* K&R C */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop_hdr.h:57"]
pub mod iprop_hdr_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2004 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
    /* #pragma ident        "@(#)iprop_hdr.h        1.1     04/02/20 SMI" */
    /*
 * This file has some defines common to the iprop client and
 * server routines.
 */
    /*
 * Maximum size for each ulog entry is 2KB and maximum
 * possible attribute-value pairs for each ulog entry is 20
 */
    /* Backoff for a maximum for 5 mts */
    #[c2rust::src_loc = "32:1"]
    pub type iprop_role = libc::c_uint;
    #[c2rust::src_loc = "35:5"]
    pub const IPROP_REPLICA: iprop_role = 2;
    #[c2rust::src_loc = "34:5"]
    pub const IPROP_MASTER: iprop_role = 1;
    #[c2rust::src_loc = "33:5"]
    pub const IPROP_NULL: iprop_role = 0;
    /* !_IPROP_HDR_H */
    /*
 * Full resync dump versioning
 */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:50"]
pub mod k5_err_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:8"]
    pub struct errinfo {
        pub code: libc::c_long,
        pub msg: *mut libc::c_char,
    }
    /* K5_ERR_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:50"]
pub mod k5_plugin_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:8"]
    pub struct plugin_dir_handle {
        pub files: *mut *mut plugin_file_handle,
    }
    extern "C" {
        #[c2rust::src_loc = "80:8"]
        pub type plugin_file_handle;
    }
    /* K5_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:50"]
pub mod profile_h {
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:50"]
pub mod com_err_h {
    /*
 * Copyright 1988, Student Information Processing Board of the
 * Massachusetts Institute of Technology.
 *
 * Copyright 1995 by Cygnus Support.
 *
 * For copyright and distribution info, see the documentation supplied
 * with this package.
 */
    /* Header file for common error description library. */
    #[c2rust::src_loc = "26:1"]
    pub type errcode_t = libc::c_long;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "54:1"]
        pub fn error_message(_: errcode_t) -> *const libc::c_char;
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/verto.h:59"]
pub mod verto_h {
    #[c2rust::src_loc = "51:9"]
    pub type verto_ev_type = libc::c_uint;
    #[c2rust::src_loc = "57:5"]
    pub const VERTO_EV_TYPE_CHILD: verto_ev_type = 16;
    #[c2rust::src_loc = "56:5"]
    pub const VERTO_EV_TYPE_SIGNAL: verto_ev_type = 8;
    #[c2rust::src_loc = "55:5"]
    pub const VERTO_EV_TYPE_IDLE: verto_ev_type = 4;
    #[c2rust::src_loc = "54:5"]
    pub const VERTO_EV_TYPE_TIMEOUT: verto_ev_type = 2;
    #[c2rust::src_loc = "53:5"]
    pub const VERTO_EV_TYPE_IO: verto_ev_type = 1;
    #[c2rust::src_loc = "52:5"]
    pub const VERTO_EV_TYPE_NONE: verto_ev_type = 0;
    extern "C" {
        #[c2rust::src_loc = "48:16"]
        pub type verto_ctx;
        #[no_mangle]
        #[c2rust::src_loc = "239:1"]
        pub fn verto_run(ctx: *mut verto_ctx);
    }
    /* VERTO_H_ */
    /* __cplusplus */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth_gssapi.h:51"]
pub mod auth_gssapi_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:16"]
    pub struct _auth_gssapi_name {
        pub name: *mut libc::c_char,
        pub type_0: gss_OID,
    }
    /* include/gssrpc/auth_gssapi.h - GSS-API style auth parameters for RPC */
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved.
 */
    /*
 * Yuck.  Some sys/types.h files leak symbols
 */
    #[c2rust::src_loc = "27:1"]
    pub type auth_gssapi_name = _auth_gssapi_name;
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
    #[c2rust::src_loc = "72:1"]
    pub type auth_gssapi_log_miscerr_func
        =
        Option<unsafe extern "C" fn(_: *mut svc_req, _: *mut rpc_msg,
                                    _: *mut libc::c_char, _: caddr_t) -> ()>;
    use super::gssapi_h::{gss_OID, OM_uint32, gss_name_t};
    use super::svc_h::{SVCXPRT, svc_req};
    use super::sys_types_h::caddr_t;
    use super::rpc_msg_h::rpc_msg;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "120:1"]
        pub fn gssrpc_svcauth_gssapi_set_names(names: *mut auth_gssapi_name,
                                               num: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "122:1"]
        pub fn gssrpc_svcauth_gssapi_unset_names();
        #[no_mangle]
        #[c2rust::src_loc = "128:1"]
        pub fn gssrpc_svcauth_gssapi_set_log_badauth2_func(func:
                                                               auth_gssapi_log_badauth2_func,
                                                           data: caddr_t);
        #[no_mangle]
        #[c2rust::src_loc = "131:1"]
        pub fn gssrpc_svcauth_gssapi_set_log_badverf_func(func:
                                                              auth_gssapi_log_badverf_func,
                                                          data: caddr_t);
        #[no_mangle]
        #[c2rust::src_loc = "134:1"]
        pub fn gssrpc_svcauth_gssapi_set_log_miscerr_func(func:
                                                              auth_gssapi_log_miscerr_func,
                                                          data: caddr_t);
        #[no_mangle]
        #[c2rust::src_loc = "140:1"]
        pub fn gssrpc_svcauth_gss_set_log_badauth2_func(_:
                                                            auth_gssapi_log_badauth2_func,
                                                        _: caddr_t);
        #[no_mangle]
        #[c2rust::src_loc = "142:1"]
        pub fn gssrpc_svcauth_gss_set_log_badverf_func(_:
                                                           auth_gssapi_log_badverf_func,
                                                       _: caddr_t);
        #[no_mangle]
        #[c2rust::src_loc = "144:1"]
        pub fn gssrpc_svcauth_gss_set_log_miscerr_func(_:
                                                           auth_gssapi_log_miscerr_func,
                                                       data: caddr_t);
    }
    /* !defined(GSSRPC_AUTH_GSSAPI_H) */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:52"]
pub mod kdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    use super::krb5_h::{krb5_enctype, krb5_int32, krb5_context,
                        krb5_error_code};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        /*
 * Register the KDB keytab type, allowing "KDB:" to be used as a keytab name.
 * For this type to work, the context used for keytab operations must have an
 * associated database handle (via krb5_db_open()).
 */
        #[no_mangle]
        #[c2rust::src_loc = "874:1"]
        pub fn krb5_db_register_keytab(context_0: krb5_context)
         -> krb5_error_code;
    }
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:52"]
pub mod admin_h {
    #[c2rust::src_loc = "71:1"]
    pub type kadm5_ret_t = libc::c_long;
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
    /*
 * Data structure returned by kadm5_get_config_params()
 */
    #[c2rust::src_loc = "241:1"]
    pub type kadm5_config_params = _kadm5_config_params;
    use super::krb5_h::{krb5_enctype, krb5_deltat, krb5_timestamp, krb5_flags,
                        krb5_int32, krb5_kvno, krb5_context, krb5_error_code,
                        krb5_ui_4};
    use super::kdb_h::krb5_key_salt_tuple;
    use super::stdint_uintn_h::uint32_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
        /* Novell */ /* ABI change? */
        /* Deprecated except for db2 backwards compatibility.  Don't add
       new uses except as fallbacks for parameters that should be
       specified in the database module section of the config
       file.  */
        /*    char *            iprop_server;*/
        /*
 * functions
 */
        #[no_mangle]
        #[c2rust::src_loc = "294:1"]
        pub fn kadm5_get_config_params(context_0: krb5_context,
                                       use_kdc_config: libc::c_int,
                                       params_in: *mut kadm5_config_params,
                                       params_out: *mut kadm5_config_params)
         -> krb5_error_code;
        /*
 * For all initialization functions, the caller must first initialize
 * a context with kadm5_init_krb5_context which will survive as long
 * as the resulting handle.  The caller should free the context with
 * krb5_free_context.
 */
        #[no_mangle]
        #[c2rust::src_loc = "312:1"]
        pub fn kadm5_init(context_0: krb5_context,
                          client_name: *mut libc::c_char,
                          pass: *mut libc::c_char,
                          service_name: *mut libc::c_char,
                          params: *mut kadm5_config_params,
                          struct_version: krb5_ui_4, api_version: krb5_ui_4,
                          db_args: *mut *mut libc::c_char,
                          server_handle: *mut *mut libc::c_void)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "356:1"]
        pub fn kadm5_destroy(server_handle: *mut libc::c_void) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "465:1"]
        pub fn kadm5_init_krb5_context(_: *mut krb5_context)
         -> krb5_error_code;
    }
    /* __KADM5_ADMIN_H__ */
}
#[c2rust::header_src = "/usr/include/stdlib.h:33"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "104:1"]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
        #[no_mangle]
        #[c2rust::src_loc = "634:1"]
        pub fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:33"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "246:14"]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "308:1"]
        pub fn setvbuf(__stream: *mut FILE, __buf: *mut libc::c_char,
                       __modes: libc::c_int, __n: size_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:33"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:33"]
pub mod unistd_h {
    use super::types_h::__pid_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "628:1"]
        pub fn getpid() -> __pid_t;
        #[no_mangle]
        #[c2rust::src_loc = "935:1"]
        pub fn daemon(__nochdir: libc::c_int, __noclose: libc::c_int)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:33"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "252:14"]
        pub fn strrchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:33"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/locale.h:35"]
pub mod locale_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "122:1"]
        pub fn setlocale(__category: libc::c_int,
                         __locale: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/krb5/gssapi_krb5.h:50"]
pub mod gssapi_krb5_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID, OM_uint32};
    extern "C" {
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
        /* C++ friendlyness */
        /* __cplusplus */
        /* Reserved static storage for GSS_oids.  See rfc 1964 for more details. */
        /* 2.1.1. Kerberos Principal Name Form: */
        #[no_mangle]
        #[c2rust::src_loc = "40:33"]
        pub static GSS_KRB5_NT_PRINCIPAL_NAME: gss_OID;
        /* Alias for Heimdal compat. */
        #[no_mangle]
        #[c2rust::src_loc = "179:1"]
        pub fn krb5_gss_register_acceptor_identity(_: *const libc::c_char)
         -> OM_uint32;
    }
    /* _GSSAPI_KRB5_H_ */
    /* __cplusplus */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/adm_proto.h:54"]
pub mod adm_proto_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_boolean, krb5_error_code};
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/adm_proto.h */
/*
 * Copyright 1995, 2007,2008,2009 by the Massachusetts Institute of Technology.
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
 * This is ugly, but avoids having to include k5-int or kdb.h for this.
 */
        /* KRB5_KDB5__ */
        /* Ditto for admin.h */
        /* KRB5_KDB5__ */
        /*
 * Function prototypes.
 */
        /* logger.c */
        #[no_mangle]
        #[c2rust::src_loc = "50:1"]
        pub fn krb5_klog_init(_: krb5_context, _: *mut libc::c_char,
                              _: *mut libc::c_char, _: krb5_boolean)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "52:1"]
        pub fn krb5_klog_close(_: krb5_context);
        #[no_mangle]
        #[c2rust::src_loc = "53:1"]
        pub fn krb5_klog_syslog(_: libc::c_int, _: *const libc::c_char,
                                _: ...) -> libc::c_int;
    }
    /* KRB5_ADM_PROTO_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kdb_kt.h:55"]
pub mod kdb_kt_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_error_code};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "36:1"]
        pub fn krb5_ktkdb_set_context(_: krb5_context) -> krb5_error_code;
    }
    /* KRB5_KDB5_DBM__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/net-server.h:59"]
pub mod net_server_h {
    use super::verto_h::{verto_ev_type, VERTO_EV_TYPE_NONE, verto_ctx};
    use super::krb5_h::krb5_error_code;
    use super::sys_types_h::u_long;
    extern "C" {
        /* exported from net-server.c */
        #[no_mangle]
        #[c2rust::src_loc = "46:1"]
        pub fn loop_init(types: verto_ev_type) -> *mut verto_ctx;
        /*
 * Add listener addresses to the loop configuration.
 *
 * Arguments:
 *
 * - default_port
 *      The port for the sockets if not specified in addresses.
 * - addresses
 *      The optional addresses for the listener sockets.  Pass NULL for the
 *      wildcard address.  Addresses may be delimited by the characters in
 *      ADDRESSES_DELIM.  Addresses are parsed with k5_parse_host_string().
 * - prognum, versnum, dispatchfn
 *      For RPC listener sockets, the svc_register() arguments to use when new
 *      TCP connections are created.
 */
        #[no_mangle]
        #[c2rust::src_loc = "63:1"]
        pub fn loop_add_udp_address(default_port: libc::c_int,
                                    addresses: *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn loop_add_tcp_address(default_port: libc::c_int,
                                    addresses: *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "65:1"]
        pub fn loop_add_rpc_service(default_port: libc::c_int,
                                    addresses: *const libc::c_char,
                                    prognum: u_long, versnum: u_long,
                                    dispatchfn:
                                        Option<unsafe extern "C" fn() -> ()>)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "69:1"]
        pub fn loop_setup_network(ctx: *mut verto_ctx,
                                  handle: *mut libc::c_void,
                                  progname_0: *const libc::c_char,
                                  tcp_listen_backlog: libc::c_int)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "72:1"]
        pub fn loop_setup_signals(ctx: *mut verto_ctx,
                                  handle: *mut libc::c_void,
                                  reset: Option<unsafe extern "C" fn() -> ()>)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn loop_free(ctx: *mut verto_ctx);
    }
    /* NET_SERVER_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kadmin/server/misc.h:59"]
pub mod misc_h {
    use super::svc_h::{svc_req, SVCXPRT};
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "25:1"]
        pub fn kadm_1(_: *mut svc_req, _: *mut SVCXPRT);
        /* network.c */
        #[no_mangle]
        #[c2rust::src_loc = "26:1"]
        pub fn krb5_iprop_prog_1(_: *mut svc_req, _: *mut SVCXPRT);
        #[no_mangle]
        #[c2rust::src_loc = "28:1"]
        pub fn trunc_name(len: *mut size_t, dots: *mut *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "39:1"]
        pub fn client_addr(xprt: *mut SVCXPRT) -> *const libc::c_char;
    }
    /* _MISC_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kadmin/server/auth.h:60"]
pub mod server_auth_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_error_code};
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kadmin/server/auth.h - kadmin authorization declarations */
/*
 * Copyright (C) 2017 by the Massachusetts Institute of Technology.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * * Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.
 *
 * * Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in
 *   the documentation and/or other materials provided with the
 *   distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 */
        /* Initialize all authorization modules. */
        #[no_mangle]
        #[c2rust::src_loc = "57:1"]
        pub fn auth_init(context_0: krb5_context,
                         acl_file: *const libc::c_char) -> krb5_error_code;
        /* Release authorization module state. */
        #[no_mangle]
        #[c2rust::src_loc = "60:1"]
        pub fn auth_fini(context_0: krb5_context);
    }
    /* AUTH_H */
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__u_short, __u_int, __u_long, __uint16_t, __int32_t,
                        __uint32_t, __off_t, __off64_t, __pid_t, __caddr_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint16_t, uint32_t};
pub use self::sys_types_h::{u_short, u_int, u_long, caddr_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::sockaddr_h::sa_family_t;
pub use self::in_h::{in_port_t, sockaddr_in, in_addr, in_addr_t};
pub use self::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t, rpc_inline_t};
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
pub use self::gssapi_h::{gss_name_t, gss_uint32, OM_uint32,
                         gss_OID_desc_struct, gss_OID, gss_buffer_desc_struct,
                         gss_buffer_desc, gss_buffer_t, gss_name_struct,
                         gss_display_status, gss_display_name,
                         gss_import_name, gss_release_name,
                         gss_release_buffer};
pub use self::svc_h::{svc_req, SVCXPRT, xp_ops, xprt_stat, XPRT_IDLE,
                      XPRT_MOREREQS, XPRT_DIED};
pub use self::svc_auth_h::{SVCAUTH, svc_auth_ops,
                           gssrpc_svcauth_gss_set_svc_name};
pub use self::krb5_h::{krb5_int32, krb5_ui_4, krb5_boolean, krb5_kvno,
                       krb5_enctype, krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _profile_t, krb5_c_random_os_entropy,
                       krb5_free_context, krb5_get_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, plugin_mapping, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle};
pub use self::kdb_log_h::{_kdb_log_context, kdb_hlog_t, kdb_hlog, ulog_map,
                          ulog_set_role};
pub use self::iprop_h::{kdb_sno_t, kdbe_time_t};
pub use self::iprop_hdr_h::{iprop_role, IPROP_REPLICA, IPROP_MASTER,
                            IPROP_NULL};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::com_err_h::{errcode_t, error_message};
pub use self::verto_h::{verto_ev_type, VERTO_EV_TYPE_CHILD,
                        VERTO_EV_TYPE_SIGNAL, VERTO_EV_TYPE_IDLE,
                        VERTO_EV_TYPE_TIMEOUT, VERTO_EV_TYPE_IO,
                        VERTO_EV_TYPE_NONE, verto_ctx, verto_run};
pub use self::auth_gssapi_h::{_auth_gssapi_name, auth_gssapi_name,
                              auth_gssapi_log_badauth2_func,
                              auth_gssapi_log_badverf_func,
                              auth_gssapi_log_miscerr_func,
                              gssrpc_svcauth_gssapi_set_names,
                              gssrpc_svcauth_gssapi_unset_names,
                              gssrpc_svcauth_gssapi_set_log_badauth2_func,
                              gssrpc_svcauth_gssapi_set_log_badverf_func,
                              gssrpc_svcauth_gssapi_set_log_miscerr_func,
                              gssrpc_svcauth_gss_set_log_badauth2_func,
                              gssrpc_svcauth_gss_set_log_badverf_func,
                              gssrpc_svcauth_gss_set_log_miscerr_func};
pub use self::kdb_h::{__krb5_key_salt_tuple, krb5_key_salt_tuple,
                      krb5_db_register_keytab};
pub use self::admin_h::{kadm5_ret_t, _kadm5_config_params,
                        kadm5_config_params, kadm5_get_config_params,
                        kadm5_init, kadm5_destroy, kadm5_init_krb5_context};
use self::stdlib_h::{atoi, realloc, free, exit, getenv};
use self::stdio_h::{stderr, fclose, fopen, setvbuf, fprintf, asprintf};
use self::errno_h::__errno_location;
use self::unistd_h::{getpid, daemon};
use self::string_h::{strlen, strrchr, strcmp, memset};
use self::libintl_h::dgettext;
use self::locale_h::setlocale;
use self::gssapi_krb5_h::{GSS_KRB5_NT_PRINCIPAL_NAME,
                          krb5_gss_register_acceptor_identity};
use self::adm_proto_h::{krb5_klog_init, krb5_klog_close, krb5_klog_syslog};
use self::kdb_kt_h::krb5_ktkdb_set_context;
use self::net_server_h::{loop_init, loop_add_udp_address,
                         loop_add_tcp_address, loop_add_rpc_service,
                         loop_setup_network, loop_setup_signals, loop_free};
use self::misc_h::{kadm_1, krb5_iprop_prog_1, trunc_name, client_addr};
use self::server_auth_h::{auth_init, auth_fini};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "210:18"]
pub struct C2RustUnnamed_6 {
    pub proc_0: rpcproc_t,
    pub proc_name: *const libc::c_char,
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved
 *
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
#[no_mangle]
#[c2rust::src_loc = "68:12"]
pub static mut gss_changepw_name: gss_name_t =
    0 as *const gss_name_struct as gss_name_t;
#[no_mangle]
#[c2rust::src_loc = "68:38"]
pub static mut gss_oldchangepw_name: gss_name_t =
    0 as *const gss_name_struct as gss_name_t;
#[no_mangle]
#[c2rust::src_loc = "69:7"]
pub static mut global_server_handle: *mut libc::c_void =
    0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
#[c2rust::src_loc = "70:5"]
pub static mut nofork: libc::c_int = 0 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "71:7"]
pub static mut kdb5_util: *mut libc::c_char =
    b"/usr/local/sbin/kdb5_util\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char;
#[no_mangle]
#[c2rust::src_loc = "72:7"]
pub static mut kprop: *mut libc::c_char =
    b"/usr/local/sbin/kprop\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char;
#[no_mangle]
#[c2rust::src_loc = "73:7"]
pub static mut dump_file: *mut libc::c_char =
    b"/usr/local/var/krb5kdc/replica_datatrans\x00" as *const u8 as
        *const libc::c_char as *mut libc::c_char;
#[no_mangle]
#[c2rust::src_loc = "74:7"]
pub static mut kprop_port: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "76:21"]
static mut context: krb5_context =
    0 as *const _krb5_context as *mut _krb5_context;
#[c2rust::src_loc = "77:14"]
static mut progname: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "83:1"]
unsafe extern "C" fn usage() {
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"Usage: kadmind [-x db_args]* [-r realm] [-m] [-nofork] [-port port-number]\n\t\t[-proponly] [-p path-to-kdb5_util] [-F dump-file]\n\t\t[-K path-to-kprop] [-k kprop-port] [-P pid_file]\n\nwhere,\n\t[-x db_args]* - any number of database specific arguments.\n\t\t\tLook at each database documentation for supported arguments\n\x00"
                         as *const u8 as *const libc::c_char));
    exit(1 as libc::c_int);
}
/*
 * Output a message to stderr and the admin server log, and exit with status 1.
 * msg should not be punctuated.  If code is given, msg should indicate what
 * operation was taking place in the present progressive.  Otherwise msg should
 * be capitalized and should indicate what went wrong.
 */
#[c2rust::src_loc = "103:1"]
unsafe extern "C" fn fail_to_start(mut code: krb5_error_code,
                                   mut msg: *const libc::c_char) {
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    if code != 0 {
        errmsg = krb5_get_error_message(context, code);
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"%s: %s while %s, aborting\n\x00" as *const u8 as
                             *const libc::c_char), progname, errmsg, msg);
        krb5_klog_syslog(3 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"%s while %s, aborting\n\x00" as *const u8
                                      as *const libc::c_char), errmsg, msg);
    } else {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"%s: %s, aborting\n\x00" as *const u8 as
                             *const libc::c_char), progname, msg);
        krb5_klog_syslog(3 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"%s, aborting\x00" as *const u8 as
                                      *const libc::c_char), msg);
    }
    exit(1 as libc::c_int);
}
#[c2rust::src_loc = "120:1"]
unsafe extern "C" fn write_pid_file(mut pid_file: *const libc::c_char)
 -> libc::c_int {
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut pid: libc::c_ulong = 0;
    let mut st1: libc::c_int = 0;
    let mut st2: libc::c_int = 0;
    file = fopen(pid_file, b"w\x00" as *const u8 as *const libc::c_char);
    if file.is_null() { return *__errno_location() }
    pid = getpid() as libc::c_ulong;
    st1 =
        if fprintf(file, b"%ld\n\x00" as *const u8 as *const libc::c_char,
                   pid) < 0 as libc::c_int {
            *__errno_location()
        } else { 0 as libc::c_int };
    st2 =
        if fclose(file) == -(1 as libc::c_int) {
            *__errno_location()
        } else { 0 as libc::c_int };
    return if st1 != 0 { st1 } else { st2 };
}
/* Set up the main loop.  If proponly is set, don't set up ports for kpasswd or
 * kadmin.  May set *ctx_out even on error. */
#[c2rust::src_loc = "138:1"]
unsafe extern "C" fn setup_loop(mut params: *mut kadm5_config_params,
                                mut proponly: libc::c_int,
                                mut ctx_out: *mut *mut verto_ctx)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut ctx: *mut verto_ctx = 0 as *mut verto_ctx;
    ctx = loop_init(VERTO_EV_TYPE_SIGNAL);
    *ctx_out = ctx;
    if ctx.is_null() { return 12 as libc::c_int }
    ret = loop_setup_signals(ctx, global_server_handle, None);
    if ret != 0 { return ret }
    if proponly == 0 {
        ret =
            loop_add_udp_address((*params).kpasswd_port,
                                 (*params).kpasswd_listen);
        if ret != 0 { return ret }
        ret =
            loop_add_tcp_address((*params).kpasswd_port,
                                 (*params).kpasswd_listen);
        if ret != 0 { return ret }
        ret =
            loop_add_rpc_service((*params).kadmind_port,
                                 (*params).kadmind_listen,
                                 2112 as libc::c_int as u_long,
                                 2 as libc::c_int as u_long,
                                 ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                         *mut svc_req,
                                                                                     _:
                                                                                         *mut SVCXPRT)
                                                                    -> ()>,
                                                         Option<unsafe extern "C" fn()
                                                                    ->
                                                                        ()>>(Some(kadm_1
                                                                                      as
                                                                                      unsafe extern "C" fn(_:
                                                                                                               *mut svc_req,
                                                                                                           _:
                                                                                                               *mut SVCXPRT)
                                                                                          ->
                                                                                              ())));
        if ret != 0 { return ret }
    }
    if (*params).iprop_enabled != 0 {
        ret =
            loop_add_rpc_service((*params).iprop_port, (*params).iprop_listen,
                                 100423 as libc::c_int as u_long,
                                 1 as libc::c_int as u_long,
                                 ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                         *mut svc_req,
                                                                                     _:
                                                                                         *mut SVCXPRT)
                                                                    -> ()>,
                                                         Option<unsafe extern "C" fn()
                                                                    ->
                                                                        ()>>(Some(krb5_iprop_prog_1
                                                                                      as
                                                                                      unsafe extern "C" fn(_:
                                                                                                               *mut svc_req,
                                                                                                           _:
                                                                                                               *mut SVCXPRT)
                                                                                          ->
                                                                                              ())));
        if ret != 0 { return ret }
    }
    return loop_setup_network(ctx, global_server_handle, progname,
                              5 as libc::c_int);
}
/* Point GSSAPI at the KDB keytab so we don't need an actual file keytab. */
#[c2rust::src_loc = "179:1"]
unsafe extern "C" fn setup_kdb_keytab() -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    ret = krb5_ktkdb_set_context(context);
    if ret != 0 { return ret }
    ret = krb5_db_register_keytab(context);
    if ret != 0 { return ret }
    return krb5_gss_register_acceptor_identity(b"KDB:\x00" as *const u8 as
                                                   *const libc::c_char) as
               krb5_error_code;
}
/* Return "name@realm". */
#[c2rust::src_loc = "195:1"]
unsafe extern "C" fn build_princ_name(mut name: *mut libc::c_char,
                                      mut realm: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut fullname: *mut libc::c_char = 0 as *mut libc::c_char;
    if asprintf(&mut fullname as *mut *mut libc::c_char,
                b"%s@%s\x00" as *const u8 as *const libc::c_char, name, realm)
           < 0 as libc::c_int {
        return 0 as *mut libc::c_char
    }
    return fullname;
}
/* Callback from GSSRPC for garbled/forged/replayed/etc messages. */
#[c2rust::src_loc = "206:1"]
unsafe extern "C" fn log_badverf(mut client_name: gss_name_t,
                                 mut server_name: gss_name_t,
                                 mut rqst: *mut svc_req,
                                 mut msg: *mut rpc_msg,
                                 mut data: *mut libc::c_char) {
    static mut proc_names: [C2RustUnnamed_6; 23] =
        [{
             let mut init =
                 C2RustUnnamed_6{proc_0: 1 as libc::c_int as rpcproc_t,
                                 proc_name:
                                     b"CREATE_PRINCIPAL\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_6{proc_0: 2 as libc::c_int as rpcproc_t,
                                 proc_name:
                                     b"DELETE_PRINCIPAL\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_6{proc_0: 3 as libc::c_int as rpcproc_t,
                                 proc_name:
                                     b"MODIFY_PRINCIPAL\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_6{proc_0: 4 as libc::c_int as rpcproc_t,
                                 proc_name:
                                     b"RENAME_PRINCIPAL\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_6{proc_0: 5 as libc::c_int as rpcproc_t,
                                 proc_name:
                                     b"GET_PRINCIPAL\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_6{proc_0: 6 as libc::c_int as rpcproc_t,
                                 proc_name:
                                     b"CHPASS_PRINCIPAL\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_6{proc_0: 7 as libc::c_int as rpcproc_t,
                                 proc_name:
                                     b"CHRAND_PRINCIPAL\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_6{proc_0: 8 as libc::c_int as rpcproc_t,
                                 proc_name:
                                     b"CREATE_POLICY\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_6{proc_0: 9 as libc::c_int as rpcproc_t,
                                 proc_name:
                                     b"DELETE_POLICY\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_6{proc_0: 10 as libc::c_int as rpcproc_t,
                                 proc_name:
                                     b"MODIFY_POLICY\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_6{proc_0: 11 as libc::c_int as rpcproc_t,
                                 proc_name:
                                     b"GET_POLICY\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_6{proc_0: 12 as libc::c_int as rpcproc_t,
                                 proc_name:
                                     b"GET_PRIVS\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_6{proc_0: 13 as libc::c_int as rpcproc_t,
                                 proc_name:
                                     b"INIT\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_6{proc_0: 14 as libc::c_int as rpcproc_t,
                                 proc_name:
                                     b"GET_PRINCS\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_6{proc_0: 15 as libc::c_int as rpcproc_t,
                                 proc_name:
                                     b"GET_POLS\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_6{proc_0: 16 as libc::c_int as rpcproc_t,
                                 proc_name:
                                     b"SETKEY_PRINCIPAL\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_6{proc_0: 18 as libc::c_int as rpcproc_t,
                                 proc_name:
                                     b"CREATE_PRINCIPAL3\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_6{proc_0: 19 as libc::c_int as rpcproc_t,
                                 proc_name:
                                     b"CHPASS_PRINCIPAL3\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_6{proc_0: 20 as libc::c_int as rpcproc_t,
                                 proc_name:
                                     b"CHRAND_PRINCIPAL3\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_6{proc_0: 21 as libc::c_int as rpcproc_t,
                                 proc_name:
                                     b"SETKEY_PRINCIPAL3\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_6{proc_0: 22 as libc::c_int as rpcproc_t,
                                 proc_name:
                                     b"PURGEKEYS\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_6{proc_0: 23 as libc::c_int as rpcproc_t,
                                 proc_name:
                                     b"GET_STRINGS\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_6{proc_0: 24 as libc::c_int as rpcproc_t,
                                 proc_name:
                                     b"SET_STRING\x00" as *const u8 as
                                         *const libc::c_char,};
             init
         }];
    let mut minor: OM_uint32 = 0;
    let mut client: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut server: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut gss_type: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut a: *const libc::c_char = 0 as *const libc::c_char;
    let mut proc_0: rpcproc_t = 0;
    let mut i: libc::c_uint = 0;
    let mut procname: *const libc::c_char = 0 as *const libc::c_char;
    let mut clen: size_t = 0;
    let mut slen: size_t = 0;
    let mut cdots: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sdots: *mut libc::c_char = 0 as *mut libc::c_char;
    client.length = 0 as libc::c_int as size_t;
    client.value = 0 as *mut libc::c_void;
    server.length = 0 as libc::c_int as size_t;
    server.value = 0 as *mut libc::c_void;
    gss_display_name(&mut minor, client_name, &mut client, &mut gss_type);
    gss_display_name(&mut minor, server_name, &mut server, &mut gss_type);
    if client.value.is_null() {
        client.value =
            b"(null)\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_void;
        clen =
            (::std::mem::size_of::<[libc::c_char; 7]>() as
                 libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                 libc::c_ulong)
    } else { clen = client.length }
    trunc_name(&mut clen, &mut cdots);
    if server.value.is_null() {
        server.value =
            b"(null)\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_void;
        slen =
            (::std::mem::size_of::<[libc::c_char; 7]>() as
                 libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                 libc::c_ulong)
    } else { slen = server.length }
    trunc_name(&mut slen, &mut sdots);
    a = client_addr((*rqst).rq_xprt);
    proc_0 = (*msg).ru.RM_cmb.cb_proc;
    procname = 0 as *const libc::c_char;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[C2RustUnnamed_6; 23]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<C2RustUnnamed_6>()
                                                   as libc::c_ulong) {
        if proc_names[i as usize].proc_0 == proc_0 {
            procname = proc_names[i as usize].proc_name;
            break ;
        } else { i = i.wrapping_add(1) }
    }
    if !procname.is_null() {
        krb5_klog_syslog(5 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"WARNING! Forged/garbled request: %s, claimed client = %.*s%s, server = %.*s%s, addr = %s\x00"
                                      as *const u8 as *const libc::c_char),
                         procname, clen as libc::c_int,
                         client.value as *mut libc::c_char, cdots,
                         slen as libc::c_int,
                         server.value as *mut libc::c_char, sdots, a);
    } else {
        krb5_klog_syslog(5 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"WARNING! Forged/garbled request: %d, claimed client = %.*s%s, server = %.*s%s, addr = %s\x00"
                                      as *const u8 as *const libc::c_char),
                         proc_0, clen as libc::c_int,
                         client.value as *mut libc::c_char, cdots,
                         slen as libc::c_int,
                         server.value as *mut libc::c_char, sdots, a);
    }
    gss_release_buffer(&mut minor, &mut client);
    gss_release_buffer(&mut minor, &mut server);
}
/* Callback from GSSRPC for miscellaneous errors */
#[c2rust::src_loc = "299:1"]
unsafe extern "C" fn log_miscerr(mut rqst: *mut svc_req,
                                 mut msg: *mut rpc_msg,
                                 mut error: *mut libc::c_char,
                                 mut data: *mut libc::c_char) {
    krb5_klog_syslog(5 as libc::c_int,
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"Miscellaneous RPC error: %s, %s\x00" as
                                  *const u8 as *const libc::c_char),
                     client_addr((*rqst).rq_xprt), error);
}
#[c2rust::src_loc = "306:1"]
unsafe extern "C" fn log_badauth_display_status_1(mut m: *mut libc::c_char,
                                                  mut code: OM_uint32,
                                                  mut type_0: libc::c_int) {
    let mut gssstat: OM_uint32 = 0;
    let mut minor_stat: OM_uint32 = 0;
    let mut msg: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut msg_ctx: OM_uint32 = 0;
    msg_ctx = 0 as libc::c_int as OM_uint32;
    loop  {
        gssstat =
            gss_display_status(&mut minor_stat, code, type_0, 0 as gss_OID,
                               &mut msg_ctx, &mut msg);
        if gssstat != 0 as libc::c_int as libc::c_uint {
            krb5_klog_syslog(3 as libc::c_int,
                             dgettext(b"mit-krb5\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"%s Cannot decode status %d\x00" as
                                          *const u8 as *const libc::c_char),
                             m, code as libc::c_int);
            return
        }
        krb5_klog_syslog(5 as libc::c_int,
                         b"%s %.*s\x00" as *const u8 as *const libc::c_char,
                         m, msg.length as libc::c_int,
                         msg.value as *mut libc::c_char);
        gss_release_buffer(&mut minor_stat, &mut msg);
        if msg_ctx == 0 { break ; }
    };
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1994 OpenVision Technologies, Inc., All Rights Reserved
 *
 */
/* Callback from GSSRPC for authentication failures */
#[no_mangle]
#[c2rust::src_loc = "333:1"]
pub unsafe extern "C" fn log_badauth(mut major: OM_uint32,
                                     mut minor: OM_uint32,
                                     mut xprt: *mut SVCXPRT,
                                     mut data: *mut libc::c_char) {
    krb5_klog_syslog(5 as libc::c_int,
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"Authentication attempt failed: %s, GSS-API error strings are:\x00"
                                  as *const u8 as *const libc::c_char),
                     client_addr(xprt));
    log_badauth_display_status_1(b"   \x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                                 major, 1 as libc::c_int);
    log_badauth_display_status_1(b"   \x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                                 minor, 2 as libc::c_int);
    krb5_klog_syslog(5 as libc::c_int,
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"   GSS-API error strings complete.\x00" as
                                  *const u8 as *const libc::c_char));
}
#[c2rust::src_loc = "344:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut minor_status: OM_uint32 = 0;
    let mut in_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut nt_krb5_name_oid: gss_OID = GSS_KRB5_NT_PRINCIPAL_NAME;
    let mut names: [auth_gssapi_name; 4] =
        [auth_gssapi_name{name: 0 as *mut libc::c_char,
                          type_0: 0 as *mut gss_OID_desc_struct,}; 4];
    let mut params: kadm5_config_params =
        kadm5_config_params{mask: 0,
                            realm: 0 as *mut libc::c_char,
                            kadmind_port: 0,
                            kpasswd_port: 0,
                            admin_server: 0 as *mut libc::c_char,
                            dbname: 0 as *mut libc::c_char,
                            acl_file: 0 as *mut libc::c_char,
                            dict_file: 0 as *mut libc::c_char,
                            mkey_from_kbd: 0,
                            stash_file: 0 as *mut libc::c_char,
                            mkey_name: 0 as *mut libc::c_char,
                            enctype: 0,
                            max_life: 0,
                            max_rlife: 0,
                            expiration: 0,
                            flags: 0,
                            keysalts: 0 as *mut krb5_key_salt_tuple,
                            num_keysalts: 0,
                            kvno: 0,
                            iprop_enabled: 0,
                            iprop_ulogsize: 0,
                            iprop_poll_time: 0,
                            iprop_logfile: 0 as *mut libc::c_char,
                            iprop_port: 0,
                            iprop_resync_timeout: 0,
                            kadmind_listen: 0 as *mut libc::c_char,
                            kpasswd_listen: 0 as *mut libc::c_char,
                            iprop_listen: 0 as *mut libc::c_char,};
    let mut vctx: *mut verto_ctx = 0 as *mut verto_ctx;
    let mut pid_file: *const libc::c_char = 0 as *const libc::c_char;
    let mut db_args: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tmpargs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut acl_file: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut db_args_size: libc::c_int = 0 as libc::c_int;
    let mut strong_random: libc::c_int = 1 as libc::c_int;
    let mut proponly: libc::c_int = 0 as libc::c_int;
    setlocale(6 as libc::c_int, b"\x00" as *const u8 as *const libc::c_char);
    setvbuf(stderr, 0 as *mut libc::c_char, 2 as libc::c_int,
            0 as libc::c_int as size_t);
    names[3 as libc::c_int as usize].name = 0 as *mut libc::c_char;
    names[2 as libc::c_int as usize].name =
        names[3 as libc::c_int as usize].name;
    names[1 as libc::c_int as usize].name =
        names[2 as libc::c_int as usize].name;
    names[0 as libc::c_int as usize].name =
        names[1 as libc::c_int as usize].name;
    names[3 as libc::c_int as usize].type_0 = nt_krb5_name_oid;
    names[2 as libc::c_int as usize].type_0 =
        names[3 as libc::c_int as usize].type_0;
    names[1 as libc::c_int as usize].type_0 =
        names[2 as libc::c_int as usize].type_0;
    names[0 as libc::c_int as usize].type_0 =
        names[1 as libc::c_int as usize].type_0;
    progname =
        if !strrchr(*argv.offset(0 as libc::c_int as isize),
                    '/' as i32).is_null() {
            strrchr(*argv.offset(0 as libc::c_int as isize),
                    '/' as i32).offset(1 as libc::c_int as isize)
        } else { *argv.offset(0 as libc::c_int as isize) };
    memset(&mut params as *mut kadm5_config_params as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<kadm5_config_params>() as libc::c_ulong);
    argc -= 1;
    argv = argv.offset(1);
    while argc != 0 {
        if strcmp(*argv, b"-x\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
            argc -= 1;
            argv = argv.offset(1);
            if argc == 0 { usage(); }
            db_args_size += 1;
            tmpargs =
                realloc(db_args as *mut libc::c_void,
                        (::std::mem::size_of::<*mut libc::c_char>() as
                             libc::c_ulong).wrapping_mul((db_args_size +
                                                              1 as
                                                                  libc::c_int)
                                                             as
                                                             libc::c_ulong))
                    as *mut *mut libc::c_char;
            if tmpargs.is_null() {
                fprintf(stderr,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"%s: cannot initialize. Not enough memory\n\x00"
                                     as *const u8 as *const libc::c_char),
                        progname);
                exit(1 as libc::c_int);
            }
            db_args = tmpargs;
            let ref mut fresh0 =
                *db_args.offset((db_args_size - 1 as libc::c_int) as isize);
            *fresh0 = *argv;
            let ref mut fresh1 = *db_args.offset(db_args_size as isize);
            *fresh1 = 0 as *mut libc::c_char
        } else if strcmp(*argv, b"-r\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            argc -= 1;
            argv = argv.offset(1);
            if argc == 0 { usage(); }
            params.realm = *argv;
            params.mask |= 0x1 as libc::c_int as libc::c_long;
            argc -= 1;
            argv = argv.offset(1);
            continue ;
        } else if strcmp(*argv, b"-m\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            params.mkey_from_kbd = 1 as libc::c_int;
            params.mask |= 0x40000 as libc::c_int as libc::c_long
        } else if strcmp(*argv,
                         b"-nofork\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            nofork = 1 as libc::c_int
        } else if strcmp(*argv,
                         b"-proponly\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            proponly = 1 as libc::c_int
        } else if strcmp(*argv,
                         b"-port\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            argc -= 1;
            argv = argv.offset(1);
            if argc == 0 { usage(); }
            params.kadmind_port = atoi(*argv);
            params.mask |= 0x4000 as libc::c_int as libc::c_long
        } else if strcmp(*argv, b"-P\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            argc -= 1;
            argv = argv.offset(1);
            if argc == 0 { usage(); }
            pid_file = *argv
        } else if strcmp(*argv, b"-W\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            strong_random = 0 as libc::c_int
        } else if strcmp(*argv, b"-p\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            argc -= 1;
            argv = argv.offset(1);
            if argc == 0 { usage(); }
            kdb5_util = *argv
        } else if strcmp(*argv, b"-F\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            argc -= 1;
            argv = argv.offset(1);
            if argc == 0 { usage(); }
            dump_file = *argv
        } else if strcmp(*argv, b"-K\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            argc -= 1;
            argv = argv.offset(1);
            if argc == 0 { usage(); }
            kprop = *argv
        } else {
            if !(strcmp(*argv, b"-k\x00" as *const u8 as *const libc::c_char)
                     == 0 as libc::c_int) {
                break ;
            }
            argc -= 1;
            argv = argv.offset(1);
            if argc == 0 { usage(); }
            kprop_port = *argv
        }
        argc -= 1;
        argv = argv.offset(1)
    }
    if argc != 0 as libc::c_int { usage(); }
    ret = kadm5_init_krb5_context(&mut context);
    if ret != 0 {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"%s: %s while initializing context, aborting\n\x00"
                             as *const u8 as *const libc::c_char), progname,
                error_message(ret as errcode_t));
        exit(1 as libc::c_int);
    }
    krb5_klog_init(context,
                   b"admin_server\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char, progname,
                   1 as libc::c_int as krb5_boolean);
    ret =
        kadm5_init(context,
                   b"kadmind\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char, 0 as *mut libc::c_char,
                   0 as *mut libc::c_char, &mut params,
                   (0x12345600 as libc::c_int | 0x1 as libc::c_int) as
                       krb5_ui_4,
                   (0x12345700 as libc::c_int | 0x4 as libc::c_int) as
                       krb5_ui_4, db_args, &mut global_server_handle) as
            libc::c_int;
    if ret != 0 {
        fail_to_start(ret,
                      dgettext(b"mit-krb5\x00" as *const u8 as
                                   *const libc::c_char,
                               b"initializing\x00" as *const u8 as
                                   *const libc::c_char));
    }
    ret =
        kadm5_get_config_params(context, 1 as libc::c_int, &mut params,
                                &mut params);
    if ret != 0 {
        fail_to_start(ret,
                      dgettext(b"mit-krb5\x00" as *const u8 as
                                   *const libc::c_char,
                               b"getting config parameters\x00" as *const u8
                                   as *const libc::c_char));
    }
    if params.mask & 0x1 as libc::c_int as libc::c_long == 0 {
        fail_to_start(0 as libc::c_int,
                      dgettext(b"mit-krb5\x00" as *const u8 as
                                   *const libc::c_char,
                               b"Missing required realm configuration\x00" as
                                   *const u8 as *const libc::c_char));
    }
    if params.mask & 0x2000 as libc::c_int as libc::c_long == 0 {
        fail_to_start(0 as libc::c_int,
                      dgettext(b"mit-krb5\x00" as *const u8 as
                                   *const libc::c_char,
                               b"Missing required ACL file configuration\x00"
                                   as *const u8 as *const libc::c_char));
    }
    if proponly != 0 && params.iprop_enabled == 0 {
        fail_to_start(0 as libc::c_int,
                      dgettext(b"mit-krb5\x00" as *const u8 as
                                   *const libc::c_char,
                               b"-proponly can only be used when iprop_enable is true\x00"
                                   as *const u8 as *const libc::c_char));
    }
    ret = setup_loop(&mut params, proponly, &mut vctx);
    if ret != 0 {
        fail_to_start(ret,
                      dgettext(b"mit-krb5\x00" as *const u8 as
                                   *const libc::c_char,
                               b"initializing network\x00" as *const u8 as
                                   *const libc::c_char));
    }
    names[0 as libc::c_int as usize].name =
        build_princ_name(b"kadmin/admin\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                         params.realm);
    names[1 as libc::c_int as usize].name =
        build_princ_name(b"kadmin/changepw\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                         params.realm);
    if names[0 as libc::c_int as usize].name.is_null() ||
           names[1 as libc::c_int as usize].name.is_null() {
        fail_to_start(0 as libc::c_int,
                      dgettext(b"mit-krb5\x00" as *const u8 as
                                   *const libc::c_char,
                               b"Cannot build GSSAPI auth names\x00" as
                                   *const u8 as *const libc::c_char));
    }
    ret = setup_kdb_keytab();
    if ret != 0 {
        fail_to_start(0 as libc::c_int,
                      dgettext(b"mit-krb5\x00" as *const u8 as
                                   *const libc::c_char,
                               b"Cannot set up KDB keytab\x00" as *const u8 as
                                   *const libc::c_char));
    }
    if gssrpc_svcauth_gssapi_set_names(names.as_mut_ptr(), 2 as libc::c_int)
           == 0 as libc::c_int {
        fail_to_start(0 as libc::c_int,
                      dgettext(b"mit-krb5\x00" as *const u8 as
                                   *const libc::c_char,
                               b"Cannot set GSSAPI authentication names\x00"
                                   as *const u8 as *const libc::c_char));
    }
    /* if set_names succeeded, this will too */
    in_buf.value = names[1 as libc::c_int as usize].name as *mut libc::c_void;
    in_buf.length =
        strlen(names[1 as libc::c_int as
                         usize].name).wrapping_add(1 as libc::c_int as
                                                       libc::c_ulong);
    gss_import_name(&mut minor_status, &mut in_buf, nt_krb5_name_oid,
                    &mut gss_changepw_name);
    gssrpc_svcauth_gssapi_set_log_badauth2_func(Some(log_badauth as
                                                         unsafe extern "C" fn(_:
                                                                                  OM_uint32,
                                                                              _:
                                                                                  OM_uint32,
                                                                              _:
                                                                                  *mut SVCXPRT,
                                                                              _:
                                                                                  *mut libc::c_char)
                                                             -> ()),
                                                0 as caddr_t);
    gssrpc_svcauth_gssapi_set_log_badverf_func(Some(log_badverf as
                                                        unsafe extern "C" fn(_:
                                                                                 gss_name_t,
                                                                             _:
                                                                                 gss_name_t,
                                                                             _:
                                                                                 *mut svc_req,
                                                                             _:
                                                                                 *mut rpc_msg,
                                                                             _:
                                                                                 *mut libc::c_char)
                                                            -> ()),
                                               0 as caddr_t);
    gssrpc_svcauth_gssapi_set_log_miscerr_func(Some(log_miscerr as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut svc_req,
                                                                             _:
                                                                                 *mut rpc_msg,
                                                                             _:
                                                                                 *mut libc::c_char,
                                                                             _:
                                                                                 *mut libc::c_char)
                                                            -> ()),
                                               0 as caddr_t);
    gssrpc_svcauth_gss_set_log_badauth2_func(Some(log_badauth as
                                                      unsafe extern "C" fn(_:
                                                                               OM_uint32,
                                                                           _:
                                                                               OM_uint32,
                                                                           _:
                                                                               *mut SVCXPRT,
                                                                           _:
                                                                               *mut libc::c_char)
                                                          -> ()),
                                             0 as caddr_t);
    gssrpc_svcauth_gss_set_log_badverf_func(Some(log_badverf as
                                                     unsafe extern "C" fn(_:
                                                                              gss_name_t,
                                                                          _:
                                                                              gss_name_t,
                                                                          _:
                                                                              *mut svc_req,
                                                                          _:
                                                                              *mut rpc_msg,
                                                                          _:
                                                                              *mut libc::c_char)
                                                         -> ()),
                                            0 as caddr_t);
    gssrpc_svcauth_gss_set_log_miscerr_func(Some(log_miscerr as
                                                     unsafe extern "C" fn(_:
                                                                              *mut svc_req,
                                                                          _:
                                                                              *mut rpc_msg,
                                                                          _:
                                                                              *mut libc::c_char,
                                                                          _:
                                                                              *mut libc::c_char)
                                                         -> ()),
                                            0 as caddr_t);
    if gssrpc_svcauth_gss_set_svc_name(0 as gss_name_t) != 1 as libc::c_int {
        fail_to_start(0 as libc::c_int,
                      dgettext(b"mit-krb5\x00" as *const u8 as
                                   *const libc::c_char,
                               b"Cannot initialize GSSAPI service name\x00" as
                                   *const u8 as *const libc::c_char));
    }
    acl_file =
        if *params.acl_file as libc::c_int != '\u{0}' as i32 {
            params.acl_file
        } else { 0 as *mut libc::c_char };
    ret = auth_init(context, acl_file);
    if ret != 0 {
        fail_to_start(ret,
                      dgettext(b"mit-krb5\x00" as *const u8 as
                                   *const libc::c_char,
                               b"initializing ACL file\x00" as *const u8 as
                                   *const libc::c_char));
    }
    if nofork == 0 &&
           daemon(0 as libc::c_int, 0 as libc::c_int) != 0 as libc::c_int {
        fail_to_start(*__errno_location(),
                      dgettext(b"mit-krb5\x00" as *const u8 as
                                   *const libc::c_char,
                               b"spawning daemon process\x00" as *const u8 as
                                   *const libc::c_char));
    }
    if !pid_file.is_null() {
        ret = write_pid_file(pid_file);
        if ret != 0 {
            fail_to_start(ret,
                          dgettext(b"mit-krb5\x00" as *const u8 as
                                       *const libc::c_char,
                                   b"creating PID file\x00" as *const u8 as
                                       *const libc::c_char));
        }
    }
    krb5_klog_syslog(6 as libc::c_int,
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"Seeding random number generator\x00" as
                                  *const u8 as *const libc::c_char));
    ret =
        krb5_c_random_os_entropy(context, strong_random,
                                 0 as *mut libc::c_int);
    if ret != 0 {
        fail_to_start(ret,
                      dgettext(b"mit-krb5\x00" as *const u8 as
                                   *const libc::c_char,
                               b"getting random seed\x00" as *const u8 as
                                   *const libc::c_char));
    }
    if params.iprop_enabled == 1 as libc::c_int {
        ulog_set_role(context, IPROP_MASTER);
        ret = ulog_map(context, params.iprop_logfile, params.iprop_ulogsize);
        if ret != 0 {
            fail_to_start(ret,
                          dgettext(b"mit-krb5\x00" as *const u8 as
                                       *const libc::c_char,
                                   b"mapping update log\x00" as *const u8 as
                                       *const libc::c_char));
        }
        if nofork != 0 {
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"%s: create IPROP svc (PROG=%d, VERS=%d)\n\x00"
                                 as *const u8 as *const libc::c_char),
                    progname, 100423 as libc::c_int, 1 as libc::c_int);
        }
    }
    if kprop_port.is_null() {
        kprop_port =
            getenv(b"KPROP_PORT\x00" as *const u8 as *const libc::c_char)
    }
    krb5_klog_syslog(6 as libc::c_int,
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"starting\x00" as *const u8 as
                                  *const libc::c_char));
    if nofork != 0 {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"%s: starting...\n\x00" as *const u8 as
                             *const libc::c_char), progname);
    }
    verto_run(vctx);
    krb5_klog_syslog(6 as libc::c_int,
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"finished, exiting\x00" as *const u8 as
                                  *const libc::c_char));
    /* Clean up memory, etc */
    gssrpc_svcauth_gssapi_unset_names();
    kadm5_destroy(global_server_handle);
    loop_free(vctx);
    auth_fini(context);
    gss_release_name(&mut minor_status, &mut gss_changepw_name);
    gss_release_name(&mut minor_status, &mut gss_oldchangepw_name);
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        free(names[i as usize].name as *mut libc::c_void);
        i += 1
    }
    krb5_klog_close(context);
    krb5_free_context(context);
    exit(0 as libc::c_int);
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
