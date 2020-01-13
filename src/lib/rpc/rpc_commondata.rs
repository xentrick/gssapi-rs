use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:34"]
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
#[c2rust::header_src = "/usr/include/sys/types.h:34"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:34"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/sys/select.h:34"]
pub mod select_h {
    #[c2rust::src_loc = "49:1"]
    pub type __fd_mask = libc::c_long;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "59:9"]
    pub struct fd_set {
        pub __fds_bits: [__fd_mask; 16],
    }
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:34"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:34"]
pub mod gssrpc_types_h {
    #[c2rust::src_loc = "89:1"]
    pub type rpcvers_t = uint32_t;
    use super::stdint_uintn_h::uint32_t;
    /* !defined(GSSRPC_TYPES_H) */
    /*
 * The below should probably be internal-only, but seem to be
 * traditionally exported in RPC implementations.
 */
    /* XXX namespace */
    /* This is for rpc/netdb.h */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth.h:34"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/clnt.h:34"]
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
        pub ru: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:2"]
    pub union C2RustUnnamed {
        pub RE_errno: libc::c_int,
        pub RE_why: auth_stat,
        pub RE_vers: C2RustUnnamed_1,
        pub RE_lb: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "99:3"]
    pub struct C2RustUnnamed_0 {
        pub s1: int32_t,
        pub s2: int32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "95:3"]
    pub struct C2RustUnnamed_1 {
        pub low: rpcvers_t,
        pub high: rpcvers_t,
    }
    /* string */
    /*
 * If a creation fails, the following allows the user to figure out why.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "327:8"]
    pub struct gssrpc_rpc_createrr {
        pub cf_stat: clnt_stat,
        pub cf_error: rpc_err,
    }
    use super::auth_h::auth_stat;
    use super::stdint_intn_h::int32_t;
    use super::gssrpc_types_h::rpcvers_t;
    /* !defined(GSSRPC_CLNT_H) */
    /* a more reasonable packet size */
    /* rpc imposed limit on udp msg size */
    /* string */
}
pub use self::types_h::{__u_int, __int32_t, __uint32_t, __caddr_t};
pub use self::sys_types_h::{u_int, caddr_t};
pub use self::stdint_intn_h::int32_t;
pub use self::select_h::{__fd_mask, fd_set};
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssrpc_types_h::rpcvers_t;
pub use self::auth_h::{auth_stat, RPCSEC_GSS_CTXPROBLEM,
                       RPCSEC_GSS_CREDPROBLEM, AUTH_FAILED, AUTH_INVALIDRESP,
                       AUTH_TOOWEAK, AUTH_REJECTEDVERF, AUTH_BADVERF,
                       AUTH_REJECTEDCRED, AUTH_BADCRED, AUTH_OK, opaque_auth};
pub use self::clnt_h::{clnt_stat, RPC_FAILED, RPC_PROGNOTREGISTERED,
                       RPC_PMAPFAILURE, RPC_UNKNOWNPROTO, RPC_UNKNOWNHOST,
                       RPC_SYSTEMERROR, RPC_CANTDECODEARGS, RPC_PROCUNAVAIL,
                       RPC_PROGVERSMISMATCH, RPC_PROGUNAVAIL, RPC_AUTHERROR,
                       RPC_VERSMISMATCH, RPC_TIMEDOUT, RPC_CANTRECV,
                       RPC_CANTSEND, RPC_CANTDECODERES, RPC_CANTENCODEARGS,
                       RPC_SUCCESS, rpc_err, C2RustUnnamed, C2RustUnnamed_0,
                       C2RustUnnamed_1, gssrpc_rpc_createrr};
/* @(#)rpc_commondata.c	2.1 88/07/29 4.0 RPCSRC */
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
 * This file should only contain common data (global data) that is exported
 * by public interfaces.
 *
 * Actually initialized to prevent creation of common blocks, which
 * can be problematic on some architectures.
 */
/* RENAMED: should be _null_auth */
#[no_mangle]
#[c2rust::src_loc = "43:20"]
pub static mut gssrpc__null_auth: opaque_auth =
    {
        let mut init =
            opaque_auth{oa_flavor: 0 as libc::c_int,
                        oa_base:
                            0 as *const libc::c_char as *mut libc::c_char,
                        oa_length: 0,};
        init
    };
#[no_mangle]
#[c2rust::src_loc = "45:8"]
pub static mut gssrpc_svc_fdset: fd_set = fd_set{__fds_bits: [0; 16],};
/* Will be zeroed in data segment */
#[no_mangle]
#[c2rust::src_loc = "46:5"]
pub static mut gssrpc_svc_fdset_init: libc::c_int = 0 as libc::c_int;
/* def FD_SETSIZE */
#[no_mangle]
#[c2rust::src_loc = "50:22"]
pub static mut gssrpc_rpc_createrr: gssrpc_rpc_createrr =
    {
        let mut init =
            gssrpc_rpc_createrr{cf_stat: RPC_SUCCESS,
                                cf_error:
                                    rpc_err{re_status: RPC_SUCCESS,
                                            ru:
                                                C2RustUnnamed{RE_errno:
                                                                  0,},},};
        init
    };
#[no_mangle]
#[c2rust::src_loc = "51:5"]
pub static mut gssrpc_svc_maxfd: libc::c_int = -(1 as libc::c_int);
