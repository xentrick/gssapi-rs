use ::libc;

pub mod k5_thread_h {

    #[inline]

    pub unsafe extern "C" fn k5_mutex_init(mut m: *mut crate::k5_thread_h::k5_mutex_t) -> i32 {
        return crate::k5_thread_h::k5_os_mutex_init(m);
    }
    #[inline]

    pub unsafe extern "C" fn k5_mutex_lock(mut m: *mut crate::k5_thread_h::k5_mutex_t) {
        let mut r = crate::k5_thread_h::k5_os_mutex_lock(m);
        if r != 0i32 {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"k5_mutex_lock: Received error %d (%s)\n\x00" as *const u8 as *const i8,
                r,
                crate::stdlib::strerror(r),
            );
        }
        if r == 0i32 {
        } else {
            crate::stdlib::__assert_fail(
                b"r == 0\x00" as *const u8 as *const i8,
                b"../../../include/k5-thread.h\x00" as *const u8 as *const i8,
                376u32,
                (*::std::mem::transmute::<&[u8; 33], &[i8; 33]>(
                    b"void k5_mutex_lock(k5_mutex_t *)\x00",
                ))
                .as_ptr(),
            );
        };
    }
    #[inline]

    pub unsafe extern "C" fn k5_mutex_unlock(mut m: *mut crate::k5_thread_h::k5_mutex_t) {
        let mut r = crate::k5_thread_h::k5_os_mutex_unlock(m);
        if r != 0i32 {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"k5_mutex_unlock: Received error %d (%s)\n\x00" as *const u8 as *const i8,
                r,
                crate::stdlib::strerror(r),
            );
        }
        if r == 0i32 {
        } else {
            crate::stdlib::__assert_fail(
                b"r == 0\x00" as *const u8 as *const i8,
                b"../../../include/k5-thread.h\x00" as *const u8 as *const i8,
                388u32,
                (*::std::mem::transmute::<&[u8; 35], &[i8; 35]>(
                    b"void k5_mutex_unlock(k5_mutex_t *)\x00",
                ))
                .as_ptr(),
            );
        };
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

    #[inline]

    pub unsafe extern "C" fn zapfreestr(mut str: *mut libc::c_void) {
        if !str.is_null() {
            crate::stdlib::explicit_bzero(str, crate::stdlib::strlen(str as *mut i8));
            crate::stdlib::free(str);
        };
    }
    #[inline]

    pub unsafe extern "C" fn data_eq_string(
        mut d: crate::krb5_h::krb5_data,
        mut s: *const i8,
    ) -> i32 {
        return (d.length as usize == crate::stdlib::strlen(s)
            && (d.length == 0u32
                || crate::stdlib::memcmp(
                    d.data as *const libc::c_void,
                    s as *const libc::c_void,
                    d.length as usize,
                ) == 0)) as i32;
    }
    #[inline]

    pub unsafe extern "C" fn make_data(
        mut data: *mut libc::c_void,
        mut len: u32,
    ) -> crate::krb5_h::krb5_data {
        let mut d = crate::krb5_h::krb5_data {
            magic: 0,
            length: 0,
            data: 0 as *mut i8,
        };
        d.magic = -(1760647422 as isize) as crate::krb5_h::krb5_magic;
        d.data = data as *mut i8;
        d.length = len;
        return d;
    }
    #[inline]

    pub unsafe extern "C" fn empty_data() -> crate::krb5_h::krb5_data {
        return make_data(0 as *mut libc::c_void, 0u32);
    }
    #[inline]

    pub unsafe extern "C" fn string2data(mut str: *mut i8) -> crate::krb5_h::krb5_data {
        return make_data(str as *mut libc::c_void, crate::stdlib::strlen(str) as u32);
    }
    #[inline]

    pub unsafe extern "C" fn k5calloc(
        mut nmemb: crate::stddef_h::size_t,
        mut size: crate::stddef_h::size_t,
        mut code: *mut crate::krb5_h::krb5_error_code,
    ) -> *mut libc::c_void {
        let mut ptr = 0 as *mut libc::c_void;
        ptr = crate::stdlib::calloc(
            if nmemb != 0 { nmemb } else { 1usize },
            if size != 0 { size } else { 1usize },
        );
        *code = if ptr.is_null() { 12i32 } else { 0i32 };
        return ptr;
    }
    #[inline]

    pub unsafe extern "C" fn k5alloc(
        mut size: crate::stddef_h::size_t,
        mut code: *mut crate::krb5_h::krb5_error_code,
    ) -> *mut libc::c_void {
        return k5calloc(1usize, size, code);
    }
    #[inline]

    pub unsafe extern "C" fn ts2tt(
        mut timestamp: crate::krb5_h::krb5_timestamp,
    ) -> crate::stdlib::time_t {
        return timestamp as crate::stdlib::time_t;
    }
    #[inline]

    pub unsafe extern "C" fn ts_delta(
        mut a: crate::krb5_h::krb5_timestamp,
        mut b: crate::krb5_h::krb5_timestamp,
    ) -> crate::krb5_h::krb5_deltat {
        return (a as crate::stdlib::uint32_t).wrapping_sub(b as crate::stdlib::uint32_t)
            as crate::krb5_h::krb5_deltat;
    }
    #[inline]

    pub unsafe extern "C" fn ts_incr(
        mut ts: crate::krb5_h::krb5_timestamp,
        mut delta: crate::krb5_h::krb5_deltat,
    ) -> crate::krb5_h::krb5_timestamp {
        return (ts as crate::stdlib::uint32_t).wrapping_add(delta as crate::stdlib::uint32_t)
            as crate::krb5_h::krb5_timestamp;
    }
    #[inline]

    pub unsafe extern "C" fn ts_after(
        mut a: crate::krb5_h::krb5_timestamp,
        mut b: crate::krb5_h::krb5_timestamp,
    ) -> crate::krb5_h::krb5_boolean {
        return (a as crate::stdlib::uint32_t > b as crate::stdlib::uint32_t)
            as crate::krb5_h::krb5_boolean;
    }

    /* _KRB5_INT_H */
}

/* K5_TRACE_H */

/* DISABLE_TRACING */

/* Try to optimize away argument evaluation and function call when we're not
 * tracing, if this source file knows the internals of the context. */
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__pthread_internal_list;
pub use crate::stdlib::__pthread_list_t;
pub use crate::stdlib::__pthread_mutex_s;
pub use crate::stdlib::__time_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::pthread_mutex_t;
pub use crate::stdlib::time_t;

pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::uint32_t;
pub use crate::stdlib::uint8_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::k5_err_h::errinfo;
pub use crate::k5_int_h::_kdb5_dal_handle;
pub use crate::k5_int_h::_kdb_log_context;
pub use crate::k5_int_h::_krb5_authdata_context;
pub use crate::k5_int_h::_krb5_authdata_context_module;
pub use crate::k5_int_h::_krb5_context;
pub use crate::k5_int_h::_krb5_kt;
pub use crate::k5_int_h::_krb5_kt_ops;
pub use crate::k5_int_h::_krb5_os_context;
pub use crate::k5_int_h::ccselect_module_handle;
pub use crate::k5_int_h::dns_canonhost;
pub use crate::k5_int_h::hostrealm_module_handle;
pub use crate::k5_int_h::k5_change_error_message_code;
pub use crate::k5_int_h::k5_kt_get_principal;
pub use crate::k5_int_h::k5_rc_close;
pub use crate::k5_int_h::k5_rc_resolve;
pub use crate::k5_int_h::k5_tls_vtable_st;
pub use crate::k5_int_h::kdb5_dal_handle;
pub use crate::k5_int_h::krb5_authdata_context;
pub use crate::k5_int_h::krb5_preauth_context;
pub use crate::k5_int_h::krb5_preauth_context_st;
pub use crate::k5_int_h::krb5int_cc_default;
pub use crate::k5_int_h::krb5int_copy_data_contents_add0;
pub use crate::k5_int_h::localauth_module_handle;
pub use crate::k5_int_h::plugin_interface;
pub use crate::k5_int_h::plugin_mapping;
pub use crate::k5_int_h::CANONHOST_FALLBACK;
pub use crate::k5_int_h::CANONHOST_FALSE;
pub use crate::k5_int_h::CANONHOST_TRUE;
pub use crate::k5_plugin_h::plugin_dir_handle;
pub use crate::k5_plugin_h::plugin_file_handle;
pub use crate::k5_thread_h::k5_mutex_t;
pub use crate::k5_thread_h::k5_os_mutex;
pub use crate::k5_thread_h::k5_os_mutex_destroy;
pub use crate::k5_thread_h::k5_os_mutex_init;
pub use crate::k5_thread_h::k5_os_mutex_lock;
pub use crate::k5_thread_h::k5_os_mutex_unlock;
pub use crate::krb5_h::_krb5_address;
pub use crate::krb5_h::_krb5_ap_req;
pub use crate::krb5_h::_krb5_auth_context;
pub use crate::krb5_h::_krb5_authdata;
pub use crate::krb5_h::_krb5_ccache;
pub use crate::krb5_h::_krb5_creds;
pub use crate::krb5_h::_krb5_data;
pub use crate::krb5_h::_krb5_enc_data;
pub use crate::krb5_h::_krb5_enc_tkt_part;
pub use crate::krb5_h::_krb5_get_init_creds_opt;
pub use crate::krb5_h::_krb5_keyblock;
pub use crate::krb5_h::_krb5_prompt;
pub use crate::krb5_h::_krb5_ticket;
pub use crate::krb5_h::_krb5_ticket_times;
pub use crate::krb5_h::_krb5_trace_info;
pub use crate::krb5_h::_krb5_transited;
pub use crate::krb5_h::_profile_t;
pub use crate::krb5_h::krb5_address;
pub use crate::krb5_h::krb5_addrtype;
pub use crate::krb5_h::krb5_ap_req;
pub use crate::krb5_h::krb5_auth_context;
pub use crate::krb5_h::krb5_authdata;
pub use crate::krb5_h::krb5_authdatatype;
pub use crate::krb5_h::krb5_boolean;
pub use crate::krb5_h::krb5_build_principal_ext;
pub use crate::krb5_h::krb5_cc_cache_match;
pub use crate::krb5_h::krb5_cc_close;
pub use crate::krb5_h::krb5_cc_cursor;
pub use crate::krb5_h::krb5_cc_default;
pub use crate::krb5_h::krb5_cc_destroy;
pub use crate::krb5_h::krb5_cc_dup;
pub use crate::krb5_h::krb5_cc_end_seq_get;
pub use crate::krb5_h::krb5_cc_get_principal;
pub use crate::krb5_h::krb5_cc_get_type;
pub use crate::krb5_h::krb5_cc_new_unique;
pub use crate::krb5_h::krb5_cc_next_cred;
pub use crate::krb5_h::krb5_cc_resolve;
pub use crate::krb5_h::krb5_cc_select;
pub use crate::krb5_h::krb5_cc_set_config;
pub use crate::krb5_h::krb5_cc_set_flags;
pub use crate::krb5_h::krb5_cc_start_seq_get;
pub use crate::krb5_h::krb5_cc_support_switch;
pub use crate::krb5_h::krb5_ccache;
pub use crate::krb5_h::krb5_cccol_have_content;
pub use crate::krb5_h::krb5_clear_error_message;
pub use crate::krb5_h::krb5_const_principal;
pub use crate::krb5_h::krb5_context;
pub use crate::krb5_h::krb5_creds;
pub use crate::krb5_h::krb5_data;
pub use crate::krb5_h::krb5_deltat;
pub use crate::krb5_h::krb5_enc_data;
pub use crate::krb5_h::krb5_enc_tkt_part;
pub use crate::krb5_h::krb5_enctype;
pub use crate::krb5_h::krb5_error_code;
pub use crate::krb5_h::krb5_flags;
pub use crate::krb5_h::krb5_free_context;
pub use crate::krb5_h::krb5_free_cred_contents;
pub use crate::krb5_h::krb5_free_data_contents;
pub use crate::krb5_h::krb5_free_keytab_entry_contents;
pub use crate::krb5_h::krb5_free_principal;
pub use crate::krb5_h::krb5_get_init_creds_keytab;
pub use crate::krb5_h::krb5_get_init_creds_opt;
pub use crate::krb5_h::krb5_get_init_creds_opt_alloc;
pub use crate::krb5_h::krb5_get_init_creds_opt_free;
pub use crate::krb5_h::krb5_get_init_creds_opt_set_out_ccache;
pub use crate::krb5_h::krb5_get_init_creds_password;
pub use crate::krb5_h::krb5_get_server_rcache;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_is_config_principal;
pub use crate::krb5_h::krb5_keyblock;
pub use crate::krb5_h::krb5_keytab;
pub use crate::krb5_h::krb5_keytab_entry;
pub use crate::krb5_h::krb5_keytab_entry_st;
pub use crate::krb5_h::krb5_kt_client_default;
pub use crate::krb5_h::krb5_kt_close;
pub use crate::krb5_h::krb5_kt_cursor;
pub use crate::krb5_h::krb5_kt_default;
pub use crate::krb5_h::krb5_kt_dup;
pub use crate::krb5_h::krb5_kt_end_seq_get;
pub use crate::krb5_h::krb5_kt_free_entry;
pub use crate::krb5_h::krb5_kt_get_entry;
pub use crate::krb5_h::krb5_kt_have_content;
pub use crate::krb5_h::krb5_kt_next_entry;
pub use crate::krb5_h::krb5_kt_resolve;
pub use crate::krb5_h::krb5_kt_start_seq_get;
pub use crate::krb5_h::krb5_kvno;
pub use crate::krb5_h::krb5_magic;
pub use crate::krb5_h::krb5_octet;
pub use crate::krb5_h::krb5_parse_name;
pub use crate::krb5_h::krb5_pointer;
pub use crate::krb5_h::krb5_post_recv_fn;
pub use crate::krb5_h::krb5_pre_send_fn;
pub use crate::krb5_h::krb5_preauthtype;
pub use crate::krb5_h::krb5_principal;
pub use crate::krb5_h::krb5_principal_compare;
pub use crate::krb5_h::krb5_principal_data;
pub use crate::krb5_h::krb5_prompt;
pub use crate::krb5_h::krb5_prompt_type;
pub use crate::krb5_h::krb5_prompter_fct;
pub use crate::krb5_h::krb5_rc_st;
pub use crate::krb5_h::krb5_rcache;
pub use crate::krb5_h::krb5_set_error_message;
pub use crate::krb5_h::krb5_sname_match;
pub use crate::krb5_h::krb5_ticket;
pub use crate::krb5_h::krb5_ticket_times;
pub use crate::krb5_h::krb5_timeofday;
pub use crate::krb5_h::krb5_timestamp;
pub use crate::krb5_h::krb5_trace_callback;
pub use crate::krb5_h::krb5_trace_info;
pub use crate::krb5_h::krb5_transited;
pub use crate::krb5_h::krb5_unparse_name;
pub use crate::profile_h::profile_t;
pub use crate::src::krb5::acquire_cred::k5_int_h::data_eq_string;
pub use crate::src::krb5::acquire_cred::k5_int_h::empty_data;
pub use crate::src::krb5::acquire_cred::k5_int_h::k5alloc;
pub use crate::src::krb5::acquire_cred::k5_int_h::k5calloc;
pub use crate::src::krb5::acquire_cred::k5_int_h::make_data;
pub use crate::src::krb5::acquire_cred::k5_int_h::string2data;
pub use crate::src::krb5::acquire_cred::k5_int_h::ts2tt;
pub use crate::src::krb5::acquire_cred::k5_int_h::ts_after;
pub use crate::src::krb5::acquire_cred::k5_int_h::ts_delta;
pub use crate::src::krb5::acquire_cred::k5_int_h::ts_incr;
pub use crate::src::krb5::acquire_cred::k5_int_h::zapfreestr;
pub use crate::src::krb5::acquire_cred::k5_thread_h::k5_mutex_init;
pub use crate::src::krb5::acquire_cred::k5_thread_h::k5_mutex_lock;
pub use crate::src::krb5::acquire_cred::k5_thread_h::k5_mutex_unlock;

pub use crate::authdata_plugin_h::authdata_client_copy_proc;
pub use crate::authdata_plugin_h::authdata_client_delete_attribute_proc;
pub use crate::authdata_plugin_h::authdata_client_export_authdata_proc;
pub use crate::authdata_plugin_h::authdata_client_export_internal_proc;
pub use crate::authdata_plugin_h::authdata_client_externalize_proc;
pub use crate::authdata_plugin_h::authdata_client_free_internal_proc;
pub use crate::authdata_plugin_h::authdata_client_get_attribute_proc;
pub use crate::authdata_plugin_h::authdata_client_get_attribute_types_proc;
pub use crate::authdata_plugin_h::authdata_client_import_authdata_proc;
pub use crate::authdata_plugin_h::authdata_client_internalize_proc;
pub use crate::authdata_plugin_h::authdata_client_plugin_fini_proc;
pub use crate::authdata_plugin_h::authdata_client_plugin_flags_proc;
pub use crate::authdata_plugin_h::authdata_client_plugin_init_proc;
pub use crate::authdata_plugin_h::authdata_client_request_fini_proc;
pub use crate::authdata_plugin_h::authdata_client_request_init_proc;
pub use crate::authdata_plugin_h::authdata_client_set_attribute_proc;
pub use crate::authdata_plugin_h::authdata_client_size_proc;
pub use crate::authdata_plugin_h::authdata_client_verify_proc;
pub use crate::authdata_plugin_h::krb5plugin_authdata_client_ftable_v0;
pub use crate::gssapiP_krb5_h::_krb5_gss_cred_id_rec;
pub use crate::gssapiP_krb5_h::_krb5_gss_name_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_cred_id_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
pub use crate::gssapiP_krb5_h::krb5_gss_import_cred_req;
pub use crate::gssapiP_krb5_h::krb5_gss_name_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_name_t;
pub use crate::gssapi_ext_h::gss_const_key_value_set_t;
pub use crate::gssapi_ext_h::gss_key_value_element_desc;
pub use crate::gssapi_ext_h::gss_key_value_element_struct;
pub use crate::gssapi_ext_h::gss_key_value_set_desc;
pub use crate::gssapi_ext_h::gss_key_value_set_struct;
pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_OID_set;
pub use crate::gssapi_h::gss_OID_set_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_cred_id_t;
pub use crate::gssapi_h::gss_cred_usage_t;
pub use crate::gssapi_h::gss_name_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::mglueP_h::gss_cred_id_struct;
pub use crate::mglueP_h::gss_name_struct;
pub use crate::src::krb5::cred_store::kg_value_from_cred_store;
pub use crate::src::krb5::disp_status::krb5_gss_save_error_info;
pub use crate::src::krb5::gssapi_krb5::gss_krb5int_initialize_library;
pub use crate::src::krb5::gssapi_krb5::kg_caller_provided_ccache_name;
pub use crate::src::krb5::gssapi_krb5::kg_sync_ccache_name;
pub use crate::src::krb5::init_sec_context::krb5_gss_init_context;
pub use crate::src::krb5::naming_exts::kg_acceptor_princ;
pub use crate::src::krb5::naming_exts::kg_duplicate_name;
pub use crate::src::krb5::naming_exts::kg_init_name;
pub use crate::src::krb5::naming_exts::kg_release_name;
pub use crate::src::krb5::val_cred::krb5_gss_validate_cred_1;

/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2000, 2007-2010 by the Massachusetts Institute of Technology.
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
#[no_mangle]

pub static mut gssint_krb5_keytab_lock: crate::k5_thread_h::k5_mutex_t =
    crate::stdlib::pthread_mutex_t {
        __data: {
            let mut init = crate::stdlib::__pthread_mutex_s {
                __lock: 0i32,
                __count: 0u32,
                __owner: 0i32,
                __nusers: 0u32,
                __kind: 0i32,
                __spins: 0i16,
                __elision: 0i16,
                __list: {
                    let mut init = crate::stdlib::__pthread_internal_list {
                        __prev: 0 as *mut crate::stdlib::__pthread_internal_list,
                        __next: 0 as *mut crate::stdlib::__pthread_internal_list,
                    };
                    init
                },
            };
            init
        },
    };

static mut krb5_gss_keytab: *mut i8 = 0 as *mut i8;
/* Heimdal calls this gsskrb5_register_acceptor_identity. */
#[no_mangle]

pub unsafe extern "C" fn gss_krb5int_register_acceptor_identity(
    mut _minor_status: *mut crate::gssapi_h::OM_uint32,
    _desired_mech: crate::gssapi_h::gss_OID,
    _desired_object: crate::gssapi_h::gss_OID,
    mut value: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut new = 0 as *mut i8;
    let mut old = 0 as *mut i8;
    let mut err: i32 = 0;
    err = crate::src::krb5::gssapi_krb5::gss_krb5int_initialize_library() as i32;
    if err != 0i32 {
        return (13u32) << 16i32;
    }
    if !(*value).value.is_null() {
        new = crate::stdlib::strdup((*value).value as *mut i8);
        if new.is_null() {
            return (13u32) << 16i32;
        }
    }
    k5_mutex_lock(&mut gssint_krb5_keytab_lock);
    old = krb5_gss_keytab;
    krb5_gss_keytab = new;
    k5_mutex_unlock(&mut gssint_krb5_keytab_lock);
    crate::stdlib::free(old as *mut libc::c_void);
    return 0u32;
}
/* Try to verify that keytab contains at least one entry for name.  Return 0 if
 * it does, KRB5_KT_NOTFOUND if it doesn't, or another error as appropriate. */

