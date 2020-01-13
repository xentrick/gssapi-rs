use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:34"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:34"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:34"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    use super::types_h::{__uint8_t, __uint16_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:34"]
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
    /*
 * Per V5 spec on definition of principal types
 */
    /* *<  Name type not known */
    /* *< Just the name of the principal
                                      as in DCE, or for users */
    /* *< Service and other unique instance (krbtgt) */
    /* *< Service with host name as instance
                                      (telnet, rcommands) */
    /* *< Service with host as remaining components */
    /* *< Unique ID */
    /* *< PKINIT */
    /* *< Name in form of SMTP email name */
    /* *< Windows 2000 UPN */
    /* *< Well-known (special) principal */
    /* *< First component of
                                                NT_WELLKNOWN principals */
    /* *< Windows 2000 UPN and SID */
    /* *< NT 4 style name */
    /* *< NT 4 style name and SID */
    /* * Constant version of krb5_principal_data */
    #[c2rust::src_loc = "261:1"]
    pub type krb5_const_principal = *const krb5_principal_data;
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
    use super::stdint_uintn_h::{uint8_t, uint16_t};
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
        #[no_mangle]
        #[c2rust::src_loc = "3489:1"]
        pub fn krb5_unparse_name(context: krb5_context,
                                 principal: krb5_const_principal,
                                 name: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4767:1"]
        pub fn krb5_free_unparsed_name(context: krb5_context,
                                       val: *mut libc::c_char);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/plugin.h:35"]
pub mod plugin_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2010 by the Massachusetts Institute of Technology.
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
    /* Generic declarations for dynamic modules implementing krb5 plugin
 * modules. */
    /* krb5_plugin_vtable is an abstract type.  Module initvt functions will cast
 * it to the appropriate interface-specific vtable type. */
    #[c2rust::src_loc = "34:1"]
    pub type krb5_plugin_vtable = *mut krb5_plugin_vtable_st;
    extern "C" {
        #[c2rust::src_loc = "34:16"]
        pub type krb5_plugin_vtable_st;
    }
    /* KRB5_PLUGIN_H */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:35"]
pub mod kdb_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1990, 1991, 2016 by the Massachusetts Institute of Technology.
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
/*
 * Copyright 2009 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
    /* KDC Database interface definitions */
    /* This API is not considered as stable as the main krb5 API.
 *
 * - We may make arbitrary incompatible changes between feature
 *   releases (e.g. from 1.7 to 1.8).
 * - We will make some effort to avoid making incompatible changes for
 *   bugfix releases, but will make them if necessary.
 */
    /* This version will be incremented when incompatible changes are made to the
 * KDB API, and will be kept in sync with the libkdb major version. */
    /* Salt types */
    /* #define KRB5_KDB_SALTTYPE_V4            1 */
    /* #define KRB5_KDB_SALTTYPE_AFS3          5 */
    /* Attributes */
    /* S4U2Self OK */
    /* Creation flags */
    /* Entry get flags */
/* Name canonicalization requested */
    /* Include authorization data generated by backend */
    /* Is AS-REQ (client referrals only) */
    /* Map cross-realm principals */
    /* Protocol transition */
    /* Constrained delegation */
    /* User-to-user */
    /* Cross-realm */
    /* Issuing referral */
    /* KDB iteration flags */
    /* String attribute names recognized by krb5 */
    /*
 * Note --- these structures cannot be modified without changing the
 * database version number in libkdb.a, but should be expandable by
 * adding new tl_data types.
 */
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
    /*
 * If this ever changes up the version number and make the arrays be as
 * big as necessary.
 *
 * Currently the first type is the enctype and the second is the salt type.
 */
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
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    use super::krb5_h::{krb5_int16, krb5_ui_2, krb5_octet, krb5_enctype,
                        krb5_int32};
    /* Array of pointers */
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:35"]
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
    pub type kadm5_principal_ent_t = *mut _kadm5_principal_ent_t;
    use super::krb5_h::{krb5_principal, krb5_timestamp, krb5_deltat,
                        krb5_flags, krb5_kvno, krb5_int16};
    use super::kdb_h::{krb5_tl_data, krb5_key_data};
    /* __KADM5_ADMIN_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/kadm5_hook_plugin.h:35"]
pub mod kadm5_hook_plugin_h {
    #[c2rust::src_loc = "81:1"]
    pub type kadm5_hook_stage = libc::c_uint;
    #[c2rust::src_loc = "86:5"]
    pub const KADM5_HOOK_STAGE_POSTCOMMIT: kadm5_hook_stage = 1;
    #[c2rust::src_loc = "84:5"]
    pub const KADM5_HOOK_STAGE_PRECOMMIT: kadm5_hook_stage = 0;
    #[c2rust::src_loc = "90:1"]
    pub type kadm5_hook_modinfo = kadm5_hook_modinfo_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "96:16"]
    pub struct kadm5_hook_vtable_1_st {
        pub name: *const libc::c_char,
        pub init: Option<unsafe extern "C" fn(_: krb5_context,
                                              _: *mut *mut kadm5_hook_modinfo)
                             -> kadm5_ret_t>,
        pub fini: Option<unsafe extern "C" fn(_: krb5_context,
                                              _: *mut kadm5_hook_modinfo)
                             -> ()>,
        pub chpass: Option<unsafe extern "C" fn(_: krb5_context,
                                                _: *mut kadm5_hook_modinfo,
                                                _: libc::c_int,
                                                _: krb5_principal,
                                                _: krb5_boolean,
                                                _: libc::c_int,
                                                _: *mut krb5_key_salt_tuple,
                                                _: *const libc::c_char)
                               -> kadm5_ret_t>,
        pub create: Option<unsafe extern "C" fn(_: krb5_context,
                                                _: *mut kadm5_hook_modinfo,
                                                _: libc::c_int,
                                                _: kadm5_principal_ent_t,
                                                _: libc::c_long,
                                                _: libc::c_int,
                                                _: *mut krb5_key_salt_tuple,
                                                _: *const libc::c_char)
                               -> kadm5_ret_t>,
        pub modify: Option<unsafe extern "C" fn(_: krb5_context,
                                                _: *mut kadm5_hook_modinfo,
                                                _: libc::c_int,
                                                _: kadm5_principal_ent_t,
                                                _: libc::c_long)
                               -> kadm5_ret_t>,
        pub remove: Option<unsafe extern "C" fn(_: krb5_context,
                                                _: *mut kadm5_hook_modinfo,
                                                _: libc::c_int,
                                                _: krb5_principal)
                               -> kadm5_ret_t>,
        pub rename: Option<unsafe extern "C" fn(_: krb5_context,
                                                _: *mut kadm5_hook_modinfo,
                                                _: libc::c_int,
                                                _: krb5_principal,
                                                _: krb5_principal)
                               -> kadm5_ret_t>,
    }
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2010 by the Massachusetts Institute of Technology.
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
    /* *
 * @file krb5/krb5_kadm5_hook_plugin.h
 * Provide a plugin interface for kadm5 operations. This interface
 * permits a plugin to intercept principal modification, creation and
 * change password operations. Operations run at two stages: a
 * precommit stage that runs before the operation is committed to the
 * database and a postcommit operation that runs after the database
 * is updated; see #kadm5_hook_stage for details on semantics.
 *
 * This interface is based on a proposed extension to Heimdal by Russ
 * Allbery; it is likely that Heimdal will adopt an approach based on
 * stacked kdb modules rather than this interface. For MIT, writing a
 * plugin to this interface is significantly easier than stacking kdb
 * modules. Also, the kadm5 interface is significantly more stable
 * than the kdb interface, so this approach is more desirable than
 * stacked kdb modules.
 *
 * This interface depends on kadm5/admin.h. As such, the interface
 * does not provide strong guarantees of ABI stability.
 *
 * The kadm5_hook interface currently has only one supported major version,
 * which is 1.  Major version 1 has a current minor version number of 2.
 *
 * kadm5_hook plugins should:
 * kadm5_hook_<modulename>_initvt, matching the signature:
 *
 *   krb5_error_code
 *   kadm5_hook_modname_initvt(krb5_context context, int maj_ver, int min_ver,
 *                         krb5_plugin_vtable vtable);
 *
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for maj_ver:
 *     maj_ver == 1: Cast to kadm5_hook_vftable_1
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */
    /* *
 * Whether the operation is being run before or after the database
 * update.
 */
    /* * In this stage, any plugin failure prevents following plugins from
     *         running and aborts the operation.*/
    /* * In this stage, plugin failures are logged but otherwise ignored.*/
    /* * Opaque module data pointer. */
    /* *
 * Interface for the v1 virtual table for the kadm5_hook plugin.
 * All entry points are optional. The name field must be provided.
 */
    #[c2rust::src_loc = "96:1"]
    pub type kadm5_hook_vftable_1 = kadm5_hook_vtable_1_st;
    use super::admin_h::{kadm5_ret_t, kadm5_principal_ent_t};
    use super::krb5_h::{krb5_context, krb5_principal, krb5_boolean};
    use super::kdb_h::krb5_key_salt_tuple;
    extern "C" {
        #[c2rust::src_loc = "90:16"]
        pub type kadm5_hook_modinfo_st;
    }
    /* * A text string identifying the plugin for logging messages. */
    /* * Initialize a plugin module.
     * @param modinfo returns newly allocated module info for future
     * calls.  Cleaned up by the fini() function.
     */
    /* * Clean up a module and free @a modinfo. */
    /* * Indicates that the password is being changed.
     * @param stage is an integer from #kadm5_hook_stage enumeration
     * @param keepold is true if existing keys are being kept.
     * @param newpass is NULL if the key sare being randomized.
     */
    /* * Indicate a principal is created. */
    /* * Modify a principal. */
    /* * Indicate a principal is deleted. */
    /* End of minor version 1. */
    /* * Indicate a principal is renamed. */
    /* End of minor version 2. */
    /*H_KRB5_KADM5_HOOK_PLUGIN*/
}
#[c2rust::header_src = "/usr/include/stdio.h:35"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:37"]
pub mod assert_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn __assert_fail(__assertion: *const libc::c_char,
                             __file: *const libc::c_char,
                             __line: libc::c_uint,
                             __function: *const libc::c_char) -> !;
    }
}
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_boolean, krb5_kvno, krb5_enctype, krb5_flags,
                       krb5_timestamp, krb5_deltat, krb5_error_code,
                       krb5_magic, _krb5_data, krb5_data, krb5_principal_data,
                       krb5_principal, krb5_const_principal, krb5_context,
                       _krb5_context, krb5_unparse_name,
                       krb5_free_unparsed_name};
