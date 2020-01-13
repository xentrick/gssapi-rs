use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:31"]
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
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:31"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:31"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:31"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
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
    #[c2rust::src_loc = "178:1"]
    pub type krb5_addrtype = krb5_int32;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "185:1"]
    pub type krb5_preauthtype = krb5_int32;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "323:16"]
    pub struct _krb5_address {
        pub magic: krb5_magic,
        pub addrtype: krb5_addrtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* *< Anonymous realm */
    /* *< Anonymous principal name */
    /*
 * end "base-defs.h"
 */
    /*
 * begin "hostaddr.h"
 */
    /* * Structure for address */
    #[c2rust::src_loc = "323:1"]
    pub type krb5_address = _krb5_address;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "6811:16"]
    pub struct _krb5_get_init_creds_opt {
        pub flags: krb5_flags,
        pub tkt_life: krb5_deltat,
        pub renew_life: krb5_deltat,
        pub forwardable: libc::c_int,
        pub proxiable: libc::c_int,
        pub etype_list: *mut krb5_enctype,
        pub etype_list_length: libc::c_int,
        pub address_list: *mut *mut krb5_address,
        pub preauth_list: *mut krb5_preauthtype,
        pub preauth_list_length: libc::c_int,
        pub salt: *mut krb5_data,
    }
    #[c2rust::src_loc = "6811:1"]
    pub type krb5_get_init_creds_opt = _krb5_get_init_creds_opt;
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
 * Convert a string (such as a password) to a key with additional parameters.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [in]  string          String to be converted
 * @param [in]  salt            Salt value
 * @param [in]  params          Parameters
 * @param [out] key             Generated key
 *
 * This function is similar to krb5_c_string_to_key(), but also takes
 * parameters which may affect the algorithm in an enctype-dependent way.  The
 * newly created @a key must be released by calling
 * krb5_free_keyblock_contents() when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "861:1"]
        pub fn krb5_c_string_to_key_with_params(context: krb5_context,
                                                enctype: krb5_enctype,
                                                string: *const krb5_data,
                                                salt: *const krb5_data,
                                                params: *const krb5_data,
                                                key: *mut krb5_keyblock)
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
        /* *
 * Start a sequential retrieval of key table entries.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [out] cursor          Cursor
 *
 * Prepare to read sequentially every key in the specified key table.  Use
 * krb5_kt_end_seq_get() to release the cursor when it is no longer needed.
 *
 * @sa krb5_kt_next_entry(), krb5_kt_end_seq_get()
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2833:1"]
        pub fn krb5_kt_start_seq_get(context: krb5_context,
                                     keytab: krb5_keytab,
                                     cursor: *mut krb5_kt_cursor)
         -> krb5_error_code;
        /* *
 * Retrieve the next entry from the key table.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [out] entry           Returned key table entry
 * @param [in]  cursor          Key table cursor
 *
 * Return the next sequential entry in @a keytab and advance @a cursor.
 * Callers must release the returned entry with krb5_kt_free_entry().
 *
 * @sa krb5_kt_start_seq_get(), krb5_kt_end_seq_get()
 *
 * @retval
 * 0 Success
 * @retval
 * KRB5_KT_END - if the last entry was reached
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2857:1"]
        pub fn krb5_kt_next_entry(context: krb5_context, keytab: krb5_keytab,
                                  entry: *mut krb5_keytab_entry,
                                  cursor: *mut krb5_kt_cursor)
         -> krb5_error_code;
        /* *
 * Release a keytab cursor.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [out] cursor          Cursor
 *
 * This function should be called to release the cursor created by
 * krb5_kt_start_seq_get().
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2876:1"]
        pub fn krb5_kt_end_seq_get(context: krb5_context, keytab: krb5_keytab,
                                   cursor: *mut krb5_kt_cursor)
         -> krb5_error_code;
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
 * Convert a krb5_principal structure to a string representation.
 *
 * @param [in]  context         Library context
 * @param [in]  principal       Principal
 * @param [out] name            String representation of principal name
 *
 * The resulting string representation uses the format and quoting conventions
 * described for krb5_parse_name().
 *
 * Use krb5_free_unparsed_name() to free @a name when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3489:1"]
        pub fn krb5_unparse_name(context: krb5_context,
                                 principal: krb5_const_principal,
                                 name: *mut *mut libc::c_char)
         -> krb5_error_code;
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
        #[c2rust::src_loc = "4313:1"]
        pub fn krb5_principal2salt(context: krb5_context,
                                   pr: krb5_const_principal,
                                   ret: *mut krb5_data) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4758:1"]
        pub fn krb5_free_data_contents(context: krb5_context,
                                       val: *mut krb5_data);
        #[no_mangle]
        #[c2rust::src_loc = "4767:1"]
        pub fn krb5_free_unparsed_name(context: krb5_context,
                                       val: *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "4837:1"]
        pub fn krb5_timeofday(context: krb5_context,
                              timeret: *mut krb5_timestamp)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6096:1"]
        pub fn krb5_read_password(context: krb5_context,
                                  prompt: *const libc::c_char,
                                  prompt2: *const libc::c_char,
                                  return_pwd: *mut libc::c_char,
                                  size_return: *mut libc::c_uint)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6266:1"]
        pub fn krb5_string_to_enctype(string: *mut libc::c_char,
                                      enctypep: *mut krb5_enctype)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6851:1"]
        pub fn krb5_get_init_creds_opt_alloc(context: krb5_context,
                                             opt:
                                                 *mut *mut krb5_get_init_creds_opt)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6863:1"]
        pub fn krb5_get_init_creds_opt_free(context: krb5_context,
                                            opt:
                                                *mut krb5_get_init_creds_opt);
        #[no_mangle]
        #[c2rust::src_loc = "6943:1"]
        pub fn krb5_get_init_creds_opt_set_etype_list(opt:
                                                          *mut krb5_get_init_creds_opt,
                                                      etype_list:
                                                          *mut krb5_enctype,
                                                      etype_list_length:
                                                          libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "7307:1"]
        pub fn krb5_get_etype_info(context: krb5_context,
                                   principal: krb5_principal,
                                   opt: *mut krb5_get_init_creds_opt,
                                   enctype_out: *mut krb5_enctype,
                                   salt_out: *mut krb5_data,
                                   s2kparams_out: *mut krb5_data)
         -> krb5_error_code;
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
    #[c2rust::src_loc = "2308:1"]
    pub unsafe extern "C" fn k5alloc(mut size: size_t,
                                     mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        return k5calloc(1 as libc::c_int as size_t, size, code);
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
    #[c2rust::src_loc = "2274:1"]
    pub unsafe extern "C" fn alloc_data(mut data: *mut krb5_data,
                                        mut len: libc::c_uint)
     -> krb5_error_code {
        let mut ptr: *mut libc::c_char =
            calloc(if len > 0 as libc::c_int as libc::c_uint {
                       len
                   } else { 1 as libc::c_int as libc::c_uint } as
                       libc::c_ulong, 1 as libc::c_int as libc::c_ulong) as
                *mut libc::c_char;
        if ptr.is_null() { return 12 as libc::c_int }
        (*data).magic = -(1760647422 as libc::c_long) as krb5_magic;
        (*data).data = ptr;
        (*data).length = len;
        return 0 as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "2268:1"]
    pub unsafe extern "C" fn string2data(mut str: *mut libc::c_char)
     -> krb5_data {
        return make_data(str as *mut libc::c_void,
                         strlen(str) as libc::c_uint);
    }
    #[inline]
    #[c2rust::src_loc = "2262:1"]
    pub unsafe extern "C" fn empty_data() -> krb5_data {
        return make_data(0 as *mut libc::c_void,
                         0 as libc::c_int as libc::c_uint);
    }
    #[inline]
    #[c2rust::src_loc = "2251:1"]
    pub unsafe extern "C" fn make_data(mut data: *mut libc::c_void,
                                       mut len: libc::c_uint) -> krb5_data {
        let mut d: krb5_data =
            krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
        d.magic = -(1760647422 as libc::c_long) as krb5_magic;
        d.data = data as *mut libc::c_char;
        d.length = len;
        return d;
    }
    #[inline]
    #[c2rust::src_loc = "656:1"]
    pub unsafe extern "C" fn zapfree(mut ptr: *mut libc::c_void,
                                     mut len: size_t) {
        if !ptr.is_null() { explicit_bzero(ptr, len); free(ptr); };
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_pointer, krb5_error_code, krb5_context,
                        krb5_keytab, krb5_const_principal, krb5_kvno,
                        krb5_keytab_entry, krb5_kt_cursor, krb5_data};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::stddef_h::size_t;
    use super::stdlib_h::{calloc, free};
    use super::string_h::{strlen, explicit_bzero};
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
        #[c2rust::src_loc = "898:1"]
        pub fn krb5int_copy_data_contents(_: krb5_context,
                                          _: *const krb5_data,
                                          _: *mut krb5_data)
         -> krb5_error_code;
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
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
  "/home/nmavis/dev/gssapi-rs/code/src/kadmin/ktutil/ktutil.h:33"]
pub mod ktutil_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kadmin/ktutil/ktutil.h */
/*
 * Copyright 1995 by the Massachusetts Institute of Technology.
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
    #[c2rust::src_loc = "27:1"]
    pub type krb5_kt_list = *mut _krb5_kt_list;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:16"]
    pub struct _krb5_kt_list {
        pub next: *mut _krb5_kt_list,
        pub entry: *mut krb5_keytab_entry,
    }
    use super::krb5_h::krb5_keytab_entry;
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
        #[c2rust::src_loc = "137:14"]
        pub static mut stdin: *mut FILE;
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
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "564:1"]
        pub fn fgets(__s: *mut libc::c_char, __n: libc::c_int,
                     __stream: *mut FILE) -> *mut libc::c_char;
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
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:31"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-hex.h:32"]
pub mod k5_hex_h {
    use super::stdint_uintn_h::uint8_t;
    use super::stddef_h::size_t;
    extern "C" {
        /*
 * Decode hex bytes, placing the result in allocated storage in *bytes_out and
 * *len_out.  Null-terminate the result (primarily for decoding passwords in
 * libkdb_ldap).  Return 0 on success, ENOMEM or EINVAL on error.
 */
        #[no_mangle]
        #[c2rust::src_loc = "51:1"]
        pub fn k5_hex_decode(hex: *const libc::c_char,
                             bytes_out: *mut *mut uint8_t,
                             len_out: *mut size_t) -> libc::c_int;
    }
    /* K5_HEX_H */
}
pub use self::types_h::{__uint8_t, __int32_t, __off_t, __off64_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint8_t;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_kvno,
                       krb5_addrtype, krb5_enctype, krb5_preauthtype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_pointer, krb5_principal_data, krb5_principal,
                       krb5_const_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _krb5_keyblock, krb5_keyblock,
                       krb5_kt_cursor, krb5_keytab_entry_st,
                       krb5_keytab_entry, krb5_keytab,
                       _krb5_get_init_creds_opt, krb5_get_init_creds_opt,
                       _profile_t, krb5_c_string_to_key_with_params,
                       krb5_kt_close, krb5_kt_start_seq_get,
                       krb5_kt_next_entry, krb5_kt_end_seq_get,
                       krb5_parse_name, krb5_unparse_name, krb5_kt_resolve,
                       krb5_kt_free_entry, krb5_kt_add_entry,
                       krb5_principal2salt, krb5_free_data_contents,
                       krb5_free_unparsed_name, krb5_timeofday,
                       krb5_read_password, krb5_string_to_enctype,
                       krb5_get_init_creds_opt_alloc,
                       krb5_get_init_creds_opt_free,
                       krb5_get_init_creds_opt_set_etype_list,
                       krb5_get_etype_info};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, _krb5_kt, _krb5_kt_ops, k5alloc,
                         k5calloc, alloc_data, string2data, empty_data,
                         make_data, zapfree, plugin_mapping, _kdb_log_context,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle,
                         krb5int_copy_data_contents};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::ktutil_h::{krb5_kt_list, _krb5_kt_list};
