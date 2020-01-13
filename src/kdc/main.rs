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
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
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
    #[c2rust::src_loc = "154:1"]
    pub type __pid_t = libc::c_int;
    #[c2rust::src_loc = "156:1"]
    pub type __clock_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/sys/types.h:27"]
pub mod sys_types_h {
    #[c2rust::src_loc = "97:1"]
    pub type pid_t = __pid_t;
    use super::types_h::__pid_t;
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
#[c2rust::header_src = "/usr/include/bits/types/__sigset_t.h:27"]
pub mod __sigset_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "5:9"]
    pub struct __sigset_t {
        pub __val: [libc::c_ulong; 16],
    }
}
#[c2rust::header_src = "/usr/include/bits/types/sigset_t.h:27"]
pub mod sigset_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type sigset_t = __sigset_t;
    use super::__sigset_t_h::__sigset_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stdarg.h:27"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:27"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
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
#[c2rust::header_src = "/usr/include/bits/types/__sigval_t.h:27"]
pub mod __sigval_t_h {
    #[c2rust::src_loc = "30:1"]
    pub type __sigval_t = sigval;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:7"]
    pub union sigval {
        pub sival_int: libc::c_int,
        pub sival_ptr: *mut libc::c_void,
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:27"]
pub mod krb5_h {
    #[c2rust::src_loc = "136:1"]
    pub type krb5_octet = uint8_t;
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
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
    #[c2rust::src_loc = "195:1"]
    pub type krb5_timestamp = krb5_int32;
    #[c2rust::src_loc = "197:1"]
    pub type krb5_deltat = krb5_int32;
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
    #[c2rust::src_loc = "363:1"]
    pub type krb5_keyblock = _krb5_keyblock;
    #[c2rust::src_loc = "479:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "490:5"]
    pub const KRB5_C_RANDSOURCE_MAX: C2RustUnnamed = 5;
    #[c2rust::src_loc = "489:5"]
    pub const KRB5_C_RANDSOURCE_EXTERNAL_PROTOCOL: C2RustUnnamed = 4;
    #[c2rust::src_loc = "488:5"]
    pub const KRB5_C_RANDSOURCE_TIMING: C2RustUnnamed = 3;
    #[c2rust::src_loc = "482:5"]
    pub const KRB5_C_RANDSOURCE_TRUSTEDPARTY: C2RustUnnamed = 2;
    #[c2rust::src_loc = "481:5"]
    pub const KRB5_C_RANDSOURCE_OSRAND: C2RustUnnamed = 1;
    #[c2rust::src_loc = "480:5"]
    pub const KRB5_C_RANDSOURCE_OLDAPI: C2RustUnnamed = 0;
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
    #[c2rust::src_loc = "2727:1"]
    pub type krb5_keytab_entry = krb5_keytab_entry_st;
    #[c2rust::src_loc = "2736:1"]
    pub type krb5_keytab = *mut _krb5_kt;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::{_krb5_context, _krb5_kt};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "125:8"]
        pub type _profile_t;
        #[no_mangle]
        #[c2rust::src_loc = "784:1"]
        pub fn krb5_c_random_add_entropy(context: krb5_context,
                                         randsource: libc::c_uint,
                                         data: *const krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2782:1"]
        pub fn krb5_kt_close(context: krb5_context, keytab: krb5_keytab)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2972:1"]
        pub fn krb5_free_context(context: krb5_context);
        #[no_mangle]
        #[c2rust::src_loc = "4025:1"]
        pub fn krb5_build_principal(context: krb5_context,
                                    princ: *mut krb5_principal,
                                    rlen: libc::c_uint,
                                    realm: *const libc::c_char, _: ...)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4912:1"]
        pub fn krb5_free_default_realm(context: krb5_context,
                                       lrealm: *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "4903:1"]
        pub fn krb5_set_default_realm(context: krb5_context,
                                      lrealm: *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4887:1"]
        pub fn krb5_get_default_realm(context: krb5_context,
                                      lrealm: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "6266:1"]
        pub fn krb5_string_to_enctype(string: *mut libc::c_char,
                                      enctypep: *mut krb5_enctype)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6341:1"]
        pub fn krb5_enctype_to_name(enctype: krb5_enctype,
                                    shortest: krb5_boolean,
                                    buffer: *mut libc::c_char, buflen: size_t)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "8000:1"]
        pub fn krb5_copy_error_message(dest_ctx: krb5_context,
                                       src_ctx: krb5_context);
        #[no_mangle]
        #[c2rust::src_loc = "8023:1"]
        pub fn krb5_get_error_message(ctx: krb5_context,
                                      code: krb5_error_code)
         -> *const libc::c_char;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:27"]
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
                        krb5_keytab_entry, krb5_kt_cursor, krb5_timestamp};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::stddef_h::size_t;
    use super::string_h::explicit_bzero;
    use super::stdlib_h::free;
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
        #[c2rust::src_loc = "2233:1"]
        pub fn krb5_set_time_offsets(_: krb5_context, _: krb5_timestamp,
                                     _: krb5_int32) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2096:1"]
        pub fn krb5int_c_deprecated_enctype(_: krb5_enctype) -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "619:1"]
        pub fn krb5int_init_context_kdc(_: *mut krb5_context)
         -> krb5_error_code;
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:27"]
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
    use super::internal::__va_list_tag;
    extern "C" {
        /* Public interfaces */
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn com_err(_: *const libc::c_char, _: errcode_t,
                       _: *const libc::c_char, _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "47:1"]
        pub fn com_err_va(whoami: *const libc::c_char, code: errcode_t,
                          fmt: *const libc::c_char, ap: ::std::ffi::VaList);
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src = "/usr/include/bits/types/siginfo_t.h:27"]
pub mod siginfo_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:9"]
    pub struct siginfo_t {
        pub si_signo: libc::c_int,
        pub si_errno: libc::c_int,
        pub si_code: libc::c_int,
        pub __pad0: libc::c_int,
        pub _sifields: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "51:5"]
    pub union C2RustUnnamed_0 {
        pub _pad: [libc::c_int; 28],
        pub _kill: C2RustUnnamed_9,
        pub _timer: C2RustUnnamed_8,
        pub _rt: C2RustUnnamed_7,
        pub _sigchld: C2RustUnnamed_6,
        pub _sigfault: C2RustUnnamed_3,
        pub _sigpoll: C2RustUnnamed_2,
        pub _sigsys: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "116:2"]
    pub struct C2RustUnnamed_1 {
        pub _call_addr: *mut libc::c_void,
        pub _syscall: libc::c_int,
        pub _arch: libc::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "108:2"]
    pub struct C2RustUnnamed_2 {
        pub si_band: libc::c_long,
        pub si_fd: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "89:2"]
    pub struct C2RustUnnamed_3 {
        pub si_addr: *mut libc::c_void,
        pub si_addr_lsb: libc::c_short,
        pub _bounds: C2RustUnnamed_4,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "94:6"]
    pub union C2RustUnnamed_4 {
        pub _addr_bnd: C2RustUnnamed_5,
        pub _pkey: __uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "97:3"]
    pub struct C2RustUnnamed_5 {
        pub _lower: *mut libc::c_void,
        pub _upper: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "79:2"]
    pub struct C2RustUnnamed_6 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
        pub si_status: libc::c_int,
        pub si_utime: __clock_t,
        pub si_stime: __clock_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "71:2"]
    pub struct C2RustUnnamed_7 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
        pub si_sigval: __sigval_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:2"]
    pub struct C2RustUnnamed_8 {
        pub si_tid: libc::c_int,
        pub si_overrun: libc::c_int,
        pub si_sigval: __sigval_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "56:2"]
    pub struct C2RustUnnamed_9 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
    }
    use super::types_h::{__uint32_t, __pid_t, __uid_t, __clock_t};
    use super::__sigval_t_h::__sigval_t;
}
#[c2rust::header_src = "/usr/include/signal.h:27"]
pub mod signal_h {
    #[c2rust::src_loc = "72:1"]
    pub type __sighandler_t
        =
        Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
    use super::types_h::__pid_t;
    use super::sigset_t_h::sigset_t;
    use super::sigaction_h::sigaction;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "112:1"]
        pub fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "196:1"]
        pub fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "240:1"]
        pub fn sigaction(__sig: libc::c_int, __act: *const sigaction,
                         __oact: *mut sigaction) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/sigaction.h:27"]
pub mod sigaction_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:8"]
    pub struct sigaction {
        pub __sigaction_handler: C2RustUnnamed_10,
        pub sa_mask: __sigset_t,
        pub sa_flags: libc::c_int,
        pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:5"]
    pub union C2RustUnnamed_10 {
        pub sa_handler: __sighandler_t,
        pub sa_sigaction: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *mut siginfo_t,
                                                      _: *mut libc::c_void)
                                     -> ()>,
    }
    use super::__sigset_t_h::__sigset_t;
    use super::signal_h::__sighandler_t;
    use super::siginfo_t_h::siginfo_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/kdcpreauth_plugin.h:31"]
