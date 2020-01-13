use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:27"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:27"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:27"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:28"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:31"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:31"]
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
        /* name_equal */
        #[no_mangle]
        #[c2rust::src_loc = "554:1"]
        pub fn gss_display_name(_: *mut OM_uint32, _: gss_name_t,
                                _: gss_buffer_t, _: *mut gss_OID)
         -> OM_uint32;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:31"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
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
    use super::stdint_intn_h::int32_t;
    extern "C" {
        #[c2rust::src_loc = "350:8"]
        pub type _krb5_context;
        #[c2rust::src_loc = "2280:8"]
        pub type _krb5_ccache;
        #[no_mangle]
        #[c2rust::src_loc = "2368:1"]
        pub fn krb5_cc_initialize(context: krb5_context, cache: krb5_ccache,
                                  principal: krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2403:1"]
        pub fn krb5_cc_close(context: krb5_context, cache: krb5_ccache)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4341:1"]
        pub fn krb5_cc_resolve(context: krb5_context,
                               name: *const libc::c_char,
                               cache: *mut krb5_ccache) -> krb5_error_code;
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
        #[c2rust::src_loc = "3427:1"]
        pub fn krb5_parse_name(context: krb5_context,
                               name: *const libc::c_char,
                               principal_out: *mut krb5_principal)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/stdio.h:27"]
pub mod stdio_h {
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
#[c2rust::header_src = "/usr/include/string.h:29"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/tests/gssapi/common.h:31"]
pub mod common_h {
    use super::gssapi_h::{gss_cred_id_t, gss_OID_desc_struct, gss_OID,
                          gss_cred_id_struct, gss_name_struct, gss_name_t,
                          OM_uint32, gss_ctx_id_t, gss_OID_set_desc,
                          gss_OID_desc};
    use super::krb5_h::{_krb5_context, krb5_context, krb5_error_code};
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn export_import_cred(cred: *mut gss_cred_id_t);
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_krb5.h:31"]
pub mod gssapi_krb5_h {
    use super::gssapi_h::{OM_uint32, gss_cred_id_struct, gss_cred_id_t};
    use super::krb5_h::{_krb5_ccache, krb5_ccache};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "196:1"]
        pub fn gss_krb5_ccache_name(minor_status: *mut OM_uint32,
                                    name: *const libc::c_char,
                                    out_name: *mut *const libc::c_char)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "191:1"]
        pub fn gss_krb5_copy_ccache(minor_status: *mut OM_uint32,
                                    cred_handle: gss_cred_id_t,
                                    out_ccache: krb5_ccache) -> OM_uint32;
    }
    /* _GSSAPI_KRB5_H_ */
    /* __cplusplus */
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__int32_t, __uint32_t, __off_t, __off64_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssapi_h::{gss_name_t, gss_cred_id_t, gss_ctx_id_t, gss_uint32,
                         OM_uint32, gss_OID_desc_struct, gss_OID_desc,
                         gss_OID, gss_OID_set_desc_struct, gss_OID_set_desc,
                         gss_OID_set, gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_cred_usage_t, gss_name_struct,
                         gss_cred_id_struct, gss_ctx_id_struct,
                         gss_acquire_cred, gss_release_cred,
                         gss_delete_sec_context, gss_display_name,
                         gss_release_name, gss_release_buffer};
pub use self::krb5_h::{krb5_int32, krb5_boolean, krb5_error_code, krb5_magic,
                       _krb5_data, krb5_data, krb5_principal_data,
                       krb5_principal, krb5_context, krb5_ccache,
                       _krb5_context, _krb5_ccache, krb5_cc_initialize,
                       krb5_cc_close, krb5_cc_resolve, krb5_free_principal,
                       krb5_init_context, krb5_free_context, krb5_parse_name};
use self::stdio_h::{stderr, fprintf, printf};
use self::string_h::strcmp;
use self::common_h::{export_import_cred, establish_contexts, import_name,
                     check_k5err, check_gsserr, mechset_spnego, mechset_krb5,
                     mech_spnego, mech_krb5};
use self::gssapi_krb5_h::{gss_krb5_ccache_name, gss_krb5_copy_ccache};
/* -*- mode: c; indent-tabs-mode: nil -*- */
/* tests/gssapi/t_s4u2proxy_deleg.c - Test S4U2Proxy after krb5 auth */
/*
 * Copyright 2011 by the Massachusetts Institute of Technology.
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
 * Usage: ./t_s4u2proxy_krb5 [--spnego] client_cache storage_cache
 *                                      [accname|-] service1 service2
 *
 * This program performs a regular Kerberos or SPNEGO authentication from the
 * default principal of client_cache to service1.  If that authentication
 * yields delegated credentials, the program stores those credentials in
 * sorage_ccache and uses that cache to perform a second authentication to
 * service2 using S4U2Proxy.
 *
 * The default keytab must contain keys for service1 and service2.  The default
 * ccache must contain a TGT for service1.  This program assumes that krb5 or
 * SPNEGO authentication requires only one token exchange.
 */
#[c2rust::src_loc = "48:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut client_ccname: *const libc::c_char = 0 as *const libc::c_char;
    let mut storage_ccname: *const libc::c_char = 0 as *const libc::c_char;
    let mut accname: *const libc::c_char = 0 as *const libc::c_char;
    let mut service1: *const libc::c_char = 0 as *const libc::c_char;
    let mut service2: *const libc::c_char = 0 as *const libc::c_char;
    let mut context: krb5_context = 0 as krb5_context;
    let mut ret: krb5_error_code = 0;
    let mut use_spnego: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut storage_ccache: krb5_ccache = 0 as krb5_ccache;
    let mut client_princ: krb5_principal = 0 as krb5_principal;
    let mut minor: OM_uint32 = 0;
    let mut major: OM_uint32 = 0;
    let mut flags: OM_uint32 = 0;
    let mut buf: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut mech: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut mechs: gss_OID_set = 0 as *mut gss_OID_set_desc_struct;
    let mut acceptor_name: gss_name_t = 0 as gss_name_t;
    let mut client_name: gss_name_t = 0 as gss_name_t;
    let mut service1_name: gss_name_t = 0 as gss_name_t;
    let mut service2_name: gss_name_t = 0 as gss_name_t;
    let mut service1_cred: gss_cred_id_t = 0 as gss_cred_id_t;
    let mut deleg_cred: gss_cred_id_t = 0 as gss_cred_id_t;
    let mut initiator_context: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut acceptor_context: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    /* Parse arguments. */
    if argc >= 2 as libc::c_int &&
           strcmp(*argv.offset(1 as libc::c_int as isize),
                  b"--spnego\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        use_spnego = 1 as libc::c_int as krb5_boolean;
        argc -= 1;
        argv = argv.offset(1)
    }
    if argc != 6 as libc::c_int {
        fprintf(stderr,
                b"./t_s4u2proxy_krb5 [--spnego] client_ccache storage_ccache [accname|-] service1 service2\n\x00"
                    as *const u8 as *const libc::c_char);
        return 1 as libc::c_int
    }
    client_ccname = *argv.offset(1 as libc::c_int as isize);
    storage_ccname = *argv.offset(2 as libc::c_int as isize);
    accname = *argv.offset(3 as libc::c_int as isize);
    service1 = *argv.offset(4 as libc::c_int as isize);
    service2 = *argv.offset(5 as libc::c_int as isize);
    mech = if use_spnego != 0 { &mut mech_spnego } else { &mut mech_krb5 };
    mechs =
        if use_spnego != 0 { &mut mechset_spnego } else { &mut mechset_krb5 };
    ret = krb5_init_context(&mut context);
    check_k5err(context,
                b"krb5_init_context\x00" as *const u8 as *const libc::c_char,
                ret);
    /* Get GSS_C_BOTH acceptor credentials, using the default ccache. */
    acceptor_name = 0 as gss_name_t;
    if strcmp(accname, b"-\x00" as *const u8 as *const libc::c_char) !=
           0 as libc::c_int {
        acceptor_name = import_name(service1)
    }
    major =
        gss_acquire_cred(&mut minor, acceptor_name,
                         0xffffffff as libc::c_ulong as OM_uint32, mechs,
                         0 as libc::c_int, &mut service1_cred,
                         0 as *mut gss_OID_set, 0 as *mut OM_uint32);
    check_gsserr(b"gss_acquire_cred(service1)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    /* Establish contexts using the client ccache. */
    service1_name = import_name(service1);
    major =
        gss_krb5_ccache_name(&mut minor, client_ccname,
                             0 as *mut *const libc::c_char);
    check_gsserr(b"gss_krb5_ccache_name(1)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    flags = (4 as libc::c_int | 8 as libc::c_int) as OM_uint32;
    establish_contexts(mech, 0 as gss_cred_id_t, service1_cred, service1_name,
                       flags, &mut initiator_context, &mut acceptor_context,
                       &mut client_name, 0 as *mut gss_OID, &mut deleg_cred);
    /* Display and remember the client principal. */
    major =
        gss_display_name(&mut minor, client_name, &mut buf,
                         0 as *mut gss_OID);
    check_gsserr(b"gss_display_name(1)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    printf(b"auth1: %.*s\n\x00" as *const u8 as *const libc::c_char,
           buf.length as libc::c_int, buf.value as *mut libc::c_char);
    /* Assumes buffer is null-terminated, which in our implementation it is. */
    ret =
        krb5_parse_name(context, buf.value as *const libc::c_char,
                        &mut client_princ);
    check_k5err(context,
                b"krb5_parse_name\x00" as *const u8 as *const libc::c_char,
                ret);
    gss_release_buffer(&mut minor, &mut buf);
    if deleg_cred.is_null() {
        printf(b"no credential delegated.\n\x00" as *const u8 as
                   *const libc::c_char);
    } else {
        /* Take the opportunity to test cred export/import on the synthesized
     * S4U2Proxy delegated cred. */
        export_import_cred(&mut deleg_cred);
        /* Store the delegated credentials. */
        ret = krb5_cc_resolve(context, storage_ccname, &mut storage_ccache);
        check_k5err(context,
                    b"krb5_cc_resolve\x00" as *const u8 as
                        *const libc::c_char, ret);
        ret = krb5_cc_initialize(context, storage_ccache, client_princ);
        check_k5err(context,
                    b"krb5_cc_initialize\x00" as *const u8 as
                        *const libc::c_char, ret);
        major = gss_krb5_copy_ccache(&mut minor, deleg_cred, storage_ccache);
        check_gsserr(b"gss_krb5_copy_ccache\x00" as *const u8 as
                         *const libc::c_char, major, minor);
        ret = krb5_cc_close(context, storage_ccache);
        check_k5err(context,
                    b"krb5_cc_close\x00" as *const u8 as *const libc::c_char,
                    ret);
        gss_delete_sec_context(&mut minor, &mut initiator_context,
                               0 as gss_buffer_t);
        gss_delete_sec_context(&mut minor, &mut acceptor_context,
                               0 as gss_buffer_t);
        gss_release_name(&mut minor, &mut client_name);
        gss_release_cred(&mut minor, &mut deleg_cred);
        /* Establish contexts using the storage ccache. */
        service2_name = import_name(service2);
        major =
            gss_krb5_ccache_name(&mut minor, storage_ccname,
                                 0 as *mut *const libc::c_char);
        check_gsserr(b"gss_krb5_ccache_name(2)\x00" as *const u8 as
                         *const libc::c_char, major, minor);
        establish_contexts(mech, 0 as gss_cred_id_t, 0 as gss_cred_id_t,
                           service2_name, flags, &mut initiator_context,
                           &mut acceptor_context, &mut client_name,
                           0 as *mut gss_OID, &mut deleg_cred);
        major =
            gss_display_name(&mut minor, client_name, &mut buf,
                             0 as *mut gss_OID);
        check_gsserr(b"gss_display_name(2)\x00" as *const u8 as
                         *const libc::c_char, major, minor);
        printf(b"auth2: %.*s\n\x00" as *const u8 as *const libc::c_char,
               buf.length as libc::c_int, buf.value as *mut libc::c_char);
        gss_release_buffer(&mut minor, &mut buf);
    }
    gss_release_name(&mut minor, &mut acceptor_name);
    gss_release_name(&mut minor, &mut client_name);
    gss_release_name(&mut minor, &mut service1_name);
    gss_release_name(&mut minor, &mut service2_name);
    gss_release_cred(&mut minor, &mut service1_cred);
    gss_release_cred(&mut minor, &mut deleg_cred);
    gss_delete_sec_context(&mut minor, &mut initiator_context,
                           0 as gss_buffer_t);
    gss_delete_sec_context(&mut minor, &mut acceptor_context,
                           0 as gss_buffer_t);
    krb5_free_principal(context, client_princ);
    krb5_free_context(context);
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