unsafe extern "C" fn check_keytab(
    mut context: crate::krb5_h::krb5_context,
    mut kt: crate::krb5_h::krb5_keytab,
    mut name: crate::gssapiP_krb5_h::krb5_gss_name_t,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut ent = crate::krb5_h::krb5_keytab_entry {
        magic: 0,
        principal: 0 as *mut crate::krb5_h::krb5_principal_data,
        timestamp: 0,
        vno: 0,
        key: crate::krb5_h::krb5_keyblock {
            magic: 0,
            enctype: 0,
            length: 0,
            contents: 0 as *mut crate::krb5_h::krb5_octet,
        },
    };
    let mut cursor = 0 as *mut libc::c_void;
    let mut accprinc = 0 as crate::krb5_h::krb5_principal;
    let mut match_0: crate::krb5_h::krb5_boolean = 0;
    let mut princname = 0 as *mut i8;
    if (*name).service.is_null() {
        code = crate::krb5_h::krb5_kt_get_entry(
            context,
            kt,
            (*name).princ as crate::krb5_h::krb5_const_principal,
            0u32,
            0i32,
            &mut ent,
        );
        if code == 0i32 {
            crate::krb5_h::krb5_kt_free_entry(context, &mut ent);
        }
        return code;
    }
    /* If we can't iterate through the keytab, skip this check. */
    if (*(*kt).ops).start_seq_get.is_none() {
        return 0i32;
    }
    /* Get the partial principal for the acceptor name. */
    code = crate::src::krb5::naming_exts::kg_acceptor_princ(context, name, &mut accprinc)
        as crate::krb5_h::krb5_error_code;
    if code != 0 {
        return code;
    }
    /* Scan the keytab for host-based entries matching accprinc. */
    code = crate::krb5_h::krb5_kt_start_seq_get(context, kt, &mut cursor);
    if !(code != 0) {
        loop {
            code = crate::krb5_h::krb5_kt_next_entry(context, kt, &mut ent, &mut cursor);
            if !(code == 0i32) {
                break;
            }
            match_0 = crate::krb5_h::krb5_sname_match(
                context,
                accprinc as crate::krb5_h::krb5_const_principal,
                ent.principal as crate::krb5_h::krb5_const_principal,
            );
            crate::krb5_h::krb5_free_keytab_entry_contents(context, &mut ent);
            if match_0 != 0 {
                break;
            }
        }
        crate::krb5_h::krb5_kt_end_seq_get(context, kt, &mut cursor);
        if code as isize == -(1765328202 as isize) {
            code = -(1765328203 as isize) as crate::krb5_h::krb5_error_code;
            if crate::krb5_h::krb5_unparse_name(
                context,
                accprinc as crate::krb5_h::krb5_const_principal,
                &mut princname,
            ) == 0i32
            {
                crate::krb5_h::krb5_set_error_message(
                    context,
                    code,
                    crate::stdlib::dgettext(
                        b"mit-krb5\x00" as *const u8 as *const i8,
                        b"No key table entry found matching %s\x00" as *const u8 as *const i8,
                    ),
                    princname,
                );
                crate::stdlib::free(princname as *mut libc::c_void);
            }
        }
    }
    crate::krb5_h::krb5_free_principal(context, accprinc);
    return code;
}
/* get credentials corresponding to a key in the krb5 keytab.
   If successful, set the keytab-specific fields in cred
*/

unsafe extern "C" fn acquire_accept_cred(
    mut context: crate::krb5_h::krb5_context,
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut req_keytab: crate::krb5_h::krb5_keytab,
    mut rcname: *const i8,
    mut cred: *mut crate::gssapiP_krb5_h::krb5_gss_cred_id_rec,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut kt = 0 as crate::krb5_h::krb5_keytab;
    let mut rc = 0 as crate::krb5_h::krb5_rcache;
    if (*cred).keytab.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"cred->keytab == NULL\x00" as *const u8 as
                          *const i8,
                      b"acquire_cred.c\x00" as *const u8 as
                          *const i8,
                      190u32,
                      (*::std::mem::transmute::<&[u8; 108],
                                                &[i8; 108]>(b"OM_uint32 acquire_accept_cred(krb5_context, OM_uint32 *, krb5_keytab, const char *, krb5_gss_cred_id_rec *)\x00")).as_ptr());
    }
    /* If we have an explicit rcache name, open it. */
    if !rcname.is_null() {
        code = crate::k5_int_h::k5_rc_resolve(context, rcname, &mut rc);
        if code != 0 {
            major = (13u32) << 16i32;
            current_block = 6869834285901761501;
        } else {
            current_block = 10886091980245723256;
        }
    } else {
        current_block = 10886091980245723256;
    }
    match current_block {
        10886091980245723256 => {
            if !req_keytab.is_null() {
                code = crate::krb5_h::krb5_kt_dup(context, req_keytab, &mut kt)
            } else {
                k5_mutex_lock(&mut gssint_krb5_keytab_lock);
                if !krb5_gss_keytab.is_null() {
                    code = crate::krb5_h::krb5_kt_resolve(context, krb5_gss_keytab, &mut kt);
                    k5_mutex_unlock(&mut gssint_krb5_keytab_lock);
                } else {
                    k5_mutex_unlock(&mut gssint_krb5_keytab_lock);
                    code = crate::krb5_h::krb5_kt_default(context, &mut kt)
                }
            }
            if code != 0 {
                major = (13u32) << 16i32
            } else {
                if !(*cred).name.is_null() {
                    /* Make sure we have keys matching the desired name in the keytab. */
                    code = check_keytab(context, kt, (*cred).name);
                    if code != 0 {
                        if code as isize == -(1765328203 as isize) {
                            crate::k5_int_h::k5_change_error_message_code(
                                context,
                                code,
                                39756033i32,
                            );
                            code = 39756033i32
                        }
                        major = (13u32) << 16i32;
                        current_block = 6869834285901761501;
                    } else if rc.is_null() {
                        /* Open the replay cache for this principal. */
                        code = crate::krb5_h::krb5_get_server_rcache(
                            context,
                            &mut *(*(*(*cred).name).princ).data.offset(0isize),
                            &mut rc,
                        );
                        if code != 0 {
                            major = (13u32) << 16i32;
                            current_block = 6869834285901761501;
                        } else {
                            current_block = 13131896068329595644;
                        }
                    } else {
                        current_block = 13131896068329595644;
                    }
                } else {
                    /* Make sure we have a keytab with keys in it. */
                    code = crate::krb5_h::krb5_kt_have_content(context, kt);
                    if code != 0 {
                        major = (13u32) << 16i32;
                        current_block = 6869834285901761501;
                    } else {
                        current_block = 13131896068329595644;
                    }
                }
                match current_block {
                    6869834285901761501 => {}
                    _ => {
                        (*cred).keytab = kt;
                        kt = 0 as crate::krb5_h::krb5_keytab;
                        (*cred).rcache = rc;
                        rc = 0 as crate::krb5_h::krb5_rcache;
                        major = 0u32
                    }
                }
            }
        }
        _ => {}
    }
    if !kt.is_null() {
        crate::krb5_h::krb5_kt_close(context, kt);
    }
    if !rc.is_null() {
        crate::k5_int_h::k5_rc_close(context, rc);
    }
    *minor_status = code as crate::gssapi_h::OM_uint32;
    return major;
}
/* LEAN_CLIENT */
/* USE_LEASH */
/* Set fields in cred according to a ccache config entry whose key (in
 * principal form) is config_princ and whose value is value. */

unsafe extern "C" fn scan_cc_config(
    mut context: crate::krb5_h::krb5_context,
    mut cred: *mut crate::gssapiP_krb5_h::krb5_gss_cred_id_rec,
    mut config_princ: crate::krb5_h::krb5_const_principal,
    mut value: *const crate::krb5_h::krb5_data,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut data0 = empty_data();
    if (*config_princ).length != 2i32 {
        return 0i32;
    }
    if data_eq_string(
        *(*config_princ).data.offset(1isize),
        b"proxy_impersonator\x00" as *const u8 as *const i8,
    ) != 0
        && (*cred).impersonator.is_null()
    {
        code = crate::k5_int_h::krb5int_copy_data_contents_add0(context, value, &mut data0);
        if code != 0 {
            return code;
        }
        code = crate::krb5_h::krb5_parse_name(context, data0.data, &mut (*cred).impersonator);
        crate::krb5_h::krb5_free_data_contents(context, &mut data0);
        if code != 0 {
            return code;
        }
    } else if data_eq_string(
        *(*config_princ).data.offset(1isize),
        b"refresh_time\x00" as *const u8 as *const i8,
    ) != 0
        && (*cred).refresh_time == 0i32
    {
        code = crate::k5_int_h::krb5int_copy_data_contents_add0(context, value, &mut data0);
        if code != 0 {
            return code;
        }
        (*cred).refresh_time = crate::stdlib::atol(data0.data) as crate::krb5_h::krb5_timestamp;
        crate::krb5_h::krb5_free_data_contents(context, &mut data0);
    }
    return 0i32;
}
/* Return true if it appears that we can non-interactively get initial
 * tickets for cred. */

unsafe extern "C" fn can_get_initial_creds(
    mut context: crate::krb5_h::krb5_context,
    mut cred: *mut crate::gssapiP_krb5_h::krb5_gss_cred_id_rec,
) -> crate::krb5_h::krb5_boolean {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut entry = crate::krb5_h::krb5_keytab_entry {
        magic: 0,
        principal: 0 as *mut crate::krb5_h::krb5_principal_data,
        timestamp: 0,
        vno: 0,
        key: crate::krb5_h::krb5_keyblock {
            magic: 0,
            enctype: 0,
            length: 0,
            contents: 0 as *mut crate::krb5_h::krb5_octet,
        },
    };
    if !(*cred).password.is_null() {
        return 1u32;
    }
    if (*cred).client_keytab.is_null() {
        return 0u32;
    }
    /* If we don't know the client principal yet, check for any keytab keys. */
    if (*cred).name.is_null() {
        return (crate::krb5_h::krb5_kt_have_content(context, (*cred).client_keytab) == 0)
            as crate::krb5_h::krb5_boolean;
    }
    /* Check if we have a keytab key for the client principal. */
    code = crate::krb5_h::krb5_kt_get_entry(
        context,
        (*cred).client_keytab,
        (*(*cred).name).princ as crate::krb5_h::krb5_const_principal,
        0u32,
        0i32,
        &mut entry,
    );
    if code != 0 {
        crate::krb5_h::krb5_clear_error_message(context);
        return 0u32;
    }
    crate::krb5_h::krb5_free_keytab_entry_contents(context, &mut entry);
    return 1u32;
}
/* Scan cred->ccache for name, expiry time, impersonator, refresh time. */

