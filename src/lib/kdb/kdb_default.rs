use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:31"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "145:1"]
    pub type __dev_t = libc::c_ulong;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "147:1"]
    pub type __gid_t = libc::c_uint;
    #[c2rust::src_loc = "148:1"]
    pub type __ino_t = libc::c_ulong;
    #[c2rust::src_loc = "150:1"]
    pub type __mode_t = libc::c_uint;
    #[c2rust::src_loc = "151:1"]
    pub type __nlink_t = libc::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "174:1"]
    pub type __blksize_t = libc::c_long;
    #[c2rust::src_loc = "179:1"]
    pub type __blkcnt_t = libc::c_long;
    #[c2rust::src_loc = "196:1"]
    pub type __syscall_slong_t = libc::c_long;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:31"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:31"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:31"]
pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "9:8"]
    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }
    use super::types_h::{__time_t, __syscall_slong_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:31"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:31"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:31"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/stat.h:31"]
pub mod stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
    pub struct stat {
        pub st_dev: __dev_t,
        pub st_ino: __ino_t,
        pub st_nlink: __nlink_t,
        pub st_mode: __mode_t,
        pub st_uid: __uid_t,
        pub st_gid: __gid_t,
        pub __pad0: libc::c_int,
        pub st_rdev: __dev_t,
        pub st_size: __off_t,
        pub st_blksize: __blksize_t,
        pub st_blocks: __blkcnt_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        pub __glibc_reserved: [__syscall_slong_t; 3],
    }
    use super::types_h::{__dev_t, __ino_t, __nlink_t, __mode_t, __uid_t,
                         __gid_t, __off_t, __blksize_t, __blkcnt_t,
                         __syscall_slong_t};
    use super::struct_timespec_h::timespec;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:31"]
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
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
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
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
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
        /* *
 * Get an entry from a key table.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [in]  principal       Principal name
 * @param [in]  vno             Key version number (0 for highest available)
 * @param [in]  enctype         Encryption type (0 zero for any enctype)
 * @param [out] entry           Returned entry from key table
 *
 * Retrieve an entry from a key table which matches the @a keytab, @a
 * principal, @a vno, and @a enctype.  If @a vno is zero, retrieve the
 * highest-numbered kvno matching the other fields.  If @a enctype is 0, match
 * any enctype.
 *
 * Use krb5_free_keytab_entry_contents() to free @a entry when it is no longer
 * needed.
 *
 * @note If @a vno is zero, the function retrieves the highest-numbered-kvno
 * entry that matches the specified principal.
 *
 * @retval
 * 0 Success
 * @retval
 * Kerberos error codes on failure
 */
        #[no_mangle]
        #[c2rust::src_loc = "2811:1"]
        pub fn krb5_kt_get_entry(context: krb5_context, keytab: krb5_keytab,
                                 principal: krb5_const_principal,
                                 vno: krb5_kvno, enctype: krb5_enctype,
                                 entry: *mut krb5_keytab_entry)
         -> krb5_error_code;
        /* libkt.spec */
        /* *
 * Get a handle for a key table.
 *
 * @param [in]  context         Library context
 * @param [in]  name            Name of the key table
 * @param [out] ktid            Key table handle
 *
 * Resolve the key table name @a name and set @a ktid to a handle identifying
 * the key table.  Use krb5_kt_close() to free @a ktid when it is no longer
 * needed.
 *
 * @a name must be of the form @c type:residual, where @a type must be a type
 * known to the library and @a residual portion should be specific to the
 * particular keytab type.  If no @a type is given, the default is @c FILE.
 *
 * If @a name is of type @c FILE, the keytab file is not opened by this call.
 *
 * @code
 *  Example: krb5_kt_resolve(context, "FILE:/tmp/filename", &ktid);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4173:1"]
        pub fn krb5_kt_resolve(context: krb5_context,
                               name: *const libc::c_char,
                               ktid: *mut krb5_keytab) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4261:1"]
        pub fn krb5_kt_free_entry(context: krb5_context,
                                  entry: *mut krb5_keytab_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4301:1"]
        pub fn krb5_kt_add_entry(context: krb5_context, id: krb5_keytab,
                                 entry: *mut krb5_keytab_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "7892:1"]
        pub fn krb5_set_error_message(ctx: krb5_context,
                                      code: krb5_error_code,
                                      fmt: *const libc::c_char, _: ...);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:31"]
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
    #[inline]
    #[c2rust::src_loc = "2296:1"]
    pub unsafe extern "C" fn k5calloc(mut nmemb: size_t, mut size: size_t,
                                      mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
        ptr =
            calloc(if nmemb != 0 {
                       nmemb
                   } else { 1 as libc::c_int as libc::c_ulong },
                   if size != 0 {
                       size
                   } else { 1 as libc::c_int as libc::c_ulong });
        *code =
            if ptr.is_null() { 12 as libc::c_int } else { 0 as libc::c_int };
        return ptr;
    }
    #[inline]
    #[c2rust::src_loc = "2308:1"]
    pub unsafe extern "C" fn k5alloc(mut size: size_t,
                                     mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        return k5calloc(1 as libc::c_int as size_t, size, code);
    }
    #[inline]
    #[c2rust::src_loc = "2315:1"]
    pub unsafe extern "C" fn k5memdup(mut in_0: *const libc::c_void,
                                      mut len: size_t,
                                      mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void = k5alloc(len, code);
        if !ptr.is_null() && len > 0 as libc::c_int as libc::c_ulong {
            memcpy(ptr, in_0, len);
        }
        return ptr;
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
    use super::stddef_h::size_t;
    use super::stdlib_h::calloc;
    use super::string_h::memcpy;
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
        #[no_mangle]
        #[c2rust::src_loc = "2093:1"]
        pub fn krb5_is_permitted_enctype(_: krb5_context, _: krb5_enctype)
         -> krb5_boolean;
    }
    /* _KRB5_INT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:31"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:31"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:31"]
pub mod profile_h {
    /*
 * profile.h
 */
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:31"]
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
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:32"]
pub mod kdb_h {
    #[c2rust::src_loc = "294:1"]
    pub type krb5_keylist_node = _krb5_keylist_node;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "294:16"]
    pub struct _krb5_keylist_node {
        pub keyblock: krb5_keyblock,
        pub kvno: krb5_kvno,
        pub next: *mut _krb5_keylist_node,
    }
    #[c2rust::src_loc = "288:1"]
    pub type krb5_mkey_aux_node = _krb5_mkey_aux_node;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "288:16"]
    pub struct _krb5_mkey_aux_node {
        pub next: *mut _krb5_mkey_aux_node,
        pub mkey_kvno: krb5_kvno,
        pub latest_mkey: krb5_key_data,
    }
    /* kvno of mkey protecting the latest_mkey */
    /* most recent mkey */
    /*
 * If this ever changes up the version number and make the arrays be as
 * big as necessary.
 *
 * Currently the first type is the enctype and the second is the salt type.
 */
    #[c2rust::src_loc = "167:1"]
    pub type krb5_key_data = _krb5_key_data;
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
    /* Version */
    /* Key Version */
    /* Array of types */
    /* Array of lengths */
    /* Array of pointers */
    /*
 * A principal database entry.  Extensions to this structure currently use the
 * tl_data list.  The e_data and e_length fields are not used by any calling
 * code except kdb5_util dump and load, which marshal and unmarshal the array
 * in the dump record.  KDB modules may use these fields internally as long as
 * they set e_length appropriately (non-zero if the data should be marshalled
 * across dump and load, zero if not) and handle null e_data values in
 * caller-constructed principal entries.
 */
    #[c2rust::src_loc = "191:1"]
    pub type krb5_db_entry = _krb5_db_entry_new;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "191:16"]
    pub struct _krb5_db_entry_new {
        pub magic: krb5_magic,
        pub len: krb5_ui_2,
        pub mask: krb5_ui_4,
        pub attributes: krb5_flags,
        pub max_life: krb5_deltat,
        pub max_renewable_life: krb5_deltat,
        pub expiration: krb5_timestamp,
        pub pw_expiration: krb5_timestamp,
        pub last_success: krb5_timestamp,
        pub last_failed: krb5_timestamp,
        pub fail_auth_count: krb5_kvno,
        pub n_tl_data: krb5_int16,
        pub n_key_data: krb5_int16,
        pub e_length: krb5_ui_2,
        pub e_data: *mut krb5_octet,
        pub princ: krb5_principal,
        pub tl_data: *mut krb5_tl_data,
        pub key_data: *mut krb5_key_data,
    }
    /* NOT saved */
    /* members currently changed/set */
    /* When the client expires */
    /* When its passwd expires */
    /* Last successful passwd */
    /* Last failed passwd attempt */
    /* # of failed passwd attempt */
    /* Length of extra data */
    /* Extra data to be saved */
    /* Length, data */
    /* Linked list */
    /* key_data must be sorted by kvno in descending order. */
    /* Array */
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
    #[c2rust::src_loc = "147:1"]
    pub type krb5_tl_data = _krb5_tl_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "147:16"]
    pub struct _krb5_tl_data {
        pub tl_data_next: *mut _krb5_tl_data,
        pub tl_data_type: krb5_int16,
        pub tl_data_length: krb5_ui_2,
        pub tl_data_contents: *mut krb5_octet,
    }
    /* NOT saved */
    /* # of array elements */
    #[c2rust::src_loc = "177:1"]
    pub type krb5_keysalt = _krb5_keysalt;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "177:16"]
    pub struct _krb5_keysalt {
        pub type_0: krb5_int16,
        pub data: krb5_data,
    }
    use super::krb5_h::{krb5_keyblock, krb5_kvno, krb5_int16, krb5_ui_2,
                        krb5_octet, krb5_magic, krb5_ui_4, krb5_flags,
                        krb5_deltat, krb5_timestamp, krb5_principal,
                        krb5_data, krb5_context, krb5_principal_data,
                        krb5_const_principal, krb5_error_code};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "370:1"]
        pub fn krb5_db_get_principal(kcontext: krb5_context,
                                     search_for: krb5_const_principal,
                                     flags: libc::c_uint,
                                     entry: *mut *mut krb5_db_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "374:1"]
        pub fn krb5_db_free_principal(kcontext: krb5_context,
                                      entry: *mut krb5_db_entry);
        #[no_mangle]
        #[c2rust::src_loc = "375:1"]
        pub fn krb5_db_put_principal(kcontext: krb5_context,
                                     entry: *mut krb5_db_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "377:1"]
        pub fn krb5_db_delete_principal(kcontext: krb5_context,
                                        search_for: krb5_principal)
         -> krb5_error_code;
        /* Length, data */
        /* *
 * Decrypts the key given in @@a key_data. If @a mkey is specified, that
 * master key is used. If @a mkey is NULL, then all master keys are tried.
 */
        #[no_mangle]
        #[c2rust::src_loc = "446:1"]
        pub fn krb5_dbe_decrypt_key_data(context: krb5_context,
                                         mkey: *const krb5_keyblock,
                                         key_data: *const krb5_key_data,
                                         dbkey: *mut krb5_keyblock,
                                         keysalt: *mut krb5_keysalt)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "498:1"]
        pub fn krb5_dbe_lookup_mkey_aux(context: krb5_context,
                                        entry: *mut krb5_db_entry,
                                        mkey_aux_data_list:
                                            *mut *mut krb5_mkey_aux_node)
         -> krb5_error_code;
        /*
 * Modify the key data of entry to explicitly store salt values using the
 * KRB5_KDB_SALTTYPE_SPECIAL salt type.
 */
        #[no_mangle]
        #[c2rust::src_loc = "617:1"]
        pub fn krb5_dbe_specialize_salt(context: krb5_context,
                                        entry: *mut krb5_db_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "851:1"]
        pub fn krb5_dbe_free_key_list(_: krb5_context,
                                      _: *mut krb5_keylist_node);
        #[no_mangle]
        #[c2rust::src_loc = "857:1"]
        pub fn krb5_dbe_free_mkey_aux_list(_: krb5_context,
                                           _: *mut krb5_mkey_aux_node);
    }
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src = "/usr/include/stdlib.h:31"]
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
#[c2rust::header_src = "/usr/include/stdio.h:31"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "148:1"]
        pub fn rename(__old: *const libc::c_char, __new: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "246:14"]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "646:15"]
        pub fn fread(_: *mut libc::c_void, _: libc::c_ulong, _: libc::c_ulong,
                     _: *mut FILE) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "786:1"]
        pub fn fileno(__stream: *mut FILE) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:31"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "175:1"]
        pub fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:31"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:31"]