pub mod kdcpreauth_plugin_h {
    extern "C" {
        #[c2rust::src_loc = "119:8"]
        pub type verto_ctx;
    }
    /* KRB5_KDCPREAUTH_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kdc/realm_data.h:31"]
pub mod realm_data_h {
    #[c2rust::src_loc = "36:1"]
    pub type kdc_realm_t = __kdc_realm_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:16"]
    pub struct __kdc_realm_data {
        pub realm_name: *mut libc::c_char,
        pub realm_context: krb5_context,
        pub realm_keytab: krb5_keytab,
        pub realm_hostbased: *mut libc::c_char,
        pub realm_no_referral: *mut libc::c_char,
        pub realm_stash: *mut libc::c_char,
        pub realm_mpname: *mut libc::c_char,
        pub realm_mprinc: krb5_principal,
        pub realm_mkey: krb5_keyblock,
        pub realm_tgsprinc: krb5_principal,
        pub realm_listen: *mut libc::c_char,
        pub realm_tcp_listen: *mut libc::c_char,
        pub realm_maxlife: krb5_deltat,
        pub realm_maxrlife: krb5_deltat,
        pub realm_reject_bad_transit: krb5_boolean,
        pub realm_restrict_anon: krb5_boolean,
    }
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kdc/realm_data.h */
/*
 * Copyright (C) 2012 by the Massachusetts Institute of Technology.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * * Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.
 *
 * * Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in
 *   the documentation and/or other materials provided with the
 *   distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 */
    /*
     * General Kerberos per-realm data.
     */
    /* Realm name                       */
    /* XXX the real context should go away once the db_context is done.
 * The db_context is then associated with the realm keytab using
 * krb5_ktkdb_resolv(). There should be nothing in the context which
 * cannot span multiple realms -- proven */
    /* Context to be used for realm     */
    /* keytab to be used for this realm */
    /* referral services for NT-UNKNOWN */
    /* non-referral services         */
    /*
     * Database per-realm data.
     */
    /* Stash file name for realm        */
    /* Master principal name for realm  */
    /* Master principal for realm       */
    /*
     * Note realm_mkey is mkey read from stash or keyboard and may not be the
     * latest.
     */
    /* Master key for this realm        */
    /*
     * TGS per-realm data.
     */
    /* TGS principal for this realm     */
    /*
     * Other per-realm data.
     */
    /* Per-realm KDC UDP listen */
    /* Per-realm KDC TCP listen */
    /*
     * Per-realm parameters.
     */
    /* Maximum ticket life for realm    */
    /* Maximum renewable life for realm */
    /* Accept unverifiable transited_realm ? */
    /* Anon to local TGT only */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "78:8"]
    pub struct server_handle {
        pub kdc_realmlist: *mut *mut kdc_realm_t,
        pub kdc_numrealms: libc::c_int,
        pub kdc_err_context: krb5_context,
    }
    use super::krb5_h::{krb5_context, krb5_keytab, krb5_principal,
                        krb5_keyblock, krb5_deltat, krb5_boolean};
    /* REALM_DATA_H */
    /*
 * These macros used to refer to a global pointer to the active realm state
 * structure for a request.  They now refer to a local variable that must be
 * properly declared in each function that uses these macros.
 */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/verto.h:31"]
pub mod verto_h {
    #[c2rust::src_loc = "51:9"]
    pub type verto_ev_type = libc::c_uint;
    #[c2rust::src_loc = "57:5"]
    pub const VERTO_EV_TYPE_CHILD: verto_ev_type = 16;
    #[c2rust::src_loc = "56:5"]
    pub const VERTO_EV_TYPE_SIGNAL: verto_ev_type = 8;
    #[c2rust::src_loc = "55:5"]
    pub const VERTO_EV_TYPE_IDLE: verto_ev_type = 4;
    #[c2rust::src_loc = "54:5"]
    pub const VERTO_EV_TYPE_TIMEOUT: verto_ev_type = 2;
    #[c2rust::src_loc = "53:5"]
    pub const VERTO_EV_TYPE_IO: verto_ev_type = 1;
    #[c2rust::src_loc = "52:5"]
    pub const VERTO_EV_TYPE_NONE: verto_ev_type = 0;
    use super::kdcpreauth_plugin_h::verto_ctx;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "274:1"]
        pub fn verto_reinitialize(ctx: *mut verto_ctx) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "239:1"]
        pub fn verto_run(ctx: *mut verto_ctx);
    }
    /* VERTO_H_ */
    /* __cplusplus */
}
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "252:14"]
        pub fn strrchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "435:1"]
        pub fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "104:1"]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
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
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
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
        #[c2rust::src_loc = "213:1"]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "246:14"]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:27"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:27"]
pub mod unistd_h {
    use super::types_h::__pid_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "628:1"]
        pub fn getpid() -> __pid_t;
        #[no_mangle]
        #[c2rust::src_loc = "756:1"]
        pub fn fork() -> __pid_t;
        #[no_mangle]
        #[c2rust::src_loc = "935:1"]
        pub fn daemon(__nochdir: libc::c_int, __noclose: libc::c_int)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/getopt_core.h:27"]
pub mod getopt_core_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "36:14"]
        pub static mut optarg: *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "50:12"]
        pub static mut optind: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "91:1"]
        pub fn getopt(___argc: libc::c_int, ___argv: *const *mut libc::c_char,
                      __shortopts: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:27"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:29"]
pub mod kdb_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_error_code, krb5_principal_data,
                        krb5_principal, krb5_enctype, krb5_boolean, krb5_kvno,
                        krb5_data, krb5_keyblock};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "358:1"]
        pub fn krb5_db_open(kcontext: krb5_context,
                            db_args: *mut *mut libc::c_char,
                            mode: libc::c_int) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "363:1"]
        pub fn krb5_db_fini(kcontext: krb5_context) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "404:1"]
        pub fn krb5_db_fetch_mkey(context: krb5_context,
                                  mname: krb5_principal, etype: krb5_enctype,
                                  fromkeyboard: krb5_boolean,
                                  twice: krb5_boolean,
                                  db_args: *mut libc::c_char,
                                  kvno: *mut krb5_kvno, salt: *mut krb5_data,
                                  key: *mut krb5_keyblock) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "413:1"]
        pub fn krb5_db_fetch_mkey_list(context: krb5_context,
                                       mname: krb5_principal,
                                       mkey: *const krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "435:1"]
        pub fn krb5_db_setup_mkey_name(context: krb5_context,
                                       keyname: *const libc::c_char,
                                       realm: *const libc::c_char,
                                       fullname: *mut *mut libc::c_char,
                                       principal: *mut krb5_principal)
         -> krb5_error_code;
    }
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/adm_proto.h:30"]
pub mod adm_proto_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_boolean, krb5_error_code,
                        krb5_pointer, krb5_deltat, krb5_int32};
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/adm_proto.h */
/*
 * Copyright 1995, 2007,2008,2009 by the Massachusetts Institute of Technology.
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
 * This is ugly, but avoids having to include k5-int or kdb.h for this.
 */
        /* KRB5_KDB5__ */
        /* Ditto for admin.h */
        /* KRB5_KDB5__ */
        /*
 * Function prototypes.
 */
        /* logger.c */
        #[no_mangle]
        #[c2rust::src_loc = "50:1"]
        pub fn krb5_klog_init(_: krb5_context, _: *mut libc::c_char,
                              _: *mut libc::c_char, _: krb5_boolean)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "51:1"]
        pub fn krb5_klog_set_context(_: krb5_context);
        #[no_mangle]
        #[c2rust::src_loc = "52:1"]
        pub fn krb5_klog_close(_: krb5_context);
        #[no_mangle]
        #[c2rust::src_loc = "53:1"]
        pub fn krb5_klog_syslog(_: libc::c_int, _: *const libc::c_char,
                                _: ...) -> libc::c_int;
        /* alt_prof.c */
        #[no_mangle]
        #[c2rust::src_loc = "61:1"]
        pub fn krb5_aprof_init(_: *mut libc::c_char, _: *mut libc::c_char,
                               _: *mut krb5_pointer) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "63:1"]
        pub fn krb5_aprof_get_boolean(_: krb5_pointer,
                                      _: *mut *const libc::c_char,
                                      _: libc::c_int, _: *mut krb5_boolean)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "65:1"]
        pub fn krb5_aprof_get_deltat(_: krb5_pointer,
                                     _: *mut *const libc::c_char,
                                     _: krb5_boolean, _: *mut krb5_deltat)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn krb5_aprof_get_string(_: krb5_pointer,
                                     _: *mut *const libc::c_char,
                                     _: krb5_boolean,
                                     _: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "69:1"]
        pub fn krb5_aprof_get_string_all(_: krb5_pointer,
                                         _: *mut *const libc::c_char,
                                         _: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "71:1"]
        pub fn krb5_aprof_get_int32(_: krb5_pointer,
                                    _: *mut *const libc::c_char,
                                    _: krb5_boolean, _: *mut krb5_int32)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "73:1"]
        pub fn krb5_aprof_finish(_: krb5_pointer) -> krb5_error_code;
    }
    /* KRB5_ADM_PROTO_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kdc/kdc_util.h:31"]
pub mod kdc_util_h {
    use super::realm_data_h::server_handle;
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_error_code};
    use super::kdcpreauth_plugin_h::verto_ctx;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "180:1"]
        pub fn load_preauth_plugins(handle: *mut server_handle,
                                    context: krb5_context,
                                    ctx: *mut verto_ctx);
        #[no_mangle]
        #[c2rust::src_loc = "183:1"]
        pub fn unload_preauth_plugins(context: krb5_context);
        /* kdc_authdata.c */
        #[no_mangle]
        #[c2rust::src_loc = "214:1"]
        pub fn load_authdata_plugins(context: krb5_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "216:1"]
        pub fn unload_authdata_plugins(context: krb5_context)
         -> krb5_error_code;
        /* replay.c */
        #[no_mangle]
        #[c2rust::src_loc = "244:1"]
        pub fn kdc_init_lookaside(context: krb5_context) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "248:1"]
        pub fn kdc_free_lookaside(_: krb5_context);
        /* kdc_util.c */
        #[no_mangle]
        #[c2rust::src_loc = "251:1"]
        pub fn reset_for_hangup(_: *mut libc::c_void);
    }
    /* __KRB5_KDC_UTIL__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/net-server.h:31"]
pub mod net_server_h {
    use super::verto_h::{verto_ev_type, VERTO_EV_TYPE_NONE};
    use super::kdcpreauth_plugin_h::verto_ctx;
    use super::krb5_h::krb5_error_code;
    extern "C" {
        /* exported from net-server.c */
        #[no_mangle]
        #[c2rust::src_loc = "46:1"]
        pub fn loop_init(types: verto_ev_type) -> *mut verto_ctx;
        /*
 * Add listener addresses to the loop configuration.
 *
 * Arguments:
 *
 * - default_port
 *      The port for the sockets if not specified in addresses.
 * - addresses
 *      The optional addresses for the listener sockets.  Pass NULL for the
 *      wildcard address.  Addresses may be delimited by the characters in
 *      ADDRESSES_DELIM.  Addresses are parsed with k5_parse_host_string().
 * - prognum, versnum, dispatchfn
 *      For RPC listener sockets, the svc_register() arguments to use when new
 *      TCP connections are created.
 */
        #[no_mangle]
        #[c2rust::src_loc = "63:1"]
        pub fn loop_add_udp_address(default_port: libc::c_int,
                                    addresses: *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn loop_add_tcp_address(default_port: libc::c_int,
                                    addresses: *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "69:1"]
        pub fn loop_setup_network(ctx: *mut verto_ctx,
                                  handle: *mut libc::c_void,
                                  progname: *const libc::c_char,
                                  tcp_listen_backlog: libc::c_int)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "72:1"]
        pub fn loop_setup_signals(ctx: *mut verto_ctx,
                                  handle: *mut libc::c_void,
                                  reset: Option<unsafe extern "C" fn() -> ()>)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn loop_free(ctx: *mut verto_ctx);
    }
    /* NET_SERVER_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kdc/kdc_audit.h:32"]
pub mod kdc_audit_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_boolean, krb5_error_code};
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/kdc_audit.h - KDC-facing API for audit */
/*
 * Copyright 2013 by the Massachusetts Institute of Technology.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * * Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.
 *
 * * Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in
 *   the documentation and/or other materials provided with the
 *   distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 */
        /* Utilities */
        /* KDC-facing audit API */
        #[no_mangle]
        #[c2rust::src_loc = "59:1"]
        pub fn kau_kdc_stop(context: krb5_context, ev_success: krb5_boolean);
        #[no_mangle]
        #[c2rust::src_loc = "39:1"]
        pub fn load_audit_modules(context: krb5_context) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "40:1"]
        pub fn unload_audit_modules(context: krb5_context);
        #[no_mangle]
        #[c2rust::src_loc = "56:1"]
        pub fn kau_kdc_start(context: krb5_context, ev_success: krb5_boolean);
    }
    /* KRB5_KDC_AUDIT__ */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/kdc/extern.h:33"]
