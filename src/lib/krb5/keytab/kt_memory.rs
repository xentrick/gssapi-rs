use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:27"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:27"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:27"]
pub mod thread_shared_types_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:16"]
    pub struct __pthread_internal_list {
        pub __prev: *mut __pthread_internal_list,
        pub __next: *mut __pthread_internal_list,
    }
    #[c2rust::src_loc = "82:1"]
    pub type __pthread_list_t = __pthread_internal_list;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:8"]
    pub struct __pthread_mutex_s {
        pub __lock: libc::c_int,
        pub __count: libc::c_uint,
        pub __owner: libc::c_int,
        pub __nusers: libc::c_uint,
        pub __kind: libc::c_int,
        pub __spins: libc::c_short,
        pub __elision: libc::c_short,
        pub __list: __pthread_list_t,
    }
}
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:27"]
pub mod pthreadtypes_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "67:9"]
    pub union pthread_mutex_t {
        pub __data: __pthread_mutex_s,
        pub __size: [libc::c_char; 40],
        pub __align: libc::c_long,
    }
    use super::thread_shared_types_h::__pthread_mutex_s;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:27"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:27"]
pub mod k5_thread_h {
    #[c2rust::src_loc = "283:1"]
    pub type k5_os_mutex = pthread_mutex_t;
    #[c2rust::src_loc = "354:1"]
    pub type k5_mutex_t = k5_os_mutex;
    #[inline]
    #[c2rust::src_loc = "379:1"]
    pub unsafe extern "C" fn k5_mutex_unlock(mut m: *mut k5_mutex_t) {
        let mut r: libc::c_int = k5_os_mutex_unlock(m);
        if r != 0 as libc::c_int {
            fprintf(stderr,
                    b"k5_mutex_unlock: Received error %d (%s)\n\x00" as
                        *const u8 as *const libc::c_char, r, strerror(r));
        }
        if r == 0 as libc::c_int {
        } else {
            __assert_fail(b"r == 0\x00" as *const u8 as *const libc::c_char,
                          b"../../../include/k5-thread.h\x00" as *const u8 as
                              *const libc::c_char,
                          388 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 35],
                                                    &[libc::c_char; 35]>(b"void k5_mutex_unlock(k5_mutex_t *)\x00")).as_ptr());
        };
    }
    #[inline]
    #[c2rust::src_loc = "367:1"]
    pub unsafe extern "C" fn k5_mutex_lock(mut m: *mut k5_mutex_t) {
        let mut r: libc::c_int = k5_os_mutex_lock(m);
        if r != 0 as libc::c_int {
            fprintf(stderr,
                    b"k5_mutex_lock: Received error %d (%s)\n\x00" as
                        *const u8 as *const libc::c_char, r, strerror(r));
        }
        if r == 0 as libc::c_int {
        } else {
            __assert_fail(b"r == 0\x00" as *const u8 as *const libc::c_char,
                          b"../../../include/k5-thread.h\x00" as *const u8 as
                              *const libc::c_char,
                          376 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 33],
                                                    &[libc::c_char; 33]>(b"void k5_mutex_lock(k5_mutex_t *)\x00")).as_ptr());
        };
    }
    #[inline]
    #[c2rust::src_loc = "360:1"]
    pub unsafe extern "C" fn k5_mutex_finish_init(mut m: *mut k5_mutex_t)
     -> libc::c_int {
        return 0 as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "356:1"]
    pub unsafe extern "C" fn k5_mutex_init(mut m: *mut k5_mutex_t)
     -> libc::c_int {
        return k5_os_mutex_init(m);
    }
    use super::pthreadtypes_h::pthread_mutex_t;
    use super::stdio_h::{fprintf, stderr};
    use super::string_h::strerror;
    use super::assert_h::__assert_fail;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "290:1"]
        pub fn k5_os_mutex_init(m: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "291:1"]
        pub fn k5_os_mutex_destroy(m: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "293:1"]
        pub fn k5_os_mutex_unlock(m: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "292:1"]
        pub fn k5_os_mutex_lock(m: *mut k5_os_mutex) -> libc::c_int;
    }
    /* multiple inclusion? */
    /* In time, many of the definitions above should move into the support
   library, and this file should be greatly simplified.  For type
   definitions, that'll take some work, since other data structures
   incorporate mutexes directly, and our mutex type is dependent on
   configuration options and system attributes.  For most functions,
   though, it should be relatively easy.

   For now, plugins should use the exported functions, and not the
   above macros, and use krb5int_mutex_alloc for allocations.  */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:27"]
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
    /* Originally used to recognize AFS and default salts.  No longer used. */
    #[c2rust::src_loc = "225:1"]
    pub type krb5_pointer = *mut libc::c_void;
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
    /*
 * end "rcache.h"
 */
    /*
 * begin "keytab.h"
 */
    /* XXX */
    /* *< Long enough for MAXPATHLEN + some extra */
    #[c2rust::src_loc = "2724:1"]
    pub type krb5_kt_cursor = krb5_pointer;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2727:16"]
    pub struct krb5_keytab_entry_st {
        pub magic: krb5_magic,
        pub principal: krb5_principal,
        pub timestamp: krb5_timestamp,
        pub vno: krb5_kvno,
        pub key: krb5_keyblock,
    }
    /* * A key table entry. */
    #[c2rust::src_loc = "2727:1"]
    pub type krb5_keytab_entry = krb5_keytab_entry_st;
    #[c2rust::src_loc = "2736:1"]
    pub type krb5_keytab = *mut _krb5_kt;
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::{_krb5_context, _krb5_kt};
    extern "C" {
        /* *< Principal of this key */
        /* *< Time entry written to keytable */
        /* *< Key version number */
        /* *< The secret key */
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
 * Compare two encryption types.
 *
 * @param [in]  context         Library context
 * @param [in]  e1              First encryption type
 * @param [in]  e2              Second encryption type
 * @param [out] similar         @c TRUE if types are similar, @c FALSE if not
 *
 * This function determines whether two encryption types use the same kind of
 * keys.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "882:1"]
        pub fn krb5_c_enctype_compare(context: krb5_context, e1: krb5_enctype,
                                      e2: krb5_enctype,
                                      similar: *mut krb5_boolean)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3664:1"]
        pub fn krb5_principal_compare(context: krb5_context,
                                      princ1: krb5_const_principal,
                                      princ2: krb5_const_principal)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "3766:1"]
        pub fn krb5_copy_keyblock_contents(context: krb5_context,
                                           from: *const krb5_keyblock,
                                           to: *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3813:1"]
        pub fn krb5_copy_principal(context: krb5_context,
                                   inprinc: krb5_const_principal,
                                   outprinc: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4261:1"]
        pub fn krb5_kt_free_entry(context: krb5_context,
                                  entry: *mut krb5_keytab_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4721:1"]
        pub fn krb5_free_keyblock_contents(context: krb5_context,
                                           key: *mut krb5_keyblock);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:27"]
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2076:8"]
    pub struct _krb5_kt {
        pub magic: krb5_magic,
        pub ops: *const _krb5_kt_ops,
        pub data: krb5_pointer,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2040:16"]
    pub struct _krb5_kt_ops {
        pub magic: krb5_magic,
        pub prefix: *mut libc::c_char,
        pub resolve: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: *const libc::c_char,
                                                 _: *mut krb5_keytab)
                                -> krb5_error_code>,
        pub get_name: Option<unsafe extern "C" fn(_: krb5_context,
                                                  _: krb5_keytab,
                                                  _: *mut libc::c_char,
                                                  _: libc::c_uint)
                                 -> krb5_error_code>,
        pub close: Option<unsafe extern "C" fn(_: krb5_context,
                                               _: krb5_keytab)
                              -> krb5_error_code>,
        pub get: Option<unsafe extern "C" fn(_: krb5_context, _: krb5_keytab,
                                             _: krb5_const_principal,
                                             _: krb5_kvno, _: krb5_enctype,
                                             _: *mut krb5_keytab_entry)
                            -> krb5_error_code>,
        pub start_seq_get: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: krb5_keytab,
                                                       _: *mut krb5_kt_cursor)
                                      -> krb5_error_code>,
        pub get_next: Option<unsafe extern "C" fn(_: krb5_context,
                                                  _: krb5_keytab,
                                                  _: *mut krb5_keytab_entry,
                                                  _: *mut krb5_kt_cursor)
                                 -> krb5_error_code>,
        pub end_get: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: krb5_keytab,
                                                 _: *mut krb5_kt_cursor)
                                -> krb5_error_code>,
        pub add: Option<unsafe extern "C" fn(_: krb5_context, _: krb5_keytab,
                                             _: *mut krb5_keytab_entry)
                            -> krb5_error_code>,
        pub remove: Option<unsafe extern "C" fn(_: krb5_context,
                                                _: krb5_keytab,
                                                _: *mut krb5_keytab_entry)
                               -> krb5_error_code>,
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_pointer, krb5_error_code, krb5_context,
                        krb5_keytab, krb5_const_principal, krb5_kvno,
                        krb5_keytab_entry, krb5_kt_cursor};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    extern "C" {
        #[c2rust::src_loc = "1134:8"]
        pub type plugin_mapping;
        #[c2rust::src_loc = "1202:8"]
        pub type _kdb_log_context;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:27"]
pub mod k5_err_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-err.h */
/*
 * Copyright 2006, 2007 Massachusetts Institute of Technology.
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
 *
 * Error-message handling
 */
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:27"]
pub mod k5_plugin_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2006 Massachusetts Institute of Technology.
 * All Rights Reserved.
 *
 * This software is being provided to you, the LICENSEE, by the
 * Massachusetts Institute of Technology (M.I.T.) under the following
 * license.  By obtaining, using and/or copying this software, you agree
 * that you have read, understood, and will comply with these terms and
 * conditions:
 *
 * Export of this software from the United States of America may
 * require a specific license from the United States Government.
 * It is the responsibility of any person or organization contemplating
 * export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify and distribute
 * this software and its documentation for any purpose and without fee or
 * royalty is hereby granted, provided that you agree to comply with the
 * following copyright notice and statements, including the disclaimer, and
 * that the same appear on ALL copies of the software and documentation,
 * including modifications that you make for internal use or for
 * distribution:
 *
 * THIS SOFTWARE IS PROVIDED "AS IS", AND M.I.T. MAKES NO REPRESENTATIONS
 * OR WARRANTIES, EXPRESS OR IMPLIED.  By way of example, but not
 * limitation, M.I.T. MAKES NO REPRESENTATIONS OR WARRANTIES OF
 * MERCHANTABILITY OR FITNESS FOR ANY PARTICULAR PURPOSE OR THAT THE USE OF
 * THE LICENSED SOFTWARE OR DOCUMENTATION WILL NOT INFRINGE ANY THIRD PARTY
 * PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
 *
 * The name of the Massachusetts Institute of Technology or M.I.T. may NOT
 * be used in advertising or publicity pertaining to distribution of the
 * software.  Title to copyright in this software and any associated
 * documentation shall at all times remain with M.I.T., and USER agrees to
 * preserve same.
 *
 * Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 */
    /* Just those definitions which are needed by util/support/plugins.c,
   which gets compiled before util/et is built, which happens before
   we can construct krb5.h, which is included by k5-int.h.

   So, no krb5 types.  */
    /*
 * Plugins normally export fixed symbol names, but when statically
 * linking plugins, we need a different symbol name for each plugin.
 * The first argument to PLUGIN_SYMBOL_NAME acts as the
 * differentiator, and is only used for static plugin linking.
 *
 * Although this macro (and thus this header file) are used in plugins
 * whose code lies inside the krb5 tree, plugins maintained separately
 * from the krb5 tree do not need it; they can just use the fixed
 * symbol name unconditionally.
 */
    /* opaque */
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:27"]
pub mod profile_h {
    /*
 * profile.h
 */
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
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
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "396:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
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
#[c2rust::header_src = "/usr/include/assert.h:27"]
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
pub use self::types_h::{__uint8_t, __int32_t, __off_t, __off64_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::pthread_mutex_t;
pub use self::stdint_uintn_h::uint8_t;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::k5_thread_h::{k5_os_mutex, k5_mutex_t, k5_mutex_unlock,
                            k5_mutex_lock, k5_mutex_finish_init,
                            k5_mutex_init, k5_os_mutex_init,
                            k5_os_mutex_destroy, k5_os_mutex_unlock,
                            k5_os_mutex_lock};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_kvno,
                       krb5_enctype, krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_pointer, krb5_principal_data, krb5_principal,
                       krb5_const_principal, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _krb5_keyblock,
                       krb5_keyblock, krb5_kt_cursor, krb5_keytab_entry_st,
                       krb5_keytab_entry, krb5_keytab, _profile_t,
                       krb5_c_enctype_compare, krb5_principal_compare,
                       krb5_copy_keyblock_contents, krb5_copy_principal,
                       krb5_kt_free_entry, krb5_free_keyblock_contents};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, _krb5_kt, _krb5_kt_ops,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
use self::stdlib_h::{malloc, calloc, free};
use self::stdio_h::{stderr, fprintf, snprintf};
use self::string_h::{strerror, strdup, strcmp, memset};
use self::assert_h::__assert_fail;
/* Per-keytab data header */
#[c2rust::src_loc = "65:1"]
pub type krb5_mkt_data = _krb5_mkt_data;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "65:16"]
pub struct _krb5_mkt_data {
    pub name: *mut libc::c_char,
    pub lock: k5_mutex_t,
    pub refcount: krb5_int32,
    pub link: krb5_mkt_cursor,
}
/* Name of the keytab */
/* Thread-safety - all but link */
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/keytab/kt_memory.c */
/*
 * Copyright 2007 by Secure Endpoints Inc.
 *
 * Permission is hereby granted, free of charge, to any person
 * obtaining a copy of this software and associated documentation files
 * (the "Software"), to deal in the Software without restriction,
 * including without limitation the rights to use, copy, modify, merge,
 * publish, distribute, sublicense, and/or sell copies of the Software,
 * and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
/*
 * Information needed by internal routines of the file-based ticket
 * cache implementation.
 */
/*
 * Constants
 */
/*
 * Types
 */
/* From krb5.h:
 * typedef struct krb5_keytab_entry_st {
 *    krb5_magic magic;
 *    krb5_principal principal;    principal of this key
 *    krb5_timestamp timestamp;    time entry written to keytable
 *    krb5_kvno vno;               key version number
 *    krb5_keyblock key;           the secret key
 *} krb5_keytab_entry;
 */
/* Individual key entries within a table, in a linked list */
#[c2rust::src_loc = "59:1"]
pub type krb5_mkt_cursor = *mut _krb5_mkt_link;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "59:16"]
pub struct _krb5_mkt_link {
    pub next: *mut _krb5_mkt_link,
    pub entry: *mut krb5_keytab_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "73:16"]
pub struct _krb5_mkt_list_node {
    pub next: *mut _krb5_mkt_list_node,
    pub keytab: krb5_keytab,
}
/* List of memory key tables */
#[c2rust::src_loc = "73:1"]
pub type krb5_mkt_list_node = _krb5_mkt_list_node;
#[c2rust::src_loc = "59:1"]
pub type krb5_mkt_link = _krb5_mkt_link;
/*
 * Globals
 */
#[c2rust::src_loc = "86:29"]
static mut krb5int_mkt_list: *mut krb5_mkt_list_node =
    0 as *const krb5_mkt_list_node as *mut krb5_mkt_list_node;
#[c2rust::src_loc = "87:19"]
static mut krb5int_mkt_mutex: k5_mutex_t =
    pthread_mutex_t{__data:
                        {
                            let mut init =
                                __pthread_mutex_s{__lock: 0 as libc::c_int,
                                                  __count:
                                                      0 as libc::c_int as
                                                          libc::c_uint,
                                                  __owner: 0 as libc::c_int,
                                                  __nusers:
                                                      0 as libc::c_int as
                                                          libc::c_uint,
                                                  __kind: 0 as libc::c_int,
                                                  __spins:
                                                      0 as libc::c_int as
                                                          libc::c_short,
                                                  __elision:
                                                      0 as libc::c_int as
                                                          libc::c_short,
                                                  __list:
                                                      {
                                                          let mut init =
                                                              __pthread_internal_list{__prev:
                                                                                          0
                                                                                              as
                                                                                              *const __pthread_internal_list
                                                                                              as
                                                                                              *mut __pthread_internal_list,
                                                                                      __next:
                                                                                          0
                                                                                              as
                                                                                              *const __pthread_internal_list
                                                                                              as
                                                                                              *mut __pthread_internal_list,};
                                                          init
                                                      },};
                            init
                        },};
#[no_mangle]
#[c2rust::src_loc = "136:1"]
pub unsafe extern "C" fn krb5int_mkt_initialize() -> libc::c_int {
    return k5_mutex_finish_init(&mut krb5int_mkt_mutex);
}
#[no_mangle]
#[c2rust::src_loc = "142:1"]
pub unsafe extern "C" fn krb5int_mkt_finalize() {
    let mut node: *mut krb5_mkt_list_node = 0 as *mut krb5_mkt_list_node;
    let mut next_node: *mut krb5_mkt_list_node = 0 as *mut krb5_mkt_list_node;
    let mut cursor: krb5_mkt_cursor = 0 as *mut _krb5_mkt_link;
    let mut next_cursor: krb5_mkt_cursor = 0 as *mut _krb5_mkt_link;
    k5_os_mutex_destroy(&mut krb5int_mkt_mutex);
    node = krb5int_mkt_list;
    while !node.is_null() {
        next_node = (*node).next;
        /* destroy the contents of node->keytab */
        free((*((*(*node).keytab).data as *mut krb5_mkt_data)).name as
                 *mut libc::c_void);
        /* free the keytab entries */
        cursor = (*((*(*node).keytab).data as *mut krb5_mkt_data)).link;
        while !cursor.is_null() {
            next_cursor = (*cursor).next;
            /* the call to krb5_kt_free_entry uses a NULL in place of the
             * krb5_context since we know that the context isn't used by
             * krb5_kt_free_entry or krb5_free_principal. */
            krb5_kt_free_entry(0 as krb5_context, (*cursor).entry);
            free((*cursor).entry as *mut libc::c_void);
            free(cursor as *mut libc::c_void);
            cursor = next_cursor
        }
        /* destroy the lock */
        k5_os_mutex_destroy(&mut (*((*(*node).keytab).data as
                                        *mut krb5_mkt_data)).lock);
        /* free the private data */
        free((*(*node).keytab).data);
        /* and the keytab */
        free((*node).keytab as *mut libc::c_void);
        /* and finally the node */
        free(node as *mut libc::c_void);
        node = next_node
    };
}
#[c2rust::src_loc = "181:1"]
unsafe extern "C" fn create_list_node(mut name: *const libc::c_char,
                                      mut listp: *mut *mut krb5_mkt_list_node)
 -> krb5_error_code {
    let mut list: *mut krb5_mkt_list_node = 0 as *mut krb5_mkt_list_node;
    let mut data: *mut krb5_mkt_data = 0 as *mut krb5_mkt_data;
    let mut err: krb5_error_code = 0;
    *listp = 0 as *mut krb5_mkt_list_node;
    list =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<krb5_mkt_list_node>() as libc::c_ulong)
            as *mut krb5_mkt_list_node;
    if list.is_null() {
        err = 12 as libc::c_int
    } else {
        (*list).keytab =
            calloc(1 as libc::c_int as libc::c_ulong,
                   ::std::mem::size_of::<_krb5_kt>() as libc::c_ulong) as
                krb5_keytab;
        if (*list).keytab.is_null() {
            err = 12 as libc::c_int
        } else {
            (*(*list).keytab).ops = &krb5_mkt_ops;
            data =
                calloc(1 as libc::c_int as libc::c_ulong,
                       ::std::mem::size_of::<krb5_mkt_data>() as
                           libc::c_ulong) as *mut krb5_mkt_data;
            if data.is_null() {
                err = 12 as libc::c_int
            } else {
                (*data).link = 0 as krb5_mkt_cursor;
                (*data).refcount = 0 as libc::c_int;
                (*data).name = strdup(name);
                if (*data).name.is_null() {
                    err = 12 as libc::c_int
                } else {
                    err = k5_mutex_init(&mut (*data).lock);
                    if !(err != 0) {
                        (*(*list).keytab).data = data as krb5_pointer;
                        (*(*list).keytab).magic =
                            -(1760647382 as libc::c_long) as krb5_magic;
                        (*list).next = 0 as *mut _krb5_mkt_list_node;
                        *listp = list;
                        return 0 as libc::c_int
                    }
                }
            }
        }
    }
    /* data->lock was initialized last, so no need to destroy. */
    if !data.is_null() { free((*data).name as *mut libc::c_void); }
    free(data as *mut libc::c_void);
    if !list.is_null() { free((*list).keytab as *mut libc::c_void); }
    free(list as *mut libc::c_void);
    return err;
}
/*
 * Macros
 */
