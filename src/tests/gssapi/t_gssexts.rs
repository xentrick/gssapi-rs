use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:26"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:26"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:26"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:26"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/stdio.h:26"]
pub mod stdio_h {
    #[c2rust::src_loc = "77:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/sys/types.h:27"]
pub mod sys_types_h {
    #[c2rust::src_loc = "79:1"]
    pub type uid_t = __uid_t;
    use super::types_h::__uid_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:27"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:30"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:30"]
pub mod gssapi_h {
    #[c2rust::src_loc = "79:1"]
    pub type gss_name_t = *mut gss_name_struct;
    #[c2rust::src_loc = "82:1"]
    pub type gss_cred_id_t = *mut gss_cred_id_struct;
    #[c2rust::src_loc = "85:1"]
    pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
    /*
 * The following type must be defined as the smallest natural unsigned integer
 * supported by the platform that has at least 32 bits of precision.
 */
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    /* OM_STRING */
    /*
 * We can't use X/Open definitions, so roll our own.
 */
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
    pub type gss_OID_desc = gss_OID_desc_struct;
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    /* OM_STRING */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:16"]
    pub struct gss_OID_set_desc_struct {
        pub count: size_t,
        pub elements: gss_OID,
    }
    #[c2rust::src_loc = "112:1"]
    pub type gss_OID_set_desc = gss_OID_set_desc_struct;
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
    #[c2rust::src_loc = "135:1"]
    pub type gss_cred_usage_t = libc::c_int;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    use super::stdio_h::ssize_t;
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
        /* token_buffer */
        #[no_mangle]
        #[c2rust::src_loc = "474:1"]
        pub fn gss_delete_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_buffer_t) -> OM_uint32;
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
        /* buffer */
        #[no_mangle]
        #[c2rust::src_loc = "579:1"]
        pub fn gss_release_oid_set(_: *mut OM_uint32, _: *mut gss_OID_set)
         -> OM_uint32;
        /* set */
        #[no_mangle]
        #[c2rust::src_loc = "584:1"]
        pub fn gss_inquire_cred(_: *mut OM_uint32, _: gss_cred_id_t,
                                _: *mut gss_name_t, _: *mut OM_uint32,
                                _: *mut gss_cred_usage_t, _: *mut gss_OID_set)
         -> OM_uint32;
        /* output_name */
        /* RFC 4401 */
        #[no_mangle]
        #[c2rust::src_loc = "796:1"]
        pub fn gss_pseudo_random(_: *mut OM_uint32, _: gss_ctx_id_t,
                                 _: libc::c_int, _: gss_buffer_t, _: ssize_t,
                                 _: gss_buffer_t) -> OM_uint32;
        /* prf_out */
        #[no_mangle]
        #[c2rust::src_loc = "805:1"]
        pub fn gss_store_cred(_: *mut OM_uint32, _: gss_cred_id_t,
                              _: gss_cred_usage_t, _: gss_OID, _: OM_uint32,
                              _: OM_uint32, _: *mut gss_OID_set,
                              _: *mut gss_cred_usage_t) -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:30"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
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
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
    #[c2rust::src_loc = "2736:1"]
    pub type krb5_keytab = *mut _krb5_kt;
    use super::stdint_intn_h::int32_t;
    extern "C" {
        #[c2rust::src_loc = "350:8"]
        pub type _krb5_context;
        #[c2rust::src_loc = "2280:8"]
        pub type _krb5_ccache;
        #[c2rust::src_loc = "2735:8"]
        pub type _krb5_kt;
        #[no_mangle]
        #[c2rust::src_loc = "2403:1"]
        pub fn krb5_cc_close(context: krb5_context, cache: krb5_ccache)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2479:1"]
        pub fn krb5_cc_get_principal(context: krb5_context,
                                     cache: krb5_ccache,
                                     principal: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2782:1"]
        pub fn krb5_kt_close(context: krb5_context, keytab: krb5_keytab)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4425:1"]
        pub fn krb5_cc_default(context: krb5_context,
                               ccache: *mut krb5_ccache) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "2922:1"]
        pub fn krb5_init_context(context: *mut krb5_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2972:1"]
        pub fn krb5_free_context(context: krb5_context);
        #[no_mangle]
        #[c2rust::src_loc = "4173:1"]
        pub fn krb5_kt_resolve(context: krb5_context,
                               name: *const libc::c_char,
                               ktid: *mut krb5_keytab) -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/string.h:28"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_ext.h:30"]
pub mod gssapi_ext_h {
    use super::gssapi_h::{OM_uint32, gss_name_struct, gss_name_t,
                          gss_OID_desc_struct, gss_OID, gss_cred_id_struct,
                          gss_cred_id_t, gss_OID_set_desc_struct, gss_OID_set,
                          gss_cred_usage_t};
    use super::sys_types_h::uid_t;
    extern "C" {
        /*
 * Copyright 2008 by the Massachusetts Institute of Technology.
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
        /* __cplusplus */
        /*
 * Solaris extensions
 */
        #[no_mangle]
        #[c2rust::src_loc = "38:1"]
        pub fn gss_pname_to_uid(minor: *mut OM_uint32, name: gss_name_t,
                                mech_type: gss_OID, uidOut: *mut uid_t)
         -> OM_uint32;
        /* iov_count */
        /*
 * Protocol transition
 */
        #[no_mangle]
        #[c2rust::src_loc = "403:1"]
        pub fn gss_acquire_cred_impersonate_name(_: *mut OM_uint32,
                                                 _: gss_cred_id_t,
                                                 _: gss_name_t, _: OM_uint32,
                                                 _: gss_OID_set,
                                                 _: gss_cred_usage_t,
                                                 _: *mut gss_cred_id_t,
                                                 _: *mut gss_OID_set,
                                                 _: *mut OM_uint32)
         -> OM_uint32;
    }
    /* GSSAPI_EXT_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/tests/gssapi/common.h:30"]
pub mod common_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID, gss_name_struct,
                          gss_name_t, gss_cred_id_struct, gss_cred_id_t,
                          OM_uint32, gss_ctx_id_t, gss_OID_set_desc,
                          gss_OID_desc};
    use super::krb5_h::{_krb5_context, krb5_context, krb5_error_code};
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "73:1"]
        pub fn display_oid(tag: *const libc::c_char, oid: gss_OID);
        #[no_mangle]
        #[c2rust::src_loc = "70:1"]
        pub fn display_canon_name(tag: *const libc::c_char, name: gss_name_t,
                                  mech: gss_OID);
        #[no_mangle]
        #[c2rust::src_loc = "59:1"]
        pub fn establish_contexts(imech: gss_OID, icred: gss_cred_id_t,
                                  acred: gss_cred_id_t, tname: gss_name_t,
                                  flags: OM_uint32, ictx: *mut gss_ctx_id_t,
                                  actx: *mut gss_ctx_id_t,
                                  src_name: *mut gss_name_t,
                                  amech: *mut gss_OID,
                                  deleg_cred: *mut gss_cred_id_t);
        #[no_mangle]
        #[c2rust::src_loc = "56:1"]
        pub fn import_name(str: *const libc::c_char) -> gss_name_t;
        #[no_mangle]
        #[c2rust::src_loc = "49:1"]
        pub fn check_k5err(context: krb5_context, msg: *const libc::c_char,
                           code: krb5_error_code);
        #[no_mangle]
        #[c2rust::src_loc = "46:1"]
        pub fn check_gsserr(msg: *const libc::c_char, major: OM_uint32,
                            minor: OM_uint32);
        #[no_mangle]
        #[c2rust::src_loc = "42:25"]
        pub static mut mechset_spnego: gss_OID_set_desc;
        #[no_mangle]
        #[c2rust::src_loc = "41:25"]
        pub static mut mechset_krb5: gss_OID_set_desc;
        #[no_mangle]
        #[c2rust::src_loc = "39:21"]
        pub static mut mech_spnego: gss_OID_desc;
        #[no_mangle]
        #[c2rust::src_loc = "38:21"]
        pub static mut mech_krb5: gss_OID_desc;
    }
    /* COMMON_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_krb5.h:30"]
pub mod gssapi_krb5_h {
    use super::gssapi_h::{OM_uint32, gss_cred_id_t};
    use super::krb5_h::{_krb5_ccache, krb5_ccache, krb5_principal_data,
                        krb5_principal, _krb5_kt, krb5_keytab};
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
        /* This name form shall be represented by the Object Identifier {iso(1)
 * member-body(2) United States(840) mit(113554) infosys(1) gssapi(2)
 * krb5(2) krb5_name(1)}.  The recommended symbolic name for this type
 * is "GSS_KRB5_NT_PRINCIPAL_NAME". */
        /* 2.1.2. Host-Based Service Name Form */
        /* This name form shall be represented by the Object Identifier {iso(1)
 * member-body(2) United States(840) mit(113554) infosys(1) gssapi(2)
 * generic(1) service_name(4)}.  The previously recommended symbolic
 * name for this type is "GSS_KRB5_NT_HOSTBASED_SERVICE_NAME".  The
 * currently preferred symbolic name for this type is
 * "GSS_C_NT_HOSTBASED_SERVICE". */
        /* 2.2.1. User Name Form */
        /* This name form shall be represented by the Object Identifier {iso(1)
 * member-body(2) United States(840) mit(113554) infosys(1) gssapi(2)
 * generic(1) user_name(1)}.  The recommended symbolic name for this
 * type is "GSS_KRB5_NT_USER_NAME". */
        /* 2.2.2. Machine UID Form */
        /* This name form shall be represented by the Object Identifier {iso(1)
 * member-body(2) United States(840) mit(113554) infosys(1) gssapi(2)
 * generic(1) machine_uid_name(2)}.  The recommended symbolic name for
 * this type is "GSS_KRB5_NT_MACHINE_UID_NAME". */
        /* 2.2.3. String UID Form */
        /* This name form shall be represented by the Object Identifier {iso(1)
 * member-body(2) United States(840) mit(113554) infosys(1) gssapi(2)
 * generic(1) string_uid_name(3)}.  The recommended symbolic name for
 * this type is "GSS_KRB5_NT_STRING_UID_NAME". */
        /* Kerberos Enterprise Name Form (see RFC 6806 section 5): */
        /* {iso(1) member-body(2) United States(840) mit(113554) infosys(1) gssapi(2)
 * krb5(2) krb5-enterprise-name(6)}. */
        /*
 * This OID can be used with gss_set_cred_option() to suppress the
 * confidentiality and integrity flags from being asserted in initial context
 * tokens.
 *
 * iso(1) member-body(2) Sweden(752) Stockholm University(43) Heimdal GSS-API
 * Extensions(13) no_ci_flags(29)
 */
        /*
 * This OID can be used with gss_inquire_cred_by_oid(0 to retrieve the
 * impersonator name (if any).
 *
 * iso(1) member-body(2) United States(840) mit(113554) infosys(1) gssapi(2)
 * krb5(2) krb5-gssapi-ext(5) get-cred-impersonator(14)
 */
        /* key encryption type */
        /* length of key data */
        /* actual key data */
        /* signing algorthm */
        /* seal/encrypt algorithm */
        /* Context key
       (Kerberos session key or subkey) */
        /* 1 if there is an acceptor_subkey
       present, 0 otherwise */
        /* Context key
       (Kerberos session key or subkey) */
        /* acceptor-asserted subkey or
       0's if no acceptor subkey */
        /* Structure version number (1)
                                       MUST be at beginning of struct! */
        /* Are we the initiator? */
        /* expiration time of context */
        /* sender sequence number */
        /* receive sequence number */
        /* 0: rfc1964,
                                       1: draft-ietf-krb-wg-gssapi-cfx-07 */
        /*
     * if (protocol == 0) rfc1964_kd should be used
     * and cfx_kd contents are invalid and should be zero
     * if (protocol == 1) cfx_kd should be used
     * and rfc1964_kd contents are invalid and should be zero
     */
        /*
 * Mask for determining the version of a lucid context structure.  Callers
 * should not require this.
 */
        /* Structure version number */
        /* Alias for Heimdal compat. */
        /*
 * Copy krb5 creds from cred_handle into out_ccache, which must already be
 * initialized.  Use gss_store_cred_into() (new in krb5 1.11) instead, if
 * possible.
 */
        /*
 * gss_krb5_set_allowable_enctypes
 *
 * This function may be called by a context initiator after calling
 * gss_acquire_cred(), but before calling gss_init_sec_context(),
 * to restrict the set of enctypes which will be negotiated during
 * context establishment to those in the provided array.
 *
 * 'cred' must be a valid credential handle obtained via
 * gss_acquire_cred().  It may not be GSS_C_NO_CREDENTIAL.
 * gss_acquire_cred() may have been called to get a handle to
 * the default credential.
 *
 * The purpose of this function is to limit the keys that may
 * be exported via gss_krb5_export_lucid_sec_context(); thus it
 * should limit the enctypes of all keys that will be needed
 * after the security context has been established.
 * (i.e. context establishment may use a session key with a
 * stronger enctype than in the provided array, however a
 * subkey must be established within the enctype limits
 * established by this function.)
 *
 */
        /*
 * Returns a non-opaque (lucid) version of the internal context
 * information.
 *
 * Note that context_handle must not be used again by the caller
 * after this call.  The GSS implementation is free to release any
 * resources associated with the original context.  It is up to the
 * GSS implementation whether it returns pointers to existing data,
 * or copies of the data.  The caller should treat the returned
 * lucid context as read-only.
 *
 * The caller must call gss_krb5_free_lucid_context() to free
 * the context and allocated resources when it is finished with it.
 *
 * 'version' is an integer indicating the requested version of the lucid
 * context.  If the implementation does not understand the requested version,
 * it will return an error.
 *
 * For example:
 *      void *return_ctx;
 *      gss_krb5_lucid_context_v1_t *ctx;
 *      OM_uint32 min_stat, maj_stat;
 *      OM_uint32 vers;
 *      gss_ctx_id_t *ctx_handle;
 *
 *      maj_stat = gss_krb5_export_lucid_sec_context(&min_stat,
 *                      ctx_handle, 1, &return_ctx);
 *      // Verify success
 *      ctx = (gss_krb5_lucid_context_v1_t *) return_ctx;
 */
        /*
 * Frees the allocated storage associated with an
 * exported struct gss_krb5_lucid_context.
 */
        #[no_mangle]
        #[c2rust::src_loc = "289:1"]
        pub fn gss_krb5_import_cred(minor_status: *mut OM_uint32,
                                    id: krb5_ccache,
                                    keytab_principal: krb5_principal,
                                    keytab: krb5_keytab,
                                    cred: *mut gss_cred_id_t) -> OM_uint32;
    }
    /* _GSSAPI_KRB5_H_ */
    /* __cplusplus */
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__int32_t, __uint32_t, __uid_t, __off_t, __off64_t,
                        __ssize_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stdio_h::{ssize_t, stderr, fprintf, printf};
pub use self::sys_types_h::uid_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssapi_h::{gss_name_t, gss_cred_id_t, gss_ctx_id_t, gss_uint32,
                         OM_uint32, gss_OID_desc_struct, gss_OID_desc,
                         gss_OID, gss_OID_set_desc_struct, gss_OID_set_desc,
                         gss_OID_set, gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_cred_usage_t, gss_name_struct,
                         gss_cred_id_struct, gss_ctx_id_struct,
                         gss_acquire_cred, gss_release_cred,
                         gss_delete_sec_context, gss_release_name,
                         gss_release_buffer, gss_release_oid_set,
                         gss_inquire_cred, gss_pseudo_random, gss_store_cred};
pub use self::krb5_h::{krb5_int32, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_principal_data, krb5_principal,
                       krb5_context, krb5_ccache, krb5_keytab, _krb5_context,
                       _krb5_ccache, _krb5_kt, krb5_cc_close,
                       krb5_cc_get_principal, krb5_kt_close, krb5_cc_default,
                       krb5_free_principal, krb5_init_context,
                       krb5_free_context, krb5_kt_resolve};
use self::stdlib_h::exit;
use self::string_h::{memcmp, strcmp, strlen};
use self::gssapi_ext_h::{gss_pname_to_uid, gss_acquire_cred_impersonate_name};
use self::common_h::{display_oid, display_canon_name, establish_contexts,
                     import_name, check_k5err, check_gsserr, mechset_spnego,
                     mechset_krb5, mech_spnego, mech_krb5};
use self::gssapi_krb5_h::gss_krb5_import_cred;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2009  by the Massachusetts Institute of Technology.
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
 * Test program for protocol transition (S4U2Self) and constrained delegation
 * (S4U2Proxy)
 *
 * Note: because of name canonicalization, the following tips may help
 * when configuring with Active Directory:
 *
 * - Create a computer account FOO$
 * - Set the UPN to host/foo.domain (no suffix); this is necessary to
 *   be able to send an AS-REQ as this principal, otherwise you would
 *   need to use the canonical name (FOO$), which will cause principal
 *   comparison errors in gss_accept_sec_context().
 * - Add a SPN of host/foo.domain
 * - Configure the computer account to support constrained delegation with
 *   protocol transition (Trust this computer for delegation to specified
 *   services only / Use any authentication protocol)
 * - Add host/foo.domain to the keytab (possibly easiest to do this
 *   with ktadd)
 *
 * For S4U2Proxy to work the TGT must be forwardable too.
 *
 * Usage eg:
 *
 * kinit -k -t test.keytab -f 'host/test.win.mit.edu@WIN.MIT.EDU'
 * ./t_s4u p:delegtest@WIN.MIT.EDU p:HOST/WIN-EQ7E4AA2WR8.win.mit.edu@WIN.MIT.EDU test.keytab
 */
#[c2rust::src_loc = "59:12"]
static mut use_spnego: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "61:1"]
unsafe extern "C" fn test_prf(mut initiatorContext: gss_ctx_id_t,
                              mut acceptorContext: gss_ctx_id_t,
                              mut flags: libc::c_int) {
    let mut constant: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut i: libc::c_uint = 0;
    let mut initiatorPrf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut acceptorPrf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    constant.value =
        b"gss prf test\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_void;
    constant.length = strlen(constant.value as *mut libc::c_char);
    initiatorPrf.value = 0 as *mut libc::c_void;
    acceptorPrf.value = 0 as *mut libc::c_void;
    major =
        gss_pseudo_random(&mut minor, initiatorContext, flags, &mut constant,
                          19 as libc::c_int as ssize_t, &mut initiatorPrf);
    check_gsserr(b"gss_pseudo_random\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           if flags == 0 as libc::c_int {
               b"PRF_KEY_FULL\x00" as *const u8 as *const libc::c_char
           } else {
               b"PRF_KEY_PARTIAL\x00" as *const u8 as *const libc::c_char
           });
    printf(b"Initiator PRF: \x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < initiatorPrf.length {
        printf(b"%02x \x00" as *const u8 as *const libc::c_char,
               *(initiatorPrf.value as *mut libc::c_char).offset(i as isize)
                   as libc::c_int & 0xff as libc::c_int);
        i = i.wrapping_add(1)
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    major =
        gss_pseudo_random(&mut minor, acceptorContext, flags, &mut constant,
                          19 as libc::c_int as ssize_t, &mut acceptorPrf);
    check_gsserr(b"gss_pseudo_random\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    printf(b"Acceptor  PRF: \x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < acceptorPrf.length {
        printf(b"%02x \x00" as *const u8 as *const libc::c_char,
               *(acceptorPrf.value as *mut libc::c_char).offset(i as isize) as
                   libc::c_int & 0xff as libc::c_int);
        i = i.wrapping_add(1)
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    if acceptorPrf.length != initiatorPrf.length ||
           memcmp(acceptorPrf.value, initiatorPrf.value, initiatorPrf.length)
               != 0 {
        fprintf(stderr,
                b"Initiator and acceptor PRF output does not match\n\x00" as
                    *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    gss_release_buffer(&mut minor, &mut initiatorPrf);
    gss_release_buffer(&mut minor, &mut acceptorPrf);
}
#[c2rust::src_loc = "108:1"]
unsafe extern "C" fn init_accept_sec_context(mut claimant_cred_handle:
                                                 gss_cred_id_t,
                                             mut verifier_cred_handle:
                                                 gss_cred_id_t,
                                             mut deleg_cred_handle:
                                                 *mut gss_cred_id_t) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut flags: OM_uint32 = 0;
    let mut source_name: gss_name_t = 0 as gss_name_t;
    let mut target_name: gss_name_t = 0 as gss_name_t;
    let mut initiator_context: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut acceptor_context: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut mech: gss_OID = 0 as *mut gss_OID_desc_struct;
    *deleg_cred_handle = 0 as gss_cred_id_t;
    major =
        gss_inquire_cred(&mut minor, verifier_cred_handle, &mut target_name,
                         0 as *mut OM_uint32, 0 as *mut gss_cred_usage_t,
                         0 as *mut gss_OID_set);
    check_gsserr(b"gss_inquire_cred\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    display_canon_name(b"Target name\x00" as *const u8 as *const libc::c_char,
                       target_name, &mut mech_krb5);
    mech = if use_spnego != 0 { &mut mech_spnego } else { &mut mech_krb5 };
    display_oid(b"Target mech\x00" as *const u8 as *const libc::c_char, mech);
    flags = (4 as libc::c_int | 8 as libc::c_int) as OM_uint32;
    establish_contexts(mech, claimant_cred_handle, verifier_cred_handle,
                       target_name, flags, &mut initiator_context,
                       &mut acceptor_context, &mut source_name,
                       0 as *mut gss_OID, deleg_cred_handle);
    test_prf(initiator_context, acceptor_context, 0 as libc::c_int);
    test_prf(initiator_context, acceptor_context, 1 as libc::c_int);
    gss_release_name(&mut minor, &mut source_name);
    gss_delete_sec_context(&mut minor, &mut acceptor_context,
                           0 as gss_buffer_t);
    gss_delete_sec_context(&mut minor, &mut initiator_context,
                           0 as gss_buffer_t);
}
#[c2rust::src_loc = "142:1"]
unsafe extern "C" fn get_default_cred(mut keytab_name: *const libc::c_char,
                                      mut mechs: gss_OID_set,
                                      mut impersonator_cred_handle:
                                          *mut gss_cred_id_t) {
    let mut major: OM_uint32 =
        (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
    let mut minor: OM_uint32 = 0;
    let mut ret: krb5_error_code = 0;
    let mut context: krb5_context = 0 as krb5_context;
    let mut keytab: krb5_keytab = 0 as krb5_keytab;
    let mut keytab_principal: krb5_principal = 0 as krb5_principal;
    let mut ccache: krb5_ccache = 0 as krb5_ccache;
    if !keytab_name.is_null() {
        ret = krb5_init_context(&mut context);
        check_k5err(context,
                    b"krb5_init_context\x00" as *const u8 as
                        *const libc::c_char, ret);
        ret = krb5_kt_resolve(context, keytab_name, &mut keytab);
        check_k5err(context,
                    b"krb5_kt_resolve\x00" as *const u8 as
                        *const libc::c_char, ret);
        ret = krb5_cc_default(context, &mut ccache);
        check_k5err(context,
                    b"krb5_cc_default\x00" as *const u8 as
                        *const libc::c_char, ret);
        ret = krb5_cc_get_principal(context, ccache, &mut keytab_principal);
        check_k5err(context,
                    b"krb5_cc_get_principal\x00" as *const u8 as
                        *const libc::c_char, ret);
        major =
            gss_krb5_import_cred(&mut minor, ccache, keytab_principal, keytab,
                                 impersonator_cred_handle);
        check_gsserr(b"gss_krb5_import_cred\x00" as *const u8 as
                         *const libc::c_char, major, minor);
        krb5_free_principal(context, keytab_principal);
        krb5_cc_close(context, ccache);
        krb5_kt_close(context, keytab);
        krb5_free_context(context);
    } else {
        major =
            gss_acquire_cred(&mut minor, 0 as gss_name_t,
                             0xffffffff as libc::c_ulong as OM_uint32, mechs,
                             0 as libc::c_int, impersonator_cred_handle,
                             0 as *mut gss_OID_set, 0 as *mut OM_uint32);
        check_gsserr(b"gss_acquire_cred\x00" as *const u8 as
                         *const libc::c_char, major, minor);
    };
}
#[c2rust::src_loc = "182:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut minor: OM_uint32 = 0;
    let mut major: OM_uint32 = 0;
    let mut impersonator_cred_handle: gss_cred_id_t = 0 as gss_cred_id_t;
    let mut user_cred_handle: gss_cred_id_t = 0 as gss_cred_id_t;
    let mut delegated_cred_handle: gss_cred_id_t = 0 as gss_cred_id_t;
    let mut user: gss_name_t = 0 as gss_name_t;
    let mut target: gss_name_t = 0 as gss_name_t;
    let mut mechs: gss_OID_set = 0 as *mut gss_OID_set_desc_struct;
    let mut actual_mechs: gss_OID_set = 0 as gss_OID_set;
    let mut uid: uid_t = 0;
    if argc < 2 as libc::c_int || argc > 5 as libc::c_int {
        fprintf(stderr,
                b"Usage: %s [--spnego] [user] [proxy-target] [keytab]\n\x00"
                    as *const u8 as *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize));
        fprintf(stderr,
                b"       proxy-target and keytab are optional\n\x00" as
                    *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if strcmp(*argv.offset(1 as libc::c_int as isize),
              b"--spnego\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        use_spnego += 1;
        argc -= 1;
        argv = argv.offset(1)
    }
    user = import_name(*argv.offset(1 as libc::c_int as isize));
    major = gss_pname_to_uid(&mut minor, user, 0 as gss_OID, &mut uid);
    check_gsserr(b"gss_pname_to_uid(user)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    if argc > 2 as libc::c_int &&
           strcmp(*argv.offset(2 as libc::c_int as isize),
                  b"-\x00" as *const u8 as *const libc::c_char) !=
               0 as libc::c_int {
        target = import_name(*argv.offset(2 as libc::c_int as isize))
    }
    mechs =
        if use_spnego != 0 { &mut mechset_spnego } else { &mut mechset_krb5 };
    get_default_cred(if argc > 3 as libc::c_int {
                         *argv.offset(3 as libc::c_int as isize)
                     } else { 0 as *mut libc::c_char }, mechs,
                     &mut impersonator_cred_handle);
    printf(b"Protocol transition tests follow\n\x00" as *const u8 as
               *const libc::c_char);
    printf(b"-----------------------------------\n\n\x00" as *const u8 as
               *const libc::c_char);
    /* get S4U2Self cred */
    major =
        gss_acquire_cred_impersonate_name(&mut minor,
                                          impersonator_cred_handle, user,
                                          0xffffffff as libc::c_ulong as
                                              OM_uint32, mechs,
                                          1 as libc::c_int,
                                          &mut user_cred_handle,
                                          &mut actual_mechs,
                                          0 as *mut OM_uint32);
    check_gsserr(b"gss_acquire_cred_impersonate_name\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    /* Try to store it in default ccache */
    major =
        gss_store_cred(&mut minor, user_cred_handle, 1 as libc::c_int,
                       &mut *(*mechs).elements.offset(0 as libc::c_int as
                                                          isize),
                       1 as libc::c_int as OM_uint32,
                       1 as libc::c_int as OM_uint32, 0 as *mut gss_OID_set,
                       0 as *mut gss_cred_usage_t);
    check_gsserr(b"gss_store_cred\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    init_accept_sec_context(user_cred_handle, impersonator_cred_handle,
                            &mut delegated_cred_handle);
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    gss_release_name(&mut minor, &mut user);
    gss_release_name(&mut minor, &mut target);
    gss_release_cred(&mut minor, &mut delegated_cred_handle);
    gss_release_cred(&mut minor, &mut impersonator_cred_handle);
    gss_release_cred(&mut minor, &mut user_cred_handle);
    gss_release_oid_set(&mut minor, &mut actual_mechs);
    return 0 as libc::c_int;
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