use self::stdlib_h::{malloc, calloc, free};
use self::stdio_h::{stdin, stderr, fprintf, printf, snprintf, fgets};
use self::libintl_h::dgettext;
use self::string_h::{explicit_bzero, strlen, memset};
use self::assert_h::__assert_fail;
use self::k5_hex_h::k5_hex_decode;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kadmin/ktutil/ktutil_funcs.c */
/*
 *(C) Copyright 1995, 1996 by the Massachusetts Institute of Technology.
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
 * Utility functions for ktutil.
 */
/*
 * Free a kt_list
 */
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn ktutil_free_kt_list(mut context: krb5_context,
                                             mut list: krb5_kt_list)
 -> krb5_error_code {
    let mut lp: krb5_kt_list = 0 as *mut _krb5_kt_list;
    let mut prev: krb5_kt_list = 0 as *mut _krb5_kt_list;
    let mut retval: krb5_error_code = 0 as libc::c_int;
    lp = list;
    while !lp.is_null() {
        retval = krb5_kt_free_entry(context, (*lp).entry);
        free((*lp).entry as *mut libc::c_void);
        if retval != 0 { break ; }
        prev = lp;
        lp = (*lp).next;
        free(prev as *mut libc::c_void);
    }
    return retval;
}
/*
 * Delete a numbered entry in a kt_list.  Takes a pointer to a kt_list
 * in case head gets deleted.
 */