/*
 * This is an implementation specific resolver.  It returns a keytab
 * initialized with memory keytab routines.
 */
#[no_mangle]
#[c2rust::src_loc = "243:1"]
pub unsafe extern "C" fn krb5_mkt_resolve(mut context: krb5_context,
                                          mut name: *const libc::c_char,
                                          mut id: *mut krb5_keytab)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut list: *mut krb5_mkt_list_node = 0 as *mut krb5_mkt_list_node;
    let mut err: krb5_error_code = 0 as libc::c_int;
    *id = 0 as krb5_keytab;
    /* First determine if a memory keytab of this name already exists */
    k5_mutex_lock(&mut krb5int_mkt_mutex);
    list = krb5int_mkt_list;
    while !list.is_null() {
        if strcmp(name,
                  (*((*(*list).keytab).data as *mut krb5_mkt_data)).name) ==
               0 as libc::c_int {
            break ;
        }
        list = (*list).next
    }
    if list.is_null() {
        /* We will now create the new key table with the specified name.
         * We do not drop the global lock, therefore the name will indeed
         * be unique when we add it.
         */
        err = create_list_node(name, &mut list);
        if err != 0 {
            current_block = 16641473711560579579;
        } else {
            (*list).next = krb5int_mkt_list;
            krb5int_mkt_list = list;
            current_block = 7651349459974463963;
        }
    } else { current_block = 7651349459974463963; }
    match current_block {
        7651349459974463963 => {
            /* Increment the reference count on the keytab we found or created. */
            k5_mutex_lock(&mut (*((*(*list).keytab).data as
                                      *mut krb5_mkt_data)).lock);
            let ref mut fresh0 =
                (*((*(*list).keytab).data as *mut krb5_mkt_data)).refcount;
            *fresh0 += 1;
            k5_mutex_unlock(&mut (*((*(*list).keytab).data as
                                        *mut krb5_mkt_data)).lock);
            *id = (*list).keytab
        }
        _ => { }
    }
    k5_mutex_unlock(&mut krb5int_mkt_mutex);
    return err;
}
/*
 * "Close" a memory-based keytab.  This is effectively a no-op.
 * We check to see if the keytab exists and that is about it.
 * Closing a file keytab does not destroy the contents.  Closing
 * a memory keytab shouldn't either.
 */
