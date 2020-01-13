use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:40"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:40"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:40"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:40"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:40"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:40"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:41"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
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
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
    #[c2rust::src_loc = "2736:1"]
    pub type krb5_keytab = *mut _krb5_kt;
    use super::stdint_intn_h::int32_t;
    extern "C" {
        /* per Kerberos v5 protocol spec */
        /* not yet in the spec... */
        /* macros to determine if a type is a local type */
        /*
 * end "hostaddr.h"
 */
        #[c2rust::src_loc = "350:8"]
        pub type _krb5_context;
        #[c2rust::src_loc = "2280:8"]
        pub type _krb5_ccache;
        #[c2rust::src_loc = "2735:8"]
        pub type _krb5_kt;
        /* *
 * Close a credential cache handle.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * This function closes a credential cache handle @a cache without affecting
 * the contents of the cache.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2403:1"]
        pub fn krb5_cc_close(context: krb5_context, cache: krb5_ccache)
         -> krb5_error_code;
        /* *
 * Close a key table handle.
 *
 * @param [in] context          Library context
 * @param [in] keytab           Key table handle
 *
 * @retval 0
 */
        #[no_mangle]
        #[c2rust::src_loc = "2782:1"]
        pub fn krb5_kt_close(context: krb5_context, keytab: krb5_keytab)
         -> krb5_error_code;
        /*
 * end "keytab.h"
 */
        /*
 * begin "func-proto.h"
 */
        /* *< Use secure context configuration */
        /* *< Use KDC configuration if available */
        /* *
 * Create a krb5 library context.
 *
 * @param [out] context         Library context
 *
 * The @a context must be released by calling krb5_free_context() when
 * it is no longer needed.
 *
 * @warning Any program or module that needs the Kerberos code to not trust the
 * environment must use krb5_init_secure_context(), or clean out the
 * environment.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2922:1"]
        pub fn krb5_init_context(context: *mut krb5_context)
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
        pub fn krb5_free_context(context: krb5_context);
        /* *
 * Convert a string principal name to a krb5_principal structure.
 *
 * @param [in]  context         Library context
 * @param [in]  name            String representation of a principal name
 * @param [out] principal_out   New principal
 *
 * Convert a string representation of a principal name to a krb5_principal
 * structure.
 *
 * A string representation of a Kerberos name consists of one or more principal
 * name components, separated by slashes, optionally followed by the \@
 * character and a realm name.  If the realm name is not specified, the local
 * realm is used.
 *
 * To use the slash and \@ symbols as part of a component (quoted) instead of
 * using them as a component separator or as a realm prefix), put a backslash
 * (\) character in front of the symbol.  Similarly, newline, tab, backspace,
 * and NULL characters can be included in a component by using @c n, @c t, @c b
 * or @c 0, respectively.
 *
 * @note The realm in a Kerberos @a name cannot contain slash, colon,
 * or NULL characters.
 *
 * Use krb5_free_principal() to free @a principal_out when it is no longer
 * needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3427:1"]
        pub fn krb5_parse_name(context: krb5_context,
                               name: *const libc::c_char,
                               principal_out: *mut krb5_principal)
         -> krb5_error_code;
        /* *
 * Resolve the default key table.
 *
 * @param [in]  context         Library context
 * @param [out] id              Key table handle
 *
 * Set @a id to a handle to the default key table.  The key table is not
 * opened.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4226:1"]
        pub fn krb5_kt_default(context: krb5_context, id: *mut krb5_keytab)
         -> krb5_error_code;
        /* *
 * Resolve the default credential cache name.
 *
 * @param [in]  context         Library context
 * @param [out] ccache          Pointer to credential cache name
 *
 * Create a handle to the default credential cache as given by
 * krb5_cc_default_name().
 *
 * @retval
 * 0  Success
 * @retval
 * KV5M_CONTEXT            Bad magic number for @c _krb5_context structure
 * @retval
 * KRB5_FCC_INTERNAL       The name of the default credential cache cannot be
 *                         obtained
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4425:1"]
        pub fn krb5_cc_default(context: krb5_context,
                               ccache: *mut krb5_ccache) -> krb5_error_code;
        /* krb5_free.c */
/* *
 * Free the storage assigned to a principal.
 *
 * @param [in] context          Library context
 * @param [in] val              Principal to be freed
 */
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:43"]
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
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
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src = "/usr/include/stdio.h:40"]
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
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/tests/gssapi/common.h:43"]
pub mod common_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID, gss_cred_id_struct,
                          gss_cred_id_t, gss_name_struct, gss_name_t,
                          OM_uint32, gss_ctx_id_t, gss_OID_desc};
    use super::krb5_h::{_krb5_context, krb5_context, krb5_error_code};
    extern "C" {
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
        #[c2rust::src_loc = "38:21"]
        pub static mut mech_krb5: gss_OID_desc;
    }
    /* COMMON_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_krb5.h:43"]
pub mod gssapi_krb5_h {
    use super::gssapi_h::{OM_uint32, gss_cred_id_t};
    use super::krb5_h::{_krb5_ccache, krb5_ccache, krb5_principal_data,
                        krb5_principal, _krb5_kt, krb5_keytab};
    extern "C" {
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
pub use self::types_h::{__int32_t, __uint32_t, __off_t, __off64_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::krb5_h::{krb5_int32, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_principal_data, krb5_principal,
                       krb5_context, krb5_ccache, krb5_keytab, _krb5_context,
                       _krb5_ccache, _krb5_kt, krb5_cc_close, krb5_kt_close,
                       krb5_init_context, krb5_free_context, krb5_parse_name,
                       krb5_kt_default, krb5_cc_default, krb5_free_principal};
pub use self::gssapi_h::{gss_name_t, gss_cred_id_t, gss_ctx_id_t, gss_uint32,
                         OM_uint32, gss_OID_desc_struct, gss_OID_desc,
                         gss_OID, gss_buffer_desc_struct, gss_buffer_t,
                         gss_name_struct, gss_cred_id_struct,
                         gss_ctx_id_struct, gss_release_cred,
                         gss_delete_sec_context, gss_release_name};
use self::stdio_h::{stderr, fprintf};
use self::common_h::{establish_contexts, import_name, check_k5err,
                     check_gsserr, mech_krb5};
use self::gssapi_krb5_h::gss_krb5_import_cred;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/gssapi/t_imp_cred.c - krb5_gss_import_cred test harness */
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
 * Test program for krb5_gss_import_cred, intended to be run from a Python test
 * script.  Creates an initiator credential for the default ccache and an
 * acceptor principal for the default keytab (possibly using a specified keytab
 * principal), and performs a one-token context exchange using a specified
 * target principal.  If the exchange is successful, queries the context for
 * the acceptor name and prints it.  If any call is unsuccessful, displays an
 * error message.  Exits with status 0 if all operations are successful, or 1
 * if not.
 *
 * Usage: ./t_imp_cred target-princ [keytab-princ]
 */