pub mod unistd_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "825:1"]
        pub fn unlink(__name: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:31"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/string.h:31"]
pub mod string_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "435:1"]
        pub fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
        #[no_mangle]
        #[c2rust::src_loc = "124:14"]
        pub fn strncpy(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/sys/stat.h:31"]
pub mod sys_stat_h {
    use super::stat_h::stat;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "205:1"]
        pub fn stat(__file: *const libc::c_char, __buf: *mut stat)
         -> libc::c_int;
    }
}
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t, __dev_t, __uid_t, __gid_t, __ino_t,
                        __mode_t, __nlink_t, __off_t, __off64_t, __time_t,
                        __blksize_t, __blkcnt_t, __syscall_slong_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::struct_timespec_h::timespec;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stat_h::stat;
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_kvno, krb5_enctype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_pointer, krb5_principal_data, krb5_principal,
                       krb5_const_principal, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _krb5_keyblock,
                       krb5_keyblock, krb5_kt_cursor, krb5_keytab_entry_st,
                       krb5_keytab_entry, krb5_keytab, _profile_t,
                       krb5_kt_close, krb5_kt_get_entry, krb5_kt_resolve,
                       krb5_kt_free_entry, krb5_kt_add_entry,
                       krb5_set_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, _krb5_kt, _krb5_kt_ops, k5calloc,
                         k5alloc, k5memdup, plugin_mapping, _kdb_log_context,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle,
                         krb5_is_permitted_enctype};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::com_err_h::{errcode_t, error_message};