#[no_mangle]
#[c2rust::src_loc = "289:1"]
pub unsafe extern "C" fn krb5_mkt_close(mut context: krb5_context,
                                        mut id: krb5_keytab)
 -> krb5_error_code {
    let mut listp: *mut *mut krb5_mkt_list_node =
        0 as *mut *mut krb5_mkt_list_node;
    let mut node: *mut krb5_mkt_list_node = 0 as *mut krb5_mkt_list_node;
    let mut data: *mut krb5_mkt_data = 0 as *mut krb5_mkt_data;
    let mut err: krb5_error_code = 0 as libc::c_int;
    /* First determine if a memory keytab of this name already exists */
    k5_mutex_lock(&mut krb5int_mkt_mutex);
    listp = &mut krb5int_mkt_list;
    while !(*listp).is_null() {
        if id == (**listp).keytab { break ; }
        listp = &mut (**listp).next
    }
    if (*listp).is_null() {
        /* The specified keytab could not be found */
        err = -(1765328203 as libc::c_long) as krb5_error_code
    } else {
        /* reduce the refcount and return */
        k5_mutex_lock(&mut (*((*id).data as *mut krb5_mkt_data)).lock);
        let ref mut fresh1 = (*((*id).data as *mut krb5_mkt_data)).refcount;
        *fresh1 -= 1;
        k5_mutex_unlock(&mut (*((*id).data as *mut krb5_mkt_data)).lock);
        /* In Heimdal if the refcount hits 0, the MEMORY keytab is
     * destroyed since there is no krb5_kt_destroy function.
     * There is no need to lock the entry while performing
     * these operations as the refcount will be 0 and we are
     * holding the global lock.
     */
        data = (*id).data as *mut krb5_mkt_data;
        if (*data).refcount == 0 as libc::c_int {
            let mut cursor: krb5_mkt_cursor = 0 as *mut _krb5_mkt_link;
            let mut next_cursor: krb5_mkt_cursor = 0 as *mut _krb5_mkt_link;
            node = *listp;
            *listp = (*node).next;
            /* destroy the contents of node->keytab (aka id) */
            free((*data).name as *mut libc::c_void);
            /* free the keytab entries */
            cursor = (*((*(*node).keytab).data as *mut krb5_mkt_data)).link;
            while !cursor.is_null() {
                next_cursor = (*cursor).next;
                krb5_kt_free_entry(context, (*cursor).entry);
                free((*cursor).entry as *mut libc::c_void);
                free(cursor as *mut libc::c_void);
                cursor = next_cursor
            }
            /* destroy the lock */
            k5_os_mutex_destroy(&mut (*data).lock);
            /* free the private data */
            free(data as *mut libc::c_void);
            /* and the keytab */
            free((*node).keytab as *mut libc::c_void);
            /* and finally the node */
            free(node as *mut libc::c_void);
        }
    }
    /* HEIMDAL_COMPATIBLE */
    k5_mutex_unlock(&mut krb5int_mkt_mutex);
    return err;
}
/*
 * This is the get_entry routine for the memory based keytab implementation.
 * It either retrieves the entry or returns an error.
 */
