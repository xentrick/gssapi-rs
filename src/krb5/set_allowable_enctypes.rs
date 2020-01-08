use ::libc;

pub mod k5_thread_h {

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

    /* Allocate zeroed memory; set *code to 0 on success or ENOMEM on failure. */
    #[inline]

    pub unsafe extern "C" fn k5calloc(
        mut nmemb: crate::stddef_h::size_t,
        mut size: crate::stddef_h::size_t,
        mut code: *mut crate::krb5_h::krb5_error_code,
    ) -> *mut libc::c_void {
        let mut ptr = 0 as *mut libc::c_void;
        /* Allocate at least one byte since zero-byte allocs may return NULL. */
        ptr = crate::stdlib::calloc(
            if nmemb != 0 { nmemb } else { 1usize },
            if size != 0 { size } else { 1usize },
        );
        *code = if ptr.is_null() { 12i32 } else { 0i32 };
        return ptr;
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
pub use crate::k5_thread_h::k5_os_mutex_lock;
pub use crate::k5_thread_h::k5_os_mutex_unlock;
pub use crate::krb5_h::_krb5_address;
pub use crate::krb5_h::_krb5_ap_req;
pub use crate::krb5_h::_krb5_auth_context;
pub use crate::krb5_h::_krb5_authdata;
pub use crate::krb5_h::_krb5_ccache;
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
pub use crate::krb5_h::krb5_c_valid_enctype;
pub use crate::krb5_h::krb5_ccache;
pub use crate::krb5_h::krb5_const_principal;
pub use crate::krb5_h::krb5_context;
pub use crate::krb5_h::krb5_data;
pub use crate::krb5_h::krb5_deltat;
pub use crate::krb5_h::krb5_enc_data;
pub use crate::krb5_h::krb5_enc_tkt_part;
pub use crate::krb5_h::krb5_enctype;
pub use crate::krb5_h::krb5_error_code;
pub use crate::krb5_h::krb5_flags;
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
pub use crate::krb5_h::krb5_timestamp;
pub use crate::krb5_h::krb5_trace_callback;
pub use crate::krb5_h::krb5_trace_info;
pub use crate::krb5_h::krb5_transited;
pub use crate::profile_h::profile_t;
pub use crate::src::krb5::set_allowable_enctypes::k5_int_h::k5calloc;
pub use crate::src::krb5::set_allowable_enctypes::k5_thread_h::k5_mutex_lock;
pub use crate::src::krb5::set_allowable_enctypes::k5_thread_h::k5_mutex_unlock;

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
pub use crate::gssapiP_krb5_h::krb5_gss_set_allowable_enctypes_req;
pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_cred_id_t;
pub use crate::gssapi_h::gss_cred_usage_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::mglueP_h::gss_cred_id_struct;

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
/*
 * These take unglued krb5-mech-specific contexts.
 */
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/gssapi/krb5/set_allowable_enctypes.c */
/*
 * Copyright 2004  by the Massachusetts Institute of Technology.
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
 * krb5_gss_set_allowable_enctypes()
 */
/*
 * gss_krb5_set_allowable_enctypes
 *
 * This function may be called by a context initiator after calling
 * gss_acquire_cred(), but before calling gss_init_sec_context(),
 * to restrict the set of enctypes which will be negotiated during
 * context establishment to those in the provided array.
 *
 * 'cred_handle' must be a valid credential handle obtained via
 * gss_acquire_cred().  It may not be GSS_C_NO_CREDENTIAL.
 * gss_acquire_cred() may be called with GSS_C_NO_CREDENTIAL
 * to get a handle to the default credential.
 *
 * The purpose of this function is to limit the keys that may
 * be exported via gss_krb5_export_lucid_sec_context(); thus it
 * should limit the enctypes of all keys that will be needed
 * after the security context has been established.
 * (i.e. context establishment may use a session key with a
 * stronger enctype than in the provided array, however a
 * subkey must be established within the enctype limits
 * established by this function.)
 *
 */
#[no_mangle]

pub unsafe extern "C" fn gss_krb5int_set_allowable_enctypes(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    _desired_oid: crate::gssapi_h::gss_OID,
    value: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    let mut new_ktypes = 0 as *mut crate::krb5_h::krb5_enctype;
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut cred = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_cred_id_rec;
    let mut kerr = 0i32;
    let mut req = 0 as *mut crate::gssapiP_krb5_h::krb5_gss_set_allowable_enctypes_req;
    /* Assume a failure */
    *minor_status = 0u32;
    major_status = (13u32) << 16i32;
    if (*value).length
        == ::std::mem::size_of::<crate::gssapiP_krb5_h::krb5_gss_set_allowable_enctypes_req>()
    {
    } else {
        crate::stdlib::__assert_fail(b"value->length == sizeof(*req)\x00" as *const u8 as
                          *const i8,
                      b"set_allowable_enctypes.c\x00" as *const u8 as
                          *const i8,
                      80u32,
                      (*::std::mem::transmute::<&[u8; 110],
                                                &[i8; 110]>(b"OM_uint32 gss_krb5int_set_allowable_enctypes(OM_uint32 *, gss_cred_id_t *, const gss_OID, const gss_buffer_t)\x00")).as_ptr());
    }
    req = (*value).value as *mut crate::gssapiP_krb5_h::krb5_gss_set_allowable_enctypes_req;
    /* verify and valildate cred handle */
    cred = *cred_handle as crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
    if (*req).ktypes.is_null() {
        k5_mutex_lock(&mut (*cred).lock);
        if !(*cred).req_enctypes.is_null() {
            crate::stdlib::free((*cred).req_enctypes as *mut libc::c_void);
        }
        (*cred).req_enctypes = 0 as *mut crate::krb5_h::krb5_enctype;
        k5_mutex_unlock(&mut (*cred).lock);
        return 0u32;
    }
    /* Copy the requested enctypes into the cred structure.  Filter out the
     * ones we don't consider valid.  Error out if no enctypes are valid. */
    new_ktypes = k5calloc(
        (*req).num_ktypes.wrapping_add(1u32) as crate::stddef_h::size_t,
        ::std::mem::size_of::<crate::krb5_h::krb5_enctype>(),
        &mut kerr,
    ) as *mut crate::krb5_h::krb5_enctype;
    if !new_ktypes.is_null() {
        i = 0u32;
        j = 0u32;
        while i < (*req).num_ktypes && *(*req).ktypes.offset(i as isize) != 0 {
            if crate::krb5_h::krb5_c_valid_enctype(*(*req).ktypes.offset(i as isize)) != 0 {
                let fresh0 = j;
                j = j.wrapping_add(1);
                *new_ktypes.offset(fresh0 as isize) = *(*req).ktypes.offset(i as isize)
            }
            i = i.wrapping_add(1)
        }
        *new_ktypes.offset(j as isize) = 0i32;
        if j == 0u32 {
            crate::stdlib::free(new_ktypes as *mut libc::c_void);
            kerr = -(1765328234 as isize) as crate::krb5_h::krb5_error_code
        } else {
            k5_mutex_lock(&mut (*cred).lock);
            if !(*cred).req_enctypes.is_null() {
                crate::stdlib::free((*cred).req_enctypes as *mut libc::c_void);
            }
            (*cred).req_enctypes = new_ktypes;
            k5_mutex_unlock(&mut (*cred).lock);
            /* Success! */
            return 0u32;
        }
    }
    *minor_status = kerr as crate::gssapi_h::OM_uint32;
    return major_status;
}
