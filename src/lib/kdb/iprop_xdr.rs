use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:7"]
pub mod types_h {
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
}
#[c2rust::header_src = "/usr/include/sys/types.h:7"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:7"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:7"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:7"]
pub mod gssrpc_types_h {
    #[c2rust::src_loc = "93:1"]
    pub type rpc_inline_t = int32_t;
    use super::stdint_intn_h::int32_t;
    /* !defined(GSSRPC_TYPES_H) */
    /*
 * The below should probably be internal-only, but seem to be
 * traditionally exported in RPC implementations.
 */
    /* XXX namespace */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:7"]
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
        #[no_mangle]
        #[c2rust::src_loc = "252:1"]
        pub fn gssrpc_xdr_int(_: *mut XDR, _: *mut libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "253:1"]
        pub fn gssrpc_xdr_u_int(_: *mut XDR, _: *mut u_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "256:1"]
        pub fn gssrpc_xdr_short(_: *mut XDR, _: *mut libc::c_short)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "258:1"]
        pub fn gssrpc_xdr_bool(_: *mut XDR, _: *mut libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "259:1"]
        pub fn gssrpc_xdr_enum(_: *mut XDR, _: *mut libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "260:1"]
        pub fn gssrpc_xdr_array(_: *mut XDR, _: *mut caddr_t, _: *mut u_int,
                                _: u_int, _: u_int, _: xdrproc_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "262:1"]
        pub fn gssrpc_xdr_bytes(_: *mut XDR, _: *mut *mut libc::c_char,
                                _: *mut u_int, _: u_int) -> libc::c_int;
    }
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop.h:7"]
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
        pub k_enctype: C2RustUnnamed_0,
        pub k_contents: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:2"]
    pub struct C2RustUnnamed {
        pub k_contents_len: u_int,
        pub k_contents_val: *mut utf8str_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:2"]
    pub struct C2RustUnnamed_0 {
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
        pub k_components: C2RustUnnamed_1,
        pub k_nametype: int32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:2"]
    pub struct C2RustUnnamed_1 {
        pub k_components_len: u_int,
        pub k_components_val: *mut kdbe_data_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "60:8"]
    pub struct kdbe_tl_t {
        pub tl_type: int16_t,
        pub tl_data: C2RustUnnamed_2,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:2"]
    pub struct C2RustUnnamed_2 {
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
        pub kdbe_val_t_u: C2RustUnnamed_3,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "100:2"]
    pub union C2RustUnnamed_3 {
        pub av_attrflags: uint32_t,
        pub av_max_life: uint32_t,
        pub av_max_renew_life: uint32_t,
        pub av_exp: uint32_t,
        pub av_pw_exp: uint32_t,
        pub av_last_success: uint32_t,
        pub av_last_failed: uint32_t,
        pub av_fail_auth_count: uint32_t,
        pub av_princ: kdbe_princ_t,
        pub av_keydata: C2RustUnnamed_7,
        pub av_tldata: C2RustUnnamed_6,
        pub av_len: int16_t,
        pub av_pw_last_change: uint32_t,
        pub av_mod_princ: kdbe_princ_t,
        pub av_mod_time: uint32_t,
        pub av_mod_where: utf8str_t,
        pub av_pw_policy: utf8str_t,
        pub av_pw_policy_switch: libc::c_int,
        pub av_pw_hist_kvno: uint32_t,
        pub av_pw_hist: C2RustUnnamed_5,
        pub av_extension: C2RustUnnamed_4,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "130:3"]
    pub struct C2RustUnnamed_4 {
        pub av_extension_len: u_int,
        pub av_extension_val: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "126:3"]
    pub struct C2RustUnnamed_5 {
        pub av_pw_hist_len: u_int,
        pub av_pw_hist_val: *mut kdbe_pw_hist_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "114:3"]
    pub struct C2RustUnnamed_6 {
        pub av_tldata_len: u_int,
        pub av_tldata_val: *mut kdbe_tl_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "110:3"]
    pub struct C2RustUnnamed_7 {
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
        pub kdb_kdcs_seen_by: C2RustUnnamed_9,
        pub kdb_futures: C2RustUnnamed_8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "154:2"]
    pub struct C2RustUnnamed_8 {
        pub kdb_futures_len: u_int,
        pub kdb_futures_val: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "150:2"]
    pub struct C2RustUnnamed_9 {
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
    /* !_IPROP_H_RPCGEN */
    /* K&R C */
    /* K&R C */
}
pub use self::types_h::{__u_int, __int16_t, __int32_t, __uint32_t, __caddr_t};
pub use self::sys_types_h::{u_int, caddr_t};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssrpc_types_h::rpc_inline_t;
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, gssrpc_xdr_int, gssrpc_xdr_u_int,
                      gssrpc_xdr_short, gssrpc_xdr_bool, gssrpc_xdr_enum,
                      gssrpc_xdr_array, gssrpc_xdr_bytes};
pub use self::iprop_h::{utf8str_t, kdb_sno_t, kdbe_time_t, kdbe_key_t,
                        C2RustUnnamed, C2RustUnnamed_0, kdbe_data_t,
                        kdbe_princ_t, C2RustUnnamed_1, kdbe_tl_t,
                        C2RustUnnamed_2, kdbe_pw_hist_t, kdbe_attr_type_t,
                        AT_PW_HIST, AT_PW_HIST_KVNO, AT_PW_POLICY_SWITCH,
                        AT_PW_POLICY, AT_PW_LAST_CHANGE, AT_MOD_WHERE,
                        AT_MOD_TIME, AT_MOD_PRINC, AT_LEN, AT_TL_DATA,
                        AT_KEYDATA, AT_PRINC, AT_FAIL_AUTH_COUNT,
                        AT_LAST_FAILED, AT_LAST_SUCCESS, AT_PW_EXP, AT_EXP,
                        AT_MAX_RENEW_LIFE, AT_MAX_LIFE, AT_ATTRFLAGS,
                        kdbe_val_t, C2RustUnnamed_3, C2RustUnnamed_4,
                        C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7,
                        kdbe_t, kdb_incr_update_t, C2RustUnnamed_8,
                        C2RustUnnamed_9, kdb_ulog_t, update_status_t,
                        UPDATE_PERM_DENIED, UPDATE_NIL, UPDATE_BUSY,
                        UPDATE_FULL_RESYNC_NEEDED, UPDATE_ERROR, UPDATE_OK,
                        kdb_last_t, kdb_incr_result_t,
                        kdb_fullresync_result_t};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Please do not edit this file.
 * It was generated using rpcgen.
 */
#[c2rust::src_loc = "12:1"]
unsafe extern "C" fn xdr_int16_t(mut xdrs: *mut XDR, mut objp: *mut int16_t)
 -> libc::c_int {
    let mut buf: *mut int32_t = 0 as *mut int32_t;
    if gssrpc_xdr_short(xdrs, objp) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "22:1"]
unsafe extern "C" fn xdr_int32_t(mut xdrs: *mut XDR, mut objp: *mut int32_t)
 -> libc::c_int {
    let mut buf: *mut int32_t = 0 as *mut int32_t;
    if gssrpc_xdr_int(xdrs, objp) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "32:1"]
unsafe extern "C" fn xdr_uint32_t(mut xdrs: *mut XDR, mut objp: *mut uint32_t)
 -> libc::c_int {
    let mut buf: *mut int32_t = 0 as *mut int32_t;
    if gssrpc_xdr_u_int(xdrs, objp) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn xdr_utf8str_t(mut xdrs: *mut XDR,
                                       mut objp: *mut utf8str_t)
 -> libc::c_int {
    let mut buf: *mut int32_t = 0 as *mut int32_t;
    if gssrpc_xdr_bytes(xdrs,
                        &mut (*objp).utf8str_t_val as *mut *mut libc::c_char,
                        &mut (*objp).utf8str_t_len as *mut u_int,
                        !(0 as libc::c_int) as u_int) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "52:1"]
pub unsafe extern "C" fn xdr_kdb_sno_t(mut xdrs: *mut XDR,
                                       mut objp: *mut kdb_sno_t)
 -> libc::c_int {
    let mut buf: *mut int32_t = 0 as *mut int32_t;
    if xdr_uint32_t(xdrs, objp) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "62:1"]
pub unsafe extern "C" fn xdr_kdbe_time_t(mut xdrs: *mut XDR,
                                         mut objp: *mut kdbe_time_t)
 -> libc::c_int {
    let mut buf: *mut int32_t = 0 as *mut int32_t;
    if xdr_uint32_t(xdrs, &mut (*objp).seconds) == 0 {
        return 0 as libc::c_int
    }
    if xdr_uint32_t(xdrs, &mut (*objp).useconds) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "74:1"]
pub unsafe extern "C" fn xdr_kdbe_key_t(mut xdrs: *mut XDR,
                                        mut objp: *mut kdbe_key_t)
 -> libc::c_int {
    let mut buf: *mut int32_t = 0 as *mut int32_t;
    if xdr_int32_t(xdrs, &mut (*objp).k_ver) == 0 { return 0 as libc::c_int }
    if xdr_int32_t(xdrs, &mut (*objp).k_kvno) == 0 { return 0 as libc::c_int }
    if gssrpc_xdr_array(xdrs,
                        &mut (*objp).k_enctype.k_enctype_val as
                            *mut *mut int32_t as *mut *mut libc::c_char,
                        &mut (*objp).k_enctype.k_enctype_len as *mut u_int,
                        !(0 as libc::c_int) as u_int,
                        ::std::mem::size_of::<int32_t>() as libc::c_ulong as
                            u_int,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut int32_t)
                                                           -> libc::c_int>,
                                                xdrproc_t>(Some(xdr_int32_t as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut XDR,
                                                                                         _:
                                                                                             *mut int32_t)
                                                                        ->
                                                                            libc::c_int)))
           == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_array(xdrs,
                        &mut (*objp).k_contents.k_contents_val as
                            *mut *mut utf8str_t as *mut *mut libc::c_char,
                        &mut (*objp).k_contents.k_contents_len as *mut u_int,
                        !(0 as libc::c_int) as u_int,
                        ::std::mem::size_of::<utf8str_t>() as libc::c_ulong as
                            u_int,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut utf8str_t)
                                                           -> libc::c_int>,
                                                xdrproc_t>(Some(xdr_utf8str_t
                                                                    as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut XDR,
                                                                                         _:
                                                                                             *mut utf8str_t)
                                                                        ->
                                                                            libc::c_int)))
           == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn xdr_kdbe_data_t(mut xdrs: *mut XDR,
                                         mut objp: *mut kdbe_data_t)
 -> libc::c_int {
    let mut buf: *mut int32_t = 0 as *mut int32_t;
    if xdr_int32_t(xdrs, &mut (*objp).k_magic) == 0 {
        return 0 as libc::c_int
    }
    if xdr_utf8str_t(xdrs, &mut (*objp).k_data) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "104:1"]
