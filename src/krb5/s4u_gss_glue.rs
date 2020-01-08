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

    pub unsafe extern "C" fn ts_delta(
        mut a: crate::krb5_h::krb5_timestamp,
        mut b: crate::krb5_h::krb5_timestamp,
    ) -> crate::krb5_h::krb5_deltat {
        return (a as crate::stdlib::uint32_t).wrapping_sub(b as crate::stdlib::uint32_t)
            as crate::krb5_h::krb5_deltat;
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

    pub unsafe extern "C" fn string2data(mut str: *mut i8) -> crate::krb5_h::krb5_data {
        return make_data(str as *mut libc::c_void, crate::stdlib::strlen(str) as u32);
    }

    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}

pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__pthread_internal_list;
pub use crate::stdlib::__pthread_list_t;
pub use crate::stdlib::__pthread_mutex_s;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::pthread_mutex_t;

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
pub use crate::k5_int_h::k5_tls_vtable_st;
pub use crate::k5_int_h::kdb5_dal_handle;
pub use crate::k5_int_h::krb5_authdata_context;
pub use crate::k5_int_h::krb5_authdata_export_authdata;
pub use crate::k5_int_h::krb5_get_credentials_for_user;
pub use crate::k5_int_h::krb5_preauth_context;
pub use crate::k5_int_h::krb5_preauth_context_st;
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
pub use crate::krb5_h::_krb5_keyblock;
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
pub use crate::krb5_h::krb5_cc_copy_creds;
pub use crate::krb5_h::krb5_cc_destroy;
pub use crate::krb5_h::krb5_cc_initialize;
pub use crate::krb5_h::krb5_cc_new_unique;
pub use crate::krb5_h::krb5_cc_set_config;
pub use crate::krb5_h::krb5_cc_store_cred;
pub use crate::krb5_h::krb5_ccache;
pub use crate::krb5_h::krb5_const_principal;
pub use crate::krb5_h::krb5_context;
pub use crate::krb5_h::krb5_copy_principal;
pub use crate::krb5_h::krb5_creds;
pub use crate::krb5_h::krb5_data;
pub use crate::krb5_h::krb5_deltat;
pub use crate::krb5_h::krb5_enc_data;
pub use crate::krb5_h::krb5_enc_tkt_part;
pub use crate::krb5_h::krb5_enctype;
pub use crate::krb5_h::krb5_error_code;
pub use crate::krb5_h::krb5_flags;
pub use crate::krb5_h::krb5_free_authdata;
pub use crate::krb5_h::krb5_free_context;
pub use crate::krb5_h::krb5_free_creds;
pub use crate::krb5_h::krb5_free_unparsed_name;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_keyblock;
pub use crate::krb5_h::krb5_keytab;
pub use crate::krb5_h::krb5_keytab_entry;
pub use crate::krb5_h::krb5_keytab_entry_st;
pub use crate::krb5_h::krb5_kt_cursor;
pub use crate::krb5_h::krb5_kvno;
pub use crate::krb5_h::krb5_magic;
pub use crate::krb5_h::krb5_octet;
pub use crate::krb5_h::krb5_pointer;
pub use crate::krb5_h::krb5_post_recv_fn;
pub use crate::krb5_h::krb5_pre_send_fn;
pub use crate::krb5_h::krb5_principal;
pub use crate::krb5_h::krb5_principal_data;
pub use crate::krb5_h::krb5_prompt_type;
pub use crate::krb5_h::krb5_rc_st;
pub use crate::krb5_h::krb5_rcache;
pub use crate::krb5_h::krb5_ticket;
pub use crate::krb5_h::krb5_ticket_times;
pub use crate::krb5_h::krb5_timeofday;
pub use crate::krb5_h::krb5_timestamp;
pub use crate::krb5_h::krb5_trace_callback;
pub use crate::krb5_h::krb5_trace_info;
pub use crate::krb5_h::krb5_transited;
pub use crate::krb5_h::krb5_unparse_name;
pub use crate::profile_h::profile_t;
pub use crate::src::krb5::s4u_gss_glue::k5_int_h::make_data;
pub use crate::src::krb5::s4u_gss_glue::k5_int_h::string2data;
pub use crate::src::krb5::s4u_gss_glue::k5_int_h::ts_delta;
pub use crate::src::krb5::s4u_gss_glue::k5_thread_h::k5_mutex_init;
pub use crate::src::krb5::s4u_gss_glue::k5_thread_h::k5_mutex_lock;
pub use crate::src::krb5::s4u_gss_glue::k5_thread_h::k5_mutex_unlock;

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
pub use crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
pub use crate::gssapiP_krb5_h::krb5_gss_name_t;
pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_OID_set;
pub use crate::gssapi_h::gss_OID_set_desc_struct;
pub use crate::gssapi_h::gss_cred_id_t;
pub use crate::gssapi_h::gss_cred_usage_t;
pub use crate::gssapi_h::gss_name_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::mglueP_h::gss_cred_id_struct;
pub use crate::mglueP_h::gss_name_struct;
pub use crate::src::krb5::acquire_cred::kg_cred_resolve;
pub use crate::src::krb5::init_sec_context::krb5_gss_init_context;
pub use crate::src::krb5::naming_exts::kg_init_name;
pub use crate::src::krb5::naming_exts::kg_release_name;