pub use self::plugin_h::{krb5_plugin_vtable, krb5_plugin_vtable_st};
pub use self::kdb_h::{_krb5_tl_data, krb5_tl_data, _krb5_key_data,
                      krb5_key_data, __krb5_key_salt_tuple,
                      krb5_key_salt_tuple};
pub use self::admin_h::{kadm5_ret_t, _kadm5_principal_ent_t,
                        kadm5_principal_ent_t};
pub use self::kadm5_hook_plugin_h::{kadm5_hook_stage,
                                    KADM5_HOOK_STAGE_POSTCOMMIT,
                                    KADM5_HOOK_STAGE_PRECOMMIT,
                                    kadm5_hook_modinfo,
                                    kadm5_hook_vtable_1_st,
                                    kadm5_hook_vftable_1,
                                    kadm5_hook_modinfo_st};
use self::stdio_h::printf;
use self::assert_h::__assert_fail;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* plugins/kadm5_hook/test/main.c */
/*
 * Copyright (C) 2010 by the Massachusetts Institute of Technology.
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
/* *
 * @file plugins/kadm5_hook/test/main.c
 *
 * This is a test kadm5_hook plugin. If enabled, it will print when kadm5_hook
 * calls are made.
 */
#[c2rust::src_loc = "39:1"]
unsafe extern "C" fn log_call(mut context: krb5_context,
                              mut function: *const libc::c_char,
                              mut stage: libc::c_int,
                              mut princ: krb5_principal) {
    let mut unparsed: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: krb5_error_code = 0;
    ret =
        krb5_unparse_name(context, princ as krb5_const_principal,
                          &mut unparsed);
    if ret == 0 as libc::c_int {
    } else {
        __assert_fail(b"ret == 0\x00" as *const u8 as *const libc::c_char,
                      b"main.c\x00" as *const u8 as *const libc::c_char,
                      48 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 63],
                                                &[libc::c_char; 63]>(b"void log_call(krb5_context, const char *, int, krb5_principal)\x00")).as_ptr());
    }
    printf(b"%s: stage %s principal %s\n\x00" as *const u8 as
               *const libc::c_char, function,
           if stage == KADM5_HOOK_STAGE_PRECOMMIT as libc::c_int {
               b"precommit\x00" as *const u8 as *const libc::c_char
           } else { b"postcommit\x00" as *const u8 as *const libc::c_char },
           unparsed);
    if !unparsed.is_null() { krb5_free_unparsed_name(context, unparsed); };
}
#[c2rust::src_loc = "57:1"]
unsafe extern "C" fn chpass(mut context: krb5_context,
                            mut modinfo: *mut kadm5_hook_modinfo,
                            mut stage: libc::c_int, mut princ: krb5_principal,
                            mut keepold: krb5_boolean,
                            mut n_ks_tuple: libc::c_int,
                            mut ks_tuple: *mut krb5_key_salt_tuple,
                            mut newpass: *const libc::c_char) -> kadm5_ret_t {
    log_call(context, b"chpass\x00" as *const u8 as *const libc::c_char,
             stage, princ);
    return 0 as libc::c_int as kadm5_ret_t;
}
#[c2rust::src_loc = "71:1"]
unsafe extern "C" fn create(mut context: krb5_context,
                            mut modinfo: *mut kadm5_hook_modinfo,
                            mut stage: libc::c_int,
                            mut princ: kadm5_principal_ent_t,
                            mut mask: libc::c_long,
                            mut n_ks_tuple: libc::c_int,
                            mut ks_tuple: *mut krb5_key_salt_tuple,
                            mut newpass: *const libc::c_char) -> kadm5_ret_t {
    log_call(context, b"create\x00" as *const u8 as *const libc::c_char,
             stage, (*princ).principal);
    return 0 as libc::c_int as kadm5_ret_t;
}
#[c2rust::src_loc = "84:1"]
unsafe extern "C" fn rename_hook(mut context: krb5_context,
                                 mut modinfo: *mut kadm5_hook_modinfo,
                                 mut stage: libc::c_int,
                                 mut oprinc: krb5_principal,
                                 mut nprinc: krb5_principal) -> kadm5_ret_t {
    log_call(context, b"rename\x00" as *const u8 as *const libc::c_char,
             stage, oprinc);
    return 0 as libc::c_int as kadm5_ret_t;
}
#[no_mangle]
#[c2rust::src_loc = "96:1"]
pub unsafe extern "C" fn kadm5_hook_test_initvt(mut context: krb5_context,
                                                mut maj_ver: libc::c_int,
                                                mut min_ver: libc::c_int,
                                                mut vtable:
                                                    krb5_plugin_vtable)
 -> krb5_error_code {
    let mut vt: *mut kadm5_hook_vftable_1 =
        vtable as *mut kadm5_hook_vftable_1;
    if maj_ver != 1 as libc::c_int {
        return -(1750600192 as libc::c_long) as krb5_error_code
    }
    (*vt).name = b"test\x00" as *const u8 as *const libc::c_char;
    (*vt).chpass =
        Some(chpass as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: *mut kadm5_hook_modinfo,
                                      _: libc::c_int, _: krb5_principal,
                                      _: krb5_boolean, _: libc::c_int,
                                      _: *mut krb5_key_salt_tuple,
                                      _: *const libc::c_char) -> kadm5_ret_t);
    (*vt).create =
        Some(create as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: *mut kadm5_hook_modinfo,
                                      _: libc::c_int,
                                      _: kadm5_principal_ent_t,
                                      _: libc::c_long, _: libc::c_int,
                                      _: *mut krb5_key_salt_tuple,
                                      _: *const libc::c_char) -> kadm5_ret_t);
    (*vt).rename =
        Some(rename_hook as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: *mut kadm5_hook_modinfo,
                                      _: libc::c_int, _: krb5_principal,
                                      _: krb5_principal) -> kadm5_ret_t);
    return 0 as libc::c_int;
}