pub use self::kdb_h::{krb5_keylist_node, _krb5_keylist_node,
                      krb5_mkey_aux_node, _krb5_mkey_aux_node, krb5_key_data,
                      _krb5_key_data, krb5_db_entry, _krb5_db_entry_new,
                      krb5_tl_data, _krb5_tl_data, krb5_keysalt,
                      _krb5_keysalt, krb5_db_get_principal,
                      krb5_db_free_principal, krb5_db_put_principal,
                      krb5_db_delete_principal, krb5_dbe_decrypt_key_data,
                      krb5_dbe_lookup_mkey_aux, krb5_dbe_specialize_salt,
                      krb5_dbe_free_key_list, krb5_dbe_free_mkey_aux_list};
use self::stdlib_h::{malloc, calloc, free};
use self::stdio_h::{rename, fclose, fopen, snprintf, asprintf, fread, fileno};
use self::fcntl_h::fcntl;
use self::errno_h::__errno_location;
use self::unistd_h::unlink;
use self::libintl_h::dgettext;
use self::string_h::{explicit_bzero, strncpy, memset, memcpy};
use self::sys_stat_h::stat;
/* default functions. Should not be directly called */
/*
 *   Default functions prototype
 */
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/kdb/kdb_default.c */
/*
 * Copyright 1995, 2009 by the Massachusetts Institute of Technology.
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
 * Copyright 2009 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
/*
 * Set *kd_out to the key data entry matching kvno, enctype, and salttype.  If
 * any of those three parameters are -1, ignore them.  If kvno is 0, match only
 * the highest kvno.  Begin searching at the index *start and set *start to the
 * index after the match.  Do not return keys of non-permitted enctypes; return
 * KRB5_KDB_NO_PERMITTED_KEY if the whole list was searched and only
 * non-permitted matches were found.
 */