pub unsafe extern "C" fn xdr_kdbe_princ_t(mut xdrs: *mut XDR,
                                          mut objp: *mut kdbe_princ_t)
 -> libc::c_int {
    let mut buf: *mut int32_t = 0 as *mut int32_t;
    if xdr_utf8str_t(xdrs, &mut (*objp).k_realm) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_array(xdrs,
                        &mut (*objp).k_components.k_components_val as
                            *mut *mut kdbe_data_t as *mut *mut libc::c_char,
                        &mut (*objp).k_components.k_components_len as
                            *mut u_int, !(0 as libc::c_int) as u_int,
                        ::std::mem::size_of::<kdbe_data_t>() as libc::c_ulong
                            as u_int,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut kdbe_data_t)
                                                           -> libc::c_int>,
                                                xdrproc_t>(Some(xdr_kdbe_data_t
                                                                    as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut XDR,
                                                                                         _:
                                                                                             *mut kdbe_data_t)
                                                                        ->
                                                                            libc::c_int)))
           == 0 {
        return 0 as libc::c_int
    }
    if xdr_int32_t(xdrs, &mut (*objp).k_nametype) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "119:1"]
pub unsafe extern "C" fn xdr_kdbe_tl_t(mut xdrs: *mut XDR,
                                       mut objp: *mut kdbe_tl_t)
 -> libc::c_int {
    let mut buf: *mut int32_t = 0 as *mut int32_t;
    if xdr_int16_t(xdrs, &mut (*objp).tl_type) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_bytes(xdrs,
                        &mut (*objp).tl_data.tl_data_val as
                            *mut *mut libc::c_char,
                        &mut (*objp).tl_data.tl_data_len as *mut u_int,
                        !(0 as libc::c_int) as u_int) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "131:1"]