pub mod extern_h {
    use super::krb5_h::krb5_int32;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "33:25"]
        pub static mut max_dgram_reply_size: krb5_int32;
    }
    /* __KRB5_KDC_EXTERN__ */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/kdc/policy.h:34"]
pub mod policy_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_error_code};
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kdc/policy.h - Declarations for policy.c */
/*
 * Copyright 1990 by the Massachusetts Institute of Technology.
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
        #[no_mangle]
        #[c2rust::src_loc = "29:1"]
        pub fn load_kdcpolicy_plugins(context: krb5_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "32:1"]
        pub fn unload_kdcpolicy_plugins(context: krb5_context);
    }
    /* __KRB5_KDC_POLICY__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kdc/kdc5_err.h:35"]
pub mod kdc5_err_h {
    extern "C" {
        /* for compatibility with older versions... */
        #[no_mangle]
        #[c2rust::src_loc = "18:1"]
        pub fn initialize_kdc5_error_table();
    }
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kdb_kt.h:36"]
pub mod kdb_kt_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_keytab, krb5_error_code};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "34:1"]
        pub fn krb5_ktkdb_resolve(_: krb5_context, _: *const libc::c_char,
                                  _: *mut krb5_keytab) -> krb5_error_code;
    }
    /* KRB5_KDB5_DBM__ */
}
#[c2rust::header_src = "/usr/include/locale.h:42"]
pub mod locale_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "122:1"]
        pub fn setlocale(__category: libc::c_int,
                         __locale: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/sys/wait.h:48"]
pub mod wait_h {
    use super::types_h::__pid_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "88:1"]
        pub fn wait(__stat_loc: *mut libc::c_int) -> __pid_t;
    }
}
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t, __uid_t, __off_t,
                        __off64_t, __pid_t, __clock_t};
pub use self::sys_types_h::pid_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::__sigset_t_h::__sigset_t;
pub use self::sigset_t_h::sigset_t;
pub use self::stdarg_h::va_list;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::__sigval_t_h::{__sigval_t, sigval};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_boolean,
                       krb5_kvno, krb5_enctype, krb5_flags, krb5_timestamp,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_pointer, krb5_principal_data,
                       krb5_principal, krb5_const_principal,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _krb5_keyblock, krb5_keyblock,
                       C2RustUnnamed, KRB5_C_RANDSOURCE_MAX,
                       KRB5_C_RANDSOURCE_EXTERNAL_PROTOCOL,
                       KRB5_C_RANDSOURCE_TIMING,
                       KRB5_C_RANDSOURCE_TRUSTEDPARTY,
                       KRB5_C_RANDSOURCE_OSRAND, KRB5_C_RANDSOURCE_OLDAPI,
                       krb5_kt_cursor, krb5_keytab_entry_st,
                       krb5_keytab_entry, krb5_keytab, _profile_t,
                       krb5_c_random_add_entropy, krb5_kt_close,
                       krb5_free_context, krb5_build_principal,
                       krb5_free_default_realm, krb5_set_default_realm,
                       krb5_get_default_realm, krb5_free_principal,
                       krb5_string_to_enctype, krb5_enctype_to_name,
                       krb5_copy_error_message, krb5_get_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, _krb5_kt, _krb5_kt_ops, zapfree,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5_set_time_offsets,
                         krb5int_c_deprecated_enctype,
                         krb5int_init_context_kdc};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::com_err_h::{errcode_t, com_err, com_err_va};
pub use self::siginfo_t_h::{siginfo_t, C2RustUnnamed_0, C2RustUnnamed_1,
                            C2RustUnnamed_2, C2RustUnnamed_3, C2RustUnnamed_4,
                            C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7,
                            C2RustUnnamed_8, C2RustUnnamed_9};
pub use self::signal_h::{__sighandler_t, kill, sigemptyset, sigaction};
pub use self::sigaction_h::{sigaction, C2RustUnnamed_10};
use self::kdcpreauth_plugin_h::verto_ctx;
pub use self::realm_data_h::{kdc_realm_t, __kdc_realm_data, server_handle};
pub use self::verto_h::{verto_ev_type, VERTO_EV_TYPE_CHILD,
                        VERTO_EV_TYPE_SIGNAL, VERTO_EV_TYPE_IDLE,
                        VERTO_EV_TYPE_TIMEOUT, VERTO_EV_TYPE_IO,
                        VERTO_EV_TYPE_NONE, verto_reinitialize, verto_run};
use self::string_h::{memset, strncmp, strdup, strrchr, strlen,
                     explicit_bzero};
use self::stdlib_h::{atoi, malloc, calloc, realloc, free, exit};
use self::stdio_h::{stderr, fclose, fopen, fprintf, asprintf};
use self::errno_h::__errno_location;
use self::unistd_h::{getpid, fork, daemon};
use self::getopt_core_h::{optarg, optind, getopt};
use self::libintl_h::dgettext;
use self::kdb_h::{krb5_db_open, krb5_db_fini, krb5_db_fetch_mkey,
                  krb5_db_fetch_mkey_list, krb5_db_setup_mkey_name};
use self::adm_proto_h::{krb5_klog_init, krb5_klog_set_context,
                        krb5_klog_close, krb5_klog_syslog, krb5_aprof_init,
                        krb5_aprof_get_boolean, krb5_aprof_get_deltat,
                        krb5_aprof_get_string, krb5_aprof_get_string_all,
                        krb5_aprof_get_int32, krb5_aprof_finish};
use self::kdc_util_h::{load_preauth_plugins, unload_preauth_plugins,
                       load_authdata_plugins, unload_authdata_plugins,
                       kdc_init_lookaside, kdc_free_lookaside,
                       reset_for_hangup};
use self::net_server_h::{loop_init, loop_add_udp_address,
                         loop_add_tcp_address, loop_setup_network,
                         loop_setup_signals, loop_free};
use self::kdc_audit_h::{kau_kdc_stop, load_audit_modules,
                        unload_audit_modules, kau_kdc_start};
use self::extern_h::max_dgram_reply_size;
use self::policy_h::{load_kdcpolicy_plugins, unload_kdcpolicy_plugins};
use self::kdc5_err_h::initialize_kdc5_error_table;
use self::kdb_kt_h::krb5_ktkdb_resolve;
use self::locale_h::setlocale;
use self::wait_h::wait;
#[c2rust::src_loc = "61:12"]
static mut nofork: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "62:12"]
static mut workers: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "63:12"]
static mut time_offset: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "64:20"]
static mut pid_file: *const libc::c_char = 0 as *const libc::c_char;
#[c2rust::src_loc = "65:12"]
static mut rkey_init_done: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "66:21"]
static mut signal_received: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "67:21"]
static mut sighup_received: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "71:20"]
static mut kdc_progname: *const libc::c_char = 0 as *const libc::c_char;
/*
 * Static server_handle for this file.  Other code will get access to
 * it through the application handle that net-server.c uses.
 */
#[c2rust::src_loc = "77:29"]
static mut shandle: server_handle =
    server_handle{kdc_realmlist:
                      0 as *const *mut kdc_realm_t as *mut *mut kdc_realm_t,
                  kdc_numrealms: 0,
                  kdc_err_context:
                      0 as *const _krb5_context as *mut _krb5_context,};
/*
 * We use krb5_klog_init to set up a com_err callback to log error
 * messages.  The callback also pulls the error message out of the
 * context we pass to krb5_klog_init; however, we use realm-specific
 * contexts for most of our krb5 library calls, so the error message
 * isn't present in the global context.  This wrapper ensures that the
 * error message state from the call context is copied into the
 * context known by krb5_klog.  call_context can be NULL if the error
 * code did not come from a krb5 library function.
 */
#[no_mangle]
#[c2rust::src_loc = "89:1"]
pub unsafe extern "C" fn kdc_err(mut call_context: krb5_context,
                                 mut code: errcode_t,
                                 mut fmt: *const libc::c_char,
                                 mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    if !call_context.is_null() {
        krb5_copy_error_message(shandle.kdc_err_context, call_context);
    }
    ap = args.clone();
    com_err_va(kdc_progname, code, fmt, ap.as_va_list());
}
/*
 * Find the realm entry for a given realm.
 */