unsafe extern "C" fn scan_ccache(
    mut context: crate::krb5_h::krb5_context,
    mut cred: *mut crate::gssapiP_krb5_h::krb5_gss_cred_id_rec,
) -> crate::krb5_h::krb5_error_code {
    let mut current_block: u64;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut ccache = (*cred).ccache;
    let mut ccache_princ = 0 as crate::krb5_h::krb5_principal;
    let mut tgt_princ = 0 as crate::krb5_h::krb5_principal;
    let mut realm = 0 as *mut crate::krb5_h::krb5_data;
    let mut cursor = 0 as *mut libc::c_void;
    let mut creds = crate::krb5_h::krb5_creds {
        magic: 0,
        client: 0 as *mut crate::krb5_h::krb5_principal_data,
        server: 0 as *mut crate::krb5_h::krb5_principal_data,
        keyblock: crate::krb5_h::krb5_keyblock {
            magic: 0,
            enctype: 0,
            length: 0,
            contents: 0 as *mut crate::krb5_h::krb5_octet,
        },
        times: crate::krb5_h::krb5_ticket_times {
            authtime: 0,
            starttime: 0,
            endtime: 0,
            renew_till: 0,
        },
        is_skey: 0,
        ticket_flags: 0,
        addresses: 0 as *mut *mut crate::krb5_h::krb5_address,
        ticket: crate::krb5_h::krb5_data {
            magic: 0,
            length: 0,
            data: 0 as *mut i8,
        },
        second_ticket: crate::krb5_h::krb5_data {
            magic: 0,
            length: 0,
            data: 0 as *mut i8,
        },
        authdata: 0 as *mut *mut crate::krb5_h::krb5_authdata,
    };
    let mut endtime: crate::krb5_h::krb5_timestamp = 0;
    let mut is_tgt: crate::krb5_h::krb5_boolean = 0;
    /* Turn on NOTICKET, as we don't need session keys here. */
    code = crate::krb5_h::krb5_cc_set_flags(context, ccache, 0x2i32);
    if code != 0 {
        return code;
    }
    /* Credentials cache principal must match the initiator name. */
    code = crate::krb5_h::krb5_cc_get_principal(context, ccache, &mut ccache_princ);
    if !(code != 0i32) {
        if !(*cred).name.is_null()
            && crate::krb5_h::krb5_principal_compare(
                context,
                ccache_princ as crate::krb5_h::krb5_const_principal,
                (*(*cred).name).princ as crate::krb5_h::krb5_const_principal,
            ) == 0
        {
            code = 39756032i32
        } else {
            /* Save the ccache principal as the credential name if not already set. */
            if (*cred).name.is_null() {
                code = crate::src::krb5::naming_exts::kg_init_name(
                    context,
                    ccache_princ,
                    0 as *mut i8,
                    0 as *mut i8,
                    0 as crate::k5_int_h::krb5_authdata_context,
                    0x1i32,
                    &mut (*cred).name,
                );
                if code != 0 {
                    current_block = 16016496521494030081;
                } else {
                    ccache_princ = 0 as crate::krb5_h::krb5_principal;
                    current_block = 1054647088692577877;
                }
            } else {
                current_block = 1054647088692577877;
            }
            match current_block {
                16016496521494030081 => {}
                _ => {
                    if !(*(*cred).name).princ.is_null() {
                    } else {
                        crate::stdlib::__assert_fail(b"cred->name->princ != NULL\x00" as
                                          *const u8 as *const i8,
                                      b"acquire_cred.c\x00" as *const u8 as
                                          *const i8,
                                      401u32,
                                      (*::std::mem::transmute::<&[u8; 66],
                                                                &[i8; 66]>(b"krb5_error_code scan_ccache(krb5_context, krb5_gss_cred_id_rec *)\x00")).as_ptr());
                    }
                    realm = &mut (*(*(*cred).name).princ).realm;
                    code = crate::krb5_h::krb5_build_principal_ext(
                        context,
                        &mut tgt_princ as *mut crate::krb5_h::krb5_principal,
                        (*realm).length,
                        (*realm).data,
                        6i32,
                        b"krbtgt\x00" as *const u8 as *const i8,
                        (*realm).length,
                        (*realm).data,
                        0i32,
                    );
                    if code != 0 {
                        return code;
                    }
                    /* If there's a tgt for the principal's local realm in here, use its expiry
                     * time.  Otherwise use the first key. */
                    code = crate::krb5_h::krb5_cc_start_seq_get(context, ccache, &mut cursor);
                    if code != 0 {
                        crate::krb5_h::krb5_free_principal(context, tgt_princ);
                        return code;
                    }
                    loop {
                        code = crate::krb5_h::krb5_cc_next_cred(
                            context,
                            ccache,
                            &mut cursor,
                            &mut creds,
                        );
                        if !(code == 0) {
                            break;
                        }
                        if crate::krb5_h::krb5_is_config_principal(
                            context,
                            creds.server as crate::krb5_h::krb5_const_principal,
                        ) != 0
                        {
                            code = scan_cc_config(
                                context,
                                cred,
                                creds.server as crate::krb5_h::krb5_const_principal,
                                &mut creds.ticket,
                            );
                            crate::krb5_h::krb5_free_cred_contents(context, &mut creds);
                            if code != 0 {
                                break;
                            }
                        } else {
                            is_tgt = crate::krb5_h::krb5_principal_compare(
                                context,
                                tgt_princ as crate::krb5_h::krb5_const_principal,
                                creds.server as crate::krb5_h::krb5_const_principal,
                            );
                            endtime = creds.times.endtime;
                            crate::krb5_h::krb5_free_cred_contents(context, &mut creds);
                            if is_tgt != 0 {
                                (*cred).have_tgt = 1u32
                            }
                            if is_tgt != 0 || (*cred).expire == 0i32 {
                                (*cred).expire = endtime
                            }
                        }
                    }
                    crate::krb5_h::krb5_cc_end_seq_get(context, ccache, &mut cursor);
                    if !(code != 0 && code as isize != -(1765328242 as isize)) {
                        code = 0i32;
                        if (*cred).expire == 0i32 && can_get_initial_creds(context, cred) == 0 {
                            code = 39756044i32
                        }
                    }
                }
            }
        }
    }
    crate::krb5_h::krb5_cc_set_flags(context, ccache, 0i32);
    crate::krb5_h::krb5_free_principal(context, ccache_princ);
    crate::krb5_h::krb5_free_principal(context, tgt_princ);
    return code;
}
/* Find an existing or destination ccache for cred->name. */

unsafe extern "C" fn get_cache_for_name(
    mut context: crate::krb5_h::krb5_context,
    mut cred: *mut crate::gssapiP_krb5_h::krb5_gss_cred_id_rec,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut can_get: crate::krb5_h::krb5_boolean = 0;
    let mut have_collection: crate::krb5_h::krb5_boolean = 0;
    let mut defcc = 0 as crate::krb5_h::krb5_ccache;
    let mut princ = 0 as crate::krb5_h::krb5_principal;
    let mut cctype = 0 as *const i8;
    if !(*cred).name.is_null() && (*cred).ccache.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"cred->name != NULL && cred->ccache == NULL\x00" as *const u8 as *const i8,
            b"acquire_cred.c\x00" as *const u8 as *const i8,
            461u32,
            (*::std::mem::transmute::<&[u8; 73], &[i8; 73]>(
                b"krb5_error_code get_cache_for_name(krb5_context, krb5_gss_cred_id_rec *)\x00",
            ))
            .as_ptr(),
        );
    }
    /* Check first whether we can acquire tickets, to avoid overwriting the
     * extended error message from krb5_cc_cache_match. */
    can_get = can_get_initial_creds(context, cred);
    /* Look for an existing cache for the client principal. */
    code = crate::krb5_h::krb5_cc_cache_match(context, (*(*cred).name).princ, &mut (*cred).ccache);
    if code == 0i32 {
        return scan_ccache(context, cred);
    }
    if code as isize != -(1765328243 as isize) || can_get == 0 {
        return code;
    }
    crate::krb5_h::krb5_clear_error_message(context);
    /* There is no existing ccache, but we can acquire credentials.  Get the
     * default ccache to help decide where we should put them. */
    code = crate::krb5_h::krb5_cc_default(context, &mut defcc);
    if code != 0 {
        return code;
    }
    cctype = crate::krb5_h::krb5_cc_get_type(context, defcc);
    have_collection = crate::krb5_h::krb5_cc_support_switch(context, cctype);
    /* We can use an empty default ccache if we're using a password or if
     * there's no collection. */
    if !(*cred).password.is_null() || have_collection == 0 {
        if crate::krb5_h::krb5_cc_get_principal(context, defcc, &mut princ) as isize
            == -(1765328189 as isize)
        {
            (*cred).ccache = defcc;
            defcc = 0 as crate::krb5_h::krb5_ccache
        }
        crate::krb5_h::krb5_clear_error_message(context);
    }
    /* Otherwise, try to use a new cache in the collection. */
    if (*cred).ccache.is_null() {
        if have_collection == 0 {
            code = 39756032i32
        } else {
            code = crate::krb5_h::krb5_cc_new_unique(
                context,
                cctype,
                0 as *const i8,
                &mut (*cred).ccache,
            );
            (code) != 0;
        }
    }
    crate::krb5_h::krb5_free_principal(context, princ);
    if !defcc.is_null() {
        crate::krb5_h::krb5_cc_close(context, defcc);
    }
    return code;
    /* not USE_LEASH */
}
/* Try to set cred->name using the client keytab. */

unsafe extern "C" fn get_name_from_client_keytab(
    mut context: crate::krb5_h::krb5_context,
    mut cred: *mut crate::gssapiP_krb5_h::krb5_gss_cred_id_rec,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut princ = 0 as *mut crate::krb5_h::krb5_principal_data;
    if (*cred).name.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"cred->name == NULL\x00" as *const u8 as
                          *const i8,
                      b"acquire_cred.c\x00" as *const u8 as
                          *const i8,
                      522u32,
                      (*::std::mem::transmute::<&[u8; 82],
                                                &[i8; 82]>(b"krb5_error_code get_name_from_client_keytab(krb5_context, krb5_gss_cred_id_rec *)\x00")).as_ptr());
    }
    if (*cred).client_keytab.is_null() {
        return -(1765328203 as isize) as crate::krb5_h::krb5_error_code;
    }
    code = crate::k5_int_h::k5_kt_get_principal(context, (*cred).client_keytab, &mut princ);
    if code != 0 {
        return code;
    }
    code = crate::src::krb5::naming_exts::kg_init_name(
        context,
        princ,
        0 as *mut i8,
        0 as *mut i8,
        0 as crate::k5_int_h::krb5_authdata_context,
        0x1i32,
        &mut (*cred).name,
    );
    if code != 0 {
        crate::krb5_h::krb5_free_principal(context, princ);
        return code;
    }
    return 0i32;
}
/* Make a note in ccache that we should attempt to refresh it from the client
 * keytab at refresh_time. */

unsafe extern "C" fn set_refresh_time(
    mut context: crate::krb5_h::krb5_context,
    mut ccache: crate::krb5_h::krb5_ccache,
    mut refresh_time: crate::krb5_h::krb5_timestamp,
) {
    let mut buf: [i8; 128] = [0; 128];
    let mut d = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    crate::stdlib::snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 128]>(),
        b"%u\x00" as *const u8 as *const i8,
        ts2tt(refresh_time) as u32,
    );
    d = string2data(buf.as_mut_ptr());
    crate::krb5_h::krb5_cc_set_config(
        context,
        ccache,
        0 as crate::krb5_h::krb5_const_principal,
        b"refresh_time\x00" as *const u8 as *const i8,
        &mut d,
    );
    crate::krb5_h::krb5_clear_error_message(context);
}
/* Return true if it's time to refresh cred from the client keytab.  If
 * returning true, avoid retrying for 30 seconds. */
#[no_mangle]

pub unsafe extern "C" fn kg_cred_time_to_refresh(
    mut context: crate::krb5_h::krb5_context,
    mut cred: *mut crate::gssapiP_krb5_h::krb5_gss_cred_id_rec,
) -> crate::krb5_h::krb5_boolean {
    let mut now: crate::krb5_h::krb5_timestamp = 0;
    if crate::krb5_h::krb5_timeofday(context, &mut now) != 0 {
        return 0u32;
    }
    if (*cred).refresh_time != 0i32 && ts_after((*cred).refresh_time, now) == 0 {
        set_refresh_time(
            context,
            (*cred).ccache,
            ts_incr((*cred).refresh_time, 30i32),
        );
        return 1u32;
    }
    return 0u32;
}
/* If appropriate, make a note to refresh cred from the client keytab when it
 * is halfway to expired. */