#[no_mangle]
#[c2rust::src_loc = "47:1"]
pub unsafe extern "C" fn krb5_dbe_def_search_enctype(mut context:
                                                         krb5_context,
                                                     mut ent:
                                                         *mut krb5_db_entry,
                                                     mut start:
                                                         *mut krb5_int32,
                                                     mut enctype: krb5_int32,
                                                     mut salttype: krb5_int32,
                                                     mut kvno: krb5_int32,
                                                     mut kd_out:
                                                         *mut *mut krb5_key_data)
 -> krb5_error_code {
    let mut kd: *mut krb5_key_data = 0 as *mut krb5_key_data;
    let mut db_salttype: krb5_int32 = 0;
    let mut saw_non_permitted: krb5_boolean =
        0 as libc::c_int as krb5_boolean;
    let mut i: libc::c_int = 0;
    *kd_out = 0 as *mut krb5_key_data;
    if enctype != -(1 as libc::c_int) &&
           krb5_is_permitted_enctype(context, enctype) == 0 {
        return -(1780008418 as libc::c_long) as krb5_error_code
    }
    if (*ent).n_key_data as libc::c_int == 0 as libc::c_int {
        return -(1780008417 as libc::c_long) as krb5_error_code
    }
    /* Match the highest kvno if kvno is 0.  Key data is sorted in descending
     * order of kvno. */
    if kvno == 0 as libc::c_int {
        kvno =
            (*(*ent).key_data.offset(0 as libc::c_int as isize)).key_data_kvno
                as krb5_int32
    }
    i = *start;
    while i < (*ent).n_key_data as libc::c_int {
        kd = &mut *(*ent).key_data.offset(i as isize) as *mut krb5_key_data;
        db_salttype =
            if (*kd).key_data_ver as libc::c_int > 1 as libc::c_int {
                (*kd).key_data_type[1 as libc::c_int as usize] as libc::c_int
            } else { 0 as libc::c_int };
        /* Match this entry against the arguments.  Stop searching if we have
         * passed the entries for the requested kvno. */
        if !(enctype != -(1 as libc::c_int) &&
                 (*kd).key_data_type[0 as libc::c_int as usize] as libc::c_int
                     != enctype) {
            if !(salttype >= 0 as libc::c_int && db_salttype != salttype) {
                if kvno >= 0 as libc::c_int &&
                       ((*kd).key_data_kvno as libc::c_int) < kvno {
                    break ;
                }
                if !(kvno >= 0 as libc::c_int &&
                         (*kd).key_data_kvno as libc::c_int != kvno) {
                    /* Filter out non-permitted enctypes. */
                    if krb5_is_permitted_enctype(context,
                                                 (*kd).key_data_type[0 as
                                                                         libc::c_int
                                                                         as
                                                                         usize]
                                                     as krb5_enctype) == 0 {
                        saw_non_permitted = 1 as libc::c_int as krb5_boolean
                    } else {
                        *start = i + 1 as libc::c_int;
                        *kd_out = kd;
                        return 0 as libc::c_int
                    }
                }
            }
        }
        i += 1
    }
    /* If we scanned the whole set of keys and matched only non-permitted
     * enctypes, indicate that. */
    return if *start == 0 as libc::c_int && saw_non_permitted != 0 {
               -(1780008418 as libc::c_long)
           } else { -(1780008417 as libc::c_long) } as krb5_error_code;
}
/*
 *  kdb default functions. Ideally, some other file should have this functions. For now, TBD.
 */