#[no_mangle]
#[c2rust::src_loc = "371:1"]
pub unsafe extern "C" fn krb5_mkt_get_entry(mut context: krb5_context,
                                            mut id: krb5_keytab,
                                            mut principal:
                                                krb5_const_principal,
                                            mut kvno: krb5_kvno,
                                            mut enctype: krb5_enctype,
                                            mut out_entry:
                                                *mut krb5_keytab_entry)
 -> krb5_error_code {
    let mut cursor: krb5_mkt_cursor = 0 as *mut _krb5_mkt_link;
    let mut entry: *mut krb5_keytab_entry = 0 as *mut krb5_keytab_entry;
    let mut match_0: *mut krb5_keytab_entry = 0 as *mut krb5_keytab_entry;
    let mut err: krb5_error_code = 0 as libc::c_int;
    let mut found_wrong_kvno: libc::c_int = 0 as libc::c_int;
    let mut similar: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    k5_mutex_lock(&mut (*((*id).data as *mut krb5_mkt_data)).lock);
    let mut current_block_10: u64;
    cursor = (*((*id).data as *mut krb5_mkt_data)).link;
    while !cursor.is_null() && !(*cursor).entry.is_null() {
        entry = (*cursor).entry;
        /* if the principal isn't the one requested, continue to the next. */
        if !(krb5_principal_compare(context, principal,
                                    (*entry).principal as
                                        krb5_const_principal) == 0) {
            /* if the enctype is not ignored and doesn't match,
           and continue to the next */
            if enctype != 0 as libc::c_int {
                err =
                    krb5_c_enctype_compare(context, enctype,
                                           (*entry).key.enctype,
                                           &mut similar);
                if err != 0 {
                    current_block_10 = 735147466149431745;
                } else if similar == 0 {
                    current_block_10 = 735147466149431745;
                } else { current_block_10 = 5399440093318478209; }
            } else { current_block_10 = 5399440093318478209; }
            match current_block_10 {
                735147466149431745 => { }
                _ => {
                    if kvno == 0 as libc::c_int as libc::c_uint ||
                           (*entry).vno == 0 as libc::c_int as libc::c_uint {
                        if match_0.is_null() {
                            match_0 = entry
                        } else if (*entry).vno > (*match_0).vno {
                            match_0 = entry
                        }
                    } else if (*entry).vno == kvno {
                        match_0 = entry;
                        break ;
                    } else { found_wrong_kvno += 1 }
                }
            }
        }
        /* we can't determine the enctype of the entry */
        cursor = (*cursor).next
    }
    /* if we found an entry that matches, ... */
    if !match_0.is_null() {
        (*out_entry).magic = (*match_0).magic;
        (*out_entry).timestamp = (*match_0).timestamp;
        (*out_entry).vno = (*match_0).vno;
        (*out_entry).key = (*match_0).key;
        err =
            krb5_copy_keyblock_contents(context, &mut (*match_0).key,
                                        &mut (*out_entry).key);
        /*
         * Coerce the enctype of the output keyblock in case we
         * got an inexact match on the enctype.
         */
        if enctype != 0 as libc::c_int { (*out_entry).key.enctype = enctype }
        if err == 0 {
            err =
                krb5_copy_principal(context,
                                    (*match_0).principal as
                                        krb5_const_principal,
                                    &mut (*out_entry).principal)
        }
    } else if err == 0 {
        err =
            if found_wrong_kvno != 0 {
                -(1765328154 as libc::c_long)
            } else { -(1765328203 as libc::c_long) } as krb5_error_code
    }
    k5_mutex_unlock(&mut (*((*id).data as *mut krb5_mkt_data)).lock);
    return err;
}
/*
 * Get the name of the memory-based keytab.
 */