pub unsafe extern "C" fn xdr_kdbe_pw_hist_t(mut xdrs: *mut XDR,
                                            mut objp: *mut kdbe_pw_hist_t)
 -> libc::c_int {
    let mut buf: *mut int32_t = 0 as *mut int32_t;
    if gssrpc_xdr_array(xdrs,
                        &mut (*objp).kdbe_pw_hist_t_val as
                            *mut *mut kdbe_key_t as *mut *mut libc::c_char,
                        &mut (*objp).kdbe_pw_hist_t_len as *mut u_int,
                        !(0 as libc::c_int) as u_int,
                        ::std::mem::size_of::<kdbe_key_t>() as libc::c_ulong
                            as u_int,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut kdbe_key_t)
                                                           -> libc::c_int>,
                                                xdrproc_t>(Some(xdr_kdbe_key_t
                                                                    as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut XDR,
                                                                                         _:
                                                                                             *mut kdbe_key_t)
                                                                        ->
                                                                            libc::c_int)))
           == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "142:1"]
pub unsafe extern "C" fn xdr_kdbe_attr_type_t(mut xdrs: *mut XDR,
                                              mut objp: *mut kdbe_attr_type_t)
 -> libc::c_int {
    let mut buf: *mut int32_t = 0 as *mut int32_t;
    if gssrpc_xdr_enum(xdrs, objp as *mut libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "152:1"]
pub unsafe extern "C" fn xdr_kdbe_val_t(mut xdrs: *mut XDR,
                                        mut objp: *mut kdbe_val_t)
 -> libc::c_int {
    let mut buf: *mut int32_t = 0 as *mut int32_t;
    if xdr_kdbe_attr_type_t(xdrs, &mut (*objp).av_type) == 0 {
        return 0 as libc::c_int
    }
    match (*objp).av_type as libc::c_uint {
        0 => {
            if xdr_uint32_t(xdrs, &mut (*objp).kdbe_val_t_u.av_attrflags) == 0
               {
                return 0 as libc::c_int
            }
        }
        1 => {
            if xdr_uint32_t(xdrs, &mut (*objp).kdbe_val_t_u.av_max_life) == 0
               {
                return 0 as libc::c_int
            }
        }
        2 => {
            if xdr_uint32_t(xdrs, &mut (*objp).kdbe_val_t_u.av_max_renew_life)
                   == 0 {
                return 0 as libc::c_int
            }
        }
        3 => {
            if xdr_uint32_t(xdrs, &mut (*objp).kdbe_val_t_u.av_exp) == 0 {
                return 0 as libc::c_int
            }
        }
        4 => {
            if xdr_uint32_t(xdrs, &mut (*objp).kdbe_val_t_u.av_pw_exp) == 0 {
                return 0 as libc::c_int
            }
        }
        5 => {
            if xdr_uint32_t(xdrs, &mut (*objp).kdbe_val_t_u.av_last_success)
                   == 0 {
                return 0 as libc::c_int
            }
        }
        6 => {
            if xdr_uint32_t(xdrs, &mut (*objp).kdbe_val_t_u.av_last_failed) ==
                   0 {
                return 0 as libc::c_int
            }
        }
        7 => {
            if xdr_uint32_t(xdrs,
                            &mut (*objp).kdbe_val_t_u.av_fail_auth_count) == 0
               {
                return 0 as libc::c_int
            }
        }
        8 => {
            if xdr_kdbe_princ_t(xdrs, &mut (*objp).kdbe_val_t_u.av_princ) == 0
               {
                return 0 as libc::c_int
            }
        }
        9 => {
            if gssrpc_xdr_array(xdrs,
                                &mut (*objp).kdbe_val_t_u.av_keydata.av_keydata_val
                                    as *mut *mut kdbe_key_t as
                                    *mut *mut libc::c_char,
                                &mut (*objp).kdbe_val_t_u.av_keydata.av_keydata_len
                                    as *mut u_int,
                                !(0 as libc::c_int) as u_int,
                                ::std::mem::size_of::<kdbe_key_t>() as
                                    libc::c_ulong as u_int,
                                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                        *mut XDR,
                                                                                    _:
                                                                                        *mut kdbe_key_t)
                                                                   ->
                                                                       libc::c_int>,
                                                        xdrproc_t>(Some(xdr_kdbe_key_t
                                                                            as
                                                                            unsafe extern "C" fn(_:
                                                                                                     *mut XDR,
                                                                                                 _:
                                                                                                     *mut kdbe_key_t)
                                                                                ->
                                                                                    libc::c_int)))
                   == 0 {
                return 0 as libc::c_int
            }
        }
        10 => {
            if gssrpc_xdr_array(xdrs,
                                &mut (*objp).kdbe_val_t_u.av_tldata.av_tldata_val
                                    as *mut *mut kdbe_tl_t as
                                    *mut *mut libc::c_char,
                                &mut (*objp).kdbe_val_t_u.av_tldata.av_tldata_len
                                    as *mut u_int,
                                !(0 as libc::c_int) as u_int,
                                ::std::mem::size_of::<kdbe_tl_t>() as
                                    libc::c_ulong as u_int,
                                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                        *mut XDR,
                                                                                    _:
                                                                                        *mut kdbe_tl_t)
                                                                   ->
                                                                       libc::c_int>,
                                                        xdrproc_t>(Some(xdr_kdbe_tl_t
                                                                            as
                                                                            unsafe extern "C" fn(_:
                                                                                                     *mut XDR,
                                                                                                 _:
                                                                                                     *mut kdbe_tl_t)
                                                                                ->
                                                                                    libc::c_int)))
                   == 0 {
                return 0 as libc::c_int
            }
        }
        11 => {
            if xdr_int16_t(xdrs, &mut (*objp).kdbe_val_t_u.av_len) == 0 {
                return 0 as libc::c_int
            }
        }
        15 => {
            if xdr_uint32_t(xdrs, &mut (*objp).kdbe_val_t_u.av_pw_last_change)
                   == 0 {
                return 0 as libc::c_int
            }
        }
        12 => {
            if xdr_kdbe_princ_t(xdrs, &mut (*objp).kdbe_val_t_u.av_mod_princ)
                   == 0 {
                return 0 as libc::c_int
            }
        }
        13 => {
            if xdr_uint32_t(xdrs, &mut (*objp).kdbe_val_t_u.av_mod_time) == 0
               {
                return 0 as libc::c_int
            }
        }
        14 => {
            if xdr_utf8str_t(xdrs, &mut (*objp).kdbe_val_t_u.av_mod_where) ==
                   0 {
                return 0 as libc::c_int
            }
        }
        16 => {
            if xdr_utf8str_t(xdrs, &mut (*objp).kdbe_val_t_u.av_pw_policy) ==
                   0 {
                return 0 as libc::c_int
            }
        }
        17 => {
            if gssrpc_xdr_bool(xdrs,
                               &mut (*objp).kdbe_val_t_u.av_pw_policy_switch)
                   == 0 {
                return 0 as libc::c_int
            }
        }
        18 => {
            if xdr_uint32_t(xdrs, &mut (*objp).kdbe_val_t_u.av_pw_hist_kvno)
                   == 0 {
                return 0 as libc::c_int
            }
        }
        19 => {
            if gssrpc_xdr_array(xdrs,
                                &mut (*objp).kdbe_val_t_u.av_pw_hist.av_pw_hist_val
                                    as *mut *mut kdbe_pw_hist_t as
                                    *mut *mut libc::c_char,
                                &mut (*objp).kdbe_val_t_u.av_pw_hist.av_pw_hist_len
                                    as *mut u_int,
                                !(0 as libc::c_int) as u_int,
                                ::std::mem::size_of::<kdbe_pw_hist_t>() as
                                    libc::c_ulong as u_int,
                                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                        *mut XDR,
                                                                                    _:
                                                                                        *mut kdbe_pw_hist_t)
                                                                   ->
                                                                       libc::c_int>,
                                                        xdrproc_t>(Some(xdr_kdbe_pw_hist_t
                                                                            as
                                                                            unsafe extern "C" fn(_:
                                                                                                     *mut XDR,
                                                                                                 _:
                                                                                                     *mut kdbe_pw_hist_t)
                                                                                ->
                                                                                    libc::c_int)))
                   == 0 {
                return 0 as libc::c_int
            }
        }
        _ => {
            if gssrpc_xdr_bytes(xdrs,
                                &mut (*objp).kdbe_val_t_u.av_extension.av_extension_val
                                    as *mut *mut libc::c_char,
                                &mut (*objp).kdbe_val_t_u.av_extension.av_extension_len
                                    as *mut u_int,
                                !(0 as libc::c_int) as u_int) == 0 {
                return 0 as libc::c_int
            }
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "251:1"]
pub unsafe extern "C" fn xdr_kdbe_t(mut xdrs: *mut XDR, mut objp: *mut kdbe_t)
 -> libc::c_int {
    let mut buf: *mut int32_t = 0 as *mut int32_t;
    if gssrpc_xdr_array(xdrs,
                        &mut (*objp).kdbe_t_val as *mut *mut kdbe_val_t as
                            *mut *mut libc::c_char,
                        &mut (*objp).kdbe_t_len as *mut u_int,
                        !(0 as libc::c_int) as u_int,
                        ::std::mem::size_of::<kdbe_val_t>() as libc::c_ulong
                            as u_int,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut kdbe_val_t)
                                                           -> libc::c_int>,
                                                xdrproc_t>(Some(xdr_kdbe_val_t
                                                                    as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut XDR,
                                                                                         _:
                                                                                             *mut kdbe_val_t)
                                                                        ->
                                                                            libc::c_int)))
           == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "262:1"]
