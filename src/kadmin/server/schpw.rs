use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:2"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:2"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:2"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:2"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/unistd.h:2"]
pub mod unistd_h {
    #[c2rust::src_loc = "274:1"]
    pub type socklen_t = __socklen_t;
    use super::types_h::__socklen_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:2"]
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
    #[c2rust::src_loc = "178:1"]
    pub type krb5_addrtype = krb5_int32;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "181:1"]
    pub type krb5_authdatatype = krb5_int32;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "323:16"]
    pub struct _krb5_address {
        pub magic: krb5_magic,
        pub addrtype: krb5_addrtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
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
    #[c2rust::src_loc = "354:1"]
    pub type krb5_auth_context = *mut _krb5_auth_context;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "398:16"]
    pub struct _krb5_enc_data {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub kvno: krb5_kvno,
        pub ciphertext: krb5_data,
    }
    #[c2rust::src_loc = "398:1"]
    pub type krb5_enc_data = _krb5_enc_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1936:16"]
    pub struct _krb5_ticket_times {
        pub authtime: krb5_timestamp,
        pub starttime: krb5_timestamp,
        pub endtime: krb5_timestamp,
        pub renew_till: krb5_timestamp,
    }
    #[c2rust::src_loc = "1936:1"]
    pub type krb5_ticket_times = _krb5_ticket_times;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1946:16"]
    pub struct _krb5_authdata {
        pub magic: krb5_magic,
        pub ad_type: krb5_authdatatype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "1946:1"]
    pub type krb5_authdata = _krb5_authdata;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1954:16"]
    pub struct _krb5_transited {
        pub magic: krb5_magic,
        pub tr_type: krb5_octet,
        pub tr_contents: krb5_data,
    }
    #[c2rust::src_loc = "1954:1"]
    pub type krb5_transited = _krb5_transited;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1961:16"]
    pub struct _krb5_enc_tkt_part {
        pub magic: krb5_magic,
        pub flags: krb5_flags,
        pub session: *mut krb5_keyblock,
        pub client: krb5_principal,
        pub transited: krb5_transited,
        pub times: krb5_ticket_times,
        pub caddrs: *mut *mut krb5_address,
        pub authorization_data: *mut *mut krb5_authdata,
    }
    #[c2rust::src_loc = "1961:1"]
    pub type krb5_enc_tkt_part = _krb5_enc_tkt_part;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1979:16"]
    pub struct _krb5_ticket {
        pub magic: krb5_magic,
        pub server: krb5_principal,
        pub enc_part: krb5_enc_data,
        pub enc_part2: *mut krb5_enc_tkt_part,
    }
    #[c2rust::src_loc = "1979:1"]
    pub type krb5_ticket = _krb5_ticket;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2107:16"]
    pub struct _krb5_error {
        pub magic: krb5_magic,
        pub ctime: krb5_timestamp,
        pub cusec: krb5_int32,
        pub susec: krb5_int32,
        pub stime: krb5_timestamp,
        pub error: krb5_ui_4,
        pub client: krb5_principal,
        pub server: krb5_principal,
        pub text: krb5_data,
        pub e_data: krb5_data,
    }
    #[c2rust::src_loc = "2107:1"]
    pub type krb5_error = _krb5_error;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2246:16"]
    pub struct krb5_replay_data {
        pub timestamp: krb5_timestamp,
        pub usec: krb5_int32,
        pub seq: krb5_ui_4,
    }
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
    extern "C" {
        #[c2rust::src_loc = "125:8"]
        pub type _profile_t;
        #[c2rust::src_loc = "353:8"]
        pub type _krb5_auth_context;
        #[no_mangle]
        #[c2rust::src_loc = "2782:1"]
        pub fn krb5_kt_close(context: krb5_context, keytab: krb5_keytab)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3226:1"]
        pub fn krb5_mk_rep(context: krb5_context,
                           auth_context: krb5_auth_context,
                           outbuf: *mut krb5_data) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3292:1"]
        pub fn krb5_mk_error(context: krb5_context,
                             dec_err: *const krb5_error,
                             enc_err: *mut krb5_data) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3390:1"]
        pub fn krb5_rd_priv(context: krb5_context,
                            auth_context: krb5_auth_context,
                            inbuf: *const krb5_data,
                            userdata_out: *mut krb5_data,
                            rdata_out: *mut krb5_replay_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3489:1"]
        pub fn krb5_unparse_name(context: krb5_context,
                                 principal: krb5_const_principal,
                                 name: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4025:1"]
        pub fn krb5_build_principal(context: krb5_context,
                                    princ: *mut krb5_principal,
                                    rlen: libc::c_uint,
                                    realm: *const libc::c_char, _: ...)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4173:1"]
        pub fn krb5_kt_resolve(context: krb5_context,
                               name: *const libc::c_char,
                               ktid: *mut krb5_keytab) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "4644:1"]
        pub fn krb5_free_ticket(context: krb5_context, val: *mut krb5_ticket);
        #[no_mangle]
        #[c2rust::src_loc = "4743:1"]
        pub fn krb5_free_data(context: krb5_context, val: *mut krb5_data);
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
        #[c2rust::src_loc = "5217:1"]
        pub fn krb5_rd_req(context: krb5_context,
                           auth_context: *mut krb5_auth_context,
                           inbuf: *const krb5_data,
                           server: krb5_const_principal, keytab: krb5_keytab,
                           ap_req_options: *mut krb5_flags,
                           ticket: *mut *mut krb5_ticket) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5337:1"]
        pub fn krb5_mk_priv(context: krb5_context,
                            auth_context: krb5_auth_context,
                            userdata: *const krb5_data,
                            der_out: *mut krb5_data,
                            rdata_out: *mut krb5_replay_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5613:1"]
        pub fn krb5_auth_con_init(context: krb5_context,
                                  auth_context: *mut krb5_auth_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5626:1"]
        pub fn krb5_auth_con_free(context: krb5_context,
                                  auth_context: krb5_auth_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5644:1"]
        pub fn krb5_auth_con_setflags(context: krb5_context,
                                      auth_context: krb5_auth_context,
                                      flags: krb5_int32) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5718:1"]
        pub fn krb5_auth_con_setaddrs(context: krb5_context,
                                      auth_context: krb5_auth_context,
                                      local_addr: *mut krb5_address,
                                      remote_addr: *mut krb5_address)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "8023:1"]
        pub fn krb5_get_error_message(ctx: krb5_context,
                                      code: krb5_error_code)
         -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "8032:1"]
        pub fn krb5_free_error_message(ctx: krb5_context,
                                       msg: *const libc::c_char);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:2"]
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
    /* could be used in a table to find an etype and initialize a block */
    /* internal message representations */
    /* user data */
    /* client time, optional */
    /* microsecond portion of time,
                                           optional */
    /* sequence #, optional */
    /* sender address */
    /* recipient address, optional */
    /* data integrity checksum */
    /* encrypted part */
    /* user data */
    /* client time, optional */
    /* microsecond portion of time, opt. */
    /* sequence #, optional */
    /* sender address */
    /* recipient address, optional */
    /*
 * Begin "asn1.h"
 */
    /* ASN.1 encoding knowledge; KEEP IN SYNC WITH ASN.1 defs! */
/* here we use some knowledge of ASN.1 encodings */
/*
  Ticket is APPLICATION 1.
  Authenticator is APPLICATION 2.
  AS_REQ is APPLICATION 10.
  AS_REP is APPLICATION 11.
  TGS_REQ is APPLICATION 12.
  TGS_REP is APPLICATION 13.
  AP_REQ is APPLICATION 14.
  AP_REP is APPLICATION 15.
  KRB_SAFE is APPLICATION 20.
  KRB_PRIV is APPLICATION 21.
  KRB_CRED is APPLICATION 22.
  EncASRepPart is APPLICATION 25.
  EncTGSRepPart is APPLICATION 26.
  EncAPRepPart is APPLICATION 27.
  EncKrbPrivPart is APPLICATION 28.
  EncKrbCredPart is APPLICATION 29.
  KRB_ERROR is APPLICATION 30.
*/
/* allow either constructed or primitive encoding, so check for bit 6
   set or reset */
    /* ************************************************************************
 * Prototypes for krb5_encode.c
 *************************************************************************/
    /*
  krb5_error_code encode_krb5_structure(const krb5_structure *rep,
  krb5_data **code);
  modifies  *code
  effects   Returns the ASN.1 encoding of *rep in **code.
  Returns ASN1_MISSING_FIELD if a required field is emtpy in *rep.
  Returns ENOMEM if memory runs out.
*/
    /* yes, the translation is identical to that used for KDC__REP */
    /* yes, the translation is identical to that used for KDC__REP */
    /* ************************************************************************
 * End of prototypes for krb5_encode.c
 *************************************************************************/
    /* ************************************************************************
 * Prototypes for krb5_decode.c
 *************************************************************************/
/*
  krb5_error_code decode_krb5_structure(const krb5_data *code,
  krb5_structure **rep);

  requires  Expects **rep to not have been allocated;
  a new *rep is allocated regardless of the old value.
  effects   Decodes *code into **rep.
  Returns ENOMEM if memory is exhausted.
  Returns asn1 and krb5 errors.
*/
    /* kdb.h */
    /* Master key version number */
    /* kvno of key_data elements (all the same) */
    /* ************************************************************************
 * End of prototypes for krb5_decode.c
 *************************************************************************/
    /* KRB5_ASN1__ */
    /*
 * End "asn1.h"
 */
    /*
 * Internal krb5 library routines
 */
    /* Return true if s is non-empty and composed solely of digits. */
    /*
 * Initialization routines.
 */
    /* [De]serialize 4-byte integer */
    /* [De]serialize 8-byte integer */
    /* [De]serialize byte string */
    /* Fill in the buffer with random alpha-numeric data. */
    /* value to use when requesting a keytab entry and KVNO doesn't matter */
    /* value to use when requesting a keytab entry and enctype doesn't matter */
    /* To keep happy libraries which are (for now) accessing internal stuff */
    /* Make sure to increment by one when changing the struct */
    /* Used for KDB LDAP back end.  */
    /*
     * pkinit asn.1 encode/decode functions
     */
    /* Set *tag_out to the integrity tag of *enc.  (Does not allocate memory;
 * returned buffer is a subrange of *ctext.) */
    /*
 * This structure was exposed and used in macros in krb5 1.2, so do not
 * change its ABI.
 */
    /* routines always present */
    /* routines to be included on extended version (write routines) */
    /* Not sure it's ready for exposure just yet.  */
    /*
 * Referral definitions and subfunctions.
 */
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
    #[c2rust::src_loc = "2262:1"]
    pub unsafe extern "C" fn empty_data() -> krb5_data {
        return make_data(0 as *mut libc::c_void,
                         0 as libc::c_int as libc::c_uint);
    }
    #[inline]
    #[c2rust::src_loc = "2274:1"]
    pub unsafe extern "C" fn alloc_data(mut data: *mut krb5_data,
                                        mut len: libc::c_uint)
     -> krb5_error_code {
        /* Allocate at least one byte since zero-byte allocs may return NULL. */
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
                        krb5_pointer, krb5_error_code, krb5_context,
                        krb5_keytab, krb5_const_principal, krb5_kvno,
                        krb5_keytab_entry, krb5_kt_cursor, krb5_data,
                        krb5_principal};
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
        #[no_mangle]
        #[c2rust::src_loc = "1669:1"]
        pub fn decode_krb5_setpw_req(_: *const krb5_data,
                                     _: *mut *mut krb5_data,
                                     _: *mut krb5_principal)
         -> krb5_error_code;
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:2"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:2"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:2"]
pub mod profile_h {
    /*
 * profile.h
 */
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:2"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/bits/socket.h:2"]
pub mod socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "178:8"]
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [libc::c_char; 14],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "191:8"]
    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
        pub __ss_padding: [libc::c_char; 118],
        pub __ss_align: libc::c_ulong,
    }
    use super::sockaddr_h::sa_family_t;
}
#[c2rust::header_src = "/usr/include/netinet/in.h:2"]
pub mod in_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "253:8"]
    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: in_port_t,
        pub sin6_flowinfo: uint32_t,
        pub sin6_addr: in6_addr,
        pub sin6_scope_id: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "212:8"]
    pub struct in6_addr {
        pub __in6_u: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "214:5"]
    pub union C2RustUnnamed {
        pub __u6_addr8: [uint8_t; 16],
        pub __u6_addr16: [uint16_t; 8],
        pub __u6_addr32: [uint32_t; 4],
    }
    #[c2rust::src_loc = "119:1"]
    pub type in_port_t = uint16_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "238:8"]
    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [libc::c_uchar; 8],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:8"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    #[c2rust::src_loc = "30:1"]
    pub type in_addr_t = uint32_t;
    use super::sockaddr_h::sa_family_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t, uint16_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "380:1"]
        pub fn htons(__hostshort: uint16_t) -> uint16_t;
    }
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:3"]
pub mod kdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    use super::krb5_h::{krb5_enctype, krb5_int32};
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:3"]
pub mod admin_h {
    #[c2rust::src_loc = "71:1"]
    pub type kadm5_ret_t = libc::c_long;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "241:16"]
    pub struct _kadm5_config_params {
        pub mask: libc::c_long,
        pub realm: *mut libc::c_char,
        pub kadmind_port: libc::c_int,
        pub kpasswd_port: libc::c_int,
        pub admin_server: *mut libc::c_char,
        pub dbname: *mut libc::c_char,
        pub acl_file: *mut libc::c_char,
        pub dict_file: *mut libc::c_char,
        pub mkey_from_kbd: libc::c_int,
        pub stash_file: *mut libc::c_char,
        pub mkey_name: *mut libc::c_char,
        pub enctype: krb5_enctype,
        pub max_life: krb5_deltat,
        pub max_rlife: krb5_deltat,
        pub expiration: krb5_timestamp,
        pub flags: krb5_flags,
        pub keysalts: *mut krb5_key_salt_tuple,
        pub num_keysalts: krb5_int32,
        pub kvno: krb5_kvno,
        pub iprop_enabled: libc::c_int,
        pub iprop_ulogsize: uint32_t,
        pub iprop_poll_time: krb5_deltat,
        pub iprop_logfile: *mut libc::c_char,
        pub iprop_port: libc::c_int,
        pub iprop_resync_timeout: libc::c_int,
        pub kadmind_listen: *mut libc::c_char,
        pub kpasswd_listen: *mut libc::c_char,
        pub iprop_listen: *mut libc::c_char,
    }
    #[c2rust::src_loc = "241:1"]
    pub type kadm5_config_params = _kadm5_config_params;
    use super::krb5_h::{krb5_enctype, krb5_deltat, krb5_timestamp, krb5_flags,
                        krb5_int32, krb5_kvno};
    use super::kdb_h::krb5_key_salt_tuple;
    use super::stdint_uintn_h::uint32_t;
    /* __KADM5_ADMIN_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/server_internal.h:9"]
pub mod server_internal_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved
 *
 * $Header$
 */
    /*
 * This header file is used internally by the Admin API server
 * libraries and Admin server.  IF YOU THINK YOU NEED TO USE THIS FILE
 * FOR ANYTHING, YOU'RE ALMOST CERTAINLY WRONG.
 */
    /*
 * This is the history key version for a newly created DB.  We use this value
 * for principals which have no password history yet to avoid having to look up
 * the history key.  Values other than 2 will cause compatibility issues with
 * pre-1.8 libkadm5 code; the older code will reject key changes when it sees
 * an unexpected value of admin_history_kvno.
 */
    /* A pwqual_handle represents a password quality plugin module. */
    #[c2rust::src_loc = "38:1"]
    pub type pwqual_handle = *mut pwqual_handle_st;
    #[c2rust::src_loc = "40:1"]
    pub type kadm5_hook_handle = *mut kadm5_hook_handle_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "42:16"]
    pub struct _kadm5_server_handle_t {
        pub magic_number: krb5_ui_4,
        pub struct_version: krb5_ui_4,
        pub api_version: krb5_ui_4,
        pub context: krb5_context,
        pub current_caller: krb5_principal,
        pub params: kadm5_config_params,
        pub lhandle: *mut _kadm5_server_handle_t,
        pub db_args: *mut *mut libc::c_char,
        pub qual_handles: *mut pwqual_handle,
        pub hook_handles: *mut kadm5_hook_handle,
    }
    #[c2rust::src_loc = "42:1"]
    pub type kadm5_server_handle_t = *mut _kadm5_server_handle_t;
    use super::krb5_h::{krb5_ui_4, krb5_context, krb5_principal};
    use super::admin_h::kadm5_config_params;
    extern "C" {
        #[c2rust::src_loc = "38:16"]
        pub type pwqual_handle_st;
        #[c2rust::src_loc = "40:16"]
        pub type kadm5_hook_handle_st;
    }
    /* __KADM5_SERVER_INTERNAL_H__ */
    /* * @}*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/verto.h:11"]
pub mod verto_h {
    extern "C" {
        #[c2rust::src_loc = "48:16"]
        pub type verto_ctx;
    }
    /* VERTO_H_ */
    /* __cplusplus */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/net-server.h:11"]