#[no_mangle]
#[c2rust::src_loc = "104:1"]
pub unsafe extern "C" fn find_realm_data(mut handle: *mut server_handle,
                                         mut rname: *mut libc::c_char,
                                         mut rsize: krb5_ui_4)
 -> *mut kdc_realm_t {
    let mut i: libc::c_int = 0;
    let mut kdc_realmlist: *mut *mut kdc_realm_t = (*handle).kdc_realmlist;
    let mut kdc_numrealms: libc::c_int = (*handle).kdc_numrealms;
    i = 0 as libc::c_int;
    while i < kdc_numrealms {
        if rsize as libc::c_ulong ==
               strlen((**kdc_realmlist.offset(i as isize)).realm_name) &&
               strncmp(rname, (**kdc_realmlist.offset(i as isize)).realm_name,
                       rsize as libc::c_ulong) == 0 {
            return *kdc_realmlist.offset(i as isize)
        }
        i += 1
    }
    return 0 as *mut libc::c_void as *mut kdc_realm_t;
}
#[no_mangle]
#[c2rust::src_loc = "119:1"]
pub unsafe extern "C" fn setup_server_realm(mut handle: *mut server_handle,
                                            mut sprinc: krb5_principal)
 -> *mut kdc_realm_t {
    let mut newrealm: *mut kdc_realm_t = 0 as *mut kdc_realm_t;
    let mut kdc_realmlist: *mut *mut kdc_realm_t = (*handle).kdc_realmlist;
    let mut kdc_numrealms: libc::c_int = (*handle).kdc_numrealms;
    if sprinc.is_null() { return 0 as *mut kdc_realm_t }
    if kdc_numrealms > 1 as libc::c_int {
        newrealm =
            find_realm_data(handle, (*sprinc).realm.data,
                            (*sprinc).realm.length)
    } else { newrealm = *kdc_realmlist.offset(0 as libc::c_int as isize) }
    if !newrealm.is_null() {
        krb5_klog_set_context((*newrealm).realm_context);
        shandle.kdc_err_context = (*newrealm).realm_context
    }
    return newrealm;
}
#[c2rust::src_loc = "142:1"]
unsafe extern "C" fn finish_realm(mut rdp: *mut kdc_realm_t) {
    if !(*rdp).realm_name.is_null() {
        free((*rdp).realm_name as *mut libc::c_void);
    }
    if !(*rdp).realm_mpname.is_null() {
        free((*rdp).realm_mpname as *mut libc::c_void);
    }
    if !(*rdp).realm_stash.is_null() {
        free((*rdp).realm_stash as *mut libc::c_void);
    }
    if !(*rdp).realm_listen.is_null() {
        free((*rdp).realm_listen as *mut libc::c_void);
    }
    if !(*rdp).realm_tcp_listen.is_null() {
        free((*rdp).realm_tcp_listen as *mut libc::c_void);
    }
    if !(*rdp).realm_keytab.is_null() {
        krb5_kt_close((*rdp).realm_context, (*rdp).realm_keytab);
    }
    if !(*rdp).realm_hostbased.is_null() {
        free((*rdp).realm_hostbased as *mut libc::c_void);
    }
    if !(*rdp).realm_no_referral.is_null() {
        free((*rdp).realm_no_referral as *mut libc::c_void);
    }
    if !(*rdp).realm_context.is_null() {
        if !(*rdp).realm_mprinc.is_null() {
            krb5_free_principal((*rdp).realm_context, (*rdp).realm_mprinc);
        }
        zapfree((*rdp).realm_mkey.contents as *mut libc::c_void,
                (*rdp).realm_mkey.length as size_t);
        krb5_db_fini((*rdp).realm_context);
        if !(*rdp).realm_tgsprinc.is_null() {
            krb5_free_principal((*rdp).realm_context, (*rdp).realm_tgsprinc);
        }
        krb5_free_context((*rdp).realm_context);
    }
    zapfree(rdp as *mut libc::c_void,
            ::std::mem::size_of::<kdc_realm_t>() as libc::c_ulong);
}
/* Set *val_out to an allocated string containing val1 and/or val2, separated
 * by a space if both are set, or NULL if neither is set. */
#[c2rust::src_loc = "175:1"]
unsafe extern "C" fn combine(mut val1: *const libc::c_char,
                             mut val2: *const libc::c_char,
                             mut val_out: *mut *mut libc::c_char)
 -> krb5_error_code {
    if val1.is_null() && val2.is_null() {
        *val_out = 0 as *mut libc::c_char
    } else if !val1.is_null() && !val2.is_null() {
        if asprintf(val_out, b"%s %s\x00" as *const u8 as *const libc::c_char,
                    val1, val2) < 0 as libc::c_int {
            *val_out = 0 as *mut libc::c_char;
            return 12 as libc::c_int
        }
    } else {
        *val_out = strdup(if !val1.is_null() { val1 } else { val2 });
        if (*val_out).is_null() { return 12 as libc::c_int }
    }
    return 0 as libc::c_int;
}
/*
 * Initialize a realm control structure from the alternate profile or from
 * the specified defaults.
 *
 * After we're complete here, the essence of the realm is embodied in the
 * realm data and we should be all set to begin operation for that realm.
 */