#[c2rust::src_loc = "45:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut minor: OM_uint32 = 0;
    let mut major: OM_uint32 = 0;
    let mut flags: OM_uint32 = 0;
    let mut initiator_cred: gss_cred_id_t = 0 as *mut gss_cred_id_struct;
    let mut acceptor_cred: gss_cred_id_t = 0 as *mut gss_cred_id_struct;
    let mut initiator_context: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut acceptor_context: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut target_name: gss_name_t = 0 as *mut gss_name_struct;
    let mut context: krb5_context = 0 as krb5_context;
    let mut cc: krb5_ccache = 0 as *mut _krb5_ccache;
    let mut kt: krb5_keytab = 0 as *mut _krb5_kt;
    let mut princ: krb5_principal = 0 as krb5_principal;
    let mut ret: krb5_error_code = 0;
    if argc < 2 as libc::c_int || argc > 3 as libc::c_int {
        fprintf(stderr,
                b"Usage: %s targetname [acceptorprinc]\n\x00" as *const u8 as
                    *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize));
        return 1 as libc::c_int
    }
    /* Import the target name. */
    target_name = import_name(*argv.offset(1 as libc::c_int as isize));
    /* Acquire the krb5 objects we need. */
    ret = krb5_init_context(&mut context);
    check_k5err(context,
                b"krb5_init_context\x00" as *const u8 as *const libc::c_char,
                ret);
    ret = krb5_cc_default(context, &mut cc);
    check_k5err(context,
                b"krb5_cc_default\x00" as *const u8 as *const libc::c_char,
                ret);
    ret = krb5_kt_default(context, &mut kt);
    check_k5err(context,
                b"krb5_kt_default\x00" as *const u8 as *const libc::c_char,
                ret);
    if argc >= 3 as libc::c_int {
        ret =
            krb5_parse_name(context, *argv.offset(2 as libc::c_int as isize),
                            &mut princ);
        check_k5err(context,
                    b"krb5_parse_name\x00" as *const u8 as
                        *const libc::c_char, ret);
    }
    /* Get initiator cred. */
    major =
        gss_krb5_import_cred(&mut minor, cc, 0 as krb5_principal,
                             0 as krb5_keytab, &mut initiator_cred);
    check_gsserr(b"gss_krb5_import_cred (initiator)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    /* Get acceptor cred. */
    major =
        gss_krb5_import_cred(&mut minor, 0 as krb5_ccache, princ, kt,
                             &mut acceptor_cred);
    check_gsserr(b"gss_krb5_import_cred (acceptor)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    flags = (4 as libc::c_int | 8 as libc::c_int) as OM_uint32;
    establish_contexts(&mut mech_krb5, initiator_cred, acceptor_cred,
                       target_name, flags, &mut initiator_context,
                       &mut acceptor_context, 0 as *mut gss_name_t,
                       0 as *mut gss_OID, 0 as *mut gss_cred_id_t);
    krb5_cc_close(context, cc);
    krb5_kt_close(context, kt);
    krb5_free_principal(context, princ);
    krb5_free_context(context);
    gss_release_name(&mut minor, &mut target_name);
    gss_release_cred(&mut minor, &mut initiator_cred);
    gss_release_cred(&mut minor, &mut acceptor_cred);
    gss_delete_sec_context(&mut minor, &mut initiator_context,
                           0 as gss_buffer_t);
    gss_delete_sec_context(&mut minor, &mut acceptor_context,
                           0 as gss_buffer_t);
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