#[no_mangle]
#[c2rust::src_loc = "63:1"]
pub unsafe extern "C" fn ktutil_delete(mut context: krb5_context,
                                       mut list: *mut krb5_kt_list,
                                       mut idx: libc::c_int)
 -> krb5_error_code {
    let mut lp: krb5_kt_list = 0 as *mut _krb5_kt_list;
    let mut prev: krb5_kt_list = 0 as *mut _krb5_kt_list;
    let mut i: libc::c_int = 0;
    lp = *list;
    i = 1 as libc::c_int;
    while !lp.is_null() {
        if i == idx {
            if i == 1 as libc::c_int {
                *list = (*lp).next
            } else { (*prev).next = (*lp).next }
            (*lp).next = 0 as *mut _krb5_kt_list;
            return ktutil_free_kt_list(context, lp)
        }
        prev = lp;
        lp = (*lp).next;
        i += 1
    }
    return 22 as libc::c_int;
}
/*
 * Determine the enctype, salt, and s2kparams for princ based on the presence
 * of the -f flag (fetch), the optionally specified salt string, and the
 * optionally specified enctype.  If the fetch flag is used, salt_str must not
 * be given; if the fetch flag is not used, the enctype must be given.
 */
#[c2rust::src_loc = "90:1"]
unsafe extern "C" fn get_etype_info(mut context: krb5_context,
                                    mut princ: krb5_principal,
                                    mut fetch: libc::c_int,
                                    mut salt_str: *mut libc::c_char,
                                    mut enctype_inout: *mut krb5_enctype,
                                    mut salt_out: *mut krb5_data,
                                    mut s2kparams_out: *mut krb5_data)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut enctype: krb5_enctype = 0;
    let mut opt: *mut krb5_get_init_creds_opt =
        0 as *mut krb5_get_init_creds_opt;
    let mut salt: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    *salt_out = empty_data();
    *s2kparams_out = empty_data();
    if fetch == 0 {
        /* Use the specified enctype and either the specified or default salt.
         * Do not produce s2kparams. */
        if *enctype_inout != 0 as libc::c_int {
        } else {
            __assert_fail(b"*enctype_inout != ENCTYPE_NULL\x00" as *const u8
                              as *const libc::c_char,
                          b"ktutil_funcs.c\x00" as *const u8 as
                              *const libc::c_char,
                          106 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 116],
                                                    &[libc::c_char; 116]>(b"krb5_error_code get_etype_info(krb5_context, krb5_principal, int, char *, krb5_enctype *, krb5_data *, krb5_data *)\x00")).as_ptr());
        }
        if !salt_str.is_null() {
            salt = string2data(salt_str);
            return krb5int_copy_data_contents(context, &mut salt, salt_out)
        } else {
            return krb5_principal2salt(context, princ as krb5_const_principal,
                                       salt_out)
        }
    }
    /* Get etype-info from the KDC. */
    if salt_str.is_null() {
    } else {
        __assert_fail(b"salt_str == NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"ktutil_funcs.c\x00" as *const u8 as
                          *const libc::c_char,
                      116 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 116],
                                                &[libc::c_char; 116]>(b"krb5_error_code get_etype_info(krb5_context, krb5_principal, int, char *, krb5_enctype *, krb5_data *, krb5_data *)\x00")).as_ptr());
    }
    if *enctype_inout != 0 as libc::c_int {
        retval = krb5_get_init_creds_opt_alloc(context, &mut opt);
        if retval != 0 { return retval }
        krb5_get_init_creds_opt_set_etype_list(opt, enctype_inout,
                                               1 as libc::c_int);
    }
    retval =
        krb5_get_etype_info(context, princ, opt, &mut enctype, salt_out,
                            s2kparams_out);
    krb5_get_init_creds_opt_free(context, opt);
    if retval != 0 { return retval }
    if enctype == 0 as libc::c_int {
        return -(1765328370 as libc::c_long) as krb5_error_code
    }
    *enctype_inout = enctype;
    return 0 as libc::c_int;
}
/*
 * Create a new keytab entry and add it to the keytab list.
 * Based on the value of use_pass, either prompt the user for a
 * password or key.  If the keytab list is NULL, allocate a new
 * one first.
 */