#[no_mangle]

pub unsafe extern "C" fn kg_cred_set_initial_refresh(
    mut context: crate::krb5_h::krb5_context,
    mut cred: *mut crate::gssapiP_krb5_h::krb5_gss_cred_id_rec,
    mut times: *mut crate::krb5_h::krb5_ticket_times,
) {
    let mut refresh: crate::krb5_h::krb5_timestamp = 0;
    /* For now, we only mark keytab-acquired credentials for refresh. */
    if !(*cred).password.is_null() {
        return;
    }
    /* Make a note to refresh these when they are halfway to expired. */
    refresh = ts_incr(
        (*times).starttime,
        ts_delta((*times).endtime, (*times).starttime) / 2i32,
    );
    set_refresh_time(context, (*cred).ccache, refresh);
}
/* Get initial credentials using the supplied password or client keytab. */

unsafe extern "C" fn get_initial_cred(
    mut context: crate::krb5_h::krb5_context,
    mut cred: *mut crate::gssapiP_krb5_h::krb5_gss_cred_id_rec,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut opt = 0 as *mut crate::krb5_h::krb5_get_init_creds_opt;
    let mut creds = crate::krb5_h::krb5_creds {
        magic: 0,
        client: 0 as *mut crate::krb5_h::krb5_principal_data,
        server: 0 as *mut crate::krb5_h::krb5_principal_data,
        keyblock: crate::krb5_h::krb5_keyblock {
            magic: 0,
            enctype: 0,
            length: 0,
            contents: 0 as *mut crate::krb5_h::krb5_octet,
        },
        times: crate::krb5_h::krb5_ticket_times {
            authtime: 0,
            starttime: 0,
            endtime: 0,
            renew_till: 0,
        },
        is_skey: 0,
        ticket_flags: 0,
        addresses: 0 as *mut *mut crate::krb5_h::krb5_address,
        ticket: crate::krb5_h::krb5_data {
            magic: 0,
            length: 0,
            data: 0 as *mut i8,
        },
        second_ticket: crate::krb5_h::krb5_data {
            magic: 0,
            length: 0,
            data: 0 as *mut i8,
        },
        authdata: 0 as *mut *mut crate::krb5_h::krb5_authdata,
    };
    code = crate::krb5_h::krb5_get_init_creds_opt_alloc(context, &mut opt);
    if code != 0 {
        return code;
    }
    code = crate::krb5_h::krb5_get_init_creds_opt_set_out_ccache(context, opt, (*cred).ccache);
    if !(code != 0) {
        if !(*cred).password.is_null() {
            code = crate::krb5_h::krb5_get_init_creds_password(
                context,
                &mut creds,
                (*(*cred).name).princ,
                (*cred).password,
                None,
                0 as *mut libc::c_void,
                0i32,
                0 as *const i8,
                opt,
            )
        } else if !(*cred).client_keytab.is_null() {
            code = crate::krb5_h::krb5_get_init_creds_keytab(
                context,
                &mut creds,
                (*(*cred).name).princ,
                (*cred).client_keytab,
                0i32,
                0 as *const i8,
                opt,
            )
        } else {
            code = -(1765328203 as isize) as crate::krb5_h::krb5_error_code
        }
        if !(code != 0) {
            kg_cred_set_initial_refresh(context, cred, &mut creds.times);
            (*cred).have_tgt = 1u32;
            (*cred).expire = creds.times.endtime;
            crate::krb5_h::krb5_free_cred_contents(context, &mut creds);
        }
    }
    crate::krb5_h::krb5_get_init_creds_opt_free(context, opt);
    return code;
}
/* Get initial credentials if we ought to and are able to. */

unsafe extern "C" fn maybe_get_initial_cred(
    mut context: crate::krb5_h::krb5_context,
    mut cred: *mut crate::gssapiP_krb5_h::krb5_gss_cred_id_rec,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    /* Don't get creds if we don't know the name or are doing IAKERB. */
    if (*cred).name.is_null() || (*cred).iakerb_mech() as i32 != 0 {
        return 0i32;
    }
    /* Get creds if we have none or if it's time to refresh. */
    if (*cred).expire == 0i32 || kg_cred_time_to_refresh(context, cred) != 0 {
        code = get_initial_cred(context, cred);
        /* If we were trying to refresh and failed, we can keep going. */
        if code != 0 && (*cred).expire == 0i32 {
            return code;
        }
        crate::krb5_h::krb5_clear_error_message(context);
    }
    return 0i32;
}