/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2009  by the Massachusetts Institute of Technology.
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

unsafe extern "C" fn kg_is_initiator_cred(
    mut cred: crate::gssapiP_krb5_h::krb5_gss_cred_id_t,
) -> i32 {
    return (((*cred).usage == 1i32 || (*cred).usage == 0i32) && !(*cred).ccache.is_null()) as i32;
}

unsafe extern "C" fn kg_impersonate_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    impersonator_cred: crate::gssapiP_krb5_h::krb5_gss_cred_id_t,
    user: crate::gssapiP_krb5_h::krb5_gss_name_t,
    mut time_req: crate::gssapi_h::OM_uint32,
    mut output_cred: *mut crate::gssapiP_krb5_h::krb5_gss_cred_id_t,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
    mut context: crate::krb5_h::krb5_context,
) -> crate::gssapi_h::OM_uint32 {
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut in_creds = crate::krb5_h::krb5_creds {
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
    let mut out_creds = 0 as *mut crate::krb5_h::krb5_creds;
    *output_cred = 0 as crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
    crate::stdlib::memset(
        &mut in_creds as *mut crate::krb5_h::krb5_creds as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::krb5_h::krb5_creds>(),
    );
    in_creds.client = (*user).princ;
    in_creds.server = (*(*impersonator_cred).name).princ;
    if !(*impersonator_cred).req_enctypes.is_null() {
        in_creds.keyblock.enctype = *(*impersonator_cred).req_enctypes.offset(0isize)
    }
    k5_mutex_lock(&mut (*user).lock);
    if !(*user).ad_context.is_null() {
        code = crate::k5_int_h::krb5_authdata_export_authdata(
            context,
            (*user).ad_context,
            0x2i32,
            &mut in_creds.authdata,
        );
        if code != 0i32 {
            k5_mutex_unlock(&mut (*user).lock);
            *minor_status = code as crate::gssapi_h::OM_uint32;
            return (13u32) << 16i32;
        }
    }
    k5_mutex_unlock(&mut (*user).lock);
    code = crate::k5_int_h::krb5_get_credentials_for_user(
        context,
        4i32 | 8i32,
        (*impersonator_cred).ccache,
        &mut in_creds,
        0 as *mut crate::krb5_h::krb5_data,
        &mut out_creds,
    );
    if code != 0i32 {
        crate::krb5_h::krb5_free_authdata(context, in_creds.authdata);
        *minor_status = code as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    major_status = kg_compose_deleg_cred(
        minor_status,
        impersonator_cred,
        out_creds,
        time_req,
        output_cred,
        time_rec,
        context,
    );
    crate::krb5_h::krb5_free_authdata(context, in_creds.authdata);
    crate::krb5_h::krb5_free_creds(context, out_creds);
    return major_status;
}
/* The mechglue always passes null desired_mechs and actual_mechs. */
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_acquire_cred_impersonate_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    impersonator_cred_handle: crate::gssapi_h::gss_cred_id_t,
    desired_name: crate::gssapi_h::gss_name_t,
    mut time_req: crate::gssapi_h::OM_uint32,
    _desired_mechs: crate::gssapi_h::gss_OID_set,
    mut cred_usage: crate::gssapi_h::gss_cred_usage_t,
    mut output_cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    mut _actual_mechs: *mut crate::gssapi_h::gss_OID_set,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut imp_cred = impersonator_cred_handle as crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
    let mut cred = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_cred_id_rec;
    let mut context = 0 as *mut crate::k5_int_h::_krb5_context;
    if impersonator_cred_handle.is_null() {
        return (1u32) << 24i32;
    }
    if desired_name.is_null() {
        return (1u32) << 24i32;
    }
    if output_cred_handle.is_null() {
        return (2u32) << 24i32;
    }
    if cred_usage != 1i32 {
        *minor_status = -(2045022969 as isize) as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    if (*imp_cred).usage != 1i32 && (*imp_cred).usage != 0i32 {
        *minor_status = 0u32;
        return (7u32) << 16i32;
    }
    *output_cred_handle = 0 as crate::gssapi_h::gss_cred_id_t;
    if !time_rec.is_null() {
        *time_rec = 0u32
    }
    code = crate::src::krb5::init_sec_context::krb5_gss_init_context(&mut context);
    if code != 0i32 {
        *minor_status = code as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    major_status = crate::src::krb5::acquire_cred::kg_cred_resolve(
        minor_status,
        context,
        impersonator_cred_handle,
        0 as crate::gssapi_h::gss_name_t,
    );
    if major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
        crate::krb5_h::krb5_free_context(context);
        return major_status;
    }
    major_status = kg_impersonate_name(
        minor_status,
        imp_cred,
        desired_name as crate::gssapiP_krb5_h::krb5_gss_name_t,
        time_req,
        &mut cred,
        time_rec,
        context,
    );
    if major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) == 0 {
        *output_cred_handle = cred as crate::gssapi_h::gss_cred_id_t
    }
    k5_mutex_unlock(&mut (*imp_cred).lock);
    crate::krb5_h::krb5_free_context(context);
    return major_status;
}
/*
 * Set up cred to be an S4U2Proxy credential by copying in the impersonator's
 * creds, setting a cache config variable with the impersonator principal name,
 * and saving the impersonator principal name in the cred structure.
 */

unsafe extern "C" fn make_proxy_cred(
    mut context: crate::krb5_h::krb5_context,
    mut cred: crate::gssapiP_krb5_h::krb5_gss_cred_id_t,
    mut impersonator_cred: crate::gssapiP_krb5_h::krb5_gss_cred_id_t,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut data = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    let mut str = 0 as *mut i8;
    code = crate::krb5_h::krb5_cc_copy_creds(context, (*impersonator_cred).ccache, (*cred).ccache);
    if code != 0 {
        return code;
    }
    code = crate::krb5_h::krb5_unparse_name(
        context,
        (*(*impersonator_cred).name).princ as crate::krb5_h::krb5_const_principal,
        &mut str,
    );
    if code != 0 {
        return code;
    }
    data = string2data(str);
    code = crate::krb5_h::krb5_cc_set_config(
        context,
        (*cred).ccache,
        0 as crate::krb5_h::krb5_const_principal,
        b"proxy_impersonator\x00" as *const u8 as *const i8,
        &mut data,
    );
    crate::krb5_h::krb5_free_unparsed_name(context, str);
    if code != 0 {
        return code;
    }
    return crate::krb5_h::krb5_copy_principal(
        context,
        (*(*impersonator_cred).name).princ as crate::krb5_h::krb5_const_principal,
        &mut (*cred).impersonator,
    );
}
/* -*- mode: c; indent-tabs-mode: nil -*- */
/*
 * Copyright 2000, 2008 by the Massachusetts Institute of Technology.
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
 *
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
/* work around sunos braindamage */
/* The include of gssapi_krb5.h will dtrt with the above #defines in
 * effect.
 */
/* for debugging */
/* * constants **/
/* Incorrect krb5 mech OID emitted by MS. */
/* IAKERB variant */
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
#[no_mangle]

pub unsafe extern "C" fn kg_compose_deleg_cred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut impersonator_cred: crate::gssapiP_krb5_h::krb5_gss_cred_id_t,
    mut subject_creds: *mut crate::krb5_h::krb5_creds,
    mut _time_req: crate::gssapi_h::OM_uint32,
    mut output_cred: *mut crate::gssapiP_krb5_h::krb5_gss_cred_id_t,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
    mut context: crate::krb5_h::krb5_context,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut cred = 0 as crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
    *output_cred = 0 as crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
    if kg_is_initiator_cred(impersonator_cred) == 0
        || (*impersonator_cred).name.is_null()
        || !(*impersonator_cred).impersonator.is_null()
    {
        code = -(2045022969 as isize) as crate::krb5_h::krb5_error_code
    } else {
        if !(*(*impersonator_cred).name).princ.is_null() {
        } else {
            crate::stdlib::__assert_fail(b"impersonator_cred->name->princ != NULL\x00" as
                              *const u8 as *const i8,
                          b"s4u_gss_glue.c\x00" as *const u8 as
                              *const i8,
                          230u32,
                          (*::std::mem::transmute::<&[u8; 139],
                                                    &[i8; 139]>(b"OM_uint32 kg_compose_deleg_cred(OM_uint32 *, krb5_gss_cred_id_t, krb5_creds *, OM_uint32, krb5_gss_cred_id_t *, OM_uint32 *, krb5_context)\x00")).as_ptr());
        }
        if !subject_creds.is_null() {
        } else {
            crate::stdlib::__assert_fail(b"subject_creds != NULL\x00" as *const u8 as
                              *const i8,
                          b"s4u_gss_glue.c\x00" as *const u8 as
                              *const i8,
                          232u32,
                          (*::std::mem::transmute::<&[u8; 139],
                                                    &[i8; 139]>(b"OM_uint32 kg_compose_deleg_cred(OM_uint32 *, krb5_gss_cred_id_t, krb5_creds *, OM_uint32, krb5_gss_cred_id_t *, OM_uint32 *, krb5_context)\x00")).as_ptr());
        }
        if !(*subject_creds).client.is_null() {
        } else {
            crate::stdlib::__assert_fail(b"subject_creds->client != NULL\x00" as *const u8 as
                              *const i8,
                          b"s4u_gss_glue.c\x00" as *const u8 as
                              *const i8,
                          233u32,
                          (*::std::mem::transmute::<&[u8; 139],
                                                    &[i8; 139]>(b"OM_uint32 kg_compose_deleg_cred(OM_uint32 *, krb5_gss_cred_id_t, krb5_creds *, OM_uint32, krb5_gss_cred_id_t *, OM_uint32 *, krb5_context)\x00")).as_ptr());
        }
        cred = crate::stdlib::malloc(::std::mem::size_of::<
            crate::gssapiP_krb5_h::_krb5_gss_cred_id_rec,
        >()) as crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
        if cred.is_null() {
            code = 12i32
        } else {
            crate::stdlib::memset(
                cred as *mut libc::c_void,
                0i32,
                ::std::mem::size_of::<crate::gssapiP_krb5_h::_krb5_gss_cred_id_rec>(),
            );
            code = k5_mutex_init(&mut (*cred).lock);
            if !(code != 0i32) {
                (*cred).usage = 1i32;
                (*cred).expire = (*subject_creds).times.endtime;
                code = crate::src::krb5::naming_exts::kg_init_name(
                    context,
                    (*subject_creds).client,
                    0 as *mut i8,
                    0 as *mut i8,
                    0 as crate::k5_int_h::krb5_authdata_context,
                    0i32,
                    &mut (*cred).name,
                );
                if !(code != 0i32) {
                    code = crate::krb5_h::krb5_cc_new_unique(
                        context,
                        b"MEMORY\x00" as *const u8 as *const i8,
                        0 as *const i8,
                        &mut (*cred).ccache,
                    );
                    if !(code != 0i32) {
                        (*cred).set_destroy_ccache(1u32);
                        code = crate::krb5_h::krb5_cc_initialize(
                            context,
                            (*cred).ccache,
                            (*subject_creds).client,
                        );
                        if !(code != 0i32) {
                            code = make_proxy_cred(context, cred, impersonator_cred);
                            if !(code != 0i32) {
                                code = crate::krb5_h::krb5_cc_store_cred(
                                    context,
                                    (*cred).ccache,
                                    subject_creds,
                                );
                                if !(code != 0i32) {
                                    if !time_rec.is_null() {
                                        let mut now: crate::krb5_h::krb5_timestamp = 0;
                                        code = crate::krb5_h::krb5_timeofday(context, &mut now);
                                        if code != 0i32 {
                                            current_block = 18186649610560263798;
                                        } else {
                                            *time_rec = ts_delta((*cred).expire, now)
                                                as crate::gssapi_h::OM_uint32;
                                            current_block = 5494826135382683477;
                                        }
                                    } else {
                                        current_block = 5494826135382683477;
                                    }
                                    match current_block {
                                        18186649610560263798 => {}
                                        _ => {
                                            major_status = 0u32;
                                            *minor_status = 0u32;
                                            *output_cred = cred
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
    if code != 0i32 {
        *minor_status = code as crate::gssapi_h::OM_uint32;
        major_status = (13u32) << 16i32
    }
    if major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 && !cred.is_null() {
        crate::k5_thread_h::k5_os_mutex_destroy(&mut (*cred).lock);
        crate::krb5_h::krb5_cc_destroy(context, (*cred).ccache);
        crate::src::krb5::naming_exts::kg_release_name(context, &mut (*cred).name);
        crate::stdlib::free(cred as *mut libc::c_void);
    }
    return major_status;
}