#[no_mangle]
#[c2rust::src_loc = "110:1"]
pub unsafe extern "C" fn krb5_def_store_mkey_list(mut context: krb5_context,
                                                  mut keyfile:
                                                      *mut libc::c_char,
                                                  mut mname: krb5_principal,
                                                  mut keylist:
                                                      *mut krb5_keylist_node,
                                                  mut master_pwd:
                                                      *mut libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut defkeyfile: [libc::c_char; 4097] = [0; 4097];
    let mut tmp_ktname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp_ktpath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut realm: *mut krb5_data = &mut (*mname).realm;
    let mut kt: krb5_keytab = 0 as krb5_keytab;
    let mut new_entry: krb5_keytab_entry =
        krb5_keytab_entry{magic: 0,
                          principal: 0 as *mut krb5_principal_data,
                          timestamp: 0,
                          vno: 0,
                          key:
                              krb5_keyblock{magic: 0,
                                            enctype: 0,
                                            length: 0,
                                            contents:
                                                0 as *mut krb5_octet,},};
    let mut stb: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    let mut statrc: libc::c_int = 0;
    if keyfile.is_null() {
        snprintf(defkeyfile.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 4097]>() as
                     libc::c_ulong,
                 b"%s%s\x00" as *const u8 as *const libc::c_char,
                 b"/usr/local/var/krb5kdc/.k5.\x00" as *const u8 as
                     *const libc::c_char, (*realm).data);
        keyfile = defkeyfile.as_mut_ptr()
    }
    statrc = stat(keyfile, &mut stb);
    if statrc >= 0 as libc::c_int {
        /* if keyfile exists it better be a regular file */
        if !(stb.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
                 0o100000 as libc::c_int as libc::c_uint) {
            retval = 22 as libc::c_int;
            krb5_set_error_message(context, retval,
                                   dgettext(b"mit-krb5\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"keyfile (%s) is not a regular file: %s\x00"
                                                as *const u8 as
                                                *const libc::c_char), keyfile,
                                   error_message(retval as errcode_t));
            current_block = 15366887411438569156;
        } else { current_block = 13586036798005543211; }
    } else { current_block = 13586036798005543211; }
    match current_block {
        13586036798005543211 => {
            /*
     * We assume the stash file is in a directory writable only by root.
     * As such, don't worry about collisions, just do an atomic rename.
     */
            retval =
                asprintf(&mut tmp_ktname as *mut *mut libc::c_char,
                         b"FILE:%s_tmp\x00" as *const u8 as
                             *const libc::c_char, keyfile);
            if retval < 0 as libc::c_int {
                krb5_set_error_message(context, retval,
                                       dgettext(b"mit-krb5\x00" as *const u8
                                                    as *const libc::c_char,
                                                b"Could not create temp keytab file name.\x00"
                                                    as *const u8 as
                                                    *const libc::c_char));
            } else {
                /*
     * Set tmp_ktpath to point to the keyfile path (skip FILE:).  Subtracting
     * 1 to account for NULL terminator in sizeof calculation of a string
     * constant.  Used further down.
     */
                tmp_ktpath =
                    tmp_ktname.offset((::std::mem::size_of::<[libc::c_char; 6]>()
                                           as
                                           libc::c_ulong).wrapping_sub(1 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ulong)
                                          as isize);
                /*
     * This time-of-check-to-time-of-access race is fine; we care only
     * about an administrator running the command twice, not an attacker
     * trying to beat us to creating the file.  Per the above comment, we
     * assume the stash file is in a directory writable only by root.
     */
                statrc = stat(tmp_ktpath, &mut stb);
                if statrc == -(1 as libc::c_int) &&
                       *__errno_location() != 2 as libc::c_int {
                    /* ENOENT is the expected case */
                    retval = *__errno_location()
                } else if statrc == 0 as libc::c_int {
                    retval = 17 as libc::c_int;
                    krb5_set_error_message(context, retval,
                                           dgettext(b"mit-krb5\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    b"Temporary stash file already exists: %s.\x00"
                                                        as *const u8 as
                                                        *const libc::c_char),
                                           tmp_ktpath);
                } else {
                    /* create new stash keytab using temp file name */
                    retval = krb5_kt_resolve(context, tmp_ktname, &mut kt);
                    if !(retval != 0 as libc::c_int) {
                        while !keylist.is_null() && retval == 0 {
                            memset(&mut new_entry as *mut krb5_keytab_entry as
                                       *mut libc::c_void, 0 as libc::c_int,
                                   ::std::mem::size_of::<krb5_keytab_entry>()
                                       as libc::c_ulong);
                            new_entry.principal = mname;
                            new_entry.key = (*keylist).keyblock;
                            new_entry.vno = (*keylist).kvno;
                            retval =
                                krb5_kt_add_entry(context, kt,
                                                  &mut new_entry);
                            keylist = (*keylist).next
                        }
                        krb5_kt_close(context, kt);
                        if retval != 0 as libc::c_int {
                            /* Clean up by deleting the tmp keyfile if it exists. */
                            unlink(tmp_ktpath);
                        } else if rename(tmp_ktpath, keyfile) <
                                      0 as libc::c_int {
                            retval = *__errno_location();
                            krb5_set_error_message(context, retval,
                                                   dgettext(b"mit-krb5\x00" as
                                                                *const u8 as
                                                                *const libc::c_char,
                                                            b"rename of temporary keyfile (%s) to (%s) failed: %s\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char),
                                                   tmp_ktpath, keyfile,
                                                   error_message(*__errno_location()
                                                                     as
                                                                     errcode_t));
                        }
                    }
                }
            }
        }
        _ => { }
    }
    if !tmp_ktname.is_null() { free(tmp_ktname as *mut libc::c_void); }
    return retval;
}
#[c2rust::src_loc = "215:1"]
unsafe extern "C" fn krb5_db_def_fetch_mkey_stash(mut context: krb5_context,
                                                  mut keyfile:
                                                      *const libc::c_char,
                                                  mut key: *mut krb5_keyblock,
                                                  mut kvno: *mut krb5_kvno)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut enctype: krb5_ui_2 = 0;
    let mut keylength: krb5_ui_4 = 0;
    let mut kf: *mut FILE = 0 as *mut FILE;
    kf = fopen(keyfile, b"rb\x00" as *const u8 as *const libc::c_char);
    if kf.is_null() {
        return -(1780008429 as libc::c_long) as krb5_error_code
    }
    fcntl(fileno(kf), 2 as libc::c_int, 1 as libc::c_int);
    if fread(&mut enctype as *mut krb5_ui_2 as krb5_pointer,
             2 as libc::c_int as libc::c_ulong,
             1 as libc::c_int as libc::c_ulong, kf) !=
           1 as libc::c_int as libc::c_ulong {
        retval = -(1780008429 as libc::c_long) as krb5_error_code
    } else {
        if (*key).enctype == 0x1ff as libc::c_int {
            (*key).enctype = enctype as krb5_enctype;
            current_block = 7746791466490516765;
        } else if enctype as libc::c_int != (*key).enctype {
            retval = -(1780008428 as libc::c_long) as krb5_error_code;
            current_block = 8136586497274448693;
        } else { current_block = 7746791466490516765; }
        match current_block {
            8136586497274448693 => { }
            _ => {
                if fread(&mut keylength as *mut krb5_ui_4 as krb5_pointer,
                         ::std::mem::size_of::<krb5_ui_4>() as libc::c_ulong,
                         1 as libc::c_int as libc::c_ulong, kf) !=
                       1 as libc::c_int as libc::c_ulong {
                    retval = -(1780008429 as libc::c_long) as krb5_error_code
                } else {
                    (*key).length = keylength;
                    if (*key).length == 0 ||
                           (*key).length > 1024 as libc::c_int as libc::c_uint
                       {
                        retval =
                            -(1780008428 as libc::c_long) as krb5_error_code
                    } else {
                        (*key).contents =
                            malloc((*key).length as libc::c_ulong) as
                                *mut krb5_octet;
                        if (*key).contents.is_null() {
                            retval = 12 as libc::c_int
                        } else {
                            if fread((*key).contents as krb5_pointer,
                                     ::std::mem::size_of::<krb5_octet>() as
                                         libc::c_ulong,
                                     (*key).length as libc::c_ulong, kf) !=
                                   (*key).length as libc::c_ulong {
                                retval =
                                    -(1780008429 as libc::c_long) as
                                        krb5_error_code;
                                explicit_bzero((*key).contents as
                                                   *mut libc::c_void,
                                               (*key).length as size_t);
                                free((*key).contents as *mut libc::c_void);
                                (*key).contents = 0 as *mut krb5_octet
                            } else { retval = 0 as libc::c_int }
                            /* Atomically rename temp keyfile to original filename. */
                            /*
     * Note, the old stash format did not store the kvno and at this point it
     * can be assumed to be 1 as is the case for the mkey princ.  If the kvno is
     * passed in and isn't ignore_vno just leave it alone as this could cause
     * verifcation trouble if the mkey princ is using a kvno other than 1.
     */
                            if !kvno.is_null() &&
                                   *kvno == 0 as libc::c_int as libc::c_uint {
                                *kvno = 1 as libc::c_int as krb5_kvno
                            }
                        }
                    }
                }
            }
        }
    }
    fclose(kf);
    return retval;
}
#[c2rust::src_loc = "291:1"]
unsafe extern "C" fn krb5_db_def_fetch_mkey_keytab(mut context: krb5_context,
                                                   mut keyfile:
                                                       *const libc::c_char,
                                                   mut mname: krb5_principal,
                                                   mut key:
                                                       *mut krb5_keyblock,
                                                   mut kvno: *mut krb5_kvno)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut kt: krb5_keytab = 0 as krb5_keytab;
    let mut kt_ent: krb5_keytab_entry =
        krb5_keytab_entry{magic: 0,
                          principal: 0 as *mut krb5_principal_data,
                          timestamp: 0,
                          vno: 0,
                          key:
                              krb5_keyblock{magic: 0,
                                            enctype: 0,
                                            length: 0,
                                            contents:
                                                0 as *mut krb5_octet,},};
    let mut enctype: krb5_enctype = 0 as libc::c_int;
    retval = krb5_kt_resolve(context, keyfile, &mut kt);
    if !(retval != 0 as libc::c_int) {
        /* override default */
        if (*key).enctype != 0x1ff as libc::c_int { enctype = (*key).enctype }
        retval =
            krb5_kt_get_entry(context, kt, mname as krb5_const_principal,
                              (if !kvno.is_null() {
                                   *kvno
                               } else { 0 as libc::c_int as libc::c_uint }),
                              enctype, &mut kt_ent);
        if retval == 0 as libc::c_int {
            if (*key).enctype == 0x1ff as libc::c_int {
                (*key).enctype = kt_ent.key.enctype
            }
            if (kt_ent.key.length as libc::c_int) < 0 as libc::c_int {
                retval = -(1780008428 as libc::c_long) as krb5_error_code;
                krb5_kt_free_entry(context, &mut kt_ent);
            } else {
                (*key).length = kt_ent.key.length;
                /*
         * If a kvno pointer was passed in and it dereferences the
         * IGNORE_VNO value then it should be assigned the value of the kvno
         * found in the keytab otherwise the KNVO specified should be the
         * same as the one returned from the keytab.
         */
                if !kvno.is_null() &&
                       *kvno == 0 as libc::c_int as libc::c_uint {
                    *kvno = kt_ent.vno
                }
                /*
         * kt_ent will be free'd so need to allocate and copy key contents for
         * output to caller.
         */
                (*key).contents =
                    k5memdup(kt_ent.key.contents as *const libc::c_void,
                             kt_ent.key.length as size_t, &mut retval) as
                        *mut krb5_octet;
                if (*key).contents.is_null() {
                    krb5_kt_free_entry(context, &mut kt_ent);
                } else { krb5_kt_free_entry(context, &mut kt_ent); }
            }
        }
    }
    if !kt.is_null() { krb5_kt_close(context, kt); }
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "355:1"]
pub unsafe extern "C" fn krb5_db_def_fetch_mkey(mut context: krb5_context,
                                                mut mname: krb5_principal,
                                                mut key: *mut krb5_keyblock,
                                                mut kvno: *mut krb5_kvno,
                                                mut db_args:
                                                    *mut libc::c_char)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut keyfile: [libc::c_char; 4097] = [0; 4097];
    let mut realm: *mut krb5_data = &mut (*mname).realm;
    (*key).magic = -(1760647421 as libc::c_long) as krb5_magic;
    if !db_args.is_null() {
        strncpy(keyfile.as_mut_ptr(), db_args,
                ::std::mem::size_of::<[libc::c_char; 4097]>() as
                    libc::c_ulong);
    } else {
        snprintf(keyfile.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 4097]>() as
                     libc::c_ulong,
                 b"%s%s\x00" as *const u8 as *const libc::c_char,
                 b"/usr/local/var/krb5kdc/.k5.\x00" as *const u8 as
                     *const libc::c_char, (*realm).data);
    }
    /* null terminate no matter what */
    keyfile[(::std::mem::size_of::<[libc::c_char; 4097]>() as
                 libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                 libc::c_ulong) as usize] =
        '\u{0}' as i32 as libc::c_char;
    /* Try the keytab and old stash file formats. */
    retval =
        krb5_db_def_fetch_mkey_keytab(context, keyfile.as_mut_ptr(), mname,
                                      key, kvno);
    if retval as libc::c_long == -(1765328171 as libc::c_long) {
        retval =
            krb5_db_def_fetch_mkey_stash(context, keyfile.as_mut_ptr(), key,
                                         kvno)
    }
    /*
     * Use a generic error code for failure to retrieve the master
     * key, but set a message indicating the actual error.
     */
    if retval != 0 as libc::c_int {
        krb5_set_error_message(context,
                               -(1780008429 as libc::c_long) as
                                   krb5_error_code,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"Can not fetch master key (error: %s).\x00"
                                            as *const u8 as
                                            *const libc::c_char),
                               error_message(retval as errcode_t));
        return -(1780008429 as libc::c_long) as krb5_error_code
    } else { return 0 as libc::c_int };
}
#[no_mangle]
#[c2rust::src_loc = "395:1"]
pub unsafe extern "C" fn krb5_def_fetch_mkey_list(mut context: krb5_context,
                                                  mut mprinc: krb5_principal,
                                                  mut mkey:
                                                      *const krb5_keyblock,
                                                  mut mkeys_list:
                                                      *mut *mut krb5_keylist_node)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut master_entry: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut found_key: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut cur_mkey: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut mkey_list_head: *mut krb5_keylist_node =
        0 as *mut krb5_keylist_node;
    let mut mkey_list_node: *mut *mut krb5_keylist_node =
        0 as *mut *mut krb5_keylist_node;
    let mut key_data: *mut krb5_key_data = 0 as *mut krb5_key_data;
    let mut mkey_aux_data_list: *mut krb5_mkey_aux_node =
        0 as *mut krb5_mkey_aux_node;
    let mut aux_data_entry: *mut krb5_mkey_aux_node =
        0 as *mut krb5_mkey_aux_node;
    let mut i: libc::c_int = 0;
    if mkeys_list.is_null() { return 22 as libc::c_int }
    memset(&mut cur_mkey as *mut krb5_keyblock as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong);
    retval =
        krb5_db_get_principal(context, mprinc as krb5_const_principal,
                              0 as libc::c_int as libc::c_uint,
                              &mut master_entry);
    if retval as libc::c_long == -(1780008443 as libc::c_long) {
        return -(1780008432 as libc::c_long) as krb5_error_code
    }
    if retval != 0 { return retval }
    if (*master_entry).n_key_data as libc::c_int == 0 as libc::c_int {
        retval = -(1780008432 as libc::c_long) as krb5_error_code
    } else {
        /*
     * Check if the input mkey is the latest key and if it isn't then find the
     * latest mkey.
     */
        if (*mkey).enctype ==
               (*(*master_entry).key_data.offset(0 as libc::c_int as
                                                     isize)).key_data_type[0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               usize]
                   as libc::c_int {
            if krb5_dbe_decrypt_key_data(context, mkey,
                                         &mut *(*master_entry).key_data.offset(0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize),
                                         &mut cur_mkey,
                                         0 as *mut krb5_keysalt) ==
                   0 as libc::c_int {
                found_key = 1 as libc::c_int as krb5_boolean
            }
        }
        if found_key == 0 {
            retval =
                krb5_dbe_lookup_mkey_aux(context, master_entry,
                                         &mut mkey_aux_data_list);
            if retval != 0 {
                current_block = 8486992653222469080;
            } else {
                aux_data_entry = mkey_aux_data_list;
                while !aux_data_entry.is_null() {
                    if krb5_dbe_decrypt_key_data(context, mkey,
                                                 &mut (*aux_data_entry).latest_mkey,
                                                 &mut cur_mkey,
                                                 0 as *mut krb5_keysalt) ==
                           0 as libc::c_int {
                        found_key = 1 as libc::c_int as krb5_boolean;
                        break ;
                    } else { aux_data_entry = (*aux_data_entry).next }
                }
                if found_key != 1 as libc::c_int as libc::c_uint {
                    krb5_set_error_message(context,
                                           -(1780008431 as libc::c_long) as
                                               krb5_error_code,
                                           dgettext(b"mit-krb5\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    b"Unable to decrypt latest master key with the provided master key\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char));
                    retval = -(1780008431 as libc::c_long) as krb5_error_code;
                    current_block = 8486992653222469080;
                } else { current_block = 11932355480408055363; }
            }
        } else { current_block = 11932355480408055363; }
        match current_block {
            8486992653222469080 => { }
            _ => {
                /*
     * Extract all the mkeys from master_entry using the most current mkey and
     * create a mkey list for the mkeys field in kdc_realm_t.
     */
                mkey_list_head =
                    malloc(::std::mem::size_of::<krb5_keylist_node>() as
                               libc::c_ulong) as *mut krb5_keylist_node;
                if mkey_list_head.is_null() {
                    retval = 12 as libc::c_int
                } else {
                    memset(mkey_list_head as *mut libc::c_void,
                           0 as libc::c_int,
                           ::std::mem::size_of::<krb5_keylist_node>() as
                               libc::c_ulong);
                    /* Set mkey_list_head to the current mkey as an optimization. */
    /* mkvno may not be latest so ... */
                    (*mkey_list_head).kvno =
                        (*(*master_entry).key_data.offset(0 as libc::c_int as
                                                              isize)).key_data_kvno
                            as krb5_kvno;
                    /* this is the latest clear mkey (avoids a redundant decrypt) */
                    (*mkey_list_head).keyblock = cur_mkey;
                    /* loop through any other master keys creating a list of krb5_keylist_nodes */
                    mkey_list_node = &mut (*mkey_list_head).next;
                    i = 1 as libc::c_int;
                    loop  {
                        if !(i < (*master_entry).n_key_data as libc::c_int) {
                            current_block = 3689906465960840878;
                            break ;
                        }
                        if (*mkey_list_node).is_null() {
                            /* *mkey_list_node points to next field of previous node */
                            *mkey_list_node =
                                malloc(::std::mem::size_of::<krb5_keylist_node>()
                                           as libc::c_ulong) as
                                    *mut krb5_keylist_node;
                            if (*mkey_list_node).is_null() {
                                retval = 12 as libc::c_int;
                                current_block = 8486992653222469080;
                                break ;
                            } else {
                                memset(*mkey_list_node as *mut libc::c_void,
                                       0 as libc::c_int,
                                       ::std::mem::size_of::<krb5_keylist_node>()
                                           as libc::c_ulong);
                            }
                        }
                        key_data =
                            &mut *(*master_entry).key_data.offset(i as isize)
                                as *mut krb5_key_data;
                        retval =
                            krb5_dbe_decrypt_key_data(context, &mut cur_mkey,
                                                      key_data,
                                                      &mut (**mkey_list_node).keyblock,
                                                      0 as *mut krb5_keysalt);
                        if retval != 0 {
                            current_block = 8486992653222469080;
                            break ;
                        }
                        (**mkey_list_node).kvno =
                            (*key_data).key_data_kvno as krb5_kvno;
                        mkey_list_node = &mut (**mkey_list_node).next;
                        i += 1
                    }
                    match current_block {
                        8486992653222469080 => { }
                        _ => { *mkeys_list = mkey_list_head }
                    }
                }
            }
        }
    }
    krb5_db_free_principal(context, master_entry);
    krb5_dbe_free_mkey_aux_list(context, mkey_aux_data_list);
    if retval != 0 as libc::c_int {
        krb5_dbe_free_key_list(context, mkey_list_head);
    }
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "515:1"]
pub unsafe extern "C" fn krb5_db_def_rename_principal(mut kcontext:
                                                          krb5_context,
                                                      mut source:
                                                          krb5_const_principal,
                                                      mut target:
                                                          krb5_const_principal)
 -> krb5_error_code {
    let mut kdb: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut oldprinc: krb5_principal = 0 as *mut krb5_principal_data;
    let mut ret: krb5_error_code = 0;
    if source.is_null() || target.is_null() { return 22 as libc::c_int }
    ret =
        krb5_db_get_principal(kcontext, source,
                              0 as libc::c_int as libc::c_uint, &mut kdb);
    if !(ret != 0) {
        /* Store salt values explicitly so that they don't depend on the principal
     * name. */
        ret = krb5_dbe_specialize_salt(kcontext, kdb);
        if !(ret != 0) {
            /* Temporarily alias kdb->princ to target and put the principal entry. */
            oldprinc = (*kdb).princ;
            (*kdb).princ = target as krb5_principal;
            ret = krb5_db_put_principal(kcontext, kdb);
            (*kdb).princ = oldprinc;
            if !(ret != 0) {
                ret =
                    krb5_db_delete_principal(kcontext,
                                             source as krb5_principal)
            }
        }
    }
    krb5_db_free_principal(kcontext, kdb);
    return ret;
}
