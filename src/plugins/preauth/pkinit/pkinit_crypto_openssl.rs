use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
}
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "31:1"]
    pub type __u_char = libc::c_uchar;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = libc::c_ulong;
    #[c2rust::src_loc = "148:1"]
    pub type __ino_t = libc::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/sys/types.h:32"]
pub mod sys_types_h {
    #[c2rust::src_loc = "33:1"]
    pub type u_char = __u_char;
    #[c2rust::src_loc = "150:1"]
    pub type uint = libc::c_uint;
    use super::types_h::__u_char;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:32"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:32"]
pub mod pthreadtypes_h {
    #[c2rust::src_loc = "53:1"]
    pub type pthread_once_t = libc::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stdarg.h:32"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint8_t, __uint32_t, __uint64_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:32"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:32"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:32"]
pub mod k5_thread_h {
    #[c2rust::src_loc = "188:1"]
    pub type k5_os_nothread_once_t = libc::c_uchar;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "256:9"]
    pub struct k5_once_t {
        pub o: pthread_once_t,
        pub n: k5_os_nothread_once_t,
    }
    use super::pthreadtypes_h::pthread_once_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "262:1"]
        pub fn k5_once(once: *mut k5_once_t,
                       fn_0: Option<unsafe extern "C" fn() -> ()>)
         -> libc::c_int;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:32"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "237:9"]
    pub struct k5_init_t {
        pub once: k5_once_t,
        pub error: libc::c_int,
        pub did_run: libc::c_int,
        pub fn_0: Option<unsafe extern "C" fn() -> ()>,
    }
    use super::k5_thread_h::k5_once_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "887:1"]
        pub fn krb5int_strlcpy(dst: *mut libc::c_char,
                               src: *const libc::c_char, siz: size_t)
         -> size_t;
    }
    /* K5_PLATFORM_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:32"]
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
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "185:1"]
    pub type krb5_preauthtype = krb5_int32;
    /* This may change, later on */
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2038:16"]
    pub struct _krb5_pa_data {
        pub magic: krb5_magic,
        pub pa_type: krb5_preauthtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* * Pre-authentication data */
    #[c2rust::src_loc = "2038:1"]
    pub type krb5_pa_data = _krb5_pa_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "6424:16"]
    pub struct _krb5_prompt {
        pub prompt: *mut libc::c_char,
        pub hidden: libc::c_int,
        pub reply: *mut krb5_data,
    }
    #[c2rust::src_loc = "6424:1"]
    pub type krb5_prompt = _krb5_prompt;
    #[c2rust::src_loc = "6431:1"]
    pub type krb5_prompter_fct
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: *const libc::c_char,
                                    _: *const libc::c_char, _: libc::c_int,
                                    _: *mut krb5_prompt) -> krb5_error_code>;
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::_krb5_context;
    use super::stddef_h::size_t;
    extern "C" {
        /* *< Preauthentication data type */
        /* *< Length of data */
        /* *< Data */
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
 * Build an anonymous principal.
 *
 * This function returns constant storage that must not be freed.
 *
 * @sa #KRB5_ANONYMOUS_PRINCSTR
 */
        #[no_mangle]
        #[c2rust::src_loc = "309:1"]
        pub fn krb5_anonymous_principal() -> krb5_const_principal;
        /* *
 * Return length of the specified key in bytes.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [out] keybytes        Number of bytes required to make a key
 * @param [out] keylength       Length of final key
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "605:1"]
        pub fn krb5_c_keylengths(context: krb5_context, enctype: krb5_enctype,
                                 keybytes: *mut size_t,
                                 keylength: *mut size_t) -> krb5_error_code;
        /* *
 * Generate an enctype-specific key from random data.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [in]  random_data     Random input data
 * @param [out] k5_random_key   Resulting key
 *
 * This function takes random input data @a random_data and produces a valid
 * key @a k5_random_key for a given @a enctype.
 *
 * @note It is assumed that @a k5_random_key has already been initialized and
 * @a k5_random_key->contents has been allocated with the correct length.
 *
 * @sa krb5_c_keylengths()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "767:1"]
        pub fn krb5_c_random_to_key(context: krb5_context,
                                    enctype: krb5_enctype,
                                    random_data: *mut krb5_data,
                                    k5_random_key: *mut krb5_keyblock)
         -> krb5_error_code;
        /* *
 * Compare two principals ignoring realm components.
 *
 * @param [in] context          Library context
 * @param [in] princ1           First principal
 * @param [in] princ2           Second principal
 *
 * Similar to krb5_principal_compare(), but do not compare the realm
 * components of the principals.
 *
 * @retval
 * TRUE if the principals are the same; FALSE otherwise
 */
        #[no_mangle]
        #[c2rust::src_loc = "3682:1"]
        pub fn krb5_principal_compare_any_realm(context: krb5_context,
                                                princ1: krb5_const_principal,
                                                princ2: krb5_const_principal)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "4721:1"]
        pub fn krb5_free_keyblock_contents(context: krb5_context,
                                           key: *mut krb5_keyblock);
        #[no_mangle]
        #[c2rust::src_loc = "4743:1"]
        pub fn krb5_free_data(context: krb5_context, val: *mut krb5_data);
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:32"]
pub mod k5_int_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 1989,1990,1991,1992,1993,1994,1995,2000,2001,
 * 2003,2006,2007,2008,2009 by the Massachusetts Institute of Technology,
 * Cambridge, MA, USA.  All Rights Reserved.
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
 * This prototype for k5-int.h (Krb5 internals include file)
 * includes the user-visible definitions from krb5.h and then
 * includes other definitions that are not user-visible but are
 * required for compiling Kerberos internal routines.
 *
 * John Gilmore, Cygnus Support, Sat Jan 21 22:45:52 PST 1995
 */
    /* KRB5_GENERAL__ */
    /*
 * Begin "k5-config.h"
 */
    /*
 * Machine-type definitions: PC Clone 386 running Microloss Windows
 */
    /* From autoconf.h */
    /* HAVE_SYS_TYPES_H */
    /* HAVE_SYS_TYPES_H */
    /* KRB5_SYSTYPES__ */
    /* one day */
    /* one week */
    /* Thu Jan  1 00:00:00 2038 UTC */
    /*
 * Windows requires a different api interface to each function. Here
 * just define it as NULL.
 */
    /* #define KRB5_OLD_CRYPTO is done in krb5.h */
    /* KRB5_CONFIG__ */
    /*
 * End "k5-config.h"
 */
    /*
 * After loading the configuration definitions, load the Kerberos definitions.
 */
    /* Get mutex support; currently used only for the replay cache.  */
    /* Get error info support.  */
    /* Get string buffer support. */
    /* Define tracing macros. */
    /* Profile variables.  Constants are named KRB5_CONF_STRING, where STRING
 * matches the variable name.  Keep these alphabetized. */
    /* Cache configuration variables */
    /* Error codes used in KRB_ERROR protocol messages.
   Return values of library routines are based on a different error table
   (which allows non-ambiguous error codes between subsystems) */
    /* KDC errors */
    /* No error */
    /* Client's entry in DB expired */
    /* Server's entry in DB expired */
    /* Requested pvno not supported */
    /* C's key encrypted in old master */
    /* S's key encrypted in old master */
    /* Client not found in Kerberos DB */
    /* Server not found in Kerberos DB */
    /* Multiple entries in Kerberos DB */
    /* The C or S has a null key */
    /* Tkt ineligible for postdating */
    /* Requested starttime > endtime */
    /* KDC policy rejects request */
    /* KDC can't do requested opt. */
    /* No support for encryption type */
    /* No support for checksum type */
    /* No support for padata type */
    /* No support for transited type */
    /* C's creds have been revoked */
    /* S's creds have been revoked */
    /* TGT has been revoked */
    /* C not yet valid */
    /* S not yet valid */
    /* Password has expired */
    /* Preauthentication failed */
    /* Additional preauthentication */
                                           /* required */
    /* Requested server and */
                                           /* ticket don't match*/
    /* Server principal valid for */
                                           /*   user2user only */
    /* KDC policy rejected transited */
                                           /*   path */
    /* A service is not
                                            * available that is
                                            * required to process the
                                            * request */
    /* Application errors */
    /* Decrypt integrity check failed */
    /* Ticket expired */
    /* Ticket not yet valid */
    /* Request is a replay */
    /* The ticket isn't for us */
    /* Ticket/authenticator don't match */
    /* Clock skew too great */
    /* Incorrect net address */
    /* Protocol version mismatch */
    /* Invalid message type */
    /* Message stream modified */
    /* Message out of order */
    /* Key version is not available */
    /* Service key not available */
    /* Mutual authentication failed */
    /* Incorrect message direction */
    /* Alternative authentication */
                                        /* method required */
    /* Incorrect sequence numnber */
                                        /* in message */
    /* Inappropriate type of */
                                        /* checksum in message */
    /* Policy rejects transited path */
    /* Response too big for UDP, */
                                        /*   retry with TCP */
    /* other errors */
    /* Generic error (description */
                                        /* in e-text) */
    /* Field is too long for impl. */
    /* PKINIT server-reported errors */
    /* client cert not trusted */
    /* client signature verify failed */
    /* invalid Diffie-Hellman parameters */
    /* client cert not verifiable to */
                                                   /* trusted root cert */
    /* client cert had invalid signature */
    /* client cert was revoked */
    /* client cert revoked, reason unknown */
    /* mismatch between client cert and */
                                                   /* principal name */
    /* bad extended key use */
    /* bad digest algorithm in client cert */
    /* missing paChecksum in PA-PK-AS-REQ */
    /* bad digest algorithm in SignedData */
    /* The IAKERB proxy could
                                                      not find a KDC */
    /* The KDC did not respond
                                                      to the IAKERB proxy */
    /* RFC 6113 */
    /* RFC 6113 */
    /* err table base max offset for protocol err codes */
    /*
 * A null-terminated array of this structure is returned by the KDC as
 * the data part of the ETYPE_INFO preauth type.  It informs the
 * client which encryption types are supported.
 * The  same data structure is used by both etype-info and etype-info2
 * but s2kparams must be null when encoding etype-info.
 */
    /*
 *  This is essentially -1 without sign extension which can screw up
 *  comparisons on 64 bit machines. If the length is this value, then
 *  the salt data is not present. This is to distinguish between not
 *  being set and being of 0 length.
 */
    /* RFC 4537 */
    /* sam_type values -- informational only */
    /*  Enigma Logic */
    /*  Digital Pathways */
    /*  S/key where  KDC has key 0 */
    /*  Traditional S/Key */
    /*  Security Dynamics */
    /*  CRYPTOCard */
    /* XXX need to figure out who has which numbers assigned */
    /*  ActivCard decimal mode */
    /*  ActivCard hex mode */
    /*  Digital Pathways hex mode */
    /* experimental */
    /* testing */
    /* special */
    /* Array of checksums */
    /* information */
    /* KRB5_SAM_* values */
    /* informational */
    /* KRB5_SAM_* values */
    /* copied */
    /* krb5_enc_sam_response_enc */
    /*
 * Keep the pkinit definitions in a separate file so that the plugin
 * only has to include k5-int-pkinit.h rather than k5-int.h
 */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* Plain text of an encrypted PA-FX-COOKIE value produced by the KDC. */
    /* In PAC options, indicates Resource-Based Constrained Delegation support. */
    /* struct stat, stat() */
    /* MAXPATHLEN */
    /* prototypes for file-related
                                           syscalls; flags for open &
                                           friends */
    /* libos.spec */
    /* Internal structure of an opaque key identifier */
    /*
     * Cache of data private to the cipher implementation, which we
     * don't want to have to recompute for every operation.  This may
     * include key schedules, iteration counts, etc.
     *
     * The cipher implementation is responsible for setting this up
     * whenever needed, and the enc_provider key_cleanup method must
     * then be provided to dispose of it.
     */
    /* Write the SHA-256 hash of in (containing n elements) to out. */
    /* Convenience function: zap and free ptr if it is non-NULL. */
    /* Convenience function: zap and free zero-terminated str if it is non-NULL. */
    /* Convenience function: zap and free krb5_data pointer if it is non-NULL. */
    /*
 * End "los-proto.h"
 */
    /*
 * Flags for the os_flags field
 *
 * KRB5_OS_TOFFSET_VALID means that the time offset fields are valid.
 * The intention is that this facility to correct the system clocks so
 * that they reflect the "real" time, for systems where for some
 * reason we can't set the system clock.  Instead we calculate the
 * offset between the system time and real time, and store the offset
 * in the os context so that we can correct the system clock as necessary.
 *
 * KRB5_OS_TOFFSET_TIME means that the time offset fields should be
 * returned as the time by the krb5 time routines.  This should only
 * be used for testing purposes (obviously!)
 */
    /* lock mode flags */
    /*
 * Begin "preauth.h"
 *
 * (Originally written by Glen Machin at Sandia Labs.)
 */
/*
 * Sandia National Laboratories also makes no representations about the
 * suitability of the modifications, or additions to this software for
 * any purpose.  It is provided "as is" without express or implied warranty.
 */
    /* check logon hour restrictions */
    /* sign with usage 27 instead of 26 */
    /* padata from req_body is used*/
    /* Bits 0-15 are critical in FAST options (RFC 6113 section 7.3). */
    /*
 * AD-CAMMAC's other-verifiers field is a sequence of Verifier, which is an
 * extensible choice with only one selection, Verifier-MAC.  For the time being
 * we will represent this field directly as an array of krb5_verifier_mac.
 * That will have to change if other selections are added.
 */
    /* Does not return a copy; original padata sequence responsible for freeing*/
    /* Allocate a pa-data object with uninitialized contents of size len.  If len
 * is 0, set the contents field to NULL. */
    /* Free a single pa-data object. */
    /* Without copying, add single element *pa to *list, reallocating as necessary.
 * If *list is NULL, allocate a new list.  Set *pa to NULL on success. */
    /* Without copying, add a pa-data element of type pa_type to *list with the
 * contents in data.  Set *data to empty_data() on success. */
    /* Add an empty pa-data element of type pa_type to *list. */
    /* KRB5_PREAUTH__ */
    /*
 * End "preauth.h"
 */
    /* #include "krb5/wordsize.h" -- comes in through base-defs.h. */
    /* ** Plugin framework ***/
    /*
 * This framework can be used to create pluggable interfaces.  Not all existing
 * pluggable interface use this framework, but new ones should.  A new
 * pluggable interface entails:
 *
 * - An interface ID definition in the list of #defines below.
 *
 * - A name in the interface_names array in lib/krb5/krb/plugins.c.
 *
 * - An installed public header file in include/krb5.  The public header should
 *   include <krb5/plugin.h> and should declare a vtable structure for each
 *   supported major version of the interface.
 *
 * - A consumer API implementation, located within the code unit which makes
 *   use of the pluggable interface.  The consumer API should consist of:
 *
 *   . An interface-specific handle type which contains a vtable structure for
 *     the module (or a union of several such structures, if there are multiple
 *     supported major versions) and, optionally, resource data bound to the
 *     handle.
 *
 *   . An interface-specific loader function which creates a handle or list of
 *     handles.  A list of handles would be created if the interface is a
 *     one-to-many interface where the consumer wants to consult all available
 *     modules; a single handle would be created for an interface where the
 *     consumer wants to consult a specific module.  The loader function should
 *     use k5_plugin_load or k5_plugin_load_all to produce one or a list of
 *     vtable initializer functions, and should use those functions to fill in
 *     the vtable structure for the module (if necessary, trying each supported
 *     major version starting from the most recent).  The loader function can
 *     also bind resource data into the handle based on caller arguments, if
 *     appropriate.
 *
 *   . For each plugin method, a wrapper function which accepts a krb5_context,
 *     a plugin handle, and the method arguments.  Wrapper functions should
 *     invoke the method function contained in the handle's vtable.
 *
 * - Possibly, built-in implementations of the interface, also located within
 *   the code unit which makes use of the interface.  Built-in implementations
 *   must be registered with k5_plugin_register before the first call to
 *   k5_plugin_load or k5_plugin_load_all.
 *
 * A pluggable interface should have one or more currently supported major
 * versions, starting at 1.  Each major version should have a current minor
 * version, also starting at 1.  If new methods are added to a vtable, the
 * minor version should be incremented and the vtable stucture should document
 * where each minor vtable version ends.  If method signatures for a vtable are
 * changed, the major version should be incremented.
 *
 * Plugin module implementations (either built-in or dynamically loaded) should
 * define a function named <interfacename>_<modulename>_initvt, matching the
 * signature of krb5_plugin_initvt_fn as declared in include/krb5/plugin.h.
 * The initvt function should check the given maj_ver argument against its own
 * supported major versions, cast the vtable pointer to the appropriate
 * interface-specific vtable type, and fill in the vtable methods, stopping as
 * appropriate for the given min_ver.  Memory for the vtable structure is
 * allocated by the caller, not by the module.
 *
 * Dynamic plugin modules are registered with the framework through the
 * [plugins] section of the profile, as described in the admin documentation
 * and krb5.conf man page.
 */
    /* Holds krb5_context information about each pluggable interface. */
    /* A list of plugin interface IDs.  Make sure to increment
 * PLUGIN_NUM_INTERFACES when a new interface is added, and add an entry to the
 * interface_names table in lib/krb5/krb/plugin.c. */
    /* Retrieve the plugin module of type interface_id and name modname,
 * storing the result into module. */
    /* Retrieve all plugin modules of type interface_id, storing the result
 * into modules.  Free the result with k5_plugin_free_handles. */
    /* Release a module list allocated by k5_plugin_load_all. */
    /* Register a plugin module of type interface_id and name modname. */
    /*
 * Register a plugin module which is part of the krb5 tree but is built as a
 * dynamic plugin.  Look for the module in modsubdir relative to the
 * context->base_plugin_dir.
 */
    /* Destroy the module state within context; used by krb5_free_context. */
    /* private, in kdb5.h */
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
    #[inline]
    #[c2rust::src_loc = "656:1"]
    pub unsafe extern "C" fn zapfree(mut ptr: *mut libc::c_void,
                                     mut len: size_t) {
        if !ptr.is_null() { explicit_bzero(ptr, len); free(ptr); };
    }
    /* Allocate zeroed memory; set *code to 0 on success or ENOMEM on failure. */
    #[inline]
    #[c2rust::src_loc = "2296:1"]
    pub unsafe extern "C" fn k5calloc(mut nmemb: size_t, mut size: size_t,
                                      mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
        /* Allocate at least one byte since zero-byte allocs may return NULL. */
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
    /* Allocate zeroed memory; set *code to 0 on success or ENOMEM on failure. */
    #[inline]
    #[c2rust::src_loc = "2308:1"]
    pub unsafe extern "C" fn k5alloc(mut size: size_t,
                                     mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        return k5calloc(1 as libc::c_int as size_t, size, code);
    }
    /* Like k5memdup, but add a final null byte. */
    #[inline]
    #[c2rust::src_loc = "2326:1"]
    pub unsafe extern "C" fn k5memdup0(mut in_0: *const libc::c_void,
                                       mut len: size_t,
                                       mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void =
            k5alloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    code);
        if !ptr.is_null() && len > 0 as libc::c_int as libc::c_ulong {
            memcpy(ptr, in_0, len);
        }
        return ptr;
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_error_code};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::stddef_h::size_t;
    use super::string_h::{explicit_bzero, memcpy};
    use super::stdlib_h::{free, calloc};
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:32"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:32"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:32"]
pub mod profile_h {
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    /*@modifies internalState@*/
    /*@modifies internalState@*/
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:32"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-buf.h:32"]
pub mod k5_buf_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-buf.h - k5buf interface declarations */
/*
 * Copyright 2008 Massachusetts Institute of Technology.
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
 * The k5buf module is intended to allow multi-step string construction in a
 * fixed or dynamic buffer without the need to check for a failure at each step
 * (and without aborting on malloc failure).  If an allocation failure occurs
 * or the fixed buffer runs out of room, the buffer will be set to an error
 * state which can be detected with k5_buf_status.  Data in a buffer is
 * terminated with a zero byte so that it can be used as a C string.
 *
 * k5buf structures are usually stack-allocated.  Do not put k5buf structure
 * pointers into public APIs.  It is okay to reference the data and len fields
 * of a buffer (they will be NULL/0 if the buffer is in an error state), but do
 * not change them.
 */
    /* Buffer type values */
    #[c2rust::src_loc = "48:1"]
    pub type k5buftype = libc::c_uint;
    #[c2rust::src_loc = "48:59"]
    pub const K5BUF_DYNAMIC_ZAP: k5buftype = 3;
    #[c2rust::src_loc = "48:44"]
    pub const K5BUF_DYNAMIC: k5buftype = 2;
    #[c2rust::src_loc = "48:31"]
    pub const K5BUF_FIXED: k5buftype = 1;
    #[c2rust::src_loc = "48:18"]
    pub const K5BUF_ERROR: k5buftype = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:8"]
    pub struct k5buf {
        pub buftype: k5buftype,
        pub data: *mut libc::c_void,
        pub space: size_t,
        pub len: size_t,
    }
    use super::stddef_h::size_t;
    extern "C" {
        /* Initialize a k5buf using an internally allocated dynamic buffer. */
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn k5_buf_init_dynamic(buf: *mut k5buf);
        /* Add a C string to BUF. */
        #[no_mangle]
        #[c2rust::src_loc = "71:1"]
        pub fn k5_buf_add(buf: *mut k5buf, data: *const libc::c_char);
        /* Add sprintf-style formatted data to BUF. */
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn k5_buf_add_fmt(buf: *mut k5buf, fmt: *const libc::c_char,
                              _: ...);
        /* Return ENOMEM if buf is in an error state, 0 otherwise. */
        #[no_mangle]
        #[c2rust::src_loc = "100:1"]
        pub fn k5_buf_status(buf: *mut k5buf) -> libc::c_int;
        /*
 * Free the storage used in the dynamic buffer BUF.  The caller may choose to
 * take responsibility for freeing the data pointer instead of using this
 * function.  If BUF is a fixed buffer, an assertion failure will result.
 * Freeing a buffer in the error state, a buffer initialized with EMPTY_K5BUF,
 * or a zeroed k5buf structure is a no-op.
 */
        #[no_mangle]
        #[c2rust::src_loc = "109:1"]
        pub fn k5_buf_free(buf: *mut k5buf);
    }
    /* K5_BUF_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int-pkinit.h:32"]
pub mod k5_int_pkinit_h {
    /* AlgorithmIdentifier */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:16"]
    pub struct _krb5_algorithm_identifier {
        pub algorithm: krb5_data,
        pub parameters: krb5_data,
    }
    #[c2rust::src_loc = "49:1"]
    pub type krb5_algorithm_identifier = _krb5_algorithm_identifier;
    /* Optional */
    /* ExternalPrincipalIdentifier */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "70:16"]
    pub struct _krb5_external_principal_identifier {
        pub subjectName: krb5_data,
        pub issuerAndSerialNumber: krb5_data,
        pub subjectKeyIdentifier: krb5_data,
    }
    #[c2rust::src_loc = "70:1"]
    pub type krb5_external_principal_identifier
        =
        _krb5_external_principal_identifier;
    /* Optional */
    /* SP80056A OtherInfo, for pkinit algorithm agility */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct _krb5_sp80056a_other_info {
        pub algorithm_identifier: krb5_algorithm_identifier,
        pub party_u_info: krb5_principal,
        pub party_v_info: krb5_principal,
        pub supp_pub_info: krb5_data,
    }
    #[c2rust::src_loc = "117:1"]
    pub type krb5_sp80056a_other_info = _krb5_sp80056a_other_info;
    /* PkinitSuppPubInfo, for pkinit algorithm agility */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "125:16"]
    pub struct _krb5_pkinit_supp_pub_info {
        pub enctype: krb5_enctype,
        pub as_req: krb5_data,
        pub pk_as_rep: krb5_data,
    }
    #[c2rust::src_loc = "125:1"]
    pub type krb5_pkinit_supp_pub_info = _krb5_pkinit_supp_pub_info;
    use super::krb5_h::{krb5_data, krb5_principal, krb5_enctype,
                        krb5_error_code};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "162:1"]
        pub fn encode_krb5_sp80056a_other_info(_:
                                                   *const krb5_sp80056a_other_info,
                                               _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "166:1"]
        pub fn encode_krb5_pkinit_supp_pub_info(_:
                                                    *const krb5_pkinit_supp_pub_info,
                                                _: *mut *mut krb5_data)
         -> krb5_error_code;
    }
    /* _KRB5_INT_PKINIT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/preauth/pkinit/pkcs11.h:33"]
pub mod pkcs11_h {
    #[c2rust::src_loc = "184:1"]
    pub type CK_FLAGS = libc::c_ulong;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "186:8"]
    pub struct _CK_VERSION {
        pub major: libc::c_uchar,
        pub minor: libc::c_uchar,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "193:8"]
    pub struct _CK_INFO {
        pub cryptokiVersion: _CK_VERSION,
        pub manufacturerID: [libc::c_uchar; 32],
        pub flags: CK_FLAGS,
        pub libraryDescription: [libc::c_uchar; 32],
        pub libraryVersion: _CK_VERSION,
    }
    #[c2rust::src_loc = "203:1"]
    pub type CK_NOTIFICATION = libc::c_ulong;
    #[c2rust::src_loc = "208:1"]
    pub type CK_SLOT_ID = libc::c_ulong;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "211:8"]
    pub struct _CK_SLOT_INFO {
        pub slotDescription: [libc::c_uchar; 64],
        pub manufacturerID: [libc::c_uchar; 32],
        pub flags: CK_FLAGS,
        pub hardwareVersion: _CK_VERSION,
        pub firmwareVersion: _CK_VERSION,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "227:8"]
    pub struct _CK_TOKEN_INFO {
        pub label: [libc::c_uchar; 32],
        pub manufacturerID: [libc::c_uchar; 32],
        pub model: [libc::c_uchar; 16],
        pub serialNumber: [libc::c_uchar; 16],
        pub flags: CK_FLAGS,
        pub ulMaxSessionCount: libc::c_ulong,
        pub ulSessionCount: libc::c_ulong,
        pub ulMaxRwSessionCount: libc::c_ulong,
        pub ulRwSessionCount: libc::c_ulong,
        pub ulMaxPinLen: libc::c_ulong,
        pub ulMinPinLen: libc::c_ulong,
        pub ulTotalPublicMemory: libc::c_ulong,
        pub ulFreePublicMemory: libc::c_ulong,
        pub ulTotalPrivateMemory: libc::c_ulong,
        pub ulFreePrivateMemory: libc::c_ulong,
        pub hardwareVersion: _CK_VERSION,
        pub firmwareVersion: _CK_VERSION,
        pub utcTime: [libc::c_uchar; 16],
    }
    #[c2rust::src_loc = "273:1"]
    pub type CK_SESSION_HANDLE = libc::c_ulong;
    #[c2rust::src_loc = "278:1"]
    pub type CK_USER_TYPE = libc::c_ulong;
    #[c2rust::src_loc = "285:1"]
    pub type CK_STATE = libc::c_ulong;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "294:8"]
    pub struct _CK_SESSION_INFO {
        pub slotID: CK_SLOT_ID,
        pub state: CK_STATE,
        pub flags: CK_FLAGS,
        pub ulDeviceError: libc::c_ulong,
    }
    #[c2rust::src_loc = "306:1"]
    pub type CK_OBJECT_HANDLE = libc::c_ulong;
    #[c2rust::src_loc = "309:1"]
    pub type CK_OBJECT_CLASS = libc::c_ulong;
    #[c2rust::src_loc = "330:1"]
    pub type CK_KEY_TYPE = libc::c_ulong;
    #[c2rust::src_loc = "360:1"]
    pub type CK_CERTIFICATE_TYPE = libc::c_ulong;
    #[c2rust::src_loc = "368:1"]
    pub type CK_ATTRIBUTE_TYPE = libc::c_ulong;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "457:8"]
    pub struct _CK_ATTRIBUTE {
        pub type_0: CK_ATTRIBUTE_TYPE,
        pub pValue: *mut libc::c_void,
        pub ulValueLen: libc::c_ulong,
    }
    #[c2rust::src_loc = "473:1"]
    pub type CK_MECHANISM_TYPE = libc::c_ulong;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "676:8"]
    pub struct _CK_MECHANISM {
        pub mechanism: CK_MECHANISM_TYPE,
        pub pParameter: *mut libc::c_void,
        pub ulParameterLen: libc::c_ulong,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "684:8"]
    pub struct _CK_MECHANISM_INFO {
        pub ulMinKeySize: libc::c_ulong,
        pub ulMaxKeySize: libc::c_ulong,
        pub flags: CK_FLAGS,
    }
    #[c2rust::src_loc = "711:1"]
    pub type CK_RV = libc::c_ulong;
    #[c2rust::src_loc = "714:1"]
    pub type CK_NOTIFY
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE, _: CK_NOTIFICATION,
                                    _: *mut libc::c_void) -> CK_RV>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1001:8"]
    pub struct _CK_FUNCTION_LIST {
        pub version: _CK_VERSION,
        pub C_Initialize: CK_C_Initialize,
        pub C_Finalize: CK_C_Finalize,
        pub C_GetInfo: CK_C_GetInfo,
        pub C_GetFunctionList: CK_C_GetFunctionList,
        pub C_GetSlotList: CK_C_GetSlotList,
        pub C_GetSlotInfo: CK_C_GetSlotInfo,
        pub C_GetTokenInfo: CK_C_GetTokenInfo,
        pub C_GetMechanismList: CK_C_GetMechanismList,
        pub C_GetMechanismInfo: CK_C_GetMechanismInfo,
        pub C_InitToken: CK_C_InitToken,
        pub C_InitPIN: CK_C_InitPIN,
        pub C_SetPIN: CK_C_SetPIN,
        pub C_OpenSession: CK_C_OpenSession,
        pub C_CloseSession: CK_C_CloseSession,
        pub C_CloseAllSessions: CK_C_CloseAllSessions,
        pub C_GetSessionInfo: CK_C_GetSessionInfo,
        pub C_GetOperationState: CK_C_GetOperationState,
        pub C_SetOperationState: CK_C_SetOperationState,
        pub C_Login: CK_C_Login,
        pub C_Logout: CK_C_Logout,
        pub C_CreateObject: CK_C_CreateObject,
        pub C_CopyObject: CK_C_CopyObject,
        pub C_DestroyObject: CK_C_DestroyObject,
        pub C_GetObjectSize: CK_C_GetObjectSize,
        pub C_GetAttributeValue: CK_C_GetAttributeValue,
        pub C_SetAttributeValue: CK_C_SetAttributeValue,
        pub C_FindObjectsInit: CK_C_FindObjectsInit,
        pub C_FindObjects: CK_C_FindObjects,
        pub C_FindObjectsFinal: CK_C_FindObjectsFinal,
        pub C_EncryptInit: CK_C_EncryptInit,
        pub C_Encrypt: CK_C_Encrypt,
        pub C_EncryptUpdate: CK_C_EncryptUpdate,
        pub C_EncryptFinal: CK_C_EncryptFinal,
        pub C_DecryptInit: CK_C_DecryptInit,
        pub C_Decrypt: CK_C_Decrypt,
        pub C_DecryptUpdate: CK_C_DecryptUpdate,
        pub C_DecryptFinal: CK_C_DecryptFinal,
        pub C_DigestInit: CK_C_DigestInit,
        pub C_Digest: CK_C_Digest,
        pub C_DigestUpdate: CK_C_DigestUpdate,
        pub C_DigestKey: CK_C_DigestKey,
        pub C_DigestFinal: CK_C_DigestFinal,
        pub C_SignInit: CK_C_SignInit,
        pub C_Sign: CK_C_Sign,
        pub C_SignUpdate: CK_C_SignUpdate,
        pub C_SignFinal: CK_C_SignFinal,
        pub C_SignRecoverInit: CK_C_SignRecoverInit,
        pub C_SignRecover: CK_C_SignRecover,
        pub C_VerifyInit: CK_C_VerifyInit,
        pub C_Verify: CK_C_Verify,
        pub C_VerifyUpdate: CK_C_VerifyUpdate,
        pub C_VerifyFinal: CK_C_VerifyFinal,
        pub C_VerifyRecoverInit: CK_C_VerifyRecoverInit,
        pub C_VerifyRecover: CK_C_VerifyRecover,
        pub C_DigestEncryptUpdate: CK_C_DigestEncryptUpdate,
        pub C_DecryptDigestUpdate: CK_C_DecryptDigestUpdate,
        pub C_SignEncryptUpdate: CK_C_SignEncryptUpdate,
        pub C_DecryptVerifyUpdate: CK_C_DecryptVerifyUpdate,
        pub C_GenerateKey: CK_C_GenerateKey,
        pub C_GenerateKeyPair: CK_C_GenerateKeyPair,
        pub C_WrapKey: CK_C_WrapKey,
        pub C_UnwrapKey: CK_C_UnwrapKey,
        pub C_DeriveKey: CK_C_DeriveKey,
        pub C_SeedRandom: CK_C_SeedRandom,
        pub C_GenerateRandom: CK_C_GenerateRandom,
        pub C_GetFunctionStatus: CK_C_GetFunctionStatus,
        pub C_CancelFunction: CK_C_CancelFunction,
        pub C_WaitForSlotEvent: CK_C_WaitForSlotEvent,
    }
    #[c2rust::src_loc = "737:1"]
    pub type CK_C_WaitForSlotEvent
        =
        Option<unsafe extern "C" fn(_: CK_FLAGS, _: *mut CK_SLOT_ID,
                                    _: *mut libc::c_void) -> CK_RV>;
    #[c2rust::src_loc = "998:1"]
    pub type CK_C_CancelFunction
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "997:1"]
    pub type CK_C_GetFunctionStatus
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "992:1"]
    pub type CK_C_GenerateRandom
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "989:1"]
    pub type CK_C_SeedRandom
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "981:1"]
    pub type CK_C_DeriveKey
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_MECHANISM,
                                    _: CK_OBJECT_HANDLE,
                                    _: *mut _CK_ATTRIBUTE, _: libc::c_ulong,
                                    _: *mut CK_OBJECT_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "972:1"]
    pub type CK_C_UnwrapKey
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_MECHANISM,
                                    _: CK_OBJECT_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut _CK_ATTRIBUTE, _: libc::c_ulong,
                                    _: *mut CK_OBJECT_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "965:1"]
    pub type CK_C_WrapKey
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_MECHANISM,
                                    _: CK_OBJECT_HANDLE, _: CK_OBJECT_HANDLE,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "956:1"]
    pub type CK_C_GenerateKeyPair
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_MECHANISM,
                                    _: *mut _CK_ATTRIBUTE, _: libc::c_ulong,
                                    _: *mut _CK_ATTRIBUTE, _: libc::c_ulong,
                                    _: *mut CK_OBJECT_HANDLE,
                                    _: *mut CK_OBJECT_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "950:1"]
    pub type CK_C_GenerateKey
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_MECHANISM,
                                    _: *mut _CK_ATTRIBUTE, _: libc::c_ulong,
                                    _: *mut CK_OBJECT_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "943:1"]
    pub type CK_C_DecryptVerifyUpdate
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "938:1"]
    pub type CK_C_SignEncryptUpdate
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "932:1"]
    pub type CK_C_DecryptDigestUpdate
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "927:1"]
    pub type CK_C_DigestEncryptUpdate
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "920:1"]
    pub type CK_C_VerifyRecover
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "916:1"]
    pub type CK_C_VerifyRecoverInit
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_MECHANISM,
                                    _: CK_OBJECT_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "912:1"]
    pub type CK_C_VerifyFinal
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "909:1"]
    pub type CK_C_VerifyUpdate
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "904:1"]
    pub type CK_C_Verify
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "900:1"]
    pub type CK_C_VerifyInit
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_MECHANISM,
                                    _: CK_OBJECT_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "894:1"]
    pub type CK_C_SignRecover
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "890:1"]
    pub type CK_C_SignRecoverInit
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_MECHANISM,
                                    _: CK_OBJECT_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "886:1"]
    pub type CK_C_SignFinal
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "883:1"]
    pub type CK_C_SignUpdate
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "878:1"]
    pub type CK_C_Sign
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "874:1"]
    pub type CK_C_SignInit
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_MECHANISM,
                                    _: CK_OBJECT_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "869:1"]
    pub type CK_C_DigestFinal
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "867:1"]
    pub type CK_C_DigestKey
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE, _: CK_OBJECT_HANDLE)
                   -> CK_RV>;
    #[c2rust::src_loc = "864:1"]
    pub type CK_C_DigestUpdate
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "859:1"]
    pub type CK_C_Digest
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "856:1"]
    pub type CK_C_DigestInit
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_MECHANISM) -> CK_RV>;
    #[c2rust::src_loc = "851:1"]
    pub type CK_C_DecryptFinal
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "846:1"]
    pub type CK_C_DecryptUpdate
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "841:1"]
    pub type CK_C_Decrypt
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "837:1"]
    pub type CK_C_DecryptInit
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_MECHANISM,
                                    _: CK_OBJECT_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "832:1"]
    pub type CK_C_EncryptFinal
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "827:1"]
    pub type CK_C_EncryptUpdate
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "822:1"]
    pub type CK_C_Encrypt
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "818:1"]
    pub type CK_C_EncryptInit
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_MECHANISM,
                                    _: CK_OBJECT_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "815:1"]
    pub type CK_C_FindObjectsFinal
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "810:1"]
    pub type CK_C_FindObjects
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut CK_OBJECT_HANDLE,
                                    _: libc::c_ulong, _: *mut libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "806:1"]
    pub type CK_C_FindObjectsInit
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_ATTRIBUTE, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "801:1"]
    pub type CK_C_SetAttributeValue
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE, _: CK_OBJECT_HANDLE,
                                    _: *mut _CK_ATTRIBUTE, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "796:1"]
    pub type CK_C_GetAttributeValue
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE, _: CK_OBJECT_HANDLE,
                                    _: *mut _CK_ATTRIBUTE, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "792:1"]
    pub type CK_C_GetObjectSize
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE, _: CK_OBJECT_HANDLE,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "789:1"]
    pub type CK_C_DestroyObject
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE, _: CK_OBJECT_HANDLE)
                   -> CK_RV>;
    #[c2rust::src_loc = "785:1"]
    pub type CK_C_CopyObject
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE, _: CK_OBJECT_HANDLE,
                                    _: *mut _CK_ATTRIBUTE, _: libc::c_ulong,
                                    _: *mut CK_OBJECT_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "781:1"]
    pub type CK_C_CreateObject
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_ATTRIBUTE, _: libc::c_ulong,
                                    _: *mut CK_OBJECT_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "779:1"]
    pub type CK_C_Logout
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "776:1"]
    pub type CK_C_Login
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE, _: CK_USER_TYPE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "770:1"]
    pub type CK_C_SetOperationState
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: CK_OBJECT_HANDLE, _: CK_OBJECT_HANDLE)
                   -> CK_RV>;
    #[c2rust::src_loc = "766:1"]
    pub type CK_C_GetOperationState
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "763:1"]
    pub type CK_C_GetSessionInfo
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_SESSION_INFO) -> CK_RV>;
    #[c2rust::src_loc = "762:1"]
    pub type CK_C_CloseAllSessions
        =
        Option<unsafe extern "C" fn(_: CK_SLOT_ID) -> CK_RV>;
    #[c2rust::src_loc = "761:1"]
    pub type CK_C_CloseSession
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "757:1"]
    pub type CK_C_OpenSession
        =
        Option<unsafe extern "C" fn(_: CK_SLOT_ID, _: CK_FLAGS,
                                    _: *mut libc::c_void, _: CK_NOTIFY,
                                    _: *mut CK_SESSION_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "752:1"]
    pub type CK_C_SetPIN
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "749:1"]
    pub type CK_C_InitPIN
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "746:1"]
    pub type CK_C_InitToken
        =
        Option<unsafe extern "C" fn(_: CK_SLOT_ID, _: *mut libc::c_uchar,
                                    _: libc::c_ulong, _: *mut libc::c_uchar)
                   -> CK_RV>;
    #[c2rust::src_loc = "743:1"]
    pub type CK_C_GetMechanismInfo
        =
        Option<unsafe extern "C" fn(_: CK_SLOT_ID, _: CK_MECHANISM_TYPE,
                                    _: *mut _CK_MECHANISM_INFO) -> CK_RV>;
    #[c2rust::src_loc = "739:1"]
    pub type CK_C_GetMechanismList
        =
        Option<unsafe extern "C" fn(_: CK_SLOT_ID, _: *mut CK_MECHANISM_TYPE,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "735:1"]
    pub type CK_C_GetTokenInfo
        =
        Option<unsafe extern "C" fn(_: CK_SLOT_ID, _: *mut _CK_TOKEN_INFO)
                   -> CK_RV>;
    #[c2rust::src_loc = "733:1"]
    pub type CK_C_GetSlotInfo
        =
        Option<unsafe extern "C" fn(_: CK_SLOT_ID, _: *mut _CK_SLOT_INFO)
                   -> CK_RV>;
    #[c2rust::src_loc = "730:1"]
    pub type CK_C_GetSlotList
        =
        Option<unsafe extern "C" fn(_: libc::c_uchar, _: *mut CK_SLOT_ID,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "727:1"]
    pub type CK_C_GetFunctionList
        =
        Option<unsafe extern "C" fn(_: *mut *mut _CK_FUNCTION_LIST) -> CK_RV>;
    #[c2rust::src_loc = "726:1"]
    pub type CK_C_GetInfo
        =
        Option<unsafe extern "C" fn(_: *mut _CK_INFO) -> CK_RV>;
    #[c2rust::src_loc = "725:1"]
    pub type CK_C_Finalize
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> CK_RV>;
    #[c2rust::src_loc = "724:1"]
    pub type CK_C_Initialize
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> CK_RV>;
    /* Compatibility layer.  */
    /* For NULL.  */
    #[c2rust::src_loc = "1193:1"]
    pub type CK_BYTE = libc::c_uchar;
    #[c2rust::src_loc = "1197:1"]
    pub type CK_ULONG = libc::c_ulong;
    #[c2rust::src_loc = "1199:1"]
    pub type CK_BYTE_PTR = *mut CK_BYTE;
    #[c2rust::src_loc = "1202:1"]
    pub type CK_ULONG_PTR = *mut CK_ULONG;
    #[c2rust::src_loc = "1222:1"]
    pub type CK_SLOT_ID_PTR = *mut CK_SLOT_ID;
    #[c2rust::src_loc = "1227:1"]
    pub type CK_TOKEN_INFO = _CK_TOKEN_INFO;
    #[c2rust::src_loc = "1239:1"]
    pub type CK_ATTRIBUTE = _CK_ATTRIBUTE;
    #[c2rust::src_loc = "1247:1"]
    pub type CK_MECHANISM = _CK_MECHANISM;
    #[c2rust::src_loc = "1254:1"]
    pub type CK_FUNCTION_LIST_PTR = *mut _CK_FUNCTION_LIST;
    #[c2rust::src_loc = "1255:1"]
    pub type CK_FUNCTION_LIST_PTR_PTR = *mut *mut _CK_FUNCTION_LIST;
    /* PKCS11_H */
    /* System dependencies.  */
    /* CRYPTOKI_COMPAT */
    /* Delete the helper macros defined at the top of the file.  */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/preauth/pkinit/pkinit_crypto_openssl.h:33"]
pub mod pkinit_crypto_openssl_h {
    /*
 * COPYRIGHT (C) 2006,2007
 * THE REGENTS OF THE UNIVERSITY OF MICHIGAN
 * ALL RIGHTS RESERVED
 *
 * Permission is granted to use, copy, create derivative works
 * and redistribute this software and such derivative works
 * for any purpose, so long as the name of The University of
 * Michigan is not used in any advertising or publicity
 * pertaining to the use of distribution of this software
 * without specific, written prior authorization.  If the
 * above copyright notice or any other identification of the
 * University of Michigan is included in any copy of any
 * portion of this software, then the disclaimer below must
 * also be included.
 *
 * THIS SOFTWARE IS PROVIDED AS IS, WITHOUT REPRESENTATION
 * FROM THE UNIVERSITY OF MICHIGAN AS TO ITS FITNESS FOR ANY
 * PURPOSE, AND WITHOUT WARRANTY BY THE UNIVERSITY OF
 * MICHIGAN OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING
 * WITHOUT LIMITATION THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE
 * REGENTS OF THE UNIVERSITY OF MICHIGAN SHALL NOT BE LIABLE
 * FOR ANY DAMAGES, INCLUDING SPECIAL, INDIRECT, INCIDENTAL, OR
 * CONSEQUENTIAL DAMAGES, WITH RESPECT TO ANY CLAIM ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OF THE SOFTWARE, EVEN
 * IF IT HAS BEEN OR IS HEREAFTER ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGES.
 */
    /* available user certs */
    /* identity name for user cert */
    /* cert to use out of available certs*/
    /* available user keys if in filesystem */
    /* available trusted ca certs */
    /* available intermediate ca certs */
    /* available crls */
    /* These are crypto-specific */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "98:8"]
    pub struct _pkinit_plg_crypto_context {
        pub dh_1024: *mut DH,
        pub dh_2048: *mut DH,
        pub dh_4096: *mut DH,
        pub id_pkinit_authData: *mut ASN1_OBJECT,
        pub id_pkinit_DHKeyData: *mut ASN1_OBJECT,
        pub id_pkinit_rkeyData: *mut ASN1_OBJECT,
        pub id_pkinit_san: *mut ASN1_OBJECT,
        pub id_ms_san_upn: *mut ASN1_OBJECT,
        pub id_pkinit_KPClientAuth: *mut ASN1_OBJECT,
        pub id_pkinit_KPKdc: *mut ASN1_OBJECT,
        pub id_ms_kp_sc_logon: *mut ASN1_OBJECT,
        pub id_kp_serverAuth: *mut ASN1_OBJECT,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "113:8"]
    pub struct _pkinit_req_crypto_context {
        pub received_cert: *mut X509,
        pub dh: *mut DH,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "69:8"]
    pub struct _pkinit_identity_crypto_context {
        pub creds: [pkinit_cred_info; 21],
        pub my_certs: *mut stack_st_X509,
        pub identity: *mut libc::c_char,
        pub cert_index: libc::c_int,
        pub my_key: *mut EVP_PKEY,
        pub trustedCAs: *mut stack_st_X509,
        pub intermediateCAs: *mut stack_st_X509,
        pub revoked: *mut stack_st_X509_CRL,
        pub pkcs11_method: libc::c_int,
        pub prompter: krb5_prompter_fct,
        pub prompter_data: *mut libc::c_void,
        pub p11_module_name: *mut libc::c_char,
        pub slotid: CK_SLOT_ID,
        pub token_label: *mut libc::c_char,
        pub cert_label: *mut libc::c_char,
        pub p11_module: *mut libc::c_void,
        pub session: CK_SESSION_HANDLE,
        pub p11: CK_FUNCTION_LIST_PTR,
        pub cert_id: *mut uint8_t,
        pub cert_id_len: size_t,
        pub mech: CK_MECHANISM_TYPE,
        pub defer_id_prompt: krb5_boolean,
        pub deferred_ids: *mut pkinit_deferred_id,
    }
    #[c2rust::src_loc = "67:1"]
    pub type pkinit_cred_info = *mut _pkinit_cred_info;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "58:8"]
    pub struct _pkinit_cred_info {
        pub name: *mut libc::c_char,
        pub cert: *mut X509,
        pub key: *mut EVP_PKEY,
        pub cert_id: CK_BYTE_PTR,
        pub cert_id_len: libc::c_int,
    }
    use super::ossl_typ_h::{DH, ASN1_OBJECT, X509, EVP_PKEY};
    use super::x509_h::{stack_st_X509, stack_st_X509_CRL};
    use super::krb5_h::{krb5_prompter_fct, krb5_boolean};
    use super::pkcs11_h::{CK_SLOT_ID, CK_SESSION_HANDLE, CK_FUNCTION_LIST_PTR,
                          CK_MECHANISM_TYPE, CK_BYTE_PTR};
    use super::stdint_uintn_h::uint8_t;
    use super::stddef_h::size_t;
    use super::pkinit_h::pkinit_deferred_id;
    /* _PKINIT_CRYPTO_OPENSSL_H */
}
#[c2rust::header_src = "/usr/include/openssl/ossl_typ.h:33"]
pub mod ossl_typ_h {
    #[c2rust::src_loc = "60:1"]
    pub type ASN1_OBJECT = asn1_object_st;
    #[c2rust::src_loc = "104:1"]
    pub type DH = dh_st;
    #[c2rust::src_loc = "120:1"]
    pub type X509 = x509_st;
    #[c2rust::src_loc = "93:1"]
    pub type EVP_PKEY = evp_pkey_st;
    #[c2rust::src_loc = "55:1"]
    pub type ASN1_STRING = asn1_string_st;
    #[c2rust::src_loc = "54:1"]
    pub type ASN1_UTF8STRING = asn1_string_st;
    #[c2rust::src_loc = "53:1"]
    pub type ASN1_VISIBLESTRING = asn1_string_st;
    #[c2rust::src_loc = "52:1"]
    pub type ASN1_GENERALIZEDTIME = asn1_string_st;
    #[c2rust::src_loc = "50:1"]
    pub type ASN1_UTCTIME = asn1_string_st;
    #[c2rust::src_loc = "48:1"]
    pub type ASN1_UNIVERSALSTRING = asn1_string_st;
    #[c2rust::src_loc = "49:1"]
    pub type ASN1_BMPSTRING = asn1_string_st;
    #[c2rust::src_loc = "47:1"]
    pub type ASN1_GENERALSTRING = asn1_string_st;
    #[c2rust::src_loc = "46:1"]
    pub type ASN1_IA5STRING = asn1_string_st;
    #[c2rust::src_loc = "45:1"]
    pub type ASN1_T61STRING = asn1_string_st;
    #[c2rust::src_loc = "44:1"]
    pub type ASN1_PRINTABLESTRING = asn1_string_st;
    #[c2rust::src_loc = "43:1"]
    pub type ASN1_OCTET_STRING = asn1_string_st;
    #[c2rust::src_loc = "42:1"]
    pub type ASN1_BIT_STRING = asn1_string_st;
    #[c2rust::src_loc = "41:1"]
    pub type ASN1_ENUMERATED = asn1_string_st;
    #[c2rust::src_loc = "40:1"]
    pub type ASN1_INTEGER = asn1_string_st;
    #[c2rust::src_loc = "56:1"]
    pub type ASN1_BOOLEAN = libc::c_int;
    #[c2rust::src_loc = "80:1"]
    pub type BIGNUM = bignum_st;
    #[c2rust::src_loc = "141:1"]
    pub type OPENSSL_INIT_SETTINGS = ossl_init_settings_st;
    #[c2rust::src_loc = "89:1"]
    pub type EVP_CIPHER = evp_cipher_st;
    #[c2rust::src_loc = "121:1"]
    pub type X509_ALGOR = X509_algor_st;
    #[c2rust::src_loc = "128:1"]
    pub type X509_STORE_CTX = x509_store_ctx_st;
    #[c2rust::src_loc = "125:1"]
    pub type X509_NAME = X509_name_st;
    #[c2rust::src_loc = "92:1"]
    pub type EVP_MD_CTX = evp_md_ctx_st;
    #[c2rust::src_loc = "79:1"]
    pub type BIO = bio_st;
    #[c2rust::src_loc = "127:1"]
    pub type X509_STORE = x509_store_st;
    #[c2rust::src_loc = "122:1"]
    pub type X509_CRL = X509_crl_st;
    #[c2rust::src_loc = "90:1"]
    pub type EVP_CIPHER_CTX = evp_cipher_ctx_st;
    #[c2rust::src_loc = "146:1"]
    pub type ENGINE = engine_st;
    #[c2rust::src_loc = "91:1"]
    pub type EVP_MD = evp_md_st;
    #[c2rust::src_loc = "62:1"]
    pub type ASN1_ITEM = ASN1_ITEM_st;
    use super::asn1_h::asn1_string_st;
    use super::x509_h::X509_algor_st;
    use super::asn1t_h::ASN1_ITEM_st;
    extern "C" {
        #[c2rust::src_loc = "60:16"]
        pub type asn1_object_st;
        #[c2rust::src_loc = "104:16"]
        pub type dh_st;
        #[c2rust::src_loc = "120:16"]
        pub type x509_st;
        #[c2rust::src_loc = "93:16"]
        pub type evp_pkey_st;
        #[c2rust::src_loc = "80:16"]
        pub type bignum_st;
        #[c2rust::src_loc = "141:16"]
        pub type ossl_init_settings_st;
        #[c2rust::src_loc = "89:16"]
        pub type evp_cipher_st;
        #[c2rust::src_loc = "128:16"]
        pub type x509_store_ctx_st;
        #[c2rust::src_loc = "125:16"]
        pub type X509_name_st;
        #[c2rust::src_loc = "92:16"]
        pub type evp_md_ctx_st;
        #[c2rust::src_loc = "79:16"]
        pub type bio_st;
        #[c2rust::src_loc = "127:16"]
        pub type x509_store_st;
        #[c2rust::src_loc = "122:16"]
        pub type X509_crl_st;
        #[c2rust::src_loc = "90:16"]
        pub type evp_cipher_ctx_st;
        #[c2rust::src_loc = "146:16"]
        pub type engine_st;
        #[c2rust::src_loc = "91:16"]
        pub type evp_md_st;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/preauth/pkinit/pkinit.h:33"]
pub mod pkinit_h {
    /*
 * notes about crypto contexts:
 *
 * the basic idea is that there are crypto contexts that live at
 * both the plugin level and request level. the identity context (that
 * keeps info about your own certs and such) is separate because
 * it is needed at different levels for the kdc and and the client.
 * (the kdc's identity is at the plugin level, the client's identity
 * information could change per-request.)
 * the identity context is meant to have the entity's cert,
 * a list of trusted and intermediate cas, a list of crls, and any
 * pkcs11 information.  the req context is meant to have the
 * received certificate and the DH related information. the plugin
 * context is meant to have global crypto information, i.e., OIDs
 * and constant DH parameter information.
 */
    /*
 * plugin crypto context should keep plugin common information,
 * eg., OIDs, known DHparams
 */
    #[c2rust::src_loc = "128:1"]
    pub type pkinit_plg_crypto_context = *mut _pkinit_plg_crypto_context;
    /*
 * request crypto context should keep reqyest common information,
 * eg., received credentials, DH parameters of this request
 */
    #[c2rust::src_loc = "134:1"]
    pub type pkinit_req_crypto_context = *mut _pkinit_req_crypto_context;
    /*
 * identity context should keep information about credentials
 * for the request, eg., my credentials, trusted ca certs,
 * intermediate ca certs, crls, pkcs11 info
 */
    /*
 * this structure keeps information about the config options
 */
    /* require EKU checking (default is true) */
    /* accept secondary EKU (default is false) */
    /* allow UPN-SAN instead of pkinit-SAN */
    /* selects DH or RSA based pkinit */
    /* require CRL for a CA (default is false) */
    /* require freshness token (default is false) */
    /* disable freshness token on client for testing */
    /* minimum DH modulus size allowed */
    /*
 * this structure keeps options used for a given request
 */
    /* initial request DH modulus size (default=1024) */
    /*
 * information about identity from config file or command line
 */
    /*
 * Client's plugin context
 */
    /*
 * Client's per-request context
 */
    /*
 * KDC's (per-realm) plugin context
 */
    /*
 * KDC's per-request context
 */
    /*
 * Functions in pkinit_lib.c
 */
    /*
 * Functions in pkinit_identity.c
 */
    /* IN */
    /* IN */
    /* IN */
    /* IN */
    /* IN/OUT */
    /* IN/OUT */
    /* IN/OUT */
    /* IN (optional) */
    /* IN */
    /* IN */
    /* IN */
    /* IN */
    /* IN/OUT */
    /* IN/OUT */
    /* IN/OUT */
    /* IN */
    /* IN (optional) */
    /*
 * Client's list of identities for which it needs PINs or passwords
 */
    #[c2rust::src_loc = "315:1"]
    pub type pkinit_deferred_id = *mut _pkinit_deferred_id;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "309:8"]
    pub struct _pkinit_deferred_id {
        pub magic: libc::c_int,
        pub identity: *mut libc::c_char,
        pub ck_flags: libc::c_ulong,
        pub password: *mut libc::c_char,
    }
    #[c2rust::src_loc = "141:1"]
    pub type pkinit_identity_crypto_context
        =
        *mut _pkinit_identity_crypto_context;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "146:16"]
    pub struct _pkinit_plg_opts {
        pub require_eku: libc::c_int,
        pub accept_secondary_eku: libc::c_int,
        pub allow_upn: libc::c_int,
        pub dh_or_rsa: libc::c_int,
        pub require_crl_checking: libc::c_int,
        pub require_freshness: libc::c_int,
        pub disable_freshness: libc::c_int,
        pub dh_min_bits: libc::c_int,
    }
    #[c2rust::src_loc = "146:1"]
    pub type pkinit_plg_opts = _pkinit_plg_opts;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "175:16"]
    pub struct _pkinit_identity_opts {
        pub identity: *mut libc::c_char,
        pub identity_alt: *mut *mut libc::c_char,
        pub anchors: *mut *mut libc::c_char,
        pub intermediates: *mut *mut libc::c_char,
        pub crls: *mut *mut libc::c_char,
        pub idtype: libc::c_int,
        pub cert_filename: *mut libc::c_char,
        pub key_filename: *mut libc::c_char,
        pub p11_module_name: *mut libc::c_char,
        pub slotid: CK_SLOT_ID,
        pub token_label: *mut libc::c_char,
        pub cert_id_string: *mut libc::c_char,
        pub cert_label: *mut libc::c_char,
    }
    #[c2rust::src_loc = "175:1"]
    pub type pkinit_identity_opts = _pkinit_identity_opts;
    /*
 * COPYRIGHT (C) 2006,2007
 * THE REGENTS OF THE UNIVERSITY OF MICHIGAN
 * ALL RIGHTS RESERVED
 *
 * Permission is granted to use, copy, create derivative works
 * and redistribute this software and such derivative works
 * for any purpose, so long as the name of The University of
 * Michigan is not used in any advertising or publicity
 * pertaining to the use of distribution of this software
 * without specific, written prior authorization.  If the
 * above copyright notice or any other identification of the
 * University of Michigan is included in any copy of any
 * portion of this software, then the disclaimer below must
 * also be included.
 *
 * THIS SOFTWARE IS PROVIDED AS IS, WITHOUT REPRESENTATION
 * FROM THE UNIVERSITY OF MICHIGAN AS TO ITS FITNESS FOR ANY
 * PURPOSE, AND WITHOUT WARRANTY BY THE UNIVERSITY OF
 * MICHIGAN OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING
 * WITHOUT LIMITATION THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE
 * REGENTS OF THE UNIVERSITY OF MICHIGAN SHALL NOT BE LIABLE
 * FOR ANY DAMAGES, INCLUDING SPECIAL, INDIRECT, INCIDENTAL, OR
 * CONSEQUENTIAL DAMAGES, WITH RESPECT TO ANY CLAIM ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OF THE SOFTWARE, EVEN
 * IF IT HAS BEEN OR IS HEREAFTER ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGES.
 */
    /* pkinit_kdc_ocsp has been removed */
    /* Make pkiDebug(fmt,...) print, or not.  */
    /* Still evaluates for side effects.  */
    #[inline]
    #[c2rust::src_loc = "88:1"]
    pub unsafe extern "C" fn pkiDebug(mut fmt: *const libc::c_char,
                                      mut args: ...) {
    }
    use super::pkinit_crypto_openssl_h::{_pkinit_plg_crypto_context,
                                         _pkinit_req_crypto_context,
                                         _pkinit_identity_crypto_context};
    use super::pkcs11_h::CK_SLOT_ID;
    use super::krb5_h::{krb5_data, krb5_magic, krb5_error_code};
    use super::k5_int_pkinit_h::krb5_external_principal_identifier;
    extern "C" {
        /* This is better if the compiler doesn't inline variadic functions
   well, but gcc will warn about "left-hand operand of comma
   expression has no effect".  Still evaluates for side effects.  */
/* #define pkiDebug	(void) */
        /* Solaris compiler doesn't grok __FUNCTION__
 * hack for now.  Fix all the uses eventually. */
        /* Macros to deal with converting between various data types... */
        #[no_mangle]
        #[c2rust::src_loc = "105:24"]
        pub static dh_oid: krb5_data;
        #[no_mangle]
        #[c2rust::src_loc = "317:1"]
        pub fn pkinit_set_deferred_id(identities:
                                          *mut *mut pkinit_deferred_id,
                                      identity: *const libc::c_char,
                                      ck_flags: libc::c_ulong,
                                      password: *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "320:1"]
        pub fn pkinit_find_deferred_id(identities: *mut pkinit_deferred_id,
                                       identity: *const libc::c_char)
         -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "322:1"]
        pub fn pkinit_get_deferred_id_flags(identities:
                                                *mut pkinit_deferred_id,
                                            identity: *const libc::c_char)
         -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "324:1"]
        pub fn pkinit_free_deferred_ids(identities: *mut pkinit_deferred_id);
        #[no_mangle]
        #[c2rust::src_loc = "339:1"]
        pub fn free_krb5_external_principal_identifier(in_0:
                                                           *mut *mut *mut krb5_external_principal_identifier);
        #[no_mangle]
        #[c2rust::src_loc = "344:1"]
        pub fn pkinit_copy_krb5_data(dst: *mut krb5_data,
                                     src: *const krb5_data)
         -> krb5_error_code;
        /*
 * debugging functions
 */
        #[no_mangle]
        #[c2rust::src_loc = "380:1"]
        pub fn print_buffer(_: *const libc::c_uchar, _: libc::c_uint);
    }
    /* _PKINIT_H */
    /*
 * Now get crypto function declarations
 */
}
#[c2rust::header_src = "/usr/include/openssl/x509.h:33"]
pub mod x509_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "59:8"]
    pub struct X509_algor_st {
        pub algorithm: *mut ASN1_OBJECT,
        pub parameter: *mut ASN1_TYPE,
    }
    #[c2rust::src_loc = "245:1"]
    pub type X509_INFO = X509_info_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "245:16"]
    pub struct X509_info_st {
        pub x509: *mut X509,
        pub crl: *mut X509_CRL,
        pub x_pkey: *mut X509_PKEY,
        pub enc_cipher: EVP_CIPHER_INFO,
        pub enc_len: libc::c_int,
        pub enc_data: *mut libc::c_char,
    }
    #[c2rust::src_loc = "230:1"]
    pub type X509_PKEY = private_key_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "230:16"]
    pub struct private_key_st {
        pub version: libc::c_int,
        pub enc_algor: *mut X509_ALGOR,
        pub enc_pkey: *mut ASN1_OCTET_STRING,
        pub dec_pkey: *mut EVP_PKEY,
        pub key_length: libc::c_int,
        pub key_data: *mut libc::c_char,
        pub key_free: libc::c_int,
        pub cipher: EVP_CIPHER_INFO,
    }
    #[c2rust::src_loc = "81:1"]
    pub type X509_EXTENSION = X509_extension_st;
    #[c2rust::src_loc = "228:1"]
    pub type sk_X509_CRL_freefunc
        =
        Option<unsafe extern "C" fn(_: *mut X509_CRL) -> ()>;
    #[c2rust::src_loc = "99:1"]
    pub type sk_X509_freefunc
        =
        Option<unsafe extern "C" fn(_: *mut X509) -> ()>;
    #[c2rust::src_loc = "254:1"]
    pub type sk_X509_INFO_freefunc
        =
        Option<unsafe extern "C" fn(_: *mut X509_INFO) -> ()>;
    #[c2rust::src_loc = "77:1"]
    pub type sk_X509_NAME_freefunc
        =
        Option<unsafe extern "C" fn(_: *mut X509_NAME) -> ()>;
    #[inline]
    #[c2rust::src_loc = "77:1"]
    pub unsafe extern "C" fn sk_X509_NAME_pop_free(mut sk:
                                                       *mut stack_st_X509_NAME,
                                                   mut freefunc:
                                                       sk_X509_NAME_freefunc) {
        OPENSSL_sk_pop_free(sk as *mut OPENSSL_STACK,
                            ::std::mem::transmute::<sk_X509_NAME_freefunc,
                                                    OPENSSL_sk_freefunc>(freefunc));
    }
    #[inline]
    #[c2rust::src_loc = "77:1"]
    pub unsafe extern "C" fn sk_X509_NAME_push(mut sk:
                                                   *mut stack_st_X509_NAME,
                                               mut ptr: *mut X509_NAME)
     -> libc::c_int {
        return OPENSSL_sk_push(sk as *mut OPENSSL_STACK,
                               ptr as *const libc::c_void);
    }
    #[inline]
    #[c2rust::src_loc = "77:1"]
    pub unsafe extern "C" fn sk_X509_NAME_new_null()
     -> *mut stack_st_X509_NAME {
        return OPENSSL_sk_new_null() as *mut stack_st_X509_NAME;
    }
    #[inline]
    #[c2rust::src_loc = "228:1"]
    pub unsafe extern "C" fn sk_X509_CRL_pop_free(mut sk:
                                                      *mut stack_st_X509_CRL,
                                                  mut freefunc:
                                                      sk_X509_CRL_freefunc) {
        OPENSSL_sk_pop_free(sk as *mut OPENSSL_STACK,
                            ::std::mem::transmute::<sk_X509_CRL_freefunc,
                                                    OPENSSL_sk_freefunc>(freefunc));
    }
    #[inline]
    #[c2rust::src_loc = "99:1"]
    pub unsafe extern "C" fn sk_X509_pop_free(mut sk: *mut stack_st_X509,
                                              mut freefunc:
                                                  sk_X509_freefunc) {
        OPENSSL_sk_pop_free(sk as *mut OPENSSL_STACK,
                            ::std::mem::transmute::<sk_X509_freefunc,
                                                    OPENSSL_sk_freefunc>(freefunc));
    }
    #[inline]
    #[c2rust::src_loc = "99:1"]
    pub unsafe extern "C" fn sk_X509_value(mut sk: *const stack_st_X509,
                                           mut idx: libc::c_int)
     -> *mut X509 {
        return OPENSSL_sk_value(sk as *const OPENSSL_STACK, idx) as *mut X509;
    }
    #[inline]
    #[c2rust::src_loc = "99:1"]
    pub unsafe extern "C" fn sk_X509_push(mut sk: *mut stack_st_X509,
                                          mut ptr: *mut X509) -> libc::c_int {
        return OPENSSL_sk_push(sk as *mut OPENSSL_STACK,
                               ptr as *const libc::c_void);
    }
    #[inline]
    #[c2rust::src_loc = "99:1"]
    pub unsafe extern "C" fn sk_X509_num(mut sk: *const stack_st_X509)
     -> libc::c_int {
        return OPENSSL_sk_num(sk as *const OPENSSL_STACK);
    }
    #[inline]
    #[c2rust::src_loc = "99:1"]
    pub unsafe extern "C" fn sk_X509_new_null() -> *mut stack_st_X509 {
        return OPENSSL_sk_new_null() as *mut stack_st_X509;
    }
    #[inline]
    #[c2rust::src_loc = "228:1"]
    pub unsafe extern "C" fn sk_X509_CRL_free(mut sk:
                                                  *mut stack_st_X509_CRL) {
        OPENSSL_sk_free(sk as *mut OPENSSL_STACK);
    }
    #[inline]
    #[c2rust::src_loc = "99:1"]
    pub unsafe extern "C" fn sk_X509_free(mut sk: *mut stack_st_X509) {
        OPENSSL_sk_free(sk as *mut OPENSSL_STACK);
    }
    #[inline]
    #[c2rust::src_loc = "228:1"]
    pub unsafe extern "C" fn sk_X509_CRL_value(mut sk:
                                                   *const stack_st_X509_CRL,
                                               mut idx: libc::c_int)
     -> *mut X509_CRL {
        return OPENSSL_sk_value(sk as *const OPENSSL_STACK, idx) as
                   *mut X509_CRL;
    }
    #[inline]
    #[c2rust::src_loc = "228:1"]
    pub unsafe extern "C" fn sk_X509_CRL_push(mut sk: *mut stack_st_X509_CRL,
                                              mut ptr: *mut X509_CRL)
     -> libc::c_int {
        return OPENSSL_sk_push(sk as *mut OPENSSL_STACK,
                               ptr as *const libc::c_void);
    }
    #[inline]
    #[c2rust::src_loc = "228:1"]
    pub unsafe extern "C" fn sk_X509_CRL_num(mut sk: *const stack_st_X509_CRL)
     -> libc::c_int {
        return OPENSSL_sk_num(sk as *const OPENSSL_STACK);
    }
    #[inline]
    #[c2rust::src_loc = "228:1"]
    pub unsafe extern "C" fn sk_X509_CRL_new_null()
     -> *mut stack_st_X509_CRL {
        return OPENSSL_sk_new_null() as *mut stack_st_X509_CRL;
    }
    #[inline]
    #[c2rust::src_loc = "254:1"]
    pub unsafe extern "C" fn sk_X509_INFO_pop_free(mut sk:
                                                       *mut stack_st_X509_INFO,
                                                   mut freefunc:
                                                       sk_X509_INFO_freefunc) {
        OPENSSL_sk_pop_free(sk as *mut OPENSSL_STACK,
                            ::std::mem::transmute::<sk_X509_INFO_freefunc,
                                                    OPENSSL_sk_freefunc>(freefunc));
    }
    #[inline]
    #[c2rust::src_loc = "254:1"]
    pub unsafe extern "C" fn sk_X509_INFO_value(mut sk:
                                                    *const stack_st_X509_INFO,
                                                mut idx: libc::c_int)
     -> *mut X509_INFO {
        return OPENSSL_sk_value(sk as *const OPENSSL_STACK, idx) as
                   *mut X509_INFO;
    }
    #[inline]
    #[c2rust::src_loc = "254:1"]
    pub unsafe extern "C" fn sk_X509_INFO_num(mut sk:
                                                  *const stack_st_X509_INFO)
     -> libc::c_int {
        return OPENSSL_sk_num(sk as *const OPENSSL_STACK);
    }
    use super::ossl_typ_h::{ASN1_OBJECT, X509, X509_CRL, X509_ALGOR,
                            ASN1_OCTET_STRING, EVP_PKEY, X509_NAME,
                            ASN1_INTEGER, BIO, X509_STORE_CTX};
    use super::asn1_h::ASN1_TYPE;
    use super::evp_h::EVP_CIPHER_INFO;
    use super::stack_h::{OPENSSL_sk_pop_free, OPENSSL_STACK,
                         OPENSSL_sk_freefunc, OPENSSL_sk_push,
                         OPENSSL_sk_new_null, OPENSSL_sk_value,
                         OPENSSL_sk_num, OPENSSL_sk_free};
    extern "C" {
        #[c2rust::src_loc = "228:1"]
        pub type stack_st_X509_CRL;
        #[c2rust::src_loc = "99:1"]
        pub type stack_st_X509;
        #[c2rust::src_loc = "254:1"]
        pub type stack_st_X509_INFO;
        #[c2rust::src_loc = "77:1"]
        pub type stack_st_X509_NAME;
        #[c2rust::src_loc = "89:1"]
        pub type stack_st_X509_ATTRIBUTE;
        #[c2rust::src_loc = "81:16"]
        pub type X509_extension_st;
        #[no_mangle]
        #[c2rust::src_loc = "551:1"]
        pub fn X509_free(a: *mut X509);
        #[no_mangle]
        #[c2rust::src_loc = "502:1"]
        pub fn X509_ALGOR_free(a: *mut X509_ALGOR);
        #[no_mangle]
        #[c2rust::src_loc = "502:1"]
        pub fn i2d_X509_ALGOR(a: *mut X509_ALGOR,
                              out: *mut *mut libc::c_uchar) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "475:1"]
        pub fn X509_ALGOR_set0(alg: *mut X509_ALGOR, aobj: *mut ASN1_OBJECT,
                               ptype: libc::c_int, pval: *mut libc::c_void)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "502:24"]
        pub fn X509_ALGOR_new() -> *mut X509_ALGOR;
        #[no_mangle]
        #[c2rust::src_loc = "547:1"]
        pub fn X509_NAME_set(xn: *mut *mut X509_NAME, name: *mut X509_NAME)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "468:1"]
        pub fn X509_dup(x509: *mut X509) -> *mut X509;
        #[no_mangle]
        #[c2rust::src_loc = "348:1"]
        pub fn X509_verify_cert_error_string(n: libc::c_long)
         -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "545:1"]
        pub fn i2d_X509_NAME(a: *mut X509_NAME, out: *mut *mut libc::c_uchar)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "545:1"]
        pub fn X509_NAME_free(a: *mut X509_NAME);
        #[no_mangle]
        #[c2rust::src_loc = "551:24"]
        pub fn d2i_X509(a: *mut *mut X509, in_0: *mut *const libc::c_uchar,
                        len: libc::c_long) -> *mut X509;
        #[no_mangle]
        #[c2rust::src_loc = "471:1"]
        pub fn X509_CRL_dup(crl: *mut X509_CRL) -> *mut X509_CRL;
        #[no_mangle]
        #[c2rust::src_loc = "545:24"]
        pub fn d2i_X509_NAME(a: *mut *mut X509_NAME,
                             in_0: *mut *const libc::c_uchar,
                             len: libc::c_long) -> *mut X509_NAME;
        #[no_mangle]
        #[c2rust::src_loc = "551:1"]
        pub fn i2d_X509(a: *mut X509, out: *mut *mut libc::c_uchar)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "593:1"]
        pub fn X509_CRL_free(a: *mut X509_CRL);
        #[no_mangle]
        #[c2rust::src_loc = "637:1"]
        pub fn X509_get_serialNumber(x: *mut X509) -> *mut ASN1_INTEGER;
        #[no_mangle]
        #[c2rust::src_loc = "640:1"]
        pub fn X509_get_issuer_name(a: *const X509) -> *mut X509_NAME;
        #[no_mangle]
        #[c2rust::src_loc = "642:1"]
        pub fn X509_get_subject_name(a: *const X509) -> *mut X509_NAME;
        #[no_mangle]
        #[c2rust::src_loc = "609:1"]
        pub fn X509_NAME_oneline(a: *const X509_NAME, buf: *mut libc::c_char,
                                 size: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "861:1"]
        pub fn X509_get_ext_by_NID(x: *const X509, nid: libc::c_int,
                                   lastpos: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "749:1"]
        pub fn X509_check_private_key(x509: *const X509,
                                      pkey: *const EVP_PKEY) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "864:1"]
        pub fn X509_get_ext(x: *const X509, loc: libc::c_int)
         -> *mut X509_EXTENSION;
        #[no_mangle]
        #[c2rust::src_loc = "789:1"]
        pub fn X509_NAME_print_ex(out: *mut BIO, nm: *const X509_NAME,
                                  indent: libc::c_int, flags: libc::c_ulong)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "608:1"]
        pub fn X509_INFO_free(a: *mut X509_INFO);
        #[no_mangle]
        #[c2rust::src_loc = "775:1"]
        pub fn X509_CRL_cmp(a: *const X509_CRL, b: *const X509_CRL)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "770:1"]
        pub fn X509_cmp(a: *const X509, b: *const X509) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "771:1"]
        pub fn X509_NAME_cmp(a: *const X509_NAME, b: *const X509_NAME)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "867:1"]
        pub fn X509_get_ext_d2i(x: *const X509, nid: libc::c_int,
                                crit: *mut libc::c_int, idx: *mut libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "972:1"]
        pub fn X509_verify_cert(ctx: *mut X509_STORE_CTX) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/preauth/pkinit/pkinit_crypto.h:33"]
pub mod pkinit_crypto_h {
    #[c2rust::src_loc = "47:1"]
    pub type cms_msg_types = libc::c_uint;
    #[c2rust::src_loc = "50:5"]
    pub const CMS_ENVEL_SERVER: cms_msg_types = 2;
    #[c2rust::src_loc = "49:5"]
    pub const CMS_SIGN_SERVER: cms_msg_types = 1;
    #[c2rust::src_loc = "48:5"]
    pub const CMS_SIGN_CLIENT: cms_msg_types = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "94:16"]
    pub struct _pkinit_cert_matching_data {
        pub subject_dn: *mut libc::c_char,
        pub issuer_dn: *mut libc::c_char,
        pub ku_bits: libc::c_uint,
        pub eku_bits: libc::c_uint,
        pub sans: *mut krb5_principal,
        pub upns: *mut *mut libc::c_char,
    }
    #[c2rust::src_loc = "94:1"]
    pub type pkinit_cert_matching_data = _pkinit_cert_matching_data;
    use super::krb5_h::{krb5_principal, krb5_octet};
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "624:22"]
        pub static krb5_pkinit_sha512_oid_len: size_t;
        #[no_mangle]
        #[c2rust::src_loc = "623:25"]
        pub static krb5_pkinit_sha512_oid: [krb5_octet; 0];
        #[no_mangle]
        #[c2rust::src_loc = "622:21"]
        pub static krb5_pkinit_sha256_oid_len: size_t;
        #[no_mangle]
        #[c2rust::src_loc = "621:25"]
        pub static krb5_pkinit_sha256_oid: [krb5_octet; 0];
        #[no_mangle]
        #[c2rust::src_loc = "620:21"]
        pub static krb5_pkinit_sha1_oid_len: size_t;
        #[no_mangle]
        #[c2rust::src_loc = "619:25"]
        pub static krb5_pkinit_sha1_oid: [krb5_octet; 0];
    }
    /* _PKINIT_CRYPTO_H */
}
#[c2rust::header_src = "/usr/include/openssl/asn1.h:33"]
pub mod asn1_h {
    #[c2rust::src_loc = "444:1"]
    pub type ASN1_TYPE = asn1_type_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "444:16"]
    pub struct asn1_type_st {
        pub type_0: libc::c_int,
        pub value: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "446:5"]
    pub union C2RustUnnamed {
        pub ptr: *mut libc::c_char,
        pub boolean: ASN1_BOOLEAN,
        pub asn1_string: *mut ASN1_STRING,
        pub object: *mut ASN1_OBJECT,
        pub integer: *mut ASN1_INTEGER,
        pub enumerated: *mut ASN1_ENUMERATED,
        pub bit_string: *mut ASN1_BIT_STRING,
        pub octet_string: *mut ASN1_OCTET_STRING,
        pub printablestring: *mut ASN1_PRINTABLESTRING,
        pub t61string: *mut ASN1_T61STRING,
        pub ia5string: *mut ASN1_IA5STRING,
        pub generalstring: *mut ASN1_GENERALSTRING,
        pub bmpstring: *mut ASN1_BMPSTRING,
        pub universalstring: *mut ASN1_UNIVERSALSTRING,
        pub utctime: *mut ASN1_UTCTIME,
        pub generalizedtime: *mut ASN1_GENERALIZEDTIME,
        pub visiblestring: *mut ASN1_VISIBLESTRING,
        pub utf8string: *mut ASN1_UTF8STRING,
        pub set: *mut ASN1_STRING,
        pub sequence: *mut ASN1_STRING,
        pub asn1_value: *mut ASN1_VALUE,
    }
    #[c2rust::src_loc = "213:1"]
    pub type ASN1_VALUE = ASN1_VALUE_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "146:8"]
    pub struct asn1_string_st {
        pub length: libc::c_int,
        pub type_0: libc::c_int,
        pub data: *mut libc::c_uchar,
        pub flags: libc::c_long,
    }
    #[c2rust::src_loc = "210:1"]
    pub type ASN1_TEMPLATE = ASN1_TEMPLATE_st;
    #[c2rust::src_loc = "318:1"]
    pub type ASN1_ITEM_EXP = ASN1_ITEM;
    #[inline]
    #[c2rust::src_loc = "536:1"]
    pub unsafe extern "C" fn sk_ASN1_OBJECT_num(mut sk:
                                                    *const stack_st_ASN1_OBJECT)
     -> libc::c_int {
        return OPENSSL_sk_num(sk as *const OPENSSL_STACK);
    }
    #[inline]
    #[c2rust::src_loc = "536:1"]
    pub unsafe extern "C" fn sk_ASN1_OBJECT_value(mut sk:
                                                      *const stack_st_ASN1_OBJECT,
                                                  mut idx: libc::c_int)
     -> *mut ASN1_OBJECT {
        return OPENSSL_sk_value(sk as *const OPENSSL_STACK, idx) as
                   *mut ASN1_OBJECT;
    }
    use super::ossl_typ_h::{ASN1_BOOLEAN, ASN1_STRING, ASN1_OBJECT,
                            ASN1_INTEGER, ASN1_ENUMERATED, ASN1_BIT_STRING,
                            ASN1_OCTET_STRING, ASN1_PRINTABLESTRING,
                            ASN1_T61STRING, ASN1_IA5STRING,
                            ASN1_GENERALSTRING, ASN1_BMPSTRING,
                            ASN1_UNIVERSALSTRING, ASN1_UTCTIME,
                            ASN1_GENERALIZEDTIME, ASN1_VISIBLESTRING,
                            ASN1_UTF8STRING, ASN1_ITEM, BIGNUM};
    use super::asn1t_h::ASN1_TEMPLATE_st;
    use super::stack_h::{OPENSSL_sk_num, OPENSSL_STACK, OPENSSL_sk_value};
    extern "C" {
        #[c2rust::src_loc = "213:16"]
        pub type ASN1_VALUE_st;
        #[c2rust::src_loc = "119:1"]
        pub type stack_st_X509_ALGOR;
        #[c2rust::src_loc = "536:1"]
        pub type stack_st_ASN1_OBJECT;
        #[no_mangle]
        #[c2rust::src_loc = "529:1"]
        pub fn ASN1_OBJECT_free(a: *mut ASN1_OBJECT);
        #[no_mangle]
        #[c2rust::src_loc = "596:1"]
        pub fn ASN1_OCTET_STRING_free(a: *mut ASN1_OCTET_STRING);
        #[no_mangle]
        #[c2rust::src_loc = "518:30"]
        pub fn ASN1_TYPE_new() -> *mut ASN1_TYPE;
        #[no_mangle]
        #[c2rust::src_loc = "600:1"]
        pub fn ASN1_OCTET_STRING_set(str: *mut ASN1_OCTET_STRING,
                                     data: *const libc::c_uchar,
                                     len: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "596:24"]
        pub fn ASN1_OCTET_STRING_new() -> *mut ASN1_OCTET_STRING;
        #[no_mangle]
        #[c2rust::src_loc = "549:1"]
        pub fn ASN1_STRING_set(str: *mut ASN1_STRING,
                               data: *const libc::c_void, len: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "596:1"]
        pub fn i2d_ASN1_OCTET_STRING(a: *mut ASN1_OCTET_STRING,
                                     out: *mut *mut libc::c_uchar)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "682:1"]
        pub fn ASN1_put_object(pp: *mut *mut libc::c_uchar,
                               constructed: libc::c_int, length: libc::c_int,
                               tag: libc::c_int, xclass: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "685:1"]
        pub fn ASN1_object_size(constructed: libc::c_int, length: libc::c_int,
                                tag: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "810:1"]
        pub fn ASN1_item_i2d(val: *mut ASN1_VALUE,
                             out: *mut *mut libc::c_uchar,
                             it: *const ASN1_ITEM) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "518:1"]
        pub fn ASN1_TYPE_free(a: *mut ASN1_TYPE);
        #[no_mangle]
        #[c2rust::src_loc = "573:1"]
        pub fn ASN1_INTEGER_dup(x: *const ASN1_INTEGER) -> *mut ASN1_INTEGER;
        #[no_mangle]
        #[c2rust::src_loc = "570:1"]
        pub fn ASN1_INTEGER_free(a: *mut ASN1_INTEGER);
        #[no_mangle]
        #[c2rust::src_loc = "657:1"]
        pub fn ASN1_INTEGER_set(a: *mut ASN1_INTEGER, v: libc::c_long)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "555:1"]
        pub fn ASN1_STRING_get0_data(x: *const ASN1_STRING)
         -> *const libc::c_uchar;
        #[no_mangle]
        #[c2rust::src_loc = "551:1"]
        pub fn ASN1_STRING_length(x: *const ASN1_STRING) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "678:1"]
        pub fn ASN1_get_object(pp: *mut *const libc::c_uchar,
                               plength: *mut libc::c_long,
                               ptag: *mut libc::c_int,
                               pclass: *mut libc::c_int, omax: libc::c_long)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "530:1"]
        pub fn i2d_ASN1_OBJECT(a: *const ASN1_OBJECT,
                               pp: *mut *mut libc::c_uchar) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "557:1"]
        pub fn ASN1_BIT_STRING_free(a: *mut ASN1_BIT_STRING);
        #[no_mangle]
        #[c2rust::src_loc = "570:1"]
        pub fn i2d_ASN1_INTEGER(a: *mut ASN1_INTEGER,
                                out: *mut *mut libc::c_uchar) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "659:1"]
        pub fn BN_to_ASN1_INTEGER(bn: *const BIGNUM, ai: *mut ASN1_INTEGER)
         -> *mut ASN1_INTEGER;
        #[no_mangle]
        #[c2rust::src_loc = "660:1"]
        pub fn ASN1_INTEGER_to_BN(ai: *const ASN1_INTEGER, bn: *mut BIGNUM)
         -> *mut BIGNUM;
        #[no_mangle]
        #[c2rust::src_loc = "570:24"]
        pub fn d2i_ASN1_INTEGER(a: *mut *mut ASN1_INTEGER,
                                in_0: *mut *const libc::c_uchar,
                                len: libc::c_long) -> *mut ASN1_INTEGER;
        #[no_mangle]
        #[c2rust::src_loc = "557:1"]
        pub static ASN1_BIT_STRING_it: ASN1_ITEM;
        #[no_mangle]
        #[c2rust::src_loc = "807:1"]
        pub fn ASN1_item_free(val: *mut ASN1_VALUE, it: *const ASN1_ITEM);
        #[no_mangle]
        #[c2rust::src_loc = "808:1"]
        pub fn ASN1_item_d2i(val: *mut *mut ASN1_VALUE,
                             in_0: *mut *const libc::c_uchar,
                             len: libc::c_long, it: *const ASN1_ITEM)
         -> *mut ASN1_VALUE;
        #[no_mangle]
        #[c2rust::src_loc = "596:24"]
        pub fn d2i_ASN1_OCTET_STRING(a: *mut *mut ASN1_OCTET_STRING,
                                     in_0: *mut *const libc::c_uchar,
                                     len: libc::c_long)
         -> *mut ASN1_OCTET_STRING;
        #[no_mangle]
        #[c2rust::src_loc = "658:1"]
        pub fn ASN1_INTEGER_get(a: *const ASN1_INTEGER) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "574:1"]
        pub fn ASN1_INTEGER_cmp(x: *const ASN1_INTEGER,
                                y: *const ASN1_INTEGER) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/openssl/pkcs7.h:33"]
pub mod pkcs7_h {
    #[c2rust::src_loc = "109:1"]
    pub type PKCS7 = pkcs7_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "109:16"]
    pub struct pkcs7_st {
        pub asn1: *mut libc::c_uchar,
        pub length: libc::c_long,
        pub state: libc::c_int,
        pub detached: libc::c_int,
        pub type_0: *mut ASN1_OBJECT,
        pub d: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "127:5"]
    pub union C2RustUnnamed_0 {
        pub ptr: *mut libc::c_char,
        pub data: *mut ASN1_OCTET_STRING,
        pub sign: *mut PKCS7_SIGNED,
        pub enveloped: *mut PKCS7_ENVELOPE,
        pub signed_and_enveloped: *mut PKCS7_SIGN_ENVELOPE,
        pub digest: *mut PKCS7_DIGEST,
        pub encrypted: *mut PKCS7_ENCRYPT,
        pub other: *mut ASN1_TYPE,
    }
    #[c2rust::src_loc = "104:1"]
    pub type PKCS7_ENCRYPT = pkcs7_encrypted_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "104:16"]
    pub struct pkcs7_encrypted_st {
        pub version: *mut ASN1_INTEGER,
        pub enc_data: *mut PKCS7_ENC_CONTENT,
    }
    #[c2rust::src_loc = "74:1"]
    pub type PKCS7_ENC_CONTENT = pkcs7_enc_content_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "74:16"]
    pub struct pkcs7_enc_content_st {
        pub content_type: *mut ASN1_OBJECT,
        pub algorithm: *mut X509_ALGOR,
        pub enc_data: *mut ASN1_OCTET_STRING,
        pub cipher: *const EVP_CIPHER,
    }
    #[c2rust::src_loc = "97:1"]
    pub type PKCS7_DIGEST = pkcs7_digest_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "97:16"]
    pub struct pkcs7_digest_st {
        pub version: *mut ASN1_INTEGER,
        pub md: *mut X509_ALGOR,
        pub contents: *mut pkcs7_st,
        pub digest: *mut ASN1_OCTET_STRING,
    }
    #[c2rust::src_loc = "87:1"]
    pub type PKCS7_SIGN_ENVELOPE = pkcs7_signedandenveloped_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "87:16"]
    pub struct pkcs7_signedandenveloped_st {
        pub version: *mut ASN1_INTEGER,
        pub md_algs: *mut stack_st_X509_ALGOR,
        pub cert: *mut stack_st_X509,
        pub crl: *mut stack_st_X509_CRL,
        pub signer_info: *mut stack_st_PKCS7_SIGNER_INFO,
        pub enc_data: *mut PKCS7_ENC_CONTENT,
        pub recipientinfo: *mut stack_st_PKCS7_RECIP_INFO,
    }
    #[c2rust::src_loc = "81:1"]
    pub type PKCS7_ENVELOPE = pkcs7_enveloped_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "81:16"]
    pub struct pkcs7_enveloped_st {
        pub version: *mut ASN1_INTEGER,
        pub recipientinfo: *mut stack_st_PKCS7_RECIP_INFO,
        pub enc_data: *mut PKCS7_ENC_CONTENT,
    }
    #[c2rust::src_loc = "61:1"]
    pub type PKCS7_SIGNED = pkcs7_signed_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "61:16"]
    pub struct pkcs7_signed_st {
        pub version: *mut ASN1_INTEGER,
        pub md_algs: *mut stack_st_X509_ALGOR,
        pub cert: *mut stack_st_X509,
        pub crl: *mut stack_st_X509_CRL,
        pub signer_info: *mut stack_st_PKCS7_SIGNER_INFO,
        pub contents: *mut pkcs7_st,
    }
    #[c2rust::src_loc = "32:1"]
    pub type PKCS7_ISSUER_AND_SERIAL = pkcs7_issuer_and_serial_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "32:16"]
    pub struct pkcs7_issuer_and_serial_st {
        pub issuer: *mut X509_NAME,
        pub serial: *mut ASN1_INTEGER,
    }
    #[c2rust::src_loc = "37:1"]
    pub type PKCS7_SIGNER_INFO = pkcs7_signer_info_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:16"]
    pub struct pkcs7_signer_info_st {
        pub version: *mut ASN1_INTEGER,
        pub issuer_and_serial: *mut PKCS7_ISSUER_AND_SERIAL,
        pub digest_alg: *mut X509_ALGOR,
        pub auth_attr: *mut stack_st_X509_ATTRIBUTE,
        pub digest_enc_alg: *mut X509_ALGOR,
        pub enc_digest: *mut ASN1_OCTET_STRING,
        pub unauth_attr: *mut stack_st_X509_ATTRIBUTE,
        pub pkey: *mut EVP_PKEY,
    }
    #[c2rust::src_loc = "51:1"]
    pub type PKCS7_RECIP_INFO = pkcs7_recip_info_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "51:16"]
    pub struct pkcs7_recip_info_st {
        pub version: *mut ASN1_INTEGER,
        pub issuer_and_serial: *mut PKCS7_ISSUER_AND_SERIAL,
        pub key_enc_algor: *mut X509_ALGOR,
        pub enc_key: *mut ASN1_OCTET_STRING,
        pub cert: *mut X509,
    }
    #[inline]
    #[c2rust::src_loc = "59:1"]
    pub unsafe extern "C" fn sk_PKCS7_RECIP_INFO_value(mut sk:
                                                           *const stack_st_PKCS7_RECIP_INFO,
                                                       mut idx: libc::c_int)
     -> *mut PKCS7_RECIP_INFO {
        return OPENSSL_sk_value(sk as *const OPENSSL_STACK, idx) as
                   *mut PKCS7_RECIP_INFO;
    }
    #[inline]
    #[c2rust::src_loc = "59:1"]
    pub unsafe extern "C" fn sk_PKCS7_RECIP_INFO_num(mut sk:
                                                         *const stack_st_PKCS7_RECIP_INFO)
     -> libc::c_int {
        return OPENSSL_sk_num(sk as *const OPENSSL_STACK);
    }
    use super::ossl_typ_h::{ASN1_OBJECT, ASN1_OCTET_STRING, ASN1_INTEGER,
                            X509_ALGOR, EVP_CIPHER, X509_NAME, EVP_PKEY, X509,
                            ASN1_ITEM, BIO};
    use super::asn1_h::{ASN1_TYPE, stack_st_X509_ALGOR, ASN1_TEMPLATE};
    use super::x509_h::{stack_st_X509, stack_st_X509_CRL,
                        stack_st_X509_ATTRIBUTE};
    use super::stack_h::{OPENSSL_sk_value, OPENSSL_STACK, OPENSSL_sk_num};
    extern "C" {
        #[c2rust::src_loc = "59:1"]
        pub type stack_st_PKCS7_RECIP_INFO;
        #[c2rust::src_loc = "49:1"]
        pub type stack_st_PKCS7_SIGNER_INFO;
        #[no_mangle]
        #[c2rust::src_loc = "227:1"]
        pub fn PKCS7_free(a: *mut PKCS7);
        #[no_mangle]
        #[c2rust::src_loc = "227:1"]
        pub fn i2d_PKCS7(a: *mut PKCS7, out: *mut *mut libc::c_uchar)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "227:24"]
        pub fn PKCS7_new() -> *mut PKCS7;
        #[no_mangle]
        #[c2rust::src_loc = "243:1"]
        pub fn PKCS7_add_signer(p7: *mut PKCS7, p7i: *mut PKCS7_SIGNER_INFO)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "229:1"]
        pub static PKCS7_ATTR_SIGN_it: ASN1_ITEM;
        #[no_mangle]
        #[c2rust::src_loc = "273:1"]
        pub fn PKCS7_add_signed_attribute(p7si: *mut PKCS7_SIGNER_INFO,
                                          nid: libc::c_int,
                                          type_0: libc::c_int,
                                          data: *mut libc::c_void)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "219:24"]
        pub fn PKCS7_SIGNER_INFO_new() -> *mut PKCS7_SIGNER_INFO;
        #[no_mangle]
        #[c2rust::src_loc = "221:24"]
        pub fn PKCS7_SIGNED_new() -> *mut PKCS7_SIGNED;
        #[no_mangle]
        #[c2rust::src_loc = "204:1"]
        pub fn PKCS7_ISSUER_AND_SERIAL_free(a: *mut PKCS7_ISSUER_AND_SERIAL);
        #[no_mangle]
        #[c2rust::src_loc = "204:1"]
        pub fn i2d_PKCS7_ISSUER_AND_SERIAL(a: *mut PKCS7_ISSUER_AND_SERIAL,
                                           out: *mut *mut libc::c_uchar)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "204:24"]
        pub fn PKCS7_ISSUER_AND_SERIAL_new() -> *mut PKCS7_ISSUER_AND_SERIAL;
        #[no_mangle]
        #[c2rust::src_loc = "296:1"]
        pub fn PKCS7_encrypt(certs: *mut stack_st_X509, in_0: *mut BIO,
                             cipher: *const EVP_CIPHER, flags: libc::c_int)
         -> *mut PKCS7;
        #[no_mangle]
        #[c2rust::src_loc = "227:24"]
        pub fn d2i_PKCS7(a: *mut *mut PKCS7, in_0: *mut *const libc::c_uchar,
                         len: libc::c_long) -> *mut PKCS7;
        #[no_mangle]
        #[c2rust::src_loc = "204:24"]
        pub fn d2i_PKCS7_ISSUER_AND_SERIAL(a:
                                               *mut *mut PKCS7_ISSUER_AND_SERIAL,
                                           in_0: *mut *const libc::c_uchar,
                                           len: libc::c_long)
         -> *mut PKCS7_ISSUER_AND_SERIAL;
    }
}
#[c2rust::header_src = "/usr/include/openssl/cms.h:153"]
pub mod cms_h {
    #[c2rust::src_loc = "24:1"]
    pub type CMS_SignerInfo = CMS_SignerInfo_st;
    #[c2rust::src_loc = "23:1"]
    pub type CMS_ContentInfo = CMS_ContentInfo_st;
    #[inline]
    #[c2rust::src_loc = "33:1"]
    pub unsafe extern "C" fn sk_CMS_SignerInfo_value(mut sk:
                                                         *const stack_st_CMS_SignerInfo,
                                                     mut idx: libc::c_int)
     -> *mut CMS_SignerInfo {
        return OPENSSL_sk_value(sk as *const OPENSSL_STACK, idx) as
                   *mut CMS_SignerInfo;
    }
    use super::stack_h::{OPENSSL_sk_value, OPENSSL_STACK};
    use super::ossl_typ_h::{ASN1_OBJECT, ASN1_OCTET_STRING, X509_STORE, BIO,
                            EVP_PKEY, X509, X509_ALGOR};
    use super::x509_h::{stack_st_X509, stack_st_X509_CRL};
    extern "C" {
        #[c2rust::src_loc = "24:16"]
        pub type CMS_SignerInfo_st;
        #[c2rust::src_loc = "33:1"]
        pub type stack_st_CMS_SignerInfo;
        #[c2rust::src_loc = "23:16"]
        pub type CMS_ContentInfo_st;
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn CMS_ContentInfo_free(a: *mut CMS_ContentInfo);
        #[no_mangle]
        #[c2rust::src_loc = "37:24"]
        pub fn d2i_CMS_ContentInfo(a: *mut *mut CMS_ContentInfo,
                                   in_0: *mut *const libc::c_uchar,
                                   len: libc::c_long) -> *mut CMS_ContentInfo;
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn CMS_get0_type(cms: *const CMS_ContentInfo)
         -> *const ASN1_OBJECT;
        #[no_mangle]
        #[c2rust::src_loc = "82:1"]
        pub fn CMS_get0_content(cms: *mut CMS_ContentInfo)
         -> *mut *mut ASN1_OCTET_STRING;
        #[no_mangle]
        #[c2rust::src_loc = "130:1"]
        pub fn CMS_verify(cms: *mut CMS_ContentInfo,
                          certs: *mut stack_st_X509, store: *mut X509_STORE,
                          dcont: *mut BIO, out: *mut BIO, flags: libc::c_uint)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "207:1"]
        pub fn CMS_get0_eContentType(cms: *mut CMS_ContentInfo)
         -> *const ASN1_OBJECT;
        #[no_mangle]
        #[c2rust::src_loc = "212:1"]
        pub fn CMS_get1_certs(cms: *mut CMS_ContentInfo)
         -> *mut stack_st_X509;
        #[no_mangle]
        #[c2rust::src_loc = "217:1"]
        pub fn CMS_get1_crls(cms: *mut CMS_ContentInfo)
         -> *mut stack_st_X509_CRL;
        #[no_mangle]
        #[c2rust::src_loc = "225:1"]
        pub fn CMS_get0_SignerInfos(cms: *mut CMS_ContentInfo)
         -> *mut stack_st_CMS_SignerInfo;
        #[no_mangle]
        #[c2rust::src_loc = "232:1"]
        pub fn CMS_set1_signers_certs(cms: *mut CMS_ContentInfo,
                                      certs: *mut stack_st_X509,
                                      flags: libc::c_uint) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "234:1"]
        pub fn CMS_SignerInfo_get0_algs(si: *mut CMS_SignerInfo,
                                        pk: *mut *mut EVP_PKEY,
                                        signer: *mut *mut X509,
                                        pdig: *mut *mut X509_ALGOR,
                                        psig: *mut *mut X509_ALGOR);
    }
}
#[c2rust::header_src = "/usr/include/dirent.h:38"]
pub mod include_dirent_h {
    #[c2rust::src_loc = "127:1"]
    pub type DIR = __dirstream;
    #[c2rust::src_loc = "105:5"]
    pub const DT_DIR: C2RustUnnamed_2 = 4;
    #[c2rust::src_loc = "97:1"]
    pub type C2RustUnnamed_2 = libc::c_uint;
    #[c2rust::src_loc = "115:5"]
    pub const DT_WHT: C2RustUnnamed_2 = 14;
    #[c2rust::src_loc = "113:5"]
    pub const DT_SOCK: C2RustUnnamed_2 = 12;
    #[c2rust::src_loc = "111:5"]
    pub const DT_LNK: C2RustUnnamed_2 = 10;
    #[c2rust::src_loc = "109:5"]
    pub const DT_REG: C2RustUnnamed_2 = 8;
    #[c2rust::src_loc = "107:5"]
    pub const DT_BLK: C2RustUnnamed_2 = 6;
    #[c2rust::src_loc = "103:5"]
    pub const DT_CHR: C2RustUnnamed_2 = 2;
    #[c2rust::src_loc = "101:5"]
    pub const DT_FIFO: C2RustUnnamed_2 = 1;
    #[c2rust::src_loc = "99:5"]
    pub const DT_UNKNOWN: C2RustUnnamed_2 = 0;
    use super::dirent_h::dirent;
    extern "C" {
        #[c2rust::src_loc = "127:16"]
        pub type __dirstream;
        #[no_mangle]
        #[c2rust::src_loc = "134:1"]
        pub fn opendir(__name: *const libc::c_char) -> *mut DIR;
        #[no_mangle]
        #[c2rust::src_loc = "149:1"]
        pub fn closedir(__dirp: *mut DIR) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "162:1"]
        pub fn readdir(__dirp: *mut DIR) -> *mut dirent;
    }
}
#[c2rust::header_src = "/usr/include/bits/dirent.h:38"]
pub mod dirent_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "22:8"]
    pub struct dirent {
        pub d_ino: __ino_t,
        pub d_off: __off_t,
        pub d_reclen: libc::c_ushort,
        pub d_type: libc::c_uchar,
        pub d_name: [libc::c_char; 256],
    }
    use super::types_h::{__ino_t, __off_t};
}
#[c2rust::header_src = "/usr/include/openssl/pem.h:33"]
pub mod pem_h {
    #[c2rust::src_loc = "231:1"]
    pub type pem_password_cb
        =
        unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int,
                             _: libc::c_int, _: *mut libc::c_void)
            -> libc::c_int;
    use super::ossl_typ_h::{BIO, EVP_PKEY, X509};
    use super::x509_h::stack_st_X509_INFO;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "318:31"]
        pub fn PEM_read_bio_PrivateKey(bp: *mut BIO, x: *mut *mut EVP_PKEY,
                                       cb: Option<pem_password_cb>,
                                       u: *mut libc::c_void) -> *mut EVP_PKEY;
        #[no_mangle]
        #[c2rust::src_loc = "290:22"]
        pub fn PEM_read_bio_X509(bp: *mut BIO, x: *mut *mut X509,
                                 cb: Option<pem_password_cb>,
                                 u: *mut libc::c_void) -> *mut X509;
        #[no_mangle]
        #[c2rust::src_loc = "258:1"]
        pub fn PEM_X509_INFO_read_bio(bp: *mut BIO,
                                      sk: *mut stack_st_X509_INFO,
                                      cb: Option<pem_password_cb>,
                                      u: *mut libc::c_void)
         -> *mut stack_st_X509_INFO;
    }
}
#[c2rust::header_src = "/usr/include/openssl/pkcs12.h:33"]
pub mod pkcs12_h {
    #[c2rust::src_loc = "45:1"]
    pub type PKCS12 = PKCS12_st;
    use super::ossl_typ_h::{EVP_PKEY, X509};
    use super::x509_h::stack_st_X509;
    use super::FILE_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "45:16"]
        pub type PKCS12_st;
        #[no_mangle]
        #[c2rust::src_loc = "187:1"]
        pub fn PKCS12_free(a: *mut PKCS12);
        #[no_mangle]
        #[c2rust::src_loc = "196:1"]
        pub fn PKCS12_parse(p12: *mut PKCS12, pass: *const libc::c_char,
                            pkey: *mut *mut EVP_PKEY, cert: *mut *mut X509,
                            ca: *mut *mut stack_st_X509) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "216:1"]
        pub fn d2i_PKCS12_fp(fp: *mut FILE, p12: *mut *mut PKCS12)
         -> *mut PKCS12;
    }
}
#[c2rust::header_src = "/usr/include/openssl/x509v3.h:33"]
pub mod x509v3_h {
    #[c2rust::src_loc = "123:1"]
    pub type GENERAL_NAME = GENERAL_NAME_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "123:16"]
    pub struct GENERAL_NAME_st {
        pub type_0: libc::c_int,
        pub d: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "134:5"]
    pub union C2RustUnnamed_1 {
        pub ptr: *mut libc::c_char,
        pub otherName: *mut OTHERNAME,
        pub rfc822Name: *mut ASN1_IA5STRING,
        pub dNSName: *mut ASN1_IA5STRING,
        pub x400Address: *mut ASN1_TYPE,
        pub directoryName: *mut X509_NAME,
        pub ediPartyName: *mut EDIPARTYNAME,
        pub uniformResourceIdentifier: *mut ASN1_IA5STRING,
        pub iPAddress: *mut ASN1_OCTET_STRING,
        pub registeredID: *mut ASN1_OBJECT,
        pub ip: *mut ASN1_OCTET_STRING,
        pub dirn: *mut X509_NAME,
        pub ia5: *mut ASN1_IA5STRING,
        pub rid: *mut ASN1_OBJECT,
        pub other: *mut ASN1_TYPE,
    }
    #[c2rust::src_loc = "118:1"]
    pub type EDIPARTYNAME = EDIPartyName_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:16"]
    pub struct EDIPartyName_st {
        pub nameAssigner: *mut ASN1_STRING,
        pub partyName: *mut ASN1_STRING,
    }
    #[c2rust::src_loc = "113:1"]
    pub type OTHERNAME = otherName_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "113:16"]
    pub struct otherName_st {
        pub type_id: *mut ASN1_OBJECT,
        pub value: *mut ASN1_TYPE,
    }
    #[c2rust::src_loc = "167:1"]
    pub type GENERAL_NAMES = stack_st_GENERAL_NAME;
    #[c2rust::src_loc = "166:1"]
    pub type sk_GENERAL_NAME_freefunc
        =
        Option<unsafe extern "C" fn(_: *mut GENERAL_NAME) -> ()>;
    #[c2rust::src_loc = "162:1"]
    pub type EXTENDED_KEY_USAGE = stack_st_ASN1_OBJECT;
    #[inline]
    #[c2rust::src_loc = "166:1"]
    pub unsafe extern "C" fn sk_GENERAL_NAME_pop_free(mut sk:
                                                          *mut stack_st_GENERAL_NAME,
                                                      mut freefunc:
                                                          sk_GENERAL_NAME_freefunc) {
        OPENSSL_sk_pop_free(sk as *mut OPENSSL_STACK,
                            ::std::mem::transmute::<sk_GENERAL_NAME_freefunc,
                                                    OPENSSL_sk_freefunc>(freefunc));
    }
    #[inline]
    #[c2rust::src_loc = "166:1"]
    pub unsafe extern "C" fn sk_GENERAL_NAME_value(mut sk:
                                                       *const stack_st_GENERAL_NAME,
                                                   mut idx: libc::c_int)
     -> *mut GENERAL_NAME {
        return OPENSSL_sk_value(sk as *const OPENSSL_STACK, idx) as
                   *mut GENERAL_NAME;
    }
    #[inline]
    #[c2rust::src_loc = "166:1"]
    pub unsafe extern "C" fn sk_GENERAL_NAME_num(mut sk:
                                                     *const stack_st_GENERAL_NAME)
     -> libc::c_int {
        return OPENSSL_sk_num(sk as *const OPENSSL_STACK);
    }
    use super::ossl_typ_h::{ASN1_IA5STRING, X509_NAME, ASN1_OCTET_STRING,
                            ASN1_OBJECT, ASN1_STRING, X509};
    use super::asn1_h::{ASN1_TYPE, stack_st_ASN1_OBJECT};
    use super::stack_h::{OPENSSL_sk_pop_free, OPENSSL_STACK,
                         OPENSSL_sk_freefunc, OPENSSL_sk_value,
                         OPENSSL_sk_num};
    use super::x509_h::X509_EXTENSION;
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        #[c2rust::src_loc = "166:1"]
        pub type stack_st_GENERAL_NAME;
        #[no_mangle]
        #[c2rust::src_loc = "469:1"]
        pub fn GENERAL_NAME_free(a: *mut GENERAL_NAME);
        #[no_mangle]
        #[c2rust::src_loc = "624:1"]
        pub fn X509V3_EXT_d2i(ext: *mut X509_EXTENSION) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "660:1"]
        pub fn X509_get_key_usage(x: *mut X509) -> uint32_t;
        #[no_mangle]
        #[c2rust::src_loc = "649:1"]
        pub fn X509_check_ca(x: *mut X509) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "511:1"]
        pub fn EXTENDED_KEY_USAGE_free(a: *mut EXTENDED_KEY_USAGE);
    }
}
#[c2rust::header_src = "/usr/include/openssl/evp.h:33"]
pub mod evp_h {
    #[c2rust::src_loc = "396:1"]
    pub type EVP_CIPHER_INFO = evp_cipher_info_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "396:16"]
    pub struct evp_cipher_info_st {
        pub cipher: *const EVP_CIPHER,
        pub iv: [libc::c_uchar; 16],
    }
    use super::ossl_typ_h::{EVP_CIPHER, EVP_PKEY, EVP_MD_CTX, EVP_MD, ENGINE,
                            EVP_CIPHER_CTX};
    use super::stddef_h::size_t;
    use super::asn1_h::ASN1_TYPE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "1041:1"]
        pub fn EVP_PKEY_free(pkey: *mut EVP_PKEY);
        #[no_mangle]
        #[c2rust::src_loc = "540:1"]
        pub fn EVP_MD_CTX_free(ctx: *mut EVP_MD_CTX);
        #[no_mangle]
        #[c2rust::src_loc = "620:8"]
        pub fn EVP_SignFinal(ctx: *mut EVP_MD_CTX, md: *mut libc::c_uchar,
                             s: *mut libc::c_uint, pkey: *mut EVP_PKEY)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "996:1"]
        pub fn EVP_PKEY_size(pkey: *const EVP_PKEY) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "550:8"]
        pub fn EVP_DigestUpdate(ctx: *mut EVP_MD_CTX, d: *const libc::c_void,
                                cnt: size_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "709:1"]
        pub fn EVP_sha1() -> *const EVP_MD;
        #[no_mangle]
        #[c2rust::src_loc = "559:8"]
        pub fn EVP_DigestInit(ctx: *mut EVP_MD_CTX, type_0: *const EVP_MD)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "538:1"]
        pub fn EVP_MD_CTX_new() -> *mut EVP_MD_CTX;
        #[no_mangle]
        #[c2rust::src_loc = "552:8"]
        pub fn EVP_DigestFinal_ex(ctx: *mut EVP_MD_CTX,
                                  md: *mut libc::c_uchar,
                                  s: *mut libc::c_uint) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "548:8"]
        pub fn EVP_DigestInit_ex(ctx: *mut EVP_MD_CTX, type_0: *const EVP_MD,
                                 impl_0: *mut ENGINE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "451:1"]
        pub fn EVP_MD_CTX_md(ctx: *const EVP_MD_CTX) -> *const EVP_MD;
        #[no_mangle]
        #[c2rust::src_loc = "756:1"]
        pub fn EVP_des_ede3_cbc() -> *const EVP_CIPHER;
        #[no_mangle]
        #[c2rust::src_loc = "681:1"]
        pub fn EVP_CIPHER_CTX_free(c: *mut EVP_CIPHER_CTX);
        #[no_mangle]
        #[c2rust::src_loc = "601:8"]
        pub fn EVP_DecryptFinal(ctx: *mut EVP_CIPHER_CTX,
                                outm: *mut libc::c_uchar,
                                outl: *mut libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "599:12"]
        pub fn EVP_DecryptUpdate(ctx: *mut EVP_CIPHER_CTX,
                                 out: *mut libc::c_uchar,
                                 outl: *mut libc::c_int,
                                 in_0: *const libc::c_uchar, inl: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "593:8"]
        pub fn EVP_DecryptInit(ctx: *mut EVP_CIPHER_CTX,
                               cipher: *const EVP_CIPHER,
                               key: *const libc::c_uchar,
                               iv: *const libc::c_uchar) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "985:1"]
        pub fn EVP_PKEY_decrypt_old(dec_key: *mut libc::c_uchar,
                                    enc_key: *const libc::c_uchar,
                                    enc_key_len: libc::c_int,
                                    private_key: *mut EVP_PKEY)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "685:1"]
        pub fn EVP_CIPHER_CTX_rand_key(ctx: *mut EVP_CIPHER_CTX,
                                       key: *mut libc::c_uchar)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1077:1"]
        pub fn EVP_CIPHER_asn1_to_param(c: *mut EVP_CIPHER_CTX,
                                        type_0: *mut ASN1_TYPE)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "679:1"]
        pub fn EVP_CIPHER_CTX_new() -> *mut EVP_CIPHER_CTX;
        #[no_mangle]
        #[c2rust::src_loc = "466:1"]
        pub fn EVP_CIPHER_block_size(cipher: *const EVP_CIPHER)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "468:1"]
        pub fn EVP_CIPHER_key_length(cipher: *const EVP_CIPHER)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "968:1"]
        pub fn EVP_get_cipherbyname(name: *const libc::c_char)
         -> *const EVP_CIPHER;
        #[no_mangle]
        #[c2rust::src_loc = "560:8"]
        pub fn EVP_DigestFinal(ctx: *mut EVP_MD_CTX, md: *mut libc::c_uchar,
                               s: *mut libc::c_uint) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "713:1"]
        pub fn EVP_sha512() -> *const EVP_MD;
        #[no_mangle]
        #[c2rust::src_loc = "711:1"]
        pub fn EVP_sha256() -> *const EVP_MD;
    }
}
#[c2rust::header_src = "/usr/include/openssl/stack.h:33"]
pub mod stack_h {
    #[c2rust::src_loc = "20:1"]
    pub type OPENSSL_sk_freefunc
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "17:1"]
    pub type OPENSSL_STACK = stack_st;
    extern "C" {
        #[c2rust::src_loc = "17:16"]
        pub type stack_st;
        #[no_mangle]
        #[c2rust::src_loc = "33:1"]
        pub fn OPENSSL_sk_pop_free(st: *mut OPENSSL_STACK,
                                   func:
                                       Option<unsafe extern "C" fn(_:
                                                                       *mut libc::c_void)
                                                  -> ()>);
        #[no_mangle]
        #[c2rust::src_loc = "24:1"]
        pub fn OPENSSL_sk_value(_: *const OPENSSL_STACK, _: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "42:1"]
        pub fn OPENSSL_sk_push(st: *mut OPENSSL_STACK,
                               data: *const libc::c_void) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "23:1"]
        pub fn OPENSSL_sk_num(_: *const OPENSSL_STACK) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "29:1"]
        pub fn OPENSSL_sk_new_null() -> *mut OPENSSL_STACK;
        #[no_mangle]
        #[c2rust::src_loc = "32:1"]
        pub fn OPENSSL_sk_free(_: *mut OPENSSL_STACK);
    }
}
#[c2rust::header_src = "/usr/include/openssl/asn1t.h:33"]
pub mod asn1t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "580:8"]
    pub struct ASN1_ITEM_st {
        pub itype: libc::c_char,
        pub utype: libc::c_long,
        pub templates: *const ASN1_TEMPLATE,
        pub tcount: libc::c_long,
        pub funcs: *const libc::c_void,
        pub size: libc::c_long,
        pub sname: *const libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "468:8"]
    pub struct ASN1_TEMPLATE_st {
        pub flags: libc::c_ulong,
        pub tag: libc::c_long,
        pub offset: libc::c_ulong,
        pub field_name: *const libc::c_char,
        pub item: *const ASN1_ITEM_EXP,
    }
    use super::asn1_h::{ASN1_TEMPLATE, ASN1_ITEM_EXP};
    use super::ossl_typ_h::ASN1_ITEM;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "908:1"]
        pub static BIGNUM_it: ASN1_ITEM;
    }
}
#[c2rust::header_src = "/usr/include/openssl/x509_vfy.h:33"]
pub mod x509_vfy_h {
    #[c2rust::src_loc = "63:1"]
    pub type X509_STORE_CTX_verify_cb
        =
        Option<unsafe extern "C" fn(_: libc::c_int, _: *mut X509_STORE_CTX)
                   -> libc::c_int>;
    use super::ossl_typ_h::{X509_STORE_CTX, X509_STORE, X509};
    use super::x509_h::{stack_st_X509, stack_st_X509_CRL};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "268:1"]
        pub fn X509_STORE_free(v: *mut X509_STORE);
        #[no_mangle]
        #[c2rust::src_loc = "335:1"]
        pub fn X509_STORE_CTX_free(ctx: *mut X509_STORE_CTX);
        #[no_mangle]
        #[c2rust::src_loc = "499:1"]
        pub fn X509_STORE_CTX_get1_chain(ctx: *mut X509_STORE_CTX)
         -> *mut stack_st_X509;
        #[no_mangle]
        #[c2rust::src_loc = "489:1"]
        pub fn X509_STORE_CTX_get_error(ctx: *mut X509_STORE_CTX)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "491:1"]
        pub fn X509_STORE_CTX_get_error_depth(ctx: *mut X509_STORE_CTX)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "338:1"]
        pub fn X509_STORE_CTX_set0_trusted_stack(ctx: *mut X509_STORE_CTX,
                                                 sk: *mut stack_st_X509);
        #[no_mangle]
        #[c2rust::src_loc = "336:1"]
        pub fn X509_STORE_CTX_init(ctx: *mut X509_STORE_CTX,
                                   store: *mut X509_STORE, x509: *mut X509,
                                   chain: *mut stack_st_X509) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "331:1"]
        pub fn X509_STORE_CTX_new() -> *mut X509_STORE_CTX;
        #[no_mangle]
        #[c2rust::src_loc = "288:1"]
        pub fn X509_STORE_set_verify_cb(ctx: *mut X509_STORE,
                                        verify_cb: X509_STORE_CTX_verify_cb);
        #[no_mangle]
        #[c2rust::src_loc = "267:1"]
        pub fn X509_STORE_new() -> *mut X509_STORE;
        #[no_mangle]
        #[c2rust::src_loc = "493:1"]
        pub fn X509_STORE_CTX_get_current_cert(ctx: *mut X509_STORE_CTX)
         -> *mut X509;
        #[no_mangle]
        #[c2rust::src_loc = "502:1"]
        pub fn X509_STORE_CTX_set0_crls(c: *mut X509_STORE_CTX,
                                        sk: *mut stack_st_X509_CRL);
        #[no_mangle]
        #[c2rust::src_loc = "276:1"]
        pub fn X509_STORE_set_flags(ctx: *mut X509_STORE,
                                    flags: libc::c_ulong) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/openssl/bio.h:33"]
pub mod bio_h {
    #[c2rust::src_loc = "249:1"]
    pub type BIO_METHOD = bio_method_st;
    use super::ossl_typ_h::BIO;
    use super::stdint_uintn_h::uint64_t;
    extern "C" {
        #[c2rust::src_loc = "249:16"]
        pub type bio_method_st;
        #[no_mangle]
        #[c2rust::src_loc = "549:1"]
        pub fn BIO_free(a: *mut BIO) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "558:1"]
        pub fn BIO_read(b: *mut BIO, data: *mut libc::c_void,
                        dlen: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "588:1"]
        pub fn BIO_s_mem() -> *const BIO_METHOD;
        #[no_mangle]
        #[c2rust::src_loc = "548:1"]
        pub fn BIO_new(type_0: *const BIO_METHOD) -> *mut BIO;
        #[no_mangle]
        #[c2rust::src_loc = "590:1"]
        pub fn BIO_new_mem_buf(buf: *const libc::c_void, len: libc::c_int)
         -> *mut BIO;
        #[no_mangle]
        #[c2rust::src_loc = "561:1"]
        pub fn BIO_write(b: *mut BIO, data: *const libc::c_void,
                         dlen: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn BIO_ctrl(bp: *mut BIO, cmd: libc::c_int, larg: libc::c_long,
                        parg: *mut libc::c_void) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "543:1"]
        pub fn BIO_s_file() -> *const BIO_METHOD;
        #[no_mangle]
        #[c2rust::src_loc = "531:1"]
        pub fn BIO_number_written(bio: *mut BIO) -> uint64_t;
        #[no_mangle]
        #[c2rust::src_loc = "544:1"]
        pub fn BIO_new_file(filename: *const libc::c_char,
                            mode: *const libc::c_char) -> *mut BIO;
    }
}
#[c2rust::header_src = "/usr/include/openssl/sha.h:33"]
pub mod sha_h {
    #[c2rust::src_loc = "34:1"]
    pub type SHA_CTX = SHAstate_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "34:16"]
    pub struct SHAstate_st {
        pub h0: libc::c_uint,
        pub h1: libc::c_uint,
        pub h2: libc::c_uint,
        pub h3: libc::c_uint,
        pub h4: libc::c_uint,
        pub Nl: libc::c_uint,
        pub Nh: libc::c_uint,
        pub data: [libc::c_uint; 16],
        pub num: libc::c_uint,
    }
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "43:1"]
        pub fn SHA1_Final(md: *mut libc::c_uchar, c: *mut SHA_CTX)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "42:1"]
        pub fn SHA1_Update(c: *mut SHA_CTX, data: *const libc::c_void,
                           len: size_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn SHA1_Init(c: *mut SHA_CTX) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:32"]
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
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:32"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    use super::internal::__va_list_tag;
    extern "C" {
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
        #[c2rust::src_loc = "366:1"]
        pub fn vasprintf(__ptr: *mut *mut libc::c_char,
                         __f: *const libc::c_char, __arg: ::std::ffi::VaList)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "786:1"]
        pub fn fileno(__stream: *mut FILE) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:32"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "175:1"]
        pub fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:32"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:32"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/string.h:32"]
pub mod string_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "46:14"]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "435:1"]
        pub fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "90:14"]
        pub fn memchr(_: *const libc::c_void, _: libc::c_int,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
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
#[c2rust::header_src = "/usr/include/assert.h:32"]
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
#[c2rust::header_src = "/usr/include/netinet/in.h:32"]
pub mod in_h {
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "378:1"]
        pub fn htonl(__hostlong: uint32_t) -> uint32_t;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-trace.h:32"]
pub mod k5_trace_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::krb5_context;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-trace.h */
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
        /*
 * This header contains trace macro definitions, which map trace points within
 * the code to krb5int_trace() calls with descriptive text strings.
 *
 * A new trace macro must be defined in this file for each new location to
 * be traced; the TRACE() macro should never be used directly.  This keeps
 * the tracing logic centralized in one place, to facilitate integration with
 * alternate tracing backends such as DTrace.
 *
 * Trace logging is intended to aid power users in diagnosing configuration
 * problems by showing what's going on behind the scenes of complex operations.
 * Although trace logging is sometimes useful to developers, it is not intended
 * as a replacement for a debugger, and it is not desirable to drown the user
 * in output.  Observe the following guidelines when adding trace points:
 *
 *   - Avoid mentioning function or variable names in messages.
 *
 *   - Try to convey what decisions are being made and what external inputs
 *     they are based on, not the process of making decisions.
 *
 *   - It is generally not necessary to trace before returning an unrecoverable
 *     error.  If an error code is unclear by itself, make it clearer with
 *     krb5_set_error_message().
 *
 *   - Keep macros simple.  Add format specifiers to krb5int_trace's formatter
 *     as necessary (and document them here) instead of transforming macro
 *     arguments.
 *
 *   - Like printf, the trace formatter interface is not type-safe.  Check your
 *     formats carefully.  Cast integral arguments to the appropriate type if
 *     they do not already patch.
 *
 * The following specifiers are supported by the formatter (see the
 * implementation in lib/krb5/os/trace.c for details):
 *
 *   {int}         int, in decimal
 *   {long}        long, in decimal
 *   {str}         const char *, display as C string
 *   {lenstr}      size_t and const char *, as a counted string
 *   {hexlenstr}   size_t and const char *, as hex bytes
 *   {hashlenstr}  size_t and const char *, as four-character hex hash
 *   {raddr}       struct remote_address *, show socket type, address, port
 *   {data}        krb5_data *, display as counted string
 *   {hexdata}     krb5_data *, display as hex bytes
 *   {errno}       int, display as number/errorstring
 *   {kerr}        krb5_error_code, display as number/errorstring
 *   {keyblock}    const krb5_keyblock *, display enctype and hash of key
 *   {key}         krb5_key, display enctype and hash of key
 *   {cksum}       const krb5_checksum *, display cksumtype and hex checksum
 *   {princ}       krb5_principal, unparse and display
 *   {ptype}       krb5_int32, krb5_principal type, display name
 *   {patype}      krb5_preauthtype, a single padata type number
 *   {patypes}     krb5_pa_data **, display list of padata type numbers
 *   {etype}       krb5_enctype, display shortest name of enctype
 *   {etypes}      krb5_enctype *, display list of enctypes
 *   {ccache}      krb5_ccache, display type:name
 *   {keytab}      krb5_keytab, display name
 *   {creds}       krb5_creds *, display clientprinc -> serverprinc
 */
        #[no_mangle]
        #[c2rust::src_loc = "94:1"]
        pub fn krb5int_trace(context: krb5_context, fmt: *const libc::c_char,
                             _: ...);
    }
    /* K5_TRACE_H */
    /* DISABLE_TRACING */
    /* Try to optimize away argument evaluation and function call when we're not
 * tracing, if this source file knows the internals of the context. */
}
#[c2rust::header_src = "/usr/include/openssl/dh.h:33"]
pub mod dh_h {
    use super::ossl_typ_h::{DH, BIGNUM};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "125:1"]
        pub fn DH_free(dh: *mut DH);
        #[no_mangle]
        #[c2rust::src_loc = "182:1"]
        pub fn DH_set0_pqg(dh: *mut DH, p: *mut BIGNUM, q: *mut BIGNUM,
                           g: *mut BIGNUM) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "124:1"]
        pub fn DH_new() -> *mut DH;
        #[no_mangle]
        #[c2rust::src_loc = "180:1"]
        pub fn DH_get0_pqg(dh: *const DH, p: *mut *const BIGNUM,
                           q: *mut *const BIGNUM, g: *mut *const BIGNUM);
        #[no_mangle]
        #[c2rust::src_loc = "150:1"]
        pub fn DH_check_pub_key(dh: *const DH, pub_key: *const BIGNUM,
                                codes: *mut libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "149:1"]
        pub fn DH_check(dh: *const DH, codes: *mut libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "183:1"]
        pub fn DH_get0_key(dh: *const DH, pub_key: *mut *const BIGNUM,
                           priv_key: *mut *const BIGNUM);
        #[no_mangle]
        #[c2rust::src_loc = "151:1"]
        pub fn DH_generate_key(dh: *mut DH) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "152:1"]
        pub fn DH_compute_key(key: *mut libc::c_uchar, pub_key: *const BIGNUM,
                              dh: *mut DH) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "128:1"]
        pub fn DH_size(dh: *const DH) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/openssl/bn.h:33"]
pub mod bn_h {
    use super::ossl_typ_h::BIGNUM;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "275:1"]
        pub fn BN_free(a: *mut BIGNUM);
        #[no_mangle]
        #[c2rust::src_loc = "271:1"]
        pub fn BN_set_word(a: *mut BIGNUM, w: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "216:1"]
        pub fn BN_new() -> *mut BIGNUM;
        #[no_mangle]
        #[c2rust::src_loc = "303:1"]
        pub fn BN_rshift1(r: *mut BIGNUM, a: *const BIGNUM) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "221:1"]
        pub fn BN_bin2bn(s: *const libc::c_uchar, len: libc::c_int,
                         ret: *mut BIGNUM) -> *mut BIGNUM;
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn BN_num_bits(a: *const BIGNUM) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "274:1"]
        pub fn BN_cmp(a: *const BIGNUM, b: *const BIGNUM) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "305:1"]
        pub fn BN_dup(a: *const BIGNUM) -> *mut BIGNUM;
    }
}
#[c2rust::header_src = "/usr/include/openssl/objects.h:33"]
pub mod objects_h {
    use super::ossl_typ_h::ASN1_OBJECT;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "63:1"]
        pub fn OBJ_txt2obj(s: *const libc::c_char, no_name: libc::c_int)
         -> *mut ASN1_OBJECT;
        #[no_mangle]
        #[c2rust::src_loc = "58:1"]
        pub fn OBJ_dup(o: *const ASN1_OBJECT) -> *mut ASN1_OBJECT;
        #[no_mangle]
        #[c2rust::src_loc = "59:1"]
        pub fn OBJ_nid2obj(n: libc::c_int) -> *mut ASN1_OBJECT;
        #[no_mangle]
        #[c2rust::src_loc = "163:1"]
        pub fn OBJ_length(obj: *const ASN1_OBJECT) -> size_t;
        #[no_mangle]
        #[c2rust::src_loc = "164:1"]
        pub fn OBJ_get0_data(obj: *const ASN1_OBJECT) -> *const libc::c_uchar;
        #[no_mangle]
        #[c2rust::src_loc = "68:1"]
        pub fn OBJ_cmp(a: *const ASN1_OBJECT, b: *const ASN1_OBJECT)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "62:1"]
        pub fn OBJ_obj2nid(o: *const ASN1_OBJECT) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "61:1"]
        pub fn OBJ_nid2sn(n: libc::c_int) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/openssl/crypto.h:33"]
pub mod crypto_h {
    use super::stdint_uintn_h::uint64_t;
    use super::ossl_typ_h::OPENSSL_INIT_SETTINGS;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "388:1"]
        pub fn OPENSSL_init_crypto(opts: uint64_t,
                                   settings: *const OPENSSL_INIT_SETTINGS)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/openssl/err.h:33"]
pub mod err_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "237:1"]
        pub fn ERR_error_string_n(e: libc::c_ulong, buf: *mut libc::c_char,
                                  len: size_t);
        #[no_mangle]
        #[c2rust::src_loc = "223:1"]
        pub fn ERR_get_error() -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "240:1"]
        pub fn ERR_reason_error_string(e: libc::c_ulong)
         -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "227:1"]
        pub fn ERR_peek_error() -> libc::c_ulong;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/preauth/pkinit/pkinit_accessor.h:33"]
pub mod pkinit_accessor_h {
    use super::krb5_h::{krb5_context, krb5_prompt_type, krb5_error_code,
                        krb5_data, krb5_principal_data};
    use super::k5_int_pkinit_h::{krb5_external_principal_identifier,
                                 krb5_algorithm_identifier};
    extern "C" {
        /*
 * COPYRIGHT (C) 2006,2007
 * THE REGENTS OF THE UNIVERSITY OF MICHIGAN
 * ALL RIGHTS RESERVED
 *
 * Permission is granted to use, copy, create derivative works
 * and redistribute this software and such derivative works
 * for any purpose, so long as the name of The University of
 * Michigan is not used in any advertising or publicity
 * pertaining to the use of distribution of this software
 * without specific, written prior authorization.  If the
 * above copyright notice or any other identification of the
 * University of Michigan is included in any copy of any
 * portion of this software, then the disclaimer below must
 * also be included.
 *
 * THIS SOFTWARE IS PROVIDED AS IS, WITHOUT REPRESENTATION
 * FROM THE UNIVERSITY OF MICHIGAN AS TO ITS FITNESS FOR ANY
 * PURPOSE, AND WITHOUT WARRANTY BY THE UNIVERSITY OF
 * MICHIGAN OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING
 * WITHOUT LIMITATION THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE
 * REGENTS OF THE UNIVERSITY OF MICHIGAN SHALL NOT BE LIABLE
 * FOR ANY DAMAGES, INCLUDING SPECIAL, INDIRECT, INCIDENTAL, OR
 * CONSEQUENTIAL DAMAGES, WITH RESPECT TO ANY CLAIM ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OF THE SOFTWARE, EVEN
 * IF IT HAS BEEN OR IS HEREAFTER ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGES.
 */
        /*
 * Function prototypes
 */
        /* special cases... */
        #[no_mangle]
        #[c2rust::src_loc = "71:15"]
        pub static mut k5int_set_prompt_types:
                   Option<unsafe extern "C" fn(_: krb5_context,
                                               _: *mut krb5_prompt_type)
                              -> ()>;
        #[no_mangle]
        #[c2rust::src_loc = "62:26"]
        pub static mut k5int_encode_krb5_td_trusted_certifiers:
                   Option<unsafe extern "C" fn(_:
                                                   *const *mut krb5_external_principal_identifier,
                                               _: *mut *mut krb5_data)
                              -> krb5_error_code>;
        #[no_mangle]
        #[c2rust::src_loc = "57:26"]
        pub static mut k5int_encode_krb5_td_dh_parameters:
                   Option<unsafe extern "C" fn(_:
                                                   *const *mut krb5_algorithm_identifier,
                                               _: *mut *mut krb5_data)
                              -> krb5_error_code>;
        #[no_mangle]
        #[c2rust::src_loc = "54:26"]
        pub static mut k5int_decode_krb5_principal_name:
                   Option<unsafe extern "C" fn(_: *const krb5_data,
                                               _:
                                                   *mut *mut krb5_principal_data)
                              -> krb5_error_code>;
    }
    /* _PKINIT_ACCESSOR_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-hex.h:35"]
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
#[c2rust::header_src = "/usr/include/dlfcn.h:36"]
pub mod dlfcn_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "56:1"]
        pub fn dlopen(__file: *const libc::c_char, __mode: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "60:1"]
        pub fn dlclose(__handle: *mut libc::c_void) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn dlsym(__handle: *mut libc::c_void, __name: *const libc::c_char)
         -> *mut libc::c_void;
    }
}
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::types_h::{__u_char, __uint8_t, __int32_t, __uint32_t,
                        __uint64_t, __ino_t, __off_t, __off64_t};
pub use self::sys_types_h::{u_char, uint};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::pthreadtypes_h::pthread_once_t;
pub use self::stdarg_h::va_list;
pub use self::stdint_uintn_h::{uint8_t, uint32_t, uint64_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::k5_thread_h::{k5_os_nothread_once_t, k5_once_t, k5_once};
pub use self::k5_platform_h::{k5_init_t, krb5int_strlcpy};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_enctype,
                       krb5_preauthtype, krb5_flags, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal,
                       krb5_const_principal, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _krb5_keyblock,
                       krb5_keyblock, _krb5_pa_data, krb5_pa_data,
                       _krb5_prompt, krb5_prompt, krb5_prompter_fct,
                       _profile_t, krb5_anonymous_principal,
                       krb5_c_keylengths, krb5_c_random_to_key,
                       krb5_principal_compare_any_realm, krb5_free_principal,
                       krb5_free_keyblock_contents, krb5_free_data,
                       krb5_set_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, zapfree, k5calloc, k5alloc,
                         k5memdup0, plugin_mapping, _kdb_log_context,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::com_err_h::{errcode_t, error_message};
pub use self::k5_buf_h::{k5buftype, K5BUF_DYNAMIC_ZAP, K5BUF_DYNAMIC,
                         K5BUF_FIXED, K5BUF_ERROR, k5buf, k5_buf_init_dynamic,
                         k5_buf_add, k5_buf_add_fmt, k5_buf_status,
                         k5_buf_free};
pub use self::k5_int_pkinit_h::{_krb5_algorithm_identifier,
                                krb5_algorithm_identifier,
                                _krb5_external_principal_identifier,
                                krb5_external_principal_identifier,
                                _krb5_sp80056a_other_info,
                                krb5_sp80056a_other_info,
                                _krb5_pkinit_supp_pub_info,
                                krb5_pkinit_supp_pub_info,
                                encode_krb5_sp80056a_other_info,
                                encode_krb5_pkinit_supp_pub_info};
pub use self::pkcs11_h::{CK_FLAGS, _CK_VERSION, _CK_INFO, CK_NOTIFICATION,
                         CK_SLOT_ID, _CK_SLOT_INFO, _CK_TOKEN_INFO,
                         CK_SESSION_HANDLE, CK_USER_TYPE, CK_STATE,
                         _CK_SESSION_INFO, CK_OBJECT_HANDLE, CK_OBJECT_CLASS,
                         CK_KEY_TYPE, CK_CERTIFICATE_TYPE, CK_ATTRIBUTE_TYPE,
                         _CK_ATTRIBUTE, CK_MECHANISM_TYPE, _CK_MECHANISM,
                         _CK_MECHANISM_INFO, CK_RV, CK_NOTIFY,
                         _CK_FUNCTION_LIST, CK_C_WaitForSlotEvent,
                         CK_C_CancelFunction, CK_C_GetFunctionStatus,
                         CK_C_GenerateRandom, CK_C_SeedRandom, CK_C_DeriveKey,
                         CK_C_UnwrapKey, CK_C_WrapKey, CK_C_GenerateKeyPair,
                         CK_C_GenerateKey, CK_C_DecryptVerifyUpdate,
                         CK_C_SignEncryptUpdate, CK_C_DecryptDigestUpdate,
                         CK_C_DigestEncryptUpdate, CK_C_VerifyRecover,
                         CK_C_VerifyRecoverInit, CK_C_VerifyFinal,
                         CK_C_VerifyUpdate, CK_C_Verify, CK_C_VerifyInit,
                         CK_C_SignRecover, CK_C_SignRecoverInit,
                         CK_C_SignFinal, CK_C_SignUpdate, CK_C_Sign,
                         CK_C_SignInit, CK_C_DigestFinal, CK_C_DigestKey,
                         CK_C_DigestUpdate, CK_C_Digest, CK_C_DigestInit,
                         CK_C_DecryptFinal, CK_C_DecryptUpdate, CK_C_Decrypt,
                         CK_C_DecryptInit, CK_C_EncryptFinal,
                         CK_C_EncryptUpdate, CK_C_Encrypt, CK_C_EncryptInit,
                         CK_C_FindObjectsFinal, CK_C_FindObjects,
                         CK_C_FindObjectsInit, CK_C_SetAttributeValue,
                         CK_C_GetAttributeValue, CK_C_GetObjectSize,
                         CK_C_DestroyObject, CK_C_CopyObject,
                         CK_C_CreateObject, CK_C_Logout, CK_C_Login,
                         CK_C_SetOperationState, CK_C_GetOperationState,
                         CK_C_GetSessionInfo, CK_C_CloseAllSessions,
                         CK_C_CloseSession, CK_C_OpenSession, CK_C_SetPIN,
                         CK_C_InitPIN, CK_C_InitToken, CK_C_GetMechanismInfo,
                         CK_C_GetMechanismList, CK_C_GetTokenInfo,
                         CK_C_GetSlotInfo, CK_C_GetSlotList,
                         CK_C_GetFunctionList, CK_C_GetInfo, CK_C_Finalize,
                         CK_C_Initialize, CK_BYTE, CK_ULONG, CK_BYTE_PTR,
                         CK_ULONG_PTR, CK_SLOT_ID_PTR, CK_TOKEN_INFO,
                         CK_ATTRIBUTE, CK_MECHANISM, CK_FUNCTION_LIST_PTR,
                         CK_FUNCTION_LIST_PTR_PTR};
pub use self::pkinit_crypto_openssl_h::{_pkinit_plg_crypto_context,
                                        _pkinit_req_crypto_context,
                                        _pkinit_identity_crypto_context,
                                        pkinit_cred_info, _pkinit_cred_info};
pub use self::ossl_typ_h::{ASN1_OBJECT, DH, X509, EVP_PKEY, ASN1_STRING,
                           ASN1_UTF8STRING, ASN1_VISIBLESTRING,
                           ASN1_GENERALIZEDTIME, ASN1_UTCTIME,
                           ASN1_UNIVERSALSTRING, ASN1_BMPSTRING,
                           ASN1_GENERALSTRING, ASN1_IA5STRING, ASN1_T61STRING,
                           ASN1_PRINTABLESTRING, ASN1_OCTET_STRING,
                           ASN1_BIT_STRING, ASN1_ENUMERATED, ASN1_INTEGER,
                           ASN1_BOOLEAN, BIGNUM, OPENSSL_INIT_SETTINGS,
                           EVP_CIPHER, X509_ALGOR, X509_STORE_CTX, X509_NAME,
                           EVP_MD_CTX, BIO, X509_STORE, X509_CRL,
                           EVP_CIPHER_CTX, ENGINE, EVP_MD, ASN1_ITEM,
                           asn1_object_st, dh_st, x509_st, evp_pkey_st,
                           bignum_st, ossl_init_settings_st, evp_cipher_st,
                           x509_store_ctx_st, X509_name_st, evp_md_ctx_st,
                           bio_st, x509_store_st, X509_crl_st,
                           evp_cipher_ctx_st, engine_st, evp_md_st};
pub use self::pkinit_h::{pkinit_plg_crypto_context, pkinit_req_crypto_context,
                         pkinit_deferred_id, _pkinit_deferred_id,
                         pkinit_identity_crypto_context, _pkinit_plg_opts,
                         pkinit_plg_opts, _pkinit_identity_opts,
                         pkinit_identity_opts, pkiDebug, dh_oid,
                         pkinit_set_deferred_id, pkinit_find_deferred_id,
                         pkinit_get_deferred_id_flags,
                         pkinit_free_deferred_ids,
                         free_krb5_external_principal_identifier,
                         pkinit_copy_krb5_data, print_buffer};
pub use self::x509_h::{X509_algor_st, X509_INFO, X509_info_st, X509_PKEY,
                       private_key_st, X509_EXTENSION, sk_X509_CRL_freefunc,
                       sk_X509_freefunc, sk_X509_INFO_freefunc,
                       sk_X509_NAME_freefunc, sk_X509_NAME_pop_free,
                       sk_X509_NAME_push, sk_X509_NAME_new_null,
                       sk_X509_CRL_pop_free, sk_X509_pop_free, sk_X509_value,
                       sk_X509_push, sk_X509_num, sk_X509_new_null,
                       sk_X509_CRL_free, sk_X509_free, sk_X509_CRL_value,
                       sk_X509_CRL_push, sk_X509_CRL_num,
                       sk_X509_CRL_new_null, sk_X509_INFO_pop_free,
                       sk_X509_INFO_value, sk_X509_INFO_num,
                       stack_st_X509_CRL, stack_st_X509, stack_st_X509_INFO,
                       stack_st_X509_NAME, stack_st_X509_ATTRIBUTE,
                       X509_extension_st, X509_free, X509_ALGOR_free,
                       i2d_X509_ALGOR, X509_ALGOR_set0, X509_ALGOR_new,
                       X509_NAME_set, X509_dup, X509_verify_cert_error_string,
                       i2d_X509_NAME, X509_NAME_free, d2i_X509, X509_CRL_dup,
                       d2i_X509_NAME, i2d_X509, X509_CRL_free,
                       X509_get_serialNumber, X509_get_issuer_name,
                       X509_get_subject_name, X509_NAME_oneline,
                       X509_get_ext_by_NID, X509_check_private_key,
                       X509_get_ext, X509_NAME_print_ex, X509_INFO_free,
                       X509_CRL_cmp, X509_cmp, X509_NAME_cmp,
                       X509_get_ext_d2i, X509_verify_cert};
pub use self::pkinit_crypto_h::{cms_msg_types, CMS_ENVEL_SERVER,
                                CMS_SIGN_SERVER, CMS_SIGN_CLIENT,
                                _pkinit_cert_matching_data,
                                pkinit_cert_matching_data,
                                krb5_pkinit_sha512_oid_len,
                                krb5_pkinit_sha512_oid,
                                krb5_pkinit_sha256_oid_len,
                                krb5_pkinit_sha256_oid,
                                krb5_pkinit_sha1_oid_len,
                                krb5_pkinit_sha1_oid};
pub use self::asn1_h::{ASN1_TYPE, asn1_type_st, C2RustUnnamed, ASN1_VALUE,
                       asn1_string_st, ASN1_TEMPLATE, ASN1_ITEM_EXP,
                       sk_ASN1_OBJECT_num, sk_ASN1_OBJECT_value,
                       ASN1_VALUE_st, stack_st_X509_ALGOR,
                       stack_st_ASN1_OBJECT, ASN1_OBJECT_free,
                       ASN1_OCTET_STRING_free, ASN1_TYPE_new,
                       ASN1_OCTET_STRING_set, ASN1_OCTET_STRING_new,
                       ASN1_STRING_set, i2d_ASN1_OCTET_STRING,
                       ASN1_put_object, ASN1_object_size, ASN1_item_i2d,
                       ASN1_TYPE_free, ASN1_INTEGER_dup, ASN1_INTEGER_free,
                       ASN1_INTEGER_set, ASN1_STRING_get0_data,
                       ASN1_STRING_length, ASN1_get_object, i2d_ASN1_OBJECT,
                       ASN1_BIT_STRING_free, i2d_ASN1_INTEGER,
                       BN_to_ASN1_INTEGER, ASN1_INTEGER_to_BN,
                       d2i_ASN1_INTEGER, ASN1_BIT_STRING_it, ASN1_item_free,
                       ASN1_item_d2i, d2i_ASN1_OCTET_STRING, ASN1_INTEGER_get,
                       ASN1_INTEGER_cmp};
pub use self::pkcs7_h::{PKCS7, pkcs7_st, C2RustUnnamed_0, PKCS7_ENCRYPT,
                        pkcs7_encrypted_st, PKCS7_ENC_CONTENT,
                        pkcs7_enc_content_st, PKCS7_DIGEST, pkcs7_digest_st,
                        PKCS7_SIGN_ENVELOPE, pkcs7_signedandenveloped_st,
                        PKCS7_ENVELOPE, pkcs7_enveloped_st, PKCS7_SIGNED,
                        pkcs7_signed_st, PKCS7_ISSUER_AND_SERIAL,
                        pkcs7_issuer_and_serial_st, PKCS7_SIGNER_INFO,
                        pkcs7_signer_info_st, PKCS7_RECIP_INFO,
                        pkcs7_recip_info_st, sk_PKCS7_RECIP_INFO_value,
                        sk_PKCS7_RECIP_INFO_num, stack_st_PKCS7_RECIP_INFO,
                        stack_st_PKCS7_SIGNER_INFO, PKCS7_free, i2d_PKCS7,
                        PKCS7_new, PKCS7_add_signer, PKCS7_ATTR_SIGN_it,
                        PKCS7_add_signed_attribute, PKCS7_SIGNER_INFO_new,
                        PKCS7_SIGNED_new, PKCS7_ISSUER_AND_SERIAL_free,
                        i2d_PKCS7_ISSUER_AND_SERIAL,
                        PKCS7_ISSUER_AND_SERIAL_new, PKCS7_encrypt, d2i_PKCS7,
                        d2i_PKCS7_ISSUER_AND_SERIAL};
pub use self::cms_h::{CMS_SignerInfo, CMS_ContentInfo,
                      sk_CMS_SignerInfo_value, CMS_SignerInfo_st,
                      stack_st_CMS_SignerInfo, CMS_ContentInfo_st,
                      CMS_ContentInfo_free, d2i_CMS_ContentInfo,
                      CMS_get0_type, CMS_get0_content, CMS_verify,
                      CMS_get0_eContentType, CMS_get1_certs, CMS_get1_crls,
                      CMS_get0_SignerInfos, CMS_set1_signers_certs,
                      CMS_SignerInfo_get0_algs};
pub use self::include_dirent_h::{DIR, DT_DIR, C2RustUnnamed_2, DT_WHT,
                                 DT_SOCK, DT_LNK, DT_REG, DT_BLK, DT_CHR,
                                 DT_FIFO, DT_UNKNOWN, __dirstream, opendir,
                                 closedir, readdir};
pub use self::dirent_h::dirent;
pub use self::pem_h::{pem_password_cb, PEM_read_bio_PrivateKey,
                      PEM_read_bio_X509, PEM_X509_INFO_read_bio};
pub use self::pkcs12_h::{PKCS12, PKCS12_st, PKCS12_free, PKCS12_parse,
                         d2i_PKCS12_fp};
pub use self::x509v3_h::{GENERAL_NAME, GENERAL_NAME_st, C2RustUnnamed_1,
                         EDIPARTYNAME, EDIPartyName_st, OTHERNAME,
                         otherName_st, GENERAL_NAMES,
                         sk_GENERAL_NAME_freefunc, EXTENDED_KEY_USAGE,
                         sk_GENERAL_NAME_pop_free, sk_GENERAL_NAME_value,
                         sk_GENERAL_NAME_num, stack_st_GENERAL_NAME,
                         GENERAL_NAME_free, X509V3_EXT_d2i,
                         X509_get_key_usage, X509_check_ca,
                         EXTENDED_KEY_USAGE_free};
pub use self::evp_h::{EVP_CIPHER_INFO, evp_cipher_info_st, EVP_PKEY_free,
                      EVP_MD_CTX_free, EVP_SignFinal, EVP_PKEY_size,
                      EVP_DigestUpdate, EVP_sha1, EVP_DigestInit,
                      EVP_MD_CTX_new, EVP_DigestFinal_ex, EVP_DigestInit_ex,
                      EVP_MD_CTX_md, EVP_des_ede3_cbc, EVP_CIPHER_CTX_free,
                      EVP_DecryptFinal, EVP_DecryptUpdate, EVP_DecryptInit,
                      EVP_PKEY_decrypt_old, EVP_CIPHER_CTX_rand_key,
                      EVP_CIPHER_asn1_to_param, EVP_CIPHER_CTX_new,
                      EVP_CIPHER_block_size, EVP_CIPHER_key_length,
                      EVP_get_cipherbyname, EVP_DigestFinal, EVP_sha512,
                      EVP_sha256};
pub use self::stack_h::{OPENSSL_sk_freefunc, OPENSSL_STACK, stack_st,
                        OPENSSL_sk_pop_free, OPENSSL_sk_value,
                        OPENSSL_sk_push, OPENSSL_sk_num, OPENSSL_sk_new_null,
                        OPENSSL_sk_free};
pub use self::asn1t_h::{ASN1_ITEM_st, ASN1_TEMPLATE_st, BIGNUM_it};
pub use self::x509_vfy_h::{X509_STORE_CTX_verify_cb, X509_STORE_free,
                           X509_STORE_CTX_free, X509_STORE_CTX_get1_chain,
                           X509_STORE_CTX_get_error,
                           X509_STORE_CTX_get_error_depth,
                           X509_STORE_CTX_set0_trusted_stack,
                           X509_STORE_CTX_init, X509_STORE_CTX_new,
                           X509_STORE_set_verify_cb, X509_STORE_new,
                           X509_STORE_CTX_get_current_cert,
                           X509_STORE_CTX_set0_crls, X509_STORE_set_flags};
pub use self::bio_h::{BIO_METHOD, bio_method_st, BIO_free, BIO_read,
                      BIO_s_mem, BIO_new, BIO_new_mem_buf, BIO_write,
                      BIO_ctrl, BIO_s_file, BIO_number_written, BIO_new_file};
pub use self::sha_h::{SHA_CTX, SHAstate_st, SHA1_Final, SHA1_Update,
                      SHA1_Init};
use self::stdlib_h::{malloc, calloc, realloc, free};
use self::stdio_h::{fclose, fopen, snprintf, vasprintf, asprintf, fileno};
use self::fcntl_h::fcntl;
use self::errno_h::__errno_location;
use self::libintl_h::dgettext;
use self::string_h::{memmove, explicit_bzero, strlen, strdup, strncmp, memchr,
                     memcmp, memset, memcpy};
use self::assert_h::__assert_fail;
use self::in_h::htonl;
use self::k5_trace_h::krb5int_trace;
use self::dh_h::{DH_free, DH_set0_pqg, DH_new, DH_get0_pqg, DH_check_pub_key,
                 DH_check, DH_get0_key, DH_generate_key, DH_compute_key,
                 DH_size};
use self::bn_h::{BN_free, BN_set_word, BN_new, BN_rshift1, BN_bin2bn,
                 BN_num_bits, BN_cmp, BN_dup};
use self::objects_h::{OBJ_txt2obj, OBJ_dup, OBJ_nid2obj, OBJ_length,
                      OBJ_get0_data, OBJ_cmp, OBJ_obj2nid, OBJ_nid2sn};
use self::crypto_h::OPENSSL_init_crypto;
use self::err_h::{ERR_error_string_n, ERR_get_error, ERR_reason_error_string,
                  ERR_peek_error};
use self::pkinit_accessor_h::{k5int_set_prompt_types,
                              k5int_encode_krb5_td_trusted_certifiers,
                              k5int_encode_krb5_td_dh_parameters,
                              k5int_decode_krb5_principal_name};
use self::k5_hex_h::k5_hex_decode;
use self::dlfcn_h::{dlopen, dlclose, dlsym};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "2942:9"]
pub struct int_dhx942_dh {
    pub p: *mut BIGNUM,
    pub q: *mut BIGNUM,
    pub g: *mut BIGNUM,
    pub j: *mut BIGNUM,
    pub vparams: *mut int_dhvparams,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "2937:9"]
pub struct int_dhvparams {
    pub seed: *mut ASN1_BIT_STRING,
    pub counter: *mut BIGNUM,
}
/* Use CMS support present in OpenSSL. */
/* OPENSSL_VERSION_NUMBER >= 0x10100000L */
/* Return true if the cert x includes a key usage which doesn't include u. */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "248:15"]
pub struct pkcs11_errstrings {
    pub code: libc::c_short,
    pub text: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "740:8"]
pub struct get_key_cb_data {
    pub context: krb5_context,
    pub id_cryptoctx: pkinit_identity_crypto_context,
    pub fsname: *const libc::c_char,
    pub filename: *mut libc::c_char,
    pub password: *const libc::c_char,
}
#[c2rust::src_loc = "251:3"]
static mut pkcs11_errstrings: [pkcs11_errstrings; 86] =
    [{
         let mut init =
             pkcs11_errstrings{code: 0 as libc::c_int as libc::c_short,
                               text:
                                   b"ok\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x1 as libc::c_int as libc::c_short,
                               text:
                                   b"cancel\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x2 as libc::c_int as libc::c_short,
                               text:
                                   b"host memory\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x3 as libc::c_int as libc::c_short,
                               text:
                                   b"slot id invalid\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x5 as libc::c_int as libc::c_short,
                               text:
                                   b"general error\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x6 as libc::c_int as libc::c_short,
                               text:
                                   b"function failed\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x7 as libc::c_int as libc::c_short,
                               text:
                                   b"arguments bad\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x8 as libc::c_int as libc::c_short,
                               text:
                                   b"no event\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x9 as libc::c_int as libc::c_short,
                               text:
                                   b"need to create threads\x00" as *const u8
                                       as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0xa as libc::c_int as libc::c_short,
                               text:
                                   b"cant lock\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x10 as libc::c_int as libc::c_short,
                               text:
                                   b"attribute read only\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x11 as libc::c_int as libc::c_short,
                               text:
                                   b"attribute sensitive\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x12 as libc::c_int as libc::c_short,
                               text:
                                   b"attribute type invalid\x00" as *const u8
                                       as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x13 as libc::c_int as libc::c_short,
                               text:
                                   b"attribute value invalid\x00" as *const u8
                                       as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x20 as libc::c_int as libc::c_short,
                               text:
                                   b"data invalid\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x21 as libc::c_int as libc::c_short,
                               text:
                                   b"data len range\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x30 as libc::c_int as libc::c_short,
                               text:
                                   b"device error\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x31 as libc::c_int as libc::c_short,
                               text:
                                   b"device memory\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x32 as libc::c_int as libc::c_short,
                               text:
                                   b"device removed\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x40 as libc::c_int as libc::c_short,
                               text:
                                   b"encrypted data invalid\x00" as *const u8
                                       as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x41 as libc::c_int as libc::c_short,
                               text:
                                   b"encrypted data len range\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x50 as libc::c_int as libc::c_short,
                               text:
                                   b"function canceled\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x51 as libc::c_int as libc::c_short,
                               text:
                                   b"function not parallel\x00" as *const u8
                                       as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x54 as libc::c_int as libc::c_short,
                               text:
                                   b"function not supported\x00" as *const u8
                                       as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x60 as libc::c_int as libc::c_short,
                               text:
                                   b"key handle invalid\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x62 as libc::c_int as libc::c_short,
                               text:
                                   b"key size range\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x63 as libc::c_int as libc::c_short,
                               text:
                                   b"key type inconsistent\x00" as *const u8
                                       as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x64 as libc::c_int as libc::c_short,
                               text:
                                   b"key not needed\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x65 as libc::c_int as libc::c_short,
                               text:
                                   b"key changed\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x66 as libc::c_int as libc::c_short,
                               text:
                                   b"key needed\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x67 as libc::c_int as libc::c_short,
                               text:
                                   b"key indigestible\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x68 as libc::c_int as libc::c_short,
                               text:
                                   b"key function not permitted\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x69 as libc::c_int as libc::c_short,
                               text:
                                   b"key not wrappable\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x6a as libc::c_int as libc::c_short,
                               text:
                                   b"key unextractable\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x70 as libc::c_int as libc::c_short,
                               text:
                                   b"mechanism invalid\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x71 as libc::c_int as libc::c_short,
                               text:
                                   b"mechanism param invalid\x00" as *const u8
                                       as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x82 as libc::c_int as libc::c_short,
                               text:
                                   b"object handle invalid\x00" as *const u8
                                       as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x90 as libc::c_int as libc::c_short,
                               text:
                                   b"operation active\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x91 as libc::c_int as libc::c_short,
                               text:
                                   b"operation not initialized\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0xa0 as libc::c_int as libc::c_short,
                               text:
                                   b"pin incorrect\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0xa1 as libc::c_int as libc::c_short,
                               text:
                                   b"pin invalid\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0xa2 as libc::c_int as libc::c_short,
                               text:
                                   b"pin len range\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0xa3 as libc::c_int as libc::c_short,
                               text:
                                   b"pin expired\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0xa4 as libc::c_int as libc::c_short,
                               text:
                                   b"pin locked\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0xb0 as libc::c_int as libc::c_short,
                               text:
                                   b"session closed\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0xb1 as libc::c_int as libc::c_short,
                               text:
                                   b"session count\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0xb3 as libc::c_int as libc::c_short,
                               text:
                                   b"session handle invalid\x00" as *const u8
                                       as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0xb4 as libc::c_int as libc::c_short,
                               text:
                                   b"session parallel not supported\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0xb5 as libc::c_int as libc::c_short,
                               text:
                                   b"session read only\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0xb6 as libc::c_int as libc::c_short,
                               text:
                                   b"session exists\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0xb7 as libc::c_int as libc::c_short,
                               text:
                                   b"session read only exists\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0xb8 as libc::c_int as libc::c_short,
                               text:
                                   b"session read write so exists\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0xc0 as libc::c_int as libc::c_short,
                               text:
                                   b"signature invalid\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0xc1 as libc::c_int as libc::c_short,
                               text:
                                   b"signature len range\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0xd0 as libc::c_int as libc::c_short,
                               text:
                                   b"template incomplete\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0xd1 as libc::c_int as libc::c_short,
                               text:
                                   b"template inconsistent\x00" as *const u8
                                       as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0xe0 as libc::c_int as libc::c_short,
                               text:
                                   b"token not present\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0xe1 as libc::c_int as libc::c_short,
                               text:
                                   b"token not recognized\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0xe2 as libc::c_int as libc::c_short,
                               text:
                                   b"token write protected\x00" as *const u8
                                       as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0xf0 as libc::c_int as libc::c_short,
                               text:
                                   b"unwrapping key handle invalid\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0xf1 as libc::c_int as libc::c_short,
                               text:
                                   b"unwrapping key size range\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0xf2 as libc::c_int as libc::c_short,
                               text:
                                   b"unwrapping key type inconsistent\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x100 as libc::c_int as libc::c_short,
                               text:
                                   b"user already logged in\x00" as *const u8
                                       as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x101 as libc::c_int as libc::c_short,
                               text:
                                   b"user not logged in\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x102 as libc::c_int as libc::c_short,
                               text:
                                   b"user pin not initialized\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x103 as libc::c_int as libc::c_short,
                               text:
                                   b"user type invalid\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x104 as libc::c_int as libc::c_short,
                               text:
                                   b"user another already logged in\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x105 as libc::c_int as libc::c_short,
                               text:
                                   b"user too many types\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x110 as libc::c_int as libc::c_short,
                               text:
                                   b"wrapped key invalid\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x112 as libc::c_int as libc::c_short,
                               text:
                                   b"wrapped key len range\x00" as *const u8
                                       as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x113 as libc::c_int as libc::c_short,
                               text:
                                   b"wrapping key handle invalid\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x114 as libc::c_int as libc::c_short,
                               text:
                                   b"wrapping key size range\x00" as *const u8
                                       as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x115 as libc::c_int as libc::c_short,
                               text:
                                   b"wrapping key type inconsistent\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x120 as libc::c_int as libc::c_short,
                               text:
                                   b"random seed not supported\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x121 as libc::c_int as libc::c_short,
                               text:
                                   b"random no rng\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x130 as libc::c_int as libc::c_short,
                               text:
                                   b"domain params invalid\x00" as *const u8
                                       as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x150 as libc::c_int as libc::c_short,
                               text:
                                   b"buffer too small\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x160 as libc::c_int as libc::c_short,
                               text:
                                   b"saved state invalid\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x170 as libc::c_int as libc::c_short,
                               text:
                                   b"information sensitive\x00" as *const u8
                                       as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x180 as libc::c_int as libc::c_short,
                               text:
                                   b"state unsaveable\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x190 as libc::c_int as libc::c_short,
                               text:
                                   b"cryptoki not initialized\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x191 as libc::c_int as libc::c_short,
                               text:
                                   b"cryptoki already initialized\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x1a0 as libc::c_int as libc::c_short,
                               text:
                                   b"mutex bad\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x1a1 as libc::c_int as libc::c_short,
                               text:
                                   b"mutex not locked\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: 0x200 as libc::c_int as libc::c_short,
                               text:
                                   b"function rejected\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,};
         init
     },
     {
         let mut init =
             pkcs11_errstrings{code: -(1 as libc::c_int) as libc::c_short,
                               text:
                                   0 as *const libc::c_char as
                                       *mut libc::c_char,};
         init
     }];
/* DH parameters */
#[c2rust::src_loc = "341:16"]
static mut oakley_1024: [uint8_t; 128] =
    [0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xc9 as libc::c_int as uint8_t, 0xf as libc::c_int as uint8_t,
     0xda as libc::c_int as uint8_t, 0xa2 as libc::c_int as uint8_t,
     0x21 as libc::c_int as uint8_t, 0x68 as libc::c_int as uint8_t,
     0xc2 as libc::c_int as uint8_t, 0x34 as libc::c_int as uint8_t,
     0xc4 as libc::c_int as uint8_t, 0xc6 as libc::c_int as uint8_t,
     0x62 as libc::c_int as uint8_t, 0x8b as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0xdc as libc::c_int as uint8_t,
     0x1c as libc::c_int as uint8_t, 0xd1 as libc::c_int as uint8_t,
     0x29 as libc::c_int as uint8_t, 0x2 as libc::c_int as uint8_t,
     0x4e as libc::c_int as uint8_t, 0x8 as libc::c_int as uint8_t,
     0x8a as libc::c_int as uint8_t, 0x67 as libc::c_int as uint8_t,
     0xcc as libc::c_int as uint8_t, 0x74 as libc::c_int as uint8_t,
     0x2 as libc::c_int as uint8_t, 0xb as libc::c_int as uint8_t,
     0xbe as libc::c_int as uint8_t, 0xa6 as libc::c_int as uint8_t,
     0x3b as libc::c_int as uint8_t, 0x13 as libc::c_int as uint8_t,
     0x9b as libc::c_int as uint8_t, 0x22 as libc::c_int as uint8_t,
     0x51 as libc::c_int as uint8_t, 0x4a as libc::c_int as uint8_t,
     0x8 as libc::c_int as uint8_t, 0x79 as libc::c_int as uint8_t,
     0x8e as libc::c_int as uint8_t, 0x34 as libc::c_int as uint8_t,
     0x4 as libc::c_int as uint8_t, 0xdd as libc::c_int as uint8_t,
     0xef as libc::c_int as uint8_t, 0x95 as libc::c_int as uint8_t,
     0x19 as libc::c_int as uint8_t, 0xb3 as libc::c_int as uint8_t,
     0xcd as libc::c_int as uint8_t, 0x3a as libc::c_int as uint8_t,
     0x43 as libc::c_int as uint8_t, 0x1b as libc::c_int as uint8_t,
     0x30 as libc::c_int as uint8_t, 0x2b as libc::c_int as uint8_t,
     0xa as libc::c_int as uint8_t, 0x6d as libc::c_int as uint8_t,
     0xf2 as libc::c_int as uint8_t, 0x5f as libc::c_int as uint8_t,
     0x14 as libc::c_int as uint8_t, 0x37 as libc::c_int as uint8_t,
     0x4f as libc::c_int as uint8_t, 0xe1 as libc::c_int as uint8_t,
     0x35 as libc::c_int as uint8_t, 0x6d as libc::c_int as uint8_t,
     0x6d as libc::c_int as uint8_t, 0x51 as libc::c_int as uint8_t,
     0xc2 as libc::c_int as uint8_t, 0x45 as libc::c_int as uint8_t,
     0xe4 as libc::c_int as uint8_t, 0x85 as libc::c_int as uint8_t,
     0xb5 as libc::c_int as uint8_t, 0x76 as libc::c_int as uint8_t,
     0x62 as libc::c_int as uint8_t, 0x5e as libc::c_int as uint8_t,
     0x7e as libc::c_int as uint8_t, 0xc6 as libc::c_int as uint8_t,
     0xf4 as libc::c_int as uint8_t, 0x4c as libc::c_int as uint8_t,
     0x42 as libc::c_int as uint8_t, 0xe9 as libc::c_int as uint8_t,
     0xa6 as libc::c_int as uint8_t, 0x37 as libc::c_int as uint8_t,
     0xed as libc::c_int as uint8_t, 0x6b as libc::c_int as uint8_t,
     0xb as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0x5c as libc::c_int as uint8_t, 0xb6 as libc::c_int as uint8_t,
     0xf4 as libc::c_int as uint8_t, 0x6 as libc::c_int as uint8_t,
     0xb7 as libc::c_int as uint8_t, 0xed as libc::c_int as uint8_t,
     0xee as libc::c_int as uint8_t, 0x38 as libc::c_int as uint8_t,
     0x6b as libc::c_int as uint8_t, 0xfb as libc::c_int as uint8_t,
     0x5a as libc::c_int as uint8_t, 0x89 as libc::c_int as uint8_t,
     0x9f as libc::c_int as uint8_t, 0xa5 as libc::c_int as uint8_t,
     0xae as libc::c_int as uint8_t, 0x9f as libc::c_int as uint8_t,
     0x24 as libc::c_int as uint8_t, 0x11 as libc::c_int as uint8_t,
     0x7c as libc::c_int as uint8_t, 0x4b as libc::c_int as uint8_t,
     0x1f as libc::c_int as uint8_t, 0xe6 as libc::c_int as uint8_t,
     0x49 as libc::c_int as uint8_t, 0x28 as libc::c_int as uint8_t,
     0x66 as libc::c_int as uint8_t, 0x51 as libc::c_int as uint8_t,
     0xec as libc::c_int as uint8_t, 0xe6 as libc::c_int as uint8_t,
     0x53 as libc::c_int as uint8_t, 0x81 as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t];
#[c2rust::src_loc = "360:16"]
static mut oakley_2048: [uint8_t; 256] =
    [0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xc9 as libc::c_int as uint8_t, 0xf as libc::c_int as uint8_t,
     0xda as libc::c_int as uint8_t, 0xa2 as libc::c_int as uint8_t,
     0x21 as libc::c_int as uint8_t, 0x68 as libc::c_int as uint8_t,
     0xc2 as libc::c_int as uint8_t, 0x34 as libc::c_int as uint8_t,
     0xc4 as libc::c_int as uint8_t, 0xc6 as libc::c_int as uint8_t,
     0x62 as libc::c_int as uint8_t, 0x8b as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0xdc as libc::c_int as uint8_t,
     0x1c as libc::c_int as uint8_t, 0xd1 as libc::c_int as uint8_t,
     0x29 as libc::c_int as uint8_t, 0x2 as libc::c_int as uint8_t,
     0x4e as libc::c_int as uint8_t, 0x8 as libc::c_int as uint8_t,
     0x8a as libc::c_int as uint8_t, 0x67 as libc::c_int as uint8_t,
     0xcc as libc::c_int as uint8_t, 0x74 as libc::c_int as uint8_t,
     0x2 as libc::c_int as uint8_t, 0xb as libc::c_int as uint8_t,
     0xbe as libc::c_int as uint8_t, 0xa6 as libc::c_int as uint8_t,
     0x3b as libc::c_int as uint8_t, 0x13 as libc::c_int as uint8_t,
     0x9b as libc::c_int as uint8_t, 0x22 as libc::c_int as uint8_t,
     0x51 as libc::c_int as uint8_t, 0x4a as libc::c_int as uint8_t,
     0x8 as libc::c_int as uint8_t, 0x79 as libc::c_int as uint8_t,
     0x8e as libc::c_int as uint8_t, 0x34 as libc::c_int as uint8_t,
     0x4 as libc::c_int as uint8_t, 0xdd as libc::c_int as uint8_t,
     0xef as libc::c_int as uint8_t, 0x95 as libc::c_int as uint8_t,
     0x19 as libc::c_int as uint8_t, 0xb3 as libc::c_int as uint8_t,
     0xcd as libc::c_int as uint8_t, 0x3a as libc::c_int as uint8_t,
     0x43 as libc::c_int as uint8_t, 0x1b as libc::c_int as uint8_t,
     0x30 as libc::c_int as uint8_t, 0x2b as libc::c_int as uint8_t,
     0xa as libc::c_int as uint8_t, 0x6d as libc::c_int as uint8_t,
     0xf2 as libc::c_int as uint8_t, 0x5f as libc::c_int as uint8_t,
     0x14 as libc::c_int as uint8_t, 0x37 as libc::c_int as uint8_t,
     0x4f as libc::c_int as uint8_t, 0xe1 as libc::c_int as uint8_t,
     0x35 as libc::c_int as uint8_t, 0x6d as libc::c_int as uint8_t,
     0x6d as libc::c_int as uint8_t, 0x51 as libc::c_int as uint8_t,
     0xc2 as libc::c_int as uint8_t, 0x45 as libc::c_int as uint8_t,
     0xe4 as libc::c_int as uint8_t, 0x85 as libc::c_int as uint8_t,
     0xb5 as libc::c_int as uint8_t, 0x76 as libc::c_int as uint8_t,
     0x62 as libc::c_int as uint8_t, 0x5e as libc::c_int as uint8_t,
     0x7e as libc::c_int as uint8_t, 0xc6 as libc::c_int as uint8_t,
     0xf4 as libc::c_int as uint8_t, 0x4c as libc::c_int as uint8_t,
     0x42 as libc::c_int as uint8_t, 0xe9 as libc::c_int as uint8_t,
     0xa6 as libc::c_int as uint8_t, 0x37 as libc::c_int as uint8_t,
     0xed as libc::c_int as uint8_t, 0x6b as libc::c_int as uint8_t,
     0xb as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0x5c as libc::c_int as uint8_t, 0xb6 as libc::c_int as uint8_t,
     0xf4 as libc::c_int as uint8_t, 0x6 as libc::c_int as uint8_t,
     0xb7 as libc::c_int as uint8_t, 0xed as libc::c_int as uint8_t,
     0xee as libc::c_int as uint8_t, 0x38 as libc::c_int as uint8_t,
     0x6b as libc::c_int as uint8_t, 0xfb as libc::c_int as uint8_t,
     0x5a as libc::c_int as uint8_t, 0x89 as libc::c_int as uint8_t,
     0x9f as libc::c_int as uint8_t, 0xa5 as libc::c_int as uint8_t,
     0xae as libc::c_int as uint8_t, 0x9f as libc::c_int as uint8_t,
     0x24 as libc::c_int as uint8_t, 0x11 as libc::c_int as uint8_t,
     0x7c as libc::c_int as uint8_t, 0x4b as libc::c_int as uint8_t,
     0x1f as libc::c_int as uint8_t, 0xe6 as libc::c_int as uint8_t,
     0x49 as libc::c_int as uint8_t, 0x28 as libc::c_int as uint8_t,
     0x66 as libc::c_int as uint8_t, 0x51 as libc::c_int as uint8_t,
     0xec as libc::c_int as uint8_t, 0xe4 as libc::c_int as uint8_t,
     0x5b as libc::c_int as uint8_t, 0x3d as libc::c_int as uint8_t,
     0xc2 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0x7c as libc::c_int as uint8_t, 0xb8 as libc::c_int as uint8_t,
     0xa1 as libc::c_int as uint8_t, 0x63 as libc::c_int as uint8_t,
     0xbf as libc::c_int as uint8_t, 0x5 as libc::c_int as uint8_t,
     0x98 as libc::c_int as uint8_t, 0xda as libc::c_int as uint8_t,
     0x48 as libc::c_int as uint8_t, 0x36 as libc::c_int as uint8_t,
     0x1c as libc::c_int as uint8_t, 0x55 as libc::c_int as uint8_t,
     0xd3 as libc::c_int as uint8_t, 0x9a as libc::c_int as uint8_t,
     0x69 as libc::c_int as uint8_t, 0x16 as libc::c_int as uint8_t,
     0x3f as libc::c_int as uint8_t, 0xa8 as libc::c_int as uint8_t,
     0xfd as libc::c_int as uint8_t, 0x24 as libc::c_int as uint8_t,
     0xcf as libc::c_int as uint8_t, 0x5f as libc::c_int as uint8_t,
     0x83 as libc::c_int as uint8_t, 0x65 as libc::c_int as uint8_t,
     0x5d as libc::c_int as uint8_t, 0x23 as libc::c_int as uint8_t,
     0xdc as libc::c_int as uint8_t, 0xa3 as libc::c_int as uint8_t,
     0xad as libc::c_int as uint8_t, 0x96 as libc::c_int as uint8_t,
     0x1c as libc::c_int as uint8_t, 0x62 as libc::c_int as uint8_t,
     0xf3 as libc::c_int as uint8_t, 0x56 as libc::c_int as uint8_t,
     0x20 as libc::c_int as uint8_t, 0x85 as libc::c_int as uint8_t,
     0x52 as libc::c_int as uint8_t, 0xbb as libc::c_int as uint8_t,
     0x9e as libc::c_int as uint8_t, 0xd5 as libc::c_int as uint8_t,
     0x29 as libc::c_int as uint8_t, 0x7 as libc::c_int as uint8_t,
     0x70 as libc::c_int as uint8_t, 0x96 as libc::c_int as uint8_t,
     0x96 as libc::c_int as uint8_t, 0x6d as libc::c_int as uint8_t,
     0x67 as libc::c_int as uint8_t, 0xc as libc::c_int as uint8_t,
     0x35 as libc::c_int as uint8_t, 0x4e as libc::c_int as uint8_t,
     0x4a as libc::c_int as uint8_t, 0xbc as libc::c_int as uint8_t,
     0x98 as libc::c_int as uint8_t, 0x4 as libc::c_int as uint8_t,
     0xf1 as libc::c_int as uint8_t, 0x74 as libc::c_int as uint8_t,
     0x6c as libc::c_int as uint8_t, 0x8 as libc::c_int as uint8_t,
     0xca as libc::c_int as uint8_t, 0x18 as libc::c_int as uint8_t,
     0x21 as libc::c_int as uint8_t, 0x7c as libc::c_int as uint8_t,
     0x32 as libc::c_int as uint8_t, 0x90 as libc::c_int as uint8_t,
     0x5e as libc::c_int as uint8_t, 0x46 as libc::c_int as uint8_t,
     0x2e as libc::c_int as uint8_t, 0x36 as libc::c_int as uint8_t,
     0xce as libc::c_int as uint8_t, 0x3b as libc::c_int as uint8_t,
     0xe3 as libc::c_int as uint8_t, 0x9e as libc::c_int as uint8_t,
     0x77 as libc::c_int as uint8_t, 0x2c as libc::c_int as uint8_t,
     0x18 as libc::c_int as uint8_t, 0xe as libc::c_int as uint8_t,
     0x86 as libc::c_int as uint8_t, 0x3 as libc::c_int as uint8_t,
     0x9b as libc::c_int as uint8_t, 0x27 as libc::c_int as uint8_t,
     0x83 as libc::c_int as uint8_t, 0xa2 as libc::c_int as uint8_t,
     0xec as libc::c_int as uint8_t, 0x7 as libc::c_int as uint8_t,
     0xa2 as libc::c_int as uint8_t, 0x8f as libc::c_int as uint8_t,
     0xb5 as libc::c_int as uint8_t, 0xc5 as libc::c_int as uint8_t,
     0x5d as libc::c_int as uint8_t, 0xf0 as libc::c_int as uint8_t,
     0x6f as libc::c_int as uint8_t, 0x4c as libc::c_int as uint8_t,
     0x52 as libc::c_int as uint8_t, 0xc9 as libc::c_int as uint8_t,
     0xde as libc::c_int as uint8_t, 0x2b as libc::c_int as uint8_t,
     0xcb as libc::c_int as uint8_t, 0xf6 as libc::c_int as uint8_t,
     0x95 as libc::c_int as uint8_t, 0x58 as libc::c_int as uint8_t,
     0x17 as libc::c_int as uint8_t, 0x18 as libc::c_int as uint8_t,
     0x39 as libc::c_int as uint8_t, 0x95 as libc::c_int as uint8_t,
     0x49 as libc::c_int as uint8_t, 0x7c as libc::c_int as uint8_t,
     0xea as libc::c_int as uint8_t, 0x95 as libc::c_int as uint8_t,
     0x6a as libc::c_int as uint8_t, 0xe5 as libc::c_int as uint8_t,
     0x15 as libc::c_int as uint8_t, 0xd2 as libc::c_int as uint8_t,
     0x26 as libc::c_int as uint8_t, 0x18 as libc::c_int as uint8_t,
     0x98 as libc::c_int as uint8_t, 0xfa as libc::c_int as uint8_t,
     0x5 as libc::c_int as uint8_t, 0x10 as libc::c_int as uint8_t,
     0x15 as libc::c_int as uint8_t, 0x72 as libc::c_int as uint8_t,
     0x8e as libc::c_int as uint8_t, 0x5a as libc::c_int as uint8_t,
     0x8a as libc::c_int as uint8_t, 0xac as libc::c_int as uint8_t,
     0xaa as libc::c_int as uint8_t, 0x68 as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t];
#[c2rust::src_loc = "395:16"]
static mut oakley_4096: [uint8_t; 512] =
    [0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xc9 as libc::c_int as uint8_t, 0xf as libc::c_int as uint8_t,
     0xda as libc::c_int as uint8_t, 0xa2 as libc::c_int as uint8_t,
     0x21 as libc::c_int as uint8_t, 0x68 as libc::c_int as uint8_t,
     0xc2 as libc::c_int as uint8_t, 0x34 as libc::c_int as uint8_t,
     0xc4 as libc::c_int as uint8_t, 0xc6 as libc::c_int as uint8_t,
     0x62 as libc::c_int as uint8_t, 0x8b as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0xdc as libc::c_int as uint8_t,
     0x1c as libc::c_int as uint8_t, 0xd1 as libc::c_int as uint8_t,
     0x29 as libc::c_int as uint8_t, 0x2 as libc::c_int as uint8_t,
     0x4e as libc::c_int as uint8_t, 0x8 as libc::c_int as uint8_t,
     0x8a as libc::c_int as uint8_t, 0x67 as libc::c_int as uint8_t,
     0xcc as libc::c_int as uint8_t, 0x74 as libc::c_int as uint8_t,
     0x2 as libc::c_int as uint8_t, 0xb as libc::c_int as uint8_t,
     0xbe as libc::c_int as uint8_t, 0xa6 as libc::c_int as uint8_t,
     0x3b as libc::c_int as uint8_t, 0x13 as libc::c_int as uint8_t,
     0x9b as libc::c_int as uint8_t, 0x22 as libc::c_int as uint8_t,
     0x51 as libc::c_int as uint8_t, 0x4a as libc::c_int as uint8_t,
     0x8 as libc::c_int as uint8_t, 0x79 as libc::c_int as uint8_t,
     0x8e as libc::c_int as uint8_t, 0x34 as libc::c_int as uint8_t,
     0x4 as libc::c_int as uint8_t, 0xdd as libc::c_int as uint8_t,
     0xef as libc::c_int as uint8_t, 0x95 as libc::c_int as uint8_t,
     0x19 as libc::c_int as uint8_t, 0xb3 as libc::c_int as uint8_t,
     0xcd as libc::c_int as uint8_t, 0x3a as libc::c_int as uint8_t,
     0x43 as libc::c_int as uint8_t, 0x1b as libc::c_int as uint8_t,
     0x30 as libc::c_int as uint8_t, 0x2b as libc::c_int as uint8_t,
     0xa as libc::c_int as uint8_t, 0x6d as libc::c_int as uint8_t,
     0xf2 as libc::c_int as uint8_t, 0x5f as libc::c_int as uint8_t,
     0x14 as libc::c_int as uint8_t, 0x37 as libc::c_int as uint8_t,
     0x4f as libc::c_int as uint8_t, 0xe1 as libc::c_int as uint8_t,
     0x35 as libc::c_int as uint8_t, 0x6d as libc::c_int as uint8_t,
     0x6d as libc::c_int as uint8_t, 0x51 as libc::c_int as uint8_t,
     0xc2 as libc::c_int as uint8_t, 0x45 as libc::c_int as uint8_t,
     0xe4 as libc::c_int as uint8_t, 0x85 as libc::c_int as uint8_t,
     0xb5 as libc::c_int as uint8_t, 0x76 as libc::c_int as uint8_t,
     0x62 as libc::c_int as uint8_t, 0x5e as libc::c_int as uint8_t,
     0x7e as libc::c_int as uint8_t, 0xc6 as libc::c_int as uint8_t,
     0xf4 as libc::c_int as uint8_t, 0x4c as libc::c_int as uint8_t,
     0x42 as libc::c_int as uint8_t, 0xe9 as libc::c_int as uint8_t,
     0xa6 as libc::c_int as uint8_t, 0x37 as libc::c_int as uint8_t,
     0xed as libc::c_int as uint8_t, 0x6b as libc::c_int as uint8_t,
     0xb as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0x5c as libc::c_int as uint8_t, 0xb6 as libc::c_int as uint8_t,
     0xf4 as libc::c_int as uint8_t, 0x6 as libc::c_int as uint8_t,
     0xb7 as libc::c_int as uint8_t, 0xed as libc::c_int as uint8_t,
     0xee as libc::c_int as uint8_t, 0x38 as libc::c_int as uint8_t,
     0x6b as libc::c_int as uint8_t, 0xfb as libc::c_int as uint8_t,
     0x5a as libc::c_int as uint8_t, 0x89 as libc::c_int as uint8_t,
     0x9f as libc::c_int as uint8_t, 0xa5 as libc::c_int as uint8_t,
     0xae as libc::c_int as uint8_t, 0x9f as libc::c_int as uint8_t,
     0x24 as libc::c_int as uint8_t, 0x11 as libc::c_int as uint8_t,
     0x7c as libc::c_int as uint8_t, 0x4b as libc::c_int as uint8_t,
     0x1f as libc::c_int as uint8_t, 0xe6 as libc::c_int as uint8_t,
     0x49 as libc::c_int as uint8_t, 0x28 as libc::c_int as uint8_t,
     0x66 as libc::c_int as uint8_t, 0x51 as libc::c_int as uint8_t,
     0xec as libc::c_int as uint8_t, 0xe4 as libc::c_int as uint8_t,
     0x5b as libc::c_int as uint8_t, 0x3d as libc::c_int as uint8_t,
     0xc2 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0x7c as libc::c_int as uint8_t, 0xb8 as libc::c_int as uint8_t,
     0xa1 as libc::c_int as uint8_t, 0x63 as libc::c_int as uint8_t,
     0xbf as libc::c_int as uint8_t, 0x5 as libc::c_int as uint8_t,
     0x98 as libc::c_int as uint8_t, 0xda as libc::c_int as uint8_t,
     0x48 as libc::c_int as uint8_t, 0x36 as libc::c_int as uint8_t,
     0x1c as libc::c_int as uint8_t, 0x55 as libc::c_int as uint8_t,
     0xd3 as libc::c_int as uint8_t, 0x9a as libc::c_int as uint8_t,
     0x69 as libc::c_int as uint8_t, 0x16 as libc::c_int as uint8_t,
     0x3f as libc::c_int as uint8_t, 0xa8 as libc::c_int as uint8_t,
     0xfd as libc::c_int as uint8_t, 0x24 as libc::c_int as uint8_t,
     0xcf as libc::c_int as uint8_t, 0x5f as libc::c_int as uint8_t,
     0x83 as libc::c_int as uint8_t, 0x65 as libc::c_int as uint8_t,
     0x5d as libc::c_int as uint8_t, 0x23 as libc::c_int as uint8_t,
     0xdc as libc::c_int as uint8_t, 0xa3 as libc::c_int as uint8_t,
     0xad as libc::c_int as uint8_t, 0x96 as libc::c_int as uint8_t,
     0x1c as libc::c_int as uint8_t, 0x62 as libc::c_int as uint8_t,
     0xf3 as libc::c_int as uint8_t, 0x56 as libc::c_int as uint8_t,
     0x20 as libc::c_int as uint8_t, 0x85 as libc::c_int as uint8_t,
     0x52 as libc::c_int as uint8_t, 0xbb as libc::c_int as uint8_t,
     0x9e as libc::c_int as uint8_t, 0xd5 as libc::c_int as uint8_t,
     0x29 as libc::c_int as uint8_t, 0x7 as libc::c_int as uint8_t,
     0x70 as libc::c_int as uint8_t, 0x96 as libc::c_int as uint8_t,
     0x96 as libc::c_int as uint8_t, 0x6d as libc::c_int as uint8_t,
     0x67 as libc::c_int as uint8_t, 0xc as libc::c_int as uint8_t,
     0x35 as libc::c_int as uint8_t, 0x4e as libc::c_int as uint8_t,
     0x4a as libc::c_int as uint8_t, 0xbc as libc::c_int as uint8_t,
     0x98 as libc::c_int as uint8_t, 0x4 as libc::c_int as uint8_t,
     0xf1 as libc::c_int as uint8_t, 0x74 as libc::c_int as uint8_t,
     0x6c as libc::c_int as uint8_t, 0x8 as libc::c_int as uint8_t,
     0xca as libc::c_int as uint8_t, 0x18 as libc::c_int as uint8_t,
     0x21 as libc::c_int as uint8_t, 0x7c as libc::c_int as uint8_t,
     0x32 as libc::c_int as uint8_t, 0x90 as libc::c_int as uint8_t,
     0x5e as libc::c_int as uint8_t, 0x46 as libc::c_int as uint8_t,
     0x2e as libc::c_int as uint8_t, 0x36 as libc::c_int as uint8_t,
     0xce as libc::c_int as uint8_t, 0x3b as libc::c_int as uint8_t,
     0xe3 as libc::c_int as uint8_t, 0x9e as libc::c_int as uint8_t,
     0x77 as libc::c_int as uint8_t, 0x2c as libc::c_int as uint8_t,
     0x18 as libc::c_int as uint8_t, 0xe as libc::c_int as uint8_t,
     0x86 as libc::c_int as uint8_t, 0x3 as libc::c_int as uint8_t,
     0x9b as libc::c_int as uint8_t, 0x27 as libc::c_int as uint8_t,
     0x83 as libc::c_int as uint8_t, 0xa2 as libc::c_int as uint8_t,
     0xec as libc::c_int as uint8_t, 0x7 as libc::c_int as uint8_t,
     0xa2 as libc::c_int as uint8_t, 0x8f as libc::c_int as uint8_t,
     0xb5 as libc::c_int as uint8_t, 0xc5 as libc::c_int as uint8_t,
     0x5d as libc::c_int as uint8_t, 0xf0 as libc::c_int as uint8_t,
     0x6f as libc::c_int as uint8_t, 0x4c as libc::c_int as uint8_t,
     0x52 as libc::c_int as uint8_t, 0xc9 as libc::c_int as uint8_t,
     0xde as libc::c_int as uint8_t, 0x2b as libc::c_int as uint8_t,
     0xcb as libc::c_int as uint8_t, 0xf6 as libc::c_int as uint8_t,
     0x95 as libc::c_int as uint8_t, 0x58 as libc::c_int as uint8_t,
     0x17 as libc::c_int as uint8_t, 0x18 as libc::c_int as uint8_t,
     0x39 as libc::c_int as uint8_t, 0x95 as libc::c_int as uint8_t,
     0x49 as libc::c_int as uint8_t, 0x7c as libc::c_int as uint8_t,
     0xea as libc::c_int as uint8_t, 0x95 as libc::c_int as uint8_t,
     0x6a as libc::c_int as uint8_t, 0xe5 as libc::c_int as uint8_t,
     0x15 as libc::c_int as uint8_t, 0xd2 as libc::c_int as uint8_t,
     0x26 as libc::c_int as uint8_t, 0x18 as libc::c_int as uint8_t,
     0x98 as libc::c_int as uint8_t, 0xfa as libc::c_int as uint8_t,
     0x5 as libc::c_int as uint8_t, 0x10 as libc::c_int as uint8_t,
     0x15 as libc::c_int as uint8_t, 0x72 as libc::c_int as uint8_t,
     0x8e as libc::c_int as uint8_t, 0x5a as libc::c_int as uint8_t,
     0x8a as libc::c_int as uint8_t, 0xaa as libc::c_int as uint8_t,
     0xc4 as libc::c_int as uint8_t, 0x2d as libc::c_int as uint8_t,
     0xad as libc::c_int as uint8_t, 0x33 as libc::c_int as uint8_t,
     0x17 as libc::c_int as uint8_t, 0xd as libc::c_int as uint8_t,
     0x4 as libc::c_int as uint8_t, 0x50 as libc::c_int as uint8_t,
     0x7a as libc::c_int as uint8_t, 0x33 as libc::c_int as uint8_t,
     0xa8 as libc::c_int as uint8_t, 0x55 as libc::c_int as uint8_t,
     0x21 as libc::c_int as uint8_t, 0xab as libc::c_int as uint8_t,
     0xdf as libc::c_int as uint8_t, 0x1c as libc::c_int as uint8_t,
     0xba as libc::c_int as uint8_t, 0x64 as libc::c_int as uint8_t,
     0xec as libc::c_int as uint8_t, 0xfb as libc::c_int as uint8_t,
     0x85 as libc::c_int as uint8_t, 0x4 as libc::c_int as uint8_t,
     0x58 as libc::c_int as uint8_t, 0xdb as libc::c_int as uint8_t,
     0xef as libc::c_int as uint8_t, 0xa as libc::c_int as uint8_t,
     0x8a as libc::c_int as uint8_t, 0xea as libc::c_int as uint8_t,
     0x71 as libc::c_int as uint8_t, 0x57 as libc::c_int as uint8_t,
     0x5d as libc::c_int as uint8_t, 0x6 as libc::c_int as uint8_t,
     0xc as libc::c_int as uint8_t, 0x7d as libc::c_int as uint8_t,
     0xb3 as libc::c_int as uint8_t, 0x97 as libc::c_int as uint8_t,
     0xf as libc::c_int as uint8_t, 0x85 as libc::c_int as uint8_t,
     0xa6 as libc::c_int as uint8_t, 0xe1 as libc::c_int as uint8_t,
     0xe4 as libc::c_int as uint8_t, 0xc7 as libc::c_int as uint8_t,
     0xab as libc::c_int as uint8_t, 0xf5 as libc::c_int as uint8_t,
     0xae as libc::c_int as uint8_t, 0x8c as libc::c_int as uint8_t,
     0xdb as libc::c_int as uint8_t, 0x9 as libc::c_int as uint8_t,
     0x33 as libc::c_int as uint8_t, 0xd7 as libc::c_int as uint8_t,
     0x1e as libc::c_int as uint8_t, 0x8c as libc::c_int as uint8_t,
     0x94 as libc::c_int as uint8_t, 0xe0 as libc::c_int as uint8_t,
     0x4a as libc::c_int as uint8_t, 0x25 as libc::c_int as uint8_t,
     0x61 as libc::c_int as uint8_t, 0x9d as libc::c_int as uint8_t,
     0xce as libc::c_int as uint8_t, 0xe3 as libc::c_int as uint8_t,
     0xd2 as libc::c_int as uint8_t, 0x26 as libc::c_int as uint8_t,
     0x1a as libc::c_int as uint8_t, 0xd2 as libc::c_int as uint8_t,
     0xee as libc::c_int as uint8_t, 0x6b as libc::c_int as uint8_t,
     0xf1 as libc::c_int as uint8_t, 0x2f as libc::c_int as uint8_t,
     0xfa as libc::c_int as uint8_t, 0x6 as libc::c_int as uint8_t,
     0xd9 as libc::c_int as uint8_t, 0x8a as libc::c_int as uint8_t,
     0x8 as libc::c_int as uint8_t, 0x64 as libc::c_int as uint8_t,
     0xd8 as libc::c_int as uint8_t, 0x76 as libc::c_int as uint8_t,
     0x2 as libc::c_int as uint8_t, 0x73 as libc::c_int as uint8_t,
     0x3e as libc::c_int as uint8_t, 0xc8 as libc::c_int as uint8_t,
     0x6a as libc::c_int as uint8_t, 0x64 as libc::c_int as uint8_t,
     0x52 as libc::c_int as uint8_t, 0x1f as libc::c_int as uint8_t,
     0x2b as libc::c_int as uint8_t, 0x18 as libc::c_int as uint8_t,
     0x17 as libc::c_int as uint8_t, 0x7b as libc::c_int as uint8_t,
     0x20 as libc::c_int as uint8_t, 0xc as libc::c_int as uint8_t,
     0xbb as libc::c_int as uint8_t, 0xe1 as libc::c_int as uint8_t,
     0x17 as libc::c_int as uint8_t, 0x57 as libc::c_int as uint8_t,
     0x7a as libc::c_int as uint8_t, 0x61 as libc::c_int as uint8_t,
     0x5d as libc::c_int as uint8_t, 0x6c as libc::c_int as uint8_t,
     0x77 as libc::c_int as uint8_t, 0x9 as libc::c_int as uint8_t,
     0x88 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0xba as libc::c_int as uint8_t, 0xd9 as libc::c_int as uint8_t,
     0x46 as libc::c_int as uint8_t, 0xe2 as libc::c_int as uint8_t,
     0x8 as libc::c_int as uint8_t, 0xe2 as libc::c_int as uint8_t,
     0x4f as libc::c_int as uint8_t, 0xa0 as libc::c_int as uint8_t,
     0x74 as libc::c_int as uint8_t, 0xe5 as libc::c_int as uint8_t,
     0xab as libc::c_int as uint8_t, 0x31 as libc::c_int as uint8_t,
     0x43 as libc::c_int as uint8_t, 0xdb as libc::c_int as uint8_t,
     0x5b as libc::c_int as uint8_t, 0xfc as libc::c_int as uint8_t,
     0xe0 as libc::c_int as uint8_t, 0xfd as libc::c_int as uint8_t,
     0x10 as libc::c_int as uint8_t, 0x8e as libc::c_int as uint8_t,
     0x4b as libc::c_int as uint8_t, 0x82 as libc::c_int as uint8_t,
     0xd1 as libc::c_int as uint8_t, 0x20 as libc::c_int as uint8_t,
     0xa9 as libc::c_int as uint8_t, 0x21 as libc::c_int as uint8_t,
     0x8 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t,
     0x1a as libc::c_int as uint8_t, 0x72 as libc::c_int as uint8_t,
     0x3c as libc::c_int as uint8_t, 0x12 as libc::c_int as uint8_t,
     0xa7 as libc::c_int as uint8_t, 0x87 as libc::c_int as uint8_t,
     0xe6 as libc::c_int as uint8_t, 0xd7 as libc::c_int as uint8_t,
     0x88 as libc::c_int as uint8_t, 0x71 as libc::c_int as uint8_t,
     0x9a as libc::c_int as uint8_t, 0x10 as libc::c_int as uint8_t,
     0xbd as libc::c_int as uint8_t, 0xba as libc::c_int as uint8_t,
     0x5b as libc::c_int as uint8_t, 0x26 as libc::c_int as uint8_t,
     0x99 as libc::c_int as uint8_t, 0xc3 as libc::c_int as uint8_t,
     0x27 as libc::c_int as uint8_t, 0x18 as libc::c_int as uint8_t,
     0x6a as libc::c_int as uint8_t, 0xf4 as libc::c_int as uint8_t,
     0xe2 as libc::c_int as uint8_t, 0x3c as libc::c_int as uint8_t,
     0x1a as libc::c_int as uint8_t, 0x94 as libc::c_int as uint8_t,
     0x68 as libc::c_int as uint8_t, 0x34 as libc::c_int as uint8_t,
     0xb6 as libc::c_int as uint8_t, 0x15 as libc::c_int as uint8_t,
     0xb as libc::c_int as uint8_t, 0xda as libc::c_int as uint8_t,
     0x25 as libc::c_int as uint8_t, 0x83 as libc::c_int as uint8_t,
     0xe9 as libc::c_int as uint8_t, 0xca as libc::c_int as uint8_t,
     0x2a as libc::c_int as uint8_t, 0xd4 as libc::c_int as uint8_t,
     0x4c as libc::c_int as uint8_t, 0xe8 as libc::c_int as uint8_t,
     0xdb as libc::c_int as uint8_t, 0xbb as libc::c_int as uint8_t,
     0xc2 as libc::c_int as uint8_t, 0xdb as libc::c_int as uint8_t,
     0x4 as libc::c_int as uint8_t, 0xde as libc::c_int as uint8_t,
     0x8e as libc::c_int as uint8_t, 0xf9 as libc::c_int as uint8_t,
     0x2e as libc::c_int as uint8_t, 0x8e as libc::c_int as uint8_t,
     0xfc as libc::c_int as uint8_t, 0x14 as libc::c_int as uint8_t,
     0x1f as libc::c_int as uint8_t, 0xbe as libc::c_int as uint8_t,
     0xca as libc::c_int as uint8_t, 0xa6 as libc::c_int as uint8_t,
     0x28 as libc::c_int as uint8_t, 0x7c as libc::c_int as uint8_t,
     0x59 as libc::c_int as uint8_t, 0x47 as libc::c_int as uint8_t,
     0x4e as libc::c_int as uint8_t, 0x6b as libc::c_int as uint8_t,
     0xc0 as libc::c_int as uint8_t, 0x5d as libc::c_int as uint8_t,
     0x99 as libc::c_int as uint8_t, 0xb2 as libc::c_int as uint8_t,
     0x96 as libc::c_int as uint8_t, 0x4f as libc::c_int as uint8_t,
     0xa0 as libc::c_int as uint8_t, 0x90 as libc::c_int as uint8_t,
     0xc3 as libc::c_int as uint8_t, 0xa2 as libc::c_int as uint8_t,
     0x23 as libc::c_int as uint8_t, 0x3b as libc::c_int as uint8_t,
     0xa1 as libc::c_int as uint8_t, 0x86 as libc::c_int as uint8_t,
     0x51 as libc::c_int as uint8_t, 0x5b as libc::c_int as uint8_t,
     0xe7 as libc::c_int as uint8_t, 0xed as libc::c_int as uint8_t,
     0x1f as libc::c_int as uint8_t, 0x61 as libc::c_int as uint8_t,
     0x29 as libc::c_int as uint8_t, 0x70 as libc::c_int as uint8_t,
     0xce as libc::c_int as uint8_t, 0xe2 as libc::c_int as uint8_t,
     0xd7 as libc::c_int as uint8_t, 0xaf as libc::c_int as uint8_t,
     0xb8 as libc::c_int as uint8_t, 0x1b as libc::c_int as uint8_t,
     0xdd as libc::c_int as uint8_t, 0x76 as libc::c_int as uint8_t,
     0x21 as libc::c_int as uint8_t, 0x70 as libc::c_int as uint8_t,
     0x48 as libc::c_int as uint8_t, 0x1c as libc::c_int as uint8_t,
     0xd0 as libc::c_int as uint8_t, 0x6 as libc::c_int as uint8_t,
     0x91 as libc::c_int as uint8_t, 0x27 as libc::c_int as uint8_t,
     0xd5 as libc::c_int as uint8_t, 0xb0 as libc::c_int as uint8_t,
     0x5a as libc::c_int as uint8_t, 0xa9 as libc::c_int as uint8_t,
     0x93 as libc::c_int as uint8_t, 0xb4 as libc::c_int as uint8_t,
     0xea as libc::c_int as uint8_t, 0x98 as libc::c_int as uint8_t,
     0x8d as libc::c_int as uint8_t, 0x8f as libc::c_int as uint8_t,
     0xdd as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x86 as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xb7 as libc::c_int as uint8_t, 0xdc as libc::c_int as uint8_t,
     0x90 as libc::c_int as uint8_t, 0xa6 as libc::c_int as uint8_t,
     0xc0 as libc::c_int as uint8_t, 0x8f as libc::c_int as uint8_t,
     0x4d as libc::c_int as uint8_t, 0xf4 as libc::c_int as uint8_t,
     0x35 as libc::c_int as uint8_t, 0xc9 as libc::c_int as uint8_t,
     0x34 as libc::c_int as uint8_t, 0x6 as libc::c_int as uint8_t,
     0x31 as libc::c_int as uint8_t, 0x99 as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t];
#[c2rust::src_loc = "462:1"]
unsafe extern "C" fn pkinit_openssl_init__aux() {
    pkinit_openssl_init__once.did_run = 1 as libc::c_int;
    pkinit_openssl_init__once.error = pkinit_openssl_init();
}
#[c2rust::src_loc = "462:1"]
static mut pkinit_openssl_init__once: k5_init_t =
    unsafe {
        {
            let mut init =
                k5_init_t{once:
                              {
                                  let mut init =
                                      k5_once_t{o: 0 as libc::c_int,
                                                n:
                                                    2 as libc::c_int as
                                                        k5_os_nothread_once_t,};
                                  init
                              },
                          error: 0 as libc::c_int,
                          did_run: 0 as libc::c_int,
                          fn_0:
                              Some(pkinit_openssl_init__aux as
                                       unsafe extern "C" fn() -> ()),};
            init
        }
    };
/*
 * Set an error string containing the formatted arguments and the first pending
 * OpenSSL error.  Write the formatted arguments and all pending OpenSSL error
 * messages to the trace log.  Return code, or KRB5KDC_ERR_PREAUTH_FAILED if
 * code is 0.
 */
#[c2rust::src_loc = "477:1"]
unsafe extern "C" fn oerr(mut context: krb5_context,
                          mut code: krb5_error_code,
                          mut fmt: *const libc::c_char, mut args: ...)
 -> krb5_error_code {
    let mut err: libc::c_ulong = 0;
    let mut ap: ::std::ffi::VaListImpl;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut r: libc::c_int = 0;
    if code == 0 { code = -(1765328360 as libc::c_long) as krb5_error_code }
    ap = args.clone();
    r = vasprintf(&mut str, fmt, ap.as_va_list());
    if r < 0 as libc::c_int { return code }
    err = ERR_peek_error();
    if err != 0 {
        krb5_set_error_message(context, code,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"%s: %s\x00" as *const u8 as
                                            *const libc::c_char), str,
                               ERR_reason_error_string(err));
    } else {
        krb5_set_error_message(context, code,
                               b"%s\x00" as *const u8 as *const libc::c_char,
                               str);
    }
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"PKINIT OpenSSL error: {str}\x00" as *const u8 as
                          *const libc::c_char, str);
    }
    loop  {
        err = ERR_get_error();
        if !(err != 0 as libc::c_int as libc::c_ulong) { break ; }
        ERR_error_string_n(err, buf.as_mut_ptr(),
                           ::std::mem::size_of::<[libc::c_char; 128]>() as
                               libc::c_ulong);
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"PKINIT OpenSSL error: {str}\x00" as *const u8 as
                              *const libc::c_char, buf.as_mut_ptr());
        }
    }
    free(str as *mut libc::c_void);
    return code;
}
/*
 * Set an appropriate error string containing msg for a certificate
 * verification failure from certctx.  Write the message and all pending
 * OpenSSL error messages to the trace log.  Return code, or
 * KRB5KDC_ERR_PREAUTH_FAILED if code is 0.
 */
#[c2rust::src_loc = "518:1"]
unsafe extern "C" fn oerr_cert(mut context: krb5_context,
                               mut code: krb5_error_code,
                               mut certctx: *mut X509_STORE_CTX,
                               mut msg: *const libc::c_char)
 -> krb5_error_code {
    let mut depth: libc::c_int = X509_STORE_CTX_get_error_depth(certctx);
    let mut err: libc::c_int = X509_STORE_CTX_get_error(certctx);
    let mut errstr: *const libc::c_char =
        X509_verify_cert_error_string(err as libc::c_long);
    return oerr(context, code,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"%s (depth %d): %s\x00" as *const u8 as
                             *const libc::c_char), msg, depth, errstr);
}
#[no_mangle]
#[c2rust::src_loc = "529:1"]
pub unsafe extern "C" fn pkinit_init_plg_crypto(mut cryptoctx:
                                                    *mut pkinit_plg_crypto_context)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut ctx: pkinit_plg_crypto_context = 0 as pkinit_plg_crypto_context;
    ({
         let mut k5int_i: *mut k5_init_t = &mut pkinit_openssl_init__once;
         let mut k5int_err: libc::c_int =
             k5_once(&mut (*k5int_i).once, (*k5int_i).fn_0);
         if k5int_err != 0 {
         } else {
             if (*k5int_i).did_run != 0 as libc::c_int {
             } else {
                 __assert_fail(b"k5int_i->did_run != 0\x00" as *const u8 as
                                   *const libc::c_char,
                               b"pkinit_crypto_openssl.c\x00" as *const u8 as
                                   *const libc::c_char,
                               535 as libc::c_int as libc::c_uint,
                               (*::std::mem::transmute::<&[u8; 68],
                                                         &[libc::c_char; 68]>(b"krb5_error_code pkinit_init_plg_crypto(pkinit_plg_crypto_context *)\x00")).as_ptr());
             }
         };
         compile_error!("Conditional expression is not supposed to be used")
     });
    ctx =
        malloc(::std::mem::size_of::<_pkinit_plg_crypto_context>() as
                   libc::c_ulong) as pkinit_plg_crypto_context;
    if !ctx.is_null() {
        memset(ctx as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<_pkinit_plg_crypto_context>() as
                   libc::c_ulong);
        pkiDebug(b"%s: initializing openssl crypto context at %p\n\x00" as
                     *const u8 as *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 23],
                                           &[libc::c_char; 23]>(b"pkinit_init_plg_crypto\x00")).as_ptr(),
                 ctx);
        retval = pkinit_init_pkinit_oids(ctx);
        if !(retval != 0) {
            retval = pkinit_init_dh_params(ctx);
            if !(retval != 0) { *cryptoctx = ctx }
        }
    }
    if retval != 0 && !ctx.is_null() { pkinit_fini_plg_crypto(ctx); }
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "561:1"]
pub unsafe extern "C" fn pkinit_fini_plg_crypto(mut cryptoctx:
                                                    pkinit_plg_crypto_context) {
    pkiDebug(b"%s: freeing context at %p\n\x00" as *const u8 as
                 *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 23],
                                       &[libc::c_char; 23]>(b"pkinit_fini_plg_crypto\x00")).as_ptr(),
             cryptoctx);
    if cryptoctx.is_null() { return }
    pkinit_fini_pkinit_oids(cryptoctx);
    pkinit_fini_dh_params(cryptoctx);
    free(cryptoctx as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "573:1"]
pub unsafe extern "C" fn pkinit_init_identity_crypto(mut idctx:
                                                         *mut pkinit_identity_crypto_context)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut ctx: pkinit_identity_crypto_context =
        0 as pkinit_identity_crypto_context;
    ctx =
        malloc(::std::mem::size_of::<_pkinit_identity_crypto_context>() as
                   libc::c_ulong) as pkinit_identity_crypto_context;
    if !ctx.is_null() {
        memset(ctx as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<_pkinit_identity_crypto_context>() as
                   libc::c_ulong);
        (*ctx).identity = 0 as *mut libc::c_char;
        retval = pkinit_init_certs(ctx);
        if !(retval != 0) {
            retval = pkinit_init_pkcs11(ctx);
            if !(retval != 0) {
                pkiDebug(b"%s: returning ctx at %p\n\x00" as *const u8 as
                             *const libc::c_char,
                         (*::std::mem::transmute::<&[u8; 28],
                                                   &[libc::c_char; 28]>(b"pkinit_init_identity_crypto\x00")).as_ptr(),
                         ctx);
                *idctx = ctx
            }
        }
    }
    if retval != 0 { if !ctx.is_null() { pkinit_fini_identity_crypto(ctx); } }
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "606:1"]
pub unsafe extern "C" fn pkinit_fini_identity_crypto(mut idctx:
                                                         pkinit_identity_crypto_context) {
    if idctx.is_null() { return }
    pkiDebug(b"%s: freeing ctx at %p\n\x00" as *const u8 as
                 *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 28],
                                       &[libc::c_char; 28]>(b"pkinit_fini_identity_crypto\x00")).as_ptr(),
             idctx);
    if !(*idctx).deferred_ids.is_null() {
        pkinit_free_deferred_ids((*idctx).deferred_ids);
    }
    free((*idctx).identity as *mut libc::c_void);
    pkinit_fini_certs(idctx);
    pkinit_fini_pkcs11(idctx);
    free(idctx as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "621:1"]
pub unsafe extern "C" fn pkinit_init_req_crypto(mut cryptoctx:
                                                    *mut pkinit_req_crypto_context)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut ctx: pkinit_req_crypto_context = 0 as pkinit_req_crypto_context;
    ctx =
        malloc(::std::mem::size_of::<_pkinit_req_crypto_context>() as
                   libc::c_ulong) as pkinit_req_crypto_context;
    if !ctx.is_null() {
        memset(ctx as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<_pkinit_req_crypto_context>() as
                   libc::c_ulong);
        (*ctx).dh = 0 as *mut DH;
        (*ctx).received_cert = 0 as *mut X509;
        *cryptoctx = ctx;
        pkiDebug(b"%s: returning ctx at %p\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 23],
                                           &[libc::c_char; 23]>(b"pkinit_init_req_crypto\x00")).as_ptr(),
                 ctx);
        retval = 0 as libc::c_int
    }
    if retval != 0 { free(ctx as *mut libc::c_void); }
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "646:1"]
pub unsafe extern "C" fn pkinit_fini_req_crypto(mut req_cryptoctx:
                                                    pkinit_req_crypto_context) {
    if req_cryptoctx.is_null() { return }
    pkiDebug(b"%s: freeing ctx at %p\n\x00" as *const u8 as
                 *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 23],
                                       &[libc::c_char; 23]>(b"pkinit_fini_req_crypto\x00")).as_ptr(),
             req_cryptoctx);
    if !(*req_cryptoctx).dh.is_null() { DH_free((*req_cryptoctx).dh); }
    if !(*req_cryptoctx).received_cert.is_null() {
        X509_free((*req_cryptoctx).received_cert);
    }
    free(req_cryptoctx as *mut libc::c_void);
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * COPYRIGHT (C) 2006,2007
 * THE REGENTS OF THE UNIVERSITY OF MICHIGAN
 * ALL RIGHTS RESERVED
 *
 * Permission is granted to use, copy, create derivative works
 * and redistribute this software and such derivative works
 * for any purpose, so long as the name of The University of
 * Michigan is not used in any advertising or publicity
 * pertaining to the use of distribution of this software
 * without specific, written prior authorization.  If the
 * above copyright notice or any other identification of the
 * University of Michigan is included in any copy of any
 * portion of this software, then the disclaimer below must
 * also be included.
 *
 * THIS SOFTWARE IS PROVIDED AS IS, WITHOUT REPRESENTATION
 * FROM THE UNIVERSITY OF MICHIGAN AS TO ITS FITNESS FOR ANY
 * PURPOSE, AND WITHOUT WARRANTY BY THE UNIVERSITY OF
 * MICHIGAN OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING
 * WITHOUT LIMITATION THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE
 * REGENTS OF THE UNIVERSITY OF MICHIGAN SHALL NOT BE LIABLE
 * FOR ANY DAMAGES, INCLUDING SPECIAL, INDIRECT, INCIDENTAL, OR
 * CONSEQUENTIAL DAMAGES, WITH RESPECT TO ANY CLAIM ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OF THE SOFTWARE, EVEN
 * IF IT HAS BEEN OR IS HEREAFTER ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGES.
 */
#[c2rust::src_loc = "661:1"]
unsafe extern "C" fn pkinit_init_pkinit_oids(mut ctx:
                                                 pkinit_plg_crypto_context)
 -> krb5_error_code {
    (*ctx).id_pkinit_san =
        OBJ_txt2obj(b"1.3.6.1.5.2.2\x00" as *const u8 as *const libc::c_char,
                    1 as libc::c_int);
    if (*ctx).id_pkinit_san.is_null() { return 12 as libc::c_int }
    (*ctx).id_pkinit_authData =
        OBJ_txt2obj(b"1.3.6.1.5.2.3.1\x00" as *const u8 as
                        *const libc::c_char, 1 as libc::c_int);
    if (*ctx).id_pkinit_authData.is_null() { return 12 as libc::c_int }
    (*ctx).id_pkinit_DHKeyData =
        OBJ_txt2obj(b"1.3.6.1.5.2.3.2\x00" as *const u8 as
                        *const libc::c_char, 1 as libc::c_int);
    if (*ctx).id_pkinit_DHKeyData.is_null() { return 12 as libc::c_int }
    (*ctx).id_pkinit_rkeyData =
        OBJ_txt2obj(b"1.3.6.1.5.2.3.3\x00" as *const u8 as
                        *const libc::c_char, 1 as libc::c_int);
    if (*ctx).id_pkinit_rkeyData.is_null() { return 12 as libc::c_int }
    (*ctx).id_pkinit_KPClientAuth =
        OBJ_txt2obj(b"1.3.6.1.5.2.3.4\x00" as *const u8 as
                        *const libc::c_char, 1 as libc::c_int);
    if (*ctx).id_pkinit_KPClientAuth.is_null() { return 12 as libc::c_int }
    (*ctx).id_pkinit_KPKdc =
        OBJ_txt2obj(b"1.3.6.1.5.2.3.5\x00" as *const u8 as
                        *const libc::c_char, 1 as libc::c_int);
    if (*ctx).id_pkinit_KPKdc.is_null() { return 12 as libc::c_int }
    (*ctx).id_ms_kp_sc_logon =
        OBJ_txt2obj(b"1.3.6.1.4.1.311.20.2.2\x00" as *const u8 as
                        *const libc::c_char, 1 as libc::c_int);
    if (*ctx).id_ms_kp_sc_logon.is_null() { return 12 as libc::c_int }
    (*ctx).id_ms_san_upn =
        OBJ_txt2obj(b"1.3.6.1.4.1.311.20.2.3\x00" as *const u8 as
                        *const libc::c_char, 1 as libc::c_int);
    if (*ctx).id_ms_san_upn.is_null() { return 12 as libc::c_int }
    (*ctx).id_kp_serverAuth =
        OBJ_txt2obj(b"1.3.6.1.5.5.7.3.1\x00" as *const u8 as
                        *const libc::c_char, 1 as libc::c_int);
    if (*ctx).id_kp_serverAuth.is_null() { return 12 as libc::c_int }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "703:1"]
unsafe extern "C" fn get_cert(mut filename: *mut libc::c_char,
                              mut retcert: *mut *mut X509)
 -> krb5_error_code {
    let mut cert: *mut X509 = 0 as *mut X509;
    let mut tmp: *mut BIO = 0 as *mut BIO;
    let mut code: libc::c_int = 0;
    let mut retval: krb5_error_code = 0;
    if filename.is_null() || retcert.is_null() { return 22 as libc::c_int }
    *retcert = 0 as *mut X509;
    tmp = BIO_new(BIO_s_file());
    if tmp.is_null() { return 12 as libc::c_int }
    code =
        BIO_ctrl(tmp, 108 as libc::c_int,
                 (0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_long,
                 filename as *mut libc::c_void) as libc::c_int;
    if code == 0 as libc::c_int {
        retval = *__errno_location()
    } else {
        cert =
            PEM_read_bio_X509(tmp, 0 as *mut *mut X509, None,
                              0 as *mut libc::c_void);
        if cert.is_null() {
            retval = 5 as libc::c_int;
            pkiDebug(b"failed to read certificate from %s\n\x00" as *const u8
                         as *const libc::c_char, filename);
        } else { *retcert = cert; retval = 0 as libc::c_int }
    }
    if !tmp.is_null() { BIO_free(tmp); }
    return retval;
}
#[c2rust::src_loc = "748:1"]
unsafe extern "C" fn get_key_cb(mut buf: *mut libc::c_char,
                                mut size: libc::c_int,
                                mut rwflag: libc::c_int,
                                mut userdata: *mut libc::c_void)
 -> libc::c_int {
    let mut data: *mut get_key_cb_data = userdata as *mut get_key_cb_data;
    let mut id_cryptoctx: pkinit_identity_crypto_context =
        0 as *mut _pkinit_identity_crypto_context;
    let mut rdat: krb5_data =
        krb5_data{magic: 0,
                  length: 0,
                  data: 0 as *const libc::c_char as *mut libc::c_char,};
    let mut kprompt: krb5_prompt =
        krb5_prompt{prompt: 0 as *mut libc::c_char,
                    hidden: 0,
                    reply: 0 as *mut krb5_data,};
    let mut prompt_type: krb5_prompt_type = 0;
    let mut retval: krb5_error_code = 0;
    let mut prompt: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*(*data).id_cryptoctx).defer_id_prompt != 0 {
        /* Supply the identity name to be passed to a responder callback. */
        pkinit_set_deferred_id(&mut (*(*data).id_cryptoctx).deferred_ids,
                               (*data).fsname,
                               0 as libc::c_int as libc::c_ulong,
                               0 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    if (*data).password.is_null() {
        /* We don't already have a password to use, so prompt for one. */
        if (*(*data).id_cryptoctx).prompter.is_none() {
            return -(1 as libc::c_int)
        }
        if asprintf(&mut prompt as *mut *mut libc::c_char,
                    b"%s %s\x00" as *const u8 as *const libc::c_char,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"Pass phrase for\x00" as *const u8 as
                                 *const libc::c_char), (*data).filename) <
               0 as libc::c_int {
            return -(1 as libc::c_int)
        }
        rdat.data = buf;
        rdat.length = size as libc::c_uint;
        kprompt.prompt = prompt;
        kprompt.hidden = 1 as libc::c_int;
        kprompt.reply = &mut rdat;
        prompt_type = 0x4 as libc::c_int;
        /* PROMPTER_INVOCATION */
        k5int_set_prompt_types.expect("non-null function pointer")((*data).context,
                                                                   &mut prompt_type);
        id_cryptoctx = (*data).id_cryptoctx;
        retval =
            (*(*data).id_cryptoctx).prompter.expect("non-null function pointer")((*data).context,
                                                                                 (*id_cryptoctx).prompter_data,
                                                                                 0
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 0
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 1
                                                                                     as
                                                                                     libc::c_int,
                                                                                 &mut kprompt);
        k5int_set_prompt_types.expect("non-null function pointer")((*data).context,
                                                                   0 as
                                                                       *mut krb5_prompt_type);
        free(prompt as *mut libc::c_void);
        if retval != 0 as libc::c_int { return -(1 as libc::c_int) }
    } else {
        /* Just use the already-supplied password. */
        rdat.length = strlen((*data).password) as libc::c_uint;
        if rdat.length as libc::c_int >= size { return -(1 as libc::c_int) }
        snprintf(buf, size as libc::c_ulong,
                 b"%s\x00" as *const u8 as *const libc::c_char,
                 (*data).password);
    }
    return rdat.length as libc::c_int;
}
#[c2rust::src_loc = "799:1"]
unsafe extern "C" fn get_key(mut context: krb5_context,
                             mut id_cryptoctx: pkinit_identity_crypto_context,
                             mut filename: *mut libc::c_char,
                             mut fsname: *const libc::c_char,
                             mut retkey: *mut *mut EVP_PKEY,
                             mut password: *const libc::c_char)
 -> krb5_error_code {
    let mut pkey: *mut EVP_PKEY = 0 as *mut EVP_PKEY;
    let mut tmp: *mut BIO = 0 as *mut BIO;
    let mut cb_data: get_key_cb_data =
        get_key_cb_data{context: 0 as *mut _krb5_context,
                        id_cryptoctx:
                            0 as *mut _pkinit_identity_crypto_context,
                        fsname: 0 as *const libc::c_char,
                        filename: 0 as *mut libc::c_char,
                        password: 0 as *const libc::c_char,};
    let mut code: libc::c_int = 0;
    let mut retval: krb5_error_code = 0;
    if filename.is_null() || retkey.is_null() { return 22 as libc::c_int }
    tmp = BIO_new(BIO_s_file());
    if tmp.is_null() { return 12 as libc::c_int }
    code =
        BIO_ctrl(tmp, 108 as libc::c_int,
                 (0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_long,
                 filename as *mut libc::c_void) as libc::c_int;
    if code == 0 as libc::c_int {
        retval = *__errno_location()
    } else {
        cb_data.context = context;
        cb_data.id_cryptoctx = id_cryptoctx;
        cb_data.filename = filename;
        cb_data.fsname = fsname;
        cb_data.password = password;
        pkey =
            PEM_read_bio_PrivateKey(tmp, 0 as *mut *mut EVP_PKEY,
                                    Some(get_key_cb as
                                             unsafe extern "C" fn(_:
                                                                      *mut libc::c_char,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      *mut libc::c_void)
                                                 -> libc::c_int),
                                    &mut cb_data as *mut get_key_cb_data as
                                        *mut libc::c_void);
        if pkey.is_null() && (*id_cryptoctx).defer_id_prompt == 0 {
            retval = 5 as libc::c_int;
            pkiDebug(b"failed to read private key from %s\n\x00" as *const u8
                         as *const libc::c_char, filename);
        } else { *retkey = pkey; retval = 0 as libc::c_int }
    }
    if !tmp.is_null() { BIO_free(tmp); }
    return retval;
}
#[c2rust::src_loc = "841:1"]
unsafe extern "C" fn pkinit_fini_pkinit_oids(mut ctx:
                                                 pkinit_plg_crypto_context) {
    if ctx.is_null() { return }
    ASN1_OBJECT_free((*ctx).id_pkinit_san);
    ASN1_OBJECT_free((*ctx).id_pkinit_authData);
    ASN1_OBJECT_free((*ctx).id_pkinit_DHKeyData);
    ASN1_OBJECT_free((*ctx).id_pkinit_rkeyData);
    ASN1_OBJECT_free((*ctx).id_pkinit_KPClientAuth);
    ASN1_OBJECT_free((*ctx).id_pkinit_KPKdc);
    ASN1_OBJECT_free((*ctx).id_ms_kp_sc_logon);
    ASN1_OBJECT_free((*ctx).id_ms_san_upn);
    ASN1_OBJECT_free((*ctx).id_kp_serverAuth);
}
/* Construct an OpenSSL DH object for an Oakley group. */
#[c2rust::src_loc = "858:1"]
unsafe extern "C" fn make_oakley_dh(mut prime: *mut uint8_t, mut len: size_t)
 -> *mut DH {
    let mut dh: *mut DH = 0 as *mut DH;
    let mut p: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut q: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut g: *mut BIGNUM = 0 as *mut BIGNUM;
    p = BN_bin2bn(prime, len as libc::c_int, 0 as *mut BIGNUM);
    if !p.is_null() {
        q = BN_new();
        if !q.is_null() {
            if !(BN_rshift1(q, p) == 0) {
                g = BN_new();
                if !g.is_null() {
                    if !(BN_set_word(g, 2 as libc::c_int as libc::c_ulong) ==
                             0) {
                        dh = DH_new();
                        if !dh.is_null() {
                            DH_set0_pqg(dh, p, q, g);
                            q = 0 as *mut BIGNUM;
                            g = q;
                            p = g
                        }
                    }
                }
            }
        }
    }
    BN_free(p);
    BN_free(q);
    BN_free(g);
    return dh;
}
#[c2rust::src_loc = "891:1"]
unsafe extern "C" fn pkinit_init_dh_params(mut plgctx:
                                               pkinit_plg_crypto_context)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 12 as libc::c_int;
    (*plgctx).dh_1024 =
        make_oakley_dh(oakley_1024.as_mut_ptr(),
                       ::std::mem::size_of::<[uint8_t; 128]>() as
                           libc::c_ulong);
    if !(*plgctx).dh_1024.is_null() {
        (*plgctx).dh_2048 =
            make_oakley_dh(oakley_2048.as_mut_ptr(),
                           ::std::mem::size_of::<[uint8_t; 256]>() as
                               libc::c_ulong);
        if !(*plgctx).dh_2048.is_null() {
            (*plgctx).dh_4096 =
                make_oakley_dh(oakley_4096.as_mut_ptr(),
                               ::std::mem::size_of::<[uint8_t; 512]>() as
                                   libc::c_ulong);
            if !(*plgctx).dh_4096.is_null() { retval = 0 as libc::c_int }
        }
    }
    if retval != 0 { pkinit_fini_dh_params(plgctx); }
    return retval;
}
#[c2rust::src_loc = "917:1"]
unsafe extern "C" fn pkinit_fini_dh_params(mut plgctx:
                                               pkinit_plg_crypto_context) {
    if !(*plgctx).dh_1024.is_null() { DH_free((*plgctx).dh_1024); }
    if !(*plgctx).dh_2048.is_null() { DH_free((*plgctx).dh_2048); }
    if !(*plgctx).dh_4096.is_null() { DH_free((*plgctx).dh_4096); }
    (*plgctx).dh_4096 = 0 as *mut DH;
    (*plgctx).dh_2048 = (*plgctx).dh_4096;
    (*plgctx).dh_1024 = (*plgctx).dh_2048;
}
#[c2rust::src_loc = "930:1"]
unsafe extern "C" fn pkinit_init_certs(mut ctx:
                                           pkinit_identity_crypto_context)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        (*ctx).creds[i as usize] = 0 as pkinit_cred_info;
        i += 1
    }
    (*ctx).my_certs = 0 as *mut stack_st_X509;
    (*ctx).cert_index = 0 as libc::c_int;
    (*ctx).my_key = 0 as *mut EVP_PKEY;
    (*ctx).trustedCAs = 0 as *mut stack_st_X509;
    (*ctx).intermediateCAs = 0 as *mut stack_st_X509;
    (*ctx).revoked = 0 as *mut stack_st_X509_CRL;
    retval = 0 as libc::c_int;
    return retval;
}
#[c2rust::src_loc = "949:1"]
unsafe extern "C" fn pkinit_fini_certs(mut ctx:
                                           pkinit_identity_crypto_context) {
    if ctx.is_null() { return }
    if !(*ctx).my_certs.is_null() {
        sk_X509_pop_free((*ctx).my_certs,
                         Some(X509_free as
                                  unsafe extern "C" fn(_: *mut X509) -> ()));
    }
    if !(*ctx).my_key.is_null() { EVP_PKEY_free((*ctx).my_key); }
    if !(*ctx).trustedCAs.is_null() {
        sk_X509_pop_free((*ctx).trustedCAs,
                         Some(X509_free as
                                  unsafe extern "C" fn(_: *mut X509) -> ()));
    }
    if !(*ctx).intermediateCAs.is_null() {
        sk_X509_pop_free((*ctx).intermediateCAs,
                         Some(X509_free as
                                  unsafe extern "C" fn(_: *mut X509) -> ()));
    }
    if !(*ctx).revoked.is_null() {
        sk_X509_CRL_pop_free((*ctx).revoked,
                             Some(X509_CRL_free as
                                      unsafe extern "C" fn(_: *mut X509_CRL)
                                          -> ()));
    };
}
#[c2rust::src_loc = "971:1"]
unsafe extern "C" fn pkinit_init_pkcs11(mut ctx:
                                            pkinit_identity_crypto_context)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 12 as libc::c_int;
    (*ctx).p11_module_name =
        strdup(b"opensc-pkcs11.so\x00" as *const u8 as *const libc::c_char);
    if (*ctx).p11_module_name.is_null() { return retval }
    (*ctx).p11_module = 0 as *mut libc::c_void;
    (*ctx).slotid = 999999 as libc::c_int as CK_SLOT_ID;
    (*ctx).token_label = 0 as *mut libc::c_char;
    (*ctx).cert_label = 0 as *mut libc::c_char;
    (*ctx).session = 0 as libc::c_int as CK_SESSION_HANDLE;
    (*ctx).p11 = 0 as CK_FUNCTION_LIST_PTR;
    (*ctx).pkcs11_method = 0 as libc::c_int;
    retval = 0 as libc::c_int;
    return retval;
}
#[c2rust::src_loc = "993:1"]
unsafe extern "C" fn pkinit_fini_pkcs11(mut ctx:
                                            pkinit_identity_crypto_context) {
    if ctx.is_null() { return }
    if !(*ctx).p11.is_null() {
        if (*ctx).session != 0 as libc::c_int as libc::c_ulong {
            (*(*ctx).p11).C_CloseSession.expect("non-null function pointer")((*ctx).session);
            (*ctx).session = 0 as libc::c_int as CK_SESSION_HANDLE
        }
        (*(*ctx).p11).C_Finalize.expect("non-null function pointer")(0 as
                                                                         *mut libc::c_void);
        (*ctx).p11 = 0 as CK_FUNCTION_LIST_PTR
    }
    if !(*ctx).p11_module.is_null() {
        pkinit_C_UnloadModule((*ctx).p11_module);
        (*ctx).p11_module = 0 as *mut libc::c_void
    }
    free((*ctx).p11_module_name as *mut libc::c_void);
    free((*ctx).token_label as *mut libc::c_void);
    free((*ctx).cert_id as *mut libc::c_void);
    free((*ctx).cert_label as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "1019:1"]
pub unsafe extern "C" fn pkinit_identity_set_prompter(mut id_cryptoctx:
                                                          pkinit_identity_crypto_context,
                                                      mut prompter:
                                                          krb5_prompter_fct,
                                                      mut prompter_data:
                                                          *mut libc::c_void)
 -> krb5_error_code {
    (*id_cryptoctx).prompter = prompter;
    (*id_cryptoctx).prompter_data = prompter_data;
    return 0 as libc::c_int;
}
/* Create a CMS ContentInfo of type oid containing the octet string in data. */
#[c2rust::src_loc = "1031:1"]
unsafe extern "C" fn create_contentinfo(mut context: krb5_context,
                                        mut oid: *mut ASN1_OBJECT,
                                        mut data: *mut libc::c_uchar,
                                        mut data_len: size_t,
                                        mut p7_out: *mut *mut PKCS7)
 -> krb5_error_code {
    let mut p7: *mut PKCS7 = 0 as *mut PKCS7;
    let mut ostr: *mut ASN1_OCTET_STRING = 0 as *mut ASN1_OCTET_STRING;
    *p7_out = 0 as *mut PKCS7;
    ostr = ASN1_OCTET_STRING_new();
    if !ostr.is_null() {
        if !(ASN1_OCTET_STRING_set(ostr, data, data_len as libc::c_int) == 0)
           {
            p7 = PKCS7_new();
            if !p7.is_null() {
                (*p7).type_0 = OBJ_dup(oid);
                if !(*p7).type_0.is_null() {
                    (*p7).d.other = ASN1_TYPE_new();
                    if !(*p7).d.other.is_null() {
                        (*(*p7).d.other).type_0 = 4 as libc::c_int;
                        (*(*p7).d.other).value.octet_string = ostr;
                        *p7_out = p7;
                        return 0 as libc::c_int
                    }
                }
            }
        }
    }
    if !ostr.is_null() { ASN1_OCTET_STRING_free(ostr); }
    if !p7.is_null() { PKCS7_free(p7); }
    return 12 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1070:1"]
pub unsafe extern "C" fn cms_contentinfo_create(mut context: krb5_context,
                                                mut plg_cryptoctx:
                                                    pkinit_plg_crypto_context,
                                                mut req_cryptoctx:
                                                    pkinit_req_crypto_context,
                                                mut id_cryptoctx:
                                                    pkinit_identity_crypto_context,
                                                mut cms_msg_type: libc::c_int,
                                                mut data: *mut libc::c_uchar,
                                                mut data_len: libc::c_uint,
                                                mut out_data:
                                                    *mut *mut libc::c_uchar,
                                                mut out_data_len:
                                                    *mut libc::c_uint)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut oid: *mut ASN1_OBJECT = 0 as *mut ASN1_OBJECT;
    let mut p7: *mut PKCS7 = 0 as *mut PKCS7;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    /* Pick the correct oid for the eContentInfo. */
    oid = pkinit_pkcs7type2oid(plg_cryptoctx, cms_msg_type);
    if !oid.is_null() {
        retval =
            create_contentinfo(context, oid, data, data_len as size_t,
                               &mut p7);
        if !(retval != 0 as libc::c_int) {
            *out_data_len =
                i2d_PKCS7(p7, 0 as *mut *mut libc::c_uchar) as libc::c_uint;
            if *out_data_len == 0 {
                retval =
                    oerr(context, 0 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"Failed to DER encode PKCS7\x00" as
                                      *const u8 as *const libc::c_char))
            } else {
                retval = 12 as libc::c_int;
                *out_data =
                    malloc(*out_data_len as libc::c_ulong) as
                        *mut libc::c_uchar;
                p = *out_data;
                if !p.is_null() {
                    /* DER encode PKCS7 data */
                    retval = i2d_PKCS7(p7, &mut p);
                    if retval == 0 {
                        retval =
                            oerr(context, 0 as libc::c_int,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"Failed to DER encode PKCS7\x00" as
                                              *const u8 as
                                              *const libc::c_char))
                    } else { retval = 0 as libc::c_int }
                }
            }
        }
    }
    if !p7.is_null() { PKCS7_free(p7); }
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "1115:1"]
pub unsafe extern "C" fn cms_signeddata_create(mut context: krb5_context,
                                               mut plg_cryptoctx:
                                                   pkinit_plg_crypto_context,
                                               mut req_cryptoctx:
                                                   pkinit_req_crypto_context,
                                               mut id_cryptoctx:
                                                   pkinit_identity_crypto_context,
                                               mut cms_msg_type: libc::c_int,
                                               mut include_certchain:
                                                   libc::c_int,
                                               mut data: *mut libc::c_uchar,
                                               mut data_len: libc::c_uint,
                                               mut signed_data:
                                                   *mut *mut libc::c_uchar,
                                               mut signed_data_len:
                                                   *mut libc::c_uint)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut p7: *mut PKCS7 = 0 as *mut PKCS7;
    let mut inner_p7: *mut PKCS7 = 0 as *mut PKCS7;
    let mut p7s: *mut PKCS7_SIGNED = 0 as *mut PKCS7_SIGNED;
    let mut p7si: *mut PKCS7_SIGNER_INFO = 0 as *mut PKCS7_SIGNER_INFO;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut cert_stack: *mut stack_st_X509 = 0 as *mut stack_st_X509;
    let mut digest_attr: *mut ASN1_OCTET_STRING = 0 as *mut ASN1_OCTET_STRING;
    let mut ctx: *mut EVP_MD_CTX = 0 as *mut EVP_MD_CTX;
    let mut md_tmp: *const EVP_MD = 0 as *const EVP_MD;
    let mut md_data: [libc::c_uchar; 64] = [0; 64];
    let mut md_data2: [libc::c_uchar; 64] = [0; 64];
    let mut digestInfo_buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut abuf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut md_len: libc::c_uint = 0;
    let mut md_len2: libc::c_uint = 0;
    let mut alen: libc::c_uint = 0;
    let mut digestInfo_len: libc::c_uint = 0;
    let mut sk: *mut stack_st_X509_ATTRIBUTE =
        0 as *mut stack_st_X509_ATTRIBUTE;
    let mut sig: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sig_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut alg: *mut X509_ALGOR = 0 as *mut X509_ALGOR;
    let mut digest: *mut ASN1_OCTET_STRING = 0 as *mut ASN1_OCTET_STRING;
    let mut alg_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut digest_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut y: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut cert: *mut X509 = 0 as *mut X509;
    let mut oid: *mut ASN1_OBJECT = 0 as *mut ASN1_OBJECT;
    let mut oid_copy: *mut ASN1_OBJECT = 0 as *mut ASN1_OBJECT;
    /* Start creating PKCS7 data. */
    p7 = PKCS7_new();
    if !p7.is_null() {
        (*p7).type_0 = OBJ_nid2obj(22 as libc::c_int);
        p7s = PKCS7_SIGNED_new();
        if !p7s.is_null() {
            (*p7).d.sign = p7s;
            if !(ASN1_INTEGER_set((*p7s).version,
                                  3 as libc::c_int as libc::c_long) == 0) {
                /* pick the correct oid for the eContentInfo */
                oid =
                    pkinit_pkcs7type2oid(plg_cryptoctx,
                                         cms_msg_type); /* we have a certificate */
                if !oid.is_null() {
                    if !(*id_cryptoctx).my_certs.is_null() {
                        /* create a cert chain that has at least the signer's certificate */
                        cert_stack = sk_X509_new_null();
                        if cert_stack.is_null() {
                            current_block = 14538362299760319115;
                        } else {
                            cert =
                                sk_X509_value((*id_cryptoctx).my_certs,
                                              (*id_cryptoctx).cert_index);
                            if include_certchain == 0 {
                                pkiDebug(b"only including signer\'s certificate\n\x00"
                                             as *const u8 as
                                             *const libc::c_char);
                                sk_X509_push(cert_stack, X509_dup(cert));
                                current_block = 13619784596304402172;
                            } else {
                                /* create a cert chain */
                                let mut certstore: *mut X509_STORE =
                                    0 as *mut X509_STORE;
                                let mut certctx: *mut X509_STORE_CTX =
                                    0 as *mut X509_STORE_CTX;
                                let mut certstack: *mut stack_st_X509 =
                                    0 as *mut stack_st_X509;
                                let mut buf: [libc::c_char; 256] = [0; 256];
                                let mut i: libc::c_uint =
                                    0 as libc::c_int as libc::c_uint;
                                let mut size: libc::c_uint =
                                    0 as libc::c_int as libc::c_uint;
                                certstore = X509_STORE_new();
                                if certstore.is_null() {
                                    current_block = 14538362299760319115;
                                } else {
                                    pkiDebug(b"building certificate chain\n\x00"
                                                 as *const u8 as
                                                 *const libc::c_char);
                                    X509_STORE_set_verify_cb(certstore,
                                                             Some(openssl_callback
                                                                      as
                                                                      unsafe extern "C" fn(_:
                                                                                               libc::c_int,
                                                                                           _:
                                                                                               *mut X509_STORE_CTX)
                                                                          ->
                                                                              libc::c_int));
                                    certctx = X509_STORE_CTX_new();
                                    if certctx.is_null() {
                                        current_block = 14538362299760319115;
                                    } else {
                                        X509_STORE_CTX_init(certctx,
                                                            certstore, cert,
                                                            (*id_cryptoctx).intermediateCAs);
                                        X509_STORE_CTX_set0_trusted_stack(certctx,
                                                                          (*id_cryptoctx).trustedCAs);
                                        if X509_verify_cert(certctx) == 0 {
                                            retval =
                                                oerr_cert(context,
                                                          0 as libc::c_int,
                                                          certctx,
                                                          dgettext(b"mit-krb5\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   b"Failed to verify own certificate\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char));
                                            current_block =
                                                14538362299760319115;
                                        } else {
                                            certstack =
                                                X509_STORE_CTX_get1_chain(certctx);
                                            size =
                                                sk_X509_num(certstack) as
                                                    libc::c_uint;
                                            pkiDebug(b"size of certificate chain = %d\n\x00"
                                                         as *const u8 as
                                                         *const libc::c_char,
                                                     size);
                                            i =
                                                0 as libc::c_int as
                                                    libc::c_uint;
                                            while i <
                                                      size.wrapping_sub(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
                                                  {
                                                let mut x: *mut X509 =
                                                    sk_X509_value(certstack,
                                                                  i as
                                                                      libc::c_int);
                                                X509_NAME_oneline(X509_get_subject_name(x),
                                                                  buf.as_mut_ptr(),
                                                                  ::std::mem::size_of::<[libc::c_char; 256]>()
                                                                      as
                                                                      libc::c_ulong
                                                                      as
                                                                      libc::c_int);
                                                pkiDebug(b"cert #%d: %s\n\x00"
                                                             as *const u8 as
                                                             *const libc::c_char,
                                                         i, buf.as_mut_ptr());
                                                sk_X509_push(cert_stack,
                                                             X509_dup(x));
                                                i = i.wrapping_add(1)
                                            }
                                            X509_STORE_CTX_free(certctx);
                                            X509_STORE_free(certstore);
                                            sk_X509_pop_free(certstack,
                                                             Some(X509_free as
                                                                      unsafe extern "C" fn(_:
                                                                                               *mut X509)
                                                                          ->
                                                                              ()));
                                            current_block =
                                                13619784596304402172;
                                        }
                                    }
                                }
                            }
                            match current_block {
                                14538362299760319115 => { }
                                _ => {
                                    (*p7s).cert = cert_stack;
                                    /* fill-in PKCS7_SIGNER_INFO */
                                    p7si = PKCS7_SIGNER_INFO_new();
                                    if p7si.is_null() {
                                        current_block = 14538362299760319115;
                                    } else if ASN1_INTEGER_set((*p7si).version,
                                                               1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_long)
                                                  == 0 {
                                        current_block = 14538362299760319115;
                                    } else if X509_NAME_set(&mut (*(*p7si).issuer_and_serial).issuer,
                                                            X509_get_issuer_name(cert))
                                                  == 0 {
                                        current_block = 14538362299760319115;
                                    } else {
                                        /* because ASN1_INTEGER_set is used to set a 'long' we will do
         * things the ugly way. */
                                        ASN1_INTEGER_free((*(*p7si).issuer_and_serial).serial);
                                        (*(*p7si).issuer_and_serial).serial =
                                            ASN1_INTEGER_dup(X509_get_serialNumber(cert));
                                        if (*(*p7si).issuer_and_serial).serial.is_null()
                                           {
                                            current_block =
                                                14538362299760319115;
                                        } else {
                                            /* will not fill-out EVP_PKEY because it's on the smartcard */
                                            /* Set digest algs */
                                            (*(*p7si).digest_alg).algorithm =
                                                OBJ_nid2obj(64 as
                                                                libc::c_int);
                                            if !(*(*p7si).digest_alg).parameter.is_null()
                                               {
                                                ASN1_TYPE_free((*(*p7si).digest_alg).parameter);
                                            }
                                            (*(*p7si).digest_alg).parameter =
                                                ASN1_TYPE_new();
                                            if (*(*p7si).digest_alg).parameter.is_null()
                                               {
                                                current_block =
                                                    14538362299760319115;
                                            } else {
                                                (*(*(*p7si).digest_alg).parameter).type_0
                                                    = 5 as libc::c_int;
                                                /* Set sig algs */
                                                if !(*(*p7si).digest_enc_alg).parameter.is_null()
                                                   {
                                                    ASN1_TYPE_free((*(*p7si).digest_enc_alg).parameter);
                                                }
                                                (*(*p7si).digest_enc_alg).algorithm
                                                    =
                                                    OBJ_nid2obj(65 as
                                                                    libc::c_int);
                                                (*(*p7si).digest_enc_alg).parameter
                                                    = ASN1_TYPE_new();
                                                if (*(*p7si).digest_enc_alg).parameter.is_null()
                                                   {
                                                    current_block =
                                                        14538362299760319115;
                                                } else {
                                                    (*(*(*p7si).digest_enc_alg).parameter).type_0
                                                        = 5 as libc::c_int;
                                                    /* add signed attributes */
        /* compute sha1 digest over the EncapsulatedContentInfo */
                                                    ctx = EVP_MD_CTX_new();
                                                    if ctx.is_null() {
                                                        current_block =
                                                            14538362299760319115;
                                                    } else {
                                                        EVP_DigestInit_ex(ctx,
                                                                          EVP_sha1(),
                                                                          0 as
                                                                              *mut ENGINE);
                                                        EVP_DigestUpdate(ctx,
                                                                         data
                                                                             as
                                                                             *const libc::c_void,
                                                                         data_len
                                                                             as
                                                                             size_t);
                                                        md_tmp =
                                                            EVP_MD_CTX_md(ctx);
                                                        EVP_DigestFinal_ex(ctx,
                                                                           md_data.as_mut_ptr(),
                                                                           &mut md_len);
                                                        EVP_MD_CTX_free(ctx);
                                                        /* create a message digest attr */
                                                        digest_attr =
                                                            ASN1_OCTET_STRING_new();
                                                        ASN1_OCTET_STRING_set(digest_attr,
                                                                              md_data.as_mut_ptr(),
                                                                              md_len
                                                                                  as
                                                                                  libc::c_int);
                                                        PKCS7_add_signed_attribute(p7si,
                                                                                   51
                                                                                       as
                                                                                       libc::c_int,
                                                                                   4
                                                                                       as
                                                                                       libc::c_int,
                                                                                   digest_attr
                                                                                       as
                                                                                       *mut libc::c_char
                                                                                       as
                                                                                       *mut libc::c_void);
                                                        /* create a content-type attr */
                                                        oid_copy =
                                                            OBJ_dup(oid);
                                                        if oid_copy.is_null()
                                                           {
                                                            current_block =
                                                                5747012219155821488;
                                                        } else {
                                                            PKCS7_add_signed_attribute(p7si,
                                                                                       50
                                                                                           as
                                                                                           libc::c_int,
                                                                                       6
                                                                                           as
                                                                                           libc::c_int,
                                                                                       oid_copy
                                                                                           as
                                                                                           *mut libc::c_void);
                                                            /* create the signature over signed attributes. get DER encoded value */
        /* This is the place where smartcard signature needs to be calculated */
                                                            sk =
                                                                (*p7si).auth_attr;
                                                            alen =
                                                                ASN1_item_i2d(sk
                                                                                  as
                                                                                  *mut ASN1_VALUE,
                                                                              &mut abuf,
                                                                              &PKCS7_ATTR_SIGN_it)
                                                                    as
                                                                    libc::c_uint;
                                                            if abuf.is_null()
                                                               {
                                                                current_block
                                                                    =
                                                                    5747012219155821488;
                                                            } else {
                                                                /* Some tokens can only do RSAEncryption without sha1 hash */
        /* to compute sha1WithRSAEncryption, encode the algorithm ID for the hash
         * function and the hash value into an ASN.1 value of type DigestInfo
         * DigestInfo::=SEQUENCE {
         *  digestAlgorithm  AlgorithmIdentifier,
         *  digest OCTET STRING }
         */
                                                                if (*id_cryptoctx).pkcs11_method
                                                                       ==
                                                                       1 as
                                                                           libc::c_int
                                                                       &&
                                                                       (*id_cryptoctx).mech
                                                                           ==
                                                                           1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_ulong
                                                                   {
                                                                    pkiDebug(b"mech = CKM_RSA_PKCS\n\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                                                                    ctx =
                                                                        EVP_MD_CTX_new();
                                                                    if ctx.is_null()
                                                                       {
                                                                        current_block
                                                                            =
                                                                            14538362299760319115;
                                                                    } else {
                                                                        EVP_DigestInit_ex(ctx,
                                                                                          md_tmp,
                                                                                          0
                                                                                              as
                                                                                              *mut ENGINE);
                                                                        EVP_DigestUpdate(ctx,
                                                                                         abuf
                                                                                             as
                                                                                             *const libc::c_void,
                                                                                         alen
                                                                                             as
                                                                                             size_t);
                                                                        EVP_DigestFinal_ex(ctx,
                                                                                           md_data2.as_mut_ptr(),
                                                                                           &mut md_len2);
                                                                        EVP_MD_CTX_free(ctx);
                                                                        alg =
                                                                            X509_ALGOR_new();
                                                                        if alg.is_null()
                                                                           {
                                                                            current_block
                                                                                =
                                                                                5747012219155821488;
                                                                        } else {
                                                                            X509_ALGOR_set0(alg,
                                                                                            OBJ_nid2obj(64
                                                                                                            as
                                                                                                            libc::c_int),
                                                                                            5
                                                                                                as
                                                                                                libc::c_int,
                                                                                            0
                                                                                                as
                                                                                                *mut libc::c_void);
                                                                            alg_len
                                                                                =
                                                                                i2d_X509_ALGOR(alg,
                                                                                               0
                                                                                                   as
                                                                                                   *mut *mut libc::c_uchar)
                                                                                    as
                                                                                    libc::c_uint;
                                                                            digest
                                                                                =
                                                                                ASN1_OCTET_STRING_new();
                                                                            if digest.is_null()
                                                                               {
                                                                                current_block
                                                                                    =
                                                                                    5747012219155821488;
                                                                            } else {
                                                                                ASN1_OCTET_STRING_set(digest,
                                                                                                      md_data2.as_mut_ptr(),
                                                                                                      md_len2
                                                                                                          as
                                                                                                          libc::c_int);
                                                                                digest_len
                                                                                    =
                                                                                    i2d_ASN1_OCTET_STRING(digest,
                                                                                                          0
                                                                                                              as
                                                                                                              *mut *mut libc::c_uchar)
                                                                                        as
                                                                                        libc::c_uint;
                                                                                digestInfo_len
                                                                                    =
                                                                                    ASN1_object_size(1
                                                                                                         as
                                                                                                         libc::c_int,
                                                                                                     alg_len.wrapping_add(digest_len)
                                                                                                         as
                                                                                                         libc::c_int,
                                                                                                     16
                                                                                                         as
                                                                                                         libc::c_int)
                                                                                        as
                                                                                        libc::c_uint;
                                                                                digestInfo_buf
                                                                                    =
                                                                                    malloc(digestInfo_len
                                                                                               as
                                                                                               libc::c_ulong)
                                                                                        as
                                                                                        *mut libc::c_uchar;
                                                                                y
                                                                                    =
                                                                                    digestInfo_buf;
                                                                                if digestInfo_buf.is_null()
                                                                                   {
                                                                                    current_block
                                                                                        =
                                                                                        5747012219155821488;
                                                                                } else {
                                                                                    ASN1_put_object(&mut y,
                                                                                                    1
                                                                                                        as
                                                                                                        libc::c_int,
                                                                                                    alg_len.wrapping_add(digest_len)
                                                                                                        as
                                                                                                        libc::c_int,
                                                                                                    16
                                                                                                        as
                                                                                                        libc::c_int,
                                                                                                    0
                                                                                                        as
                                                                                                        libc::c_int);
                                                                                    i2d_X509_ALGOR(alg,
                                                                                                   &mut y);
                                                                                    i2d_ASN1_OCTET_STRING(digest,
                                                                                                          &mut y);
                                                                                    retval
                                                                                        =
                                                                                        pkinit_sign_data(context,
                                                                                                         id_cryptoctx,
                                                                                                         digestInfo_buf,
                                                                                                         digestInfo_len,
                                                                                                         &mut sig,
                                                                                                         &mut sig_len);
                                                                                    current_block
                                                                                        =
                                                                                        2220405792722996547;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    pkiDebug(b"mech = %s\n\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char,
                                                                             if (*id_cryptoctx).pkcs11_method
                                                                                    ==
                                                                                    1
                                                                                        as
                                                                                        libc::c_int
                                                                                {
                                                                                 b"CKM_SHA1_RSA_PKCS\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char
                                                                             } else {
                                                                                 b"FS\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char
                                                                             });
                                                                    retval =
                                                                        pkinit_sign_data(context,
                                                                                         id_cryptoctx,
                                                                                         abuf,
                                                                                         alen,
                                                                                         &mut sig,
                                                                                         &mut sig_len);
                                                                    current_block
                                                                        =
                                                                        2220405792722996547;
                                                                }
                                                                match current_block
                                                                    {
                                                                    5747012219155821488
                                                                    => {
                                                                    }
                                                                    14538362299760319115
                                                                    => {
                                                                    }
                                                                    _ => {
                                                                        free(abuf
                                                                                 as
                                                                                 *mut libc::c_void);
                                                                        if retval
                                                                               !=
                                                                               0
                                                                           {
                                                                            current_block
                                                                                =
                                                                                5747012219155821488;
                                                                        } else if ASN1_STRING_set((*p7si).enc_digest,
                                                                                                  sig
                                                                                                      as
                                                                                                      *const libc::c_void,
                                                                                                  sig_len
                                                                                                      as
                                                                                                      libc::c_int)
                                                                                      ==
                                                                                      0
                                                                         {
                                                                            retval
                                                                                =
                                                                                oerr(context,
                                                                                     0
                                                                                         as
                                                                                         libc::c_int,
                                                                                     dgettext(b"mit-krb5\x00"
                                                                                                  as
                                                                                                  *const u8
                                                                                                  as
                                                                                                  *const libc::c_char,
                                                                                              b"Failed to add digest attribute\x00"
                                                                                                  as
                                                                                                  *const u8
                                                                                                  as
                                                                                                  *const libc::c_char));
                                                                            current_block
                                                                                =
                                                                                5747012219155821488;
                                                                        } else if PKCS7_add_signer(p7,
                                                                                                   p7si)
                                                                                      ==
                                                                                      0
                                                                         {
                                                                            current_block
                                                                                =
                                                                                5747012219155821488;
                                                                        } else {
                                                                            current_block
                                                                                =
                                                                                3575278370434307847;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else { current_block = 3575278370434307847; }
                    match current_block {
                        14538362299760319115 => { }
                        _ => {
                            match current_block {
                                3575278370434307847 => {
                                    /* Add signature */
                                    /* adder signer_info to pkcs7 signed */
                                    /* start on adding data to the pkcs7 signed */
                                    retval =
                                        create_contentinfo(context, oid, data,
                                                           data_len as size_t,
                                                           &mut inner_p7);
                                    if !(*p7s).contents.is_null() {
                                        PKCS7_free((*p7s).contents);
                                    }
                                    (*p7s).contents = inner_p7;
                                    *signed_data_len =
                                        i2d_PKCS7(p7,
                                                  0 as
                                                      *mut *mut libc::c_uchar)
                                            as libc::c_uint;
                                    if *signed_data_len == 0 {
                                        retval =
                                            oerr(context, 0 as libc::c_int,
                                                 dgettext(b"mit-krb5\x00" as
                                                              *const u8 as
                                                              *const libc::c_char,
                                                          b"Failed to DER encode PKCS7\x00"
                                                              as *const u8 as
                                                              *const libc::c_char))
                                    } else {
                                        retval = 12 as libc::c_int;
                                        *signed_data =
                                            malloc(*signed_data_len as
                                                       libc::c_ulong) as
                                                *mut libc::c_uchar;
                                        p = *signed_data;
                                        if !p.is_null() {
                                            /* DER encode PKCS7 data */
                                            retval = i2d_PKCS7(p7, &mut p);
                                            if retval == 0 {
                                                retval =
                                                    oerr(context,
                                                         0 as libc::c_int,
                                                         dgettext(b"mit-krb5\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char,
                                                                  b"Failed to DER encode PKCS7\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char))
                                            } else {
                                                retval = 0 as libc::c_int
                                            }
                                        }
                                    }
                                }
                                _ => { }
                            }
                            if !p7si.is_null() {
                                if (*id_cryptoctx).pkcs11_method ==
                                       1 as libc::c_int &&
                                       (*id_cryptoctx).mech ==
                                           1 as libc::c_int as libc::c_ulong {
                                    free(digestInfo_buf as *mut libc::c_void);
                                    if !digest.is_null() {
                                        ASN1_OCTET_STRING_free(digest);
                                    }
                                }
                                if !alg.is_null() { X509_ALGOR_free(alg); }
                            }
                        }
                    }
                }
            }
        }
    }
    if !p7.is_null() { PKCS7_free(p7); }
    free(sig as *mut libc::c_void);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "1405:1"]
pub unsafe extern "C" fn cms_signeddata_verify(mut context: krb5_context,
                                               mut plgctx:
                                                   pkinit_plg_crypto_context,
                                               mut reqctx:
                                                   pkinit_req_crypto_context,
                                               mut idctx:
                                                   pkinit_identity_crypto_context,
                                               mut cms_msg_type: libc::c_int,
                                               mut require_crl_checking:
                                                   libc::c_int,
                                               mut signed_data:
                                                   *mut libc::c_uchar,
                                               mut signed_data_len:
                                                   libc::c_uint,
                                               mut data:
                                                   *mut *mut libc::c_uchar,
                                               mut data_len:
                                                   *mut libc::c_uint,
                                               mut authz_data:
                                                   *mut *mut libc::c_uchar,
                                               mut authz_data_len:
                                                   *mut libc::c_uint,
                                               mut is_signed:
                                                   *mut libc::c_int)
 -> krb5_error_code {
    let mut current_block: u64;
    /*
     * Warning: Since most openssl functions do not set retval, large chunks of
     * this function assume that retval is always a failure and may go to
     * cleanup without setting retval explicitly. Make sure retval is not set
     * to 0 or errors such as signature verification failure may be converted
     * to success with significant security consequences.
     */
    let mut retval: krb5_error_code =
        -(1765328360 as libc::c_long) as krb5_error_code;
    let mut cms: *mut CMS_ContentInfo = 0 as *mut CMS_ContentInfo;
    let mut out: *mut BIO = 0 as *mut BIO;
    let mut flags: libc::c_int = 0x20 as libc::c_int;
    let mut valid_oid: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut vflags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut size: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut p: *const libc::c_uchar = signed_data;
    let mut si_sk: *mut stack_st_CMS_SignerInfo =
        0 as *mut stack_st_CMS_SignerInfo;
    let mut si: *mut CMS_SignerInfo = 0 as *mut CMS_SignerInfo;
    let mut x: *mut X509 = 0 as *mut X509;
    let mut store: *mut X509_STORE = 0 as *mut X509_STORE;
    let mut cert_ctx: *mut X509_STORE_CTX = 0 as *mut X509_STORE_CTX;
    let mut signerCerts: *mut stack_st_X509 = 0 as *mut stack_st_X509;
    let mut intermediateCAs: *mut stack_st_X509 = 0 as *mut stack_st_X509;
    let mut signerRevoked: *mut stack_st_X509_CRL =
        0 as *mut stack_st_X509_CRL;
    let mut revoked: *mut stack_st_X509_CRL = 0 as *mut stack_st_X509_CRL;
    let mut verified_chain: *mut stack_st_X509 = 0 as *mut stack_st_X509;
    let mut oid: *mut ASN1_OBJECT = 0 as *mut ASN1_OBJECT;
    let mut type_0: *const ASN1_OBJECT = 0 as *const ASN1_OBJECT;
    let mut etype: *const ASN1_OBJECT = 0 as *const ASN1_OBJECT;
    let mut octets: *mut *mut ASN1_OCTET_STRING =
        0 as *mut *mut ASN1_OCTET_STRING;
    let mut krb5_verified_chain:
            *mut *mut krb5_external_principal_identifier =
        0 as *mut *mut krb5_external_principal_identifier;
    let mut authz: *mut krb5_data = 0 as *mut krb5_data;
    let mut buf: [libc::c_char; 256] = [0; 256];
    if !is_signed.is_null() { *is_signed = 1 as libc::c_int }
    oid = pkinit_pkcs7type2oid(plgctx, cms_msg_type);
    if !oid.is_null() {
        /* decode received CMS message */
        cms =
            d2i_CMS_ContentInfo(0 as *mut *mut CMS_ContentInfo, &mut p,
                                signed_data_len as libc::c_int as
                                    libc::c_long);
        if cms.is_null() {
            retval =
                oerr(context, 0 as libc::c_int,
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"Failed to decode CMS message\x00" as *const u8
                                  as *const libc::c_char))
        } else {
            etype = CMS_get0_eContentType(cms);
            /*
     * Prior to 1.10 the MIT client incorrectly emitted the pkinit structure
     * directly in a CMS ContentInfo rather than using SignedData with no
     * signers. Handle that case.
     */
            type_0 = CMS_get0_type(cms);
            if !is_signed.is_null() && OBJ_cmp(type_0, oid) == 0 {
                let mut d: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                *is_signed = 0 as libc::c_int;
                octets = CMS_get0_content(cms);
                if octets.is_null() || (**octets).type_0 != 4 as libc::c_int {
                    retval = -(1765328360 as libc::c_long) as krb5_error_code;
                    krb5_set_error_message(context, retval,
                                           dgettext(b"mit-krb5\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    b"Invalid pkinit packet: octet string expected\x00"
                                                        as *const u8 as
                                                        *const libc::c_char));
                    current_block = 9715249154490124015;
                } else {
                    *data_len = ASN1_STRING_length(*octets) as libc::c_uint;
                    d =
                        malloc(*data_len as libc::c_ulong) as
                            *mut libc::c_uchar;
                    if d.is_null() {
                        retval = 12 as libc::c_int;
                        current_block = 9715249154490124015;
                    } else {
                        memcpy(d as *mut libc::c_void,
                               ASN1_STRING_get0_data(*octets) as
                                   *const libc::c_void,
                               *data_len as libc::c_ulong);
                        *data = d;
                        current_block = 6184902848125730047;
                    }
                }
            } else if OBJ_obj2nid(type_0) != 22 as libc::c_int {
                pkiDebug(b"Expected id-signedData CMS msg (received type = %d)\n\x00"
                             as *const u8 as *const libc::c_char,
                         OBJ_obj2nid(type_0));
                krb5_set_error_message(context, retval,
                                       dgettext(b"mit-krb5\x00" as *const u8
                                                    as *const libc::c_char,
                                                b"wrong oid\n\x00" as
                                                    *const u8 as
                                                    *const libc::c_char));
                current_block = 9715249154490124015;
            } else {
                /* Verify that the received message is CMS SignedData message. */
                /* setup to verify X509 certificate used to sign CMS message */
                store = X509_STORE_new();
                if store.is_null() {
                    current_block = 9715249154490124015;
                } else {
                    /* check if we are inforcing CRL checking */
                    vflags =
                        (0x4 as libc::c_int | 0x8 as libc::c_int) as
                            libc::c_uint;
                    if require_crl_checking != 0 {
                        X509_STORE_set_verify_cb(store,
                                                 Some(openssl_callback as
                                                          unsafe extern "C" fn(_:
                                                                                   libc::c_int,
                                                                               _:
                                                                                   *mut X509_STORE_CTX)
                                                              ->
                                                                  libc::c_int));
                    } else {
                        X509_STORE_set_verify_cb(store,
                                                 Some(openssl_callback_ignore_crls
                                                          as
                                                          unsafe extern "C" fn(_:
                                                                                   libc::c_int,
                                                                               _:
                                                                                   *mut X509_STORE_CTX)
                                                              ->
                                                                  libc::c_int));
                    }
                    X509_STORE_set_flags(store, vflags as libc::c_ulong);
                    /*
     * Get the signer's information from the CMS message.  Match signer ID
     * against anchors and intermediate CAs in case no certs are present in the
     * SignedData.  If we start sending kdcPkId values in requests, we'll need
     * to match against the source of that information too.
     */
                    CMS_set1_signers_certs(cms, 0 as *mut stack_st_X509,
                                           0 as libc::c_int as
                                               libc::c_uint); /* message was signed */
                    CMS_set1_signers_certs(cms, (*idctx).trustedCAs,
                                           0x10 as libc::c_int as
                                               libc::c_uint);
                    CMS_set1_signers_certs(cms, (*idctx).intermediateCAs,
                                           0x10 as libc::c_int as
                                               libc::c_uint);
                    si_sk = CMS_get0_SignerInfos(cms);
                    if si_sk.is_null() ||
                           {
                               si =
                                   sk_CMS_SignerInfo_value(si_sk,
                                                           0 as libc::c_int);
                               si.is_null()
                           } {
                        /* Not actually signed; anonymous case */
                        if is_signed.is_null() {
                            current_block = 9715249154490124015;
                        } else {
                            *is_signed = 0 as libc::c_int;
                            /* We cannot use CMS_dataInit because there may be no digest */
                            octets = CMS_get0_content(cms);
                            if !octets.is_null() {
                                out =
                                    BIO_new_mem_buf((**octets).data as
                                                        *const libc::c_void,
                                                    (**octets).length)
                            }
                            if out.is_null() {
                                current_block = 9715249154490124015;
                            } else { current_block = 17736998403848444560; }
                        }
                    } else {
                        CMS_SignerInfo_get0_algs(si, 0 as *mut *mut EVP_PKEY,
                                                 &mut x,
                                                 0 as *mut *mut X509_ALGOR,
                                                 0 as *mut *mut X509_ALGOR);
                        if x.is_null() {
                            current_block = 9715249154490124015;
                        } else {
                            /* create available CRL information (get local CRLs and include CRLs
         * received in the CMS message
         */
                            signerRevoked = CMS_get1_crls(cms);
                            if (*idctx).revoked.is_null() {
                                revoked = signerRevoked
                            } else if signerRevoked.is_null() {
                                revoked = (*idctx).revoked
                            } else {
                                size =
                                    sk_X509_CRL_num((*idctx).revoked) as
                                        libc::c_uint;
                                revoked = sk_X509_CRL_new_null();
                                i = 0 as libc::c_int as libc::c_uint;
                                while i < size {
                                    sk_X509_CRL_push(revoked,
                                                     sk_X509_CRL_value((*idctx).revoked,
                                                                       i as
                                                                           libc::c_int));
                                    i = i.wrapping_add(1)
                                }
                                size =
                                    sk_X509_CRL_num(signerRevoked) as
                                        libc::c_uint;
                                i = 0 as libc::c_int as libc::c_uint;
                                while i < size {
                                    sk_X509_CRL_push(revoked,
                                                     sk_X509_CRL_value(signerRevoked,
                                                                       i as
                                                                           libc::c_int));
                                    i = i.wrapping_add(1)
                                }
                            }
                            /* create available intermediate CAs chains (get local intermediateCAs and
         * include the CA chain received in the CMS message
         */
                            signerCerts = CMS_get1_certs(cms);
                            if (*idctx).intermediateCAs.is_null() {
                                intermediateCAs = signerCerts
                            } else if signerCerts.is_null() {
                                intermediateCAs = (*idctx).intermediateCAs
                            } else {
                                size =
                                    sk_X509_num((*idctx).intermediateCAs) as
                                        libc::c_uint;
                                intermediateCAs = sk_X509_new_null();
                                i = 0 as libc::c_int as libc::c_uint;
                                while i < size {
                                    sk_X509_push(intermediateCAs,
                                                 sk_X509_value((*idctx).intermediateCAs,
                                                               i as
                                                                   libc::c_int));
                                    i = i.wrapping_add(1)
                                }
                                size =
                                    sk_X509_num(signerCerts) as libc::c_uint;
                                i = 0 as libc::c_int as libc::c_uint;
                                while i < size {
                                    sk_X509_push(intermediateCAs,
                                                 sk_X509_value(signerCerts,
                                                               i as
                                                                   libc::c_int));
                                    i = i.wrapping_add(1)
                                }
                            }
                            /* initialize x509 context with the received certificate and
         * trusted and intermediate CA chains and CRLs
         */
                            cert_ctx = X509_STORE_CTX_new();
                            if cert_ctx.is_null() {
                                current_block = 9715249154490124015;
                            } else if X509_STORE_CTX_init(cert_ctx, store, x,
                                                          intermediateCAs) ==
                                          0 {
                                current_block = 9715249154490124015;
                            } else {
                                X509_STORE_CTX_set0_crls(cert_ctx, revoked);
                                /* add trusted CAs certificates for cert verification */
                                if !(*idctx).trustedCAs.is_null() {
                                    X509_STORE_CTX_set0_trusted_stack(cert_ctx,
                                                                      (*idctx).trustedCAs);
                                    i =
                                        X509_verify_cert(cert_ctx) as
                                            libc::c_uint;
                                    if i <= 0 as libc::c_int as libc::c_uint {
                                        let mut j: libc::c_int =
                                            X509_STORE_CTX_get_error(cert_ctx);
                                        let mut cert: *mut X509 =
                                            0 as *mut X509;
                                        cert =
                                            X509_STORE_CTX_get_current_cert(cert_ctx);
                                        (*reqctx).received_cert =
                                            X509_dup(cert);
                                        match j {
                                            23 => {
                                                retval =
                                                    -(1765328312 as
                                                          libc::c_long) as
                                                        krb5_error_code
                                            }
                                            3 => {
                                                retval =
                                                    -(1765328311 as
                                                          libc::c_long) as
                                                        krb5_error_code
                                            }
                                            2 | 20 => {
                                                retval =
                                                    -(1765328314 as
                                                          libc::c_long) as
                                                        krb5_error_code
                                            }
                                            _ => {
                                                retval =
                                                    -(1765328313 as
                                                          libc::c_long) as
                                                        krb5_error_code
                                            }
                                        }
                                        oerr_cert(context, retval, cert_ctx,
                                                  dgettext(b"mit-krb5\x00" as
                                                               *const u8 as
                                                               *const libc::c_char,
                                                           b"Failed to verify received certificate\x00"
                                                               as *const u8 as
                                                               *const libc::c_char));
                                        if (*reqctx).received_cert.is_null() {
                                            krb5int_strlcpy(buf.as_mut_ptr(),
                                                            b"(none)\x00" as
                                                                *const u8 as
                                                                *const libc::c_char,
                                                            ::std::mem::size_of::<[libc::c_char; 256]>()
                                                                as
                                                                libc::c_ulong);
                                        } else {
                                            X509_NAME_oneline(X509_get_subject_name((*reqctx).received_cert),
                                                              buf.as_mut_ptr(),
                                                              ::std::mem::size_of::<[libc::c_char; 256]>()
                                                                  as
                                                                  libc::c_ulong
                                                                  as
                                                                  libc::c_int);
                                        }
                                        pkiDebug(b"problem with cert DN = %s (error=%d) %s\n\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 buf.as_mut_ptr(), j,
                                                 X509_verify_cert_error_string(j
                                                                                   as
                                                                                   libc::c_long));
                                    } else if cms_msg_type ==
                                                  CMS_SIGN_CLIENT as
                                                      libc::c_int {
                                        verified_chain =
                                            X509_STORE_CTX_get1_chain(cert_ctx)
                                    }
                                    X509_STORE_CTX_free(cert_ctx);
                                    if i <= 0 as libc::c_int as libc::c_uint {
                                        current_block = 9715249154490124015;
                                    } else {
                                        out = BIO_new(BIO_s_mem());
                                        if CMS_verify(cms,
                                                      0 as *mut stack_st_X509,
                                                      store, 0 as *mut BIO,
                                                      out,
                                                      flags as libc::c_uint)
                                               == 0 as libc::c_int {
                                            let mut err: libc::c_ulong =
                                                ERR_peek_error();
                                            match (err &
                                                       0xfff as libc::c_long
                                                           as libc::c_ulong)
                                                      as libc::c_int {
                                                101 => {
                                                    retval =
                                                        -(1765328304 as
                                                              libc::c_long) as
                                                            krb5_error_code
                                                }
                                                105 | _ => {
                                                    retval =
                                                        -(1765328320 as
                                                              libc::c_long) as
                                                            krb5_error_code
                                                }
                                            }
                                            oerr(context, retval,
                                                 dgettext(b"mit-krb5\x00" as
                                                              *const u8 as
                                                              *const libc::c_char,
                                                          b"Failed to verify CMS message\x00"
                                                              as *const u8 as
                                                              *const libc::c_char));
                                            current_block =
                                                9715249154490124015;
                                        } else {
                                            current_block =
                                                17736998403848444560;
                                        }
                                    }
                                } else {
                                    pkiDebug(b"unable to find any trusted CAs\n\x00"
                                                 as *const u8 as
                                                 *const libc::c_char);
                                    current_block = 9715249154490124015;
                                }
                            }
                        }
                    }
                    match current_block {
                        9715249154490124015 => { }
                        _ => {
                            if OBJ_cmp(etype, oid) == 0 {
                                valid_oid = 1 as libc::c_int
                            }
                            if valid_oid != 0 {
                                pkiDebug(b"CMS Verification successful\n\x00"
                                             as *const u8 as
                                             *const libc::c_char);
                                /* retrieve verified certificate chain */
                                /* transfer the data from CMS message into return buffer */
                                size = 0 as libc::c_int as libc::c_uint;
                                loop  {
                                    let mut remain: libc::c_int = 0;
                                    retval = 12 as libc::c_int;
                                    *data =
                                        realloc(*data as *mut libc::c_void,
                                                size.wrapping_add((1024 as
                                                                       libc::c_int
                                                                       *
                                                                       10 as
                                                                           libc::c_int)
                                                                      as
                                                                      libc::c_uint)
                                                    as libc::c_ulong) as
                                            *mut libc::c_uchar;
                                    if (*data).is_null() {
                                        current_block = 9715249154490124015;
                                        break ;
                                    }
                                    remain =
                                        BIO_read(out,
                                                 &mut *(*data).offset(size as
                                                                          isize)
                                                     as *mut libc::c_uchar as
                                                     *mut libc::c_void,
                                                 1024 as libc::c_int *
                                                     10 as libc::c_int);
                                    if remain <= 0 as libc::c_int {
                                        current_block = 7545150590528655645;
                                        break ;
                                    }
                                    size =
                                        size.wrapping_add(remain as
                                                              libc::c_uint)
                                }
                                match current_block {
                                    9715249154490124015 => { }
                                    _ => {
                                        *data_len = size;
                                        if !x.is_null() {
                                            (*reqctx).received_cert =
                                                X509_dup(x);
                                            /* generate authorization data */
                                            if cms_msg_type ==
                                                   CMS_SIGN_CLIENT as
                                                       libc::c_int {
                                                if authz_data.is_null() ||
                                                       authz_data_len.is_null()
                                                   {
                                                    current_block =
                                                        6184902848125730047;
                                                } else {
                                                    *authz_data =
                                                        0 as
                                                            *mut libc::c_uchar;
                                                    retval =
                                                        create_identifiers_from_stack(verified_chain,
                                                                                      &mut krb5_verified_chain);
                                                    if retval != 0 {
                                                        pkiDebug(b"create_identifiers_from_stack failed\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
                                                        current_block =
                                                            9715249154490124015;
                                                    } else {
                                                        retval =
                                                            k5int_encode_krb5_td_trusted_certifiers.expect("non-null function pointer")(krb5_verified_chain
                                                                                                                                            as
                                                                                                                                            *const *mut krb5_external_principal_identifier,
                                                                                                                                        &mut authz);
                                                        if retval != 0 {
                                                            pkiDebug(b"encode_krb5_td_trusted_certifiers failed\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char);
                                                            current_block =
                                                                9715249154490124015;
                                                        } else {
                                                            *authz_data =
                                                                malloc((*authz).length
                                                                           as
                                                                           libc::c_ulong)
                                                                    as
                                                                    *mut libc::c_uchar;
                                                            if (*authz_data).is_null()
                                                               {
                                                                retval =
                                                                    12 as
                                                                        libc::c_int;
                                                                current_block
                                                                    =
                                                                    9715249154490124015;
                                                            } else {
                                                                memcpy(*authz_data
                                                                           as
                                                                           *mut libc::c_void,
                                                                       (*authz).data
                                                                           as
                                                                           *const libc::c_void,
                                                                       (*authz).length
                                                                           as
                                                                           libc::c_ulong);
                                                                *authz_data_len
                                                                    =
                                                                    (*authz).length;
                                                                current_block
                                                                    =
                                                                    6184902848125730047;
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                current_block =
                                                    6184902848125730047;
                                            }
                                        } else {
                                            current_block =
                                                6184902848125730047;
                                        }
                                    }
                                }
                            } else {
                                pkiDebug(b"wrong oid in eContentType\n\x00" as
                                             *const u8 as
                                             *const libc::c_char);
                                print_buffer(OBJ_get0_data(etype),
                                             OBJ_length(etype) as
                                                 libc::c_uint);
                                retval =
                                    -(1765328360 as libc::c_long) as
                                        krb5_error_code;
                                krb5_set_error_message(context, retval,
                                                       b"wrong oid\n\x00" as
                                                           *const u8 as
                                                           *const libc::c_char);
                                current_block = 9715249154490124015;
                            }
                        }
                    }
                }
            }
            match current_block {
                9715249154490124015 => { }
                _ => { retval = 0 as libc::c_int }
            }
        }
    }
    if !out.is_null() { BIO_free(out); }
    if !store.is_null() { X509_STORE_free(store); }
    if !cms.is_null() {
        if !signerCerts.is_null() {
            sk_X509_pop_free(signerCerts,
                             Some(X509_free as
                                      unsafe extern "C" fn(_: *mut X509)
                                          -> ()));
        }
        if !(*idctx).intermediateCAs.is_null() && !signerCerts.is_null() {
            sk_X509_free(intermediateCAs);
        }
        if !signerRevoked.is_null() {
            sk_X509_CRL_pop_free(signerRevoked,
                                 Some(X509_CRL_free as
                                          unsafe extern "C" fn(_:
                                                                   *mut X509_CRL)
                                              -> ()));
        }
        if !(*idctx).revoked.is_null() && !signerRevoked.is_null() {
            sk_X509_CRL_free(revoked);
        }
        CMS_ContentInfo_free(cms);
    }
    if !verified_chain.is_null() {
        sk_X509_pop_free(verified_chain,
                         Some(X509_free as
                                  unsafe extern "C" fn(_: *mut X509) -> ()));
    }
    if !krb5_verified_chain.is_null() {
        free_krb5_external_principal_identifier(&mut krb5_verified_chain);
    }
    if !authz.is_null() { krb5_free_data(context, authz); }
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "1784:1"]
pub unsafe extern "C" fn cms_envelopeddata_create(mut context: krb5_context,
                                                  mut plgctx:
                                                      pkinit_plg_crypto_context,
                                                  mut reqctx:
                                                      pkinit_req_crypto_context,
                                                  mut idctx:
                                                      pkinit_identity_crypto_context,
                                                  mut pa_type:
                                                      krb5_preauthtype,
                                                  mut include_certchain:
                                                      libc::c_int,
                                                  mut key_pack:
                                                      *mut libc::c_uchar,
                                                  mut key_pack_len:
                                                      libc::c_uint,
                                                  mut out:
                                                      *mut *mut libc::c_uchar,
                                                  mut out_len:
                                                      *mut libc::c_uint)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut p7: *mut PKCS7 = 0 as *mut PKCS7;
    let mut in_0: *mut BIO = 0 as *mut BIO;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut signed_data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut enc_data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut signed_data_len: libc::c_int = 0 as libc::c_int;
    let mut enc_data_len: libc::c_int = 0 as libc::c_int;
    let mut flags: libc::c_int = 0x80 as libc::c_int;
    let mut encerts: *mut stack_st_X509 = 0 as *mut stack_st_X509;
    let mut cipher: *const EVP_CIPHER = 0 as *const EVP_CIPHER;
    retval =
        cms_signeddata_create(context, plgctx, reqctx, idctx,
                              CMS_ENVEL_SERVER as libc::c_int,
                              include_certchain, key_pack, key_pack_len,
                              &mut signed_data,
                              &mut signed_data_len as *mut libc::c_int as
                                  *mut libc::c_uint);
    if retval != 0 {
        pkiDebug(b"failed to create pkcs7 signed data\n\x00" as *const u8 as
                     *const libc::c_char);
    } else if (*reqctx).received_cert.is_null() {
        retval = -(1765328360 as libc::c_long) as krb5_error_code
    } else {
        encerts = sk_X509_new_null();
        sk_X509_push(encerts, (*reqctx).received_cert);
        cipher = EVP_des_ede3_cbc();
        in_0 = BIO_new(BIO_s_mem());
        prepare_enc_data(signed_data, signed_data_len, &mut enc_data,
                         &mut enc_data_len);
        retval =
            BIO_write(in_0, enc_data as *const libc::c_void, enc_data_len);
        if retval != enc_data_len {
            pkiDebug(b"BIO_write only wrote %d\n\x00" as *const u8 as
                         *const libc::c_char, retval);
        } else {
            p7 = PKCS7_encrypt(encerts, in_0, cipher, flags);
            if p7.is_null() {
                retval =
                    oerr(context, 0 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"Failed to encrypt PKCS7 object\x00" as
                                      *const u8 as *const libc::c_char))
            } else {
                (*(*(*p7).d.enveloped).enc_data).content_type =
                    OBJ_nid2obj(22 as libc::c_int);
                *out_len =
                    i2d_PKCS7(p7, 0 as *mut *mut libc::c_uchar) as
                        libc::c_uint;
                if *out_len == 0 ||
                       {
                           *out =
                               malloc(*out_len as libc::c_ulong) as
                                   *mut libc::c_uchar;
                           p = *out;
                           p.is_null()
                       } {
                    retval = 12 as libc::c_int
                } else {
                    retval = i2d_PKCS7(p7, &mut p);
                    if retval == 0 {
                        retval =
                            oerr(context, 0 as libc::c_int,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"Failed to DER encode PKCS7\x00" as
                                              *const u8 as
                                              *const libc::c_char))
                    } else { retval = 0 as libc::c_int }
                }
            }
        }
    }
    if !p7.is_null() { PKCS7_free(p7); }
    if !in_0.is_null() { BIO_free(in_0); }
    free(signed_data as *mut libc::c_void);
    free(enc_data as *mut libc::c_void);
    if !encerts.is_null() { sk_X509_free(encerts); }
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "1868:1"]
pub unsafe extern "C" fn cms_envelopeddata_verify(mut context: krb5_context,
                                                  mut plg_cryptoctx:
                                                      pkinit_plg_crypto_context,
                                                  mut req_cryptoctx:
                                                      pkinit_req_crypto_context,
                                                  mut id_cryptoctx:
                                                      pkinit_identity_crypto_context,
                                                  mut pa_type:
                                                      krb5_preauthtype,
                                                  mut require_crl_checking:
                                                      libc::c_int,
                                                  mut enveloped_data:
                                                      *mut libc::c_uchar,
                                                  mut enveloped_data_len:
                                                      libc::c_uint,
                                                  mut data:
                                                      *mut *mut libc::c_uchar,
                                                  mut data_len:
                                                      *mut libc::c_uint)
 -> krb5_error_code {
    let mut retval: krb5_error_code =
        -(1765328360 as libc::c_long) as krb5_error_code;
    let mut p7: *mut PKCS7 = 0 as *mut PKCS7;
    let mut p: *const libc::c_uchar = enveloped_data;
    let mut tmp_buf_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut tmp_buf2_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut vfy_buf_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut tmp_buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp_buf2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut vfy_buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    /* check we have client's certificate */
    /* decode received PKCS7 message */
    p7 =
        d2i_PKCS7(0 as *mut *mut PKCS7, &mut p,
                  enveloped_data_len as libc::c_int as libc::c_long);
    if p7.is_null() {
        retval =
            oerr(context, 0 as libc::c_int,
                 dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                          b"Failed to decode PKCS7\x00" as *const u8 as
                              *const libc::c_char))
    } else if OBJ_obj2nid((*p7).type_0) != 23 as libc::c_int {
        pkiDebug(b"Expected id-enveloped PKCS7 msg (received type = %d)\n\x00"
                     as *const u8 as *const libc::c_char,
                 OBJ_obj2nid((*p7).type_0));
        krb5_set_error_message(context, retval,
                               b"wrong oid\n\x00" as *const u8 as
                                   *const libc::c_char);
    } else if pkcs7_decrypt(context, id_cryptoctx, p7, &mut tmp_buf,
                            &mut tmp_buf_len) != 0 {
        pkiDebug(b"PKCS7 decryption successful\n\x00" as *const u8 as
                     *const libc::c_char);
        /* verify that the received message is PKCS7 EnvelopedData message */
        /* decrypt received PKCS7 message */
        /* verify PKCS7 SignedData message */
    /* Wrap the signed data to make decoding easier in the verify routine. */
        retval =
            wrap_signeddata(tmp_buf, tmp_buf_len, &mut tmp_buf2,
                            &mut tmp_buf2_len);
        if retval != 0 {
            pkiDebug(b"failed to encode signeddata\n\x00" as *const u8 as
                         *const libc::c_char);
        } else {
            vfy_buf = tmp_buf2;
            vfy_buf_len = tmp_buf2_len;
            retval =
                cms_signeddata_verify(context, plg_cryptoctx, req_cryptoctx,
                                      id_cryptoctx,
                                      CMS_ENVEL_SERVER as libc::c_int,
                                      require_crl_checking, vfy_buf,
                                      vfy_buf_len, data, data_len,
                                      0 as *mut *mut libc::c_uchar,
                                      0 as *mut libc::c_uint,
                                      0 as *mut libc::c_int);
            if retval == 0 {
                pkiDebug(b"PKCS7 Verification Success\n\x00" as *const u8 as
                             *const libc::c_char);
                retval = 0 as libc::c_int
            } else {
                pkiDebug(b"PKCS7 Verification Failure\n\x00" as *const u8 as
                             *const libc::c_char);
            }
        }
    } else {
        retval =
            oerr(context, 0 as libc::c_int,
                 dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                          b"Failed to decrypt PKCS7 message\x00" as *const u8
                              as *const libc::c_char))
    }
    if !p7.is_null() { PKCS7_free(p7); }
    free(tmp_buf as *mut libc::c_void);
    free(tmp_buf2 as *mut libc::c_void);
    return retval;
}
#[c2rust::src_loc = "1954:1"]
unsafe extern "C" fn crypto_retrieve_X509_sans(mut context: krb5_context,
                                               mut plgctx:
                                                   pkinit_plg_crypto_context,
                                               mut reqctx:
                                                   pkinit_req_crypto_context,
                                               mut cert: *mut X509,
                                               mut princs_ret:
                                                   *mut *mut krb5_principal,
                                               mut upn_ret:
                                                   *mut *mut *mut libc::c_char,
                                               mut dns_ret:
                                                   *mut *mut *mut libc::c_uchar)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 22 as libc::c_int;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut p: libc::c_int = 0 as libc::c_int;
    let mut u: libc::c_int = 0 as libc::c_int;
    let mut d: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut l: libc::c_int = 0;
    let mut princs: *mut krb5_principal = 0 as *mut krb5_principal;
    let mut upns: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut dnss: *mut *mut libc::c_uchar = 0 as *mut *mut libc::c_uchar;
    let mut i: libc::c_uint = 0;
    let mut num_found: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut num_sans: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut ext: *mut X509_EXTENSION = 0 as *mut X509_EXTENSION;
    let mut ialt: *mut GENERAL_NAMES = 0 as *mut GENERAL_NAMES;
    let mut gen: *mut GENERAL_NAME = 0 as *mut GENERAL_NAME;
    if !princs_ret.is_null() { *princs_ret = 0 as *mut krb5_principal }
    if !upn_ret.is_null() { *upn_ret = 0 as *mut *mut libc::c_char }
    if !dns_ret.is_null() { *dns_ret = 0 as *mut *mut libc::c_uchar }
    if princs_ret.is_null() && upn_ret.is_null() && dns_ret.is_null() {
        pkiDebug(b"%s: nowhere to return any values!\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 26],
                                           &[libc::c_char; 26]>(b"crypto_retrieve_X509_sans\x00")).as_ptr());
        return retval
    }
    if cert.is_null() {
        pkiDebug(b"%s: no certificate!\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 26],
                                           &[libc::c_char; 26]>(b"crypto_retrieve_X509_sans\x00")).as_ptr());
        return retval
    }
    X509_NAME_oneline(X509_get_subject_name(cert), buf.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 256]>() as
                          libc::c_ulong as libc::c_int);
    pkiDebug(b"%s: looking for SANs in cert = %s\n\x00" as *const u8 as
                 *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 26],
                                       &[libc::c_char; 26]>(b"crypto_retrieve_X509_sans\x00")).as_ptr(),
             buf.as_mut_ptr());
    l = X509_get_ext_by_NID(cert, 85 as libc::c_int, -(1 as libc::c_int));
    if l < 0 as libc::c_int { return 0 as libc::c_int }
    ext = X509_get_ext(cert, l);
    if ext.is_null() ||
           {
               ialt = X509V3_EXT_d2i(ext) as *mut GENERAL_NAMES;
               ialt.is_null()
           } {
        pkiDebug(b"%s: found no subject alt name extensions\n\x00" as
                     *const u8 as *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 26],
                                           &[libc::c_char; 26]>(b"crypto_retrieve_X509_sans\x00")).as_ptr());
    } else {
        num_sans = sk_GENERAL_NAME_num(ialt) as libc::c_uint;
        pkiDebug(b"%s: found %d subject alt name extension(s)\n\x00" as
                     *const u8 as *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 26],
                                           &[libc::c_char; 26]>(b"crypto_retrieve_X509_sans\x00")).as_ptr(),
                 num_sans);
        /* OK, we're likely returning something. Allocate return values */
        if !princs_ret.is_null() {
            princs =
                calloc(num_sans.wrapping_add(1 as libc::c_int as libc::c_uint)
                           as libc::c_ulong,
                       ::std::mem::size_of::<krb5_principal>() as
                           libc::c_ulong) as *mut krb5_principal;
            if princs.is_null() {
                retval = 12 as libc::c_int;
                current_block = 3309414955773600855;
            } else { current_block = 11459959175219260272; }
        } else { current_block = 11459959175219260272; }
        match current_block {
            3309414955773600855 => { }
            _ => {
                if !upn_ret.is_null() {
                    upns =
                        calloc(num_sans.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                   libc::c_ulong,
                               ::std::mem::size_of::<*mut libc::c_char>() as
                                   libc::c_ulong) as *mut *mut libc::c_char;
                    if upns.is_null() {
                        retval = 12 as libc::c_int;
                        current_block = 3309414955773600855;
                    } else { current_block = 17184638872671510253; }
                } else { current_block = 17184638872671510253; }
                match current_block {
                    3309414955773600855 => { }
                    _ => {
                        if !dns_ret.is_null() {
                            dnss =
                                calloc(num_sans.wrapping_add(1 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
                                           as libc::c_ulong,
                                       ::std::mem::size_of::<*mut libc::c_uchar>()
                                           as libc::c_ulong) as
                                    *mut *mut libc::c_uchar;
                            if dnss.is_null() {
                                retval = 12 as libc::c_int;
                                current_block = 3309414955773600855;
                            } else { current_block = 572715077006366937; }
                        } else { current_block = 572715077006366937; }
                        match current_block {
                            3309414955773600855 => { }
                            _ => {
                                i = 0 as libc::c_int as libc::c_uint;
                                loop  {
                                    if !(i < num_sans) {
                                        current_block = 3736434875406665187;
                                        break ;
                                    }
                                    let mut name: krb5_data =
                                        {
                                            let mut init =
                                                _krb5_data{magic:
                                                               0 as
                                                                   libc::c_int,
                                                           length:
                                                               0 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint,
                                                           data:
                                                               0 as
                                                                   *mut libc::c_char,};
                                            init
                                        };
                                    gen =
                                        sk_GENERAL_NAME_value(ialt,
                                                              i as
                                                                  libc::c_int);
                                    match (*gen).type_0 {
                                        0 => {
                                            name.length =
                                                (*(*(*(*gen).d.otherName).value).value.sequence).length
                                                    as libc::c_uint;
                                            name.data =
                                                (*(*(*(*gen).d.otherName).value).value.sequence).data
                                                    as *mut libc::c_char;
                                            if !princs.is_null() &&
                                                   OBJ_cmp((*plgctx).id_pkinit_san,
                                                           (*(*gen).d.otherName).type_id)
                                                       == 0 as libc::c_int {
                                                ret =
                                                    k5int_decode_krb5_principal_name.expect("non-null function pointer")(&mut name,
                                                                                                                         &mut *princs.offset(p
                                                                                                                                                 as
                                                                                                                                                 isize));
                                                if ret != 0 {
                                                    pkiDebug(b"%s: failed decoding pkinit san value\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             (*::std::mem::transmute::<&[u8; 26],
                                                                                       &[libc::c_char; 26]>(b"crypto_retrieve_X509_sans\x00")).as_ptr());
                                                } else {
                                                    p += 1;
                                                    num_found =
                                                        num_found.wrapping_add(1)
                                                }
                                            } else if !upns.is_null() &&
                                                          OBJ_cmp((*plgctx).id_ms_san_upn,
                                                                  (*(*gen).d.otherName).type_id)
                                                              ==
                                                              0 as libc::c_int
                                             {
                                                /* Prevent abuse of embedded null characters. */
                                                if memchr(name.data as
                                                              *const libc::c_void,
                                                          '\u{0}' as i32,
                                                          name.length as
                                                              libc::c_ulong).is_null()
                                                   {
                                                    let ref mut fresh0 =
                                                        *upns.offset(u as
                                                                         isize);
                                                    *fresh0 =
                                                        k5memdup0(name.data as
                                                                      *const libc::c_void,
                                                                  name.length
                                                                      as
                                                                      size_t,
                                                                  &mut ret) as
                                                            *mut libc::c_char;
                                                    if (*upns.offset(u as
                                                                         isize)).is_null()
                                                       {
                                                        current_block =
                                                            3309414955773600855;
                                                        break ;
                                                    }
                                                }
                                            } else {
                                                pkiDebug(b"%s: unrecognized othername oid in SAN\n\x00"
                                                             as *const u8 as
                                                             *const libc::c_char,
                                                         (*::std::mem::transmute::<&[u8; 26],
                                                                                   &[libc::c_char; 26]>(b"crypto_retrieve_X509_sans\x00")).as_ptr());
                                            }
                                        }
                                        2 => {
                                            if !dnss.is_null() {
                                                /* Prevent abuse of embedded null characters. */
                                                if memchr((*(*gen).d.dNSName).data
                                                              as
                                                              *const libc::c_void,
                                                          '\u{0}' as i32,
                                                          (*(*gen).d.dNSName).length
                                                              as
                                                              libc::c_ulong).is_null()
                                                   {
                                                    pkiDebug(b"%s: found dns name = %s\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             (*::std::mem::transmute::<&[u8; 26],
                                                                                       &[libc::c_char; 26]>(b"crypto_retrieve_X509_sans\x00")).as_ptr(),
                                                             (*(*gen).d.dNSName).data);
                                                    let ref mut fresh1 =
                                                        *dnss.offset(d as
                                                                         isize);
                                                    *fresh1 =
                                                        strdup((*(*gen).d.dNSName).data
                                                                   as
                                                                   *mut libc::c_char)
                                                            as
                                                            *mut libc::c_uchar;
                                                    if (*dnss.offset(d as
                                                                         isize)).is_null()
                                                       {
                                                        pkiDebug(b"%s: failed to duplicate dns name\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 (*::std::mem::transmute::<&[u8; 26],
                                                                                           &[libc::c_char; 26]>(b"crypto_retrieve_X509_sans\x00")).as_ptr());
                                                    } else {
                                                        d += 1;
                                                        num_found =
                                                            num_found.wrapping_add(1)
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            pkiDebug(b"%s: SAN type = %d expecting %d\n\x00"
                                                         as *const u8 as
                                                         *const libc::c_char,
                                                     (*::std::mem::transmute::<&[u8; 26],
                                                                               &[libc::c_char; 26]>(b"crypto_retrieve_X509_sans\x00")).as_ptr(),
                                                     (*gen).type_0,
                                                     0 as libc::c_int);
                                        }
                                    }
                                    i = i.wrapping_add(1)
                                }
                                match current_block {
                                    3309414955773600855 => { }
                                    _ => {
                                        sk_GENERAL_NAME_pop_free(ialt,
                                                                 Some(GENERAL_NAME_free
                                                                          as
                                                                          unsafe extern "C" fn(_:
                                                                                                   *mut GENERAL_NAME)
                                                                              ->
                                                                                  ()));
                                        retval = 0 as libc::c_int;
                                        if !princs.is_null() &&
                                               !(*princs).is_null() {
                                            *princs_ret = princs;
                                            princs = 0 as *mut krb5_principal
                                        }
                                        if !upns.is_null() &&
                                               !(*upns).is_null() {
                                            *upn_ret = upns;
                                            upns = 0 as *mut *mut libc::c_char
                                        }
                                        if !dnss.is_null() &&
                                               !(*dnss).is_null() {
                                            *dns_ret = dnss;
                                            dnss =
                                                0 as *mut *mut libc::c_uchar
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    i = 0 as libc::c_int as libc::c_uint;
    while !princs.is_null() && !(*princs.offset(i as isize)).is_null() {
        krb5_free_principal(context, *princs.offset(i as isize));
        i = i.wrapping_add(1)
    }
    free(princs as *mut libc::c_void);
    i = 0 as libc::c_int as libc::c_uint;
    while !upns.is_null() && !(*upns.offset(i as isize)).is_null() {
        free(*upns.offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    free(upns as *mut libc::c_void);
    i = 0 as libc::c_int as libc::c_uint;
    while !dnss.is_null() && !(*dnss.offset(i as isize)).is_null() {
        free(*dnss.offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    free(dnss as *mut libc::c_void);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "2121:1"]
pub unsafe extern "C" fn crypto_retrieve_signer_identity(mut context:
                                                             krb5_context,
                                                         mut id_cryptoctx:
                                                             pkinit_identity_crypto_context,
                                                         mut identity:
                                                             *mut *const libc::c_char)
 -> krb5_error_code {
    *identity = (*id_cryptoctx).identity;
    if (*identity).is_null() { return 2 as libc::c_int }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "2132:1"]
pub unsafe extern "C" fn crypto_retrieve_cert_sans(mut context: krb5_context,
                                                   mut plgctx:
                                                       pkinit_plg_crypto_context,
                                                   mut reqctx:
                                                       pkinit_req_crypto_context,
                                                   mut idctx:
                                                       pkinit_identity_crypto_context,
                                                   mut princs_ret:
                                                       *mut *mut krb5_principal,
                                                   mut upn_ret:
                                                       *mut *mut *mut libc::c_char,
                                                   mut dns_ret:
                                                       *mut *mut *mut libc::c_uchar)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 22 as libc::c_int;
    if (*reqctx).received_cert.is_null() {
        pkiDebug(b"%s: No certificate!\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 26],
                                           &[libc::c_char; 26]>(b"crypto_retrieve_cert_sans\x00")).as_ptr());
        return retval
    }
    return crypto_retrieve_X509_sans(context, plgctx, reqctx,
                                     (*reqctx).received_cert, princs_ret,
                                     upn_ret, dns_ret);
}
#[no_mangle]
#[c2rust::src_loc = "2152:1"]
pub unsafe extern "C" fn crypto_check_cert_eku(mut context: krb5_context,
                                               mut plgctx:
                                                   pkinit_plg_crypto_context,
                                               mut reqctx:
                                                   pkinit_req_crypto_context,
                                               mut idctx:
                                                   pkinit_identity_crypto_context,
                                               mut checking_kdc_cert:
                                                   libc::c_int,
                                               mut allow_secondary_usage:
                                                   libc::c_int,
                                               mut valid_eku:
                                                   *mut libc::c_int)
 -> krb5_error_code {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut found_eku: libc::c_int = 0 as libc::c_int;
    let mut retval: krb5_error_code = 22 as libc::c_int;
    let mut i: libc::c_int = 0;
    *valid_eku = 0 as libc::c_int;
    if !(*reqctx).received_cert.is_null() {
        X509_NAME_oneline(X509_get_subject_name((*reqctx).received_cert),
                          buf.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 256]>() as
                              libc::c_ulong as libc::c_int);
        i =
            X509_get_ext_by_NID((*reqctx).received_cert, 126 as libc::c_int,
                                -(1 as libc::c_int));
        if i >= 0 as libc::c_int {
            let mut extusage: *mut EXTENDED_KEY_USAGE =
                0 as *mut EXTENDED_KEY_USAGE;
            extusage =
                X509_get_ext_d2i((*reqctx).received_cert, 126 as libc::c_int,
                                 0 as *mut libc::c_int, 0 as *mut libc::c_int)
                    as *mut EXTENDED_KEY_USAGE;
            if !extusage.is_null() {
                pkiDebug(b"%s: found eku info in the cert\n\x00" as *const u8
                             as *const libc::c_char,
                         (*::std::mem::transmute::<&[u8; 22],
                                                   &[libc::c_char; 22]>(b"crypto_check_cert_eku\x00")).as_ptr());
                i = 0 as libc::c_int;
                while found_eku == 0 as libc::c_int &&
                          i < sk_ASN1_OBJECT_num(extusage) {
                    let mut tmp_oid: *mut ASN1_OBJECT = 0 as *mut ASN1_OBJECT;
                    tmp_oid = sk_ASN1_OBJECT_value(extusage, i);
                    pkiDebug(b"%s: checking eku %d of %d, allow_secondary = %d\n\x00"
                                 as *const u8 as *const libc::c_char,
                             (*::std::mem::transmute::<&[u8; 22],
                                                       &[libc::c_char; 22]>(b"crypto_check_cert_eku\x00")).as_ptr(),
                             i + 1 as libc::c_int,
                             sk_ASN1_OBJECT_num(extusage),
                             allow_secondary_usage);
                    if checking_kdc_cert != 0 {
                        if OBJ_cmp(tmp_oid, (*plgctx).id_pkinit_KPKdc) ==
                               0 as libc::c_int ||
                               allow_secondary_usage != 0 &&
                                   OBJ_cmp(tmp_oid,
                                           (*plgctx).id_kp_serverAuth) ==
                                       0 as libc::c_int {
                            found_eku = 1 as libc::c_int
                        }
                    } else if OBJ_cmp(tmp_oid,
                                      (*plgctx).id_pkinit_KPClientAuth) ==
                                  0 as libc::c_int ||
                                  allow_secondary_usage != 0 &&
                                      OBJ_cmp(tmp_oid,
                                              (*plgctx).id_ms_kp_sc_logon) ==
                                          0 as libc::c_int {
                        found_eku = 1 as libc::c_int
                    }
                    i += 1
                }
            }
            EXTENDED_KEY_USAGE_free(extusage);
            if found_eku != 0 {
                let mut usage: *mut ASN1_BIT_STRING =
                    0 as *mut ASN1_BIT_STRING;
                /* check that digitalSignature KeyUsage is present */
                X509_check_ca((*reqctx).received_cert);
                usage =
                    X509_get_ext_d2i((*reqctx).received_cert,
                                     83 as libc::c_int, 0 as *mut libc::c_int,
                                     0 as *mut libc::c_int) as
                        *mut ASN1_BIT_STRING;
                if !usage.is_null() {
                    if X509_get_key_usage((*reqctx).received_cert) &
                           0x80 as libc::c_int as libc::c_uint != 0 {
                        if (*context).trace_callback.is_some() {
                            krb5int_trace(context,
                                          b"PKINIT found acceptable EKU and digitalSignature KU\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                        }
                        *valid_eku = 1 as libc::c_int
                    } else if (*context).trace_callback.is_some() {
                        krb5int_trace(context,
                                      b"PKINIT found acceptable EKU but no digitalSignature KU\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                    }
                }
                ASN1_BIT_STRING_free(usage);
            }
        }
        retval = 0 as libc::c_int
    }
    pkiDebug(b"%s: returning retval %d, valid_eku %d\n\x00" as *const u8 as
                 *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 22],
                                       &[libc::c_char; 22]>(b"crypto_check_cert_eku\x00")).as_ptr(),
             retval, *valid_eku);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "2228:1"]
pub unsafe extern "C" fn pkinit_octetstring2key(mut context: krb5_context,
                                                mut etype: krb5_enctype,
                                                mut key: *mut libc::c_uchar,
                                                mut dh_key_len: libc::c_uint,
                                                mut key_block:
                                                    *mut krb5_keyblock)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut md: [libc::c_uchar; 20] = [0; 20];
    let mut counter: libc::c_uchar = 0;
    let mut keybytes: size_t = 0;
    let mut keylength: size_t = 0;
    let mut offset: size_t = 0;
    let mut random_data: krb5_data =
        krb5_data{magic: 0,
                  length: 0,
                  data: 0 as *const libc::c_char as *mut libc::c_char,};
    buf = malloc(dh_key_len as libc::c_ulong) as *mut libc::c_uchar;
    if buf.is_null() {
        retval = 12 as libc::c_int
    } else {
        memset(buf as *mut libc::c_void, 0 as libc::c_int,
               dh_key_len as libc::c_ulong);
        counter = 0 as libc::c_int as libc::c_uchar;
        offset = 0 as libc::c_int as size_t;
        loop  {
            let mut c: SHA_CTX =
                SHA_CTX{h0: 0,
                        h1: 0,
                        h2: 0,
                        h3: 0,
                        h4: 0,
                        Nl: 0,
                        Nh: 0,
                        data: [0; 16],
                        num: 0,};
            SHA1_Init(&mut c);
            SHA1_Update(&mut c,
                        &mut counter as *mut libc::c_uchar as
                            *const libc::c_void, 1 as libc::c_int as size_t);
            SHA1_Update(&mut c, key as *const libc::c_void,
                        dh_key_len as size_t);
            SHA1_Final(md.as_mut_ptr(), &mut c);
            if (dh_key_len as libc::c_ulong).wrapping_sub(offset) <
                   ::std::mem::size_of::<[libc::c_uchar; 20]>() as
                       libc::c_ulong {
                memcpy(buf.offset(offset as isize) as *mut libc::c_void,
                       md.as_mut_ptr() as *const libc::c_void,
                       (dh_key_len as libc::c_ulong).wrapping_sub(offset));
            } else {
                memcpy(buf.offset(offset as isize) as *mut libc::c_void,
                       md.as_mut_ptr() as *const libc::c_void,
                       ::std::mem::size_of::<[libc::c_uchar; 20]>() as
                           libc::c_ulong);
            }
            offset =
                (offset as
                     libc::c_ulong).wrapping_add(::std::mem::size_of::<[libc::c_uchar; 20]>()
                                                     as libc::c_ulong) as
                    size_t as size_t;
            counter = counter.wrapping_add(1);
            if !(offset < dh_key_len as libc::c_ulong) { break ; }
        }
        (*key_block).magic = 0 as libc::c_int;
        (*key_block).enctype = etype;
        retval =
            krb5_c_keylengths(context, etype, &mut keybytes, &mut keylength);
        if !(retval != 0) {
            (*key_block).length = keylength as libc::c_uint;
            (*key_block).contents = malloc(keylength) as *mut krb5_octet;
            if (*key_block).contents.is_null() {
                retval = 12 as libc::c_int
            } else {
                random_data.length = keybytes as libc::c_uint;
                random_data.data = buf as *mut libc::c_char;
                retval =
                    krb5_c_random_to_key(context, etype, &mut random_data,
                                         key_block)
            }
        }
    }
    free(buf as *mut libc::c_void);
    /* If this is an error return, free the allocated keyblock, if any */
    if retval != 0 { krb5_free_keyblock_contents(context, key_block); }
    return retval;
}
/* *
 * Given an algorithm_identifier, this function returns the hash length
 * and EVP function associated with that algorithm.
 */
#[c2rust::src_loc = "2301:1"]
unsafe extern "C" fn pkinit_alg_values(mut context: krb5_context,
                                       mut alg_id: *const krb5_data,
                                       mut hash_bytes: *mut size_t,
                                       mut func:
                                           *mut Option<unsafe extern "C" fn()
                                                           -> *const EVP_MD>)
 -> krb5_error_code {
    *hash_bytes = 0 as libc::c_int as size_t;
    *func = None;
    if (*alg_id).length as libc::c_ulong == krb5_pkinit_sha1_oid_len &&
           0 as libc::c_int ==
               memcmp((*alg_id).data as *const libc::c_void,
                      &krb5_pkinit_sha1_oid as *const [krb5_octet; 0] as
                          *const libc::c_void, krb5_pkinit_sha1_oid_len) {
        *hash_bytes = 20 as libc::c_int as size_t;
        *func = Some(EVP_sha1 as unsafe extern "C" fn() -> *const EVP_MD);
        return 0 as libc::c_int
    } else if (*alg_id).length as libc::c_ulong == krb5_pkinit_sha256_oid_len
                  &&
                  0 as libc::c_int ==
                      memcmp((*alg_id).data as *const libc::c_void,
                             krb5_pkinit_sha256_oid.as_ptr() as
                                 *const libc::c_void,
                             krb5_pkinit_sha256_oid_len) {
        *hash_bytes = 32 as libc::c_int as size_t;
        *func = Some(EVP_sha256 as unsafe extern "C" fn() -> *const EVP_MD);
        return 0 as libc::c_int
    } else if (*alg_id).length as libc::c_ulong == krb5_pkinit_sha512_oid_len
                  &&
                  0 as libc::c_int ==
                      memcmp((*alg_id).data as *const libc::c_void,
                             krb5_pkinit_sha512_oid.as_ptr() as
                                 *const libc::c_void,
                             krb5_pkinit_sha512_oid_len) {
        *hash_bytes = 64 as libc::c_int as size_t;
        *func = Some(EVP_sha512 as unsafe extern "C" fn() -> *const EVP_MD);
        return 0 as libc::c_int
    } else {
        krb5_set_error_message(context,
                               -(1765328140 as libc::c_long) as
                                   krb5_error_code,
                               b"Bad algorithm ID passed to PK-INIT KDF.\x00"
                                   as *const u8 as *const libc::c_char);
        return -(1765328140 as libc::c_long) as krb5_error_code
    };
}
/* pkinit_alg_values() */
/* pkinit_alg_agility_kdf() --
 * This function generates a key using the KDF described in
 * draft_ietf_krb_wg_pkinit_alg_agility-04.txt.  The algorithm is
 * described as follows:
 *
 *     1.  reps = keydatalen (K) / hash length (H)
 *
 *     2.  Initialize a 32-bit, big-endian bit string counter as 1.
 *
 *     3.  For i = 1 to reps by 1, do the following:
 *
 *         -  Compute Hashi = H(counter || Z || OtherInfo).
 *
 *         -  Increment counter (modulo 2^32)
 *
 *     4.  Set key = Hash1 || Hash2 || ... so that length of key is K bytes.
 */
#[no_mangle]
#[c2rust::src_loc = "2352:1"]
pub unsafe extern "C" fn pkinit_alg_agility_kdf(mut context: krb5_context,
                                                mut secret: *mut krb5_data,
                                                mut alg_oid: *mut krb5_data,
                                                mut party_u_info:
                                                    krb5_const_principal,
                                                mut party_v_info:
                                                    krb5_const_principal,
                                                mut enctype: krb5_enctype,
                                                mut as_req: *mut krb5_data,
                                                mut pk_as_rep: *mut krb5_data,
                                                mut key_block:
                                                    *mut krb5_keyblock)
 -> krb5_error_code {
    let mut current_block: u64; /* Does this type work on Windows? */
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut reps: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut counter: uint32_t = 1 as libc::c_int as uint32_t;
    let mut offset: size_t = 0 as libc::c_int as size_t;
    let mut hash_len: size_t = 0 as libc::c_int as size_t;
    let mut rand_len: size_t = 0 as libc::c_int as size_t;
    let mut key_len: size_t = 0 as libc::c_int as size_t;
    let mut random_data: krb5_data =
        krb5_data{magic: 0,
                  length: 0,
                  data: 0 as *const libc::c_char as *mut libc::c_char,};
    let mut other_info_fields: krb5_sp80056a_other_info =
        krb5_sp80056a_other_info{algorithm_identifier:
                                     krb5_algorithm_identifier{algorithm:
                                                                   krb5_data{magic:
                                                                                 0,
                                                                             length:
                                                                                 0,
                                                                             data:
                                                                                 0
                                                                                     as
                                                                                     *const libc::c_char
                                                                                     as
                                                                                     *mut libc::c_char,},
                                                               parameters:
                                                                   krb5_data{magic:
                                                                                 0,
                                                                             length:
                                                                                 0,
                                                                             data:
                                                                                 0
                                                                                     as
                                                                                     *const libc::c_char
                                                                                     as
                                                                                     *mut libc::c_char,},},
                                 party_u_info: 0 as *mut krb5_principal_data,
                                 party_v_info: 0 as *mut krb5_principal_data,
                                 supp_pub_info:
                                     krb5_data{magic: 0,
                                               length: 0,
                                               data:
                                                   0 as *const libc::c_char as
                                                       *mut libc::c_char,},};
    let mut supp_pub_info_fields: krb5_pkinit_supp_pub_info =
        krb5_pkinit_supp_pub_info{enctype: 0,
                                  as_req:
                                      krb5_data{magic: 0,
                                                length: 0,
                                                data:
                                                    0 as *const libc::c_char
                                                        as
                                                        *mut libc::c_char,},
                                  pk_as_rep:
                                      krb5_data{magic: 0,
                                                length: 0,
                                                data:
                                                    0 as *const libc::c_char
                                                        as
                                                        *mut libc::c_char,},};
    let mut other_info: *mut krb5_data = 0 as *mut krb5_data;
    let mut supp_pub_info: *mut krb5_data = 0 as *mut krb5_data;
    let mut alg_id: krb5_algorithm_identifier =
        krb5_algorithm_identifier{algorithm:
                                      krb5_data{magic: 0,
                                                length: 0,
                                                data:
                                                    0 as *const libc::c_char
                                                        as
                                                        *mut libc::c_char,},
                                  parameters:
                                      krb5_data{magic: 0,
                                                length: 0,
                                                data:
                                                    0 as *const libc::c_char
                                                        as
                                                        *mut libc::c_char,},};
    let mut ctx: *mut EVP_MD_CTX = 0 as *mut EVP_MD_CTX;
    let mut EVP_func: Option<unsafe extern "C" fn() -> *const EVP_MD> = None;
    /* initialize random_data here to make clean-up safe */
    random_data.length = 0 as libc::c_int as libc::c_uint;
    random_data.data = 0 as *mut libc::c_char;
    /* allocate and initialize the key block */
    (*key_block).magic = 0 as libc::c_int;
    (*key_block).enctype = enctype;
    retval = krb5_c_keylengths(context, enctype, &mut rand_len, &mut key_len);
    if !(0 as libc::c_int != retval) {
        random_data.length = rand_len as libc::c_uint;
        (*key_block).length = key_len as libc::c_uint;
        (*key_block).contents =
            malloc((*key_block).length as libc::c_ulong) as *mut krb5_octet;
        if (*key_block).contents.is_null() {
            retval = 12 as libc::c_int
        } else {
            memset((*key_block).contents as *mut libc::c_void,
                   0 as libc::c_int, (*key_block).length as libc::c_ulong);
            /* If this is anonymous pkinit, use the anonymous principle for party_u_info */
            if !party_u_info.is_null() &&
                   krb5_principal_compare_any_realm(context, party_u_info,
                                                    krb5_anonymous_principal())
                       != 0 {
                party_u_info =
                    krb5_anonymous_principal() as krb5_principal as
                        krb5_const_principal
            }
            retval =
                pkinit_alg_values(context, alg_oid, &mut hash_len,
                                  &mut EVP_func);
            if !(0 as libc::c_int != retval) {
                /* 1.  reps = keydatalen (K) / hash length (H) */
                reps =
                    ((*key_block).length as
                         libc::c_ulong).wrapping_div(hash_len) as
                        libc::c_uint;
                /* ... and round up, if necessary */
                if (*key_block).length as libc::c_ulong >
                       (reps as libc::c_ulong).wrapping_mul(hash_len) {
                    reps = reps.wrapping_add(1)
                }
                /* Allocate enough space in the random data buffer to hash directly into
     * it, even if the last hash will make it bigger than the key length. */
                random_data.data =
                    malloc((reps as libc::c_ulong).wrapping_mul(hash_len)) as
                        *mut libc::c_char;
                if random_data.data.is_null() {
                    retval = 12 as libc::c_int
                } else {
                    /* Encode the ASN.1 octet string for "SuppPubInfo" */
                    supp_pub_info_fields.enctype = enctype;
                    supp_pub_info_fields.as_req = *as_req;
                    supp_pub_info_fields.pk_as_rep = *pk_as_rep;
                    retval =
                        encode_krb5_pkinit_supp_pub_info(&mut supp_pub_info_fields,
                                                         &mut supp_pub_info);
                    if !(0 as libc::c_int != retval) {
                        /* Now encode the ASN.1 octet string for "OtherInfo" */
                        memset(&mut alg_id as *mut krb5_algorithm_identifier
                                   as *mut libc::c_void, 0 as libc::c_int,
                               ::std::mem::size_of::<krb5_algorithm_identifier>()
                                   as libc::c_ulong); /*alias*/
                        alg_id.algorithm = *alg_oid;
                        other_info_fields.algorithm_identifier = alg_id;
                        other_info_fields.party_u_info =
                            party_u_info as krb5_principal;
                        other_info_fields.party_v_info =
                            party_v_info as krb5_principal;
                        other_info_fields.supp_pub_info = *supp_pub_info;
                        retval =
                            encode_krb5_sp80056a_other_info(&mut other_info_fields,
                                                            &mut other_info);
                        if !(0 as libc::c_int != retval) {
                            /* 2.  Initialize a 32-bit, big-endian bit string counter as 1.
     * 3.  For i = 1 to reps by 1, do the following:
     *     -   Compute Hashi = H(counter || Z || OtherInfo).
     *     -   Increment counter (modulo 2^32)
     */
                            counter = 1 as libc::c_int as uint32_t;
                            loop  {
                                if !(counter <= reps) {
                                    current_block = 16415152177862271243;
                                    break ;
                                }
                                let mut s: uint = 0 as libc::c_int as uint;
                                let mut be_counter: uint32_t = htonl(counter);
                                ctx = EVP_MD_CTX_new();
                                if ctx.is_null() {
                                    retval =
                                        -(1765328206 as libc::c_long) as
                                            krb5_error_code;
                                    current_block = 10879271936871293117;
                                    break ;
                                } else if EVP_DigestInit(ctx,
                                                         EVP_func.expect("non-null function pointer")())
                                              == 0 {
                                    krb5_set_error_message(context,
                                                           -(1765328206 as
                                                                 libc::c_long)
                                                               as
                                                               krb5_error_code,
                                                           b"Call to OpenSSL EVP_DigestInit() returned an error.\x00"
                                                               as *const u8 as
                                                               *const libc::c_char);
                                    retval =
                                        -(1765328206 as libc::c_long) as
                                            krb5_error_code;
                                    current_block = 10879271936871293117;
                                    break ;
                                } else if EVP_DigestUpdate(ctx,
                                                           &mut be_counter as
                                                               *mut uint32_t
                                                               as
                                                               *const libc::c_void,
                                                           4 as libc::c_int as
                                                               size_t) == 0 ||
                                              EVP_DigestUpdate(ctx,
                                                               (*secret).data
                                                                   as
                                                                   *const libc::c_void,
                                                               (*secret).length
                                                                   as size_t)
                                                  == 0 ||
                                              EVP_DigestUpdate(ctx,
                                                               (*other_info).data
                                                                   as
                                                                   *const libc::c_void,
                                                               (*other_info).length
                                                                   as size_t)
                                                  == 0 {
                                    krb5_set_error_message(context,
                                                           -(1765328206 as
                                                                 libc::c_long)
                                                               as
                                                               krb5_error_code,
                                                           b"Call to OpenSSL EVP_DigestUpdate() returned an error.\x00"
                                                               as *const u8 as
                                                               *const libc::c_char);
                                    retval =
                                        -(1765328206 as libc::c_long) as
                                            krb5_error_code;
                                    current_block = 10879271936871293117;
                                    break ;
                                } else if EVP_DigestFinal(ctx,
                                                          (random_data.data as
                                                               *mut uint8_t).offset(offset
                                                                                        as
                                                                                        isize),
                                                          &mut s) == 0 {
                                    krb5_set_error_message(context,
                                                           -(1765328206 as
                                                                 libc::c_long)
                                                               as
                                                               krb5_error_code,
                                                           b"Call to OpenSSL EVP_DigestUpdate() returned an error.\x00"
                                                               as *const u8 as
                                                               *const libc::c_char);
                                    retval =
                                        -(1765328206 as libc::c_long) as
                                            krb5_error_code;
                                    current_block = 10879271936871293117;
                                    break ;
                                } else {
                                    offset =
                                        (offset as
                                             libc::c_ulong).wrapping_add(s as
                                                                             libc::c_ulong)
                                            as size_t as size_t;
                                    if s as libc::c_ulong == hash_len {
                                    } else {
                                        __assert_fail(b"s == hash_len\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      b"pkinit_crypto_openssl.c\x00"
                                                          as *const u8 as
                                                          *const libc::c_char,
                                                      2482 as libc::c_int as
                                                          libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 180],
                                                                                &[libc::c_char; 180]>(b"krb5_error_code pkinit_alg_agility_kdf(krb5_context, krb5_data *, krb5_data *, krb5_const_principal, krb5_const_principal, krb5_enctype, krb5_data *, krb5_data *, krb5_keyblock *)\x00")).as_ptr());
                                    }
                                    EVP_MD_CTX_free(ctx);
                                    ctx = 0 as *mut EVP_MD_CTX;
                                    counter = counter.wrapping_add(1)
                                }
                            }
                            match current_block {
                                10879271936871293117 => { }
                                _ => {
                                    retval =
                                        krb5_c_random_to_key(context, enctype,
                                                             &mut random_data,
                                                             key_block)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    EVP_MD_CTX_free(ctx);
    /* -   Compute Hashi = H(counter || Z || OtherInfo). */
    /* 4.  Set key = Hash1 || Hash2 || ... so that length of key is K bytes. */
    /* If this has been an error, free the allocated key_block, if any */
    if retval != 0 { krb5_free_keyblock_contents(context, key_block); }
    /* free other allocated resources, either way */
    if !random_data.data.is_null() {
        free(random_data.data as *mut libc::c_void);
    }
    krb5_free_data(context, other_info);
    krb5_free_data(context, supp_pub_info);
    return retval;
}
/*pkinit_alg_agility_kdf() */
/* Call DH_compute_key() and ensure that we left-pad short results instead of
 * leaving junk bytes at the end of the buffer. */
#[c2rust::src_loc = "2510:1"]
unsafe extern "C" fn compute_dh(mut buf: *mut libc::c_uchar,
                                mut size: libc::c_int,
                                mut server_pub_key: *mut BIGNUM,
                                mut dh: *mut DH) {
    let mut len: libc::c_int = 0;
    let mut pad: libc::c_int = 0;
    len = DH_compute_key(buf, server_pub_key, dh);
    if len >= 0 as libc::c_int && len <= size {
    } else {
        __assert_fail(b"len >= 0 && len <= size\x00" as *const u8 as
                          *const libc::c_char,
                      b"pkinit_crypto_openssl.c\x00" as *const u8 as
                          *const libc::c_char,
                      2516 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 54],
                                                &[libc::c_char; 54]>(b"void compute_dh(unsigned char *, int, BIGNUM *, DH *)\x00")).as_ptr());
    }
    if len < size {
        pad = size - len;
        memmove(buf.offset(pad as isize) as *mut libc::c_void,
                buf as *const libc::c_void, len as libc::c_ulong);
        memset(buf as *mut libc::c_void, 0 as libc::c_int,
               pad as libc::c_ulong);
    };
}
#[no_mangle]
#[c2rust::src_loc = "2524:1"]
pub unsafe extern "C" fn client_create_dh(mut context: krb5_context,
                                          mut plg_cryptoctx:
                                              pkinit_plg_crypto_context,
                                          mut cryptoctx:
                                              pkinit_req_crypto_context,
                                          mut id_cryptoctx:
                                              pkinit_identity_crypto_context,
                                          mut dh_size: libc::c_int,
                                          mut dh_params_out:
                                              *mut *mut libc::c_uchar,
                                          mut dh_params_len_out:
                                              *mut libc::c_uint,
                                          mut dh_pubkey_out:
                                              *mut *mut libc::c_uchar,
                                          mut dh_pubkey_len_out:
                                              *mut libc::c_uint)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code =
        -(1765328360 as libc::c_long) as krb5_error_code;
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut dh_err: libc::c_int = 0 as libc::c_int;
    let mut pub_key: *mut ASN1_INTEGER = 0 as *mut ASN1_INTEGER;
    let mut pubkey_bn: *const BIGNUM = 0 as *const BIGNUM;
    let mut p: *const BIGNUM = 0 as *const BIGNUM;
    let mut q: *const BIGNUM = 0 as *const BIGNUM;
    let mut g: *const BIGNUM = 0 as *const BIGNUM;
    let mut dh_params: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut dh_pubkey: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut dh_params_len: libc::c_uint = 0;
    let mut dh_pubkey_len: libc::c_uint = 0;
    *dh_pubkey_out = 0 as *mut libc::c_uchar;
    *dh_params_out = *dh_pubkey_out;
    *dh_pubkey_len_out = 0 as libc::c_int as libc::c_uint;
    *dh_params_len_out = *dh_pubkey_len_out;
    if (*cryptoctx).dh.is_null() {
        if dh_size == 1024 as libc::c_int {
            (*cryptoctx).dh =
                make_oakley_dh(oakley_1024.as_mut_ptr(),
                               ::std::mem::size_of::<[uint8_t; 128]>() as
                                   libc::c_ulong)
        } else if dh_size == 2048 as libc::c_int {
            (*cryptoctx).dh =
                make_oakley_dh(oakley_2048.as_mut_ptr(),
                               ::std::mem::size_of::<[uint8_t; 256]>() as
                                   libc::c_ulong)
        } else if dh_size == 4096 as libc::c_int {
            (*cryptoctx).dh =
                make_oakley_dh(oakley_4096.as_mut_ptr(),
                               ::std::mem::size_of::<[uint8_t; 512]>() as
                                   libc::c_ulong)
        }
        if (*cryptoctx).dh.is_null() {
            current_block = 13903266157893408459;
        } else { current_block = 4166486009154926805; }
    } else { current_block = 4166486009154926805; }
    match current_block {
        4166486009154926805 => {
            DH_generate_key((*cryptoctx).dh);
            DH_get0_key((*cryptoctx).dh, &mut pubkey_bn,
                        0 as *mut *const BIGNUM);
            DH_check((*cryptoctx).dh, &mut dh_err);
            if dh_err != 0 as libc::c_int {
                pkiDebug(b"Warning: dh_check failed with %d\n\x00" as
                             *const u8 as *const libc::c_char, dh_err);
                if dh_err & 0x1 as libc::c_int != 0 {
                    pkiDebug(b"p value is not prime\n\x00" as *const u8 as
                                 *const libc::c_char);
                }
                if dh_err & 0x2 as libc::c_int != 0 {
                    pkiDebug(b"p value is not a safe prime\n\x00" as *const u8
                                 as *const libc::c_char);
                }
                if dh_err & 0x4 as libc::c_int != 0 {
                    pkiDebug(b"unable to check the generator value\n\x00" as
                                 *const u8 as *const libc::c_char);
                }
                if dh_err & 0x8 as libc::c_int != 0 {
                    pkiDebug(b"the g value is not a generator\n\x00" as
                                 *const u8 as *const libc::c_char);
                }
            }
            DH_check_pub_key((*cryptoctx).dh, pubkey_bn, &mut dh_err);
            if dh_err != 0 as libc::c_int {
                pkiDebug(b"dh_check_pub_key failed with %d\n\x00" as *const u8
                             as *const libc::c_char, dh_err);
            } else {
                /* pack DHparams */
    /* aglo: usually we could just call i2d_DHparams to encode DH params
     * however, PKINIT requires RFC3279 encoding and openssl does pkcs#3.
     */
                DH_get0_pqg((*cryptoctx).dh, &mut p, &mut q, &mut g);
                retval =
                    pkinit_encode_dh_params(p, g, q, &mut dh_params,
                                            &mut dh_params_len);
                if !(retval != 0) {
                    /* pack DH public key */
    /* Diffie-Hellman public key must be ASN1 encoded as an INTEGER; this
     * encoding shall be used as the contents (the value) of the
     * subjectPublicKey component (a BIT STRING) of the SubjectPublicKeyInfo
     * data element
     */
                    pub_key =
                        BN_to_ASN1_INTEGER(pubkey_bn, 0 as *mut ASN1_INTEGER);
                    if pub_key.is_null() {
                        retval = 12 as libc::c_int
                    } else {
                        dh_pubkey_len =
                            i2d_ASN1_INTEGER(pub_key,
                                             0 as *mut *mut libc::c_uchar) as
                                libc::c_uint;
                        dh_pubkey =
                            malloc(dh_pubkey_len as libc::c_ulong) as
                                *mut libc::c_uchar;
                        buf = dh_pubkey;
                        if dh_pubkey.is_null() {
                            retval = 12 as libc::c_int
                        } else {
                            i2d_ASN1_INTEGER(pub_key, &mut buf);
                            *dh_params_out = dh_params;
                            *dh_params_len_out = dh_params_len;
                            *dh_pubkey_out = dh_pubkey;
                            *dh_pubkey_len_out = dh_pubkey_len;
                            dh_pubkey = 0 as *mut libc::c_uchar;
                            dh_params = dh_pubkey;
                            retval = 0 as libc::c_int
                        }
                    }
                }
            }
        }
        _ => { }
    }
    if retval != 0 {
        DH_free((*cryptoctx).dh);
        (*cryptoctx).dh = 0 as *mut DH
    }
    free(dh_params as *mut libc::c_void);
    free(dh_pubkey as *mut libc::c_void);
    ASN1_INTEGER_free(pub_key);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "2630:1"]
pub unsafe extern "C" fn client_process_dh(mut context: krb5_context,
                                           mut plg_cryptoctx:
                                               pkinit_plg_crypto_context,
                                           mut cryptoctx:
                                               pkinit_req_crypto_context,
                                           mut id_cryptoctx:
                                               pkinit_identity_crypto_context,
                                           mut subjectPublicKey_data:
                                               *mut libc::c_uchar,
                                           mut subjectPublicKey_length:
                                               libc::c_uint,
                                           mut client_key_out:
                                               *mut *mut libc::c_uchar,
                                           mut client_key_len_out:
                                               *mut libc::c_uint)
 -> krb5_error_code {
    let mut retval: krb5_error_code =
        -(1765328360 as libc::c_long) as krb5_error_code;
    let mut server_pub_key: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut pub_key: *mut ASN1_INTEGER = 0 as *mut ASN1_INTEGER;
    let mut client_key: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut client_key_len: libc::c_uint = 0;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    *client_key_out = 0 as *mut libc::c_uchar;
    *client_key_len_out = 0 as libc::c_int as libc::c_uint;
    client_key_len = DH_size((*cryptoctx).dh) as libc::c_uint;
    client_key =
        malloc(client_key_len as libc::c_ulong) as *mut libc::c_uchar;
    if client_key.is_null() {
        retval = 12 as libc::c_int
    } else {
        p = subjectPublicKey_data;
        pub_key =
            d2i_ASN1_INTEGER(0 as *mut *mut ASN1_INTEGER, &mut p,
                             subjectPublicKey_length as libc::c_long);
        if !pub_key.is_null() {
            server_pub_key = ASN1_INTEGER_to_BN(pub_key, 0 as *mut BIGNUM);
            if !server_pub_key.is_null() {
                compute_dh(client_key, client_key_len as libc::c_int,
                           server_pub_key, (*cryptoctx).dh);
                *client_key_out = client_key;
                *client_key_len_out = client_key_len;
                client_key = 0 as *mut libc::c_uchar;
                retval = 0 as libc::c_int
            }
        }
    }
    BN_free(server_pub_key);
    ASN1_INTEGER_free(pub_key);
    free(client_key as *mut libc::c_void);
    return retval;
}
/* Return 1 if dh is a permitted well-known group, otherwise return 0. */
#[c2rust::src_loc = "2684:1"]
unsafe extern "C" fn check_dh_wellknown(mut cryptoctx:
                                            pkinit_plg_crypto_context,
                                        mut dh: *mut DH,
                                        mut nbits: libc::c_int)
 -> libc::c_int {
    match nbits {
        1024 => {
            /* Oakley MODP group 2 */
            if pkinit_check_dh_params((*cryptoctx).dh_1024, dh) ==
                   0 as libc::c_int {
                return 1 as libc::c_int
            }
        }
        2048 => {
            /* Oakley MODP group 14 */
            if pkinit_check_dh_params((*cryptoctx).dh_2048, dh) ==
                   0 as libc::c_int {
                return 1 as libc::c_int
            }
        }
        4096 => {
            /* Oakley MODP group 16 */
            if pkinit_check_dh_params((*cryptoctx).dh_4096, dh) ==
                   0 as libc::c_int {
                return 1 as libc::c_int
            }
        }
        _ => { }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "2713:1"]
pub unsafe extern "C" fn server_check_dh(mut context: krb5_context,
                                         mut cryptoctx:
                                             pkinit_plg_crypto_context,
                                         mut req_cryptoctx:
                                             pkinit_req_crypto_context,
                                         mut id_cryptoctx:
                                             pkinit_identity_crypto_context,
                                         mut dh_params: *mut krb5_data,
                                         mut minbits: libc::c_int)
 -> krb5_error_code {
    let mut dh: *mut DH = 0 as *mut DH;
    let mut p: *const BIGNUM = 0 as *const BIGNUM;
    let mut dh_prime_bits: libc::c_int = 0;
    let mut retval: krb5_error_code =
        -(1765328319 as libc::c_long) as krb5_error_code;
    dh =
        decode_dh_params((*dh_params).data as *mut uint8_t,
                         (*dh_params).length);
    if dh.is_null() {
        pkiDebug(b"failed to decode dhparams\n\x00" as *const u8 as
                     *const libc::c_char);
    } else {
        /* KDC SHOULD check to see if the key parameters satisfy its policy */
        DH_get0_pqg(dh, &mut p, 0 as *mut *const BIGNUM,
                    0 as *mut *const BIGNUM);
        dh_prime_bits = BN_num_bits(p);
        if minbits != 0 && dh_prime_bits < minbits {
            pkiDebug(b"client sent dh params with %d bits, we require %d\n\x00"
                         as *const u8 as *const libc::c_char, dh_prime_bits,
                     minbits);
        } else if check_dh_wellknown(cryptoctx, dh, dh_prime_bits) != 0 {
            retval = 0 as libc::c_int
        }
    }
    if retval == 0 as libc::c_int {
        (*req_cryptoctx).dh = dh
    } else { DH_free(dh); }
    return retval;
}
/* Duplicate a DH handle (parameters only, not public or private key). */
#[c2rust::src_loc = "2754:1"]
unsafe extern "C" fn dup_dh_params(mut src: *const DH) -> *mut DH {
    let mut oldp: *const BIGNUM = 0 as *const BIGNUM;
    let mut oldq: *const BIGNUM = 0 as *const BIGNUM;
    let mut oldg: *const BIGNUM = 0 as *const BIGNUM;
    let mut p: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut q: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut g: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut dh: *mut DH = 0 as *mut DH;
    DH_get0_pqg(src, &mut oldp, &mut oldq, &mut oldg);
    p = BN_dup(oldp);
    q = BN_dup(oldq);
    g = BN_dup(oldg);
    dh = DH_new();
    if p.is_null() || q.is_null() || g.is_null() || dh.is_null() {
        BN_free(p);
        BN_free(q);
        BN_free(g);
        DH_free(dh);
        return 0 as *mut DH
    }
    DH_set0_pqg(dh, p, q, g);
    return dh;
}
/* kdc's dh function */
#[no_mangle]
#[c2rust::src_loc = "2778:1"]
pub unsafe extern "C" fn server_process_dh(mut context: krb5_context,
                                           mut plg_cryptoctx:
                                               pkinit_plg_crypto_context,
                                           mut cryptoctx:
                                               pkinit_req_crypto_context,
                                           mut id_cryptoctx:
                                               pkinit_identity_crypto_context,
                                           mut data: *mut libc::c_uchar,
                                           mut data_len: libc::c_uint,
                                           mut dh_pubkey_out:
                                               *mut *mut libc::c_uchar,
                                           mut dh_pubkey_len_out:
                                               *mut libc::c_uint,
                                           mut server_key_out:
                                               *mut *mut libc::c_uchar,
                                           mut server_key_len_out:
                                               *mut libc::c_uint)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut dh: *mut DH = 0 as *mut DH;
    let mut dh_server: *mut DH = 0 as *mut DH;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut pub_key: *mut ASN1_INTEGER = 0 as *mut ASN1_INTEGER;
    let mut client_pubkey: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut server_pubkey: *const BIGNUM = 0 as *const BIGNUM;
    let mut dh_pubkey: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut server_key: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut dh_pubkey_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut server_key_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    *server_key_out = 0 as *mut libc::c_uchar;
    *dh_pubkey_out = *server_key_out;
    *server_key_len_out = 0 as libc::c_int as libc::c_uint;
    *dh_pubkey_len_out = *server_key_len_out;
    /* get client's received DH parameters that we saved in server_check_dh */
    dh = (*cryptoctx).dh;
    dh_server = dup_dh_params(dh);
    if !dh_server.is_null() {
        /* decode client's public key */
        p = data;
        pub_key =
            d2i_ASN1_INTEGER(0 as *mut *mut ASN1_INTEGER,
                             &mut p as *mut *mut libc::c_uchar as
                                 *mut *const libc::c_uchar,
                             data_len as libc::c_int as libc::c_long);
        if !pub_key.is_null() {
            client_pubkey = ASN1_INTEGER_to_BN(pub_key, 0 as *mut BIGNUM);
            if !client_pubkey.is_null() {
                ASN1_INTEGER_free(pub_key);
                if !(DH_generate_key(dh_server) == 0) {
                    DH_get0_key(dh_server, &mut server_pubkey,
                                0 as *mut *const BIGNUM);
                    /* generate DH session key */
                    server_key_len = DH_size(dh_server) as libc::c_uint;
                    server_key =
                        malloc(server_key_len as libc::c_ulong) as
                            *mut libc::c_uchar;
                    if !server_key.is_null() {
                        compute_dh(server_key, server_key_len as libc::c_int,
                                   client_pubkey, dh_server);
                        /* KDC reply */
    /* pack DH public key */
    /* Diffie-Hellman public key must be ASN1 encoded as an INTEGER; this
     * encoding shall be used as the contents (the value) of the
     * subjectPublicKey component (a BIT STRING) of the SubjectPublicKeyInfo
     * data element
     */
                        pub_key =
                            BN_to_ASN1_INTEGER(server_pubkey,
                                               0 as *mut ASN1_INTEGER);
                        if !pub_key.is_null() {
                            dh_pubkey_len =
                                i2d_ASN1_INTEGER(pub_key,
                                                 0 as *mut *mut libc::c_uchar)
                                    as libc::c_uint;
                            dh_pubkey =
                                malloc(dh_pubkey_len as libc::c_ulong) as
                                    *mut libc::c_uchar;
                            p = dh_pubkey;
                            if !dh_pubkey.is_null() {
                                i2d_ASN1_INTEGER(pub_key, &mut p);
                                if !pub_key.is_null() {
                                    ASN1_INTEGER_free(pub_key);
                                }
                                *dh_pubkey_out = dh_pubkey;
                                *dh_pubkey_len_out = dh_pubkey_len;
                                *server_key_out = server_key;
                                *server_key_len_out = server_key_len;
                                server_key = 0 as *mut libc::c_uchar;
                                dh_pubkey = server_key;
                                retval = 0 as libc::c_int
                            }
                        }
                    }
                }
            }
        }
    }
    BN_free(client_pubkey);
    DH_free(dh_server);
    free(dh_pubkey as *mut libc::c_void);
    free(server_key as *mut libc::c_void);
    return retval;
}
#[c2rust::src_loc = "2872:1"]
unsafe extern "C" fn pkinit_openssl_init() -> libc::c_int {
    /* Initialize OpenSSL. */
    OPENSSL_init_crypto(0x2 as libc::c_long as uint64_t,
                        0 as *const OPENSSL_INIT_SETTINGS);
    OPENSSL_init_crypto((0x4 as libc::c_long | 0x8 as libc::c_long) as
                            uint64_t, 0 as *const OPENSSL_INIT_SETTINGS);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "2881:1"]
unsafe extern "C" fn pkinit_encode_dh_params(mut p: *const BIGNUM,
                                             mut g: *const BIGNUM,
                                             mut q: *const BIGNUM,
                                             mut buf: *mut *mut uint8_t,
                                             mut buf_len: *mut libc::c_uint)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut bufsize: libc::c_int = 0 as libc::c_int;
    let mut r: libc::c_int = 0 as libc::c_int;
    let mut tmp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ap: *mut ASN1_INTEGER = 0 as *mut ASN1_INTEGER;
    let mut ag: *mut ASN1_INTEGER = 0 as *mut ASN1_INTEGER;
    let mut aq: *mut ASN1_INTEGER = 0 as *mut ASN1_INTEGER;
    ap = BN_to_ASN1_INTEGER(p, 0 as *mut ASN1_INTEGER);
    if !ap.is_null() {
        ag = BN_to_ASN1_INTEGER(g, 0 as *mut ASN1_INTEGER);
        if !ag.is_null() {
            aq = BN_to_ASN1_INTEGER(q, 0 as *mut ASN1_INTEGER);
            if !aq.is_null() {
                bufsize = i2d_ASN1_INTEGER(ap, 0 as *mut *mut libc::c_uchar);
                bufsize += i2d_ASN1_INTEGER(ag, 0 as *mut *mut libc::c_uchar);
                bufsize += i2d_ASN1_INTEGER(aq, 0 as *mut *mut libc::c_uchar);
                r =
                    ASN1_object_size(1 as libc::c_int, bufsize,
                                     16 as libc::c_int);
                *buf = malloc(r as size_t) as *mut uint8_t;
                tmp = *buf;
                if !tmp.is_null() {
                    ASN1_put_object(&mut tmp, 1 as libc::c_int, bufsize,
                                    16 as libc::c_int, 0 as libc::c_int);
                    i2d_ASN1_INTEGER(ap, &mut tmp);
                    i2d_ASN1_INTEGER(ag, &mut tmp);
                    i2d_ASN1_INTEGER(aq, &mut tmp);
                    *buf_len = r as libc::c_uint;
                    retval = 0 as libc::c_int
                }
            }
        }
    }
    if !ap.is_null() { ASN1_INTEGER_free(ap); }
    if !ag.is_null() { ASN1_INTEGER_free(ag); }
    if !aq.is_null() { ASN1_INTEGER_free(aq); }
    return retval;
}
#[c2rust::src_loc = "2950:1"]
static mut DHvparams_seq_tt: [ASN1_TEMPLATE; 2] =
    unsafe {
        [{
             let mut init =
                 ASN1_TEMPLATE_st{flags: 0 as libc::c_int as libc::c_ulong,
                                  tag: 0 as libc::c_int as libc::c_long,
                                  offset: 0 as libc::c_ulong,
                                  field_name:
                                      b"seed\x00" as *const u8 as
                                          *const libc::c_char,
                                  item:
                                      &ASN1_BIT_STRING_it as
                                          *const ASN1_ITEM,};
             init
         },
         {
             let mut init =
                 ASN1_TEMPLATE_st{flags: 0 as libc::c_int as libc::c_ulong,
                                  tag: 0 as libc::c_int as libc::c_long,
                                  offset: 8 as libc::c_ulong,
                                  field_name:
                                      b"counter\x00" as *const u8 as
                                          *const libc::c_char,
                                  item: &BIGNUM_it as *const ASN1_ITEM,};
             init
         }]
    };
// Initialized in run_static_initializers
#[c2rust::src_loc = "2953:3"]
static mut DHvparams_it: ASN1_ITEM =
    ASN1_ITEM{itype: 0,
              utype: 0,
              templates: 0 as *const ASN1_TEMPLATE,
              tcount: 0,
              funcs: 0 as *const libc::c_void,
              size: 0,
              sname: 0 as *const libc::c_char,};
#[c2rust::src_loc = "2955:1"]
static mut DHxparams_seq_tt: [ASN1_TEMPLATE; 5] =
    unsafe {
        [{
             let mut init =
                 ASN1_TEMPLATE_st{flags: 0 as libc::c_int as libc::c_ulong,
                                  tag: 0 as libc::c_int as libc::c_long,
                                  offset: 0 as libc::c_ulong,
                                  field_name:
                                      b"p\x00" as *const u8 as
                                          *const libc::c_char,
                                  item: &BIGNUM_it as *const ASN1_ITEM,};
             init
         },
         {
             let mut init =
                 ASN1_TEMPLATE_st{flags: 0 as libc::c_int as libc::c_ulong,
                                  tag: 0 as libc::c_int as libc::c_long,
                                  offset: 16 as libc::c_ulong,
                                  field_name:
                                      b"g\x00" as *const u8 as
                                          *const libc::c_char,
                                  item: &BIGNUM_it as *const ASN1_ITEM,};
             init
         },
         {
             let mut init =
                 ASN1_TEMPLATE_st{flags: 0x1 as libc::c_int as libc::c_ulong,
                                  tag: 0 as libc::c_int as libc::c_long,
                                  offset: 8 as libc::c_ulong,
                                  field_name:
                                      b"q\x00" as *const u8 as
                                          *const libc::c_char,
                                  item: &BIGNUM_it as *const ASN1_ITEM,};
             init
         },
         {
             let mut init =
                 ASN1_TEMPLATE_st{flags: 0x1 as libc::c_int as libc::c_ulong,
                                  tag: 0 as libc::c_int as libc::c_long,
                                  offset: 24 as libc::c_ulong,
                                  field_name:
                                      b"j\x00" as *const u8 as
                                          *const libc::c_char,
                                  item: &BIGNUM_it as *const ASN1_ITEM,};
             init
         },
         {
             let mut init =
                 ASN1_TEMPLATE_st{flags: 0x1 as libc::c_int as libc::c_ulong,
                                  tag: 0 as libc::c_int as libc::c_long,
                                  offset: 32 as libc::c_ulong,
                                  field_name:
                                      b"vparams\x00" as *const u8 as
                                          *const libc::c_char,
                                  item: &DHvparams_it as *const ASN1_ITEM,};
             init
         }]
    };
// Initialized in run_static_initializers
#[c2rust::src_loc = "2961:3"]
static mut DHxparams_it: ASN1_ITEM =
    ASN1_ITEM{itype: 0,
              utype: 0,
              templates: 0 as *const ASN1_TEMPLATE,
              tcount: 0,
              funcs: 0 as *const libc::c_void,
              size: 0,
              sname: 0 as *const libc::c_char,};
#[c2rust::src_loc = "2963:1"]
unsafe extern "C" fn decode_dh_params(mut p: *const uint8_t,
                                      mut len: libc::c_uint) -> *mut DH {
    let mut params: *mut int_dhx942_dh = 0 as *mut int_dhx942_dh;
    let mut dh: *mut DH = 0 as *mut DH;
    dh = DH_new();
    if dh.is_null() { return 0 as *mut DH }
    params =
        ASN1_item_d2i(0 as *mut *mut ASN1_VALUE, &mut p, len as libc::c_long,
                      &DHxparams_it) as *mut int_dhx942_dh;
    if params.is_null() { DH_free(dh); return 0 as *mut DH }
    /* Steal the p, q, and g values from dhparams for dh.  Ignore j and
     * vparams. */
    DH_set0_pqg(dh, (*params).p, (*params).q, (*params).g);
    (*params).g = 0 as *mut BIGNUM;
    (*params).q = (*params).g;
    (*params).p = (*params).q;
    ASN1_item_free(params as *mut ASN1_VALUE, &DHxparams_it);
    return dh;
}
/* OPENSSL_VERSION_NUMBER < 0x10100000L */
/* OPENSSL_VERSION_NUMBER < 0x10100000L */
#[c2rust::src_loc = "3066:1"]
unsafe extern "C" fn pkinit_create_sequence_of_principal_identifiers(mut context:
                                                                         krb5_context,
                                                                     mut plg_cryptoctx:
                                                                         pkinit_plg_crypto_context,
                                                                     mut req_cryptoctx:
                                                                         pkinit_req_crypto_context,
                                                                     mut id_cryptoctx:
                                                                         pkinit_identity_crypto_context,
                                                                     mut type_0:
                                                                         libc::c_int,
                                                                     mut e_data_out:
                                                                         *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code =
        -(1765328324 as libc::c_long) as krb5_error_code;
    let mut krb5_trusted_certifiers:
            *mut *mut krb5_external_principal_identifier =
        0 as *mut *mut krb5_external_principal_identifier;
    let mut td_certifiers: *mut krb5_data = 0 as *mut krb5_data;
    let mut pa_data: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    match type_0 {
        104 => {
            retval =
                create_krb5_trustedCertifiers(context, plg_cryptoctx,
                                              req_cryptoctx, id_cryptoctx,
                                              &mut krb5_trusted_certifiers);
            if retval != 0 {
                pkiDebug(b"create_krb5_trustedCertifiers failed\n\x00" as
                             *const u8 as *const libc::c_char);
                current_block = 11477982493795055420;
            } else { current_block = 10599921512955367680; }
        }
        105 => {
            retval =
                create_krb5_invalidCertificates(context, plg_cryptoctx,
                                                req_cryptoctx, id_cryptoctx,
                                                &mut krb5_trusted_certifiers);
            if retval != 0 {
                pkiDebug(b"create_krb5_invalidCertificates failed\n\x00" as
                             *const u8 as *const libc::c_char);
                current_block = 11477982493795055420;
            } else { current_block = 10599921512955367680; }
        }
        _ => {
            retval = -(1 as libc::c_int);
            current_block = 11477982493795055420;
        }
    }
    match current_block {
        10599921512955367680 => {
            retval =
                k5int_encode_krb5_td_trusted_certifiers.expect("non-null function pointer")(krb5_trusted_certifiers
                                                                                                as
                                                                                                *const *mut krb5_external_principal_identifier,
                                                                                            &mut td_certifiers);
            if retval != 0 {
                pkiDebug(b"encode_krb5_td_trusted_certifiers failed\n\x00" as
                             *const u8 as *const libc::c_char);
            } else {
                pa_data =
                    malloc((2 as libc::c_int as
                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_pa_data>()
                                                                as
                                                                libc::c_ulong))
                        as *mut *mut krb5_pa_data;
                if pa_data.is_null() {
                    retval = 12 as libc::c_int
                } else {
                    let ref mut fresh2 =
                        *pa_data.offset(1 as libc::c_int as isize);
                    *fresh2 = 0 as *mut krb5_pa_data;
                    let ref mut fresh3 =
                        *pa_data.offset(0 as libc::c_int as isize);
                    *fresh3 =
                        malloc(::std::mem::size_of::<krb5_pa_data>() as
                                   libc::c_ulong) as *mut krb5_pa_data;
                    if (*pa_data.offset(0 as libc::c_int as isize)).is_null()
                       {
                        free(pa_data as *mut libc::c_void);
                        retval = 12 as libc::c_int
                    } else {
                        (**pa_data.offset(0 as libc::c_int as isize)).pa_type
                            = type_0;
                        (**pa_data.offset(0 as libc::c_int as isize)).length =
                            (*td_certifiers).length;
                        let ref mut fresh4 =
                            (**pa_data.offset(0 as libc::c_int as
                                                  isize)).contents;
                        *fresh4 = (*td_certifiers).data as *mut krb5_octet;
                        *e_data_out = pa_data;
                        retval = 0 as libc::c_int
                    }
                }
            }
        }
        _ => { }
    }
    if !krb5_trusted_certifiers.is_null() {
        free_krb5_external_principal_identifier(&mut krb5_trusted_certifiers);
    }
    free(td_certifiers as *mut libc::c_void);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "3136:1"]
pub unsafe extern "C" fn pkinit_create_td_trusted_certifiers(mut context:
                                                                 krb5_context,
                                                             mut plg_cryptoctx:
                                                                 pkinit_plg_crypto_context,
                                                             mut req_cryptoctx:
                                                                 pkinit_req_crypto_context,
                                                             mut id_cryptoctx:
                                                                 pkinit_identity_crypto_context,
                                                             mut e_data_out:
                                                                 *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut retval: krb5_error_code =
        -(1765328324 as libc::c_long) as krb5_error_code;
    retval =
        pkinit_create_sequence_of_principal_identifiers(context,
                                                        plg_cryptoctx,
                                                        req_cryptoctx,
                                                        id_cryptoctx,
                                                        104 as libc::c_int,
                                                        e_data_out);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "3152:1"]
pub unsafe extern "C" fn pkinit_create_td_invalid_certificate(mut context:
                                                                  krb5_context,
                                                              mut plg_cryptoctx:
                                                                  pkinit_plg_crypto_context,
                                                              mut req_cryptoctx:
                                                                  pkinit_req_crypto_context,
                                                              mut id_cryptoctx:
                                                                  pkinit_identity_crypto_context,
                                                              mut e_data_out:
                                                                  *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut retval: krb5_error_code =
        -(1765328324 as libc::c_long) as krb5_error_code;
    retval =
        pkinit_create_sequence_of_principal_identifiers(context,
                                                        plg_cryptoctx,
                                                        req_cryptoctx,
                                                        id_cryptoctx,
                                                        105 as libc::c_int,
                                                        e_data_out);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "3169:1"]
pub unsafe extern "C" fn pkinit_create_td_dh_parameters(mut context:
                                                            krb5_context,
                                                        mut plg_cryptoctx:
                                                            pkinit_plg_crypto_context,
                                                        mut req_cryptoctx:
                                                            pkinit_req_crypto_context,
                                                        mut id_cryptoctx:
                                                            pkinit_identity_crypto_context,
                                                        mut opts:
                                                            *mut pkinit_plg_opts,
                                                        mut e_data_out:
                                                            *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut buf1_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut buf2_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut buf3_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut buf1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut buf2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut buf3: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut pa_data: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    let mut encoded_algId: *mut krb5_data = 0 as *mut krb5_data;
    let mut algId: *mut *mut krb5_algorithm_identifier =
        0 as *mut *mut krb5_algorithm_identifier;
    let mut p: *const BIGNUM = 0 as *const BIGNUM;
    let mut q: *const BIGNUM = 0 as *const BIGNUM;
    let mut g: *const BIGNUM = 0 as *const BIGNUM;
    if !((*opts).dh_min_bits > 4096 as libc::c_int) {
        if (*opts).dh_min_bits <= 1024 as libc::c_int {
            DH_get0_pqg((*plg_cryptoctx).dh_1024, &mut p, &mut q, &mut g);
            retval =
                pkinit_encode_dh_params(p, g, q, &mut buf1, &mut buf1_len);
            if retval != 0 {
                current_block = 12522200916456487735;
            } else { current_block = 1394248824506584008; }
        } else { current_block = 1394248824506584008; }
        match current_block {
            12522200916456487735 => { }
            _ => {
                if (*opts).dh_min_bits <= 2048 as libc::c_int {
                    DH_get0_pqg((*plg_cryptoctx).dh_2048, &mut p, &mut q,
                                &mut g);
                    retval =
                        pkinit_encode_dh_params(p, g, q, &mut buf2,
                                                &mut buf2_len);
                    if retval != 0 {
                        current_block = 12522200916456487735;
                    } else { current_block = 3512920355445576850; }
                } else { current_block = 3512920355445576850; }
                match current_block {
                    12522200916456487735 => { }
                    _ => {
                        DH_get0_pqg((*plg_cryptoctx).dh_4096, &mut p, &mut q,
                                    &mut g);
                        retval =
                            pkinit_encode_dh_params(p, g, q, &mut buf3,
                                                    &mut buf3_len);
                        if !(retval != 0) {
                            if (*opts).dh_min_bits <= 1024 as libc::c_int {
                                algId =
                                    malloc((4 as libc::c_int as
                                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_algorithm_identifier>()
                                                                                as
                                                                                libc::c_ulong))
                                        as
                                        *mut *mut krb5_algorithm_identifier;
                                if algId.is_null() {
                                    current_block = 12522200916456487735;
                                } else {
                                    let ref mut fresh5 =
                                        *algId.offset(3 as libc::c_int as
                                                          isize);
                                    *fresh5 =
                                        0 as *mut krb5_algorithm_identifier;
                                    let ref mut fresh6 =
                                        *algId.offset(0 as libc::c_int as
                                                          isize);
                                    *fresh6 =
                                        malloc(::std::mem::size_of::<krb5_algorithm_identifier>()
                                                   as libc::c_ulong) as
                                            *mut krb5_algorithm_identifier;
                                    if (*algId.offset(0 as libc::c_int as
                                                          isize)).is_null() {
                                        current_block = 12522200916456487735;
                                    } else {
                                        let ref mut fresh7 =
                                            (**algId.offset(0 as libc::c_int
                                                                as
                                                                isize)).parameters.data;
                                        *fresh7 =
                                            malloc(buf2_len as libc::c_ulong)
                                                as *mut libc::c_char;
                                        if (**algId.offset(0 as libc::c_int as
                                                               isize)).parameters.data.is_null()
                                           {
                                            current_block =
                                                12522200916456487735;
                                        } else {
                                            memcpy((**algId.offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)).parameters.data
                                                       as *mut libc::c_void,
                                                   buf2 as
                                                       *const libc::c_void,
                                                   buf2_len as libc::c_ulong);
                                            (**algId.offset(0 as libc::c_int
                                                                as
                                                                isize)).parameters.length
                                                = buf2_len;
                                            (**algId.offset(0 as libc::c_int
                                                                as
                                                                isize)).algorithm
                                                = dh_oid;
                                            let ref mut fresh8 =
                                                *algId.offset(1 as libc::c_int
                                                                  as isize);
                                            *fresh8 =
                                                malloc(::std::mem::size_of::<krb5_algorithm_identifier>()
                                                           as libc::c_ulong)
                                                    as
                                                    *mut krb5_algorithm_identifier;
                                            if (*algId.offset(1 as libc::c_int
                                                                  as
                                                                  isize)).is_null()
                                               {
                                                current_block =
                                                    12522200916456487735;
                                            } else {
                                                let ref mut fresh9 =
                                                    (**algId.offset(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)).parameters.data;
                                                *fresh9 =
                                                    malloc(buf3_len as
                                                               libc::c_ulong)
                                                        as *mut libc::c_char;
                                                if (**algId.offset(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)).parameters.data.is_null()
                                                   {
                                                    current_block =
                                                        12522200916456487735;
                                                } else {
                                                    memcpy((**algId.offset(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)).parameters.data
                                                               as
                                                               *mut libc::c_void,
                                                           buf3 as
                                                               *const libc::c_void,
                                                           buf3_len as
                                                               libc::c_ulong);
                                                    (**algId.offset(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)).parameters.length
                                                        = buf3_len;
                                                    (**algId.offset(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)).algorithm
                                                        = dh_oid;
                                                    let ref mut fresh10 =
                                                        *algId.offset(2 as
                                                                          libc::c_int
                                                                          as
                                                                          isize);
                                                    *fresh10 =
                                                        malloc(::std::mem::size_of::<krb5_algorithm_identifier>()
                                                                   as
                                                                   libc::c_ulong)
                                                            as
                                                            *mut krb5_algorithm_identifier;
                                                    if (*algId.offset(2 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)).is_null()
                                                       {
                                                        current_block =
                                                            12522200916456487735;
                                                    } else {
                                                        let ref mut fresh11 =
                                                            (**algId.offset(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)).parameters.data;
                                                        *fresh11 =
                                                            malloc(buf1_len as
                                                                       libc::c_ulong)
                                                                as
                                                                *mut libc::c_char;
                                                        if (**algId.offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)).parameters.data.is_null()
                                                           {
                                                            current_block =
                                                                12522200916456487735;
                                                        } else {
                                                            memcpy((**algId.offset(2
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       isize)).parameters.data
                                                                       as
                                                                       *mut libc::c_void,
                                                                   buf1 as
                                                                       *const libc::c_void,
                                                                   buf1_len as
                                                                       libc::c_ulong);
                                                            (**algId.offset(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)).parameters.length
                                                                = buf1_len;
                                                            (**algId.offset(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)).algorithm
                                                                = dh_oid;
                                                            current_block =
                                                                12264624100856317061;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else if (*opts).dh_min_bits <=
                                          2048 as libc::c_int {
                                algId =
                                    malloc((3 as libc::c_int as
                                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_algorithm_identifier>()
                                                                                as
                                                                                libc::c_ulong))
                                        as
                                        *mut *mut krb5_algorithm_identifier;
                                if algId.is_null() {
                                    current_block = 12522200916456487735;
                                } else {
                                    let ref mut fresh12 =
                                        *algId.offset(2 as libc::c_int as
                                                          isize);
                                    *fresh12 =
                                        0 as *mut krb5_algorithm_identifier;
                                    let ref mut fresh13 =
                                        *algId.offset(0 as libc::c_int as
                                                          isize);
                                    *fresh13 =
                                        malloc(::std::mem::size_of::<krb5_algorithm_identifier>()
                                                   as libc::c_ulong) as
                                            *mut krb5_algorithm_identifier;
                                    if (*algId.offset(0 as libc::c_int as
                                                          isize)).is_null() {
                                        current_block = 12522200916456487735;
                                    } else {
                                        let ref mut fresh14 =
                                            (**algId.offset(0 as libc::c_int
                                                                as
                                                                isize)).parameters.data;
                                        *fresh14 =
                                            malloc(buf2_len as libc::c_ulong)
                                                as *mut libc::c_char;
                                        if (**algId.offset(0 as libc::c_int as
                                                               isize)).parameters.data.is_null()
                                           {
                                            current_block =
                                                12522200916456487735;
                                        } else {
                                            memcpy((**algId.offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)).parameters.data
                                                       as *mut libc::c_void,
                                                   buf2 as
                                                       *const libc::c_void,
                                                   buf2_len as libc::c_ulong);
                                            (**algId.offset(0 as libc::c_int
                                                                as
                                                                isize)).parameters.length
                                                = buf2_len;
                                            (**algId.offset(0 as libc::c_int
                                                                as
                                                                isize)).algorithm
                                                = dh_oid;
                                            let ref mut fresh15 =
                                                *algId.offset(1 as libc::c_int
                                                                  as isize);
                                            *fresh15 =
                                                malloc(::std::mem::size_of::<krb5_algorithm_identifier>()
                                                           as libc::c_ulong)
                                                    as
                                                    *mut krb5_algorithm_identifier;
                                            if (*algId.offset(1 as libc::c_int
                                                                  as
                                                                  isize)).is_null()
                                               {
                                                current_block =
                                                    12522200916456487735;
                                            } else {
                                                let ref mut fresh16 =
                                                    (**algId.offset(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)).parameters.data;
                                                *fresh16 =
                                                    malloc(buf3_len as
                                                               libc::c_ulong)
                                                        as *mut libc::c_char;
                                                if (**algId.offset(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)).parameters.data.is_null()
                                                   {
                                                    current_block =
                                                        12522200916456487735;
                                                } else {
                                                    memcpy((**algId.offset(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)).parameters.data
                                                               as
                                                               *mut libc::c_void,
                                                           buf3 as
                                                               *const libc::c_void,
                                                           buf3_len as
                                                               libc::c_ulong);
                                                    (**algId.offset(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)).parameters.length
                                                        = buf3_len;
                                                    (**algId.offset(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)).algorithm
                                                        = dh_oid;
                                                    current_block =
                                                        12264624100856317061;
                                                }
                                            }
                                        }
                                    }
                                }
                            } else if (*opts).dh_min_bits <=
                                          4096 as libc::c_int {
                                algId =
                                    malloc((2 as libc::c_int as
                                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_algorithm_identifier>()
                                                                                as
                                                                                libc::c_ulong))
                                        as
                                        *mut *mut krb5_algorithm_identifier;
                                if algId.is_null() {
                                    current_block = 12522200916456487735;
                                } else {
                                    let ref mut fresh17 =
                                        *algId.offset(1 as libc::c_int as
                                                          isize);
                                    *fresh17 =
                                        0 as *mut krb5_algorithm_identifier;
                                    let ref mut fresh18 =
                                        *algId.offset(0 as libc::c_int as
                                                          isize);
                                    *fresh18 =
                                        malloc(::std::mem::size_of::<krb5_algorithm_identifier>()
                                                   as libc::c_ulong) as
                                            *mut krb5_algorithm_identifier;
                                    if (*algId.offset(0 as libc::c_int as
                                                          isize)).is_null() {
                                        current_block = 12522200916456487735;
                                    } else {
                                        let ref mut fresh19 =
                                            (**algId.offset(0 as libc::c_int
                                                                as
                                                                isize)).parameters.data;
                                        *fresh19 =
                                            malloc(buf3_len as libc::c_ulong)
                                                as *mut libc::c_char;
                                        if (**algId.offset(0 as libc::c_int as
                                                               isize)).parameters.data.is_null()
                                           {
                                            current_block =
                                                12522200916456487735;
                                        } else {
                                            memcpy((**algId.offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)).parameters.data
                                                       as *mut libc::c_void,
                                                   buf3 as
                                                       *const libc::c_void,
                                                   buf3_len as libc::c_ulong);
                                            (**algId.offset(0 as libc::c_int
                                                                as
                                                                isize)).parameters.length
                                                = buf3_len;
                                            (**algId.offset(0 as libc::c_int
                                                                as
                                                                isize)).algorithm
                                                = dh_oid;
                                            current_block =
                                                12264624100856317061;
                                        }
                                    }
                                }
                            } else { current_block = 12264624100856317061; }
                            match current_block {
                                12522200916456487735 => { }
                                _ => {
                                    retval =
                                        k5int_encode_krb5_td_dh_parameters.expect("non-null function pointer")(algId
                                                                                                                   as
                                                                                                                   *const *mut krb5_algorithm_identifier,
                                                                                                               &mut encoded_algId);
                                    if !(retval != 0) {
                                        pa_data =
                                            malloc((2 as libc::c_int as
                                                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_pa_data>()
                                                                                        as
                                                                                        libc::c_ulong))
                                                as *mut *mut krb5_pa_data;
                                        if pa_data.is_null() {
                                            retval = 12 as libc::c_int
                                        } else {
                                            let ref mut fresh20 =
                                                *pa_data.offset(1 as
                                                                    libc::c_int
                                                                    as isize);
                                            *fresh20 = 0 as *mut krb5_pa_data;
                                            let ref mut fresh21 =
                                                *pa_data.offset(0 as
                                                                    libc::c_int
                                                                    as isize);
                                            *fresh21 =
                                                malloc(::std::mem::size_of::<krb5_pa_data>()
                                                           as libc::c_ulong)
                                                    as *mut krb5_pa_data;
                                            if (*pa_data.offset(0 as
                                                                    libc::c_int
                                                                    as
                                                                    isize)).is_null()
                                               {
                                                free(pa_data as
                                                         *mut libc::c_void);
                                                retval = 12 as libc::c_int
                                            } else {
                                                (**pa_data.offset(0 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)).pa_type
                                                    = 109 as libc::c_int;
                                                (**pa_data.offset(0 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)).length
                                                    = (*encoded_algId).length;
                                                let ref mut fresh22 =
                                                    (**pa_data.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)).contents;
                                                *fresh22 =
                                                    (*encoded_algId).data as
                                                        *mut krb5_octet;
                                                *e_data_out = pa_data;
                                                retval = 0 as libc::c_int
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    free(buf1 as *mut libc::c_void);
    free(buf2 as *mut libc::c_void);
    free(buf3 as *mut libc::c_void);
    free(encoded_algId as *mut libc::c_void);
    if !algId.is_null() {
        while !(*algId.offset(i as isize)).is_null() {
            free((**algId.offset(i as isize)).parameters.data as
                     *mut libc::c_void);
            free(*algId.offset(i as isize) as *mut libc::c_void);
            i = i.wrapping_add(1)
        }
        free(algId as *mut libc::c_void);
    }
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "3324:1"]
pub unsafe extern "C" fn pkinit_check_kdc_pkid(mut context: krb5_context,
                                               mut plg_cryptoctx:
                                                   pkinit_plg_crypto_context,
                                               mut req_cryptoctx:
                                                   pkinit_req_crypto_context,
                                               mut id_cryptoctx:
                                                   pkinit_identity_crypto_context,
                                               mut pdid_buf:
                                                   *mut libc::c_uchar,
                                               mut pkid_len: libc::c_uint,
                                               mut valid_kdcPkId:
                                                   *mut libc::c_int)
 -> krb5_error_code {
    let mut is: *mut PKCS7_ISSUER_AND_SERIAL =
        0 as *mut PKCS7_ISSUER_AND_SERIAL;
    let mut p: *const libc::c_uchar = pdid_buf;
    let mut status: libc::c_int = 1 as libc::c_int;
    let mut kdc_cert: *mut X509 =
        sk_X509_value((*id_cryptoctx).my_certs, (*id_cryptoctx).cert_index);
    *valid_kdcPkId = 0 as libc::c_int;
    pkiDebug(b"found kdcPkId in AS REQ\n\x00" as *const u8 as
                 *const libc::c_char);
    is =
        d2i_PKCS7_ISSUER_AND_SERIAL(0 as *mut *mut PKCS7_ISSUER_AND_SERIAL,
                                    &mut p,
                                    pkid_len as libc::c_int as libc::c_long);
    if is.is_null() {
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    status = X509_NAME_cmp(X509_get_issuer_name(kdc_cert), (*is).issuer);
    if status == 0 {
        status =
            ASN1_INTEGER_cmp(X509_get_serialNumber(kdc_cert), (*is).serial);
        if status == 0 { *valid_kdcPkId = 1 as libc::c_int }
    }
    X509_NAME_free((*is).issuer);
    ASN1_INTEGER_free((*is).serial);
    free(is as *mut libc::c_void);
    return 0 as libc::c_int;
}
/* Check parameters against a well-known DH group. */
#[c2rust::src_loc = "3359:1"]
unsafe extern "C" fn pkinit_check_dh_params(mut dh1: *mut DH,
                                            mut dh2: *mut DH) -> libc::c_int {
    let mut p1: *const BIGNUM = 0 as *const BIGNUM;
    let mut p2: *const BIGNUM = 0 as *const BIGNUM;
    let mut g1: *const BIGNUM = 0 as *const BIGNUM;
    let mut g2: *const BIGNUM = 0 as *const BIGNUM;
    DH_get0_pqg(dh1, &mut p1, 0 as *mut *const BIGNUM, &mut g1);
    DH_get0_pqg(dh2, &mut p2, 0 as *mut *const BIGNUM, &mut g2);
    if BN_cmp(p1, p2) != 0 as libc::c_int {
        pkiDebug(b"p is not well-known group dhparameter\n\x00" as *const u8
                     as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    if BN_cmp(g1, g2) != 0 as libc::c_int {
        pkiDebug(b"bad g dhparameter\n\x00" as *const u8 as
                     *const libc::c_char);
        return -(1 as libc::c_int)
    }
    pkiDebug(b"good %d dhparams\n\x00" as *const u8 as *const libc::c_char,
             BN_num_bits(p1));
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "3378:1"]
pub unsafe extern "C" fn pkinit_process_td_dh_params(mut context:
                                                         krb5_context,
                                                     mut cryptoctx:
                                                         pkinit_plg_crypto_context,
                                                     mut req_cryptoctx:
                                                         pkinit_req_crypto_context,
                                                     mut id_cryptoctx:
                                                         pkinit_identity_crypto_context,
                                                     mut algId:
                                                         *mut *mut krb5_algorithm_identifier,
                                                     mut new_dh_size:
                                                         *mut libc::c_int)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code =
        -(1765328319 as libc::c_long) as krb5_error_code;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut use_sent_dh: libc::c_int = 0 as libc::c_int;
    let mut ok: libc::c_int = 0 as libc::c_int;
    pkiDebug(b"dh parameters\n\x00" as *const u8 as *const libc::c_char);
    loop  {
        if (*algId.offset(i as isize)).is_null() {
            current_block = 8704759739624374314;
            break ;
        }
        let mut dh: *mut DH = 0 as *mut DH;
        let mut p: *const BIGNUM = 0 as *const BIGNUM;
        let mut dh_prime_bits: libc::c_int = 0 as libc::c_int;
        if (**algId.offset(i as isize)).algorithm.length != dh_oid.length ||
               memcmp((**algId.offset(i as isize)).algorithm.data as
                          *const libc::c_void,
                      dh_oid.data as *const libc::c_void,
                      dh_oid.length as libc::c_ulong) != 0 {
            current_block = 11942242594846645614;
            break ;
        }
        dh =
            decode_dh_params((**algId.offset(i as isize)).parameters.data as
                                 *mut uint8_t,
                             (**algId.offset(i as isize)).parameters.length);
        if dh.is_null() { current_block = 11942242594846645614; break ; }
        DH_get0_pqg(dh, &mut p, 0 as *mut *const BIGNUM,
                    0 as *mut *const BIGNUM);
        dh_prime_bits = BN_num_bits(p);
        pkiDebug(b"client sent %d DH bits server prefers %d DH bits\n\x00" as
                     *const u8 as *const libc::c_char, *new_dh_size,
                 dh_prime_bits);
        ok = check_dh_wellknown(cryptoctx, dh, dh_prime_bits);
        if ok != 0 { *new_dh_size = dh_prime_bits }
        if ok == 0 {
            DH_check(dh, &mut retval);
            if retval != 0 as libc::c_int {
                pkiDebug(b"DH parameters provided by server are unacceptable\n\x00"
                             as *const u8 as *const libc::c_char);
                retval = -(1765328319 as libc::c_long) as krb5_error_code
            } else { use_sent_dh = 1 as libc::c_int; ok = 1 as libc::c_int }
        }
        if use_sent_dh == 0 { DH_free(dh); }
        if ok != 0 {
            if !(*req_cryptoctx).dh.is_null() {
                DH_free((*req_cryptoctx).dh);
                (*req_cryptoctx).dh = 0 as *mut DH
            }
            if use_sent_dh != 0 { (*req_cryptoctx).dh = dh }
            current_block = 8704759739624374314;
            break ;
        } else { i += 1 }
    }
    match current_block {
        8704759739624374314 => { if ok != 0 { retval = 0 as libc::c_int } }
        _ => { }
    }
    return retval;
}
#[c2rust::src_loc = "3444:1"]
unsafe extern "C" fn openssl_callback(mut ok: libc::c_int,
                                      mut ctx: *mut X509_STORE_CTX)
 -> libc::c_int {
    return ok;
}
#[c2rust::src_loc = "3462:1"]
unsafe extern "C" fn openssl_callback_ignore_crls(mut ok: libc::c_int,
                                                  mut ctx:
                                                      *mut X509_STORE_CTX)
 -> libc::c_int {
    if ok != 0 { return ok }
    return (X509_STORE_CTX_get_error(ctx) == 3 as libc::c_int) as libc::c_int;
}
#[c2rust::src_loc = "3470:1"]
unsafe extern "C" fn pkinit_pkcs7type2oid(mut cryptoctx:
                                              pkinit_plg_crypto_context,
                                          mut pkcs7_type: libc::c_int)
 -> *mut ASN1_OBJECT {
    match pkcs7_type {
        0 => { return (*cryptoctx).id_pkinit_authData }
        1 => { return (*cryptoctx).id_pkinit_DHKeyData }
        2 => { return (*cryptoctx).id_pkinit_rkeyData }
        _ => { return 0 as *mut ASN1_OBJECT }
    };
}
#[c2rust::src_loc = "3486:1"]
unsafe extern "C" fn wrap_signeddata(mut data: *mut libc::c_uchar,
                                     mut data_len: libc::c_uint,
                                     mut out: *mut *mut libc::c_uchar,
                                     mut out_len: *mut libc::c_uint)
 -> libc::c_int {
    let mut orig_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut oid_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut tot_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut oid: *mut ASN1_OBJECT = 0 as *mut ASN1_OBJECT;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    /* Get length to wrap the original data with SEQUENCE tag */
    orig_len =
        ASN1_object_size(1 as libc::c_int, data_len as libc::c_int,
                         16 as libc::c_int) as libc::c_uint;
    tot_len = orig_len;
    /* Add the signedData OID and adjust lengths */
    oid = OBJ_nid2obj(22 as libc::c_int);
    oid_len =
        i2d_ASN1_OBJECT(oid, 0 as *mut *mut libc::c_uchar) as libc::c_uint;
    tot_len =
        ASN1_object_size(1 as libc::c_int,
                         orig_len.wrapping_add(oid_len) as libc::c_int,
                         16 as libc::c_int) as libc::c_uint;
    *out = malloc(tot_len as libc::c_ulong) as *mut libc::c_uchar;
    p = *out;
    if p.is_null() { return -(1 as libc::c_int) }
    ASN1_put_object(&mut p, 1 as libc::c_int,
                    orig_len.wrapping_add(oid_len) as libc::c_int,
                    16 as libc::c_int, 0 as libc::c_int);
    i2d_ASN1_OBJECT(oid, &mut p);
    ASN1_put_object(&mut p, 1 as libc::c_int, data_len as libc::c_int,
                    0 as libc::c_int, 0x80 as libc::c_int);
    memcpy(p as *mut libc::c_void, data as *const libc::c_void,
           data_len as libc::c_ulong);
    *out_len = tot_len;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "3520:1"]
unsafe extern "C" fn prepare_enc_data(mut indata: *const uint8_t,
                                      mut indata_len: libc::c_int,
                                      mut outdata: *mut *mut uint8_t,
                                      mut outdata_len: *mut libc::c_int)
 -> libc::c_int {
    let mut tag: libc::c_int = 0;
    let mut class: libc::c_int = 0;
    let mut tlen: libc::c_long = 0;
    let mut slen: libc::c_long = 0;
    let mut p: *const uint8_t = indata;
    let mut oldp: *const uint8_t = 0 as *const uint8_t;
    if ASN1_get_object(&mut p, &mut slen, &mut tag, &mut class,
                       indata_len as libc::c_long) & 0x80 as libc::c_int != 0
       {
        return 22 as libc::c_int
    }
    if tag != 16 as libc::c_int { return 22 as libc::c_int }
    oldp = p;
    if ASN1_get_object(&mut p, &mut tlen, &mut tag, &mut class, slen) &
           0x80 as libc::c_int != 0 {
        return 22 as libc::c_int
    }
    p = p.offset(tlen as isize);
    slen -= p.wrapping_offset_from(oldp) as libc::c_long;
    if ASN1_get_object(&mut p, &mut tlen, &mut tag, &mut class, slen) &
           0x80 as libc::c_int != 0 {
        return 22 as libc::c_int
    }
    *outdata = malloc(tlen as libc::c_ulong) as *mut uint8_t;
    if (*outdata).is_null() { return 12 as libc::c_int }
    memcpy(*outdata as *mut libc::c_void, p as *const libc::c_void,
           tlen as libc::c_ulong);
    *outdata_len = tlen as libc::c_int;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "3551:1"]
unsafe extern "C" fn pkinit_C_LoadModule(mut modname: *const libc::c_char,
                                         mut p11p: CK_FUNCTION_LIST_PTR_PTR)
 -> *mut libc::c_void {
    let mut handle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut getflist:
            Option<unsafe extern "C" fn(_: CK_FUNCTION_LIST_PTR_PTR)
                       -> CK_RV> = None;
    pkiDebug(b"loading module \"%s\"... \x00" as *const u8 as
                 *const libc::c_char, modname);
    handle = dlopen(modname, 0x2 as libc::c_int);
    if handle.is_null() {
        pkiDebug(b"not found\n\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut libc::c_void
    }
    getflist =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                CK_FUNCTION_LIST_PTR_PTR)
                                           ->
                                               CK_RV>>(dlsym(handle,
                                                             b"C_GetFunctionList\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char));
    if getflist.is_none() ||
           Some(getflist.expect("non-null function pointer")).expect("non-null function pointer")(p11p)
               != 0 as libc::c_int as libc::c_ulong {
        dlclose(handle);
        pkiDebug(b"failed\n\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut libc::c_void
    }
    pkiDebug(b"ok\n\x00" as *const u8 as *const libc::c_char);
    return handle;
}
#[c2rust::src_loc = "3573:1"]
unsafe extern "C" fn pkinit_C_UnloadModule(mut handle: *mut libc::c_void)
 -> CK_RV {
    dlclose(handle);
    return 0 as libc::c_int as CK_RV;
}
#[c2rust::src_loc = "3580:1"]
unsafe extern "C" fn pkinit_login(mut context: krb5_context,
                                  mut id_cryptoctx:
                                      pkinit_identity_crypto_context,
                                  mut tip: *mut CK_TOKEN_INFO,
                                  mut password: *const libc::c_char)
 -> krb5_error_code {
    let mut rdat: krb5_data =
        krb5_data{magic: 0,
                  length: 0,
                  data: 0 as *const libc::c_char as *mut libc::c_char,};
    let mut prompt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut warning: *const libc::c_char = 0 as *const libc::c_char;
    let mut kprompt: krb5_prompt =
        krb5_prompt{prompt: 0 as *mut libc::c_char,
                    hidden: 0,
                    reply: 0 as *mut krb5_data,};
    let mut prompt_type: krb5_prompt_type = 0;
    let mut r: libc::c_int = 0 as libc::c_int;
    if (*tip).flags &
           ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong != 0 {
        rdat.data = 0 as *mut libc::c_char;
        rdat.length = 0 as libc::c_int as libc::c_uint
    } else if !password.is_null() {
        rdat.data = strdup(password);
        rdat.length = strlen(password) as libc::c_uint
    } else if (*id_cryptoctx).prompter.is_none() {
        r = -(1765328254 as libc::c_long) as libc::c_int;
        rdat.data = 0 as *mut libc::c_char
    } else {
        if (*tip).flags &
               ((1 as libc::c_int) << 18 as libc::c_int) as libc::c_ulong != 0
           {
            warning =
                b" (Warning: PIN locked)\x00" as *const u8 as
                    *const libc::c_char
        } else if (*tip).flags &
                      ((1 as libc::c_int) << 17 as libc::c_int) as
                          libc::c_ulong != 0 {
            warning =
                b" (Warning: PIN final try)\x00" as *const u8 as
                    *const libc::c_char
        } else if (*tip).flags &
                      ((1 as libc::c_int) << 16 as libc::c_int) as
                          libc::c_ulong != 0 {
            warning =
                b" (Warning: PIN count low)\x00" as *const u8 as
                    *const libc::c_char
        } else { warning = b"\x00" as *const u8 as *const libc::c_char }
        if asprintf(&mut prompt as *mut *mut libc::c_char,
                    b"%.*s PIN%s\x00" as *const u8 as *const libc::c_char,
                    ::std::mem::size_of::<[libc::c_uchar; 32]>() as
                        libc::c_ulong as libc::c_int,
                    (*tip).label.as_mut_ptr(), warning) < 0 as libc::c_int {
            return 12 as libc::c_int
        }
        rdat.data =
            malloc((*tip).ulMaxPinLen.wrapping_add(2 as libc::c_int as
                                                       libc::c_ulong)) as
                *mut libc::c_char;
        rdat.length =
            (*tip).ulMaxPinLen.wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_uint;
        kprompt.prompt = prompt;
        kprompt.hidden = 1 as libc::c_int;
        kprompt.reply = &mut rdat;
        prompt_type = 0x4 as libc::c_int;
        /* PROMPTER_INVOCATION */
        k5int_set_prompt_types.expect("non-null function pointer")(context,
                                                                   &mut prompt_type); /* session already open */
        r =
            Some((*id_cryptoctx).prompter.expect("non-null function pointer")).expect("non-null function pointer")(context,
                                                                                                                   (*id_cryptoctx).prompter_data,
                                                                                                                   0
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   0
                                                                                                                       as
                                                                                                                       *const libc::c_char,
                                                                                                                   1
                                                                                                                       as
                                                                                                                       libc::c_int,
                                                                                                                   &mut kprompt);
        k5int_set_prompt_types.expect("non-null function pointer")(context,
                                                                   0 as
                                                                       *mut krb5_prompt_type);
        free(prompt as *mut libc::c_void);
    }
    if r == 0 as libc::c_int {
        r =
            (*(*id_cryptoctx).p11).C_Login.expect("non-null function pointer")((*id_cryptoctx).session,
                                                                               1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   CK_USER_TYPE,
                                                                               rdat.data
                                                                                   as
                                                                                   *mut u_char,
                                                                               rdat.length
                                                                                   as
                                                                                   libc::c_ulong)
                as libc::c_int;
        if r != 0 as libc::c_int {
            pkiDebug(b"C_Login: %s\n\x00" as *const u8 as *const libc::c_char,
                     pkinit_pkcs11_code_to_text(r));
            r = -(1765328360 as libc::c_long) as libc::c_int
        }
    }
    free(rdat.data as *mut libc::c_void);
    return r;
}
#[c2rust::src_loc = "3643:1"]
unsafe extern "C" fn pkinit_open_session(mut context: krb5_context,
                                         mut cctx:
                                             pkinit_identity_crypto_context)
 -> krb5_error_code {
    let mut i: CK_ULONG = 0;
    let mut r: CK_ULONG = 0;
    let mut cp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut label_len: size_t = 0;
    let mut count: CK_ULONG = 0 as libc::c_int as CK_ULONG;
    let mut slotlist: CK_SLOT_ID_PTR = 0 as *mut CK_SLOT_ID;
    let mut tinfo: CK_TOKEN_INFO =
        CK_TOKEN_INFO{label: [0; 32],
                      manufacturerID: [0; 32],
                      model: [0; 16],
                      serialNumber: [0; 16],
                      flags: 0,
                      ulMaxSessionCount: 0,
                      ulSessionCount: 0,
                      ulMaxRwSessionCount: 0,
                      ulRwSessionCount: 0,
                      ulMaxPinLen: 0,
                      ulMinPinLen: 0,
                      ulTotalPublicMemory: 0,
                      ulFreePublicMemory: 0,
                      ulTotalPrivateMemory: 0,
                      ulFreePrivateMemory: 0,
                      hardwareVersion: _CK_VERSION{major: 0, minor: 0,},
                      firmwareVersion: _CK_VERSION{major: 0, minor: 0,},
                      utcTime: [0; 16],};
    let mut p11name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut password: *const libc::c_char = 0 as *const libc::c_char;
    if !(*cctx).p11_module.is_null() { return 0 as libc::c_int }
    /* Load module */
    (*cctx).p11_module =
        pkinit_C_LoadModule((*cctx).p11_module_name, &mut (*cctx).p11);
    if (*cctx).p11_module.is_null() {
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    /* Init */
    r =
        (*(*cctx).p11).C_Initialize.expect("non-null function pointer")(0 as
                                                                            *mut libc::c_void);
    if r != 0 as libc::c_int as libc::c_ulong {
        pkiDebug(b"C_Initialize: %s\n\x00" as *const u8 as
                     *const libc::c_char,
                 pkinit_pkcs11_code_to_text(r as libc::c_int));
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    /* Get the list of available slots */
    if (*(*cctx).p11).C_GetSlotList.expect("non-null function pointer")(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uchar,
                                                                        0 as
                                                                            *mut CK_SLOT_ID,
                                                                        &mut count)
           != 0 as libc::c_int as libc::c_ulong {
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    if count == 0 as libc::c_int as libc::c_ulong {
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    slotlist =
        calloc(count, ::std::mem::size_of::<CK_SLOT_ID>() as libc::c_ulong) as
            CK_SLOT_ID_PTR;
    if slotlist.is_null() { return 12 as libc::c_int }
    if (*(*cctx).p11).C_GetSlotList.expect("non-null function pointer")(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uchar,
                                                                        slotlist,
                                                                        &mut count)
           != 0 as libc::c_int as libc::c_ulong {
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    /* Look for the given token label, or if none given take the first one */
    i = 0 as libc::c_int as CK_ULONG;
    while i < count {
        /* Skip slots that don't match the specified slotid, if given. */
        if !((*cctx).slotid != 999999 as libc::c_int as libc::c_ulong &&
                 (*cctx).slotid != *slotlist.offset(i as isize)) {
            /* Open session */
            r =
                (*(*cctx).p11).C_OpenSession.expect("non-null function pointer")(*slotlist.offset(i
                                                                                                      as
                                                                                                      isize),
                                                                                 ((1
                                                                                       as
                                                                                       libc::c_int)
                                                                                      <<
                                                                                      2
                                                                                          as
                                                                                          libc::c_int)
                                                                                     as
                                                                                     CK_FLAGS,
                                                                                 0
                                                                                     as
                                                                                     *mut libc::c_void,
                                                                                 None,
                                                                                 &mut (*cctx).session);
            if r != 0 as libc::c_int as libc::c_ulong {
                pkiDebug(b"C_OpenSession: %s\n\x00" as *const u8 as
                             *const libc::c_char,
                         pkinit_pkcs11_code_to_text(r as libc::c_int));
                return -(1765328360 as libc::c_long) as krb5_error_code
            }
            /* Get token info */
            r =
                (*(*cctx).p11).C_GetTokenInfo.expect("non-null function pointer")(*slotlist.offset(i
                                                                                                       as
                                                                                                       isize),
                                                                                  &mut tinfo);
            if r != 0 as libc::c_int as libc::c_ulong {
                pkiDebug(b"C_GetTokenInfo: %s\n\x00" as *const u8 as
                             *const libc::c_char,
                         pkinit_pkcs11_code_to_text(r as libc::c_int));
                return -(1765328360 as libc::c_long) as krb5_error_code
            }
            /* tinfo.label is zero-filled but not necessarily zero-terminated.
         * Find the length, ignoring any trailing spaces. */
            cp =
                tinfo.label.as_mut_ptr().offset(::std::mem::size_of::<[libc::c_uchar; 32]>()
                                                    as libc::c_ulong as
                                                    isize);
            while cp > tinfo.label.as_mut_ptr() {
                if *cp.offset(-(1 as libc::c_int) as isize) as libc::c_int !=
                       '\u{0}' as i32 &&
                       *cp.offset(-(1 as libc::c_int) as isize) as libc::c_int
                           != ' ' as i32 {
                    break ;
                }
                cp = cp.offset(-1)
            }
            label_len =
                cp.wrapping_offset_from(tinfo.label.as_mut_ptr()) as
                    libc::c_long as size_t;
            pkiDebug(b"open_session: slotid %d token \"%.*s\"\n\x00" as
                         *const u8 as *const libc::c_char,
                     *slotlist.offset(i as isize) as libc::c_int,
                     label_len as libc::c_int, tinfo.label.as_mut_ptr());
            if (*cctx).token_label.is_null() ||
                   strlen((*cctx).token_label) == label_len &&
                       memcmp((*cctx).token_label as *const libc::c_void,
                              tinfo.label.as_mut_ptr() as *const libc::c_void,
                              label_len) == 0 as libc::c_int {
                break ;
            }
            (*(*cctx).p11).C_CloseSession.expect("non-null function pointer")((*cctx).session);
        }
        i = i.wrapping_add(1)
    }
    if i >= count {
        free(slotlist as *mut libc::c_void);
        pkiDebug(b"open_session: no matching token found\n\x00" as *const u8
                     as *const libc::c_char);
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    (*cctx).slotid = *slotlist.offset(i as isize);
    free(slotlist as *mut libc::c_void);
    pkiDebug(b"open_session: slotid %d (%lu of %d)\n\x00" as *const u8 as
                 *const libc::c_char, (*cctx).slotid as libc::c_int,
             i.wrapping_add(1 as libc::c_int as libc::c_ulong),
             count as libc::c_int);
    /* Login if needed */
    if tinfo.flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong
           != 0 {
        if !(*cctx).p11_module_name.is_null() {
            if (*cctx).slotid != 999999 as libc::c_int as libc::c_ulong {
                if asprintf(&mut p11name as *mut *mut libc::c_char,
                            b"PKCS11:module_name=%s:slotid=%ld:token=%.*s\x00"
                                as *const u8 as *const libc::c_char,
                            (*cctx).p11_module_name,
                            (*cctx).slotid as libc::c_long,
                            label_len as libc::c_int,
                            tinfo.label.as_mut_ptr()) < 0 as libc::c_int {
                    p11name = 0 as *mut libc::c_char
                }
            } else if asprintf(&mut p11name as *mut *mut libc::c_char,
                               b"PKCS11:module_name=%s,token=%.*s\x00" as
                                   *const u8 as *const libc::c_char,
                               (*cctx).p11_module_name,
                               label_len as libc::c_int,
                               tinfo.label.as_mut_ptr()) < 0 as libc::c_int {
                p11name = 0 as *mut libc::c_char
            }
        } else { p11name = 0 as *mut libc::c_char }
        if (*cctx).defer_id_prompt != 0 {
            /* Supply the identity name to be passed to the responder. */
            pkinit_set_deferred_id(&mut (*cctx).deferred_ids, p11name,
                                   tinfo.flags, 0 as *const libc::c_char);
            free(p11name as *mut libc::c_void);
            return -(1765328324 as libc::c_long) as krb5_error_code
        }
        /* Look up a responder-supplied password for the token. */
        password = pkinit_find_deferred_id((*cctx).deferred_ids, p11name);
        free(p11name as *mut libc::c_void);
        r = pkinit_login(context, cctx, &mut tinfo, password) as CK_ULONG
    }
    return r as krb5_error_code;
}
/*
 * Look for a key that's:
 * 1. private
 * 2. capable of the specified operation (usually signing or decrypting)
 * 3. RSA (this may be wrong but it's all we can do for now)
 * 4. matches the id of the cert we chose
 *
 * You must call pkinit_get_certs before calling pkinit_find_private_key
 * (that's because we need the ID of the private key)
 *
 * pkcs11 says the id of the key doesn't have to match that of the cert, but
 * I can't figure out any other way to decide which key to use.
 *
 * We should only find one key that fits all the requirements.
 * If there are more than one, we just take the first one.
 */
#[c2rust::src_loc = "3779:1"]
unsafe extern "C" fn pkinit_find_private_key(mut id_cryptoctx:
                                                 pkinit_identity_crypto_context,
                                             mut usage: CK_ATTRIBUTE_TYPE,
                                             mut objp: *mut CK_OBJECT_HANDLE)
 -> krb5_error_code {
    let mut cls: CK_OBJECT_CLASS = 0;
    let mut attrs: [CK_ATTRIBUTE; 4] =
        [CK_ATTRIBUTE{type_0: 0,
                      pValue: 0 as *mut libc::c_void,
                      ulValueLen: 0,}; 4];
    let mut count: CK_ULONG = 0;
    let mut keytype: CK_KEY_TYPE = 0;
    let mut nattrs: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut r: libc::c_int = 0;
    cls = 3 as libc::c_int as CK_OBJECT_CLASS;
    attrs[nattrs as usize].type_0 = 0 as libc::c_int as CK_ATTRIBUTE_TYPE;
    attrs[nattrs as usize].pValue =
        &mut cls as *mut CK_OBJECT_CLASS as *mut libc::c_void;
    attrs[nattrs as usize].ulValueLen =
        ::std::mem::size_of::<CK_OBJECT_CLASS>() as libc::c_ulong;
    nattrs = nattrs.wrapping_add(1);
    keytype = 0 as libc::c_int as CK_KEY_TYPE;
    attrs[nattrs as usize].type_0 = 0x100 as libc::c_int as CK_ATTRIBUTE_TYPE;
    attrs[nattrs as usize].pValue =
        &mut keytype as *mut CK_KEY_TYPE as *mut libc::c_void;
    attrs[nattrs as usize].ulValueLen =
        ::std::mem::size_of::<CK_KEY_TYPE>() as libc::c_ulong;
    nattrs = nattrs.wrapping_add(1);
    attrs[nattrs as usize].type_0 = 0x102 as libc::c_int as CK_ATTRIBUTE_TYPE;
    attrs[nattrs as usize].pValue =
        (*id_cryptoctx).cert_id as *mut libc::c_void;
    attrs[nattrs as usize].ulValueLen = (*id_cryptoctx).cert_id_len;
    nattrs = nattrs.wrapping_add(1);
    r =
        (*(*id_cryptoctx).p11).C_FindObjectsInit.expect("non-null function pointer")((*id_cryptoctx).session,
                                                                                     attrs.as_mut_ptr(),
                                                                                     nattrs
                                                                                         as
                                                                                         libc::c_ulong)
            as libc::c_int;
    if r != 0 as libc::c_int {
        pkiDebug(b"krb5_pkinit_sign_data: C_FindObjectsInit: %s\n\x00" as
                     *const u8 as *const libc::c_char,
                 pkinit_pkcs11_code_to_text(r));
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    r =
        (*(*id_cryptoctx).p11).C_FindObjects.expect("non-null function pointer")((*id_cryptoctx).session,
                                                                                 objp,
                                                                                 1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong,
                                                                                 &mut count)
            as libc::c_int;
    (*(*id_cryptoctx).p11).C_FindObjectsFinal.expect("non-null function pointer")((*id_cryptoctx).session);
    pkiDebug(b"found %d private keys (%s)\n\x00" as *const u8 as
                 *const libc::c_char, count as libc::c_int,
             pkinit_pkcs11_code_to_text(r));
    if r != 0 as libc::c_int || count < 1 as libc::c_int as libc::c_ulong {
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "3841:1"]
unsafe extern "C" fn pkinit_decode_data_fs(mut context: krb5_context,
                                           mut id_cryptoctx:
                                               pkinit_identity_crypto_context,
                                           mut data: *const uint8_t,
                                           mut data_len: libc::c_uint,
                                           mut decoded_data:
                                               *mut *mut uint8_t,
                                           mut decoded_data_len:
                                               *mut libc::c_uint)
 -> krb5_error_code {
    let mut cert: *mut X509 =
        sk_X509_value((*id_cryptoctx).my_certs, (*id_cryptoctx).cert_index);
    let mut pkey: *mut EVP_PKEY = (*id_cryptoctx).my_key;
    let mut buf: *mut uint8_t = 0 as *mut uint8_t;
    let mut buf_len: libc::c_int = 0;
    let mut decrypt_len: libc::c_int = 0;
    *decoded_data = 0 as *mut uint8_t;
    *decoded_data_len = 0 as libc::c_int as libc::c_uint;
    if !cert.is_null() && X509_check_private_key(cert, pkey) == 0 {
        pkiDebug(b"private key does not match certificate\n\x00" as *const u8
                     as *const libc::c_char);
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    buf_len = EVP_PKEY_size(pkey);
    buf =
        malloc((buf_len + 10 as libc::c_int) as libc::c_ulong) as
            *mut uint8_t;
    if buf.is_null() {
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    decrypt_len =
        EVP_PKEY_decrypt_old(buf, data, data_len as libc::c_int, pkey);
    if decrypt_len <= 0 as libc::c_int {
        pkiDebug(b"unable to decrypt received data (len=%d)\n\x00" as
                     *const u8 as *const libc::c_char, data_len);
        free(buf as *mut libc::c_void);
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    *decoded_data = buf;
    *decoded_data_len = decrypt_len as libc::c_uint;
    return 0 as libc::c_int;
}
/*
 * When using the ActivCard Linux pkcs11 library (v2.0.1), the decrypt function
 * fails.  By inserting an extra function call, which serves nothing but to
 * change the stack, we were able to work around the issue.  If the ActivCard
 * library is fixed in the future, this function can be inlined back into the
 * caller.
 */
#[c2rust::src_loc = "3886:1"]
unsafe extern "C" fn pkinit_C_Decrypt(mut id_cryptoctx:
                                          pkinit_identity_crypto_context,
                                      mut pEncryptedData: CK_BYTE_PTR,
                                      mut ulEncryptedDataLen: CK_ULONG,
                                      mut pData: CK_BYTE_PTR,
                                      mut pulDataLen: CK_ULONG_PTR) -> CK_RV {
    let mut rv: CK_RV = 0 as libc::c_int as CK_RV;
    rv =
        (*(*id_cryptoctx).p11).C_Decrypt.expect("non-null function pointer")((*id_cryptoctx).session,
                                                                             pEncryptedData,
                                                                             ulEncryptedDataLen,
                                                                             pData,
                                                                             pulDataLen);
    if rv == 0 as libc::c_int as libc::c_ulong {
        pkiDebug(b"pData %p *pulDataLen %d\n\x00" as *const u8 as
                     *const libc::c_char, pData as *mut libc::c_void,
                 *pulDataLen as libc::c_int);
    }
    return rv;
}
#[c2rust::src_loc = "3904:1"]
unsafe extern "C" fn pkinit_decode_data_pkcs11(mut context: krb5_context,
                                               mut id_cryptoctx:
                                                   pkinit_identity_crypto_context,
                                               mut data: *const uint8_t,
                                               mut data_len: libc::c_uint,
                                               mut decoded_data:
                                                   *mut *mut uint8_t,
                                               mut decoded_data_len:
                                                   *mut libc::c_uint)
 -> krb5_error_code {
    let mut obj: CK_OBJECT_HANDLE = 0;
    let mut len: CK_ULONG = 0;
    let mut mech: CK_MECHANISM =
        CK_MECHANISM{mechanism: 0,
                     pParameter: 0 as *mut libc::c_void,
                     ulParameterLen: 0,};
    let mut cp: *mut uint8_t = 0 as *mut uint8_t;
    let mut r: libc::c_int = 0;
    *decoded_data = 0 as *mut uint8_t;
    *decoded_data_len = 0 as libc::c_int as libc::c_uint;
    if pkinit_open_session(context, id_cryptoctx) != 0 {
        pkiDebug(b"can\'t open pkcs11 session\n\x00" as *const u8 as
                     *const libc::c_char);
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    pkinit_find_private_key(id_cryptoctx,
                            0x105 as libc::c_int as CK_ATTRIBUTE_TYPE,
                            &mut obj);
    mech.mechanism = 1 as libc::c_int as CK_MECHANISM_TYPE;
    mech.pParameter = 0 as *mut libc::c_void;
    mech.ulParameterLen = 0 as libc::c_int as libc::c_ulong;
    r =
        (*(*id_cryptoctx).p11).C_DecryptInit.expect("non-null function pointer")((*id_cryptoctx).session,
                                                                                 &mut mech,
                                                                                 obj)
            as libc::c_int;
    if r != 0 as libc::c_int {
        pkiDebug(b"C_DecryptInit: 0x%x\n\x00" as *const u8 as
                     *const libc::c_char, r);
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    pkiDebug(b"data_len = %d\n\x00" as *const u8 as *const libc::c_char,
             data_len);
    cp = malloc(data_len as size_t) as *mut uint8_t;
    if cp.is_null() { return 12 as libc::c_int }
    len = data_len as CK_ULONG;
    pkiDebug(b"session %p edata %p edata_len %d data %p datalen @%p %d\n\x00"
                 as *const u8 as *const libc::c_char,
             (*id_cryptoctx).session as *mut libc::c_void,
             data as *mut libc::c_void, data_len as libc::c_int,
             cp as *mut libc::c_void,
             &mut len as *mut CK_ULONG as *mut libc::c_void,
             len as libc::c_int);
    r =
        pkinit_C_Decrypt(id_cryptoctx, data as CK_BYTE_PTR,
                         data_len as CK_ULONG, cp, &mut len) as libc::c_int;
    if r != 0 as libc::c_int {
        pkiDebug(b"C_Decrypt: %s\n\x00" as *const u8 as *const libc::c_char,
                 pkinit_pkcs11_code_to_text(r));
        if r == 0x150 as libc::c_int {
            pkiDebug(b"decrypt %d needs %d\n\x00" as *const u8 as
                         *const libc::c_char, data_len as libc::c_int,
                     len as libc::c_int);
        }
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    pkiDebug(b"decrypt %d -> %d\n\x00" as *const u8 as *const libc::c_char,
             data_len as libc::c_int, len as libc::c_int);
    *decoded_data_len = len as libc::c_uint;
    *decoded_data = cp;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "3960:1"]
unsafe extern "C" fn pkinit_decode_data(mut context: krb5_context,
                                        mut id_cryptoctx:
                                            pkinit_identity_crypto_context,
                                        mut data: *const uint8_t,
                                        mut data_len: libc::c_uint,
                                        mut decoded_data: *mut *mut uint8_t,
                                        mut decoded_data_len:
                                            *mut libc::c_uint)
 -> krb5_error_code {
    let mut retval: krb5_error_code =
        -(1765328360 as libc::c_long) as krb5_error_code;
    *decoded_data = 0 as *mut uint8_t;
    *decoded_data_len = 0 as libc::c_int as libc::c_uint;
    if (*id_cryptoctx).pkcs11_method != 1 as libc::c_int {
        retval =
            pkinit_decode_data_fs(context, id_cryptoctx, data, data_len,
                                  decoded_data, decoded_data_len)
    } else {
        retval =
            pkinit_decode_data_pkcs11(context, id_cryptoctx, data, data_len,
                                      decoded_data, decoded_data_len)
    }
    return retval;
}
/* WITHOUT_PKCS11 */
#[c2rust::src_loc = "3983:1"]
unsafe extern "C" fn pkinit_sign_data_fs(mut context: krb5_context,
                                         mut id_cryptoctx:
                                             pkinit_identity_crypto_context,
                                         mut data: *mut libc::c_uchar,
                                         mut data_len: libc::c_uint,
                                         mut sig: *mut *mut libc::c_uchar,
                                         mut sig_len: *mut libc::c_uint)
 -> krb5_error_code {
    if create_signature(sig, sig_len, data, data_len, (*id_cryptoctx).my_key)
           != 0 as libc::c_int {
        pkiDebug(b"failed to create the signature\n\x00" as *const u8 as
                     *const libc::c_char);
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "4000:1"]
unsafe extern "C" fn pkinit_sign_data_pkcs11(mut context: krb5_context,
                                             mut id_cryptoctx:
                                                 pkinit_identity_crypto_context,
                                             mut data: *mut libc::c_uchar,
                                             mut data_len: libc::c_uint,
                                             mut sig: *mut *mut libc::c_uchar,
                                             mut sig_len: *mut libc::c_uint)
 -> krb5_error_code {
    let mut obj: CK_OBJECT_HANDLE = 0;
    let mut len: CK_ULONG = 0;
    let mut mech: CK_MECHANISM =
        CK_MECHANISM{mechanism: 0,
                     pParameter: 0 as *mut libc::c_void,
                     ulParameterLen: 0,};
    let mut cp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut r: libc::c_int = 0;
    if pkinit_open_session(context, id_cryptoctx) != 0 {
        pkiDebug(b"can\'t open pkcs11 session\n\x00" as *const u8 as
                     *const libc::c_char);
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    pkinit_find_private_key(id_cryptoctx,
                            0x108 as libc::c_int as CK_ATTRIBUTE_TYPE,
                            &mut obj);
    mech.mechanism = (*id_cryptoctx).mech;
    mech.pParameter = 0 as *mut libc::c_void;
    mech.ulParameterLen = 0 as libc::c_int as libc::c_ulong;
    r =
        (*(*id_cryptoctx).p11).C_SignInit.expect("non-null function pointer")((*id_cryptoctx).session,
                                                                              &mut mech,
                                                                              obj)
            as libc::c_int;
    if r != 0 as libc::c_int {
        pkiDebug(b"C_SignInit: %s\n\x00" as *const u8 as *const libc::c_char,
                 pkinit_pkcs11_code_to_text(r));
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    /*
     * Key len would give an upper bound on sig size, but there's no way to
     * get that. So guess, and if it's too small, re-malloc.
     */
    len = 1000 as libc::c_int as CK_ULONG;
    cp = malloc(len) as *mut libc::c_uchar;
    if cp.is_null() { return 12 as libc::c_int }
    r =
        (*(*id_cryptoctx).p11).C_Sign.expect("non-null function pointer")((*id_cryptoctx).session,
                                                                          data,
                                                                          data_len
                                                                              as
                                                                              CK_ULONG,
                                                                          cp,
                                                                          &mut len)
            as libc::c_int;
    if r == 0x150 as libc::c_int ||
           r == 0 as libc::c_int &&
               len >= 1000 as libc::c_int as libc::c_ulong {
        free(cp as *mut libc::c_void);
        pkiDebug(b"C_Sign realloc %d\n\x00" as *const u8 as
                     *const libc::c_char, len as libc::c_int);
        cp = malloc(len) as *mut libc::c_uchar;
        r =
            (*(*id_cryptoctx).p11).C_Sign.expect("non-null function pointer")((*id_cryptoctx).session,
                                                                              data,
                                                                              data_len
                                                                                  as
                                                                                  CK_ULONG,
                                                                              cp,
                                                                              &mut len)
                as libc::c_int
    }
    if r != 0 as libc::c_int {
        pkiDebug(b"C_Sign: %s\n\x00" as *const u8 as *const libc::c_char,
                 pkinit_pkcs11_code_to_text(r));
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    pkiDebug(b"sign %d -> %d\n\x00" as *const u8 as *const libc::c_char,
             data_len as libc::c_int, len as libc::c_int);
    *sig_len = len as libc::c_uint;
    *sig = cp;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "4061:1"]
unsafe extern "C" fn pkinit_sign_data(mut context: krb5_context,
                                      mut id_cryptoctx:
                                          pkinit_identity_crypto_context,
                                      mut data: *mut libc::c_uchar,
                                      mut data_len: libc::c_uint,
                                      mut sig: *mut *mut libc::c_uchar,
                                      mut sig_len: *mut libc::c_uint)
 -> krb5_error_code {
    let mut retval: krb5_error_code =
        -(1765328360 as libc::c_long) as krb5_error_code;
    if id_cryptoctx.is_null() ||
           (*id_cryptoctx).pkcs11_method != 1 as libc::c_int {
        retval =
            pkinit_sign_data_fs(context, id_cryptoctx, data, data_len, sig,
                                sig_len)
    } else {
        retval =
            pkinit_sign_data_pkcs11(context, id_cryptoctx, data, data_len,
                                    sig, sig_len)
    }
    return retval;
}
#[c2rust::src_loc = "4084:1"]
unsafe extern "C" fn create_signature(mut sig: *mut *mut libc::c_uchar,
                                      mut sig_len: *mut libc::c_uint,
                                      mut data: *mut libc::c_uchar,
                                      mut data_len: libc::c_uint,
                                      mut pkey: *mut EVP_PKEY)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut ctx: *mut EVP_MD_CTX = 0 as *mut EVP_MD_CTX;
    if pkey.is_null() { return retval }
    ctx = EVP_MD_CTX_new();
    if ctx.is_null() { return 12 as libc::c_int }
    EVP_DigestInit(ctx, EVP_sha1());
    EVP_DigestUpdate(ctx, data as *const libc::c_void, data_len as size_t);
    *sig_len = EVP_PKEY_size(pkey) as libc::c_uint;
    *sig = malloc(*sig_len as libc::c_ulong) as *mut libc::c_uchar;
    if !(*sig).is_null() {
        EVP_SignFinal(ctx, *sig, sig_len, pkey);
        retval = 0 as libc::c_int
    }
    EVP_MD_CTX_free(ctx);
    return retval;
}
/*
 * Note:
 * This is not the routine the KDC uses to get its certificate.
 * This routine is intended to be called by the client
 * to obtain the KDC's certificate from some local storage
 * to be sent as a hint in its request to the KDC.
 */
#[no_mangle]
#[c2rust::src_loc = "4119:1"]
pub unsafe extern "C" fn pkinit_get_kdc_cert(mut context: krb5_context,
                                             mut plg_cryptoctx:
                                                 pkinit_plg_crypto_context,
                                             mut req_cryptoctx:
                                                 pkinit_req_crypto_context,
                                             mut id_cryptoctx:
                                                 pkinit_identity_crypto_context,
                                             mut princ: krb5_principal)
 -> krb5_error_code {
    let mut retval: krb5_error_code =
        -(1765328360 as libc::c_long) as krb5_error_code;
    (*req_cryptoctx).received_cert = 0 as *mut X509;
    retval = 0 as libc::c_int;
    return retval;
}
#[c2rust::src_loc = "4133:1"]
unsafe extern "C" fn reassemble_pkcs12_name(mut filename: *const libc::c_char)
 -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    if asprintf(&mut ret as *mut *mut libc::c_char,
                b"PKCS12:%s\x00" as *const u8 as *const libc::c_char,
                filename) < 0 as libc::c_int {
        return 0 as *mut libc::c_char
    }
    return ret;
}
#[c2rust::src_loc = "4143:1"]
unsafe extern "C" fn pkinit_get_certs_pkcs12(mut context: krb5_context,
                                             mut plg_cryptoctx:
                                                 pkinit_plg_crypto_context,
                                             mut req_cryptoctx:
                                                 pkinit_req_crypto_context,
                                             mut idopts:
                                                 *mut pkinit_identity_opts,
                                             mut id_cryptoctx:
                                                 pkinit_identity_crypto_context,
                                             mut princ: krb5_principal)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code =
        -(1765328360 as libc::c_long) as krb5_error_code;
    let mut prompt_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: *mut X509 = 0 as *mut X509;
    let mut p12: *mut PKCS12 = 0 as *mut PKCS12;
    let mut ret: libc::c_int = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut y: *mut EVP_PKEY = 0 as *mut EVP_PKEY;
    if (*idopts).cert_filename.is_null() {
        pkiDebug(b"%s: failed to get user\'s cert location\n\x00" as *const u8
                     as *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 24],
                                           &[libc::c_char; 24]>(b"pkinit_get_certs_pkcs12\x00")).as_ptr());
    } else if (*idopts).key_filename.is_null() {
        pkiDebug(b"%s: failed to get user\'s private key location\n\x00" as
                     *const u8 as *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 24],
                                           &[libc::c_char; 24]>(b"pkinit_get_certs_pkcs12\x00")).as_ptr());
    } else {
        fp =
            fopen((*idopts).cert_filename,
                  b"rb\x00" as *const u8 as *const libc::c_char);
        if fp.is_null() {
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"PKINIT failed to open PKCS12 file {str}: err {errno}\x00"
                                  as *const u8 as *const libc::c_char,
                              (*idopts).cert_filename, *__errno_location());
            }
        } else {
            fcntl(fileno(fp), 2 as libc::c_int, 1 as libc::c_int);
            p12 = d2i_PKCS12_fp(fp, 0 as *mut *mut PKCS12);
            fclose(fp);
            if p12.is_null() {
                if (*context).trace_callback.is_some() {
                    krb5int_trace(context,
                                  b"PKINIT failed to decode PKCS12 file {str} contents\x00"
                                      as *const u8 as *const libc::c_char,
                                  (*idopts).cert_filename);
                }
            } else {
                /*
     * Try parsing with no pass phrase first.  If that fails,
     * prompt for the pass phrase and try again.
     */
                ret =
                    PKCS12_parse(p12, 0 as *const libc::c_char, &mut y,
                                 &mut x, 0 as *mut *mut stack_st_X509);
                if ret == 0 as libc::c_int {
                    let mut rdat: krb5_data =
                        krb5_data{magic: 0,
                                  length: 0,
                                  data:
                                      0 as *const libc::c_char as
                                          *mut libc::c_char,};
                    let mut kprompt: krb5_prompt =
                        krb5_prompt{prompt: 0 as *mut libc::c_char,
                                    hidden: 0,
                                    reply: 0 as *mut krb5_data,};
                    let mut prompt_type: krb5_prompt_type = 0;
                    let mut r: krb5_error_code = 0;
                    let mut prompt_reply: [libc::c_char; 128] = [0; 128];
                    let mut prompt_prefix: *mut libc::c_char =
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"Pass phrase for\x00" as *const u8 as
                                     *const libc::c_char);
                    let mut p12name: *mut libc::c_char =
                        reassemble_pkcs12_name((*idopts).cert_filename);
                    let mut tmp: *const libc::c_char =
                        0 as *const libc::c_char;
                    if (*context).trace_callback.is_some() {
                        krb5int_trace(context,
                                      b"PKINIT initial PKCS12_parse with no password failed\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                    }
                    if (*id_cryptoctx).defer_id_prompt != 0 {
                        /* Supply the identity name to be passed to the responder. */
                        pkinit_set_deferred_id(&mut (*id_cryptoctx).deferred_ids,
                                               p12name,
                                               0 as libc::c_int as
                                                   libc::c_ulong,
                                               0 as *const libc::c_char);
                        free(p12name as *mut libc::c_void);
                        retval = 0 as libc::c_int;
                        current_block = 15340138857687241975;
                    } else {
                        /* Try to read a responder-supplied password. */
                        tmp =
                            pkinit_find_deferred_id((*id_cryptoctx).deferred_ids,
                                                    p12name);
                        free(p12name as *mut libc::c_void);
                        if !tmp.is_null() {
                            /* Try using the responder-supplied password. */
                            rdat.data = tmp as *mut libc::c_char;
                            rdat.length = strlen(tmp) as libc::c_uint;
                            current_block = 4888910987971495881;
                        } else if (*id_cryptoctx).prompter.is_none() {
                            current_block = 15340138857687241975;
                        } else {
                            /* Ask using a prompter. */
                            memset(prompt_reply.as_mut_ptr() as
                                       *mut libc::c_void, '\u{0}' as i32,
                                   ::std::mem::size_of::<[libc::c_char; 128]>()
                                       as libc::c_ulong);
                            rdat.data = prompt_reply.as_mut_ptr();
                            rdat.length =
                                ::std::mem::size_of::<[libc::c_char; 128]>()
                                    as libc::c_ulong as libc::c_uint;
                            if asprintf(&mut prompt_string as
                                            *mut *mut libc::c_char,
                                        b"%s %s\x00" as *const u8 as
                                            *const libc::c_char,
                                        prompt_prefix,
                                        (*idopts).cert_filename) <
                                   0 as libc::c_int {
                                prompt_string = 0 as *mut libc::c_char;
                                current_block = 15340138857687241975;
                            } else {
                                kprompt.prompt = prompt_string;
                                kprompt.hidden = 1 as libc::c_int;
                                kprompt.reply = &mut rdat;
                                prompt_type = 0x4 as libc::c_int;
                                /* PROMPTER_INVOCATION */
                                k5int_set_prompt_types.expect("non-null function pointer")(context,
                                                                                           &mut prompt_type);
                                r =
                                    Some((*id_cryptoctx).prompter.expect("non-null function pointer")).expect("non-null function pointer")(context,
                                                                                                                                           (*id_cryptoctx).prompter_data,
                                                                                                                                           0
                                                                                                                                               as
                                                                                                                                               *const libc::c_char,
                                                                                                                                           0
                                                                                                                                               as
                                                                                                                                               *const libc::c_char,
                                                                                                                                           1
                                                                                                                                               as
                                                                                                                                               libc::c_int,
                                                                                                                                           &mut kprompt);
                                k5int_set_prompt_types.expect("non-null function pointer")(context,
                                                                                           0
                                                                                               as
                                                                                               *mut krb5_prompt_type);
                                if r != 0 {
                                    if (*context).trace_callback.is_some() {
                                        krb5int_trace(context,
                                                      b"PKINIT failed to prompt for PKCS12 password\x00"
                                                          as *const u8 as
                                                          *const libc::c_char);
                                    }
                                    current_block = 15340138857687241975;
                                } else {
                                    current_block = 4888910987971495881;
                                }
                            }
                        }
                        match current_block {
                            15340138857687241975 => { }
                            _ => {
                                ret =
                                    PKCS12_parse(p12, rdat.data, &mut y,
                                                 &mut x,
                                                 0 as
                                                     *mut *mut stack_st_X509);
                                if ret == 0 as libc::c_int {
                                    if (*context).trace_callback.is_some() {
                                        krb5int_trace(context,
                                                      b"PKINIT second PKCS12_parse with password failed\x00"
                                                          as *const u8 as
                                                          *const libc::c_char);
                                    }
                                    current_block = 15340138857687241975;
                                } else { current_block = 129780949503461575; }
                            }
                        }
                    }
                } else { current_block = 129780949503461575; }
                match current_block {
                    15340138857687241975 => { }
                    _ => {
                        (*id_cryptoctx).creds[0 as libc::c_int as usize] =
                            malloc(::std::mem::size_of::<_pkinit_cred_info>()
                                       as libc::c_ulong) as pkinit_cred_info;
                        if !(*id_cryptoctx).creds[0 as libc::c_int as
                                                      usize].is_null() {
                            (*(*id_cryptoctx).creds[0 as libc::c_int as
                                                        usize]).name =
                                reassemble_pkcs12_name((*idopts).cert_filename);
                            (*(*id_cryptoctx).creds[0 as libc::c_int as
                                                        usize]).cert = x;
                            (*(*id_cryptoctx).creds[0 as libc::c_int as
                                                        usize]).cert_id =
                                0 as CK_BYTE_PTR;
                            (*(*id_cryptoctx).creds[0 as libc::c_int as
                                                        usize]).cert_id_len =
                                0 as libc::c_int;
                            (*(*id_cryptoctx).creds[0 as libc::c_int as
                                                        usize]).key = y;
                            (*id_cryptoctx).creds[1 as libc::c_int as usize] =
                                0 as pkinit_cred_info;
                            retval = 0 as libc::c_int
                        }
                    }
                }
            }
        }
    }
    /* We can't use a prompter. */
    free(prompt_string as *mut libc::c_void);
    if !p12.is_null() { PKCS12_free(p12); }
    if retval != 0 {
        if !x.is_null() { X509_free(x); }
        if !y.is_null() { EVP_PKEY_free(y); }
    }
    return retval;
}
#[c2rust::src_loc = "4277:1"]
unsafe extern "C" fn reassemble_files_name(mut certfile: *const libc::c_char,
                                           mut keyfile: *const libc::c_char)
 -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    if !keyfile.is_null() {
        if asprintf(&mut ret as *mut *mut libc::c_char,
                    b"FILE:%s,%s\x00" as *const u8 as *const libc::c_char,
                    certfile, keyfile) < 0 as libc::c_int {
            return 0 as *mut libc::c_char
        }
    } else if asprintf(&mut ret as *mut *mut libc::c_char,
                       b"FILE:%s\x00" as *const u8 as *const libc::c_char,
                       certfile) < 0 as libc::c_int {
        return 0 as *mut libc::c_char
    }
    return ret;
}
#[c2rust::src_loc = "4292:1"]
unsafe extern "C" fn pkinit_load_fs_cert_and_key(mut context: krb5_context,
                                                 mut id_cryptoctx:
                                                     pkinit_identity_crypto_context,
                                                 mut certname:
                                                     *mut libc::c_char,
                                                 mut keyname:
                                                     *mut libc::c_char,
                                                 mut cindex: libc::c_int)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut x: *mut X509 = 0 as *mut X509;
    let mut y: *mut EVP_PKEY = 0 as *mut EVP_PKEY;
    let mut fsname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut password: *const libc::c_char = 0 as *const libc::c_char;
    fsname = reassemble_files_name(certname, keyname);
    /* Try to read a responder-supplied password. */
    password = pkinit_find_deferred_id((*id_cryptoctx).deferred_ids, fsname);
    /* Load the certificate. */
    retval = get_cert(certname, &mut x);
    if retval != 0 as libc::c_int || x.is_null() {
        retval =
            oerr(context, 0 as libc::c_int,
                 dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                          b"Cannot read certificate file \'%s\'\x00" as
                              *const u8 as *const libc::c_char), certname)
    } else {
        /* Load the key. */
        retval =
            get_key(context, id_cryptoctx, keyname, fsname, &mut y, password);
        if retval != 0 as libc::c_int || y.is_null() {
            retval =
                oerr(context, 0 as libc::c_int,
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"Cannot read key file \'%s\'\x00" as *const u8
                                  as *const libc::c_char), fsname)
        } else {
            (*id_cryptoctx).creds[cindex as usize] =
                malloc(::std::mem::size_of::<_pkinit_cred_info>() as
                           libc::c_ulong) as pkinit_cred_info;
            if (*id_cryptoctx).creds[cindex as usize].is_null() {
                retval = 12 as libc::c_int
            } else {
                (*(*id_cryptoctx).creds[cindex as usize]).name =
                    reassemble_files_name(certname, keyname);
                (*(*id_cryptoctx).creds[cindex as usize]).cert = x;
                (*(*id_cryptoctx).creds[cindex as usize]).cert_id =
                    0 as CK_BYTE_PTR;
                (*(*id_cryptoctx).creds[cindex as usize]).cert_id_len =
                    0 as libc::c_int;
                (*(*id_cryptoctx).creds[cindex as usize]).key = y;
                (*id_cryptoctx).creds[(cindex + 1 as libc::c_int) as usize] =
                    0 as pkinit_cred_info;
                retval = 0 as libc::c_int
            }
        }
    }
    free(fsname as *mut libc::c_void);
    if retval != 0 as libc::c_int || y.is_null() {
        if !x.is_null() { X509_free(x); }
        if !y.is_null() { EVP_PKEY_free(y); }
    }
    return retval;
}
#[c2rust::src_loc = "4352:1"]
unsafe extern "C" fn pkinit_get_certs_fs(mut context: krb5_context,
                                         mut plg_cryptoctx:
                                             pkinit_plg_crypto_context,
                                         mut req_cryptoctx:
                                             pkinit_req_crypto_context,
                                         mut idopts:
                                             *mut pkinit_identity_opts,
                                         mut id_cryptoctx:
                                             pkinit_identity_crypto_context,
                                         mut princ: krb5_principal)
 -> krb5_error_code {
    let mut retval: krb5_error_code =
        -(1765328360 as libc::c_long) as krb5_error_code;
    if (*idopts).cert_filename.is_null() {
        pkiDebug(b"%s: failed to get user\'s cert location\n\x00" as *const u8
                     as *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 20],
                                           &[libc::c_char; 20]>(b"pkinit_get_certs_fs\x00")).as_ptr());
    } else if (*idopts).key_filename.is_null() {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"PKINIT no private key provided\x00" as *const u8
                              as *const libc::c_char);
        }
    } else {
        retval =
            pkinit_load_fs_cert_and_key(context, id_cryptoctx,
                                        (*idopts).cert_filename,
                                        (*idopts).key_filename,
                                        0 as libc::c_int)
    }
    return retval;
}
#[c2rust::src_loc = "4379:1"]
unsafe extern "C" fn pkinit_get_certs_dir(mut context: krb5_context,
                                          mut plg_cryptoctx:
                                              pkinit_plg_crypto_context,
                                          mut req_cryptoctx:
                                              pkinit_req_crypto_context,
                                          mut idopts:
                                              *mut pkinit_identity_opts,
                                          mut id_cryptoctx:
                                              pkinit_identity_crypto_context,
                                          mut princ: krb5_principal)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut d: *mut DIR = 0 as *mut DIR;
    let mut dentry: *mut dirent = 0 as *mut dirent;
    let mut certname: [libc::c_char; 1024] = [0; 1024];
    let mut keyname: [libc::c_char; 1024] = [0; 1024];
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0;
    let mut dirname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut suf: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*idopts).cert_filename.is_null() {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"PKINIT no certificate provided\x00" as *const u8
                              as *const libc::c_char);
        }
        return 2 as libc::c_int
    }
    dirname = (*idopts).cert_filename;
    d = opendir(dirname);
    if d.is_null() { return *__errno_location() }
    /*
     * We'll assume that certs are named XXX.crt and the corresponding
     * key is named XXX.key
     */
    while i < 20 as libc::c_int && { dentry = readdir(d); !dentry.is_null() }
          {
        /* Ignore subdirectories and anything starting with a dot */
        if (*dentry).d_type as libc::c_int == DT_DIR as libc::c_int {
            continue ;
        }
        if (*dentry).d_name[0 as libc::c_int as usize] as libc::c_int ==
               '.' as i32 {
            continue ;
        }
        len = strlen((*dentry).d_name.as_mut_ptr()) as libc::c_int;
        if len < 5 as libc::c_int { continue ; }
        suf =
            (*dentry).d_name.as_mut_ptr().offset((len - 4 as libc::c_int) as
                                                     isize);
        if strncmp(suf, b".crt\x00" as *const u8 as *const libc::c_char,
                   4 as libc::c_int as libc::c_ulong) != 0 as libc::c_int {
            continue ;
        }
        /* Checked length */
        if strlen(dirname).wrapping_add(strlen((*dentry).d_name.as_mut_ptr())).wrapping_add(2
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_ulong)
               >
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
           {
            pkiDebug(b"%s: Path too long -- directory \'%s\' and file \'%s\'\n\x00"
                         as *const u8 as *const libc::c_char,
                     (*::std::mem::transmute::<&[u8; 21],
                                               &[libc::c_char; 21]>(b"pkinit_get_certs_dir\x00")).as_ptr(),
                     dirname, (*dentry).d_name.as_mut_ptr());
        } else {
            snprintf(certname.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 1024]>() as
                         libc::c_ulong,
                     b"%s/%s\x00" as *const u8 as *const libc::c_char,
                     dirname, (*dentry).d_name.as_mut_ptr());
            snprintf(keyname.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 1024]>() as
                         libc::c_ulong,
                     b"%s/%s\x00" as *const u8 as *const libc::c_char,
                     dirname, (*dentry).d_name.as_mut_ptr());
            len = strlen(keyname.as_mut_ptr()) as libc::c_int;
            keyname[(len - 3 as libc::c_int) as usize] =
                'k' as i32 as libc::c_char;
            keyname[(len - 2 as libc::c_int) as usize] =
                'e' as i32 as libc::c_char;
            keyname[(len - 1 as libc::c_int) as usize] =
                'y' as i32 as libc::c_char;
            retval =
                pkinit_load_fs_cert_and_key(context, id_cryptoctx,
                                            certname.as_mut_ptr(),
                                            keyname.as_mut_ptr(), i);
            if !(retval == 0 as libc::c_int) { continue ; }
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"PKINIT loaded cert and key for {str}\x00" as
                                  *const u8 as *const libc::c_char,
                              (*dentry).d_name.as_mut_ptr());
            }
            i += 1
        }
    }
    if (*id_cryptoctx).defer_id_prompt == 0 && i == 0 as libc::c_int {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"PKINIT no cert and key pair found in directory {str}\x00"
                              as *const u8 as *const libc::c_char,
                          (*idopts).cert_filename);
        }
        retval = 2 as libc::c_int
    } else { retval = 0 as libc::c_int }
    if !d.is_null() { closedir(d); }
    return retval;
}
#[c2rust::src_loc = "4463:1"]
unsafe extern "C" fn reassemble_pkcs11_name(mut idopts:
                                                *mut pkinit_identity_opts)
 -> *mut libc::c_char {
    let mut buf: k5buf =
        k5buf{buftype: K5BUF_ERROR,
              data: 0 as *mut libc::c_void,
              space: 0,
              len: 0,};
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    k5_buf_init_dynamic(&mut buf);
    k5_buf_add(&mut buf, b"PKCS11:\x00" as *const u8 as *const libc::c_char);
    n = 0 as libc::c_int;
    if !(*idopts).p11_module_name.is_null() {
        let fresh23 = n;
        n = n + 1;
        k5_buf_add_fmt(&mut buf as *mut k5buf,
                       b"%smodule_name=%s\x00" as *const u8 as
                           *const libc::c_char,
                       if fresh23 != 0 {
                           b":\x00" as *const u8 as *const libc::c_char
                       } else { b"\x00" as *const u8 as *const libc::c_char },
                       (*idopts).p11_module_name);
    }
    if !(*idopts).token_label.is_null() {
        let fresh24 = n;
        n = n + 1;
        k5_buf_add_fmt(&mut buf as *mut k5buf,
                       b"%stoken=%s\x00" as *const u8 as *const libc::c_char,
                       if fresh24 != 0 {
                           b":\x00" as *const u8 as *const libc::c_char
                       } else { b"\x00" as *const u8 as *const libc::c_char },
                       (*idopts).token_label);
    }
    if !(*idopts).cert_label.is_null() {
        let fresh25 = n;
        n = n + 1;
        k5_buf_add_fmt(&mut buf as *mut k5buf,
                       b"%scertlabel=%s\x00" as *const u8 as
                           *const libc::c_char,
                       if fresh25 != 0 {
                           b":\x00" as *const u8 as *const libc::c_char
                       } else { b"\x00" as *const u8 as *const libc::c_char },
                       (*idopts).cert_label);
    }
    if !(*idopts).cert_id_string.is_null() {
        let fresh26 = n;
        n = n + 1;
        k5_buf_add_fmt(&mut buf as *mut k5buf,
                       b"%scertid=%s\x00" as *const u8 as *const libc::c_char,
                       if fresh26 != 0 {
                           b":\x00" as *const u8 as *const libc::c_char
                       } else { b"\x00" as *const u8 as *const libc::c_char },
                       (*idopts).cert_id_string);
    }
    if (*idopts).slotid != 999999 as libc::c_int as libc::c_ulong {
        let fresh27 = n;
        n = n + 1;
        k5_buf_add_fmt(&mut buf as *mut k5buf,
                       b"%sslotid=%ld\x00" as *const u8 as
                           *const libc::c_char,
                       if fresh27 != 0 {
                           b":\x00" as *const u8 as *const libc::c_char
                       } else { b"\x00" as *const u8 as *const libc::c_char },
                       (*idopts).slotid as libc::c_long);
    }
    if k5_buf_status(&mut buf) == 0 as libc::c_int {
        ret = strdup(buf.data as *const libc::c_char)
    } else { ret = 0 as *mut libc::c_char }
    k5_buf_free(&mut buf);
    return ret;
}
#[c2rust::src_loc = "4501:1"]
unsafe extern "C" fn pkinit_get_certs_pkcs11(mut context: krb5_context,
                                             mut plg_cryptoctx:
                                                 pkinit_plg_crypto_context,
                                             mut req_cryptoctx:
                                                 pkinit_req_crypto_context,
                                             mut idopts:
                                                 *mut pkinit_identity_opts,
                                             mut id_cryptoctx:
                                                 pkinit_identity_crypto_context,
                                             mut princ: krb5_principal)
 -> krb5_error_code {
    let mut cls: CK_OBJECT_CLASS = 0;
    let mut obj: CK_OBJECT_HANDLE = 0;
    let mut attrs: [CK_ATTRIBUTE; 4] =
        [CK_ATTRIBUTE{type_0: 0,
                      pValue: 0 as *mut libc::c_void,
                      ulValueLen: 0,}; 4];
    let mut count: CK_ULONG = 0;
    let mut certtype: CK_CERTIFICATE_TYPE = 0;
    let mut cert: CK_BYTE_PTR = 0 as CK_BYTE_PTR;
    let mut cert_id: CK_BYTE_PTR = 0 as *mut CK_BYTE;
    let mut cp: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut nattrs: libc::c_uint = 0;
    let mut x: *mut X509 = 0 as *mut X509;
    /* Copy stuff from idopts -> id_cryptoctx */
    if !(*idopts).p11_module_name.is_null() {
        free((*id_cryptoctx).p11_module_name as *mut libc::c_void);
        (*id_cryptoctx).p11_module_name = strdup((*idopts).p11_module_name);
        if (*id_cryptoctx).p11_module_name.is_null() {
            return 12 as libc::c_int
        }
    }
    if !(*idopts).token_label.is_null() {
        (*id_cryptoctx).token_label = strdup((*idopts).token_label);
        if (*id_cryptoctx).token_label.is_null() { return 12 as libc::c_int }
    }
    if !(*idopts).cert_label.is_null() {
        (*id_cryptoctx).cert_label = strdup((*idopts).cert_label);
        if (*id_cryptoctx).cert_label.is_null() { return 12 as libc::c_int }
    }
    /* Convert the ascii cert_id string into a binary blob */
    if !(*idopts).cert_id_string.is_null() {
        r =
            k5_hex_decode((*idopts).cert_id_string,
                          &mut (*id_cryptoctx).cert_id,
                          &mut (*id_cryptoctx).cert_id_len);
        if r != 0 as libc::c_int {
            pkiDebug(b"Failed to convert certid string [%s]\n\x00" as
                         *const u8 as *const libc::c_char,
                     (*idopts).cert_id_string);
            return r
        }
    }
    (*id_cryptoctx).slotid = (*idopts).slotid;
    (*id_cryptoctx).pkcs11_method = 1 as libc::c_int;
    if pkinit_open_session(context, id_cryptoctx) != 0 {
        pkiDebug(b"can\'t open pkcs11 session\n\x00" as *const u8 as
                     *const libc::c_char);
        if (*id_cryptoctx).defer_id_prompt == 0 {
            return -(1765328360 as libc::c_long) as krb5_error_code
        }
    }
    if (*id_cryptoctx).defer_id_prompt != 0 {
        /*
         * We need to reset all of the PKCS#11 state, so that the next time we
         * poke at it, it'll be in as close to the state it was in after we
         * loaded it the first time as we can make it.
         */
        pkinit_fini_pkcs11(id_cryptoctx);
        pkinit_init_pkcs11(id_cryptoctx);
        return 0 as libc::c_int
    }
    /*
     * We'd like to use CKM_SHA1_RSA_PKCS for signing if it's available, but
     * many cards seems to be confused about whether they are capable of
     * this or not. The safe thing seems to be to ignore the mechanism list,
     * always use CKM_RSA_PKCS and calculate the sha1 digest ourselves.
     */
    (*id_cryptoctx).mech = 1 as libc::c_int as CK_MECHANISM_TYPE;
    cls = 1 as libc::c_int as CK_OBJECT_CLASS;
    attrs[0 as libc::c_int as usize].type_0 =
        0 as libc::c_int as CK_ATTRIBUTE_TYPE;
    attrs[0 as libc::c_int as usize].pValue =
        &mut cls as *mut CK_OBJECT_CLASS as *mut libc::c_void;
    attrs[0 as libc::c_int as usize].ulValueLen =
        ::std::mem::size_of::<CK_OBJECT_CLASS>() as libc::c_ulong;
    certtype = 0 as libc::c_int as CK_CERTIFICATE_TYPE;
    attrs[1 as libc::c_int as usize].type_0 =
        0x80 as libc::c_int as CK_ATTRIBUTE_TYPE;
    attrs[1 as libc::c_int as usize].pValue =
        &mut certtype as *mut CK_CERTIFICATE_TYPE as *mut libc::c_void;
    attrs[1 as libc::c_int as usize].ulValueLen =
        ::std::mem::size_of::<CK_CERTIFICATE_TYPE>() as libc::c_ulong;
    nattrs = 2 as libc::c_int as libc::c_uint;
    /* If a cert id and/or label were given, use them too */
    if (*id_cryptoctx).cert_id_len > 0 as libc::c_int as libc::c_ulong {
        attrs[nattrs as usize].type_0 =
            0x102 as libc::c_int as CK_ATTRIBUTE_TYPE;
        attrs[nattrs as usize].pValue =
            (*id_cryptoctx).cert_id as *mut libc::c_void;
        attrs[nattrs as usize].ulValueLen = (*id_cryptoctx).cert_id_len;
        nattrs = nattrs.wrapping_add(1)
    }
    if !(*id_cryptoctx).cert_label.is_null() {
        attrs[nattrs as usize].type_0 = 3 as libc::c_int as CK_ATTRIBUTE_TYPE;
        attrs[nattrs as usize].pValue =
            (*id_cryptoctx).cert_label as *mut libc::c_void;
        attrs[nattrs as usize].ulValueLen =
            strlen((*id_cryptoctx).cert_label);
        nattrs = nattrs.wrapping_add(1)
    }
    r =
        (*(*id_cryptoctx).p11).C_FindObjectsInit.expect("non-null function pointer")((*id_cryptoctx).session,
                                                                                     attrs.as_mut_ptr(),
                                                                                     nattrs
                                                                                         as
                                                                                         libc::c_ulong)
            as libc::c_int;
    if r != 0 as libc::c_int {
        pkiDebug(b"C_FindObjectsInit: %s\n\x00" as *const u8 as
                     *const libc::c_char, pkinit_pkcs11_code_to_text(r));
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    i = 0 as libc::c_int;
    loop  {
        if i >= 20 as libc::c_int {
            return -(1765328360 as libc::c_long) as krb5_error_code
        }
        /* Look for x.509 cert */
        r =
            (*(*id_cryptoctx).p11).C_FindObjects.expect("non-null function pointer")((*id_cryptoctx).session,
                                                                                     &mut obj,
                                                                                     1
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong,
                                                                                     &mut count)
                as libc::c_int;
        if r != 0 as libc::c_int || count <= 0 as libc::c_int as libc::c_ulong
           {
            (*id_cryptoctx).creds[i as usize] = 0 as pkinit_cred_info;
            break ;
        } else {
            /* Get cert and id len */
            attrs[0 as libc::c_int as usize].type_0 =
                0x11 as libc::c_int as CK_ATTRIBUTE_TYPE;
            attrs[0 as libc::c_int as usize].pValue = 0 as *mut libc::c_void;
            attrs[0 as libc::c_int as usize].ulValueLen =
                0 as libc::c_int as libc::c_ulong;
            attrs[1 as libc::c_int as usize].type_0 =
                0x102 as libc::c_int as CK_ATTRIBUTE_TYPE;
            attrs[1 as libc::c_int as usize].pValue = 0 as *mut libc::c_void;
            attrs[1 as libc::c_int as usize].ulValueLen =
                0 as libc::c_int as libc::c_ulong;
            r =
                (*(*id_cryptoctx).p11).C_GetAttributeValue.expect("non-null function pointer")((*id_cryptoctx).session,
                                                                                               obj,
                                                                                               attrs.as_mut_ptr(),
                                                                                               2
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulong)
                    as libc::c_int;
            if r != 0 as libc::c_int && r != 0x150 as libc::c_int {
                pkiDebug(b"C_GetAttributeValue: %s\n\x00" as *const u8 as
                             *const libc::c_char,
                         pkinit_pkcs11_code_to_text(r));
                return -(1765328360 as libc::c_long) as krb5_error_code
            }
            cert =
                malloc(attrs[0 as libc::c_int as
                                 usize].ulValueLen.wrapping_add(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong))
                    as CK_BYTE_PTR;
            cert_id =
                malloc(attrs[1 as libc::c_int as
                                 usize].ulValueLen.wrapping_add(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong))
                    as CK_BYTE_PTR;
            if cert.is_null() || cert_id.is_null() {
                return 12 as libc::c_int
            }
            /* Read the cert and id off the card */
            attrs[0 as libc::c_int as usize].type_0 =
                0x11 as libc::c_int as CK_ATTRIBUTE_TYPE;
            attrs[0 as libc::c_int as usize].pValue =
                cert as *mut libc::c_void;
            attrs[1 as libc::c_int as usize].type_0 =
                0x102 as libc::c_int as CK_ATTRIBUTE_TYPE;
            attrs[1 as libc::c_int as usize].pValue =
                cert_id as *mut libc::c_void;
            r =
                (*(*id_cryptoctx).p11).C_GetAttributeValue.expect("non-null function pointer")((*id_cryptoctx).session,
                                                                                               obj,
                                                                                               attrs.as_mut_ptr(),
                                                                                               2
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulong)
                    as libc::c_int;
            if r != 0 as libc::c_int {
                pkiDebug(b"C_GetAttributeValue: %s\n\x00" as *const u8 as
                             *const libc::c_char,
                         pkinit_pkcs11_code_to_text(r));
                return -(1765328360 as libc::c_long) as krb5_error_code
            }
            pkiDebug(b"cert %d size %d id %d idlen %d\n\x00" as *const u8 as
                         *const libc::c_char, i,
                     attrs[0 as libc::c_int as usize].ulValueLen as
                         libc::c_int,
                     *cert_id.offset(0 as libc::c_int as isize) as
                         libc::c_int,
                     attrs[1 as libc::c_int as usize].ulValueLen as
                         libc::c_int);
            cp = cert as *mut libc::c_uchar;
            x =
                d2i_X509(0 as *mut *mut X509, &mut cp,
                         attrs[0 as libc::c_int as usize].ulValueLen as
                             libc::c_int as libc::c_long);
            if x.is_null() {
                return -(1765328360 as libc::c_long) as krb5_error_code
            }
            (*id_cryptoctx).creds[i as usize] =
                malloc(::std::mem::size_of::<_pkinit_cred_info>() as
                           libc::c_ulong) as pkinit_cred_info;
            if (*id_cryptoctx).creds[i as usize].is_null() {
                return -(1765328360 as libc::c_long) as krb5_error_code
            }
            (*(*id_cryptoctx).creds[i as usize]).name =
                reassemble_pkcs11_name(idopts);
            (*(*id_cryptoctx).creds[i as usize]).cert = x;
            (*(*id_cryptoctx).creds[i as usize]).key = 0 as *mut EVP_PKEY;
            (*(*id_cryptoctx).creds[i as usize]).cert_id = cert_id;
            (*(*id_cryptoctx).creds[i as usize]).cert_id_len =
                attrs[1 as libc::c_int as usize].ulValueLen as libc::c_int;
            free(cert as *mut libc::c_void);
            i += 1
        }
    }
    (*(*id_cryptoctx).p11).C_FindObjectsFinal.expect("non-null function pointer")((*id_cryptoctx).session);
    if cert.is_null() {
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "4713:1"]
unsafe extern "C" fn free_cred_info(mut context: krb5_context,
                                    mut id_cryptoctx:
                                        pkinit_identity_crypto_context,
                                    mut cred: *mut _pkinit_cred_info) {
    if !cred.is_null() {
        if !(*cred).cert.is_null() { X509_free((*cred).cert); }
        if !(*cred).key.is_null() { EVP_PKEY_free((*cred).key); }
        free((*cred).cert_id as *mut libc::c_void);
        free((*cred).name as *mut libc::c_void);
        free(cred as *mut libc::c_void);
    };
}
#[no_mangle]
#[c2rust::src_loc = "4731:1"]
pub unsafe extern "C" fn crypto_free_cert_info(mut context: krb5_context,
                                               mut plg_cryptoctx:
                                                   pkinit_plg_crypto_context,
                                               mut req_cryptoctx:
                                                   pkinit_req_crypto_context,
                                               mut id_cryptoctx:
                                                   pkinit_identity_crypto_context)
 -> krb5_error_code {
    let mut i: libc::c_int = 0;
    if id_cryptoctx.is_null() { return 22 as libc::c_int }
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        if !(*id_cryptoctx).creds[i as usize].is_null() {
            free_cred_info(context, id_cryptoctx,
                           (*id_cryptoctx).creds[i as usize]);
            (*id_cryptoctx).creds[i as usize] = 0 as pkinit_cred_info
        }
        i += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "4751:1"]
pub unsafe extern "C" fn crypto_load_certs(mut context: krb5_context,
                                           mut plg_cryptoctx:
                                               pkinit_plg_crypto_context,
                                           mut req_cryptoctx:
                                               pkinit_req_crypto_context,
                                           mut idopts:
                                               *mut pkinit_identity_opts,
                                           mut id_cryptoctx:
                                               pkinit_identity_crypto_context,
                                           mut princ: krb5_principal,
                                           mut defer_id_prompts: krb5_boolean)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    (*id_cryptoctx).defer_id_prompt = defer_id_prompts;
    match (*idopts).idtype {
        1 => {
            retval =
                pkinit_get_certs_fs(context, plg_cryptoctx, req_cryptoctx,
                                    idopts, id_cryptoctx, princ)
        }
        2 => {
            retval =
                pkinit_get_certs_dir(context, plg_cryptoctx, req_cryptoctx,
                                     idopts, id_cryptoctx, princ)
        }
        3 => {
            retval =
                pkinit_get_certs_pkcs11(context, plg_cryptoctx, req_cryptoctx,
                                        idopts, id_cryptoctx, princ)
        }
        5 => {
            retval =
                pkinit_get_certs_pkcs12(context, plg_cryptoctx, req_cryptoctx,
                                        idopts, id_cryptoctx, princ)
        }
        _ => { retval = 22 as libc::c_int }
    }
    (retval) != 0;
    return retval;
}
/*
 * Get certificate Key Usage and Extended Key Usage
 */
#[c2rust::src_loc = "4800:1"]
unsafe extern "C" fn crypto_retrieve_X509_key_usage(mut context: krb5_context,
                                                    mut plgcctx:
                                                        pkinit_plg_crypto_context,
                                                    mut reqcctx:
                                                        pkinit_req_crypto_context,
                                                    mut x: *mut X509,
                                                    mut ret_ku_bits:
                                                        *mut libc::c_uint,
                                                    mut ret_eku_bits:
                                                        *mut libc::c_uint)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut eku_bits: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut ku_bits: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut usage: *mut ASN1_BIT_STRING = 0 as *mut ASN1_BIT_STRING;
    if ret_ku_bits.is_null() && ret_eku_bits.is_null() {
        return 22 as libc::c_int
    }
    if !ret_eku_bits.is_null() {
        *ret_eku_bits = 0 as libc::c_int as libc::c_uint;
        /* Start with Extended Key usage */
        i = X509_get_ext_by_NID(x, 126 as libc::c_int, -(1 as libc::c_int));
        if i >= 0 as libc::c_int {
            let mut eku: *mut EXTENDED_KEY_USAGE =
                0 as *mut EXTENDED_KEY_USAGE;
            eku =
                X509_get_ext_d2i(x, 126 as libc::c_int, 0 as *mut libc::c_int,
                                 0 as *mut libc::c_int) as
                    *mut EXTENDED_KEY_USAGE;
            if !eku.is_null() {
                i = 0 as libc::c_int;
                while i < sk_ASN1_OBJECT_num(eku) {
                    let mut certoid: *mut ASN1_OBJECT = 0 as *mut ASN1_OBJECT;
                    certoid = sk_ASN1_OBJECT_value(eku, i);
                    if OBJ_cmp(certoid, (*plgcctx).id_pkinit_KPClientAuth) ==
                           0 as libc::c_int {
                        eku_bits |= 0x80000000 as libc::c_uint
                    } else if OBJ_cmp(certoid,
                                      OBJ_nid2obj(648 as libc::c_int)) ==
                                  0 as libc::c_int {
                        eku_bits |= 0x40000000 as libc::c_int as libc::c_uint
                    } else if OBJ_cmp(certoid,
                                      OBJ_nid2obj(130 as libc::c_int)) ==
                                  0 as libc::c_int {
                        eku_bits |= 0x20000000 as libc::c_int as libc::c_uint
                    } else if OBJ_cmp(certoid,
                                      OBJ_nid2obj(132 as libc::c_int)) ==
                                  0 as libc::c_int {
                        eku_bits |= 0x10000000 as libc::c_int as libc::c_uint
                    }
                    i += 1
                }
                EXTENDED_KEY_USAGE_free(eku);
            }
        }
        pkiDebug(b"%s: returning eku 0x%08x\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 31],
                                           &[libc::c_char; 31]>(b"crypto_retrieve_X509_key_usage\x00")).as_ptr(),
                 eku_bits);
        *ret_eku_bits = eku_bits
    } else {
        pkiDebug(b"%s: EKUs not requested, not checking\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 31],
                                           &[libc::c_char; 31]>(b"crypto_retrieve_X509_key_usage\x00")).as_ptr());
    }
    /* Now the Key Usage bits */
    if !ret_ku_bits.is_null() {
        *ret_ku_bits = 0 as libc::c_int as libc::c_uint;
        /* Make sure usage exists before checking bits */
        X509_check_ca(x);
        usage =
            X509_get_ext_d2i(x, 83 as libc::c_int, 0 as *mut libc::c_int,
                             0 as *mut libc::c_int) as *mut ASN1_BIT_STRING;
        if !usage.is_null() {
            if X509_get_key_usage(x) & 0x80 as libc::c_int as libc::c_uint !=
                   0 {
                ku_bits |= 0x80000000 as libc::c_uint
            }
            if X509_get_key_usage(x) & 0x20 as libc::c_int as libc::c_uint !=
                   0 {
                ku_bits |= 0x40000000 as libc::c_int as libc::c_uint
            }
            ASN1_BIT_STRING_free(usage);
        }
        pkiDebug(b"%s: returning ku 0x%08x\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 31],
                                           &[libc::c_char; 31]>(b"crypto_retrieve_X509_key_usage\x00")).as_ptr(),
                 ku_bits);
        *ret_ku_bits = ku_bits;
        retval = 0 as libc::c_int
    } else {
        pkiDebug(b"%s: KUs not requested, not checking\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 31],
                                           &[libc::c_char; 31]>(b"crypto_retrieve_X509_key_usage\x00")).as_ptr());
    }
    return retval;
}
#[c2rust::src_loc = "4875:1"]
unsafe extern "C" fn rfc2253_name(mut name: *mut X509_NAME,
                                  mut str_out: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut b: *mut BIO = 0 as *mut BIO;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    *str_out = 0 as *mut libc::c_char;
    b = BIO_new(BIO_s_mem());
    if b.is_null() { return 12 as libc::c_int }
    if !(X509_NAME_print_ex(b, name, 0 as libc::c_int,
                            ((1 as libc::c_int) << 16 as libc::c_int) as
                                libc::c_ulong) < 0 as libc::c_int) {
        str =
            calloc(BIO_number_written(b).wrapping_add(1 as libc::c_int as
                                                          libc::c_ulong),
                   1 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
        if !str.is_null() {
            BIO_read(b, str as *mut libc::c_void,
                     BIO_number_written(b) as libc::c_int);
            BIO_free(b);
            *str_out = str;
            return 0 as libc::c_int
        }
    }
    BIO_free(b);
    return 12 as libc::c_int;
}
/*
 * Get number of certificates available after crypto_load_certs()
 */
#[c2rust::src_loc = "4903:1"]
unsafe extern "C" fn crypto_cert_get_count(mut id_cryptoctx:
                                               pkinit_identity_crypto_context,
                                           mut cert_count: *mut libc::c_int)
 -> krb5_error_code {
    let mut count: libc::c_int = 0;
    *cert_count = 0 as libc::c_int;
    if id_cryptoctx.is_null() ||
           (*id_cryptoctx).creds[0 as libc::c_int as usize].is_null() {
        return 22 as libc::c_int
    }
    count = 0 as libc::c_int;
    while count <= 20 as libc::c_int &&
              !(*id_cryptoctx).creds[count as usize].is_null() {
        count += 1
    }
    *cert_count = count;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "4920:1"]
pub unsafe extern "C" fn crypto_cert_free_matching_data(mut context:
                                                            krb5_context,
                                                        mut md:
                                                            *mut pkinit_cert_matching_data) {
    let mut i: libc::c_int = 0;
    if md.is_null() { return }
    free((*md).subject_dn as *mut libc::c_void);
    free((*md).issuer_dn as *mut libc::c_void);
    i = 0 as libc::c_int;
    while !(*md).sans.is_null() && !(*(*md).sans.offset(i as isize)).is_null()
          {
        krb5_free_principal(context, *(*md).sans.offset(i as isize));
        i += 1
    }
    free((*md).sans as *mut libc::c_void);
    i = 0 as libc::c_int;
    while !(*md).upns.is_null() && !(*(*md).upns.offset(i as isize)).is_null()
          {
        free(*(*md).upns.offset(i as isize) as *mut libc::c_void);
        i += 1
    }
    free((*md).upns as *mut libc::c_void);
    free(md as *mut libc::c_void);
}
/*
 * Free certificate matching data.
 */
#[no_mangle]
#[c2rust::src_loc = "4942:1"]
pub unsafe extern "C" fn crypto_cert_free_matching_data_list(mut context:
                                                                 krb5_context,
                                                             mut list:
                                                                 *mut *mut pkinit_cert_matching_data) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !list.is_null() && !(*list.offset(i as isize)).is_null() {
        crypto_cert_free_matching_data(context, *list.offset(i as isize));
        i += 1
    }
    free(list as *mut libc::c_void);
}
/*
 * Get certificate matching data for cert.
 */
#[c2rust::src_loc = "4956:1"]
unsafe extern "C" fn get_matching_data(mut context: krb5_context,
                                       mut plg_cryptoctx:
                                           pkinit_plg_crypto_context,
                                       mut req_cryptoctx:
                                           pkinit_req_crypto_context,
                                       mut cert: *mut X509,
                                       mut md_out:
                                           *mut *mut pkinit_cert_matching_data)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 12 as libc::c_int;
    let mut md: *mut pkinit_cert_matching_data =
        0 as *mut pkinit_cert_matching_data;
    *md_out = 0 as *mut pkinit_cert_matching_data;
    md =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<pkinit_cert_matching_data>() as
                   libc::c_ulong) as *mut pkinit_cert_matching_data;
    if !md.is_null() {
        ret =
            rfc2253_name(X509_get_subject_name(cert), &mut (*md).subject_dn);
        if !(ret != 0) {
            ret =
                rfc2253_name(X509_get_issuer_name(cert),
                             &mut (*md).issuer_dn);
            if !(ret != 0) {
                /* Get the SAN data. */
                ret =
                    crypto_retrieve_X509_sans(context, plg_cryptoctx,
                                              req_cryptoctx, cert,
                                              &mut (*md).sans,
                                              &mut (*md).upns,
                                              0 as
                                                  *mut *mut *mut libc::c_uchar);
                if !(ret != 0) {
                    /* Get the KU and EKU data. */
                    ret =
                        crypto_retrieve_X509_key_usage(context, plg_cryptoctx,
                                                       req_cryptoctx, cert,
                                                       &mut (*md).ku_bits,
                                                       &mut (*md).eku_bits);
                    if !(ret != 0) {
                        *md_out = md;
                        md = 0 as *mut pkinit_cert_matching_data
                    }
                }
            }
        }
    }
    crypto_cert_free_matching_data(context, md);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "4999:1"]
pub unsafe extern "C" fn crypto_cert_get_matching_data(mut context:
                                                           krb5_context,
                                                       mut plg_cryptoctx:
                                                           pkinit_plg_crypto_context,
                                                       mut req_cryptoctx:
                                                           pkinit_req_crypto_context,
                                                       mut id_cryptoctx:
                                                           pkinit_identity_crypto_context,
                                                       mut md_out:
                                                           *mut *mut *mut pkinit_cert_matching_data)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut md_list: *mut *mut pkinit_cert_matching_data =
        0 as *mut *mut pkinit_cert_matching_data;
    let mut count: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    ret = crypto_cert_get_count(id_cryptoctx, &mut count);
    if !(ret != 0) {
        md_list =
            calloc((count + 1 as libc::c_int) as libc::c_ulong,
                   ::std::mem::size_of::<*mut pkinit_cert_matching_data>() as
                       libc::c_ulong) as *mut *mut pkinit_cert_matching_data;
        if md_list.is_null() {
            ret = 12 as libc::c_int
        } else {
            i = 0 as libc::c_int;
            loop  {
                if !(i < count) {
                    current_block = 13586036798005543211;
                    break ;
                }
                ret =
                    get_matching_data(context, plg_cryptoctx, req_cryptoctx,
                                      (*(*id_cryptoctx).creds[i as
                                                                  usize]).cert,
                                      &mut *md_list.offset(i as isize));
                if ret != 0 {
                    pkiDebug(b"%s: crypto_cert_get_matching_data error %d, %s\n\x00"
                                 as *const u8 as *const libc::c_char,
                             (*::std::mem::transmute::<&[u8; 30],
                                                       &[libc::c_char; 30]>(b"crypto_cert_get_matching_data\x00")).as_ptr(),
                             ret, error_message(ret as errcode_t));
                    current_block = 3694914606196668789;
                    break ;
                } else { i += 1 }
            }
            match current_block {
                3694914606196668789 => { }
                _ => {
                    *md_out = md_list;
                    md_list = 0 as *mut *mut pkinit_cert_matching_data
                }
            }
        }
    }
    crypto_cert_free_matching_data_list(context, md_list);
    return ret;
}
/*
 * Set the certificate in idctx->creds[cred_index] as the selected certificate.
 */
#[no_mangle]
#[c2rust::src_loc = "5041:1"]
pub unsafe extern "C" fn crypto_cert_select(mut context: krb5_context,
                                            mut idctx:
                                                pkinit_identity_crypto_context,
                                            mut cred_index: size_t)
 -> krb5_error_code {
    let mut ci: pkinit_cred_info = 0 as pkinit_cred_info;
    if cred_index >= 20 as libc::c_int as libc::c_ulong ||
           (*idctx).creds[cred_index as usize].is_null() {
        return 2 as libc::c_int
    }
    ci = (*idctx).creds[cred_index as usize];
    /* copy the selected cert into our id_cryptoctx */
    if !(*idctx).my_certs.is_null() {
        sk_X509_pop_free((*idctx).my_certs,
                         Some(X509_free as
                                  unsafe extern "C" fn(_: *mut X509) -> ()));
    }
    (*idctx).my_certs = sk_X509_new_null();
    sk_X509_push((*idctx).my_certs, (*ci).cert);
    free((*idctx).identity as *mut libc::c_void);
    /* hang on to the selected credential name */
    if !(*ci).name.is_null() {
        (*idctx).identity = strdup((*ci).name)
    } else {
        (*idctx).identity = 0 as *mut libc::c_char
    } /* Don't free it twice */
    (*ci).cert = 0 as *mut X509;
    (*idctx).cert_index = 0 as libc::c_int;
    if (*idctx).pkcs11_method != 1 as libc::c_int {
        (*idctx).my_key = (*ci).key;
        (*ci).key = 0 as *mut EVP_PKEY
        /* Don't free it twice */
    } else {
        (*idctx).cert_id = (*ci).cert_id; /* Don't free it twice */
        (*ci).cert_id = 0 as CK_BYTE_PTR;
        (*idctx).cert_id_len = (*ci).cert_id_len as size_t
    }
    return 0 as libc::c_int;
}
/*
 * Choose the default certificate as "the chosen one"
 */
#[no_mangle]
#[c2rust::src_loc = "5082:1"]
pub unsafe extern "C" fn crypto_cert_select_default(mut context: krb5_context,
                                                    mut plg_cryptoctx:
                                                        pkinit_plg_crypto_context,
                                                    mut req_cryptoctx:
                                                        pkinit_req_crypto_context,
                                                    mut id_cryptoctx:
                                                        pkinit_identity_crypto_context)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut cert_count: libc::c_int = 0;
    retval = crypto_cert_get_count(id_cryptoctx, &mut cert_count);
    if !(retval != 0) {
        if cert_count != 1 as libc::c_int {
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"PKINIT error: There are {int} certs, but there must be exactly one.\x00"
                                  as *const u8 as *const libc::c_char,
                              cert_count);
            }
            retval = 22 as libc::c_int
        } else {
            /* copy the selected cert into our id_cryptoctx */
            if !(*id_cryptoctx).my_certs.is_null() {
                sk_X509_pop_free((*id_cryptoctx).my_certs,
                                 Some(X509_free as
                                          unsafe extern "C" fn(_: *mut X509)
                                              ->
                                                  ())); /* Don't free it twice */
            }
            (*id_cryptoctx).my_certs = sk_X509_new_null();
            sk_X509_push((*id_cryptoctx).my_certs,
                         (*(*id_cryptoctx).creds[0 as libc::c_int as
                                                     usize]).cert);
            (*(*id_cryptoctx).creds[0 as libc::c_int as usize]).cert =
                0 as *mut X509;
            (*id_cryptoctx).cert_index = 0 as libc::c_int;
            /* hang on to the selected credential name */
            if !(*(*id_cryptoctx).creds[0 as libc::c_int as
                                            usize]).name.is_null() {
                (*id_cryptoctx).identity =
                    strdup((*(*id_cryptoctx).creds[0 as libc::c_int as
                                                       usize]).name)
            } else { (*id_cryptoctx).identity = 0 as *mut libc::c_char }
            if (*id_cryptoctx).pkcs11_method != 1 as libc::c_int {
                (*id_cryptoctx).my_key =
                    (*(*id_cryptoctx).creds[0 as libc::c_int as usize]).key;
                (*(*id_cryptoctx).creds[0 as libc::c_int as usize]).key =
                    0 as *mut EVP_PKEY
                /* Don't free it twice */
            } else {
                (*id_cryptoctx).cert_id =
                    (*(*id_cryptoctx).creds[0 as libc::c_int as
                                                usize]).cert_id; /* Don't free it twice */
                (*(*id_cryptoctx).creds[0 as libc::c_int as usize]).cert_id =
                    0 as CK_BYTE_PTR;
                (*id_cryptoctx).cert_id_len =
                    (*(*id_cryptoctx).creds[0 as libc::c_int as
                                                usize]).cert_id_len as size_t
            }
            retval = 0 as libc::c_int
        }
    }
    return retval;
}
#[c2rust::src_loc = "5132:1"]
unsafe extern "C" fn load_cas_and_crls(mut context: krb5_context,
                                       mut plg_cryptoctx:
                                           pkinit_plg_crypto_context,
                                       mut req_cryptoctx:
                                           pkinit_req_crypto_context,
                                       mut id_cryptoctx:
                                           pkinit_identity_crypto_context,
                                       mut catype: libc::c_int,
                                       mut filename: *mut libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut sk: *mut stack_st_X509_INFO = 0 as *mut stack_st_X509_INFO;
    let mut ca_certs: *mut stack_st_X509 = 0 as *mut stack_st_X509;
    let mut ca_crls: *mut stack_st_X509_CRL = 0 as *mut stack_st_X509_CRL;
    let mut in_0: *mut BIO = 0 as *mut BIO;
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    /* If there isn't already a stack in the context,
     * create a temporary one now */
    match catype {
        1 => {
            if !(*id_cryptoctx).trustedCAs.is_null() {
                ca_certs = (*id_cryptoctx).trustedCAs
            } else {
                ca_certs = sk_X509_new_null();
                if ca_certs.is_null() { return 12 as libc::c_int }
            }
        }
        2 => {
            if !(*id_cryptoctx).intermediateCAs.is_null() {
                ca_certs = (*id_cryptoctx).intermediateCAs
            } else {
                ca_certs = sk_X509_new_null();
                if ca_certs.is_null() { return 12 as libc::c_int }
            }
        }
        3 => {
            if !(*id_cryptoctx).revoked.is_null() {
                ca_crls = (*id_cryptoctx).revoked
            } else {
                ca_crls = sk_X509_CRL_new_null();
                if ca_crls.is_null() { return 12 as libc::c_int }
            }
        }
        _ => { return 95 as libc::c_int }
    }
    in_0 =
        BIO_new_file(filename, b"r\x00" as *const u8 as *const libc::c_char);
    if in_0.is_null() {
        retval =
            oerr(context, 0 as libc::c_int,
                 dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                          b"Cannot open file \'%s\'\x00" as *const u8 as
                              *const libc::c_char), filename)
    } else {
        /* This loads from a file, a stack of x509/crl/pkey sets */
        sk =
            PEM_X509_INFO_read_bio(in_0, 0 as *mut stack_st_X509_INFO, None,
                                   0 as *mut libc::c_void);
        if sk.is_null() {
            pkiDebug(b"%s: error reading file \'%s\'\n\x00" as *const u8 as
                         *const libc::c_char,
                     (*::std::mem::transmute::<&[u8; 18],
                                               &[libc::c_char; 18]>(b"load_cas_and_crls\x00")).as_ptr(),
                     filename);
            retval =
                oerr(context, 0 as libc::c_int,
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"Cannot read file \'%s\'\x00" as *const u8 as
                                  *const libc::c_char), filename)
        } else {
            /* scan over the stack created from loading the file contents,
     * weed out duplicates, and push new ones onto the return stack
     */
            i = 0 as libc::c_int;
            while i < sk_X509_INFO_num(sk) {
                let mut xi: *mut X509_INFO = sk_X509_INFO_value(sk, i);
                if !xi.is_null() && !(*xi).x509.is_null() &&
                       catype != 3 as libc::c_int {
                    let mut j: libc::c_int = 0 as libc::c_int;
                    let mut size: libc::c_int = sk_X509_num(ca_certs);
                    let mut flag: libc::c_int = 0 as libc::c_int;
                    if size == 0 {
                        sk_X509_push(ca_certs, (*xi).x509);
                        (*xi).x509 = 0 as *mut X509
                    } else {
                        j = 0 as libc::c_int;
                        while j < size {
                            let mut x: *mut X509 = sk_X509_value(ca_certs, j);
                            flag = X509_cmp(x, (*xi).x509);
                            if flag == 0 as libc::c_int { break ; }
                            j += 1
                        }
                        if flag != 0 as libc::c_int {
                            sk_X509_push(ca_certs, X509_dup((*xi).x509));
                        }
                    }
                } else if !xi.is_null() && !(*xi).crl.is_null() &&
                              catype == 3 as libc::c_int {
                    let mut j_0: libc::c_int = 0 as libc::c_int;
                    let mut size_0: libc::c_int = sk_X509_CRL_num(ca_crls);
                    let mut flag_0: libc::c_int = 0 as libc::c_int;
                    if size_0 == 0 {
                        sk_X509_CRL_push(ca_crls, (*xi).crl);
                        (*xi).crl = 0 as *mut X509_CRL
                    } else {
                        j_0 = 0 as libc::c_int;
                        while j_0 < size_0 {
                            let mut x_0: *mut X509_CRL =
                                sk_X509_CRL_value(ca_crls, j_0);
                            flag_0 = X509_CRL_cmp(x_0, (*xi).crl);
                            if flag_0 == 0 as libc::c_int { break ; }
                            j_0 += 1
                        }
                        if flag_0 != 0 as libc::c_int {
                            sk_X509_CRL_push(ca_crls,
                                             X509_CRL_dup((*xi).crl));
                        }
                    }
                }
                i += 1
            }
            /* If we added something and there wasn't a stack in the
     * context before, add the temporary stack to the context.
     */
            match catype {
                1 => {
                    if sk_X509_num(ca_certs) == 0 as libc::c_int {
                        if (*context).trace_callback.is_some() {
                            krb5int_trace(context,
                                          b"PKINIT no anchor CA in file {str}\x00"
                                              as *const u8 as
                                              *const libc::c_char, filename);
                        }
                        if (*id_cryptoctx).trustedCAs.is_null() {
                            sk_X509_free(ca_certs);
                        }
                    } else if (*id_cryptoctx).trustedCAs.is_null() {
                        (*id_cryptoctx).trustedCAs = ca_certs
                    }
                    current_block = 1417769144978639029;
                }
                2 => {
                    if sk_X509_num(ca_certs) == 0 as libc::c_int {
                        if (*context).trace_callback.is_some() {
                            krb5int_trace(context,
                                          b"PKINIT no intermediate CA in file {str}\x00"
                                              as *const u8 as
                                              *const libc::c_char, filename);
                        }
                        if (*id_cryptoctx).intermediateCAs.is_null() {
                            sk_X509_free(ca_certs);
                        }
                    } else if (*id_cryptoctx).intermediateCAs.is_null() {
                        (*id_cryptoctx).intermediateCAs = ca_certs
                    }
                    current_block = 1417769144978639029;
                }
                3 => {
                    if sk_X509_CRL_num(ca_crls) == 0 as libc::c_int {
                        if (*context).trace_callback.is_some() {
                            krb5int_trace(context,
                                          b"PKINIT no CRL in file {str}\x00"
                                              as *const u8 as
                                              *const libc::c_char, filename);
                        }
                        if (*id_cryptoctx).revoked.is_null() {
                            sk_X509_CRL_free(ca_crls);
                        }
                    } else if (*id_cryptoctx).revoked.is_null() {
                        (*id_cryptoctx).revoked = ca_crls
                    }
                    current_block = 1417769144978639029;
                }
                _ => {
                    /* Should have been caught above! */
                    retval = 22 as libc::c_int;
                    current_block = 8497798769386294116;
                }
            }
            match current_block {
                8497798769386294116 => { }
                _ => { retval = 0 as libc::c_int }
            }
        }
    }
    if !in_0.is_null() { BIO_free(in_0); }
    if !sk.is_null() {
        sk_X509_INFO_pop_free(sk,
                              Some(X509_INFO_free as
                                       unsafe extern "C" fn(_: *mut X509_INFO)
                                           -> ()));
    }
    return retval;
}
#[c2rust::src_loc = "5290:1"]
unsafe extern "C" fn load_cas_and_crls_dir(mut context: krb5_context,
                                           mut plg_cryptoctx:
                                               pkinit_plg_crypto_context,
                                           mut req_cryptoctx:
                                               pkinit_req_crypto_context,
                                           mut id_cryptoctx:
                                               pkinit_identity_crypto_context,
                                           mut catype: libc::c_int,
                                           mut dirname: *mut libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 22 as libc::c_int;
    let mut d: *mut DIR = 0 as *mut DIR;
    let mut dentry: *mut dirent = 0 as *mut dirent;
    let mut filename: [libc::c_char; 1024] = [0; 1024];
    if dirname.is_null() { return 22 as libc::c_int }
    d = opendir(dirname);
    if d.is_null() { return 2 as libc::c_int }
    loop  {
        dentry = readdir(d);
        if dentry.is_null() { current_block = 13056961889198038528; break ; }
        if strlen(dirname).wrapping_add(strlen((*dentry).d_name.as_mut_ptr())).wrapping_add(2
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_ulong)
               >
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
           {
            pkiDebug(b"%s: Path too long -- directory \'%s\' and file \'%s\'\n\x00"
                         as *const u8 as *const libc::c_char,
                     (*::std::mem::transmute::<&[u8; 22],
                                               &[libc::c_char; 22]>(b"load_cas_and_crls_dir\x00")).as_ptr(),
                     dirname, (*dentry).d_name.as_mut_ptr());
            current_block = 2841173334396386269;
            break ;
        } else {
            /* Ignore subdirectories and anything starting with a dot */
            if (*dentry).d_type as libc::c_int == DT_DIR as libc::c_int {
                continue ;
            }
            if (*dentry).d_name[0 as libc::c_int as usize] as libc::c_int ==
                   '.' as i32 {
                continue ;
            }
            snprintf(filename.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 1024]>() as
                         libc::c_ulong,
                     b"%s/%s\x00" as *const u8 as *const libc::c_char,
                     dirname, (*dentry).d_name.as_mut_ptr());
            retval =
                load_cas_and_crls(context, plg_cryptoctx, req_cryptoctx,
                                  id_cryptoctx, catype,
                                  filename.as_mut_ptr());
            if retval != 0 { current_block = 2841173334396386269; break ; }
        }
    }
    match current_block {
        13056961889198038528 => { retval = 0 as libc::c_int }
        _ => { }
    }
    if !d.is_null() { closedir(d); }
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "5340:1"]
pub unsafe extern "C" fn crypto_load_cas_and_crls(mut context: krb5_context,
                                                  mut plg_cryptoctx:
                                                      pkinit_plg_crypto_context,
                                                  mut req_cryptoctx:
                                                      pkinit_req_crypto_context,
                                                  mut idopts:
                                                      *mut pkinit_identity_opts,
                                                  mut id_cryptoctx:
                                                      pkinit_identity_crypto_context,
                                                  mut idtype: libc::c_int,
                                                  mut catype: libc::c_int,
                                                  mut id: *mut libc::c_char)
 -> krb5_error_code {
    match idtype {
        1 => {
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"PKINIT loading CA certs and CRLs from FILE\x00"
                                  as *const u8 as *const libc::c_char);
            }
            return load_cas_and_crls(context, plg_cryptoctx, req_cryptoctx,
                                     id_cryptoctx, catype, id)
        }
        2 => {
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"PKINIT loading CA certs and CRLs from DIR\x00"
                                  as *const u8 as *const libc::c_char);
            }
            return load_cas_and_crls_dir(context, plg_cryptoctx,
                                         req_cryptoctx, id_cryptoctx, catype,
                                         id)
        }
        _ => { return 95 as libc::c_int }
    };
}
#[c2rust::src_loc = "5367:1"]
unsafe extern "C" fn create_identifiers_from_stack(mut sk: *mut stack_st_X509,
                                                   mut ids:
                                                       *mut *mut *mut krb5_external_principal_identifier)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut sk_size: libc::c_int = sk_X509_num(sk);
    let mut krb5_cas: *mut *mut krb5_external_principal_identifier =
        0 as *mut *mut krb5_external_principal_identifier;
    let mut x: *mut X509 = 0 as *mut X509;
    let mut xn: *mut X509_NAME = 0 as *mut X509_NAME;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut is: *mut PKCS7_ISSUER_AND_SERIAL =
        0 as *mut PKCS7_ISSUER_AND_SERIAL;
    let mut buf: [libc::c_char; 256] = [0; 256];
    *ids = 0 as *mut *mut krb5_external_principal_identifier;
    krb5_cas =
        calloc((sk_size + 1 as libc::c_int) as libc::c_ulong,
               ::std::mem::size_of::<*mut krb5_external_principal_identifier>()
                   as libc::c_ulong) as
            *mut *mut krb5_external_principal_identifier;
    if krb5_cas.is_null() { return 12 as libc::c_int }
    i = 0 as libc::c_int;
    loop  {
        if !(i < sk_size) { current_block = 13460095289871124136; break ; }
        let ref mut fresh28 = *krb5_cas.offset(i as isize);
        *fresh28 =
            malloc(::std::mem::size_of::<krb5_external_principal_identifier>()
                       as libc::c_ulong) as
                *mut krb5_external_principal_identifier;
        x = sk_X509_value(sk, i);
        X509_NAME_oneline(X509_get_subject_name(x), buf.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 256]>() as
                              libc::c_ulong as libc::c_int);
        pkiDebug(b"#%d cert= %s\n\x00" as *const u8 as *const libc::c_char, i,
                 buf.as_mut_ptr());
        /* fill-in subjectName */
        (**krb5_cas.offset(i as isize)).subjectName.magic = 0 as libc::c_int;
        (**krb5_cas.offset(i as isize)).subjectName.length =
            0 as libc::c_int as libc::c_uint;
        let ref mut fresh29 =
            (**krb5_cas.offset(i as isize)).subjectName.data;
        *fresh29 = 0 as *mut libc::c_char;
        xn = X509_get_subject_name(x);
        len = i2d_X509_NAME(xn, 0 as *mut *mut libc::c_uchar);
        p = malloc(len as size_t) as *mut libc::c_uchar;
        if p.is_null() { current_block = 7203032973976327305; break ; }
        let ref mut fresh30 =
            (**krb5_cas.offset(i as isize)).subjectName.data;
        *fresh30 = p as *mut libc::c_char;
        i2d_X509_NAME(xn, &mut p);
        (**krb5_cas.offset(i as isize)).subjectName.length =
            len as libc::c_uint;
        /* fill-in issuerAndSerialNumber */
        (**krb5_cas.offset(i as isize)).issuerAndSerialNumber.length =
            0 as libc::c_int as libc::c_uint;
        (**krb5_cas.offset(i as isize)).issuerAndSerialNumber.magic =
            0 as libc::c_int;
        let ref mut fresh31 =
            (**krb5_cas.offset(i as isize)).issuerAndSerialNumber.data;
        *fresh31 = 0 as *mut libc::c_char;
        is = PKCS7_ISSUER_AND_SERIAL_new();
        if is.is_null() { current_block = 7203032973976327305; break ; }
        X509_NAME_set(&mut (*is).issuer, X509_get_issuer_name(x));
        ASN1_INTEGER_free((*is).serial);
        (*is).serial = ASN1_INTEGER_dup(X509_get_serialNumber(x));
        if (*is).serial.is_null() {
            current_block = 7203032973976327305;
            break ;
        }
        len = i2d_PKCS7_ISSUER_AND_SERIAL(is, 0 as *mut *mut libc::c_uchar);
        p = malloc(len as libc::c_ulong) as *mut libc::c_uchar;
        if p.is_null() { current_block = 7203032973976327305; break ; }
        let ref mut fresh32 =
            (**krb5_cas.offset(i as isize)).issuerAndSerialNumber.data;
        *fresh32 = p as *mut libc::c_char;
        i2d_PKCS7_ISSUER_AND_SERIAL(is, &mut p);
        (**krb5_cas.offset(i as isize)).issuerAndSerialNumber.length =
            len as libc::c_uint;
        /* fill-in subjectKeyIdentifier */
        (**krb5_cas.offset(i as isize)).subjectKeyIdentifier.length =
            0 as libc::c_int as libc::c_uint;
        (**krb5_cas.offset(i as isize)).subjectKeyIdentifier.magic =
            0 as libc::c_int;
        let ref mut fresh33 =
            (**krb5_cas.offset(i as isize)).subjectKeyIdentifier.data;
        *fresh33 = 0 as *mut libc::c_char;
        if X509_get_ext_by_NID(x, 82 as libc::c_int, -(1 as libc::c_int)) >=
               0 as libc::c_int {
            let mut ikeyid: *mut ASN1_OCTET_STRING =
                0 as *mut ASN1_OCTET_STRING;
            ikeyid =
                X509_get_ext_d2i(x, 82 as libc::c_int, 0 as *mut libc::c_int,
                                 0 as *mut libc::c_int) as
                    *mut ASN1_OCTET_STRING;
            if !ikeyid.is_null() {
                len =
                    i2d_ASN1_OCTET_STRING(ikeyid,
                                          0 as *mut *mut libc::c_uchar);
                p = malloc(len as libc::c_ulong) as *mut libc::c_uchar;
                if p.is_null() {
                    current_block = 7203032973976327305;
                    break ;
                }
                let ref mut fresh34 =
                    (**krb5_cas.offset(i as isize)).subjectKeyIdentifier.data;
                *fresh34 = p as *mut libc::c_char;
                i2d_ASN1_OCTET_STRING(ikeyid, &mut p);
                (**krb5_cas.offset(i as isize)).subjectKeyIdentifier.length =
                    len as libc::c_uint;
                ASN1_OCTET_STRING_free(ikeyid);
            }
        }
        PKCS7_ISSUER_AND_SERIAL_free(is);
        is = 0 as *mut PKCS7_ISSUER_AND_SERIAL;
        i += 1
    }
    match current_block {
        7203032973976327305 => {
            free_krb5_external_principal_identifier(&mut krb5_cas);
            PKCS7_ISSUER_AND_SERIAL_free(is);
            return 12 as libc::c_int
        }
        _ => { *ids = krb5_cas; return 0 as libc::c_int }
    };
}
#[c2rust::src_loc = "5462:1"]
unsafe extern "C" fn create_krb5_invalidCertificates(mut context:
                                                         krb5_context,
                                                     mut plg_cryptoctx:
                                                         pkinit_plg_crypto_context,
                                                     mut req_cryptoctx:
                                                         pkinit_req_crypto_context,
                                                     mut id_cryptoctx:
                                                         pkinit_identity_crypto_context,
                                                     mut ids:
                                                         *mut *mut *mut krb5_external_principal_identifier)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut sk: *mut stack_st_X509 = 0 as *mut stack_st_X509;
    *ids = 0 as *mut *mut krb5_external_principal_identifier;
    if (*req_cryptoctx).received_cert.is_null() {
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    sk = sk_X509_new_null();
    if !sk.is_null() {
        sk_X509_push(sk, (*req_cryptoctx).received_cert);
        retval = create_identifiers_from_stack(sk, ids);
        sk_X509_free(sk);
    }
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "5490:1"]
pub unsafe extern "C" fn create_krb5_supportedCMSTypes(mut context:
                                                           krb5_context,
                                                       mut plg_cryptoctx:
                                                           pkinit_plg_crypto_context,
                                                       mut req_cryptoctx:
                                                           pkinit_req_crypto_context,
                                                       mut id_cryptoctx:
                                                           pkinit_identity_crypto_context,
                                                       mut oids:
                                                           *mut *mut *mut krb5_algorithm_identifier)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut loids: *mut *mut krb5_algorithm_identifier =
        0 as *mut *mut krb5_algorithm_identifier;
    let mut des3oid: krb5_data =
        {
            let mut init =
                _krb5_data{magic: 0 as libc::c_int,
                           length: 8 as libc::c_int as libc::c_uint,
                           data:
                               b"*\x86H\x86\xf7\r\x03\x07\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,};
            init
        };
    *oids = 0 as *mut *mut krb5_algorithm_identifier;
    loids =
        malloc((2 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_algorithm_identifier>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_algorithm_identifier;
    if !loids.is_null() {
        let ref mut fresh35 = *loids.offset(1 as libc::c_int as isize);
        *fresh35 = 0 as *mut krb5_algorithm_identifier;
        let ref mut fresh36 = *loids.offset(0 as libc::c_int as isize);
        *fresh36 =
            malloc(::std::mem::size_of::<krb5_algorithm_identifier>() as
                       libc::c_ulong) as *mut krb5_algorithm_identifier;
        if (*loids.offset(0 as libc::c_int as isize)).is_null() {
            free(loids as *mut libc::c_void);
        } else {
            retval =
                pkinit_copy_krb5_data(&mut (**loids.offset(0 as libc::c_int as
                                                               isize)).algorithm,
                                      &mut des3oid);
            if retval != 0 {
                free(*loids.offset(0 as libc::c_int as isize) as
                         *mut libc::c_void);
                free(loids as *mut libc::c_void);
            } else {
                (**loids.offset(0 as libc::c_int as isize)).parameters.length
                    = 0 as libc::c_int as libc::c_uint;
                let ref mut fresh37 =
                    (**loids.offset(0 as libc::c_int as
                                        isize)).parameters.data;
                *fresh37 = 0 as *mut libc::c_char;
                *oids = loids;
                retval = 0 as libc::c_int
            }
        }
    }
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "5528:1"]
pub unsafe extern "C" fn create_krb5_trustedCertifiers(mut context:
                                                           krb5_context,
                                                       mut plg_cryptoctx:
                                                           pkinit_plg_crypto_context,
                                                       mut req_cryptoctx:
                                                           pkinit_req_crypto_context,
                                                       mut id_cryptoctx:
                                                           pkinit_identity_crypto_context,
                                                       mut ids:
                                                           *mut *mut *mut krb5_external_principal_identifier)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut sk: *mut stack_st_X509 = (*id_cryptoctx).trustedCAs;
    *ids = 0 as *mut *mut krb5_external_principal_identifier;
    if (*id_cryptoctx).trustedCAs.is_null() {
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    retval = create_identifiers_from_stack(sk, ids);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "5548:1"]
pub unsafe extern "C" fn create_issuerAndSerial(mut context: krb5_context,
                                                mut plg_cryptoctx:
                                                    pkinit_plg_crypto_context,
                                                mut req_cryptoctx:
                                                    pkinit_req_crypto_context,
                                                mut id_cryptoctx:
                                                    pkinit_identity_crypto_context,
                                                mut out:
                                                    *mut *mut libc::c_uchar,
                                                mut out_len:
                                                    *mut libc::c_uint)
 -> krb5_error_code {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut is: *mut PKCS7_ISSUER_AND_SERIAL =
        0 as *mut PKCS7_ISSUER_AND_SERIAL;
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut cert: *mut X509 = (*req_cryptoctx).received_cert;
    *out = 0 as *mut libc::c_uchar;
    *out_len = 0 as libc::c_int as libc::c_uint;
    if (*req_cryptoctx).received_cert.is_null() { return 0 as libc::c_int }
    is = PKCS7_ISSUER_AND_SERIAL_new();
    X509_NAME_set(&mut (*is).issuer, X509_get_issuer_name(cert));
    ASN1_INTEGER_free((*is).serial);
    (*is).serial = ASN1_INTEGER_dup(X509_get_serialNumber(cert));
    len = i2d_PKCS7_ISSUER_AND_SERIAL(is, 0 as *mut *mut libc::c_uchar);
    *out = malloc(len as size_t) as *mut libc::c_uchar;
    p = *out;
    if !p.is_null() {
        i2d_PKCS7_ISSUER_AND_SERIAL(is, &mut p);
        *out_len = len as libc::c_uint;
        retval = 0 as libc::c_int
    }
    X509_NAME_free((*is).issuer);
    ASN1_INTEGER_free((*is).serial);
    free(is as *mut libc::c_void);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "5586:1"]
pub unsafe extern "C" fn pkinit_process_td_trusted_certifiers(mut context:
                                                                  krb5_context,
                                                              mut plg_cryptoctx:
                                                                  pkinit_plg_crypto_context,
                                                              mut req_cryptoctx:
                                                                  pkinit_req_crypto_context,
                                                              mut id_cryptoctx:
                                                                  pkinit_identity_crypto_context,
                                                              mut krb5_trusted_certifiers:
                                                                  *mut *mut krb5_external_principal_identifier,
                                                              mut td_type:
                                                                  libc::c_int)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut sk_xn: *mut stack_st_X509_NAME = 0 as *mut stack_st_X509_NAME;
    let mut xn: *mut X509_NAME = 0 as *mut X509_NAME;
    let mut is: *mut PKCS7_ISSUER_AND_SERIAL =
        0 as *mut PKCS7_ISSUER_AND_SERIAL;
    let mut id: *mut ASN1_OCTET_STRING = 0 as *mut ASN1_OCTET_STRING;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut i: libc::c_int = 0 as libc::c_int;
    if td_type == 104 as libc::c_int {
        pkiDebug(b"received trusted certifiers\n\x00" as *const u8 as
                     *const libc::c_char);
    } else {
        pkiDebug(b"received invalid certificate\n\x00" as *const u8 as
                     *const libc::c_char);
    }
    sk_xn = sk_X509_NAME_new_null();
    loop  {
        if (*krb5_trusted_certifiers.offset(i as isize)).is_null() {
            current_block = 11913429853522160501;
            break ;
        }
        if !(**krb5_trusted_certifiers.offset(i as
                                                  isize)).subjectName.data.is_null()
           {
            p =
                (**krb5_trusted_certifiers.offset(i as
                                                      isize)).subjectName.data
                    as *mut libc::c_uchar;
            xn =
                d2i_X509_NAME(0 as *mut *mut X509_NAME, &mut p,
                              (**krb5_trusted_certifiers.offset(i as
                                                                    isize)).subjectName.length
                                  as libc::c_int as libc::c_long);
            if xn.is_null() { current_block = 11236409306149708511; break ; }
            X509_NAME_oneline(xn, buf.as_mut_ptr(),
                              ::std::mem::size_of::<[libc::c_char; 256]>() as
                                  libc::c_ulong as libc::c_int);
            if td_type == 104 as libc::c_int {
                pkiDebug(b"#%d cert = %s is trusted by kdc\n\x00" as *const u8
                             as *const libc::c_char, i, buf.as_mut_ptr());
            } else {
                pkiDebug(b"#%d cert = %s is invalid\n\x00" as *const u8 as
                             *const libc::c_char, i, buf.as_mut_ptr());
            }
            sk_X509_NAME_push(sk_xn, xn);
        }
        if !(**krb5_trusted_certifiers.offset(i as
                                                  isize)).issuerAndSerialNumber.data.is_null()
           {
            p =
                (**krb5_trusted_certifiers.offset(i as
                                                      isize)).issuerAndSerialNumber.data
                    as *mut libc::c_uchar;
            is =
                d2i_PKCS7_ISSUER_AND_SERIAL(0 as
                                                *mut *mut PKCS7_ISSUER_AND_SERIAL,
                                            &mut p,
                                            (**krb5_trusted_certifiers.offset(i
                                                                                  as
                                                                                  isize)).issuerAndSerialNumber.length
                                                as libc::c_int as
                                                libc::c_long);
            if is.is_null() { current_block = 11236409306149708511; break ; }
            X509_NAME_oneline((*is).issuer, buf.as_mut_ptr(),
                              ::std::mem::size_of::<[libc::c_char; 256]>() as
                                  libc::c_ulong as libc::c_int);
            if td_type == 104 as libc::c_int {
                pkiDebug(b"#%d issuer = %s serial = %ld is trusted bu kdc\n\x00"
                             as *const u8 as *const libc::c_char, i,
                         buf.as_mut_ptr(), ASN1_INTEGER_get((*is).serial));
            } else {
                pkiDebug(b"#%d issuer = %s serial = %ld is invalid\n\x00" as
                             *const u8 as *const libc::c_char, i,
                         buf.as_mut_ptr(), ASN1_INTEGER_get((*is).serial));
            }
            PKCS7_ISSUER_AND_SERIAL_free(is);
        }
        if !(**krb5_trusted_certifiers.offset(i as
                                                  isize)).subjectKeyIdentifier.data.is_null()
           {
            p =
                (**krb5_trusted_certifiers.offset(i as
                                                      isize)).subjectKeyIdentifier.data
                    as *mut libc::c_uchar;
            id =
                d2i_ASN1_OCTET_STRING(0 as *mut *mut ASN1_OCTET_STRING,
                                      &mut p,
                                      (**krb5_trusted_certifiers.offset(i as
                                                                            isize)).subjectKeyIdentifier.length
                                          as libc::c_int as libc::c_long);
            if id.is_null() { current_block = 11236409306149708511; break ; }
            /* XXX */
            ASN1_OCTET_STRING_free(id);
        }
        i += 1
    }
    match current_block {
        11913429853522160501 => {
            /* XXX Since we not doing anything with received trusted certifiers
     * return an error. this is the place where we can pick a different
     * client certificate based on the information in td_trusted_certifiers
     */
            retval = -(1765328360 as libc::c_long) as krb5_error_code
        }
        _ => { }
    }
    if !sk_xn.is_null() {
        sk_X509_NAME_pop_free(sk_xn,
                              Some(X509_NAME_free as
                                       unsafe extern "C" fn(_: *mut X509_NAME)
                                           -> ()));
    }
    return retval;
}
/* Originally based on OpenSSL's PKCS7_dataDecode(), now modified to remove the
 * use of BIO objects and to fit the PKINIT internal interfaces. */
#[c2rust::src_loc = "5668:1"]
unsafe extern "C" fn pkcs7_decrypt(mut context: krb5_context,
                                   mut id_cryptoctx:
                                       pkinit_identity_crypto_context,
                                   mut p7: *mut PKCS7,
                                   mut data_out: *mut *mut libc::c_uchar,
                                   mut len_out: *mut libc::c_uint)
 -> libc::c_int {
    let mut ret: krb5_error_code = 0;
    let mut ok: libc::c_int = 0 as libc::c_int;
    let mut plaintext_len: libc::c_int = 0 as libc::c_int;
    let mut final_len: libc::c_int = 0;
    let mut keylen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut eklen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut blocksize: libc::c_uint = 0;
    let mut ek: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tkey: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut plaintext: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut use_key: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut data_body: *mut ASN1_OCTET_STRING =
        (*(*(*p7).d.enveloped).enc_data).enc_data;
    let mut evp_cipher: *const EVP_CIPHER = 0 as *const EVP_CIPHER;
    let mut evp_ctx: *mut EVP_CIPHER_CTX = 0 as *mut EVP_CIPHER_CTX;
    let mut enc_alg: *mut X509_ALGOR =
        (*(*(*p7).d.enveloped).enc_data).algorithm;
    let mut rsk: *mut stack_st_PKCS7_RECIP_INFO =
        (*(*p7).d.enveloped).recipientinfo;
    let mut ri: *mut PKCS7_RECIP_INFO = 0 as *mut PKCS7_RECIP_INFO;
    *data_out = 0 as *mut libc::c_uchar;
    *len_out = 0 as libc::c_int as libc::c_uint;
    (*p7).state = 0 as libc::c_int;
    /* RFC 4556 section 3.2.3.2 requires that there be exactly one
     * recipientInfo. */
    if sk_PKCS7_RECIP_INFO_num(rsk) != 1 as libc::c_int {
        pkiDebug(b"invalid number of EnvelopedData RecipientInfos\n\x00" as
                     *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    ri = sk_PKCS7_RECIP_INFO_value(rsk, 0 as libc::c_int);
    evp_cipher =
        EVP_get_cipherbyname(OBJ_nid2sn(OBJ_obj2nid((*enc_alg).algorithm)));
    if !evp_cipher.is_null() {
        keylen = EVP_CIPHER_key_length(evp_cipher) as libc::c_uint;
        blocksize = EVP_CIPHER_block_size(evp_cipher) as libc::c_uint;
        evp_ctx = EVP_CIPHER_CTX_new();
        if !evp_ctx.is_null() {
            if !(EVP_DecryptInit(evp_ctx, evp_cipher,
                                 0 as *const libc::c_uchar,
                                 0 as *const libc::c_uchar) == 0 ||
                     EVP_CIPHER_asn1_to_param(evp_ctx, (*enc_alg).parameter)
                         <= 0 as libc::c_int) {
                /* Generate a random symmetric key to avoid exposing timing data if RSA
     * decryption fails the padding check. */
                tkey = malloc(keylen as libc::c_ulong) as *mut libc::c_uchar;
                if !(tkey.is_null() ||
                         EVP_CIPHER_CTX_rand_key(evp_ctx, tkey) == 0) {
                    /* Decrypt the secret key with the private key. */
                    ret =
                        pkinit_decode_data(context, id_cryptoctx,
                                           ASN1_STRING_get0_data((*ri).enc_key),
                                           ASN1_STRING_length((*ri).enc_key)
                                               as libc::c_uint, &mut ek,
                                           &mut eklen);
                    use_key =
                        if ret != 0 || eklen != keylen { tkey } else { ek };
                    /* Allocate a plaintext buffer and decrypt data_body into it. */
                    plaintext =
                        malloc(((*data_body).length as
                                    libc::c_uint).wrapping_add(blocksize) as
                                   libc::c_ulong) as *mut libc::c_uchar;
                    if !plaintext.is_null() {
                        if !(EVP_DecryptInit(evp_ctx, 0 as *const EVP_CIPHER,
                                             use_key,
                                             0 as *const libc::c_uchar) == 0)
                           {
                            if !(EVP_DecryptUpdate(evp_ctx, plaintext,
                                                   &mut plaintext_len,
                                                   (*data_body).data,
                                                   (*data_body).length) == 0)
                               {
                                if !(EVP_DecryptFinal(evp_ctx,
                                                      plaintext.offset(plaintext_len
                                                                           as
                                                                           isize),
                                                      &mut final_len) == 0) {
                                    plaintext_len += final_len;
                                    *len_out = plaintext_len as libc::c_uint;
                                    *data_out = plaintext;
                                    plaintext = 0 as *mut libc::c_uchar;
                                    ok = 1 as libc::c_int
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    EVP_CIPHER_CTX_free(evp_ctx);
    zapfree(plaintext as *mut libc::c_void, plaintext_len as size_t);
    zapfree(ek as *mut libc::c_void, eklen as size_t);
    zapfree(tkey as *mut libc::c_void, keylen as size_t);
    return ok;
}
#[c2rust::src_loc = "5788:1"]
unsafe extern "C" fn pkinit_pkcs11_code_to_text(mut err: libc::c_int)
 -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    static mut uc: [libc::c_char; 32] = [0; 32];
    i = 0 as libc::c_int;
    while !pkcs11_errstrings[i as usize].text.is_null() {
        if pkcs11_errstrings[i as usize].code as libc::c_int == err {
            break ;
        }
        i += 1
    }
    if !pkcs11_errstrings[i as usize].text.is_null() {
        return pkcs11_errstrings[i as usize].text
    }
    snprintf(uc.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
             dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                      b"unknown code 0x%x\x00" as *const u8 as
                          *const libc::c_char), err);
    return uc.as_mut_ptr();
}
/*
 * Add an item to the pkinit_identity_crypto_context's list of deferred
 * identities.
 */
#[no_mangle]
#[c2rust::src_loc = "5807:1"]
pub unsafe extern "C" fn crypto_set_deferred_id(mut context: krb5_context,
                                                mut id_cryptoctx:
                                                    pkinit_identity_crypto_context,
                                                mut identity:
                                                    *const libc::c_char,
                                                mut password:
                                                    *const libc::c_char)
 -> krb5_error_code {
    let mut ck_flags: libc::c_ulong = 0;
    ck_flags =
        pkinit_get_deferred_id_flags((*id_cryptoctx).deferred_ids, identity);
    return pkinit_set_deferred_id(&mut (*id_cryptoctx).deferred_ids, identity,
                                  ck_flags, password);
}
/*
 * Retrieve a read-only copy of the pkinit_identity_crypto_context's list of
 * deferred identities, sure to be valid only until the next time someone calls
 * either pkinit_set_deferred_id() or crypto_set_deferred_id().
 */
#[no_mangle]
#[c2rust::src_loc = "5825:1"]
pub unsafe extern "C" fn crypto_get_deferred_ids(mut context: krb5_context,
                                                 mut id_cryptoctx:
                                                     pkinit_identity_crypto_context)
 -> *const pkinit_deferred_id {
    let mut deferred: *mut pkinit_deferred_id = 0 as *mut pkinit_deferred_id;
    let mut ret: *const pkinit_deferred_id = 0 as *const pkinit_deferred_id;
    deferred = (*id_cryptoctx).deferred_ids;
    ret = deferred as *const pkinit_deferred_id;
    return ret;
}
/* Return the received certificate as DER-encoded data. */
#[no_mangle]
#[c2rust::src_loc = "5838:1"]
pub unsafe extern "C" fn crypto_encode_der_cert(mut context: krb5_context,
                                                mut reqctx:
                                                    pkinit_req_crypto_context,
                                                mut der_out:
                                                    *mut *mut uint8_t,
                                                mut der_len: *mut size_t)
 -> krb5_error_code {
    let mut len: libc::c_int = 0;
    let mut der: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    *der_out = 0 as *mut uint8_t;
    *der_len = 0 as libc::c_int as size_t;
    if (*reqctx).received_cert.is_null() { return 22 as libc::c_int }
    p = 0 as *mut libc::c_uchar;
    len = i2d_X509((*reqctx).received_cert, 0 as *mut *mut libc::c_uchar);
    if len <= 0 as libc::c_int { return 22 as libc::c_int }
    der = malloc(len as libc::c_ulong) as *mut libc::c_uchar;
    p = der;
    if der.is_null() { return 12 as libc::c_int }
    if i2d_X509((*reqctx).received_cert, &mut p) <= 0 as libc::c_int {
        free(der as *mut libc::c_void);
        return 22 as libc::c_int
    }
    *der_out = der;
    *der_len = len as size_t;
    return 0 as libc::c_int;
}
/*
 * COPYRIGHT (C) 2007
 * THE REGENTS OF THE UNIVERSITY OF MICHIGAN
 * ALL RIGHTS RESERVED
 *
 * Permission is granted to use, copy, create derivative works
 * and redistribute this software and such derivative works
 * for any purpose, so long as the name of The University of
 * Michigan is not used in any advertising or publicity
 * pertaining to the use of distribution of this software
 * without specific, written prior authorization.  If the
 * above copyright notice or any other identification of the
 * University of Michigan is included in any copy of any
 * portion of this software, then the disclaimer below must
 * also be included.
 *
 * THIS SOFTWARE IS PROVIDED AS IS, WITHOUT REPRESENTATION
 * FROM THE UNIVERSITY OF MICHIGAN AS TO ITS FITNESS FOR ANY
 * PURPOSE, AND WITHOUT WARRANTY BY THE UNIVERSITY OF
 * MICHIGAN OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING
 * WITHOUT LIMITATION THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE
 * REGENTS OF THE UNIVERSITY OF MICHIGAN SHALL NOT BE LIABLE
 * FOR ANY DAMAGES, INCLUDING SPECIAL, INDIRECT, INCIDENTAL, OR
 * CONSEQUENTIAL DAMAGES, WITH RESPECT TO ANY CLAIM ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OF THE SOFTWARE, EVEN
 * IF IT HAS BEEN OR IS HEREAFTER ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGES.
 */
/*
 * This header defines the cryptographic interface
 */
/*
 * these describe the CMS message types
 */
/*
 * storage types for identity information
 */
/*
 * ca/crl types
 */
/*
 * The following represent Key Usage values that we
 * may care about in a certificate
 */
/*
 * The following represent Extended Key Usage oid values
 * that we may care about in a certificate
 */
/* Handle to cert, opaque above crypto interface */
/* Handle to cert iteration information, opaque above crypto interface */
/* XXX */
/* rfc2253-style subject name string */
/* rfc2253-style issuer name string */
/* key usage information */
/* extended key usage information */
/* Null-terminated array of PKINIT SANs */
/* Null-terimnated array of UPN SANs */
/*
 * Functions to initialize and cleanup crypto contexts
 */
/* *Create a pkinit ContentInfo*/
/* IN */
/* IN */
/* IN */
/* IN */
/*
 * this function creates a CMS message where eContentType is SignedData
 */
/* IN */
/* IN */
/* IN */
/* IN */
/* IN
		    specifies CMS_SIGN_CLIENT for client-side CMS message
		    and CMS_SIGN_SERVER for kdc-side */
/* IN
		    specifies where certificates field in SignedData
		    should contain certificate path */
/* IN
		    contains DER encoded AuthPack (CMS_SIGN_CLIENT)
		    or DER encoded DHRepInfo (CMS_SIGN_SERVER) */
/* IN
		    contains length of auth_pack */
/* OUT
		    for CMS_SIGN_CLIENT receives DER encoded
		    SignedAuthPack (CMS_SIGN_CLIENT) or DER
		    encoded DHInfo (CMS_SIGN_SERVER) */
/* OUT
		    receives length of signed_data */
/*
 * this function verifies a CMS message where eContentType is SignedData
 */
/* IN */
/* IN */
/* IN */
/* IN */
/* IN
		    specifies CMS_SIGN_CLIENT for client-side
		    CMS message and CMS_SIGN_SERVER for kdc-side */
/* IN
		    specifies whether CRL checking should be
		    strictly enforced, i.e. if no CRLs available
		    for the CA then fail verification.
		    note, if the value is 0, crls are still
		    checked if present */
/* IN
		    contains DER encoded SignedAuthPack (CMS_SIGN_CLIENT)
		    or DER encoded DHInfo (CMS_SIGN_SERVER) */
/* IN
		    contains length of signed_data*/
/* OUT
		    receives DER encoded AuthPack (CMS_SIGN_CLIENT)
		    or DER encoded DHRepInfo (CMS_SIGN_SERVER)*/
/* OUT
		    receives length of auth_pack */
/* OUT
		    receives required authorization data that
		    contains the verified certificate chain
		    (only used by the KDC) */
/* OUT
		    receives length of authz_data */
/* OUT
		    receives whether message is signed */
/*
 * this function creates a CMS message where eContentType is EnvelopedData
 */
/* IN */
/* IN */
/* IN */
/* IN */
/* IN */
/* IN
		    specifies whether the certificates field in
		    SignedData should contain certificate path */
/* IN
		    contains DER encoded ReplyKeyPack */
/* IN
		    contains length of key_pack */
/* OUT
		    receives DER encoded encKeyPack */
/* OUT
		    receives length of envel_data */
/*
 * this function creates a CMS message where eContentType is EnvelopedData
 */
/* IN */
/* IN */
/* IN */
/* IN */
/* IN */
/* IN
		    specifies whether CRL checking should be
		    strictly enforced */
/* IN
		    contains DER encoded encKeyPack */
/* IN
		    contains length of envel_data */
/* OUT
		    receives ReplyKeyPack */
/* OUT
		    receives length of signed_data */
/*
 * This function retrieves the signer's identity, in a form that could
 * be passed back in to a future invocation of this module as a candidate
 * client identity location.
 */
/* IN */
/* IN */
/* OUT */
/*
 * this function returns SAN information found in the
 * received certificate.  at least one of pkinit_sans,
 * upn_sans, or kdc_hostnames must be non-NULL.
 */
/* IN */
/* IN */
/* IN */
/* IN */
/* OUT
		    if non-NULL, a null-terminated array of
		    id-pkinit-san values found in the certificate
		    are returned */
/* OUT
		    if non-NULL, a null-terminated array of
		    id-ms-upn-san values found in the certificate
		    are returned */
/* OUT
		    if non-NULL, a null-terminated array of
		    dNSName (hostname) SAN values found in the
		    certificate are returned */
/*
 * this function checks for acceptable key usage values
 * in the received certificate.
 *
 * when checking a received kdc certificate, it looks for
 * the kpKdc key usage.  if allow_secondary_usage is
 * non-zero, it will also accept kpServerAuth.
 *
 * when checking a received user certificate, it looks for
 * kpClientAuth key usage.  if allow_secondary_usage is
 * non-zero, it will also accept id-ms-sc-logon EKU.
 *
 * this function must also assert that the digitalSignature
 * key usage is consistent.
 */
/* IN */
/* IN */
/* IN */
/* IN */
/* IN
		    specifies if the received certificate is
		    a KDC certificate (non-zero),
		    or a user certificate (zero) */
/* IN
		    specifies if the secondary key usage
		    should be accepted or not (see above) */
/* OUT
		    receives non-zero if an acceptable EKU was found */
/*
 * this functions takes in generated DH secret key and converts
 * it in to a kerberos session key. it takes into the account the
 * enc type and then follows the procedure specified in the RFC p 22.
 */
/* IN */
/* IN
		    specifies the enc type */
/* IN
		    contains the DH secret key */
/* IN
		    contains length of key */
/* OUT
		    receives kerberos session key */
/*
 * this function implements clients first part of the DH protocol.
 * client selects its DH parameters and pub key
 */
/* IN */
/* IN */
/* IN */
/* IN */
/* IN
		    specifies the DH modulous, eg 1024, 2048, or 4096 */
/* OUT
		    contains DER encoded DH params */
/* OUT
		    contains length of encoded DH params */
/* OUT
		    receives DER encoded DH pub key */
/* OUT
		    receives length of DH pub key */
/*
 * this function completes client's the DH protocol. client
 * processes received DH pub key from the KDC and computes
 * the DH secret key
 */
/* IN */
/* IN */
/* IN */
/* IN */
/* IN
		    contains client's DER encoded DH pub key */
/* IN
		    contains length of dh_pubkey */
/* OUT
		    receives DH secret key */
/* OUT
		    receives length of DH secret key */
/*
 * this function implements the KDC first part of the DH protocol.
 * it decodes the client's DH parameters and pub key and checks
 * if they are acceptable.
 */
/* IN */
/* IN */
/* IN */
/* IN */
/* IN
		    ???? */
/* IN
		    the mininum number of key bits acceptable */
/*
 * this function completes the KDC's DH protocol. The KDC generates
 * its DH pub key and computes the DH secret key
 */
/* IN */
/* IN */
/* IN */
/* IN */
/* IN
		    contains client's DER encoded DH pub key */
/* IN
		    contains length of received_pubkey */
/* OUT
		    receives KDC's DER encoded DH pub key */
/* OUT
		    receives length of dh_pubkey */
/* OUT
		    receives DH secret key */
/* OUT
		    receives length of DH secret key */
/*
 * this functions takes in crypto specific representation of
 * supportedCMSTypes and creates a list of
 * krb5_algorithm_identifier
 */
/* IN */
/* IN */
/* IN */
/* IN */
/* OUT */
/*
 * this functions takes in crypto specific representation of
 * trustedCertifiers and creates a list of
 * krb5_external_principal_identifier
 */
/* IN */
/* IN */
/* IN */
/* IN */
/* OUT */
/*
 * this functions takes in crypto specific representation of the
 * KDC's certificate and creates a DER encoded kdcPKId
 */
/* IN */
/* IN */
/* IN */
/* IN */
/* OUT
		    receives DER encoded kdcPKId */
/* OUT
		    receives length of encoded kdcPKId */
/*
 * These functions manipulate the deferred-identities list in the identity
 * context, which is opaque outside of the crypto-specific bits.
 */
/*
 * process the values from idopts and obtain the cert(s)
 * specified by those options, populating the id_cryptoctx.
 */
/* IN */
/* IN */
/* IN */
/* IN */
/* IN/OUT */
/* IN */
/* IN */
/*
 * Free up information held from crypto_load_certs()
 */
/*
 * Get a null-terminated list of certificate matching data objects for the
 * certificates loaded in id_cryptoctx.
 */
/*
 * Free a matching data object.
 */
/*
 * Free a list of matching data objects.
 */
/*
 * Choose one of the certificates loaded in idctx to use for PKINIT client
 * operations.  cred_index must be an index into the array of matching objects
 * returned by crypto_cert_get_matching_data().
 */
/*
 * Select the default certificate as "the chosen one"
 */
/* IN */
/* IN */
/* IN */
/* IN */
/*
 * process the values from idopts and obtain the anchor or
 * intermediate certificates, or crls specified by idtype,
 * catype, and id
 */
/* IN */
/* IN */
/* IN */
/* IN */
/* IN/OUT */
/* IN
		    defines the storage type (file, directory, etc) */
/* IN
		    defines the ca type (anchor, intermediate, crls) */
/* IN
		    defines the location (filename, directory name, etc) */
/*
 * on the client, obtain the kdc's certificate to include
 * in a request
 */
/* IN */
/* IN */
/* IN */
/* IN/OUT */
/* IN */
/*
 * this function creates edata that contains TD-DH-PARAMETERS
 */
/* IN */
/* IN */
/* IN */
/* IN */
/* IN */
/* OUT */
/*
 * this function processes edata that contains TD-DH-PARAMETERS.
 * the client processes the received acceptable by KDC DH
 * parameters and picks the first acceptable to it. it matches
 * them against the known DH parameters.
 */
/* IN */
/* IN */
/* IN */
/* IN */
/* IN */
/* OUT
		    receives the new DH modulus to use in the new AS-REQ */
/*
 * this function creates edata that contains TD-INVALID-CERTIFICATES
 */
/* IN */
/* IN */
/* IN */
/* IN */
/* OUT */
/*
 * this function creates edata that contains TD-TRUSTED-CERTIFIERS
 */
/* IN */
/* IN */
/* IN */
/* IN */
/* OUT */
/*
 * this function processes edata that contains either
 * TD-TRUSTED-CERTIFICATES or TD-INVALID-CERTIFICATES.
 * current implementation only decodes the received message
 * but does not act on it
 */
/* IN */
/* IN */
/* IN */
/* IN */
/* IN */
/* IN */
/*
 * this function checks if the received kdcPKId matches
 * the KDC's certificate
 */
/* IN */
/* IN */
/* IN */
/* IN */
/* IN
		    contains DER encoded kdcPKId */
/* IN
		    contains length of pdid_buf */
/* OUT
		    1 if kdcPKId matches, otherwise 0 */
/* IN */
/* IN */
/* IN */
/* *
 * An ordered set of OIDs, stored as krb5_data, of KDF algorithms
 * supported by this implementation. The order of this array controls
 * the order in which the server will pick.
 */
/*
 * Get the certificate matching data from the request certificate.
 */
#[no_mangle]
#[c2rust::src_loc = "5869:1"]
pub unsafe extern "C" fn crypto_req_cert_matching_data(mut context:
                                                           krb5_context,
                                                       mut plgctx:
                                                           pkinit_plg_crypto_context,
                                                       mut reqctx:
                                                           pkinit_req_crypto_context,
                                                       mut md_out:
                                                           *mut *mut pkinit_cert_matching_data)
 -> krb5_error_code {
    *md_out = 0 as *mut pkinit_cert_matching_data;
    if reqctx.is_null() || (*reqctx).received_cert.is_null() {
        return 2 as libc::c_int
    }
    return get_matching_data(context, plgctx, reqctx, (*reqctx).received_cert,
                             md_out);
}
unsafe extern "C" fn run_static_initializers() {
    DHvparams_it =
        {
            let mut init =
                ASN1_ITEM_st{itype: 0x1 as libc::c_int as libc::c_char,
                             utype: 16 as libc::c_int as libc::c_long,
                             templates: DHvparams_seq_tt.as_ptr(),
                             tcount:
                                 (::std::mem::size_of::<[ASN1_TEMPLATE; 2]>()
                                      as
                                      libc::c_ulong).wrapping_div(::std::mem::size_of::<ASN1_TEMPLATE>()
                                                                      as
                                                                      libc::c_ulong)
                                     as libc::c_long,
                             funcs: 0 as *const libc::c_void,
                             size:
                                 ::std::mem::size_of::<int_dhvparams>() as
                                     libc::c_ulong as libc::c_long,
                             sname:
                                 b"int_dhvparams\x00" as *const u8 as
                                     *const libc::c_char,};
            init
        };
    DHxparams_it =
        {
            let mut init =
                ASN1_ITEM_st{itype: 0x1 as libc::c_int as libc::c_char,
                             utype: 16 as libc::c_int as libc::c_long,
                             templates: DHxparams_seq_tt.as_ptr(),
                             tcount:
                                 (::std::mem::size_of::<[ASN1_TEMPLATE; 5]>()
                                      as
                                      libc::c_ulong).wrapping_div(::std::mem::size_of::<ASN1_TEMPLATE>()
                                                                      as
                                                                      libc::c_ulong)
                                     as libc::c_long,
                             funcs: 0 as *const libc::c_void,
                             size:
                                 ::std::mem::size_of::<int_dhx942_dh>() as
                                     libc::c_ulong as libc::c_long,
                             sname:
                                 b"int_dhx942_dh\x00" as *const u8 as
                                     *const libc::c_char,};
            init
        }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