#[no_mangle]
#[c2rust::src_loc = "141:1"]
pub unsafe extern "C" fn ktutil_add(mut context: krb5_context,
                                    mut list: *mut krb5_kt_list,
                                    mut princ_str: *mut libc::c_char,
                                    mut fetch: libc::c_int,
                                    mut kvno: krb5_kvno,
                                    mut enctype_str: *mut libc::c_char,
                                    mut use_pass: libc::c_int,
                                    mut salt_str: *mut libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut entry: *mut krb5_keytab_entry = 0 as *mut krb5_keytab_entry;
    let mut lp: krb5_kt_list = 0 as *mut _krb5_kt_list;
    let mut last: *mut krb5_kt_list = 0 as *mut krb5_kt_list;
    let mut princ: krb5_principal = 0 as *mut krb5_principal_data;
    let mut enctype: krb5_enctype = 0 as libc::c_int;
    let mut now: krb5_timestamp = 0;
    let mut retval: krb5_error_code = 0;
    let mut password: krb5_data = empty_data();
    let mut salt: krb5_data = empty_data();
    let mut params: krb5_data = empty_data();
    let mut s2kparams: *mut krb5_data = 0 as *mut krb5_data;
    let mut key: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut promptstr: [libc::c_char; 1024] = [0; 1024];
    let mut princ_full: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut keybytes: *mut uint8_t = 0 as *mut uint8_t;
    let mut keylen: size_t = 0;
    let mut pwsize: libc::c_uint = 8192 as libc::c_int as libc::c_uint;
    retval = krb5_parse_name(context, princ_str, &mut princ);
    if !(retval != 0) {
        /* now unparse in order to get the default realm appended
       to princ_str, if no realm was specified */
        retval =
            krb5_unparse_name(context, princ as krb5_const_principal,
                              &mut princ_full);
        if !(retval != 0) {
            if !enctype_str.is_null() {
                retval = krb5_string_to_enctype(enctype_str, &mut enctype);
                if retval != 0 {
                    retval = -(1765328196 as libc::c_long) as krb5_error_code;
                    current_block = 4462129297499890814;
                } else { current_block = 8457315219000651999; }
            } else { current_block = 8457315219000651999; }
            match current_block {
                4462129297499890814 => { }
                _ => {
                    retval = krb5_timeofday(context, &mut now);
                    if !(retval != 0) {
                        entry =
                            k5alloc(::std::mem::size_of::<krb5_keytab_entry>()
                                        as libc::c_ulong, &mut retval) as
                                *mut krb5_keytab_entry;
                        if !entry.is_null() {
                            if use_pass != 0 {
                                retval = alloc_data(&mut password, pwsize);
                                if retval != 0 {
                                    current_block = 4462129297499890814;
                                } else {
                                    snprintf(promptstr.as_mut_ptr(),
                                             ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                 as libc::c_ulong,
                                             dgettext(b"mit-krb5\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      b"Password for %.1000s\x00"
                                                          as *const u8 as
                                                          *const libc::c_char),
                                             princ_full);
                                    retval =
                                        krb5_read_password(context,
                                                           promptstr.as_mut_ptr(),
                                                           0 as
                                                               *const libc::c_char,
                                                           password.data,
                                                           &mut password.length);
                                    if retval != 0 {
                                        current_block = 4462129297499890814;
                                    } else {
                                        retval =
                                            get_etype_info(context, princ,
                                                           fetch, salt_str,
                                                           &mut enctype,
                                                           &mut salt,
                                                           &mut params);
                                        if retval != 0 {
                                            current_block =
                                                4462129297499890814;
                                        } else {
                                            s2kparams =
                                                if params.length >
                                                       0 as libc::c_int as
                                                           libc::c_uint {
                                                    &mut params
                                                } else {
                                                    0 as *mut krb5_data
                                                };
                                            retval =
                                                krb5_c_string_to_key_with_params(context,
                                                                                 enctype,
                                                                                 &mut password,
                                                                                 &mut salt,
                                                                                 s2kparams,
                                                                                 &mut key);
                                            if retval != 0 {
                                                current_block =
                                                    4462129297499890814;
                                            } else {
                                                (*entry).key = key;
                                                current_block =
                                                    16738040538446813684;
                                            }
                                        }
                                    }
                                }
                            } else {
                                printf(dgettext(b"mit-krb5\x00" as *const u8
                                                    as *const libc::c_char,
                                                b"Key for %s (hex): \x00" as
                                                    *const u8 as
                                                    *const libc::c_char),
                                       princ_full);
                                fgets(buf.as_mut_ptr(), 8192 as libc::c_int,
                                      stdin);
                                /*
         * We need to get rid of the trailing '\n' from fgets.
         * If we have an even number of hex digits (as we should),
         * write a '\0' over the '\n'.  If for some reason we have
         * an odd number of hex digits, force an even number of hex
         * digits by writing a '0' into the last position (the string
         * will still be null-terminated).
         */
                                buf[strlen(buf.as_mut_ptr()).wrapping_sub(1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong)
                                        as usize] =
                                    if strlen(buf.as_mut_ptr()).wrapping_rem(2
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_ulong)
                                           != 0 {
                                        '\u{0}' as i32
                                    } else { '0' as i32 } as libc::c_char;
                                if strlen(buf.as_mut_ptr()) ==
                                       0 as libc::c_int as libc::c_ulong {
                                    fprintf(stderr,
                                            dgettext(b"mit-krb5\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     b"addent: Error reading key.\n\x00"
                                                         as *const u8 as
                                                         *const libc::c_char));
                                    retval = 0 as libc::c_int;
                                    current_block = 4462129297499890814;
                                } else {
                                    retval =
                                        k5_hex_decode(buf.as_mut_ptr(),
                                                      &mut keybytes,
                                                      &mut keylen);
                                    if retval != 0 {
                                        if retval == 22 as libc::c_int {
                                            fprintf(stderr,
                                                    dgettext(b"mit-krb5\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             b"addent: Illegal character in key.\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char));
                                            retval = 0 as libc::c_int
                                        }
                                        current_block = 4462129297499890814;
                                    } else {
                                        (*entry).key.enctype = enctype;
                                        (*entry).key.contents = keybytes;
                                        (*entry).key.length =
                                            keylen as libc::c_uint;
                                        current_block = 16738040538446813684;
                                    }
                                }
                            }
                            match current_block {
                                4462129297499890814 => { }
                                _ => {
                                    (*entry).principal = princ;
                                    (*entry).vno = kvno;
                                    (*entry).timestamp = now;
                                    /* Add entry to the end of the list (or create a new list if empty). */
                                    lp =
                                        k5alloc(::std::mem::size_of::<_krb5_kt_list>()
                                                    as libc::c_ulong,
                                                &mut retval) as krb5_kt_list;
                                    if !lp.is_null() {
                                        (*lp).next = 0 as *mut _krb5_kt_list;
                                        (*lp).entry = entry;
                                        entry = 0 as *mut krb5_keytab_entry;
                                        last = list;
                                        while !(*last).is_null() {
                                            last = &mut (**last).next
                                        }
                                        *last = lp
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    krb5_kt_free_entry(context, entry);
    zapfree(password.data as *mut libc::c_void, password.length as size_t);
    krb5_free_data_contents(context, &mut salt);
    krb5_free_data_contents(context, &mut params);
    krb5_free_unparsed_name(context, princ_full);
    return retval;
}
/*
 * Read in a keytab and append it to list.  If list starts as NULL,
 * allocate a new one if necessary.
 */
#[no_mangle]
#[c2rust::src_loc = "271:1"]
pub unsafe extern "C" fn ktutil_read_keytab(mut context: krb5_context,
                                            mut name: *mut libc::c_char,
                                            mut list: *mut krb5_kt_list)
 -> krb5_error_code {
    let mut lp: krb5_kt_list = 0 as krb5_kt_list;
    let mut tail: krb5_kt_list = 0 as krb5_kt_list;
    let mut back: krb5_kt_list = 0 as krb5_kt_list;
    let mut kt: krb5_keytab = 0 as *mut _krb5_kt;
    let mut entry: *mut krb5_keytab_entry = 0 as *mut krb5_keytab_entry;
    let mut cursor: krb5_kt_cursor = 0 as *mut libc::c_void;
    let mut retval: krb5_error_code = 0 as libc::c_int;
    if !(*list).is_null() {
        /* point lp at the tail of the list */
        lp = *list;
        while !(*lp).next.is_null() { lp = (*lp).next }
        back = lp
    }
    retval = krb5_kt_resolve(context, name, &mut kt);
    if retval != 0 { return retval }
    retval = krb5_kt_start_seq_get(context, kt, &mut cursor);
    if !(retval != 0) {
        loop  {
            entry =
                malloc(::std::mem::size_of::<krb5_keytab_entry>() as
                           libc::c_ulong) as *mut krb5_keytab_entry;
            if entry.is_null() {
                retval = 12 as libc::c_int;
                break ;
            } else {
                memset(entry as *mut libc::c_void, 0 as libc::c_int,
                       ::std::mem::size_of::<krb5_keytab_entry>() as
                           libc::c_ulong);
                retval = krb5_kt_next_entry(context, kt, entry, &mut cursor);
                if retval != 0 { break ; }
                if lp.is_null() {
                    /* if list is empty, start one */
                    lp =
                        malloc(::std::mem::size_of::<_krb5_kt_list>() as
                                   libc::c_ulong) as krb5_kt_list;
                    if lp.is_null() { retval = 12 as libc::c_int; break ; }
                } else {
                    (*lp).next =
                        malloc(::std::mem::size_of::<_krb5_kt_list>() as
                                   libc::c_ulong) as krb5_kt_list;
                    if (*lp).next.is_null() {
                        retval = 12 as libc::c_int;
                        break ;
                    } else { lp = (*lp).next }
                }
                if tail.is_null() { tail = lp }
                (*lp).next = 0 as *mut _krb5_kt_list;
                (*lp).entry = entry
            }
        }
        if !entry.is_null() { free(entry as *mut libc::c_void); }
        if retval != 0 {
            if retval as libc::c_long == -(1765328202 as libc::c_long) {
                retval = 0 as libc::c_int
            } else {
                ktutil_free_kt_list(context, tail);
                tail = 0 as krb5_kt_list;
                if !back.is_null() { (*back).next = 0 as *mut _krb5_kt_list }
            }
        }
        if (*list).is_null() { *list = tail }
        krb5_kt_end_seq_get(context, kt, &mut cursor);
    }
    krb5_kt_close(context, kt);
    return retval;
}
/*
 * Takes a kt_list and writes it to the named keytab.
 */
#[no_mangle]
#[c2rust::src_loc = "346:1"]
pub unsafe extern "C" fn ktutil_write_keytab(mut context: krb5_context,
                                             mut list: krb5_kt_list,
                                             mut name: *mut libc::c_char)
 -> krb5_error_code {
    let mut lp: krb5_kt_list = 0 as *mut _krb5_kt_list;
    let mut kt: krb5_keytab = 0 as *mut _krb5_kt;
    let mut ktname: [libc::c_char; 4105] = [0; 4105];
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut result: libc::c_int = 0;
    result =
        snprintf(ktname.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 4105]>() as
                     libc::c_ulong,
                 b"WRFILE:%s\x00" as *const u8 as *const libc::c_char, name);
    if result as libc::c_uint as libc::c_ulong >=
           ::std::mem::size_of::<[libc::c_char; 4105]>() as libc::c_ulong {
        return 36 as libc::c_int
    }
    retval = krb5_kt_resolve(context, ktname.as_mut_ptr(), &mut kt);
    if retval != 0 { return retval }
    lp = list;
    while !lp.is_null() {
        retval = krb5_kt_add_entry(context, kt, (*lp).entry);
        if retval != 0 { break ; }
        lp = (*lp).next
    }
    krb5_kt_close(context, kt);
    return retval;
}