pub mod net_server_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:16"]
    pub struct _krb5_fulladdr {
        pub address: *mut krb5_address,
        pub port: krb5_ui_4,
    }
    #[c2rust::src_loc = "37:1"]
    pub type krb5_fulladdr = _krb5_fulladdr;
    /* to be supplied by the server application */
    /*
 * Two routines for processing an incoming message and getting a
 * result to send back.
 *
 * The first, dispatch(), is for normal processing of a request.  The
 * second, make_toolong_error(), is obviously for generating an error
 * to send back when the incoming message is bigger than
 * the main loop can accept.
 */
    #[c2rust::src_loc = "87:1"]
    pub type loop_respond_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: krb5_error_code,
                                    _: *mut krb5_data) -> ()>;
    use super::krb5_h::{krb5_address, krb5_ui_4, krb5_error_code, krb5_data};
    /* NET_SERVER_H */
}
#[c2rust::header_src = "/usr/include/string.h:2"]
pub mod string_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "435:1"]
        pub fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:2"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:2"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:2"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/netdb.h:2"]
pub mod netdb_h {
    use super::socket_h::sockaddr;
    use super::unistd_h::socklen_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "675:1"]
        pub fn getnameinfo(__sa: *const sockaddr, __salen: socklen_t,
                           __host: *mut libc::c_char, __hostlen: socklen_t,
                           __serv: *mut libc::c_char, __servlen: socklen_t,
                           __flags: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/socket-utils.h:2"]
pub mod socket_utils_h {
    #[inline]
    #[c2rust::src_loc = "79:1"]
    pub unsafe extern "C" fn ss2sa(mut ss: *mut sockaddr_storage)
     -> *mut sockaddr {
        return ss as *mut sockaddr;
    }
    #[inline]
    #[c2rust::src_loc = "83:1"]
    pub unsafe extern "C" fn ss2sin(mut ss: *mut sockaddr_storage)
     -> *mut sockaddr_in {
        return ss as *mut sockaddr_in;
    }
    #[inline]
    #[c2rust::src_loc = "87:1"]
    pub unsafe extern "C" fn ss2sin6(mut ss: *mut sockaddr_storage)
     -> *mut sockaddr_in6 {
        return ss as *mut sockaddr_in6;
    }
    use super::socket_h::{sockaddr_storage, sockaddr};
    use super::in_h::{sockaddr_in, sockaddr_in6};
    /* SOCKET_UTILS_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:2"]
pub mod k5_platform_h {
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/adm_proto.h:5"]
pub mod adm_proto_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "53:1"]
        pub fn krb5_klog_syslog(_: libc::c_int, _: *const libc::c_char,
                                _: ...) -> libc::c_int;
    }
    /* KRB5_ADM_PROTO_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kadmin/server/misc.h:11"]
pub mod misc_h {
    use super::stddef_h::size_t;
    use super::krb5_h::{krb5_principal_data, krb5_principal, krb5_boolean};
    use super::admin_h::kadm5_ret_t;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1994 OpenVision Technologies, Inc., All Rights Reserved
 *
 */
        #[no_mangle]
        #[c2rust::src_loc = "28:1"]
        pub fn trunc_name(len: *mut size_t, dots: *mut *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "16:1"]
        pub fn schpw_util_wrapper(server_handle: *mut libc::c_void,
                                  client: krb5_principal,
                                  target: krb5_principal,
                                  initial_flag: krb5_boolean,
                                  new_pw: *mut libc::c_char,
                                  ret_pw: *mut *mut libc::c_char,
                                  msg_ret: *mut libc::c_char,
                                  msg_len: libc::c_uint) -> kadm5_ret_t;
    }
    /* _MISC_H */
}
pub use self::types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t,
                        __socklen_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::unistd_h::socklen_t;
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_boolean,
                       krb5_kvno, krb5_addrtype, krb5_enctype,
                       krb5_authdatatype, krb5_flags, krb5_timestamp,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_pointer, krb5_principal_data,
                       krb5_principal, krb5_const_principal, _krb5_address,
                       krb5_address, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, krb5_auth_context,
                       _krb5_keyblock, krb5_keyblock, _krb5_enc_data,
                       krb5_enc_data, _krb5_ticket_times, krb5_ticket_times,
                       _krb5_authdata, krb5_authdata, _krb5_transited,
                       krb5_transited, _krb5_enc_tkt_part, krb5_enc_tkt_part,
                       _krb5_ticket, krb5_ticket, _krb5_error, krb5_error,
                       krb5_replay_data, krb5_kt_cursor, krb5_keytab_entry_st,
                       krb5_keytab_entry, krb5_keytab, _profile_t,
                       _krb5_auth_context, krb5_kt_close, krb5_mk_rep,
                       krb5_mk_error, krb5_rd_priv, krb5_unparse_name,
                       krb5_build_principal, krb5_kt_resolve,
                       krb5_free_principal, krb5_free_ticket, krb5_free_data,
                       krb5_free_unparsed_name, krb5_timeofday, krb5_rd_req,
                       krb5_mk_priv, krb5_auth_con_init, krb5_auth_con_free,
                       krb5_auth_con_setflags, krb5_auth_con_setaddrs,
                       krb5_get_error_message, krb5_free_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, _krb5_kt, _krb5_kt_ops, zapfree,
                         make_data, empty_data, alloc_data, k5calloc, k5alloc,
                         k5memdup0, plugin_mapping, _kdb_log_context,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle,
                         decode_krb5_setpw_req};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::sockaddr_h::sa_family_t;