#[no_mangle]
#[c2rust::src_loc = "453:1"]
pub unsafe extern "C" fn krb5_mkt_get_name(mut context: krb5_context,
                                           mut id: krb5_keytab,
                                           mut name: *mut libc::c_char,
                                           mut len: libc::c_uint)
 -> krb5_error_code {
    let mut result: libc::c_int = 0;
    memset(name as *mut libc::c_void, 0 as libc::c_int, len as libc::c_ulong);
    result =
        snprintf(name, len as libc::c_ulong,
                 b"%s:%s\x00" as *const u8 as *const libc::c_char,
                 (*(*id).ops).prefix,
                 (*((*id).data as *mut krb5_mkt_data)).name);
    if result as libc::c_uint as libc::c_ulong >= len as size_t {
        return -(1765328155 as libc::c_long) as krb5_error_code
    }
    return 0 as libc::c_int;
}
/*
 * krb5_mkt_start_seq_get()
 */
#[no_mangle]
#[c2rust::src_loc = "469:1"]
pub unsafe extern "C" fn krb5_mkt_start_seq_get(mut context: krb5_context,
                                                mut id: krb5_keytab,
                                                mut cursorp:
                                                    *mut krb5_kt_cursor)
 -> krb5_error_code {
    k5_mutex_lock(&mut (*((*id).data as *mut krb5_mkt_data)).lock);
    *cursorp = (*((*id).data as *mut krb5_mkt_data)).link as krb5_kt_cursor;
    k5_mutex_unlock(&mut (*((*id).data as *mut krb5_mkt_data)).lock);
    return 0 as libc::c_int;
}
/*
 * krb5_mkt_get_next()
 */