pub unsafe extern "C" fn xdr_kdb_incr_update_t(mut xdrs: *mut XDR,
                                               mut objp:
                                                   *mut kdb_incr_update_t)
 -> libc::c_int {
    let mut buf: *mut int32_t = 0 as *mut int32_t;
    if xdr_utf8str_t(xdrs, &mut (*objp).kdb_princ_name) == 0 {
        return 0 as libc::c_int
    }
    if xdr_kdb_sno_t(xdrs, &mut (*objp).kdb_entry_sno) == 0 {
        return 0 as libc::c_int
    }
    if xdr_kdbe_time_t(xdrs, &mut (*objp).kdb_time) == 0 {
        return 0 as libc::c_int
    }
    if xdr_kdbe_t(xdrs, &mut (*objp).kdb_update) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_bool(xdrs, &mut (*objp).kdb_deleted) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_bool(xdrs, &mut (*objp).kdb_commit) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_array(xdrs,
                        &mut (*objp).kdb_kdcs_seen_by.kdb_kdcs_seen_by_val as
                            *mut *mut utf8str_t as *mut *mut libc::c_char,
                        &mut (*objp).kdb_kdcs_seen_by.kdb_kdcs_seen_by_len as
                            *mut u_int, !(0 as libc::c_int) as u_int,
                        ::std::mem::size_of::<utf8str_t>() as libc::c_ulong as
                            u_int,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut utf8str_t)
                                                           -> libc::c_int>,
                                                xdrproc_t>(Some(xdr_utf8str_t
                                                                    as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut XDR,
                                                                                         _:
                                                                                             *mut utf8str_t)
                                                                        ->
                                                                            libc::c_int)))
           == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_bytes(xdrs,
                        &mut (*objp).kdb_futures.kdb_futures_val as
                            *mut *mut libc::c_char,
                        &mut (*objp).kdb_futures.kdb_futures_len as
                            *mut u_int, !(0 as libc::c_int) as u_int) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "287:1"]