pub use self::socket_h::{sockaddr, sockaddr_storage};
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed, in_port_t,
                     sockaddr_in, in_addr, in_addr_t, htons};
pub use self::kdb_h::{__krb5_key_salt_tuple, krb5_key_salt_tuple};
pub use self::admin_h::{kadm5_ret_t, _kadm5_config_params,
                        kadm5_config_params};
pub use self::server_internal_h::{pwqual_handle, kadm5_hook_handle,
                                  _kadm5_server_handle_t,
                                  kadm5_server_handle_t, pwqual_handle_st,
                                  kadm5_hook_handle_st};
use self::verto_h::verto_ctx;
pub use self::net_server_h::{_krb5_fulladdr, krb5_fulladdr, loop_respond_fn};
use self::string_h::{memcpy, strlen, explicit_bzero};
use self::stdlib_h::{calloc, free};
use self::stdio_h::snprintf;
use self::libintl_h::dgettext;
use self::netdb_h::getnameinfo;
pub use self::socket_utils_h::{ss2sa, ss2sin, ss2sin6};
use self::k5_platform_h::krb5int_strlcpy;
use self::adm_proto_h::krb5_klog_syslog;
use self::misc_h::{trunc_name, schpw_util_wrapper};
#[c2rust::src_loc = "19:1"]
unsafe extern "C" fn process_chpw_request(mut context: krb5_context,
                                          mut server_handle:
                                              *mut libc::c_void,
                                          mut realm: *mut libc::c_char,
                                          mut keytab: krb5_keytab,
                                          mut local_addr:
                                              *const krb5_fulladdr,
                                          mut remote_addr:
                                              *const krb5_fulladdr,
                                          mut req: *mut krb5_data,
                                          mut rep: *mut krb5_data)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut plen: libc::c_uint = 0;
    let mut vno: libc::c_uint = 0;
    let mut ap_req: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut ap_rep: krb5_data = empty_data();
    let mut cipher: krb5_data = empty_data();
    let mut clear: krb5_data = empty_data();
    let mut auth_context: krb5_auth_context = 0 as krb5_auth_context;
    let mut changepw: krb5_principal = 0 as krb5_principal;
    let mut client: krb5_principal = 0 as *mut krb5_principal_data;
    let mut target: krb5_principal = 0 as krb5_principal;
    let mut ticket: *mut krb5_ticket = 0 as *mut krb5_ticket;
    let mut replay: krb5_replay_data =
        krb5_replay_data{timestamp: 0, usec: 0, seq: 0,};
    let mut krberror: krb5_error =
        krb5_error{magic: 0,
                   ctime: 0,
                   cusec: 0,
                   susec: 0,
                   stime: 0,
                   error: 0,
                   client: 0 as *mut krb5_principal_data,
                   server: 0 as *mut krb5_principal_data,
                   text:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},
                   e_data:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},};
    let mut numresult: libc::c_int = 0;
    let mut strresult: [libc::c_char; 1024] = [0; 1024];
    let mut clientstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut targetstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    let mut clen: size_t = 0;
    let mut cdots: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ss: sockaddr_storage =
        sockaddr_storage{ss_family: 0,
                         __ss_padding: [0; 118],
                         __ss_align: 0,};
    let mut salen: socklen_t = 0;
    let mut addrbuf: [libc::c_char; 100] = [0; 100];
    let mut addr: *mut krb5_address = (*remote_addr).address;
    *rep = empty_data();
    if (*req).length < 4 as libc::c_int as libc::c_uint {
        /* either this, or the server is printing bad messages,
           or the caller passed in garbage */
        ret = -(1765328343 as libc::c_long) as krb5_error_code;
        numresult = 1 as libc::c_int;
        krb5int_strlcpy(strresult.as_mut_ptr(),
                        b"Request was truncated\x00" as *const u8 as
                            *const libc::c_char,
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as
                            libc::c_ulong);
    } else {
        ptr = (*req).data;
        /* verify length */
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        plen = (*fresh0 as libc::c_int & 0xff as libc::c_int) as libc::c_uint;
        let fresh1 = ptr;
        ptr = ptr.offset(1);
        plen =
            plen << 8 as libc::c_int |
                (*fresh1 as libc::c_int & 0xff as libc::c_int) as
                    libc::c_uint;
        if plen != (*req).length {
            ret = -(1765328343 as libc::c_long) as krb5_error_code;
            numresult = 1 as libc::c_int;
            krb5int_strlcpy(strresult.as_mut_ptr(),
                            b"Request length was inconsistent\x00" as
                                *const u8 as *const libc::c_char,
                            ::std::mem::size_of::<[libc::c_char; 1024]>() as
                                libc::c_ulong);
        } else {
            /* verify version number */
            let fresh2 = ptr;
            ptr = ptr.offset(1);
            vno =
                (*fresh2 as libc::c_int & 0xff as libc::c_int) as
                    libc::c_uint;
            let fresh3 = ptr;
            ptr = ptr.offset(1);
            vno =
                vno << 8 as libc::c_int |
                    (*fresh3 as libc::c_int & 0xff as libc::c_int) as
                        libc::c_uint;
            if vno != 1 as libc::c_int as libc::c_uint &&
                   vno != 0xff80 as libc::c_int as libc::c_uint {
                ret = -(1765328381 as libc::c_long) as krb5_error_code;
                numresult = 6 as libc::c_int;
                snprintf(strresult.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 1024]>() as
                             libc::c_ulong,
                         b"Request contained unknown protocol version number %d\x00"
                             as *const u8 as *const libc::c_char, vno);
            } else {
                /* read, check ap-req length */
                let fresh4 = ptr;
                ptr = ptr.offset(1);
                ap_req.length =
                    (*fresh4 as libc::c_int & 0xff as libc::c_int) as
                        libc::c_uint;
                let fresh5 = ptr;
                ptr = ptr.offset(1);
                ap_req.length =
                    ap_req.length << 8 as libc::c_int |
                        (*fresh5 as libc::c_int & 0xff as libc::c_int) as
                            libc::c_uint;
                if ptr.offset(ap_req.length as isize) >=
                       (*req).data.offset((*req).length as isize) {
                    ret = -(1765328343 as libc::c_long) as krb5_error_code;
                    numresult = 1 as libc::c_int;
                    krb5int_strlcpy(strresult.as_mut_ptr(),
                                    b"Request was truncated in AP-REQ\x00" as
                                        *const u8 as *const libc::c_char,
                                    ::std::mem::size_of::<[libc::c_char; 1024]>()
                                        as libc::c_ulong);
                } else {
                    /* verify ap_req */
                    ap_req.data = ptr;
                    ptr = ptr.offset(ap_req.length as isize);
                    ret = krb5_auth_con_init(context, &mut auth_context);
                    if ret != 0 {
                        numresult = 2 as libc::c_int;
                        krb5int_strlcpy(strresult.as_mut_ptr(),
                                        b"Failed initializing auth context\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        ::std::mem::size_of::<[libc::c_char; 1024]>()
                                            as libc::c_ulong);
                    } else {
                        ret =
                            krb5_auth_con_setflags(context, auth_context,
                                                   0x4 as libc::c_int);
                        if ret != 0 {
                            numresult = 2 as libc::c_int;
                            krb5int_strlcpy(strresult.as_mut_ptr(),
                                            b"Failed initializing auth context\x00"
                                                as *const u8 as
                                                *const libc::c_char,
                                            ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                as libc::c_ulong);
                        } else {
                            ret =
                                krb5_build_principal(context,
                                                     &mut changepw as
                                                         *mut krb5_principal,
                                                     strlen(realm) as
                                                         libc::c_uint, realm,
                                                     b"kadmin\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     b"changepw\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     0 as *mut libc::c_void);
                            if ret != 0 {
                                numresult = 2 as libc::c_int;
                                krb5int_strlcpy(strresult.as_mut_ptr(),
                                                b"Failed building kadmin/changepw principal\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                    as libc::c_ulong);
                            } else {
                                ret =
                                    krb5_rd_req(context, &mut auth_context,
                                                &mut ap_req,
                                                changepw as
                                                    krb5_const_principal,
                                                keytab, 0 as *mut krb5_flags,
                                                &mut ticket);
                                if ret != 0 {
                                    numresult = 3 as libc::c_int;
                                    krb5int_strlcpy(strresult.as_mut_ptr(),
                                                    b"Failed reading application request\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                        as libc::c_ulong);
                                } else {
                                    /* construct the ap-rep */
                                    ret =
                                        krb5_mk_rep(context, auth_context,
                                                    &mut ap_rep);
                                    if ret != 0 {
                                        numresult = 3 as libc::c_int;
                                        krb5int_strlcpy(strresult.as_mut_ptr(),
                                                        b"Failed replying to application request\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                            as libc::c_ulong);
                                    } else {
                                        /* decrypt the ChangePasswdData */
                                        cipher.length =
                                            (*req).data.offset((*req).length
                                                                   as
                                                                   isize).wrapping_offset_from(ptr)
                                                as libc::c_long as
                                                libc::c_uint;
                                        cipher.data = ptr;
                                        /*
     * Don't set a remote address in auth_context before calling krb5_rd_priv,
     * so that we can work against clients behind a NAT.  Reflection attacks
     * aren't a concern since we use sequence numbers and since our requests
     * don't look anything like our responses.  Also don't set a local address,
     * since we don't know what interface the request was received on.
     */
                                        ret =
                                            krb5_rd_priv(context,
                                                         auth_context,
                                                         &mut cipher,
                                                         &mut clear,
                                                         &mut replay);
                                        if ret != 0 {
                                            numresult = 2 as libc::c_int;
                                            krb5int_strlcpy(strresult.as_mut_ptr(),
                                                            b"Failed decrypting request\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                                as
                                                                libc::c_ulong);
                                        } else {
                                            client =
                                                (*(*ticket).enc_part2).client;
                                            /* decode ChangePasswdData for setpw requests */
                                            if vno ==
                                                   0xff80 as libc::c_int as
                                                       libc::c_uint {
                                                let mut clear_data:
                                                        *mut krb5_data =
                                                    0 as *mut krb5_data;
                                                ret =
                                                    decode_krb5_setpw_req(&mut clear,
                                                                          &mut clear_data,
                                                                          &mut target);
                                                if ret != 0 as libc::c_int {
                                                    numresult =
                                                        1 as libc::c_int;
                                                    krb5int_strlcpy(strresult.as_mut_ptr(),
                                                                    b"Failed decoding ChangePasswdData\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                                        as
                                                                        libc::c_ulong);
                                                    current_block =
                                                        10513779607626524868;
                                                } else {
                                                    zapfree(clear.data as
                                                                *mut libc::c_void,
                                                            clear.length as
                                                                size_t);
                                                    clear = *clear_data;
                                                    free(clear_data as
                                                             *mut libc::c_void);
                                                    if !target.is_null() {
                                                        ret =
                                                            krb5_unparse_name(context,
                                                                              target
                                                                                  as
                                                                                  krb5_const_principal,
                                                                              &mut targetstr);
                                                        if ret !=
                                                               0 as
                                                                   libc::c_int
                                                           {
                                                            numresult =
                                                                2 as
                                                                    libc::c_int;
                                                            krb5int_strlcpy(strresult.as_mut_ptr(),
                                                                            b"Failed unparsing target name for log\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char,
                                                                            ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                                                as
                                                                                libc::c_ulong);
                                                            current_block =
                                                                10513779607626524868;
                                                        } else {
                                                            current_block =
                                                                851619935621435220;
                                                        }
                                                    } else {
                                                        current_block =
                                                            851619935621435220;
                                                    }
                                                }
                                            } else {
                                                current_block =
                                                    851619935621435220;
                                            }
                                            match current_block {
                                                10513779607626524868 => { }
                                                _ => {
                                                    ret =
                                                        krb5_unparse_name(context,
                                                                          client
                                                                              as
                                                                              krb5_const_principal,
                                                                          &mut clientstr);
                                                    if ret != 0 {
                                                        numresult =
                                                            2 as libc::c_int;
                                                        krb5int_strlcpy(strresult.as_mut_ptr(),
                                                                        b"Failed unparsing client name for log\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                                            as
                                                                            libc::c_ulong);
                                                    } else {
                                                        /* change the password */
                                                        ptr =
                                                            k5memdup0(clear.data
                                                                          as
                                                                          *const libc::c_void,
                                                                      clear.length
                                                                          as
                                                                          size_t,
                                                                      &mut ret)
                                                                as
                                                                *mut libc::c_char;
                                                        ret =
                                                            schpw_util_wrapper(server_handle,
                                                                               client,
                                                                               target,
                                                                               ((*(*ticket).enc_part2).flags
                                                                                    &
                                                                                    0x400000
                                                                                        as
                                                                                        libc::c_int
                                                                                    !=
                                                                                    0
                                                                                        as
                                                                                        libc::c_int)
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   krb5_boolean,
                                                                               ptr,
                                                                               0
                                                                                   as
                                                                                   *mut *mut libc::c_char,
                                                                               strresult.as_mut_ptr(),
                                                                               ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                                                   as
                                                                                   libc::c_ulong
                                                                                   as
                                                                                   libc::c_uint)
                                                                as
                                                                krb5_error_code;
                                                        if ret != 0 {
                                                            errmsg =
                                                                krb5_get_error_message(context,
                                                                                       ret)
                                                        }
                                                        /* zap the password */
                                                        zapfree(clear.data as
                                                                    *mut libc::c_void,
                                                                clear.length
                                                                    as
                                                                    size_t);
                                                        zapfree(ptr as
                                                                    *mut libc::c_void,
                                                                clear.length
                                                                    as
                                                                    size_t);
                                                        clear = empty_data();
                                                        clen =
                                                            strlen(clientstr);
                                                        trunc_name(&mut clen,
                                                                   &mut cdots);
                                                        match (*addr).addrtype
                                                            {
                                                            2 => {
                                                                let mut sin:
                                                                        *mut sockaddr_in =
                                                                    ss2sin(&mut ss);
                                                                (*sin).sin_family
                                                                    =
                                                                    2 as
                                                                        libc::c_int
                                                                        as
                                                                        sa_family_t;
                                                                memcpy(&mut (*sin).sin_addr
                                                                           as
                                                                           *mut in_addr
                                                                           as
                                                                           *mut libc::c_void,
                                                                       (*addr).contents
                                                                           as
                                                                           *const libc::c_void,
                                                                       (*addr).length
                                                                           as
                                                                           libc::c_ulong);
                                                                (*sin).sin_port
                                                                    =
                                                                    htons((*remote_addr).port
                                                                              as
                                                                              uint16_t);
                                                                salen =
                                                                    ::std::mem::size_of::<sockaddr_in>()
                                                                        as
                                                                        libc::c_ulong
                                                                        as
                                                                        socklen_t
                                                            }
                                                            24 => {
                                                                let mut sin6:
                                                                        *mut sockaddr_in6 =
                                                                    ss2sin6(&mut ss);
                                                                (*sin6).sin6_family
                                                                    =
                                                                    10 as
                                                                        libc::c_int
                                                                        as
                                                                        sa_family_t;
                                                                memcpy(&mut (*sin6).sin6_addr
                                                                           as
                                                                           *mut in6_addr
                                                                           as
                                                                           *mut libc::c_void,
                                                                       (*addr).contents
                                                                           as
                                                                           *const libc::c_void,
                                                                       (*addr).length
                                                                           as
                                                                           libc::c_ulong);
                                                                (*sin6).sin6_port
                                                                    =
                                                                    htons((*remote_addr).port
                                                                              as
                                                                              uint16_t);
                                                                salen =
                                                                    ::std::mem::size_of::<sockaddr_in6>()
                                                                        as
                                                                        libc::c_ulong
                                                                        as
                                                                        socklen_t
                                                            }
                                                            _ => {
                                                                let mut sa:
                                                                        *mut sockaddr =
                                                                    ss2sa(&mut ss);
                                                                (*sa).sa_family
                                                                    =
                                                                    0 as
                                                                        libc::c_int
                                                                        as
                                                                        sa_family_t;
                                                                salen =
                                                                    ::std::mem::size_of::<sockaddr>()
                                                                        as
                                                                        libc::c_ulong
                                                                        as
                                                                        socklen_t
                                                            }
                                                        }
                                                        if getnameinfo(ss2sa(&mut ss),
                                                                       salen,
                                                                       addrbuf.as_mut_ptr(),
                                                                       ::std::mem::size_of::<[libc::c_char; 100]>()
                                                                           as
                                                                           libc::c_ulong
                                                                           as
                                                                           socklen_t,
                                                                       0 as
                                                                           *mut libc::c_char,
                                                                       0 as
                                                                           libc::c_int
                                                                           as
                                                                           socklen_t,
                                                                       1 as
                                                                           libc::c_int
                                                                           |
                                                                           2
                                                                               as
                                                                               libc::c_int)
                                                               !=
                                                               0 as
                                                                   libc::c_int
                                                           {
                                                            krb5int_strlcpy(addrbuf.as_mut_ptr(),
                                                                            b"<unprintable>\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char,
                                                                            ::std::mem::size_of::<[libc::c_char; 100]>()
                                                                                as
                                                                                libc::c_ulong);
                                                        }
                                                        if vno ==
                                                               0xff80 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint
                                                           {
                                                            let mut tlen:
                                                                    size_t =
                                                                0;
                                                            let mut tdots:
                                                                    *mut libc::c_char =
                                                                0 as
                                                                    *mut libc::c_char;
                                                            let mut targetp:
                                                                    *const libc::c_char =
                                                                0 as
                                                                    *const libc::c_char;
                                                            if target.is_null()
                                                               {
                                                                tlen = clen;
                                                                tdots = cdots;
                                                                targetp =
                                                                    targetstr
                                                            } else {
                                                                tlen =
                                                                    strlen(targetstr);
                                                                trunc_name(&mut tlen,
                                                                           &mut tdots);
                                                                targetp =
                                                                    clientstr
                                                            }
                                                            krb5_klog_syslog(5
                                                                                 as
                                                                                 libc::c_int,
                                                                             dgettext(b"mit-krb5\x00"
                                                                                          as
                                                                                          *const u8
                                                                                          as
                                                                                          *const libc::c_char,
                                                                                      b"setpw request from %s by %.*s%s for %.*s%s: %s\x00"
                                                                                          as
                                                                                          *const u8
                                                                                          as
                                                                                          *const libc::c_char),
                                                                             addrbuf.as_mut_ptr(),
                                                                             clen
                                                                                 as
                                                                                 libc::c_int,
                                                                             clientstr,
                                                                             cdots,
                                                                             tlen
                                                                                 as
                                                                                 libc::c_int,
                                                                             targetp,
                                                                             tdots,
                                                                             if !errmsg.is_null()
                                                                                {
                                                                                 errmsg
                                                                             } else {
                                                                                 b"success\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char
                                                                             });
                                                        } else {
                                                            krb5_klog_syslog(5
                                                                                 as
                                                                                 libc::c_int,
                                                                             dgettext(b"mit-krb5\x00"
                                                                                          as
                                                                                          *const u8
                                                                                          as
                                                                                          *const libc::c_char,
                                                                                      b"chpw request from %s for %.*s%s: %s\x00"
                                                                                          as
                                                                                          *const u8
                                                                                          as
                                                                                          *const libc::c_char),
                                                                             addrbuf.as_mut_ptr(),
                                                                             clen
                                                                                 as
                                                                                 libc::c_int,
                                                                             clientstr,
                                                                             cdots,
                                                                             if !errmsg.is_null()
                                                                                {
                                                                                 errmsg
                                                                             } else {
                                                                                 b"success\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char
                                                                             });
                                                        }
                                                        match ret {
                                                            43787565 => {
                                                                numresult =
                                                                    5 as
                                                                        libc::c_int
                                                            }
                                                            43787582 => {
                                                                numresult =
                                                                    7 as
                                                                        libc::c_int
                                                            }
                                                            43787542 |
                                                            43787545 |
                                                            43787543 |
                                                            43787544 |
                                                            43787577 |
                                                            43787546 => {
                                                                numresult =
                                                                    4 as
                                                                        libc::c_int
                                                            }
                                                            0 => {
                                                                numresult =
                                                                    0 as
                                                                        libc::c_int;
                                                                krb5int_strlcpy(strresult.as_mut_ptr(),
                                                                                b"\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char,
                                                                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                                                    as
                                                                                    libc::c_ulong);
                                                            }
                                                            _ => {
                                                                numresult =
                                                                    2 as
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
                    ret =
                        alloc_data(&mut clear,
                                   (2 as libc::c_int as
                                        libc::c_ulong).wrapping_add(strlen(strresult.as_mut_ptr()))
                                       as libc::c_uint);
                    if !(ret != 0) {
                        ptr = clear.data;
                        let fresh6 = ptr;
                        ptr = ptr.offset(1);
                        *fresh6 =
                            (numresult >> 8 as libc::c_int &
                                 0xff as libc::c_int) as libc::c_char;
                        let fresh7 = ptr;
                        ptr = ptr.offset(1);
                        *fresh7 =
                            (numresult & 0xff as libc::c_int) as libc::c_char;
                        memcpy(ptr as *mut libc::c_void,
                               strresult.as_mut_ptr() as *const libc::c_void,
                               strlen(strresult.as_mut_ptr()));
                        cipher = empty_data();
                        if ap_rep.length != 0 {
                            ret =
                                krb5_auth_con_setaddrs(context, auth_context,
                                                       (*local_addr).address,
                                                       0 as
                                                           *mut krb5_address);
                            if ret != 0 {
                                numresult = 2 as libc::c_int;
                                krb5int_strlcpy(strresult.as_mut_ptr(),
                                                b"Failed storing client and server internet addresses\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                    as libc::c_ulong);
                            } else {
                                ret =
                                    krb5_mk_priv(context, auth_context,
                                                 &mut clear, &mut cipher,
                                                 &mut replay);
                                if ret != 0 {
                                    numresult = 2 as libc::c_int;
                                    krb5int_strlcpy(strresult.as_mut_ptr(),
                                                    b"Failed encrypting reply\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                        as libc::c_ulong);
                                }
                            }
                        }
                        /* if no KRB-PRIV was constructed, then we need a KRB-ERROR.
       if this fails, just bail.  there's nothing else we can do. */
                        if cipher.length == 0 as libc::c_int as libc::c_uint {
                            /* clear out ap_rep now, so that it won't be inserted in the
           reply */
                            if ap_rep.length != 0 {
                                free(ap_rep.data as *mut libc::c_void);
                                ap_rep = empty_data()
                            }
                            krberror.ctime = 0 as libc::c_int;
                            krberror.cusec = 0 as libc::c_int;
                            krberror.susec = 0 as libc::c_int;
                            ret =
                                krb5_timeofday(context, &mut krberror.stime);
                            if ret != 0 {
                                current_block = 4314144610356015125;
                            } else {
                                /* this is really icky.  but it's what all the other callers
           to mk_error do. */
                                krberror.error = ret as krb5_ui_4;
                                krberror.error =
                                    (krberror.error as libc::c_long -
                                         -(1765328384 as libc::c_long)) as
                                        krb5_ui_4;
                                if krberror.error >
                                       127 as libc::c_int as libc::c_uint {
                                    krberror.error =
                                        60 as libc::c_int as krb5_ui_4
                                }
                                krberror.client = 0 as krb5_principal;
                                ret =
                                    krb5_build_principal(context,
                                                         &mut krberror.server
                                                             as
                                                             *mut krb5_principal,
                                                         strlen(realm) as
                                                             libc::c_uint,
                                                         realm,
                                                         b"kadmin\x00" as
                                                             *const u8 as
                                                             *const libc::c_char,
                                                         b"changepw\x00" as
                                                             *const u8 as
                                                             *const libc::c_char,
                                                         0 as
                                                             *mut libc::c_void);
                                if ret != 0 {
                                    current_block = 4314144610356015125;
                                } else {
                                    krberror.text.length =
                                        0 as libc::c_int as libc::c_uint;
                                    krberror.e_data = clear;
                                    ret =
                                        krb5_mk_error(context, &mut krberror,
                                                      &mut cipher);
                                    krb5_free_principal(context,
                                                        krberror.server);
                                    if ret != 0 {
                                        current_block = 4314144610356015125;
                                    } else {
                                        current_block = 17648591037158480576;
                                    }
                                }
                            }
                        } else { current_block = 17648591037158480576; }
                        match current_block {
                            4314144610356015125 => { }
                            _ => {
                                /* construct the reply */
                                ret =
                                    alloc_data(rep,
                                               (6 as libc::c_int as
                                                    libc::c_uint).wrapping_add(ap_rep.length).wrapping_add(cipher.length));
                                if !(ret != 0) {
                                    ptr = (*rep).data;
                                    /* length */
                                    let fresh8 = ptr;
                                    ptr = ptr.offset(1);
                                    *fresh8 =
                                        ((*rep).length >> 8 as libc::c_int &
                                             0xff as libc::c_int as
                                                 libc::c_uint) as
                                            libc::c_char;
                                    let fresh9 = ptr;
                                    ptr = ptr.offset(1);
                                    *fresh9 =
                                        ((*rep).length &
                                             0xff as libc::c_int as
                                                 libc::c_uint) as
                                            libc::c_char;
                                    /* version == 0x0001 big-endian */
                                    let fresh10 = ptr;
                                    ptr = ptr.offset(1);
                                    *fresh10 =
                                        0 as libc::c_int as libc::c_char;
                                    let fresh11 = ptr;
                                    ptr = ptr.offset(1);
                                    *fresh11 =
                                        1 as libc::c_int as libc::c_char;
                                    /* ap_rep length, big-endian */
                                    let fresh12 = ptr;
                                    ptr = ptr.offset(1);
                                    *fresh12 =
                                        (ap_rep.length >> 8 as libc::c_int &
                                             0xff as libc::c_int as
                                                 libc::c_uint) as
                                            libc::c_char;
                                    let fresh13 = ptr;
                                    ptr = ptr.offset(1);
                                    *fresh13 =
                                        (ap_rep.length &
                                             0xff as libc::c_int as
                                                 libc::c_uint) as
                                            libc::c_char;
                                    /* ap-rep data */
                                    if ap_rep.length != 0 {
                                        memcpy(ptr as *mut libc::c_void,
                                               ap_rep.data as
                                                   *const libc::c_void,
                                               ap_rep.length as
                                                   libc::c_ulong);
                                        ptr =
                                            ptr.offset(ap_rep.length as isize)
                                    }
                                    /* krb-priv or krb-error */
                                    memcpy(ptr as *mut libc::c_void,
                                           cipher.data as *const libc::c_void,
                                           cipher.length as libc::c_ulong);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    krb5_auth_con_free(context, auth_context);
    krb5_free_principal(context, changepw);
    krb5_free_ticket(context, ticket);
    free(ap_rep.data as *mut libc::c_void);
    free(clear.data as *mut libc::c_void);
    free(cipher.data as *mut libc::c_void);
    krb5_free_principal(context, target);
    krb5_free_unparsed_name(context, targetstr);
    krb5_free_unparsed_name(context, clientstr);
    krb5_free_error_message(context, errmsg);
    return ret;
}
/* Dispatch routine for set/change password */
#[no_mangle]
#[c2rust::src_loc = "432:1"]
pub unsafe extern "C" fn dispatch(mut handle: *mut libc::c_void,
                                  mut local_addr: *const krb5_fulladdr,
                                  mut remote_addr: *const krb5_fulladdr,
                                  mut request: *mut krb5_data,
                                  mut is_tcp: libc::c_int,
                                  mut vctx: *mut verto_ctx,
                                  mut respond: loop_respond_fn,
                                  mut arg: *mut libc::c_void) {
    let mut ret: krb5_error_code = 0;
    let mut kt: krb5_keytab = 0 as krb5_keytab;
    let mut server_handle: kadm5_server_handle_t =
        handle as kadm5_server_handle_t;
    let mut response: *mut krb5_data = 0 as *mut krb5_data;
    let mut emsg: *const libc::c_char = 0 as *const libc::c_char;
    ret =
        krb5_kt_resolve((*server_handle).context,
                        b"KDB:\x00" as *const u8 as *const libc::c_char,
                        &mut kt);
    if ret != 0 as libc::c_int {
        emsg = krb5_get_error_message((*server_handle).context, ret);
        krb5_klog_syslog(3 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"chpw: Couldn\'t open admin keytab %s\x00"
                                      as *const u8 as *const libc::c_char),
                         emsg);
        krb5_free_error_message((*server_handle).context, emsg);
    } else {
        response =
            k5alloc(::std::mem::size_of::<krb5_data>() as libc::c_ulong,
                    &mut ret) as *mut krb5_data;
        if !response.is_null() {
            ret =
                process_chpw_request((*server_handle).context, handle,
                                     (*server_handle).params.realm, kt,
                                     local_addr, remote_addr, request,
                                     response)
        }
    }
    if ret != 0 { krb5_free_data((*server_handle).context, response); }
    krb5_kt_close((*server_handle).context, kt);
    Some(respond.expect("non-null function pointer")).expect("non-null function pointer")(arg,
                                                                                          ret,
                                                                                          if ret
                                                                                                 ==
                                                                                                 0
                                                                                                     as
                                                                                                     libc::c_int
                                                                                             {
                                                                                              response
                                                                                          } else {
                                                                                              0
                                                                                                  as
                                                                                                  *mut krb5_data
                                                                                          });
}