#[no_mangle]
#[c2rust::src_loc = "483:1"]
pub unsafe extern "C" fn krb5_mkt_get_next(mut context: krb5_context,
                                           mut id: krb5_keytab,
                                           mut entry: *mut krb5_keytab_entry,
                                           mut cursor: *mut krb5_kt_cursor)
 -> krb5_error_code {
    let mut mkt_cursor: krb5_mkt_cursor = *cursor as krb5_mkt_cursor;
    let mut err: krb5_error_code = 0 as libc::c_int;
    k5_mutex_lock(&mut (*((*id).data as *mut krb5_mkt_data)).lock);
    if mkt_cursor.is_null() {
        k5_mutex_unlock(&mut (*((*id).data as *mut krb5_mkt_data)).lock);
        return -(1765328202 as libc::c_long) as krb5_error_code
    }
    (*entry).magic = (*(*mkt_cursor).entry).magic;
    (*entry).timestamp = (*(*mkt_cursor).entry).timestamp;
    (*entry).vno = (*(*mkt_cursor).entry).vno;
    (*entry).key = (*(*mkt_cursor).entry).key;
    err =
        krb5_copy_keyblock_contents(context, &mut (*(*mkt_cursor).entry).key,
                                    &mut (*entry).key);
    if err == 0 {
        err =
            krb5_copy_principal(context,
                                (*(*mkt_cursor).entry).principal as
                                    krb5_const_principal,
                                &mut (*entry).principal)
    }
    if err == 0 {
        *cursor = (*mkt_cursor).next as *mut krb5_kt_cursor as krb5_kt_cursor
    }
    k5_mutex_unlock(&mut (*((*id).data as *mut krb5_mkt_data)).lock);
    return err;
}
/*
 * krb5_mkt_end_get()
 */