pub unsafe extern "C" fn xdr_kdb_ulog_t(mut xdrs: *mut XDR,
                                        mut objp: *mut kdb_ulog_t)
 -> libc::c_int {
    let mut buf: *mut int32_t = 0 as *mut int32_t;
    if gssrpc_xdr_array(xdrs,
                        &mut (*objp).kdb_ulog_t_val as
                            *mut *mut kdb_incr_update_t as
                            *mut *mut libc::c_char,
                        &mut (*objp).kdb_ulog_t_len as *mut u_int,
                        !(0 as libc::c_int) as u_int,
                        ::std::mem::size_of::<kdb_incr_update_t>() as
                            libc::c_ulong as u_int,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut kdb_incr_update_t)
                                                           -> libc::c_int>,
                                                xdrproc_t>(Some(xdr_kdb_incr_update_t
                                                                    as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut XDR,
                                                                                         _:
                                                                                             *mut kdb_incr_update_t)
                                                                        ->
                                                                            libc::c_int)))
           == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "298:1"]
pub unsafe extern "C" fn xdr_update_status_t(mut xdrs: *mut XDR,
                                             mut objp: *mut update_status_t)
 -> libc::c_int {
    let mut buf: *mut int32_t = 0 as *mut int32_t;
    if gssrpc_xdr_enum(xdrs, objp as *mut libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "308:1"]
pub unsafe extern "C" fn xdr_kdb_last_t(mut xdrs: *mut XDR,
                                        mut objp: *mut kdb_last_t)
 -> libc::c_int {
    let mut buf: *mut int32_t = 0 as *mut int32_t;
    if xdr_kdb_sno_t(xdrs, &mut (*objp).last_sno) == 0 {
        return 0 as libc::c_int
    }
    if xdr_kdbe_time_t(xdrs, &mut (*objp).last_time) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "320:1"]
pub unsafe extern "C" fn xdr_kdb_incr_result_t(mut xdrs: *mut XDR,
                                               mut objp:
                                                   *mut kdb_incr_result_t)
 -> libc::c_int {
    let mut buf: *mut int32_t = 0 as *mut int32_t;
    if xdr_kdb_last_t(xdrs, &mut (*objp).lastentry) == 0 {
        return 0 as libc::c_int
    }
    if xdr_kdb_ulog_t(xdrs, &mut (*objp).updates) == 0 {
        return 0 as libc::c_int
    }
    if xdr_update_status_t(xdrs, &mut (*objp).ret) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/*
 * Please do not edit this file.
 * It was generated using rpcgen.
 */
/* K&R C */
/* K&R C */
/* the xdr functions */
#[no_mangle]
#[c2rust::src_loc = "334:1"]
pub unsafe extern "C" fn xdr_kdb_fullresync_result_t(mut xdrs: *mut XDR,
                                                     mut objp:
                                                         *mut kdb_fullresync_result_t)
 -> libc::c_int {
    let mut buf: *mut int32_t = 0 as *mut int32_t;
    if xdr_kdb_last_t(xdrs, &mut (*objp).lastentry) == 0 {
        return 0 as libc::c_int
    }
    if xdr_update_status_t(xdrs, &mut (*objp).ret) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