unsafe extern "C" fn acquire_init_cred(
    mut context: crate::krb5_h::krb5_context,
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut req_ccache: crate::krb5_h::krb5_ccache,
    mut password: crate::gssapi_h::gss_buffer_t,
    mut client_keytab: crate::krb5_h::krb5_keytab,
    mut cred: *mut crate::gssapiP_krb5_h::krb5_gss_cred_id_rec,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut pwdata = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    let mut pwcopy = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    let mut caller_ccname = 0i32;
    /* Get ccache from caller if available. */
    if crate::src::krb5::gssapi_krb5::kg_sync_ccache_name(context, minor_status)
        & ((0o377u32) << 24i32 | (0o377u32) << 16i32)
        != 0
    {
        return (13u32) << 16i32;
    }
    if crate::src::krb5::gssapi_krb5::kg_caller_provided_ccache_name(
        minor_status,
        &mut caller_ccname,
    ) & ((0o377u32) << 24i32 | (0o377u32) << 16i32)
        != 0
    {
        return (13u32) << 16i32;
    }
    if !password.is_null() {
        pwdata = make_data((*password).value, (*password).length as u32);
        code = crate::k5_int_h::krb5int_copy_data_contents_add0(context, &mut pwdata, &mut pwcopy);
        if code != 0 {
            current_block = 15700575537292975296;
        } else {
            (*cred).password = pwcopy.data;
            /* We will fetch the credential into a private memory ccache. */
            if req_ccache.is_null() {
            } else {
                crate::stdlib::__assert_fail(b"req_ccache == NULL\x00" as *const u8 as
                                  *const i8,
                              b"acquire_cred.c\x00" as *const u8 as
                                  *const i8,
                              673u32,
                              (*::std::mem::transmute::<&[u8; 119],
                                                        &[i8; 119]>(b"OM_uint32 acquire_init_cred(krb5_context, OM_uint32 *, krb5_ccache, gss_buffer_t, krb5_keytab, krb5_gss_cred_id_rec *)\x00")).as_ptr());
            }
            code = crate::krb5_h::krb5_cc_new_unique(
                context,
                b"MEMORY\x00" as *const u8 as *const i8,
                0 as *const i8,
                &mut (*cred).ccache,
            );
            if code != 0 {
                current_block = 15700575537292975296;
            } else {
                (*cred).set_destroy_ccache(1u32);
                current_block = 12147880666119273379;
            }
        }
    } else if !req_ccache.is_null() {
        code = crate::krb5_h::krb5_cc_dup(context, req_ccache, &mut (*cred).ccache);
        if code != 0 {
            current_block = 15700575537292975296;
        } else {
            current_block = 12147880666119273379;
        }
    } else if caller_ccname != 0 {
        /* Caller's ccache name has been set as the context default. */
        code = crate::k5_int_h::krb5int_cc_default(context, &mut (*cred).ccache);
        if code != 0 {
            current_block = 15700575537292975296;
        } else {
            current_block = 12147880666119273379;
        }
    } else {
        current_block = 12147880666119273379;
    }
    match current_block {
        12147880666119273379 => {
            if !client_keytab.is_null() {
                code =
                    crate::krb5_h::krb5_kt_dup(context, client_keytab, &mut (*cred).client_keytab)
            } else {
                code = crate::krb5_h::krb5_kt_client_default(context, &mut (*cred).client_keytab);
                if code != 0 {
                    /* Treat resolution failure similarly to a client keytab which
                     * resolves but doesn't exist or has no content. */
                    if (*context).trace_callback.is_some() {
                        crate::k5_trace_h::krb5int_trace(
                            context,
                            b"Unable to resolve default client keytab: {kerr}\x00" as *const u8
                                as *const i8,
                            code,
                        );
                    }
                    crate::krb5_h::krb5_clear_error_message(context);
                    code = 0i32
                }
            }
            if !(code != 0) {
                if !(*cred).ccache.is_null() {
                    /* The caller specified a ccache; check what's in it. */
                    code = scan_ccache(context, cred);
                    if code as isize == -(1765328189 as isize) {
                        /* See if we can get initial creds.  If the caller didn't specify
                         * a name, pick one from the client keytab. */
                        if (*cred).name.is_null() {
                            if get_name_from_client_keytab(context, cred) == 0 {
                                code = 0i32
                            }
                        } else if can_get_initial_creds(context, cred) != 0 {
                            code = 0i32
                        }
                    }
                    if code != 0 {
                        current_block = 15700575537292975296;
                    } else {
                        current_block = 13678349939556791712;
                    }
                } else if !(*cred).name.is_null() {
                    /* The caller specified a name but not a ccache; pick a cache. */
                    code = get_cache_for_name(context, cred);
                    if code != 0 {
                        current_block = 15700575537292975296;
                    } else {
                        current_block = 13678349939556791712;
                    }
                } else {
                    current_block = 13678349939556791712;
                }
                match current_block {
                    15700575537292975296 => {}
                    _ =>
                    /* If we haven't picked a name, make sure we have or can get any creds,
                     * unless we're using Leash and might be able to get them interactively. */
                    {
                        if (*cred).name.is_null() && can_get_initial_creds(context, cred) == 0 {
                            code = crate::krb5_h::krb5_cccol_have_content(context);
                            if code != 0 {
                                current_block = 15700575537292975296;
                            } else {
                                current_block = 2290177392965769716;
                            }
                        } else {
                            current_block = 2290177392965769716;
                        }
                        match current_block {
                            15700575537292975296 => {}
                            _ => {
                                code = maybe_get_initial_cred(context, cred);
                                if !(code != 0) {
                                    *minor_status = 0u32;
                                    return 0u32;
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    *minor_status = code as crate::gssapi_h::OM_uint32;
    return (13u32) << 16i32;
}

unsafe extern "C" fn acquire_cred_context(
    mut context: crate::krb5_h::krb5_context,
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut desired_name: crate::gssapi_h::gss_name_t,
    mut password: crate::gssapi_h::gss_buffer_t,
    mut _time_req: crate::gssapi_h::OM_uint32,
    mut cred_usage: crate::gssapi_h::gss_cred_usage_t,
    mut ccache: crate::krb5_h::krb5_ccache,
    mut client_keytab: crate::krb5_h::krb5_keytab,
    mut keytab: crate::krb5_h::krb5_keytab,
    mut rcname: *const i8,
    mut iakerb: crate::krb5_h::krb5_boolean,
    mut output_cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut cred = 0 as crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
    let mut name = desired_name as crate::gssapiP_krb5_h::krb5_gss_name_t;
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut code = 0i32;
    /* make sure all outputs are valid */
    *output_cred_handle = 0 as crate::gssapi_h::gss_cred_id_t;
    if !time_rec.is_null() {
        *time_rec = 0u32
    }
    /* create the gss cred structure */
    cred = k5alloc(
        ::std::mem::size_of::<crate::gssapiP_krb5_h::krb5_gss_cred_id_rec>(),
        &mut code,
    ) as crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
    if cred.is_null() {
        current_block = 6108249593228632422;
    } else {
        (*cred).usage = cred_usage;
        (*cred).name = 0 as crate::gssapiP_krb5_h::krb5_gss_name_t;
        (*cred).impersonator = 0 as crate::krb5_h::krb5_principal;
        (*cred).set_iakerb_mech(iakerb);
        (*cred).set_default_identity((name == 0 as crate::gssapiP_krb5_h::krb5_gss_name_t) as u32);
        (*cred).keytab = 0 as crate::krb5_h::krb5_keytab;
        /* LEAN_CLIENT */
        (*cred).set_destroy_ccache(0u32);
        (*cred).set_suppress_ci_flags(0u32);
        (*cred).ccache = 0 as crate::krb5_h::krb5_ccache;
        code = k5_mutex_init(&mut (*cred).lock);
        if code != 0 {
            current_block = 6108249593228632422;
        } else {
            match cred_usage {
                1 | 2 | 0 => {
                    if !name.is_null() {
                        code = crate::src::krb5::naming_exts::kg_duplicate_name(
                            context,
                            name,
                            &mut (*cred).name,
                        );
                        if code != 0 {
                            current_block = 6108249593228632422;
                        } else {
                            current_block = 13472856163611868459;
                        }
                    } else {
                        current_block = 13472856163611868459;
                    }
                    match current_block {
                        6108249593228632422 => {}
                        _ =>
                        /*
                         * If requested, acquire credentials for accepting. This will fill
                         * in cred->name if desired_princ is specified.
                         */
                        {
                            if cred_usage == 2i32 || cred_usage == 0i32 {
                                ret = acquire_accept_cred(
                                    context,
                                    minor_status,
                                    keytab,
                                    rcname,
                                    cred,
                                );
                                if ret != 0u32 {
                                    current_block = 11183415134345190833;
                                } else {
                                    current_block = 15345278821338558188;
                                }
                            } else {
                                current_block = 15345278821338558188;
                            }
                            match current_block {
                                11183415134345190833 => {}
                                _ =>
                                /* LEAN_CLIENT */
                                /*
                                 * If requested, acquire credentials for initiation. This will fill
                                 * in cred->name if it wasn't set above.
                                 */
                                {
                                    if cred_usage == 1i32 || cred_usage == 0i32 {
                                        ret = acquire_init_cred(
                                            context,
                                            minor_status,
                                            ccache,
                                            password,
                                            client_keytab,
                                            cred,
                                        );
                                        if ret != 0u32 {
                                            current_block = 11183415134345190833;
                                        } else {
                                            current_block = 15897653523371991391;
                                        }
                                    } else {
                                        current_block = 15897653523371991391;
                                    }
                                    match current_block {
                                        11183415134345190833 => {}
                                        _ => {
                                            if (*cred).default_identity() as i32 != 0
                                                || !(*cred).name.is_null()
                                            {
                                            } else {
                                                crate::stdlib::__assert_fail(b"cred->default_identity || cred->name != NULL\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const i8,
                                                              b"acquire_cred.c\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const i8,
                                                              828u32,
                                                              (*::std::mem::transmute::<&[u8; 210],
                                                                                        &[i8; 210]>(b"OM_uint32 acquire_cred_context(krb5_context, OM_uint32 *, gss_name_t, gss_buffer_t, OM_uint32, gss_cred_usage_t, krb5_ccache, krb5_keytab, krb5_keytab, const char *, krb5_boolean, gss_cred_id_t *, OM_uint32 *)\x00")).as_ptr());
                                            }
                                            /* ** at this point, the cred structure has been completely created */
                                            if cred_usage == 2i32 {
                                                if !time_rec.is_null() {
                                                    *time_rec = 0xffffffffu32
                                                }
                                                current_block = 1622411330066726685;
                                            } else {
                                                let mut now: crate::krb5_h::krb5_timestamp = 0;
                                                code = crate::krb5_h::krb5_timeofday(
                                                    context, &mut now,
                                                );
                                                if code != 0i32 {
                                                    current_block = 6108249593228632422;
                                                } else if !time_rec.is_null() {
                                                    /* Resolve cred now to determine the expiration time. */
                                                    ret = kg_cred_resolve(
                                                        minor_status,
                                                        context,
                                                        cred as crate::gssapi_h::gss_cred_id_t,
                                                        0 as crate::gssapi_h::gss_name_t,
                                                    );
                                                    if ret
                                                        & ((0o377u32) << 24i32
                                                            | (0o377u32) << 16i32)
                                                        != 0
                                                    {
                                                        current_block = 11183415134345190833;
                                                    } else {
                                                        *time_rec =
                                                            if ts_after((*cred).expire, now) != 0 {
                                                                ts_delta((*cred).expire, now)
                                                            } else {
                                                                0i32
                                                            }
                                                                as crate::gssapi_h::OM_uint32;
                                                        k5_mutex_unlock(&mut (*cred).lock);
                                                        current_block = 1622411330066726685;
                                                    }
                                                } else {
                                                    current_block = 1622411330066726685;
                                                }
                                            }
                                            match current_block {
                                                11183415134345190833 => {}
                                                6108249593228632422 => {}
                                                _ => {
                                                    *minor_status = 0u32;
                                                    *output_cred_handle =
                                                        cred as crate::gssapi_h::gss_cred_id_t;
                                                    return 0u32;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {
                    ret = (13u32) << 16i32;
                    *minor_status = -(2045022969 as isize) as crate::gssapi_h::OM_uint32;
                    current_block = 11183415134345190833;
                }
            }
        }
    }
    match current_block {
        6108249593228632422 => {
            *minor_status = code as crate::gssapi_h::OM_uint32;
            ret = (13u32) << 16i32
        }
        _ => {}
    }
    if !cred.is_null() {
        if !(*cred).ccache.is_null() {
            if (*cred).destroy_ccache() != 0 {
                crate::krb5_h::krb5_cc_destroy(context, (*cred).ccache);
            } else {
                crate::krb5_h::krb5_cc_close(context, (*cred).ccache);
            }
        }
        if !(*cred).client_keytab.is_null() {
            crate::krb5_h::krb5_kt_close(context, (*cred).client_keytab);
        }
        if !(*cred).keytab.is_null() {
            crate::krb5_h::krb5_kt_close(context, (*cred).keytab);
        }
        /* LEAN_CLIENT */
        if !(*cred).rcache.is_null() {
            crate::k5_int_h::k5_rc_close(context, (*cred).rcache);
        }
        if !(*cred).name.is_null() {
            crate::src::krb5::naming_exts::kg_release_name(context, &mut (*cred).name);
        }
        crate::krb5_h::krb5_free_principal(context, (*cred).impersonator);
        zapfreestr((*cred).password as *mut libc::c_void);
        crate::k5_thread_h::k5_os_mutex_destroy(&mut (*cred).lock);
        crate::stdlib::free(cred as *mut libc::c_void);
    }
    crate::src::krb5::disp_status::krb5_gss_save_error_info(*minor_status, context);
    return ret;
}

unsafe extern "C" fn acquire_cred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut desired_name: crate::gssapi_h::gss_name_t,
    mut password: crate::gssapi_h::gss_buffer_t,
    mut time_req: crate::gssapi_h::OM_uint32,
    mut cred_usage: crate::gssapi_h::gss_cred_usage_t,
    mut ccache: crate::krb5_h::krb5_ccache,
    mut keytab: crate::krb5_h::krb5_keytab,
    mut iakerb: crate::krb5_h::krb5_boolean,
    mut output_cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut context = 0 as crate::krb5_h::krb5_context;
    let mut code = 0i32;
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    code = crate::src::krb5::gssapi_krb5::gss_krb5int_initialize_library()
        as crate::krb5_h::krb5_error_code;
    if code != 0 {
        *minor_status = code as crate::gssapi_h::OM_uint32;
        ret = (13u32) << 16i32
    } else {
        code = crate::src::krb5::init_sec_context::krb5_gss_init_context(&mut context);
        if code != 0 {
            *minor_status = code as crate::gssapi_h::OM_uint32;
            ret = (13u32) << 16i32
        } else {
            ret = acquire_cred_context(
                context,
                minor_status,
                desired_name,
                password,
                time_req,
                cred_usage,
                ccache,
                0 as crate::krb5_h::krb5_keytab,
                keytab,
                0 as *const i8,
                iakerb,
                output_cred_handle,
                time_rec,
            )
        }
    }
    crate::krb5_h::krb5_free_context(context);
    return ret;
}
/*
 * Resolve the name and ccache for an initiator credential if it has not yet
 * been done.  If specified, use the target name to pick an appropriate ccache
 * within the collection.  Validates cred_handle and leaves it locked on
 * success.
 */
#[no_mangle]

pub unsafe extern "C" fn kg_cred_resolve(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context: crate::krb5_h::krb5_context,
    mut cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut target_name: crate::gssapi_h::gss_name_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut maj: crate::gssapi_h::OM_uint32 = 0;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut cred = cred_handle as crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
    let mut tname = target_name as crate::gssapiP_krb5_h::krb5_gss_name_t;
    let mut client_princ = 0 as *mut crate::krb5_h::krb5_principal_data;
    *minor_status = 0u32;
    maj = crate::src::krb5::val_cred::krb5_gss_validate_cred_1(minor_status, cred_handle, context);
    if maj != 0u32 {
        return maj;
    }
    if (*cred).usage == 2i32 || !(*cred).name.is_null() {
        return 0u32;
    }
    /* acquire_init_cred should have set both name and ccache, or neither. */
    if (*cred).ccache.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"cred->ccache == NULL\x00" as *const u8 as
                          *const i8,
                      b"acquire_cred.c\x00" as *const u8 as
                          *const i8,
                      950u32,
                      (*::std::mem::transmute::<&[u8; 80],
                                                &[i8; 80]>(b"OM_uint32 kg_cred_resolve(OM_uint32 *, krb5_context, gss_cred_id_t, gss_name_t)\x00")).as_ptr());
    }
    if !tname.is_null() {
        /* Use the target name to select an existing ccache or a principal. */
        code = crate::krb5_h::krb5_cc_select(
            context,
            (*tname).princ,
            &mut (*cred).ccache,
            &mut client_princ,
        );
        if code != 0 && code as isize != -(1765328243 as isize) {
            current_block = 721532972098336643;
        } else {
            if !client_princ.is_null() {
                code = crate::src::krb5::naming_exts::kg_init_name(
                    context,
                    client_princ,
                    0 as *mut i8,
                    0 as *mut i8,
                    0 as crate::k5_int_h::krb5_authdata_context,
                    0x1i32,
                    &mut (*cred).name,
                );
                if code != 0 {
                    crate::krb5_h::krb5_free_principal(context, client_princ);
                    current_block = 721532972098336643;
                } else {
                    current_block = 15976848397966268834;
                }
            } else {
                current_block = 15976848397966268834;
            }
            match current_block {
                721532972098336643 => {}
                _ => {
                    if !(*cred).ccache.is_null() {
                        code = scan_ccache(context, cred);
                        if code != 0 {
                            current_block = 721532972098336643;
                        } else {
                            current_block = 4495394744059808450;
                        }
                    } else {
                        current_block = 4495394744059808450;
                    }
                }
            }
        }
    } else {
        current_block = 4495394744059808450;
    }
    match current_block {
        4495394744059808450 =>
        /* If we still haven't picked a client principal, try using an existing
         * default ccache.  (On Windows, this may acquire initial creds.) */
        {
            if (*cred).name.is_null() {
                code = crate::k5_int_h::krb5int_cc_default(context, &mut (*cred).ccache);
                if code != 0 {
                    current_block = 721532972098336643;
                } else {
                    code = scan_ccache(context, cred);
                    if code as isize == -(1765328189 as isize) {
                        /* Default ccache doesn't exist; fall through to client keytab. */
                        crate::krb5_h::krb5_cc_close(context, (*cred).ccache);
                        (*cred).ccache = 0 as crate::krb5_h::krb5_ccache;
                        current_block = 11636175345244025579;
                    } else if code != 0 {
                        current_block = 721532972098336643;
                    } else {
                        current_block = 11636175345244025579;
                    }
                }
            } else {
                current_block = 11636175345244025579;
            }
            match current_block {
                721532972098336643 => {}
                _ =>
                /* If that didn't work, try getting a name from the client keytab. */
                {
                    if (*cred).name.is_null() {
                        code = get_name_from_client_keytab(context, cred);
                        if code != 0 {
                            code = 39756044i32;
                            current_block = 721532972098336643;
                        } else {
                            current_block = 2604890879466389055;
                        }
                    } else {
                        current_block = 2604890879466389055;
                    }
                    match current_block {
                        721532972098336643 => {}
                        _ => {
                            if !(*cred).name.is_null() && (*cred).ccache.is_null() {
                                /* Pick a cache for the name we chose (from krb5_cc_select or from the
                                 * client keytab). */
                                code = get_cache_for_name(context, cred);
                                if code != 0 {
                                    current_block = 721532972098336643;
                                } else {
                                    current_block = 9007357115414505193;
                                }
                            } else {
                                current_block = 9007357115414505193;
                            }
                            match current_block {
                                721532972098336643 => {}
                                _ => {
                                    /* Resolve name to ccache and possibly get initial credentials. */
                                    code = maybe_get_initial_cred(context, cred);
                                    if !(code != 0) {
                                        return 0u32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    k5_mutex_unlock(&mut (*cred).lock);
    crate::src::krb5::disp_status::krb5_gss_save_error_info(
        code as crate::gssapi_h::OM_uint32,
        context,
    );
    *minor_status = code as crate::gssapi_h::OM_uint32;
    return (13u32) << 16i32;
}
#[no_mangle]

pub unsafe extern "C" fn gss_krb5int_set_cred_rcache(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    _desired_oid: crate::gssapi_h::gss_OID,
    value: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut cred = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_cred_id_rec;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut context = 0 as *mut crate::k5_int_h::_krb5_context;
    let mut rcache = 0 as *mut crate::krb5_h::krb5_rc_st;
    if (*value).length == ::std::mem::size_of::<crate::krb5_h::krb5_rcache>() {
    } else {
        crate::stdlib::__assert_fail(b"value->length == sizeof(rcache)\x00" as *const u8 as
                          *const i8,
                      b"acquire_cred.c\x00" as *const u8 as
                          *const i8,
                      1031u32,
                      (*::std::mem::transmute::<&[u8; 103],
                                                &[i8; 103]>(b"OM_uint32 gss_krb5int_set_cred_rcache(OM_uint32 *, gss_cred_id_t *, const gss_OID, const gss_buffer_t)\x00")).as_ptr());
    }
    if (*value).length != ::std::mem::size_of::<crate::krb5_h::krb5_rcache>() {
        return (13u32) << 16i32;
    }
    rcache = (*value).value as crate::krb5_h::krb5_rcache;
    cred = *cred_handle as crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
    code = crate::src::krb5::init_sec_context::krb5_gss_init_context(&mut context);
    if code != 0 {
        *minor_status = code as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    if !(*cred).rcache.is_null() {
        crate::k5_int_h::k5_rc_close(context, (*cred).rcache);
    }
    (*cred).rcache = rcache;
    crate::krb5_h::krb5_free_context(context);
    *minor_status = 0u32;
    return 0u32;
}
/*
 * krb5 and IAKERB mech API functions follow.  The mechglue always passes null
 * desired_mechs and actual_mechs, so we ignore those parameters.
 */
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_acquire_cred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut desired_name: crate::gssapi_h::gss_name_t,
    mut time_req: crate::gssapi_h::OM_uint32,
    mut _desired_mechs: crate::gssapi_h::gss_OID_set,
    mut cred_usage: crate::gssapi_h::gss_cred_usage_t,
    mut output_cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    mut _actual_mechs: *mut crate::gssapi_h::gss_OID_set,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    return acquire_cred(
        minor_status,
        desired_name,
        0 as crate::gssapi_h::gss_buffer_t,
        time_req,
        cred_usage,
        0 as crate::krb5_h::krb5_ccache,
        0 as crate::krb5_h::krb5_keytab,
        0u32,
        output_cred_handle,
        time_rec,
    );
}
#[no_mangle]

pub unsafe extern "C" fn iakerb_gss_acquire_cred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut desired_name: crate::gssapi_h::gss_name_t,
    mut time_req: crate::gssapi_h::OM_uint32,
    mut _desired_mechs: crate::gssapi_h::gss_OID_set,
    mut cred_usage: crate::gssapi_h::gss_cred_usage_t,
    mut output_cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    mut _actual_mechs: *mut crate::gssapi_h::gss_OID_set,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    return acquire_cred(
        minor_status,
        desired_name,
        0 as crate::gssapi_h::gss_buffer_t,
        time_req,
        cred_usage,
        0 as crate::krb5_h::krb5_ccache,
        0 as crate::krb5_h::krb5_keytab,
        1u32,
        output_cred_handle,
        time_rec,
    );
}
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_acquire_cred_with_password(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    desired_name: crate::gssapi_h::gss_name_t,
    password: crate::gssapi_h::gss_buffer_t,
    mut time_req: crate::gssapi_h::OM_uint32,
    _desired_mechs: crate::gssapi_h::gss_OID_set,
    mut cred_usage: i32,
    mut output_cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    mut _actual_mechs: *mut crate::gssapi_h::gss_OID_set,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    return acquire_cred(
        minor_status,
        desired_name,
        password,
        time_req,
        cred_usage,
        0 as crate::krb5_h::krb5_ccache,
        0 as crate::krb5_h::krb5_keytab,
        0u32,
        output_cred_handle,
        time_rec,
    );
}
#[no_mangle]

pub unsafe extern "C" fn iakerb_gss_acquire_cred_with_password(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    desired_name: crate::gssapi_h::gss_name_t,
    password: crate::gssapi_h::gss_buffer_t,
    mut time_req: crate::gssapi_h::OM_uint32,
    _desired_mechs: crate::gssapi_h::gss_OID_set,
    mut cred_usage: i32,
    mut output_cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    mut _actual_mechs: *mut crate::gssapi_h::gss_OID_set,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    return acquire_cred(
        minor_status,
        desired_name,
        password,
        time_req,
        cred_usage,
        0 as crate::krb5_h::krb5_ccache,
        0 as crate::krb5_h::krb5_keytab,
        1u32,
        output_cred_handle,
        time_rec,
    );
}
#[no_mangle]

pub unsafe extern "C" fn gss_krb5int_import_cred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    _desired_oid: crate::gssapi_h::gss_OID,
    value: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut req = 0 as *mut crate::gssapiP_krb5_h::krb5_gss_import_cred_req;
    let mut name = crate::gssapiP_krb5_h::krb5_gss_name_rec {
        princ: 0 as *mut crate::krb5_h::krb5_principal_data,
        service: 0 as *mut i8,
        host: 0 as *mut i8,
        lock: crate::stdlib::pthread_mutex_t {
            __data: crate::stdlib::__pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: crate::stdlib::__pthread_list_t {
                    __prev: 0 as *mut crate::stdlib::__pthread_internal_list,
                    __next: 0 as *mut crate::stdlib::__pthread_internal_list,
                },
            },
        },
        ad_context: 0 as *mut crate::k5_int_h::_krb5_authdata_context,
    };
    let mut time_rec: crate::gssapi_h::OM_uint32 = 0;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut usage: crate::gssapi_h::gss_cred_usage_t = 0;
    let mut desired_name = 0 as crate::gssapi_h::gss_name_t;
    if (*value).length == ::std::mem::size_of::<crate::gssapiP_krb5_h::krb5_gss_import_cred_req>() {
    } else {
        crate::stdlib::__assert_fail(b"value->length == sizeof(*req)\x00" as *const u8 as
                          *const i8,
                      b"acquire_cred.c\x00" as *const u8 as
                          *const i8,
                      1128u32,
                      (*::std::mem::transmute::<&[u8; 99],
                                                &[i8; 99]>(b"OM_uint32 gss_krb5int_import_cred(OM_uint32 *, gss_cred_id_t *, const gss_OID, const gss_buffer_t)\x00")).as_ptr());
    }
    if (*value).length != ::std::mem::size_of::<crate::gssapiP_krb5_h::krb5_gss_import_cred_req>() {
        return (13u32) << 16i32;
    }
    req = (*value).value as *mut crate::gssapiP_krb5_h::krb5_gss_import_cred_req;
    if !(*req).id.is_null() {
        usage = if !(*req).keytab.is_null() { 0i32 } else { 1i32 }
    } else if !(*req).keytab.is_null() {
        usage = 2i32
    } else {
        *minor_status = 22u32;
        return (13u32) << 16i32;
    }
    if !(*req).keytab_principal.is_null() {
        crate::stdlib::memset(
            &mut name as *mut crate::gssapiP_krb5_h::krb5_gss_name_rec as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<crate::gssapiP_krb5_h::krb5_gss_name_rec>(),
        );
        code = k5_mutex_init(&mut name.lock);
        if code != 0i32 {
            *minor_status = code as crate::gssapi_h::OM_uint32;
            return (13u32) << 16i32;
        }
        name.princ = (*req).keytab_principal;
        desired_name = &mut name as *mut crate::gssapiP_krb5_h::krb5_gss_name_rec
            as crate::gssapi_h::gss_name_t
    }
    code = acquire_cred(
        minor_status,
        desired_name,
        0 as crate::gssapi_h::gss_buffer_t,
        0xffffffffu32,
        usage,
        (*req).id,
        (*req).keytab,
        0u32,
        cred_handle,
        &mut time_rec,
    ) as crate::krb5_h::krb5_error_code;
    if !(*req).keytab_principal.is_null() {
        crate::k5_thread_h::k5_os_mutex_destroy(&mut name.lock);
    }
    return code as crate::gssapi_h::OM_uint32;
}
/* * CFX flags **/
/* These are to be stored in little-endian order, i.e., des-mac is
stored as 02 00.  */
/* SGN_ALG_DES_MAC_MD5           = 0x0000, */
/* SGN_ALG_MD2_5                 = 0x0001, */
/* SGN_ALG_DES_MAC               = 0x0002, */
/* SGN_ALG_3                     = 0x0003, /\* not published *\/ */
/* microsoft w2k;  */
/* SEAL_ALG_DES             = 0x0000, */
/* SEAL_ALG_1               = 0x0001, /\* not published *\/ */
/* microsoft w2k;  */
/* for 3DES */
/* for draft-ietf-krb-wg-gssapi-cfx-01 */
/* GSS_KRB5_INTEG_C_QOP_MD5       = 0x0001, */
/* GSS_KRB5_INTEG_C_QOP_DES_MD5   = 0x0002, */
/* GSS_KRB5_INTEG_C_QOP_DES_MAC   = 0x0003, */
/* GSS_KRB5_CONF_C_QOP_DES        = 0x0100, */
/* * internal types **/
/* immutable */
/* immutable */
/* immutable */
/* protects ad_context only for now */
/* protect against simultaneous accesses */
/* name/type of credential */
/* keytab (accept) data */
/* ccache (init) data */
/* limit negotiated enctypes to this list */
/* nonzero if initiating, zero if accepting */
/* XXX tested but never actually set */
/* One of two potential keys to use with RFC 4121
 * packets; this key must always be set. */
/* RFC 1964 encryption key; seq xored with a constant
 * for DES, seq for other RFC 1964 enctypes  */
/* RFC 1964 sequencing key */
/* XXX these used to be signed.  the old spec is inspecific, and
the new spec specifies unsigned.  I don't believe that the change
affects the wire encoding. */
/* Protocol spec revision for sending packets
0 => RFC 1964 with 3DES and RC4 enhancements
1 => RFC 4121
No others defined so far.  It is always permitted to receive
tokens in RFC 4121 format.  If enc is non-null, receiving RFC
1964 tokens is permitted.*/
/* for "main" subkey */
/* CFX only */
/* did we get rcache from creds? */
/* LEAN_CLIENT */
/* * helper functions **/
/* Encrypt length bytes at ptr in place, with the given key and usage.  If
 * iv is not NULL, use it as the cipher state. */
/* AEAD */
/* for conf len */
/* * declarations of internal name mechanism functions **/
/* minor_status */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* minor_status */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* minor_status */
/* cred_handle */
/* minor_status */
/* claimant_cred_handle */
/* context_handle */
/* target_name */
/* mech_type */
/* req_flags */
/* time_req */
/* input_chan_bindings */
/* input_token */
/* actual_mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* minor_status */
/* claimant_cred_handle */
/* context_handle */
/* target_name */
/* mech_type */
/* req_flags */
/* time_req */
/* input_chan_bindings */
/* input_token */
/* actual_mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* exts */
/* minor_status */
/* context_handle */
/* verifier_cred_handle */
/* input_token_buffer */
/* input_chan_bindings */
/* src_name */
/* mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* delegated_cred_handle */
/* minor_status */
/* context_handle */
/* verifier_cred_handle */
/* input_token_buffer */
/* input_chan_bindings */
/* src_name */
/* mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* delegated_cred_handle */
/*exts */
/* LEAN_CLIENT */
/* minor_status */
/* context_handle */
/* desired_object */
/* data_set */
/* minor_status */
/* context_handle */
/* desired_object */
/* value */
/* minor_status */
/* context_handle */
/* token_buffer */
/* minor_status */
/* context_handle */
/* output_token */
/* minor_status */
/* context_handle */
/* time_rec */
/* minor_status */
/* status_value */
/* status_type */
/* mech_type */
/* message_context */
/* status_string */
/* minor_status */
/* mech_set */
/* minor_status */
/* name1 */
/* name2 */
/* name_equal */
/* minor_status */
/* input_name */
/* output_name_buffer */
/* output_name_type */
/* minor_status */
/* input_name_buffer */
/* input_name_type */
/* output_name */
/* minor_status */
/* input_name */
/* minor_status */
/* cred_handle */
/* name */
/* lifetime */
/* cred_usage */
/* mechanisms */
/* minor_status */
/* context_handle */
/* initiator_name */
/* acceptor_name */
/* lifetime_rec */
/* mech_type */
/* ret_flags */
/* locally_initiated */
/* open */
/* New V2 entry points */
/* minor_status */
/* context_handle */
/* qop_req */
/* message_buffer */
/* message_token */
/* minor_status */
/* context_handle */
/* qop_req */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* qop_req */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* message_buffer */
/* message_token */
/* qop_state */
/* minor_status */
/* context_handle */
/* qop_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* input_message_buffer */
/* conf_state */
/* output_message_buffer */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* conf_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* conf_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* input_message_buffer */
/* output_message_buffer */
/* conf_state */
/* qop_state */
/* minor_status */
/* context_handle */
/* conf_state */
/* qop_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* req_output_size */
/* max_input_size */
/* minor_status */
/* input_name */
/* input_name_type */
/* output_name */
/* minor_status */
/* input_name */
/* desired_name_type */
/* output_name */
/* minor_status */
/* cred_handle */
/* mech_type */
/* name */
/* initiator_lifetime */
/* acceptor_lifetime */
/* cred_usage */
/* minor_status */
/* context_handle */
/* interprocess_token */
/* minor_status */
/* interprocess_token */
/* context_handle */
/* LEAN_CLIENT */
/* minor_status */
/* oid */
/* minor_status */
/* oid */
/* minor_status */
/* mechanism */
/* name_types */
/* minor_status */
/* input_name */
/* mech_type */
/* output_name */
/* minor_status */
/* input_name */
/* exported_name */
/* minor_status */
/* input_name */
/* dest_name */
/* minor_status */
/* cred */
/* minor_status */
/* impersonator_cred_handle */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* minor_status */
/* cred_handle */
/* context */
/* naming_exts.c */
/* s4u_gss_glue.c */
/*
 * These take unglued krb5-mech-specific contexts.
 */
/* _GSS_STATIC_LINK */
/* For error message handling.  */
/* Returns a shared string, not a private copy!  */
/* Prefix concatenated with Kerberos encryption type */
/* IAKERB */
/*
 * Transfer contents of a krb5_data to a gss_buffer and invalidate the source
 * On unix, this is a simple pointer copy
 * On windows, memory is reallocated and copied.
 */
/* Credential store extensions */
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_acquire_cred_from(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    desired_name: crate::gssapi_h::gss_name_t,
    mut time_req: crate::gssapi_h::OM_uint32,
    _desired_mechs: crate::gssapi_h::gss_OID_set,
    mut cred_usage: crate::gssapi_h::gss_cred_usage_t,
    mut cred_store: crate::gssapi_ext_h::gss_const_key_value_set_t,
    mut output_cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    mut _actual_mechs: *mut crate::gssapi_h::gss_OID_set,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut context = 0 as crate::krb5_h::krb5_context;
    let mut code = 0i32;
    let mut client_keytab = 0 as crate::krb5_h::krb5_keytab;
    let mut keytab = 0 as crate::krb5_h::krb5_keytab;
    let mut ccache = 0 as crate::krb5_h::krb5_ccache;
    let mut rcname = 0 as *const i8;
    let mut value = 0 as *const i8;
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    code = crate::src::krb5::gssapi_krb5::gss_krb5int_initialize_library()
        as crate::krb5_h::krb5_error_code;
    if code != 0 {
        *minor_status = code as crate::gssapi_h::OM_uint32;
        ret = (13u32) << 16i32
    } else {
        code = crate::src::krb5::init_sec_context::krb5_gss_init_context(&mut context);
        if code != 0 {
            *minor_status = code as crate::gssapi_h::OM_uint32;
            ret = (13u32) << 16i32
        } else {
            ret = crate::src::krb5::cred_store::kg_value_from_cred_store(
                cred_store,
                b"ccache\x00" as *const u8 as *const i8,
                &mut value,
            );
            if !(ret & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0) {
                if !value.is_null() {
                    code = crate::krb5_h::krb5_cc_resolve(context, value, &mut ccache);
                    if code != 0i32 {
                        *minor_status = code as crate::gssapi_h::OM_uint32;
                        ret = (13u32) << 16i32;
                        current_block = 11412884123443887248;
                    } else {
                        current_block = 4808432441040389987;
                    }
                } else {
                    current_block = 4808432441040389987;
                }
                match current_block {
                    11412884123443887248 => {}
                    _ => {
                        ret = crate::src::krb5::cred_store::kg_value_from_cred_store(
                            cred_store,
                            b"client_keytab\x00" as *const u8 as *const i8,
                            &mut value,
                        );
                        if !(ret & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0) {
                            if !value.is_null() {
                                code = crate::krb5_h::krb5_kt_resolve(
                                    context,
                                    value,
                                    &mut client_keytab,
                                );
                                if code != 0i32 {
                                    *minor_status = code as crate::gssapi_h::OM_uint32;
                                    ret = (13u32) << 16i32;
                                    current_block = 11412884123443887248;
                                } else {
                                    current_block = 15345278821338558188;
                                }
                            } else {
                                current_block = 15345278821338558188;
                            }
                            match current_block {
                                11412884123443887248 => {}
                                _ => {
                                    ret = crate::src::krb5::cred_store::kg_value_from_cred_store(
                                        cred_store,
                                        b"keytab\x00" as *const u8 as *const i8,
                                        &mut value,
                                    );
                                    if !(ret & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0) {
                                        if !value.is_null() {
                                            code = crate::krb5_h::krb5_kt_resolve(
                                                context,
                                                value,
                                                &mut keytab,
                                            );
                                            if code != 0i32 {
                                                *minor_status = code as crate::gssapi_h::OM_uint32;
                                                ret = (13u32) << 16i32;
                                                current_block = 11412884123443887248;
                                            } else {
                                                current_block = 17184638872671510253;
                                            }
                                        } else {
                                            current_block = 17184638872671510253;
                                        }
                                        match current_block {
                                            11412884123443887248 => {}
                                            _ => {
                                                ret =
                                                    crate::src::krb5::cred_store::kg_value_from_cred_store(cred_store,
                                                                             b"rcache\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const i8,
                                                                             &mut rcname);
                                                if !(ret
                                                    & ((0o377u32) << 24i32 | (0o377u32) << 16i32)
                                                    != 0)
                                                {
                                                    ret = acquire_cred_context(
                                                        context,
                                                        minor_status,
                                                        desired_name,
                                                        0 as crate::gssapi_h::gss_buffer_t,
                                                        time_req,
                                                        cred_usage,
                                                        ccache,
                                                        client_keytab,
                                                        keytab,
                                                        rcname,
                                                        0u32,
                                                        output_cred_handle,
                                                        time_rec,
                                                    )
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
    if !ccache.is_null() {
        crate::krb5_h::krb5_cc_close(context, ccache);
    }
    if !client_keytab.is_null() {
        crate::krb5_h::krb5_kt_close(context, client_keytab);
    }
    if !keytab.is_null() {
        crate::krb5_h::krb5_kt_close(context, keytab);
    }
    crate::krb5_h::krb5_free_context(context);
    return ret;
}