#[no_mangle]
#[c2rust::src_loc = "515:1"]
pub unsafe extern "C" fn krb5_mkt_end_get(mut context: krb5_context,
                                          mut id: krb5_keytab,
                                          mut cursor: *mut krb5_kt_cursor)
 -> krb5_error_code {
    *cursor = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
/* routines to be included on extended version (write routines) */
/*
 * krb5_mkt_add()
 */
#[no_mangle]
#[c2rust::src_loc = "527:1"]
pub unsafe extern "C" fn krb5_mkt_add(mut context: krb5_context,
                                      mut id: krb5_keytab,
                                      mut entry: *mut krb5_keytab_entry)
 -> krb5_error_code {
    let mut err: krb5_error_code = 0 as libc::c_int;
    let mut cursor: krb5_mkt_cursor = 0 as *mut _krb5_mkt_link;
    k5_mutex_lock(&mut (*((*id).data as *mut krb5_mkt_data)).lock);
    cursor =
        malloc(::std::mem::size_of::<krb5_mkt_link>() as libc::c_ulong) as
            krb5_mkt_cursor;
    if cursor.is_null() {
        err = 12 as libc::c_int
    } else {
        (*cursor).entry =
            malloc(::std::mem::size_of::<krb5_keytab_entry>() as
                       libc::c_ulong) as *mut krb5_keytab_entry;
        if (*cursor).entry.is_null() {
            free(cursor as *mut libc::c_void);
            err = 12 as libc::c_int
        } else {
            (*(*cursor).entry).magic = (*entry).magic;
            (*(*cursor).entry).timestamp = (*entry).timestamp;
            (*(*cursor).entry).vno = (*entry).vno;
            err =
                krb5_copy_keyblock_contents(context, &mut (*entry).key,
                                            &mut (*(*cursor).entry).key);
            if err != 0 {
                free((*cursor).entry as *mut libc::c_void);
                free(cursor as *mut libc::c_void);
            } else {
                err =
                    krb5_copy_principal(context,
                                        (*entry).principal as
                                            krb5_const_principal,
                                        &mut (*(*cursor).entry).principal);
                if err != 0 {
                    krb5_free_keyblock_contents(context,
                                                &mut (*(*cursor).entry).key);
                    free((*cursor).entry as *mut libc::c_void);
                    free(cursor as *mut libc::c_void);
                } else if (*((*id).data as *mut krb5_mkt_data)).link.is_null()
                 {
                    (*cursor).next = 0 as *mut _krb5_mkt_link;
                    let ref mut fresh2 =
                        (*((*id).data as *mut krb5_mkt_data)).link;
                    *fresh2 = cursor
                } else {
                    (*cursor).next =
                        (*((*id).data as *mut krb5_mkt_data)).link;
                    let ref mut fresh3 =
                        (*((*id).data as *mut krb5_mkt_data)).link;
                    *fresh3 = cursor
                }
            }
        }
    }
    k5_mutex_unlock(&mut (*((*id).data as *mut krb5_mkt_data)).lock);
    return err;
}
/*
 * krb5_mkt_remove()
 */
#[no_mangle]
#[c2rust::src_loc = "582:1"]
pub unsafe extern "C" fn krb5_mkt_remove(mut context: krb5_context,
                                         mut id: krb5_keytab,
                                         mut entry: *mut krb5_keytab_entry)
 -> krb5_error_code {
    let mut pcursor: *mut krb5_mkt_cursor = 0 as *mut krb5_mkt_cursor;
    let mut next: krb5_mkt_cursor = 0 as *mut _krb5_mkt_link;
    let mut err: krb5_error_code = 0 as libc::c_int;
    k5_mutex_lock(&mut (*((*id).data as *mut krb5_mkt_data)).lock);
    if (*((*id).data as *mut krb5_mkt_data)).link.is_null() {
        err = -(1765328203 as libc::c_long) as krb5_error_code
    } else {
        pcursor = &mut (*((*id).data as *mut krb5_mkt_data)).link;
        while !(*pcursor).is_null() {
            if (*(**pcursor).entry).vno == (*entry).vno &&
                   (*(**pcursor).entry).key.enctype == (*entry).key.enctype &&
                   krb5_principal_compare(context,
                                          (*(**pcursor).entry).principal as
                                              krb5_const_principal,
                                          (*entry).principal as
                                              krb5_const_principal) != 0 {
                break ;
            }
            pcursor = &mut (**pcursor).next
        }
        if (*pcursor).is_null() {
            err = -(1765328203 as libc::c_long) as krb5_error_code
        } else {
            krb5_kt_free_entry(context, (**pcursor).entry);
            free((**pcursor).entry as *mut libc::c_void);
            next = (**pcursor).next;
            free(*pcursor as *mut libc::c_void);
            *pcursor = next
        }
    }
    k5_mutex_unlock(&mut (*((*id).data as *mut krb5_mkt_data)).lock);
    return err;
}
/*
 * krb5_mkt_ops
 */
#[no_mangle]
#[c2rust::src_loc = "623:27"]
pub static mut krb5_mkt_ops: _krb5_kt_ops =
    unsafe {
        {
            let mut init =
                _krb5_kt_ops{magic: 0 as libc::c_int,
                             prefix:
                                 b"MEMORY\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             resolve:
                                 Some(krb5_mkt_resolve as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *const libc::c_char,
                                                               _:
                                                                   *mut krb5_keytab)
                                              -> krb5_error_code),
                             get_name:
                                 Some(krb5_mkt_get_name as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   *mut libc::c_char,
                                                               _:
                                                                   libc::c_uint)
                                              -> krb5_error_code),
                             close:
                                 Some(krb5_mkt_close as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab)
                                              -> krb5_error_code),
                             get:
                                 Some(krb5_mkt_get_entry as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   krb5_const_principal,
                                                               _: krb5_kvno,
                                                               _:
                                                                   krb5_enctype,
                                                               _:
                                                                   *mut krb5_keytab_entry)
                                              -> krb5_error_code),
                             start_seq_get:
                                 Some(krb5_mkt_start_seq_get as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   *mut krb5_kt_cursor)
                                              -> krb5_error_code),
                             get_next:
                                 Some(krb5_mkt_get_next as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   *mut krb5_keytab_entry,
                                                               _:
                                                                   *mut krb5_kt_cursor)
                                              -> krb5_error_code),
                             end_get:
                                 Some(krb5_mkt_end_get as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   *mut krb5_kt_cursor)
                                              -> krb5_error_code),
                             add:
                                 Some(krb5_mkt_add as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   *mut krb5_keytab_entry)
                                              -> krb5_error_code),
                             remove:
                                 Some(krb5_mkt_remove as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   *mut krb5_keytab_entry)
                                              -> krb5_error_code),};
            init
        }
    };
/* LEAN_CLIENT */