#[c2rust::src_loc = "200:1"]
unsafe extern "C" fn init_realm(mut rdp: *mut kdc_realm_t,
                                mut aprof: krb5_pointer,
                                mut realm: *mut libc::c_char,
                                mut def_mpname: *mut libc::c_char,
                                mut def_enctype: krb5_enctype,
                                mut def_udp_listen: *mut libc::c_char,
                                mut def_tcp_listen: *mut libc::c_char,
                                mut def_manual: krb5_boolean,
                                mut def_restrict_anon: krb5_boolean,
                                mut db_args: *mut *mut libc::c_char,
                                mut no_referral: *mut libc::c_char,
                                mut hostbased: *mut libc::c_char)
 -> krb5_error_code {
    let mut kret: krb5_error_code = 0;
    let mut manual: krb5_boolean = 0;
    let mut kdb_open_flags: libc::c_int = 0;
    let mut svalue: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hierarchy: [*const libc::c_char; 4] =
        [0 as *const libc::c_char; 4];
    let mut mkvno: krb5_kvno = 0 as libc::c_int as krb5_kvno;
    let mut ename: [libc::c_char; 32] = [0; 32];
    memset(rdp as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<kdc_realm_t>() as libc::c_ulong);
    if realm.is_null() {
        kret = 22 as libc::c_int
    } else {
        if def_enctype != 0x1ff as libc::c_int &&
               krb5int_c_deprecated_enctype(def_enctype) != 0 {
            if krb5_enctype_to_name(def_enctype,
                                    0 as libc::c_int as krb5_boolean,
                                    ename.as_mut_ptr(),
                                    ::std::mem::size_of::<[libc::c_char; 32]>()
                                        as libc::c_ulong) != 0 {
                ename[0 as libc::c_int as usize] =
                    '\u{0}' as i32 as libc::c_char
            }
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"Requested master password enctype %s in %s is DEPRECATED!\n\x00"
                                 as *const u8 as *const libc::c_char),
                    ename.as_mut_ptr(), realm);
        }
        hierarchy[0 as libc::c_int as usize] =
            b"realms\x00" as *const u8 as *const libc::c_char;
        hierarchy[1 as libc::c_int as usize] = realm;
        hierarchy[3 as libc::c_int as usize] = 0 as *const libc::c_char;
        (*rdp).realm_name = strdup(realm);
        if (*rdp).realm_name.is_null() {
            kret = 12 as libc::c_int
        } else {
            kret = krb5int_init_context_kdc(&mut (*rdp).realm_context);
            if kret != 0 {
                kdc_err(0 as krb5_context, kret as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while getting context for realm %s\x00" as
                                     *const u8 as *const libc::c_char),
                        realm);
            } else {
                if time_offset != 0 as libc::c_int {
                    krb5_set_time_offsets((*rdp).realm_context, time_offset,
                                          0 as libc::c_int);
                }
                /* Handle master key name */
                hierarchy[2 as libc::c_int as usize] =
                    b"master_key_name\x00" as *const u8 as
                        *const libc::c_char;
                if krb5_aprof_get_string(aprof, hierarchy.as_mut_ptr(),
                                         1 as libc::c_int as krb5_boolean,
                                         &mut (*rdp).realm_mpname) != 0 {
                    (*rdp).realm_mpname =
                        if !def_mpname.is_null() {
                            strdup(def_mpname)
                        } else {
                            strdup(b"K/M\x00" as *const u8 as
                                       *const libc::c_char)
                        }
                }
                if (*rdp).realm_mpname.is_null() {
                    kret = 12 as libc::c_int
                } else {
                    /* Handle KDC addresses/ports */
                    hierarchy[2 as libc::c_int as usize] =
                        b"kdc_listen\x00" as *const u8 as *const libc::c_char;
                    if krb5_aprof_get_string(aprof, hierarchy.as_mut_ptr(),
                                             1 as libc::c_int as krb5_boolean,
                                             &mut (*rdp).realm_listen) != 0 {
                        /* Try the old kdc_ports configuration option. */
                        hierarchy[2 as libc::c_int as usize] =
                            b"kdc_ports\x00" as *const u8 as
                                *const libc::c_char;
                        if krb5_aprof_get_string(aprof,
                                                 hierarchy.as_mut_ptr(),
                                                 1 as libc::c_int as
                                                     krb5_boolean,
                                                 &mut (*rdp).realm_listen) !=
                               0 {
                            (*rdp).realm_listen = strdup(def_udp_listen)
                        }
                    }
                    if (*rdp).realm_listen.is_null() {
                        kret = 12 as libc::c_int
                    } else {
                        hierarchy[2 as libc::c_int as usize] =
                            b"kdc_tcp_listen\x00" as *const u8 as
                                *const libc::c_char;
                        if krb5_aprof_get_string(aprof,
                                                 hierarchy.as_mut_ptr(),
                                                 1 as libc::c_int as
                                                     krb5_boolean,
                                                 &mut (*rdp).realm_tcp_listen)
                               != 0 {
                            /* Try the old kdc_tcp_ports configuration option. */
                            hierarchy[2 as libc::c_int as usize] =
                                b"kdc_tcp_ports\x00" as *const u8 as
                                    *const libc::c_char;
                            if krb5_aprof_get_string(aprof,
                                                     hierarchy.as_mut_ptr(),
                                                     1 as libc::c_int as
                                                         krb5_boolean,
                                                     &mut (*rdp).realm_tcp_listen)
                                   != 0 {
                                (*rdp).realm_tcp_listen =
                                    strdup(def_tcp_listen)
                            }
                        }
                        if (*rdp).realm_tcp_listen.is_null() {
                            kret = 12 as libc::c_int
                        } else {
                            /* Handle stash file */
                            hierarchy[2 as libc::c_int as usize] =
                                b"key_stash_file\x00" as *const u8 as
                                    *const libc::c_char;
                            if krb5_aprof_get_string(aprof,
                                                     hierarchy.as_mut_ptr(),
                                                     1 as libc::c_int as
                                                         krb5_boolean,
                                                     &mut (*rdp).realm_stash)
                                   != 0 {
                                manual = def_manual
                            } else {
                                manual = 0 as libc::c_int as krb5_boolean
                            }
                            hierarchy[2 as libc::c_int as usize] =
                                b"restrict_anonymous_to_tgt\x00" as *const u8
                                    as *const libc::c_char;
                            if krb5_aprof_get_boolean(aprof,
                                                      hierarchy.as_mut_ptr(),
                                                      1 as libc::c_int,
                                                      &mut (*rdp).realm_restrict_anon)
                                   != 0 {
                                (*rdp).realm_restrict_anon = def_restrict_anon
                            }
                            /* Handle master key type */
                            hierarchy[2 as libc::c_int as usize] =
                                b"master_key_type\x00" as *const u8 as
                                    *const libc::c_char;
                            if krb5_aprof_get_string(aprof,
                                                     hierarchy.as_mut_ptr(),
                                                     1 as libc::c_int as
                                                         krb5_boolean,
                                                     &mut svalue) != 0 ||
                                   krb5_string_to_enctype(svalue,
                                                          &mut (*rdp).realm_mkey.enctype)
                                       != 0 {
                                (*rdp).realm_mkey.enctype =
                                    if manual != 0 {
                                        def_enctype
                                    } else { 0x1ff as libc::c_int }
                            }
                            free(svalue as *mut libc::c_void);
                            svalue = 0 as *mut libc::c_char;
                            /* Handle reject-bad-transit flag */
                            hierarchy[2 as libc::c_int as usize] =
                                b"reject_bad_transit\x00" as *const u8 as
                                    *const libc::c_char;
                            if krb5_aprof_get_boolean(aprof,
                                                      hierarchy.as_mut_ptr(),
                                                      1 as libc::c_int,
                                                      &mut (*rdp).realm_reject_bad_transit)
                                   != 0 {
                                (*rdp).realm_reject_bad_transit =
                                    1 as libc::c_int as krb5_boolean
                            }
                            /* Handle ticket maximum life */
                            hierarchy[2 as libc::c_int as usize] =
                                b"max_life\x00" as *const u8 as
                                    *const libc::c_char;
                            if krb5_aprof_get_deltat(aprof,
                                                     hierarchy.as_mut_ptr(),
                                                     1 as libc::c_int as
                                                         krb5_boolean,
                                                     &mut (*rdp).realm_maxlife)
                                   != 0 {
                                (*rdp).realm_maxlife =
                                    60 as libc::c_int * 60 as libc::c_int *
                                        24 as libc::c_int
                            }
                            /* Handle ticket renewable maximum life */
                            hierarchy[2 as libc::c_int as usize] =
                                b"max_renewable_life\x00" as *const u8 as
                                    *const libc::c_char;
                            if krb5_aprof_get_deltat(aprof,
                                                     hierarchy.as_mut_ptr(),
                                                     1 as libc::c_int as
                                                         krb5_boolean,
                                                     &mut (*rdp).realm_maxrlife)
                                   != 0 {
                                (*rdp).realm_maxrlife =
                                    60 as libc::c_int * 60 as libc::c_int *
                                        24 as libc::c_int * 7 as libc::c_int
                            }
                            /* Handle KDC referrals */
                            hierarchy[2 as libc::c_int as usize] =
                                b"no_host_referral\x00" as *const u8 as
                                    *const libc::c_char;
                            krb5_aprof_get_string_all(aprof,
                                                      hierarchy.as_mut_ptr(),
                                                      &mut svalue);
                            kret =
                                combine(no_referral, svalue,
                                        &mut (*rdp).realm_no_referral);
                            if !(kret != 0) {
                                free(svalue as *mut libc::c_void);
                                svalue = 0 as *mut libc::c_char;
                                hierarchy[2 as libc::c_int as usize] =
                                    b"host_based_services\x00" as *const u8 as
                                        *const libc::c_char;
                                krb5_aprof_get_string_all(aprof,
                                                          hierarchy.as_mut_ptr(),
                                                          &mut svalue);
                                kret =
                                    combine(hostbased, svalue,
                                            &mut (*rdp).realm_hostbased);
                                if !(kret != 0) {
                                    free(svalue as *mut libc::c_void);
                                    svalue = 0 as *mut libc::c_char;
                                    /*
     * We've got our parameters, now go and setup our realm context.
     */
                                    /* Set the default realm of this context */
                                    kret =
                                        krb5_set_default_realm((*rdp).realm_context,
                                                               realm);
                                    if kret != 0 {
                                        kdc_err((*rdp).realm_context,
                                                kret as errcode_t,
                                                dgettext(b"mit-krb5\x00" as
                                                             *const u8 as
                                                             *const libc::c_char,
                                                         b"while setting default realm to %s\x00"
                                                             as *const u8 as
                                                             *const libc::c_char),
                                                realm);
                                    } else {
                                        /* first open the database  before doing anything */
                                        kdb_open_flags =
                                            0 as libc::c_int |
                                                0x100 as libc::c_int;
                                        kret =
                                            krb5_db_open((*rdp).realm_context,
                                                         db_args,
                                                         kdb_open_flags);
                                        if kret != 0 {
                                            kdc_err((*rdp).realm_context,
                                                    kret as errcode_t,
                                                    dgettext(b"mit-krb5\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             b"while initializing database for realm %s\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char),
                                                    realm);
                                        } else {
                                            /* Assemble and parse the master key name */
                                            kret =
                                                krb5_db_setup_mkey_name((*rdp).realm_context,
                                                                        (*rdp).realm_mpname,
                                                                        (*rdp).realm_name,
                                                                        0 as
                                                                            *mut libc::c_void
                                                                            as
                                                                            *mut *mut libc::c_char,
                                                                        &mut (*rdp).realm_mprinc);
                                            if kret != 0 {
                                                kdc_err((*rdp).realm_context,
                                                        kret as errcode_t,
                                                        dgettext(b"mit-krb5\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"while setting up master key name %s for realm %s\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char),
                                                        (*rdp).realm_mpname,
                                                        realm);
                                            } else {
                                                /*
     * Get the master key (note, may not be the most current mkey).
     */
                                                kret =
                                                    krb5_db_fetch_mkey((*rdp).realm_context,
                                                                       (*rdp).realm_mprinc,
                                                                       (*rdp).realm_mkey.enctype,
                                                                       manual,
                                                                       0 as
                                                                           libc::c_int
                                                                           as
                                                                           krb5_boolean,
                                                                       (*rdp).realm_stash,
                                                                       &mut mkvno,
                                                                       0 as
                                                                           *mut krb5_data,
                                                                       &mut (*rdp).realm_mkey);
                                                if kret != 0 {
                                                    kdc_err((*rdp).realm_context,
                                                            kret as errcode_t,
                                                            dgettext(b"mit-krb5\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     b"while fetching master key %s for realm %s\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char),
                                                            (*rdp).realm_mpname,
                                                            realm);
                                                } else {
                                                    if krb5int_c_deprecated_enctype((*rdp).realm_mkey.enctype)
                                                           != 0 {
                                                        if krb5_enctype_to_name((*rdp).realm_mkey.enctype,
                                                                                0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    krb5_boolean,
                                                                                ename.as_mut_ptr(),
                                                                                ::std::mem::size_of::<[libc::c_char; 32]>()
                                                                                    as
                                                                                    libc::c_ulong)
                                                               != 0 {
                                                            ename[0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize] =
                                                                '\u{0}' as i32
                                                                    as
                                                                    libc::c_char
                                                        }
                                                        fprintf(stderr,
                                                                dgettext(b"mit-krb5\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         b"Stash file %s uses DEPRECATED enctype %s!\n\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char),
                                                                (*rdp).realm_stash,
                                                                ename.as_mut_ptr());
                                                    }
                                                    kret =
                                                        krb5_db_fetch_mkey_list((*rdp).realm_context,
                                                                                (*rdp).realm_mprinc,
                                                                                &mut (*rdp).realm_mkey);
                                                    if kret != 0 {
                                                        kdc_err((*rdp).realm_context,
                                                                kret as
                                                                    errcode_t,
                                                                dgettext(b"mit-krb5\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         b"while fetching master keys list for realm %s\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char),
                                                                realm);
                                                    } else {
                                                        /* Set up the keytab */
                                                        kret =
                                                            krb5_ktkdb_resolve((*rdp).realm_context,
                                                                               0
                                                                                   as
                                                                                   *const libc::c_char,
                                                                               &mut (*rdp).realm_keytab);
                                                        if kret != 0 {
                                                            kdc_err((*rdp).realm_context,
                                                                    kret as
                                                                        errcode_t,
                                                                    dgettext(b"mit-krb5\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char,
                                                                             b"while resolving kdb keytab for realm %s\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char),
                                                                    realm);
                                                        } else {
                                                            /* Preformat the TGS name */
                                                            kret =
                                                                krb5_build_principal((*rdp).realm_context,
                                                                                     &mut (*rdp).realm_tgsprinc
                                                                                         as
                                                                                         *mut krb5_principal,
                                                                                     strlen(realm)
                                                                                         as
                                                                                         libc::c_uint,
                                                                                     realm,
                                                                                     b"krbtgt\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     realm,
                                                                                     0
                                                                                         as
                                                                                         *mut libc::c_void
                                                                                         as
                                                                                         *mut libc::c_char);
                                                            if kret != 0 {
                                                                kdc_err((*rdp).realm_context,
                                                                        kret
                                                                            as
                                                                            errcode_t,
                                                                        dgettext(b"mit-krb5\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"while building TGS name for realm %s\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char),
                                                                        realm);
                                                            } else if rkey_init_done
                                                                          == 0
                                                             {
                                                                let mut seed:
                                                                        krb5_data =
                                                                    krb5_data{magic:
                                                                                  0,
                                                                              length:
                                                                                  0,
                                                                              data:
                                                                                  0
                                                                                      as
                                                                                      *mut libc::c_char,};
                                                                /*
         * If all that worked, then initialize the random key
         * generators.
         */
                                                                seed.length =
                                                                    (*rdp).realm_mkey.length;
                                                                seed.data =
                                                                    (*rdp).realm_mkey.contents
                                                                        as
                                                                        *mut libc::c_char;
                                                                kret =
                                                                    krb5_c_random_add_entropy((*rdp).realm_context,
                                                                                              KRB5_C_RANDSOURCE_TRUSTEDPARTY
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  libc::c_uint,
                                                                                              &mut seed);
                                                                if !(kret !=
                                                                         0) {
                                                                    rkey_init_done
                                                                        =
                                                                        1 as
                                                                            libc::c_int
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
            }
        }
    }
    /*
     * If we choked, then clean up any dirt we may have dropped on the floor.
     */
    if kret != 0 { finish_realm(rdp); }
    return kret;
}
#[c2rust::src_loc = "439:1"]
unsafe extern "C" fn on_monitor_signal(mut signo: libc::c_int) {
    ::std::ptr::write_volatile(&mut signal_received as *mut libc::c_int,
                               signo);
}
#[c2rust::src_loc = "451:1"]
unsafe extern "C" fn on_monitor_sighup(mut signo: libc::c_int) {
    ::std::ptr::write_volatile(&mut sighup_received as *mut libc::c_int,
                               1 as libc::c_int);
}
/*
 * Kill the worker subprocesses given by pids[0..bound-1], skipping any which
 * are set to -1, and wait for them to exit (so that we know the ports are no
 * longer in use).
 */
#[c2rust::src_loc = "468:1"]
unsafe extern "C" fn terminate_workers(mut pids: *mut pid_t,
                                       mut bound: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut num_active: libc::c_int = 0 as libc::c_int;
    let mut pid: pid_t = 0;
    /* Kill the active worker pids. */
    i = 0 as libc::c_int;
    while i < bound {
        if !(*pids.offset(i as isize) == -(1 as libc::c_int)) {
            kill(*pids.offset(i as isize), 15 as libc::c_int);
            num_active += 1
        }
        i += 1
    }
    /* Wait for them to exit. */
    while num_active > 0 as libc::c_int {
        pid = wait(&mut status);
        if pid >= 0 as libc::c_int { num_active -= 1 }
    };
}
/*
 * Create num worker processes and return successfully in each child.  The
 * parent process will act as a supervisor and will only return from this
 * function in error cases.
 */
#[c2rust::src_loc = "495:1"]
unsafe extern "C" fn create_workers(mut ctx: *mut verto_ctx,
                                    mut num: libc::c_int) -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut i: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut pid: pid_t = 0;
    let mut pids: *mut pid_t = 0 as *mut pid_t;
    let mut s_action: sigaction =
        sigaction{__sigaction_handler: C2RustUnnamed_10{sa_handler: None,},
                  sa_mask: __sigset_t{__val: [0; 16],},
                  sa_flags: 0,
                  sa_restorer: None,};
    /* POSIX_SIGNALS */
    /*
     * Setup our signal handlers which will forward to the children.
     * These handlers will be overriden in the child processes.
     */
    sigemptyset(&mut s_action.sa_mask);
    s_action.sa_flags = 0 as libc::c_int;
    s_action.__sigaction_handler.sa_handler =
        Some(on_monitor_signal as unsafe extern "C" fn(_: libc::c_int) -> ());
    sigaction(2 as libc::c_int, &mut s_action,
              0 as *mut libc::c_void as *mut sigaction);
    sigaction(15 as libc::c_int, &mut s_action,
              0 as *mut libc::c_void as *mut sigaction);
    sigaction(3 as libc::c_int, &mut s_action,
              0 as *mut libc::c_void as *mut sigaction);
    s_action.__sigaction_handler.sa_handler =
        Some(on_monitor_sighup as unsafe extern "C" fn(_: libc::c_int) -> ());
    sigaction(1 as libc::c_int, &mut s_action,
              0 as *mut libc::c_void as *mut sigaction);
    /* POSIX_SIGNALS */
    /* POSIX_SIGNALS */
    /* Create child worker processes; return in each child. */
    krb5_klog_syslog(6 as libc::c_int,
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"creating %d worker processes\x00" as *const u8
                                  as *const libc::c_char), num);
    pids =
        calloc(num as libc::c_ulong,
               ::std::mem::size_of::<pid_t>() as libc::c_ulong) as *mut pid_t;
    if pids.is_null() { return 12 as libc::c_int }
    i = 0 as libc::c_int;
    while i < num {
        pid = fork();
        if pid == 0 as libc::c_int {
            free(pids as *mut libc::c_void);
            if verto_reinitialize(ctx) == 0 {
                krb5_klog_syslog(3 as libc::c_int,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"Unable to reinitialize main loop\x00"
                                              as *const u8 as
                                              *const libc::c_char));
                return 12 as libc::c_int
            }
            retval =
                loop_setup_signals(ctx,
                                   &mut shandle as *mut server_handle as
                                       *mut libc::c_void,
                                   ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                           *mut libc::c_void)
                                                                      -> ()>,
                                                           Option<unsafe extern "C" fn()
                                                                      ->
                                                                          ()>>(Some(reset_for_hangup
                                                                                        as
                                                                                        unsafe extern "C" fn(_:
                                                                                                                 *mut libc::c_void)
                                                                                            ->
                                                                                                ())));
            if retval != 0 {
                krb5_klog_syslog(3 as libc::c_int,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"Unable to initialize signal handlers in pid %d\x00"
                                              as *const u8 as
                                              *const libc::c_char), pid);
                return retval
            }
            /* Avoid race condition */
            if signal_received != 0 { exit(0 as libc::c_int); }
            /* Return control to main() in the new worker process. */
            return 0 as libc::c_int
        }
        if pid == -(1 as libc::c_int) {
            /* Couldn't fork enough times. */
            status = *__errno_location();
            terminate_workers(pids, i);
            free(pids as *mut libc::c_void);
            return status
        }
        *pids.offset(i as isize) = pid;
        i += 1
    }
    /* We're going to use our own main loop here. */
    loop_free(ctx);
    /* Supervise the worker processes. */
    while signal_received == 0 {
        /* Wait until a worker process exits or we get a signal. */
        pid = wait(&mut status);
        if pid >= 0 as libc::c_int {
            krb5_klog_syslog(3 as libc::c_int,
                             dgettext(b"mit-krb5\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"worker %ld exited with status %d\x00"
                                          as *const u8 as
                                          *const libc::c_char),
                             pid as libc::c_long, status);
            /* Remove the pid from the table. */
            i = 0 as libc::c_int;
            while i < num {
                if *pids.offset(i as isize) == pid {
                    *pids.offset(i as isize) = -(1 as libc::c_int)
                }
                i += 1
            }
            break ;
        } else if sighup_received != 0 {
            ::std::ptr::write_volatile(&mut sighup_received as
                                           *mut libc::c_int,
                                       0 as libc::c_int);
            i = 0 as libc::c_int;
            while i < num {
                if *pids.offset(i as isize) != -(1 as libc::c_int) {
                    kill(*pids.offset(i as isize), 1 as libc::c_int);
                }
                i += 1
            }
        }
    }
    if signal_received != 0 {
        krb5_klog_syslog(6 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"signal %d received in supervisor\x00" as
                                      *const u8 as *const libc::c_char),
                         signal_received);
    }
    terminate_workers(pids, num);
    free(pids as *mut libc::c_void);
    exit(0 as libc::c_int);
}
/* Propagate HUP signal to worker processes if we received one. */
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kdc/main.c - Main procedure body for the KDC server process */
/*
 * Copyright 1990,2001,2008,2009,2016 by the Massachusetts Institute of
 * Technology.
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
#[c2rust::src_loc = "603:1"]
unsafe extern "C" fn usage(mut name: *mut libc::c_char) {
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"usage: %s [-x db_args]* [-d dbpathname] [-r dbrealmname]\n\t\t[-R replaycachename] [-m] [-k masterenctype]\n\t\t[-M masterkeyname] [-p port] [-P pid_file]\n\t\t[-n] [-w numworkers] [/]\n\nwhere,\n\t[-x db_args]* - Any number of database specific arguments.\n\t\t\tLook at each database module documentation for \t\t\tsupported arguments\n\x00"
                         as *const u8 as *const libc::c_char), name);
    exit(1 as libc::c_int);
}
#[c2rust::src_loc = "620:1"]
unsafe extern "C" fn initialize_realms(mut kcontext: krb5_context,
                                       mut argc: libc::c_int,
                                       mut argv: *mut *mut libc::c_char,
                                       mut tcp_listen_backlog_out:
                                           *mut libc::c_int) {
    let mut c: libc::c_int = 0;
    let mut db_name: *mut libc::c_char =
        0 as *mut libc::c_void as *mut libc::c_char;
    let mut lrealm: *mut libc::c_char =
        0 as *mut libc::c_void as *mut libc::c_char;
    let mut mkey_name: *mut libc::c_char =
        0 as *mut libc::c_void as *mut libc::c_char;
    let mut retval: krb5_error_code = 0;
    let mut menctype: krb5_enctype = 0x1ff as libc::c_int;
    let mut rdatap: *mut kdc_realm_t = 0 as *mut kdc_realm_t;
    let mut manual: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut def_restrict_anon: krb5_boolean = 0;
    let mut def_udp_listen: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut def_tcp_listen: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut aprof: krb5_pointer = 0 as *mut libc::c_void;
    let mut hierarchy: [*const libc::c_char; 3] =
        [0 as *const libc::c_char; 3];
    let mut no_referral: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hostbased: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut db_args_size: libc::c_int = 0 as libc::c_int;
    let mut db_args: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    extern "C" {
        #[link_name = "optarg"]
        pub static mut optarg_0: *mut libc::c_char;
    }
    if krb5_aprof_init(b"/usr/local/var/krb5kdc/kdc.conf\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char,
                       b"KRB5_KDC_PROFILE\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char,
                       &mut aprof) == 0 {
        hierarchy[0 as libc::c_int as usize] =
            b"kdcdefaults\x00" as *const u8 as *const libc::c_char;
        hierarchy[1 as libc::c_int as usize] =
            b"kdc_listen\x00" as *const u8 as *const libc::c_char;
        hierarchy[2 as libc::c_int as usize] =
            0 as *mut libc::c_void as *mut libc::c_char;
        if krb5_aprof_get_string(aprof, hierarchy.as_mut_ptr(),
                                 1 as libc::c_int as krb5_boolean,
                                 &mut def_udp_listen) != 0 {
            hierarchy[1 as libc::c_int as usize] =
                b"kdc_ports\x00" as *const u8 as *const libc::c_char;
            if krb5_aprof_get_string(aprof, hierarchy.as_mut_ptr(),
                                     1 as libc::c_int as krb5_boolean,
                                     &mut def_udp_listen) != 0 {
                def_udp_listen = 0 as *mut libc::c_char
            }
        }
        hierarchy[1 as libc::c_int as usize] =
            b"kdc_tcp_listen\x00" as *const u8 as *const libc::c_char;
        if krb5_aprof_get_string(aprof, hierarchy.as_mut_ptr(),
                                 1 as libc::c_int as krb5_boolean,
                                 &mut def_tcp_listen) != 0 {
            hierarchy[1 as libc::c_int as usize] =
                b"kdc_tcp_ports\x00" as *const u8 as *const libc::c_char;
            if krb5_aprof_get_string(aprof, hierarchy.as_mut_ptr(),
                                     1 as libc::c_int as krb5_boolean,
                                     &mut def_tcp_listen) != 0 {
                def_tcp_listen = 0 as *mut libc::c_char
            }
        }
        hierarchy[1 as libc::c_int as usize] =
            b"kdc_max_dgram_reply_size\x00" as *const u8 as
                *const libc::c_char;
        if krb5_aprof_get_int32(aprof, hierarchy.as_mut_ptr(),
                                1 as libc::c_int as krb5_boolean,
                                &mut max_dgram_reply_size) != 0 {
            max_dgram_reply_size = 65536 as libc::c_int
        }
        if !tcp_listen_backlog_out.is_null() {
            hierarchy[1 as libc::c_int as usize] =
                b"kdc_tcp_listen_backlog\x00" as *const u8 as
                    *const libc::c_char;
            if krb5_aprof_get_int32(aprof, hierarchy.as_mut_ptr(),
                                    1 as libc::c_int as krb5_boolean,
                                    tcp_listen_backlog_out) != 0 {
                *tcp_listen_backlog_out = 5 as libc::c_int
            }
        }
        hierarchy[1 as libc::c_int as usize] =
            b"restrict_anonymous_to_tgt\x00" as *const u8 as
                *const libc::c_char;
        if krb5_aprof_get_boolean(aprof, hierarchy.as_mut_ptr(),
                                  1 as libc::c_int, &mut def_restrict_anon) !=
               0 {
            def_restrict_anon = 0 as libc::c_int as krb5_boolean
        }
        hierarchy[1 as libc::c_int as usize] =
            b"no_host_referral\x00" as *const u8 as *const libc::c_char;
        if krb5_aprof_get_string_all(aprof, hierarchy.as_mut_ptr(),
                                     &mut no_referral) != 0 {
            no_referral = 0 as *mut libc::c_char
        }
        hierarchy[1 as libc::c_int as usize] =
            b"host_based_services\x00" as *const u8 as *const libc::c_char;
        if krb5_aprof_get_string_all(aprof, hierarchy.as_mut_ptr(),
                                     &mut hostbased) != 0 {
            hostbased = 0 as *mut libc::c_char
        }
    }
    if def_udp_listen.is_null() {
        def_udp_listen =
            strdup(b"88\x00" as *const u8 as *const libc::c_char);
        if def_udp_listen.is_null() {
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b" KDC cannot initialize. Not enough memory\n\x00"
                                 as *const u8 as *const libc::c_char));
            exit(1 as libc::c_int);
        }
    }
    if def_tcp_listen.is_null() {
        def_tcp_listen =
            strdup(b"88\x00" as *const u8 as *const libc::c_char);
        if def_tcp_listen.is_null() {
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b" KDC cannot initialize. Not enough memory\n\x00"
                                 as *const u8 as *const libc::c_char));
            exit(1 as libc::c_int);
        }
    }
    /*
     * Loop through the option list.  Each time we encounter a realm name, use
     * the previously scanned options to fill in for defaults.  We do this
     * twice if worker processes are used, so we must initialize optind.
     */
    optind = 1 as libc::c_int; /* one for NULL */
    loop  {
        c =
            getopt(argc, argv,
                   b"x:r:d:mM:k:R:e:P:p:s:nw:4:T:X3\x00" as *const u8 as
                       *const libc::c_char);
        if !(c != -(1 as libc::c_int)) { break ; }
        match c {
            120 => {
                db_args_size += 1;
                let mut temp: *mut *mut libc::c_char =
                    realloc(db_args as *mut libc::c_void,
                            (::std::mem::size_of::<*mut libc::c_char>() as
                                 libc::c_ulong).wrapping_mul((db_args_size +
                                                                  1 as
                                                                      libc::c_int)
                                                                 as
                                                                 libc::c_ulong))
                        as *mut *mut libc::c_char;
                if temp.is_null() {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"%s: KDC cannot initialize. Not enough memory\n\x00"
                                         as *const u8 as *const libc::c_char),
                            *argv.offset(0 as libc::c_int as isize));
                    exit(1 as libc::c_int);
                }
                db_args = temp;
                let ref mut fresh0 =
                    *db_args.offset((db_args_size - 1 as libc::c_int) as
                                        isize);
                *fresh0 = optarg;
                let ref mut fresh1 = *db_args.offset(db_args_size as isize);
                *fresh1 = 0 as *mut libc::c_char
            }
            114 => {
                /* realm name for db */
                if find_realm_data(&mut shandle, optarg,
                                   strlen(optarg) as krb5_ui_4).is_null() {
                    rdatap =
                        malloc(::std::mem::size_of::<kdc_realm_t>() as
                                   libc::c_ulong) as *mut kdc_realm_t;
                    if !rdatap.is_null() {
                        retval =
                            init_realm(rdatap, aprof, optarg, mkey_name,
                                       menctype, def_udp_listen,
                                       def_tcp_listen, manual,
                                       def_restrict_anon, db_args,
                                       no_referral, hostbased);
                        if retval != 0 {
                            fprintf(stderr,
                                    dgettext(b"mit-krb5\x00" as *const u8 as
                                                 *const libc::c_char,
                                             b"%s: cannot initialize realm %s - see log file for details\n\x00"
                                                 as *const u8 as
                                                 *const libc::c_char),
                                    *argv.offset(0 as libc::c_int as isize),
                                    optarg);
                            exit(1 as libc::c_int);
                        }
                        let ref mut fresh2 =
                            *shandle.kdc_realmlist.offset(shandle.kdc_numrealms
                                                              as isize);
                        *fresh2 = rdatap;
                        shandle.kdc_numrealms += 1;
                        free(db_args as *mut libc::c_void);
                        db_args = 0 as *mut *mut libc::c_char;
                        db_args_size = 0 as libc::c_int
                    } else {
                        fprintf(stderr,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"%s: cannot initialize realm %s. Not enough memory\n\x00"
                                             as *const u8 as
                                             *const libc::c_char),
                                *argv.offset(0 as libc::c_int as isize),
                                optarg);
                        exit(1 as libc::c_int);
                    }
                }
            }
            100 => {
                /* pathname for db */
                /* now db_name is not a separate argument.
             * It has to be passed as part of the db_args
             */
                if db_name.is_null() {
                    if asprintf(&mut db_name as *mut *mut libc::c_char,
                                b"dbname=%s\x00" as *const u8 as
                                    *const libc::c_char, optarg) <
                           0 as libc::c_int {
                        fprintf(stderr,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"%s: KDC cannot initialize. Not enough memory\n\x00"
                                             as *const u8 as
                                             *const libc::c_char),
                                *argv.offset(0 as libc::c_int as
                                                 isize)); /* one for NULL */
                        exit(1 as libc::c_int);
                    }
                }
                db_args_size += 1;
                let mut temp_0: *mut *mut libc::c_char =
                    realloc(db_args as *mut libc::c_void,
                            (::std::mem::size_of::<*mut libc::c_char>() as
                                 libc::c_ulong).wrapping_mul((db_args_size +
                                                                  1 as
                                                                      libc::c_int)
                                                                 as
                                                                 libc::c_ulong))
                        as *mut *mut libc::c_char;
                if temp_0.is_null() {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"%s: KDC cannot initialize. Not enough memory\n\x00"
                                         as *const u8 as *const libc::c_char),
                            *argv.offset(0 as libc::c_int as isize));
                    exit(1 as libc::c_int);
                }
                db_args = temp_0;
                let ref mut fresh3 =
                    *db_args.offset((db_args_size - 1 as libc::c_int) as
                                        isize);
                *fresh3 = db_name;
                let ref mut fresh4 = *db_args.offset(db_args_size as isize);
                *fresh4 = 0 as *mut libc::c_char
            }
            109 => {
                /* manual type-in of master key */
                manual = 1 as libc::c_int as krb5_boolean;
                if menctype == 0x1ff as libc::c_int {
                    menctype = 0x12 as libc::c_int
                }
            }
            77 => {
                /* master key name in DB */
                mkey_name = optarg
            }
            110 => { nofork += 1 }
            119 => { /* don't detach from terminal */
                /* create multiple worker processes */
                workers = atoi(optarg);
                if workers <= 0 as libc::c_int {
                    usage(*argv.offset(0 as libc::c_int as isize));
                }
            }
            107 => {
                /* enctype for master key */
                if krb5_string_to_enctype(optarg, &mut menctype) != 0 {
                    com_err(*argv.offset(0 as libc::c_int as isize),
                            0 as libc::c_int as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"invalid enctype %s\x00" as *const u8 as
                                         *const libc::c_char), optarg);
                }
            }
            80 => { pid_file = optarg }
            112 => {
                free(def_udp_listen as *mut libc::c_void);
                free(def_tcp_listen as *mut libc::c_void);
                def_udp_listen = strdup(optarg);
                def_tcp_listen = strdup(optarg);
                if def_udp_listen.is_null() || def_tcp_listen.is_null() {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b" KDC cannot initialize. Not enough memory\n\x00"
                                         as *const u8 as
                                         *const libc::c_char));
                    exit(1 as libc::c_int);
                }
            }
            84 => { time_offset = atoi(optarg) }
            82 | 52 | 88 => { }
            63 | _ => { usage(*argv.offset(0 as libc::c_int as isize)); }
        }
    }
    /*
     * Check to see if we processed any realms.
     */
    if shandle.kdc_numrealms == 0 as libc::c_int {
        /* no realm specified, use default realm */
        retval = krb5_get_default_realm(kcontext, &mut lrealm);
        if retval != 0 {
            com_err(*argv.offset(0 as libc::c_int as isize),
                    retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while attempting to retrieve default realm\x00"
                                 as *const u8 as *const libc::c_char));
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"%s: %s, attempting to retrieve default realm\n\x00"
                                 as *const u8 as *const libc::c_char),
                    *argv.offset(0 as libc::c_int as isize),
                    krb5_get_error_message(kcontext, retval));
            exit(1 as libc::c_int);
        }
        rdatap =
            malloc(::std::mem::size_of::<kdc_realm_t>() as libc::c_ulong) as
                *mut kdc_realm_t;
        if !rdatap.is_null() {
            retval =
                init_realm(rdatap, aprof, lrealm, mkey_name, menctype,
                           def_udp_listen, def_tcp_listen, manual,
                           def_restrict_anon, db_args, no_referral,
                           hostbased);
            if retval != 0 {
                fprintf(stderr,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"%s: cannot initialize realm %s - see log file for details\n\x00"
                                     as *const u8 as *const libc::c_char),
                        *argv.offset(0 as libc::c_int as isize), lrealm);
                exit(1 as libc::c_int);
            }
            let ref mut fresh5 =
                *shandle.kdc_realmlist.offset(0 as libc::c_int as isize);
            *fresh5 = rdatap;
            shandle.kdc_numrealms += 1
        }
        krb5_free_default_realm(kcontext, lrealm);
    }
    if !def_udp_listen.is_null() {
        free(def_udp_listen as *mut libc::c_void);
    }
    if !def_tcp_listen.is_null() {
        free(def_tcp_listen as *mut libc::c_void);
    }
    if !db_args.is_null() { free(db_args as *mut libc::c_void); }
    if !db_name.is_null() { free(db_name as *mut libc::c_void); }
    if !hostbased.is_null() { free(hostbased as *mut libc::c_void); }
    if !no_referral.is_null() { free(no_referral as *mut libc::c_void); }
    if !aprof.is_null() { krb5_aprof_finish(aprof); };
}
#[c2rust::src_loc = "869:1"]
unsafe extern "C" fn write_pid_file(mut path: *const libc::c_char)
 -> krb5_error_code {
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut pid: libc::c_ulong = 0;
    file = fopen(path, b"w\x00" as *const u8 as *const libc::c_char);
    if file.is_null() { return *__errno_location() }
    pid = getpid() as libc::c_ulong;
    if fprintf(file, b"%ld\n\x00" as *const u8 as *const libc::c_char, pid) <
           0 as libc::c_int || fclose(file) == -(1 as libc::c_int) {
        return *__errno_location()
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "884:1"]
unsafe extern "C" fn finish_realms() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < shandle.kdc_numrealms {
        finish_realm(*shandle.kdc_realmlist.offset(i as isize));
        let ref mut fresh6 = *shandle.kdc_realmlist.offset(i as isize);
        *fresh6 = 0 as *mut kdc_realm_t;
        i += 1
    }
    shandle.kdc_numrealms = 0 as libc::c_int;
}
/*
  outline:

  process args & setup

  initialize database access (fetch master key, open DB)

  initialize network

  loop:
  listen for packet

  determine packet type, dispatch to handling routine
  (AS or TGS (or V4?))

  reflect response

  exit on signal

  clean up secrets, close db

  shut down network

  exit
*/
#[c2rust::src_loc = "922:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut kcontext: krb5_context = 0 as *mut _krb5_context;
    let mut realm: *mut kdc_realm_t = 0 as *mut kdc_realm_t;
    let mut ctx: *mut verto_ctx = 0 as *mut verto_ctx;
    let mut tcp_listen_backlog: libc::c_int = 0;
    let mut errout: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    setlocale(6 as libc::c_int, b"\x00" as *const u8 as *const libc::c_char);
    if !strrchr(*argv.offset(0 as libc::c_int as isize), '/' as i32).is_null()
       {
        let ref mut fresh7 = *argv.offset(0 as libc::c_int as isize);
        *fresh7 =
            strrchr(*argv.offset(0 as libc::c_int as isize),
                    '/' as i32).offset(1 as libc::c_int as isize)
    }
    shandle.kdc_realmlist =
        malloc((::std::mem::size_of::<*mut kdc_realm_t>() as
                    libc::c_ulong).wrapping_mul(32 as libc::c_int as
                                                    libc::c_ulong)) as
            *mut *mut kdc_realm_t;
    if shandle.kdc_realmlist.is_null() {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"%s: cannot get memory for realm list\n\x00" as
                             *const u8 as *const libc::c_char),
                *argv.offset(0 as libc::c_int as isize));
        exit(1 as libc::c_int);
    }
    memset(shandle.kdc_realmlist as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<*mut kdc_realm_t>() as
                libc::c_ulong).wrapping_mul(32 as libc::c_int as
                                                libc::c_ulong));
    /*
     * A note about Kerberos contexts: This context, "kcontext", is used
     * for the KDC operations, i.e. setup, network connection and error
     * reporting.  The per-realm operations use the "realm_context"
     * associated with each realm.
     */
    retval = krb5int_init_context_kdc(&mut kcontext);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while initializing krb5\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    krb5_klog_init(kcontext,
                   b"kdc\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
                   *argv.offset(0 as libc::c_int as isize),
                   1 as libc::c_int as krb5_boolean);
    shandle.kdc_err_context = kcontext;
    kdc_progname = *argv.offset(0 as libc::c_int as isize);
    /* N.B.: After this point, com_err sends output to the KDC log
       file, and not to stderr.  We use the kdc_err wrapper around
       com_err to ensure that the error state exists in the context
       known to the krb5_klog callback. */
    initialize_kdc5_error_table();
    /*
     * Scan through the argument list
     */
    initialize_realms(kcontext, argc, argv, &mut tcp_listen_backlog);
    retval = kdc_init_lookaside(kcontext);
    if retval != 0 {
        kdc_err(kcontext, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while initializing lookaside cache\x00" as
                             *const u8 as *const libc::c_char));
        finish_realms();
        return 1 as libc::c_int
    }
    ctx = loop_init(VERTO_EV_TYPE_NONE);
    if ctx.is_null() {
        kdc_err(kcontext, 12 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while creating main loop\x00" as *const u8 as
                             *const libc::c_char));
        finish_realms();
        return 1 as libc::c_int
    }
    load_preauth_plugins(&mut shandle, kcontext, ctx);
    load_authdata_plugins(kcontext);
    retval = load_kdcpolicy_plugins(kcontext);
    if retval != 0 {
        kdc_err(kcontext, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while loading KDC policy plugin\x00" as *const u8
                             as *const libc::c_char));
        finish_realms();
        return 1 as libc::c_int
    }
    /* Add each realm's listener addresses to the loop. */
    i = 0 as libc::c_int;
    loop  {
        if !(i < shandle.kdc_numrealms) {
            current_block = 2122094917359643297;
            break ;
        }
        realm = *shandle.kdc_realmlist.offset(i as isize);
        if *(*realm).realm_listen as libc::c_int != '\u{0}' as i32 {
            retval =
                loop_add_udp_address(88 as libc::c_int,
                                     (*realm).realm_listen);
            if retval != 0 { current_block = 6399395098838863461; break ; }
        }
        if *(*realm).realm_tcp_listen as libc::c_int != '\u{0}' as i32 {
            retval =
                loop_add_tcp_address(88 as libc::c_int,
                                     (*realm).realm_tcp_listen);
            if retval != 0 { current_block = 6399395098838863461; break ; }
        }
        i += 1
    }
    match current_block {
        2122094917359643297 => {
            if workers == 0 as libc::c_int {
                retval =
                    loop_setup_signals(ctx,
                                       &mut shandle as *mut server_handle as
                                           *mut libc::c_void,
                                       ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                               *mut libc::c_void)
                                                                          ->
                                                                              ()>,
                                                               Option<unsafe extern "C" fn()
                                                                          ->
                                                                              ()>>(Some(reset_for_hangup
                                                                                            as
                                                                                            unsafe extern "C" fn(_:
                                                                                                                     *mut libc::c_void)
                                                                                                ->
                                                                                                    ())));
                if retval != 0 {
                    kdc_err(kcontext, retval as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while initializing signal handlers\x00"
                                         as *const u8 as
                                         *const libc::c_char));
                    finish_realms();
                    return 1 as libc::c_int
                }
            }
            retval =
                loop_setup_network(ctx,
                                   &mut shandle as *mut server_handle as
                                       *mut libc::c_void, kdc_progname,
                                   tcp_listen_backlog);
            if !(retval != 0) {
                if nofork == 0 &&
                       daemon(0 as libc::c_int, 0 as libc::c_int) != 0 {
                    kdc_err(kcontext, *__errno_location() as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while detaching from tty\x00" as
                                         *const u8 as *const libc::c_char));
                    finish_realms();
                    return 1 as libc::c_int
                }
                if !pid_file.is_null() {
                    retval = write_pid_file(pid_file);
                    if retval != 0 {
                        kdc_err(kcontext, retval as errcode_t,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"while creating PID file\x00" as
                                             *const u8 as
                                             *const libc::c_char));
                        finish_realms();
                        return 1 as libc::c_int
                    }
                }
                if workers > 0 as libc::c_int {
                    finish_realms();
                    retval = create_workers(ctx, workers);
                    if retval != 0 {
                        kdc_err(kcontext, *__errno_location() as errcode_t,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"creating worker processes\x00" as
                                             *const u8 as
                                             *const libc::c_char));
                        return 1 as libc::c_int
                    }
                    /* We get here only in a worker child process; re-initialize realms. */
                    initialize_realms(kcontext, argc, argv,
                                      0 as *mut libc::c_int);
                }
                /* Initialize audit system and audit KDC startup. */
                retval = load_audit_modules(kcontext);
                if retval != 0 {
                    kdc_err(kcontext, retval as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while loading audit plugin module(s)\x00"
                                         as *const u8 as
                                         *const libc::c_char));
                    finish_realms();
                    return 1 as libc::c_int
                }
                krb5_klog_syslog(6 as libc::c_int,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"commencing operation\x00" as
                                              *const u8 as
                                              *const libc::c_char));
                if nofork != 0 {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"%s: starting...\n\x00" as *const u8 as
                                         *const libc::c_char), kdc_progname);
                }
                kau_kdc_start(kcontext, 1 as libc::c_int as krb5_boolean);
                verto_run(ctx);
                loop_free(ctx);
                kau_kdc_stop(kcontext, 1 as libc::c_int as krb5_boolean);
                krb5_klog_syslog(6 as libc::c_int,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"shutting down\x00" as *const u8 as
                                              *const libc::c_char));
                unload_preauth_plugins(kcontext);
                unload_authdata_plugins(kcontext);
                unload_kdcpolicy_plugins(kcontext);
                unload_audit_modules(kcontext);
                krb5_klog_close(kcontext);
                finish_realms();
                if !shandle.kdc_realmlist.is_null() {
                    free(shandle.kdc_realmlist as *mut libc::c_void);
                }
                kdc_free_lookaside(kcontext);
                krb5_free_context(kcontext);
                return errout
            }
        }
        _ => { }
    }
    kdc_err(kcontext, retval as errcode_t,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"while initializing network\x00" as *const u8 as
                         *const libc::c_char));
    finish_realms();
    return 1 as libc::c_int;
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
