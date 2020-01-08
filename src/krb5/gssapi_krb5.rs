use ::libc;

pub mod k5_thread_h {

    #[inline]

    pub unsafe extern "C" fn k5_mutex_finish_init(
        mut _m: *mut crate::k5_thread_h::k5_mutex_t,
    ) -> i32 {
        return 0i32;
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
    /* Allocate zeroed memory; set *code to 0 on success or ENOMEM on failure. */
    #[inline]

    pub unsafe extern "C" fn k5alloc(
        mut size: crate::stddef_h::size_t,
        mut code: *mut crate::krb5_h::krb5_error_code,
    ) -> *mut libc::c_void {
        return k5calloc(1usize, size, code);
    }
    /* Like k5memdup, but add a final null byte. */
    #[inline]

    pub unsafe extern "C" fn k5memdup0(
        mut in_0: *const libc::c_void,
        mut len: crate::stddef_h::size_t,
        mut code: *mut crate::krb5_h::krb5_error_code,
    ) -> *mut libc::c_void {
        let mut ptr = k5alloc(len.wrapping_add(1usize), code);
        if !ptr.is_null() && len > 0usize {
            crate::stdlib::memcpy(ptr, in_0, len);
        }
        return ptr;
    }

    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}

pub mod gssapi_alloc_h {
    #[inline]

    pub unsafe extern "C" fn gssalloc_malloc(
        mut size: crate::stddef_h::size_t,
    ) -> *mut libc::c_void {
        return crate::stdlib::malloc(size);
    }
    /* not _WIN32 or DEBUG_GSSALLOC */
    #[inline]

    pub unsafe extern "C" fn gssalloc_strdup(mut str: *const i8) -> *mut i8 {
        let mut size = crate::stdlib::strlen(str).wrapping_add(1usize);
        let mut copy = gssalloc_malloc(size) as *mut i8;
        if !copy.is_null() {
            crate::stdlib::memcpy(copy as *mut libc::c_void, str as *const libc::c_void, size);
            *copy.offset(size.wrapping_sub(1usize) as isize) = '\u{0}' as i8
        }
        return copy;
    }
}

/*@modifies internalState@*/
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__pthread_internal_list;
pub use crate::stdlib::__pthread_list_t;
pub use crate::stdlib::__pthread_mutex_s;
pub use crate::stdlib::__ssize_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint64_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::pthread_mutex_t;
pub use crate::stdlib::ssize_t;

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
pub use crate::k5_int_h::derived_key;
pub use crate::k5_int_h::dns_canonhost;
pub use crate::k5_int_h::hostrealm_module_handle;
pub use crate::k5_int_h::k5_tls_vtable_st;
pub use crate::k5_int_h::kdb5_dal_handle;
pub use crate::k5_int_h::krb5_authdata_context;
pub use crate::k5_int_h::krb5_key_st;
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
pub use crate::k5_thread_h::k5_key_t;
pub use crate::k5_thread_h::k5_mutex_t;
pub use crate::k5_thread_h::k5_os_mutex;
pub use crate::k5_thread_h::k5_os_mutex_destroy;
pub use crate::k5_thread_h::krb5int_getspecific;
pub use crate::k5_thread_h::krb5int_key_delete;
pub use crate::k5_thread_h::krb5int_key_register;
pub use crate::k5_thread_h::krb5int_setspecific;
pub use crate::k5_thread_h::K5_KEY_COM_ERR;
pub use crate::k5_thread_h::K5_KEY_GSS_KRB5_CCACHE_NAME;
pub use crate::k5_thread_h::K5_KEY_GSS_KRB5_ERROR_MESSAGE;
pub use crate::k5_thread_h::K5_KEY_GSS_KRB5_SET_CCACHE_OLD_NAME;
pub use crate::k5_thread_h::K5_KEY_GSS_SPNEGO_STATUS;
pub use crate::k5_thread_h::K5_KEY_MAX;
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
pub use crate::krb5_h::krb5_aname_to_localname;
pub use crate::krb5_h::krb5_ap_req;
pub use crate::krb5_h::krb5_auth_context;
pub use crate::krb5_h::krb5_authdata;
pub use crate::krb5_h::krb5_authdatatype;
pub use crate::krb5_h::krb5_boolean;
pub use crate::krb5_h::krb5_cc_default_name;
pub use crate::krb5_h::krb5_cc_set_default_name;
pub use crate::krb5_h::krb5_ccache;
pub use crate::krb5_h::krb5_cksumtype;
pub use crate::krb5_h::krb5_const_principal;
pub use crate::krb5_h::krb5_context;
pub use crate::krb5_h::krb5_data;
pub use crate::krb5_h::krb5_deltat;
pub use crate::krb5_h::krb5_enc_data;
pub use crate::krb5_h::krb5_enc_tkt_part;
pub use crate::krb5_h::krb5_enctype;
pub use crate::krb5_h::krb5_error_code;
pub use crate::krb5_h::krb5_flags;
pub use crate::krb5_h::krb5_free_context;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_key;
pub use crate::krb5_h::krb5_keyblock;
pub use crate::krb5_h::krb5_keytab;
pub use crate::krb5_h::krb5_keytab_entry;
pub use crate::krb5_h::krb5_keytab_entry_st;
pub use crate::krb5_h::krb5_kt_cursor;
pub use crate::krb5_h::krb5_kuserok;
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
pub use crate::src::krb5::gssapi_krb5::k5_int_h::k5alloc;
pub use crate::src::krb5::gssapi_krb5::k5_int_h::k5calloc;
pub use crate::src::krb5::gssapi_krb5::k5_int_h::k5memdup0;
pub use crate::src::krb5::gssapi_krb5::k5_thread_h::k5_mutex_finish_init;
pub use crate::stdlib::uint32_t;
pub use crate::stdlib::uint64_t;
pub use crate::stdlib::uint8_t;

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
pub use crate::com_err_h::add_error_table;
pub use crate::com_err_h::errcode_t;
pub use crate::com_err_h::error_table;
pub use crate::com_err_h::remove_error_table;
pub use crate::gssapiP_generic_h::g_seqnum_state;
pub use crate::gssapiP_generic_h::g_seqnum_state_st;
pub use crate::gssapiP_generic_h::g_set;
pub use crate::gssapiP_krb5_h::_krb5_gss_cred_id_rec;
pub use crate::gssapiP_krb5_h::_krb5_gss_ctx_id_rec;
pub use crate::gssapiP_krb5_h::_krb5_gss_name_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
pub use crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_name_t;
pub use crate::gssapi_ext_h::gss_any;
pub use crate::gssapi_ext_h::gss_any_t;
pub use crate::gssapi_ext_h::gss_buffer_set_desc_struct;
pub use crate::gssapi_ext_h::gss_buffer_set_t;
pub use crate::gssapi_ext_h::gss_const_key_value_set_t;
pub use crate::gssapi_ext_h::gss_iov_buffer_desc;
pub use crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
pub use crate::gssapi_ext_h::gss_key_value_element_desc;
pub use crate::gssapi_ext_h::gss_key_value_element_struct;
pub use crate::gssapi_ext_h::gss_key_value_set_desc;
pub use crate::gssapi_ext_h::gss_key_value_set_struct;
pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_OID_set;
pub use crate::gssapi_h::gss_OID_set_desc;
pub use crate::gssapi_h::gss_OID_set_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_channel_bindings_struct;
pub use crate::gssapi_h::gss_channel_bindings_t;
pub use crate::gssapi_h::gss_const_OID;
pub use crate::gssapi_h::gss_const_buffer_t;
pub use crate::gssapi_h::gss_cred_id_t;
pub use crate::gssapi_h::gss_cred_usage_t;
pub use crate::gssapi_h::gss_ctx_id_struct;
pub use crate::gssapi_h::gss_ctx_id_t;
pub use crate::gssapi_h::gss_name_t;
pub use crate::gssapi_h::gss_qop_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::mglueP_h::gss_config;
pub use crate::mglueP_h::gss_cred_id_struct;
pub use crate::mglueP_h::gss_mech_config;
pub use crate::mglueP_h::gss_mech_info;
pub use crate::mglueP_h::gss_mechanism;
pub use crate::mglueP_h::gss_name_struct;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_AUTH_INIT;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_AUTH_INIT_INIT;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_AUTH_TARG;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_CBINDINGS;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_CONF_PROT;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_CTX_TRANS;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_DELEG_CRED;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_DEPRECATED;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_INTEG_PROT;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_ITOK_FRAMED;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_MECH_CONCRETE;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_MIC;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_NOT_DFLT_MECH;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_OOS_DET;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_PROT_READY;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_REPLAY_DET;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_WRAP;
pub use crate::src::generic::gssapi_generic::GSS_C_NT_USER_NAME;
pub use crate::src::generic::util_buffer::gssint_g_make_string_buffer;
pub use crate::src::krb5::accept_sec_context::krb5_gss_accept_sec_context;
pub use crate::src::krb5::acquire_cred::gss_krb5int_import_cred;
pub use crate::src::krb5::acquire_cred::gss_krb5int_register_acceptor_identity;
pub use crate::src::krb5::acquire_cred::gss_krb5int_set_cred_rcache;
pub use crate::src::krb5::acquire_cred::gssint_krb5_keytab_lock;
pub use crate::src::krb5::acquire_cred::iakerb_gss_acquire_cred;
pub use crate::src::krb5::acquire_cred::iakerb_gss_acquire_cred_with_password;
pub use crate::src::krb5::acquire_cred::krb5_gss_acquire_cred;
pub use crate::src::krb5::acquire_cred::krb5_gss_acquire_cred_from;
pub use crate::src::krb5::acquire_cred::krb5_gss_acquire_cred_with_password;
pub use crate::src::krb5::compare_name::krb5_gss_compare_name;
pub use crate::src::krb5::context_time::krb5_gss_context_time;
pub use crate::src::krb5::copy_ccache::gss_krb5int_copy_ccache;
pub use crate::src::krb5::delete_sec_context::krb5_gss_delete_sec_context;
pub use crate::src::krb5::disp_name::krb5_gss_display_name;
pub use crate::src::krb5::disp_status::krb5_gss_delete_error_info;
pub use crate::src::krb5::disp_status::krb5_gss_display_status;
pub use crate::src::krb5::disp_status::krb5_gss_save_error_info;
pub use crate::src::krb5::duplicate_name::krb5_gss_duplicate_name;
pub use crate::src::krb5::export_cred::krb5_gss_export_cred;
pub use crate::src::krb5::export_name::krb5_gss_export_name;
pub use crate::src::krb5::export_sec_context::krb5_gss_export_sec_context;
pub use crate::src::krb5::get_tkt_flags::gss_krb5int_get_tkt_flags;

pub use crate::src::krb5::gssapi_krb5::gssapi_alloc_h::gssalloc_malloc;
pub use crate::src::krb5::gssapi_krb5::gssapi_alloc_h::gssalloc_strdup;
pub use crate::src::krb5::iakerb::iakerb_gss_accept_sec_context;
pub use crate::src::krb5::iakerb::iakerb_gss_context_time;
pub use crate::src::krb5::iakerb::iakerb_gss_delete_sec_context;
pub use crate::src::krb5::iakerb::iakerb_gss_export_sec_context;
pub use crate::src::krb5::iakerb::iakerb_gss_get_mic;
pub use crate::src::krb5::iakerb::iakerb_gss_get_mic_iov;
pub use crate::src::krb5::iakerb::iakerb_gss_get_mic_iov_length;
pub use crate::src::krb5::iakerb::iakerb_gss_import_sec_context;
pub use crate::src::krb5::iakerb::iakerb_gss_init_sec_context;
pub use crate::src::krb5::iakerb::iakerb_gss_inquire_context;
pub use crate::src::krb5::iakerb::iakerb_gss_inquire_sec_context_by_oid;
pub use crate::src::krb5::iakerb::iakerb_gss_process_context_token;
pub use crate::src::krb5::iakerb::iakerb_gss_pseudo_random;
pub use crate::src::krb5::iakerb::iakerb_gss_set_sec_context_option;
pub use crate::src::krb5::iakerb::iakerb_gss_unwrap;
pub use crate::src::krb5::iakerb::iakerb_gss_unwrap_iov;
pub use crate::src::krb5::iakerb::iakerb_gss_verify_mic;
pub use crate::src::krb5::iakerb::iakerb_gss_verify_mic_iov;
pub use crate::src::krb5::iakerb::iakerb_gss_wrap;
pub use crate::src::krb5::iakerb::iakerb_gss_wrap_iov;
pub use crate::src::krb5::iakerb::iakerb_gss_wrap_iov_length;
pub use crate::src::krb5::iakerb::iakerb_gss_wrap_size_limit;
pub use crate::src::krb5::import_cred::krb5_gss_import_cred;
pub use crate::src::krb5::import_name::krb5_gss_import_name;
pub use crate::src::krb5::import_sec_context::krb5_gss_import_sec_context;
pub use crate::src::krb5::indicate_mechs::krb5_gss_indicate_mechs;
pub use crate::src::krb5::init_sec_context::kg_kdc_flag_mutex;
pub use crate::src::krb5::init_sec_context::krb5_gss_init_context;
pub use crate::src::krb5::init_sec_context::krb5_gss_init_sec_context;
pub use crate::src::krb5::init_sec_context::krb5int_gss_use_kdc_context;
pub use crate::src::krb5::inq_context::gss_krb5int_extract_authtime_from_sec_context;
pub use crate::src::krb5::inq_context::gss_krb5int_extract_authz_data_from_sec_context;
pub use crate::src::krb5::inq_context::gss_krb5int_inq_session_key;
pub use crate::src::krb5::inq_context::gss_krb5int_sec_context_sasl_ssf;
pub use crate::src::krb5::inq_context::krb5_gss_inquire_context;
pub use crate::src::krb5::inq_cred::gss_krb5int_get_cred_impersonator;
pub use crate::src::krb5::inq_cred::krb5_gss_inquire_cred;
pub use crate::src::krb5::inq_cred::krb5_gss_inquire_cred_by_mech;
pub use crate::src::krb5::inq_names::krb5_gss_inquire_names_for_mech;
pub use crate::src::krb5::k5seal::krb5_gss_get_mic;
pub use crate::src::krb5::k5seal::krb5_gss_wrap;
pub use crate::src::krb5::k5sealiov::krb5_gss_get_mic_iov;
pub use crate::src::krb5::k5sealiov::krb5_gss_get_mic_iov_length;
pub use crate::src::krb5::k5sealiov::krb5_gss_wrap_iov;
pub use crate::src::krb5::k5sealiov::krb5_gss_wrap_iov_length;
pub use crate::src::krb5::k5unseal::krb5_gss_unwrap;
pub use crate::src::krb5::k5unseal::krb5_gss_verify_mic;
pub use crate::src::krb5::k5unsealiov::krb5_gss_unwrap_iov;
pub use crate::src::krb5::k5unsealiov::krb5_gss_verify_mic_iov;
pub use crate::src::krb5::lucid_context::gss_krb5int_export_lucid_sec_context;
pub use crate::src::krb5::lucid_context::gss_krb5int_free_lucid_sec_context;
pub use crate::src::krb5::naming_exts::krb5_gss_delete_name_attribute;
pub use crate::src::krb5::naming_exts::krb5_gss_export_name_composite;
pub use crate::src::krb5::naming_exts::krb5_gss_get_name_attribute;
pub use crate::src::krb5::naming_exts::krb5_gss_inquire_name;
pub use crate::src::krb5::naming_exts::krb5_gss_map_name_to_any;
pub use crate::src::krb5::naming_exts::krb5_gss_release_any_name_mapping;
pub use crate::src::krb5::naming_exts::krb5_gss_set_name_attribute;
pub use crate::src::krb5::prf::krb5_gss_pseudo_random;
pub use crate::src::krb5::process_context_token::krb5_gss_process_context_token;
pub use crate::src::krb5::rel_cred::krb5_gss_release_cred;
pub use crate::src::krb5::rel_name::krb5_gss_release_name;
pub use crate::src::krb5::rel_oid::krb5_gss_internal_release_oid;
pub use crate::src::krb5::s4u_gss_glue::krb5_gss_acquire_cred_impersonate_name;
pub use crate::src::krb5::set_allowable_enctypes::gss_krb5int_set_allowable_enctypes;
pub use crate::src::krb5::set_ccache::gss_krb5int_ccache_name;
pub use crate::src::krb5::store_cred::krb5_gss_store_cred;
pub use crate::src::krb5::store_cred::krb5_gss_store_cred_into;
pub use crate::src::krb5::val_cred::krb5_gss_validate_cred;
pub use crate::src::krb5::wrap_size_limit::krb5_gss_wrap_size_limit;
pub use crate::src::mechglue::g_initialize::gssint_mechglue_initialize_library;
pub use crate::src::mechglue::g_initialize::gssint_register_mechinfo;
pub use crate::src::mechglue::g_oid_ops::gss_add_oid_set_member;
pub use crate::src::mechglue::g_oid_ops::gss_create_empty_oid_set;
pub use crate::src::mechglue::g_rel_oid_set::gss_release_oid_set;

/*
 * gss_inquire_sec_context_by_oid() methods
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_8 {
    pub oid: crate::gssapi_h::gss_OID_desc,
    pub func: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_ctx_id_t,
            _: crate::gssapi_h::gss_OID,
            _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
}
/*
 * gssspi_mech_invoke() methods
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_9 {
    pub oid: crate::gssapi_h::gss_OID_desc,
    pub func: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_OID,
            _: crate::gssapi_h::gss_OID,
            _: crate::gssapi_h::gss_buffer_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
}
/*
 * gssspi_set_cred_option() methods
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_10 {
    pub oid: crate::gssapi_h::gss_OID_desc,
    pub func: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: *mut crate::gssapi_h::gss_cred_id_t,
            _: crate::gssapi_h::gss_OID,
            _: crate::gssapi_h::gss_buffer_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
}
/*
 * gss_inquire_cred_by_oid() methods
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_11 {
    pub oid: crate::gssapi_h::gss_OID_desc,
    pub func: Option<
        unsafe extern "C" fn(
            _: *mut crate::gssapi_h::OM_uint32,
            _: crate::gssapi_h::gss_cred_id_t,
            _: crate::gssapi_h::gss_OID,
            _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
        ) -> crate::gssapi_h::OM_uint32,
    >,
}
#[no_mangle]

pub static mut krb5_gss_oid_array: [crate::gssapi_h::gss_OID_desc; 11] = [
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 9u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 5u32,
            elements: b"+\x05\x01\x05\x02\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 9u32,
            elements: b"*\x86H\x82\xf7\x12\x01\x02\x02\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 6u32,
            elements: b"+\x06\x01\x05\x02\x05\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 9u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x03\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 10u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x01\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 10u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x02\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 6u32,
            elements: b"*\x85p+\r\x1d\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 11u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x0e\x00" as *const u8
                as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 10u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x06\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 0u32,
            elements: 0 as *mut libc::c_void,
        };
        init
    },
];
// Initialized in run_static_initializers
#[no_mangle]

pub static mut gss_mech_krb5: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut gss_mech_krb5_old: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut gss_mech_krb5_wrong: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut gss_mech_iakerb: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut gss_nt_krb5_name: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut gss_nt_krb5_principal: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_KRB5_NT_PRINCIPAL_NAME: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_KRB5_CRED_NO_CI_FLAGS_X: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_KRB5_GET_CRED_IMPERSONATOR: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_KRB5_NT_ENTERPRISE_NAME: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers

static mut oidsets: [crate::gssapi_h::gss_OID_set_desc; 4] = [crate::gssapi_h::gss_OID_set_desc {
    count: 0,
    elements: 0 as *mut crate::gssapi_h::gss_OID_desc_struct,
}; 4];
// Initialized in run_static_initializers
#[no_mangle]

pub static mut gss_mech_set_krb5: crate::gssapi_h::gss_OID_set =
    0 as *mut crate::gssapi_h::gss_OID_set_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut gss_mech_set_krb5_old: crate::gssapi_h::gss_OID_set =
    0 as *mut crate::gssapi_h::gss_OID_set_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut gss_mech_set_krb5_both: crate::gssapi_h::gss_OID_set =
    0 as *mut crate::gssapi_h::gss_OID_set_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut kg_all_mechs: crate::gssapi_h::gss_OID_set =
    0 as *mut crate::gssapi_h::gss_OID_set_desc_struct;
#[no_mangle]

pub static mut kg_vdb: crate::gssapiP_generic_h::g_set = {
    let mut init = crate::gssapiP_generic_h::g_set {
        mutex: crate::stdlib::pthread_mutex_t {
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
        },
        data: 0 as *mut libc::c_void,
    };
    init
};
/* * default credential support */
/*
 * init_sec_context() will explicitly re-acquire default credentials,
 * so handling the expiration/invalidation condition here isn't needed.
 */
#[no_mangle]

pub unsafe extern "C" fn kg_get_defcred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut cred: *mut crate::gssapi_h::gss_cred_id_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    major = crate::src::krb5::acquire_cred::krb5_gss_acquire_cred(
        minor_status,
        0 as crate::gssapi_h::gss_name_t,
        0xffffffffu32,
        0 as crate::gssapi_h::gss_OID_set,
        1i32,
        cred,
        0 as *mut crate::gssapi_h::gss_OID_set,
        0 as *mut crate::gssapi_h::OM_uint32,
    );
    if major != 0 && major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
        return major;
    }
    *minor_status = 0u32;
    return 0u32;
}
#[no_mangle]

pub unsafe extern "C" fn kg_sync_ccache_name(
    mut context: crate::krb5_h::krb5_context,
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut err = 0u32;
    /*
     * Sync up the context ccache name with the GSSAPI ccache name.
     * If kg_ccache_name is NULL -- normal unless someone has called
     * gss_krb5_ccache_name() -- then the system default ccache will
     * be picked up and used by resetting the context default ccache.
     * This is needed for platforms which support multiple ccaches.
     */
    if err == 0 {
        /* if NULL, resets the context default ccache */
        err = crate::krb5_h::krb5_cc_set_default_name(
            context,
            crate::k5_thread_h::krb5int_getspecific(crate::k5_thread_h::K5_KEY_GSS_KRB5_CCACHE_NAME)
                as *mut i8,
        ) as crate::gssapi_h::OM_uint32
    }
    *minor_status = err;
    return if *minor_status == 0u32 {
        0u32
    } else {
        (13u32) << 16i32
    };
}
/* This function returns whether or not the caller set a cccache name.  Used by
 * gss_acquire_cred to figure out if the caller wants to only look at this
 * ccache or search the cache collection for the desired name */
#[no_mangle]

pub unsafe extern "C" fn kg_caller_provided_ccache_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut out_caller_provided_name: *mut i32,
) -> crate::gssapi_h::OM_uint32 {
    if !out_caller_provided_name.is_null() {
        *out_caller_provided_name = (crate::k5_thread_h::krb5int_getspecific(
            crate::k5_thread_h::K5_KEY_GSS_KRB5_CCACHE_NAME,
        ) != 0 as *mut libc::c_void) as i32
    }
    *minor_status = 0u32;
    return 0u32;
}
#[no_mangle]

pub unsafe extern "C" fn kg_get_ccache_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut out_name: *mut *const i8,
) -> crate::gssapi_h::OM_uint32 {
    let mut name = 0 as *const i8;
    let mut err = 0u32;
    let mut kg_ccache_name = 0 as *mut i8;
    kg_ccache_name =
        crate::k5_thread_h::krb5int_getspecific(crate::k5_thread_h::K5_KEY_GSS_KRB5_CCACHE_NAME)
            as *mut i8;
    if !kg_ccache_name.is_null() {
        name = crate::stdlib::strdup(kg_ccache_name);
        if name.is_null() {
            err = 12u32
        }
    } else {
        let mut context = 0 as crate::krb5_h::krb5_context;
        /* Reset the context default ccache (see text above), and then
        retrieve it.  */
        err = crate::src::krb5::init_sec_context::krb5_gss_init_context(&mut context)
            as crate::gssapi_h::OM_uint32;
        if err == 0 {
            err = crate::krb5_h::krb5_cc_set_default_name(context, 0 as *const i8)
                as crate::gssapi_h::OM_uint32
        }
        if err == 0 {
            name = crate::krb5_h::krb5_cc_default_name(context);
            if !name.is_null() {
                name = crate::stdlib::strdup(name);
                if name.is_null() {
                    err = 12u32
                }
            }
        }
        if err != 0 && !context.is_null() {
            crate::src::krb5::disp_status::krb5_gss_save_error_info(err, context);
        }
        if !context.is_null() {
            crate::krb5_h::krb5_free_context(context);
        }
    }
    if err == 0 {
        if !out_name.is_null() {
            *out_name = name
        }
    }
    *minor_status = err;
    return if *minor_status == 0u32 {
        0u32
    } else {
        (13u32) << 16i32
    };
}
#[no_mangle]

pub unsafe extern "C" fn kg_set_ccache_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut name: *const i8,
) -> crate::gssapi_h::OM_uint32 {
    let mut new_name = 0 as *mut i8;
    let mut swap = 0 as *mut i8;
    let mut kg_ccache_name = 0 as *mut i8;
    let mut kerr: crate::krb5_h::krb5_error_code = 0;
    if !name.is_null() {
        new_name = crate::stdlib::strdup(name);
        if new_name.is_null() {
            *minor_status = 12u32;
            return (13u32) << 16i32;
        }
    }
    kg_ccache_name =
        crate::k5_thread_h::krb5int_getspecific(crate::k5_thread_h::K5_KEY_GSS_KRB5_CCACHE_NAME)
            as *mut i8;
    swap = kg_ccache_name;
    kg_ccache_name = new_name;
    new_name = swap;
    kerr = crate::k5_thread_h::krb5int_setspecific(
        crate::k5_thread_h::K5_KEY_GSS_KRB5_CCACHE_NAME,
        kg_ccache_name as *mut libc::c_void,
    );
    if kerr != 0i32 {
        /* Can't store, so free up the storage.  */
        crate::stdlib::free(kg_ccache_name as *mut libc::c_void);
        /* ??? free(new_name); */
        *minor_status = kerr as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    crate::stdlib::free(new_name as *mut libc::c_void);
    *minor_status = 0u32;
    return 0u32;
}

static mut krb5_gss_inquire_sec_context_by_oid_ops: [C2RustUnnamed_8; 6] = {
    [
        {
            let mut init = C2RustUnnamed_8 {
                oid: {
                    let mut init = crate::gssapi_h::gss_OID_desc_struct {
                        length: 11u32,
                        elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x01\x00" as *const u8
                            as *mut libc::c_void,
                    };
                    init
                },
                func: Some(
                    crate::src::krb5::get_tkt_flags::gss_krb5int_get_tkt_flags
                        as unsafe extern "C" fn(
                            _: *mut crate::gssapi_h::OM_uint32,
                            _: crate::gssapi_h::gss_ctx_id_t,
                            _: crate::gssapi_h::gss_OID,
                            _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                        )
                            -> crate::gssapi_h::OM_uint32,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_8 {
                oid: {
                    let mut init = crate::gssapi_h::gss_OID_desc_struct {
                        length: 11u32,
                        elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\n\x00" as *const u8
                            as *mut libc::c_void,
                    };
                    init
                },
                func: Some(
                    crate::src::krb5::inq_context::gss_krb5int_extract_authz_data_from_sec_context
                        as unsafe extern "C" fn(
                            _: *mut crate::gssapi_h::OM_uint32,
                            _: crate::gssapi_h::gss_ctx_id_t,
                            _: crate::gssapi_h::gss_OID,
                            _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                        )
                            -> crate::gssapi_h::OM_uint32,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_8 {
                oid: {
                    let mut init = crate::gssapi_h::gss_OID_desc_struct {
                        length: 11u32,
                        elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x05\x00" as *const u8
                            as *mut libc::c_void,
                    };
                    init
                },
                func: Some(
                    crate::src::krb5::inq_context::gss_krb5int_inq_session_key
                        as unsafe extern "C" fn(
                            _: *mut crate::gssapi_h::OM_uint32,
                            _: crate::gssapi_h::gss_ctx_id_t,
                            _: crate::gssapi_h::gss_OID,
                            _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                        )
                            -> crate::gssapi_h::OM_uint32,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_8 {
                oid: {
                    let mut init = crate::gssapi_h::gss_OID_desc_struct {
                        length: 11u32,
                        elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x06\x00" as *const u8
                            as *mut libc::c_void,
                    };
                    init
                },
                func: Some(
                    crate::src::krb5::lucid_context::gss_krb5int_export_lucid_sec_context
                        as unsafe extern "C" fn(
                            _: *mut crate::gssapi_h::OM_uint32,
                            _: crate::gssapi_h::gss_ctx_id_t,
                            _: crate::gssapi_h::gss_OID,
                            _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                        )
                            -> crate::gssapi_h::OM_uint32,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_8 {
                oid: {
                    let mut init = crate::gssapi_h::gss_OID_desc_struct {
                        length: 11u32,
                        elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x0c\x00" as *const u8
                            as *mut libc::c_void,
                    };
                    init
                },
                func: Some(
                    crate::src::krb5::inq_context::gss_krb5int_extract_authtime_from_sec_context
                        as unsafe extern "C" fn(
                            _: *mut crate::gssapi_h::OM_uint32,
                            _: crate::gssapi_h::gss_ctx_id_t,
                            _: crate::gssapi_h::gss_OID,
                            _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                        )
                            -> crate::gssapi_h::OM_uint32,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_8 {
                oid: {
                    let mut init = crate::gssapi_h::gss_OID_desc_struct {
                        length: 11u32,
                        elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x0f\x00" as *const u8
                            as *mut libc::c_void,
                    };
                    init
                },
                func: Some(
                    crate::src::krb5::inq_context::gss_krb5int_sec_context_sasl_ssf
                        as unsafe extern "C" fn(
                            _: *mut crate::gssapi_h::OM_uint32,
                            _: crate::gssapi_h::gss_ctx_id_t,
                            _: crate::gssapi_h::gss_OID,
                            _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                        )
                            -> crate::gssapi_h::OM_uint32,
                ),
            };
            init
        },
    ]
};
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_inquire_sec_context_by_oid(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    context_handle: crate::gssapi_h::gss_ctx_id_t,
    desired_object: crate::gssapi_h::gss_OID,
    mut data_set: *mut crate::gssapi_ext_h::gss_buffer_set_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = 0 as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    let mut i: crate::stddef_h::size_t = 0;
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    *minor_status = 0u32;
    if desired_object.is_null() {
        return (1u32) << 24i32;
    }
    if data_set.is_null() {
        return (2u32) << 24i32;
    }
    *data_set = 0 as crate::gssapi_ext_h::gss_buffer_set_t;
    ctx = context_handle as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    if (*ctx).terminated() as i32 != 0 || (*ctx).established() == 0 {
        return (8u32) << 16i32;
    }
    i = 0usize;
    while i
        < (::std::mem::size_of::<[C2RustUnnamed_8; 6]>())
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_8>())
    {
        if (*desired_object).length >= krb5_gss_inquire_sec_context_by_oid_ops[i].oid.length
            && crate::stdlib::memcmp(
                (*desired_object).elements,
                krb5_gss_inquire_sec_context_by_oid_ops[i].oid.elements,
                krb5_gss_inquire_sec_context_by_oid_ops[i].oid.length as usize,
            ) == 0i32
        {
            return Some(
                (*krb5_gss_inquire_sec_context_by_oid_ops
                    .as_mut_ptr()
                    .offset(i as isize))
                .func
                .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                minor_status,
                context_handle,
                desired_object,
                data_set,
            );
        }
        i = i.wrapping_add(1)
    }
    *minor_status = 22u32;
    return (16u32) << 16i32;
}

static mut krb5_gss_inquire_cred_by_oid_ops: [C2RustUnnamed_11; 1] = {
    [{
        let mut init = C2RustUnnamed_11 {
            oid: {
                let mut init = crate::gssapi_h::gss_OID_desc_struct {
                    length: 11u32,
                    elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x0e\x00" as *const u8
                        as *mut libc::c_void,
                };
                init
            },
            func: Some(
                crate::src::krb5::inq_cred::gss_krb5int_get_cred_impersonator
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: crate::gssapi_h::gss_OID,
                        _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
        };
        init
    }]
};

unsafe extern "C" fn krb5_gss_inquire_cred_by_oid(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    cred_handle: crate::gssapi_h::gss_cred_id_t,
    desired_object: crate::gssapi_h::gss_OID,
    mut data_set: *mut crate::gssapi_ext_h::gss_buffer_set_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut major_status = (13u32) << 16i32;
    let mut i: crate::stddef_h::size_t = 0;
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    *minor_status = 0u32;
    if desired_object.is_null() {
        return (1u32) << 24i32;
    }
    if data_set.is_null() {
        return (2u32) << 24i32;
    }
    *data_set = 0 as crate::gssapi_ext_h::gss_buffer_set_t;
    if cred_handle.is_null() {
        *minor_status = -(1765328181 as isize) as crate::gssapi_h::OM_uint32;
        return (7u32) << 16i32;
    }
    major_status = crate::src::krb5::val_cred::krb5_gss_validate_cred(minor_status, cred_handle);
    if major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
        return major_status;
    }
    i = 0usize;
    while i
        < (::std::mem::size_of::<[C2RustUnnamed_11; 1]>())
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_11>())
    {
        if (*desired_object).length >= krb5_gss_inquire_cred_by_oid_ops[i].oid.length
            && crate::stdlib::memcmp(
                (*desired_object).elements,
                krb5_gss_inquire_cred_by_oid_ops[i].oid.elements,
                krb5_gss_inquire_cred_by_oid_ops[i].oid.length as usize,
            ) == 0i32
        {
            return Some(
                (*krb5_gss_inquire_cred_by_oid_ops
                    .as_mut_ptr()
                    .offset(i as isize))
                .func
                .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                minor_status,
                cred_handle,
                desired_object,
                data_set,
            );
        }
        i = i.wrapping_add(1)
    }
    *minor_status = 22u32;
    return (16u32) << 16i32;
}
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_set_sec_context_option(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    desired_object: crate::gssapi_h::gss_OID,
    _value: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    *minor_status = 0u32;
    if context_handle.is_null() {
        return (1u32) << 24i32;
    }
    if desired_object.is_null() {
        return (1u32) << 24i32;
    }
    *minor_status = 22u32;
    return (16u32) << 16i32;
}

unsafe extern "C" fn no_ci_flags(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    _desired_oid: crate::gssapi_h::gss_OID,
    _value: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut cred = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_cred_id_rec;
    cred = *cred_handle as crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
    (*cred).set_suppress_ci_flags(1u32);
    *minor_status = 0u32;
    return 0u32;
}

static mut krb5_gssspi_set_cred_option_ops: [C2RustUnnamed_10; 5] = {
    [
        {
            let mut init = C2RustUnnamed_10 {
                oid: {
                    let mut init = crate::gssapi_h::gss_OID_desc_struct {
                        length: 11u32,
                        elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x02\x00" as *const u8
                            as *mut libc::c_void,
                    };
                    init
                },
                func: Some(
                    crate::src::krb5::copy_ccache::gss_krb5int_copy_ccache
                        as unsafe extern "C" fn(
                            _: *mut crate::gssapi_h::OM_uint32,
                            _: *mut crate::gssapi_h::gss_cred_id_t,
                            _: crate::gssapi_h::gss_OID,
                            _: crate::gssapi_h::gss_buffer_t,
                        )
                            -> crate::gssapi_h::OM_uint32,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                oid: {
                    let mut init = crate::gssapi_h::gss_OID_desc_struct {
                        length: 11u32,
                        elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x04\x00" as *const u8
                            as *mut libc::c_void,
                    };
                    init
                },
                func: Some(
                    crate::src::krb5::set_allowable_enctypes::gss_krb5int_set_allowable_enctypes
                        as unsafe extern "C" fn(
                            _: *mut crate::gssapi_h::OM_uint32,
                            _: *mut crate::gssapi_h::gss_cred_id_t,
                            _: crate::gssapi_h::gss_OID,
                            _: crate::gssapi_h::gss_buffer_t,
                        )
                            -> crate::gssapi_h::OM_uint32,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                oid: {
                    let mut init = crate::gssapi_h::gss_OID_desc_struct {
                        length: 11u32,
                        elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x0b\x00" as *const u8
                            as *mut libc::c_void,
                    };
                    init
                },
                func: Some(
                    crate::src::krb5::acquire_cred::gss_krb5int_set_cred_rcache
                        as unsafe extern "C" fn(
                            _: *mut crate::gssapi_h::OM_uint32,
                            _: *mut crate::gssapi_h::gss_cred_id_t,
                            _: crate::gssapi_h::gss_OID,
                            _: crate::gssapi_h::gss_buffer_t,
                        )
                            -> crate::gssapi_h::OM_uint32,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                oid: {
                    let mut init = crate::gssapi_h::gss_OID_desc_struct {
                        length: 11u32,
                        elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\r\x00" as *const u8
                            as *mut libc::c_void,
                    };
                    init
                },
                func: Some(
                    crate::src::krb5::acquire_cred::gss_krb5int_import_cred
                        as unsafe extern "C" fn(
                            _: *mut crate::gssapi_h::OM_uint32,
                            _: *mut crate::gssapi_h::gss_cred_id_t,
                            _: crate::gssapi_h::gss_OID,
                            _: crate::gssapi_h::gss_buffer_t,
                        )
                            -> crate::gssapi_h::OM_uint32,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                oid: {
                    let mut init = crate::gssapi_h::gss_OID_desc_struct {
                        length: 6u32,
                        elements: b"*\x85p+\r\x1d\x00" as *const u8 as *mut libc::c_void,
                    };
                    init
                },
                func: Some(
                    no_ci_flags
                        as unsafe extern "C" fn(
                            _: *mut crate::gssapi_h::OM_uint32,
                            _: *mut crate::gssapi_h::gss_cred_id_t,
                            _: crate::gssapi_h::gss_OID,
                            _: crate::gssapi_h::gss_buffer_t,
                        )
                            -> crate::gssapi_h::OM_uint32,
                ),
            };
            init
        },
    ]
};

unsafe extern "C" fn krb5_gssspi_set_cred_option(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    desired_object: crate::gssapi_h::gss_OID,
    value: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut major_status = (13u32) << 16i32;
    let mut i: crate::stddef_h::size_t = 0;
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    if cred_handle.is_null() {
        return (2u32) << 24i32;
    }
    *minor_status = 0u32;
    if desired_object.is_null() {
        return (1u32) << 24i32;
    }
    if !(*cred_handle).is_null() {
        major_status =
            crate::src::krb5::val_cred::krb5_gss_validate_cred(minor_status, *cred_handle);
        if major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
            return major_status;
        }
    }
    i = 0usize;
    while i
        < (::std::mem::size_of::<[C2RustUnnamed_10; 5]>())
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_10>())
    {
        if (*desired_object).length >= krb5_gssspi_set_cred_option_ops[i].oid.length
            && crate::stdlib::memcmp(
                (*desired_object).elements,
                krb5_gssspi_set_cred_option_ops[i].oid.elements,
                krb5_gssspi_set_cred_option_ops[i].oid.length as usize,
            ) == 0i32
        {
            return Some(
                (*krb5_gssspi_set_cred_option_ops
                    .as_mut_ptr()
                    .offset(i as isize))
                .func
                .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                minor_status, cred_handle, desired_object, value
            );
        }
        i = i.wrapping_add(1)
    }
    *minor_status = 22u32;
    return (16u32) << 16i32;
}

static mut krb5_gssspi_mech_invoke_ops: [C2RustUnnamed_9; 4] = {
    [
        {
            let mut init = C2RustUnnamed_9 {
                oid: {
                    let mut init = crate::gssapi_h::gss_OID_desc_struct {
                        length: 11u32,
                        elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\t\x00" as *const u8
                            as *mut libc::c_void,
                    };
                    init
                },
                func: Some(
                    crate::src::krb5::acquire_cred::gss_krb5int_register_acceptor_identity
                        as unsafe extern "C" fn(
                            _: *mut crate::gssapi_h::OM_uint32,
                            _: crate::gssapi_h::gss_OID,
                            _: crate::gssapi_h::gss_OID,
                            _: crate::gssapi_h::gss_buffer_t,
                        )
                            -> crate::gssapi_h::OM_uint32,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_9 {
                oid: {
                    let mut init = crate::gssapi_h::gss_OID_desc_struct {
                        length: 11u32,
                        elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x03\x00" as *const u8
                            as *mut libc::c_void,
                    };
                    init
                },
                func: Some(
                    crate::src::krb5::set_ccache::gss_krb5int_ccache_name
                        as unsafe extern "C" fn(
                            _: *mut crate::gssapi_h::OM_uint32,
                            _: crate::gssapi_h::gss_OID,
                            _: crate::gssapi_h::gss_OID,
                            _: crate::gssapi_h::gss_buffer_t,
                        )
                            -> crate::gssapi_h::OM_uint32,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_9 {
                oid: {
                    let mut init = crate::gssapi_h::gss_OID_desc_struct {
                        length: 11u32,
                        elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x07\x00" as *const u8
                            as *mut libc::c_void,
                    };
                    init
                },
                func: Some(
                    crate::src::krb5::lucid_context::gss_krb5int_free_lucid_sec_context
                        as unsafe extern "C" fn(
                            _: *mut crate::gssapi_h::OM_uint32,
                            _: crate::gssapi_h::gss_OID,
                            _: crate::gssapi_h::gss_OID,
                            _: crate::gssapi_h::gss_buffer_t,
                        )
                            -> crate::gssapi_h::OM_uint32,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_9 {
                oid: {
                    let mut init = crate::gssapi_h::gss_OID_desc_struct {
                        length: 11u32,
                        elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x08\x00" as *const u8
                            as *mut libc::c_void,
                    };
                    init
                },
                func: Some(
                    crate::src::krb5::init_sec_context::krb5int_gss_use_kdc_context
                        as unsafe extern "C" fn(
                            _: *mut crate::gssapi_h::OM_uint32,
                            _: crate::gssapi_h::gss_OID,
                            _: crate::gssapi_h::gss_OID,
                            _: crate::gssapi_h::gss_buffer_t,
                        )
                            -> crate::gssapi_h::OM_uint32,
                ),
            };
            init
        },
    ]
};

unsafe extern "C" fn krb5_gssspi_mech_invoke(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    desired_mech: crate::gssapi_h::gss_OID,
    desired_object: crate::gssapi_h::gss_OID,
    mut value: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut i: crate::stddef_h::size_t = 0;
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    *minor_status = 0u32;
    if desired_mech.is_null() {
        return (1u32) << 16i32;
    }
    if desired_object.is_null() {
        return (1u32) << 24i32;
    }
    i = 0usize;
    while i
        < (::std::mem::size_of::<[C2RustUnnamed_9; 4]>())
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_9>())
    {
        if (*desired_object).length >= krb5_gssspi_mech_invoke_ops[i].oid.length
            && crate::stdlib::memcmp(
                (*desired_object).elements,
                krb5_gssspi_mech_invoke_ops[i].oid.elements,
                krb5_gssspi_mech_invoke_ops[i].oid.length as usize,
            ) == 0i32
        {
            return Some(
                (*krb5_gssspi_mech_invoke_ops.as_mut_ptr().offset(i as isize))
                    .func
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                minor_status,
                desired_mech,
                desired_object,
                value,
            );
        }
        i = i.wrapping_add(1)
    }
    *minor_status = 22u32;
    return (16u32) << 16i32;
}

unsafe extern "C" fn krb5_gss_inquire_mech_for_saslname(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    sasl_mech_name: crate::gssapi_h::gss_buffer_t,
    mut mech_type: *mut crate::gssapi_h::gss_OID,
) -> crate::gssapi_h::OM_uint32 {
    *minor_status = 0u32;
    if (*sasl_mech_name).length == (::std::mem::size_of::<[i8; 9]>()).wrapping_sub(1usize)
        && crate::stdlib::memcmp(
            (*sasl_mech_name).value,
            b"GS2-KRB5\x00" as *const u8 as *const libc::c_void,
            (::std::mem::size_of::<[i8; 9]>()).wrapping_sub(1usize),
        ) == 0i32
    {
        if !mech_type.is_null() {
            *mech_type = gss_mech_krb5
        }
        return 0u32;
    } else {
        if (*sasl_mech_name).length == (::std::mem::size_of::<[i8; 11]>()).wrapping_sub(1usize)
            && crate::stdlib::memcmp(
                (*sasl_mech_name).value,
                b"GS2-IAKERB\x00" as *const u8 as *const libc::c_void,
                (::std::mem::size_of::<[i8; 11]>()).wrapping_sub(1usize),
            ) == 0i32
        {
            if !mech_type.is_null() {
                *mech_type = gss_mech_iakerb
            }
            return 0u32;
        }
    }
    return (1u32) << 16i32;
}

unsafe extern "C" fn krb5_gss_inquire_saslname_for_mech(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    desired_mech: crate::gssapi_h::gss_OID,
    mut sasl_mech_name: crate::gssapi_h::gss_buffer_t,
    mut mech_name: crate::gssapi_h::gss_buffer_t,
    mut mech_description: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    if (*desired_mech).length == (*gss_mech_iakerb).length
        && crate::stdlib::memcmp(
            (*desired_mech).elements,
            (*gss_mech_iakerb).elements,
            (*desired_mech).length as usize,
        ) == 0i32
    {
        if crate::src::generic::util_buffer::gssint_g_make_string_buffer(
            b"GS2-IAKERB\x00" as *const u8 as *const i8,
            sasl_mech_name,
        ) == 0
            || crate::src::generic::util_buffer::gssint_g_make_string_buffer(
                b"iakerb\x00" as *const u8 as *const i8,
                mech_name,
            ) == 0
            || crate::src::generic::util_buffer::gssint_g_make_string_buffer(
                b"Initial and Pass Through Authentication Kerberos Mechanism (IAKERB)\x00"
                    as *const u8 as *const i8,
                mech_description,
            ) == 0
        {
            current_block = 2215464789205338958;
        } else {
            current_block = 735147466149431745;
        }
    } else if crate::src::generic::util_buffer::gssint_g_make_string_buffer(
        b"GS2-KRB5\x00" as *const u8 as *const i8,
        sasl_mech_name,
    ) == 0
        || crate::src::generic::util_buffer::gssint_g_make_string_buffer(
            b"krb5\x00" as *const u8 as *const i8,
            mech_name,
        ) == 0
        || crate::src::generic::util_buffer::gssint_g_make_string_buffer(
            b"Kerberos 5 GSS-API Mechanism\x00" as *const u8 as *const i8,
            mech_description,
        ) == 0
    {
        current_block = 2215464789205338958;
    } else {
        current_block = 735147466149431745;
    }
    match current_block {
        735147466149431745 => {
            *minor_status = 0u32;
            return 0u32;
        }
        _ => {
            *minor_status = 12u32;
            return (13u32) << 16i32;
        }
    };
}

unsafe extern "C" fn krb5_gss_inquire_attrs_for_mech(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut mech: crate::gssapi_h::gss_const_OID,
    mut mech_attrs: *mut crate::gssapi_h::gss_OID_set,
    mut _known_mech_attrs: *mut crate::gssapi_h::gss_OID_set,
) -> crate::gssapi_h::OM_uint32 {
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpMinor: crate::gssapi_h::OM_uint32 = 0;
    if mech_attrs.is_null() {
        *minor_status = 0u32;
        return 0u32;
    }
    major = crate::src::mechglue::g_oid_ops::gss_create_empty_oid_set(minor_status, mech_attrs);
    if !(major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0) {
        major = crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(
            minor_status,
            crate::src::generic::gssapi_generic::GSS_C_MA_MECH_CONCRETE as crate::gssapi_h::gss_OID,
            mech_attrs,
        );
        if !(major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0) {
            major = crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(
                minor_status,
                crate::src::generic::gssapi_generic::GSS_C_MA_ITOK_FRAMED
                    as crate::gssapi_h::gss_OID,
                mech_attrs,
            );
            if !(major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0) {
                major = crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(
                    minor_status,
                    crate::src::generic::gssapi_generic::GSS_C_MA_AUTH_INIT
                        as crate::gssapi_h::gss_OID,
                    mech_attrs,
                );
                if !(major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0) {
                    major = crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(
                        minor_status,
                        crate::src::generic::gssapi_generic::GSS_C_MA_AUTH_TARG
                            as crate::gssapi_h::gss_OID,
                        mech_attrs,
                    );
                    if !(major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0) {
                        major = crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(
                            minor_status,
                            crate::src::generic::gssapi_generic::GSS_C_MA_DELEG_CRED
                                as crate::gssapi_h::gss_OID,
                            mech_attrs,
                        );
                        if !(major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0) {
                            major = crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(
                                minor_status,
                                crate::src::generic::gssapi_generic::GSS_C_MA_INTEG_PROT
                                    as crate::gssapi_h::gss_OID,
                                mech_attrs,
                            );
                            if !(major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0) {
                                major = crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(
                                    minor_status,
                                    crate::src::generic::gssapi_generic::GSS_C_MA_CONF_PROT
                                        as crate::gssapi_h::gss_OID,
                                    mech_attrs,
                                );
                                if !(major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0) {
                                    major = crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(
                                        minor_status,
                                        crate::src::generic::gssapi_generic::GSS_C_MA_MIC
                                            as crate::gssapi_h::gss_OID,
                                        mech_attrs,
                                    );
                                    if !(major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0) {
                                        major =
                                            crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(
                                                minor_status,
                                                crate::src::generic::gssapi_generic::GSS_C_MA_WRAP
                                                    as crate::gssapi_h::gss_OID,
                                                mech_attrs,
                                            );
                                        if !(major & ((0o377u32) << 24i32 | (0o377u32) << 16i32)
                                            != 0)
                                        {
                                            major =
                                                crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(minor_status,
                                                                       crate::src::generic::gssapi_generic::GSS_C_MA_PROT_READY
                                                                           as
                                                                           crate::gssapi_h::gss_OID,
                                                                       mech_attrs);
                                            if !(major
                                                & ((0o377u32) << 24i32 | (0o377u32) << 16i32)
                                                != 0)
                                            {
                                                major =
                                                    crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(minor_status,
                                                                           crate::src::generic::gssapi_generic::GSS_C_MA_REPLAY_DET
                                                                               as
                                                                               crate::gssapi_h::gss_OID,
                                                                           mech_attrs);
                                                if !(major
                                                    & ((0o377u32) << 24i32 | (0o377u32) << 16i32)
                                                    != 0)
                                                {
                                                    major =
                                                        crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(minor_status,
                                                                               crate::src::generic::gssapi_generic::GSS_C_MA_OOS_DET
                                                                                   as
                                                                                   crate::gssapi_h::gss_OID,
                                                                               mech_attrs);
                                                    if !(major
                                                        & ((0o377u32) << 24i32
                                                            | (0o377u32) << 16i32)
                                                        != 0)
                                                    {
                                                        major =
                                                            crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(minor_status,
                                                                                   crate::src::generic::gssapi_generic::GSS_C_MA_CBINDINGS
                                                                                       as
                                                                                       crate::gssapi_h::gss_OID,
                                                                                   mech_attrs);
                                                        if !(major
                                                            & ((0o377u32) << 24i32
                                                                | (0o377u32) << 16i32)
                                                            != 0)
                                                        {
                                                            major =
                                                                crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(minor_status,
                                                                                       crate::src::generic::gssapi_generic::GSS_C_MA_CTX_TRANS
                                                                                           as
                                                                                           crate::gssapi_h::gss_OID,
                                                                                       mech_attrs);
                                                            if !(major
                                                                & ((0o377u32) << 24i32
                                                                    | (0o377u32) << 16i32)
                                                                != 0)
                                                            {
                                                                if (*mech).length
                                                                    == (*gss_mech_iakerb).length
                                                                    && crate::stdlib::memcmp(
                                                                        (*mech).elements,
                                                                        (*gss_mech_iakerb).elements,
                                                                        (*mech).length as usize,
                                                                    ) == 0i32
                                                                {
                                                                    major =
                                                                        crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(minor_status,
                                                                                               crate::src::generic::gssapi_generic::GSS_C_MA_AUTH_INIT_INIT
                                                                                                   as
                                                                                                   crate::gssapi_h::gss_OID,
                                                                                               mech_attrs);
                                                                    if !(major
                                                                        & ((0o377u32) << 24i32
                                                                            | (0o377u32) << 16i32)
                                                                        != 0)
                                                                    {
                                                                        major
                                                                            =
                                                                            crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(minor_status,
                                                                                                   crate::src::generic::gssapi_generic::GSS_C_MA_NOT_DFLT_MECH
                                                                                                       as
                                                                                                       crate::gssapi_h::gss_OID,
                                                                                                   mech_attrs);
                                                                        (major
                                                                            & ((0o377u32)
                                                                                << 24i32
                                                                                | (0o377u32)
                                                                                    << 16i32))
                                                                            != 0;
                                                                    }
                                                                } else if !((*mech).length
                                                                    == (*gss_mech_krb5).length
                                                                    && crate::stdlib::memcmp(
                                                                        (*mech).elements,
                                                                        (*gss_mech_krb5).elements,
                                                                        (*mech).length as usize,
                                                                    ) == 0i32)
                                                                {
                                                                    major =
                                                                        crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(minor_status,
                                                                                               crate::src::generic::gssapi_generic::GSS_C_MA_DEPRECATED
                                                                                                   as
                                                                                                   crate::gssapi_h::gss_OID,
                                                                                               mech_attrs);
                                                                    (major
                                                                        & ((0o377u32) << 24i32
                                                                            | (0o377u32) << 16i32))
                                                                        != 0;
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
    if major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
        crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(&mut tmpMinor, mech_attrs);
    }
    return major;
}

unsafe extern "C" fn krb5_gss_localname(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    pname: crate::gssapi_h::gss_name_t,
    _mech_type: crate::gssapi_h::gss_const_OID,
    mut localname: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut context = 0 as *mut crate::k5_int_h::_krb5_context;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut kname = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_name_rec;
    let mut lname: [i8; 8192] = [0; 8192];
    code = crate::src::krb5::init_sec_context::krb5_gss_init_context(&mut context);
    if code != 0i32 {
        *minor = code as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    kname = pname as crate::gssapiP_krb5_h::krb5_gss_name_t;
    code = crate::krb5_h::krb5_aname_to_localname(
        context,
        (*kname).princ as crate::krb5_h::krb5_const_principal,
        ::std::mem::size_of::<[i8; 8192]>() as i32,
        lname.as_mut_ptr(),
    );
    if code != 0i32 {
        *minor = -(1765328227 as isize) as crate::gssapi_h::OM_uint32;
        crate::krb5_h::krb5_free_context(context);
        return (13u32) << 16i32;
    }
    crate::krb5_h::krb5_free_context(context);
    (*localname).value = gssalloc_strdup(lname.as_mut_ptr()) as *mut libc::c_void;
    (*localname).length = crate::stdlib::strlen(lname.as_mut_ptr());
    return 0u32;
}

unsafe extern "C" fn krb5_gss_authorize_localname(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    pname: crate::gssapi_h::gss_name_t,
    mut local_user: crate::gssapi_h::gss_const_buffer_t,
    mut name_type: crate::gssapi_h::gss_const_OID,
) -> crate::gssapi_h::OM_uint32 {
    let mut context = 0 as *mut crate::k5_int_h::_krb5_context;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut kname = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_name_rec;
    let mut user = 0 as *mut i8;
    let mut user_ok: i32 = 0;
    if !name_type.is_null()
        && !((*name_type).length
            == (*crate::src::generic::gssapi_generic::GSS_C_NT_USER_NAME).length
            && crate::stdlib::memcmp(
                (*name_type).elements,
                (*crate::src::generic::gssapi_generic::GSS_C_NT_USER_NAME).elements,
                (*name_type).length as usize,
            ) == 0i32)
    {
        return (3u32) << 16i32;
    }
    kname = pname as crate::gssapiP_krb5_h::krb5_gss_name_t;
    code = crate::src::krb5::init_sec_context::krb5_gss_init_context(&mut context);
    if code != 0i32 {
        *minor = code as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    user = k5memdup0((*local_user).value, (*local_user).length, &mut code) as *mut i8;
    if user.is_null() {
        *minor = code as crate::gssapi_h::OM_uint32;
        crate::krb5_h::krb5_free_context(context);
        return (13u32) << 16i32;
    }
    user_ok = crate::krb5_h::krb5_kuserok(context, (*kname).princ, user) as i32;
    crate::stdlib::free(user as *mut libc::c_void);
    crate::krb5_h::krb5_free_context(context);
    *minor = 0u32;
    return if user_ok != 0 { 0u32 } else { (15u32) << 16i32 };
}

static mut krb5_mechanism: crate::mglueP_h::gss_config = {
    {
        let mut init = crate::mglueP_h::gss_config {
            mech_type: {
                let mut init = crate::gssapi_h::gss_OID_desc_struct {
                    length: 9u32,
                    elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x00" as *const u8
                        as *mut libc::c_void,
                };
                init
            },
            context: 0 as *mut libc::c_void,
            gss_acquire_cred: Some(
                crate::src::krb5::acquire_cred::krb5_gss_acquire_cred
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_OID_set,
                        _: crate::gssapi_h::gss_cred_usage_t,
                        _: *mut crate::gssapi_h::gss_cred_id_t,
                        _: *mut crate::gssapi_h::gss_OID_set,
                        _: *mut crate::gssapi_h::OM_uint32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_release_cred: Some(
                crate::src::krb5::rel_cred::krb5_gss_release_cred
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_cred_id_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_init_sec_context: Some(
                crate::src::krb5::init_sec_context::krb5_gss_init_sec_context
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: *mut crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_channel_bindings_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::OM_uint32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_accept_sec_context: Some(
                crate::src::krb5::accept_sec_context::krb5_gss_accept_sec_context
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_channel_bindings_t,
                        _: *mut crate::gssapi_h::gss_name_t,
                        _: *mut crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_cred_id_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_process_context_token: Some(
                crate::src::krb5::process_context_token::krb5_gss_process_context_token
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_delete_sec_context: Some(
                crate::src::krb5::delete_sec_context::krb5_gss_delete_sec_context
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_context_time: Some(
                crate::src::krb5::context_time::krb5_gss_context_time
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: *mut crate::gssapi_h::OM_uint32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_get_mic: Some(
                crate::src::krb5::k5seal::krb5_gss_get_mic
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_qop_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_verify_mic: Some(
                crate::src::krb5::k5unseal::krb5_gss_verify_mic
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_h::gss_qop_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_wrap: Some(
                crate::src::krb5::k5seal::krb5_gss_wrap
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: i32,
                        _: crate::gssapi_h::gss_qop_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut i32,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_unwrap: Some(
                crate::src::krb5::k5unseal::krb5_gss_unwrap
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut i32,
                        _: *mut crate::gssapi_h::gss_qop_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_display_status: Some(
                crate::src::krb5::disp_status::krb5_gss_display_status
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::OM_uint32,
                        _: i32,
                        _: crate::gssapi_h::gss_OID,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_indicate_mechs: Some(
                crate::src::krb5::indicate_mechs::krb5_gss_indicate_mechs
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_OID_set,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_compare_name: Some(
                crate::src::krb5::compare_name::krb5_gss_compare_name
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_name_t,
                        _: *mut i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_display_name: Some(
                crate::src::krb5::disp_name::krb5_gss_display_name
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_h::gss_OID,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_import_name: Some(
                crate::src::krb5::import_name::krb5_gss_import_name
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_OID,
                        _: *mut crate::gssapi_h::gss_name_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_release_name: Some(
                crate::src::krb5::rel_name::krb5_gss_release_name
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_name_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_inquire_cred: Some(
                crate::src::krb5::inq_cred::krb5_gss_inquire_cred
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: *mut crate::gssapi_h::gss_name_t,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_cred_usage_t,
                        _: *mut crate::gssapi_h::gss_OID_set,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_add_cred: None,
            gss_export_sec_context: Some(
                crate::src::krb5::export_sec_context::krb5_gss_export_sec_context
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_import_sec_context: Some(
                crate::src::krb5::import_sec_context::krb5_gss_import_sec_context
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_h::gss_ctx_id_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_inquire_cred_by_mech: Some(
                crate::src::krb5::inq_cred::krb5_gss_inquire_cred_by_mech
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: crate::gssapi_h::gss_OID,
                        _: *mut crate::gssapi_h::gss_name_t,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_cred_usage_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_inquire_names_for_mech: Some(
                crate::src::krb5::inq_names::krb5_gss_inquire_names_for_mech
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_OID,
                        _: *mut crate::gssapi_h::gss_OID_set,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_inquire_context: Some(
                crate::src::krb5::inq_context::krb5_gss_inquire_context
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: *mut crate::gssapi_h::gss_name_t,
                        _: *mut crate::gssapi_h::gss_name_t,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_OID,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut i32,
                        _: *mut i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_internal_release_oid: Some(
                crate::src::krb5::rel_oid::krb5_gss_internal_release_oid
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_OID,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_wrap_size_limit: Some(
                crate::src::krb5::wrap_size_limit::krb5_gss_wrap_size_limit
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: i32,
                        _: crate::gssapi_h::gss_qop_t,
                        _: crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::OM_uint32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_localname: Some(
                krb5_gss_localname
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_const_OID,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gssspi_authorize_localname: Some(
                krb5_gss_authorize_localname
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_const_buffer_t,
                        _: crate::gssapi_h::gss_const_OID,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_export_name: Some(
                crate::src::krb5::export_name::krb5_gss_export_name
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_duplicate_name: Some(
                crate::src::krb5::duplicate_name::krb5_gss_duplicate_name
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: *mut crate::gssapi_h::gss_name_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_store_cred: Some(
                crate::src::krb5::store_cred::krb5_gss_store_cred
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: crate::gssapi_h::gss_cred_usage_t,
                        _: crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_OID_set,
                        _: *mut crate::gssapi_h::gss_cred_usage_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_inquire_sec_context_by_oid: Some(
                krb5_gss_inquire_sec_context_by_oid
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_OID,
                        _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_inquire_cred_by_oid: Some(
                krb5_gss_inquire_cred_by_oid
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: crate::gssapi_h::gss_OID,
                        _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_set_sec_context_option: Some(
                krb5_gss_set_sec_context_option
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gssspi_set_cred_option: Some(
                krb5_gssspi_set_cred_option
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_cred_id_t,
                        _: crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gssspi_mech_invoke: Some(
                krb5_gssspi_mech_invoke
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_wrap_aead: None,
            gss_unwrap_aead: None,
            gss_wrap_iov: Some(
                crate::src::krb5::k5sealiov::krb5_gss_wrap_iov
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: i32,
                        _: crate::gssapi_h::gss_qop_t,
                        _: *mut i32,
                        _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                        _: i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_unwrap_iov: Some(
                crate::src::krb5::k5unsealiov::krb5_gss_unwrap_iov
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: *mut i32,
                        _: *mut crate::gssapi_h::gss_qop_t,
                        _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                        _: i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_wrap_iov_length: Some(
                crate::src::krb5::k5sealiov::krb5_gss_wrap_iov_length
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: i32,
                        _: crate::gssapi_h::gss_qop_t,
                        _: *mut i32,
                        _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                        _: i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_complete_auth_token: None,
            gss_acquire_cred_impersonate_name: Some(
                crate::src::krb5::s4u_gss_glue::krb5_gss_acquire_cred_impersonate_name
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_OID_set,
                        _: crate::gssapi_h::gss_cred_usage_t,
                        _: *mut crate::gssapi_h::gss_cred_id_t,
                        _: *mut crate::gssapi_h::gss_OID_set,
                        _: *mut crate::gssapi_h::OM_uint32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_add_cred_impersonate_name: None,
            gss_display_name_ext: None,
            gss_inquire_name: Some(
                crate::src::krb5::naming_exts::krb5_gss_inquire_name
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: *mut i32,
                        _: *mut crate::gssapi_h::gss_OID,
                        _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_get_name_attribute: Some(
                crate::src::krb5::naming_exts::krb5_gss_get_name_attribute
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut i32,
                        _: *mut i32,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_set_name_attribute: Some(
                crate::src::krb5::naming_exts::krb5_gss_set_name_attribute
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: i32,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_delete_name_attribute: Some(
                crate::src::krb5::naming_exts::krb5_gss_delete_name_attribute
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_export_name_composite: Some(
                crate::src::krb5::naming_exts::krb5_gss_export_name_composite
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_map_name_to_any: Some(
                crate::src::krb5::naming_exts::krb5_gss_map_name_to_any
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: i32,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_ext_h::gss_any_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_release_any_name_mapping: Some(
                crate::src::krb5::naming_exts::krb5_gss_release_any_name_mapping
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_ext_h::gss_any_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_pseudo_random: Some(
                crate::src::krb5::prf::krb5_gss_pseudo_random
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: i32,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::stdlib::ssize_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_set_neg_mechs: None,
            gss_inquire_saslname_for_mech: Some(
                krb5_gss_inquire_saslname_for_mech
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_inquire_mech_for_saslname: Some(
                krb5_gss_inquire_mech_for_saslname
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_h::gss_OID,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_inquire_attrs_for_mech: Some(
                krb5_gss_inquire_attrs_for_mech
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_const_OID,
                        _: *mut crate::gssapi_h::gss_OID_set,
                        _: *mut crate::gssapi_h::gss_OID_set,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_acquire_cred_from: Some(
                crate::src::krb5::acquire_cred::krb5_gss_acquire_cred_from
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_OID_set,
                        _: crate::gssapi_h::gss_cred_usage_t,
                        _: crate::gssapi_ext_h::gss_const_key_value_set_t,
                        _: *mut crate::gssapi_h::gss_cred_id_t,
                        _: *mut crate::gssapi_h::gss_OID_set,
                        _: *mut crate::gssapi_h::OM_uint32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_store_cred_into: Some(
                crate::src::krb5::store_cred::krb5_gss_store_cred_into
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: crate::gssapi_h::gss_cred_usage_t,
                        _: crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_ext_h::gss_const_key_value_set_t,
                        _: *mut crate::gssapi_h::gss_OID_set,
                        _: *mut crate::gssapi_h::gss_cred_usage_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gssspi_acquire_cred_with_password: Some(
                crate::src::krb5::acquire_cred::krb5_gss_acquire_cred_with_password
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_OID_set,
                        _: i32,
                        _: *mut crate::gssapi_h::gss_cred_id_t,
                        _: *mut crate::gssapi_h::gss_OID_set,
                        _: *mut crate::gssapi_h::OM_uint32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_export_cred: Some(
                crate::src::krb5::export_cred::krb5_gss_export_cred
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_import_cred: Some(
                crate::src::krb5::import_cred::krb5_gss_import_cred
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_h::gss_cred_id_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gssspi_import_sec_context_by_mech: None,
            gssspi_import_name_by_mech: None,
            gssspi_import_cred_by_mech: None,
            gss_get_mic_iov: Some(
                crate::src::krb5::k5sealiov::krb5_gss_get_mic_iov
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_qop_t,
                        _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                        _: i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_verify_mic_iov: Some(
                crate::src::krb5::k5unsealiov::krb5_gss_verify_mic_iov
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: *mut crate::gssapi_h::gss_qop_t,
                        _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                        _: i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_get_mic_iov_length: Some(
                crate::src::krb5::k5sealiov::krb5_gss_get_mic_iov_length
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_qop_t,
                        _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                        _: i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gssspi_query_meta_data: None,
            gssspi_exchange_meta_data: None,
            gssspi_query_mechanism_info: None,
        };
        init
    }
};
/* Functions which use security contexts or acquire creds are IAKERB-specific;
 * other functions can borrow from the krb5 mech. */

static mut iakerb_mechanism: crate::mglueP_h::gss_config = {
    {
        let mut init = crate::mglueP_h::gss_config {
            mech_type: {
                let mut init = crate::gssapi_h::gss_OID_desc_struct {
                    length: 9u32,
                    elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x00" as *const u8
                        as *mut libc::c_void,
                };
                init
            },
            context: 0 as *mut libc::c_void,
            gss_acquire_cred: Some(
                crate::src::krb5::acquire_cred::iakerb_gss_acquire_cred
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_OID_set,
                        _: crate::gssapi_h::gss_cred_usage_t,
                        _: *mut crate::gssapi_h::gss_cred_id_t,
                        _: *mut crate::gssapi_h::gss_OID_set,
                        _: *mut crate::gssapi_h::OM_uint32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_release_cred: Some(
                crate::src::krb5::rel_cred::krb5_gss_release_cred
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_cred_id_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_init_sec_context: Some(
                crate::src::krb5::iakerb::iakerb_gss_init_sec_context
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: *mut crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_channel_bindings_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::OM_uint32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_accept_sec_context: Some(
                crate::src::krb5::iakerb::iakerb_gss_accept_sec_context
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_channel_bindings_t,
                        _: *mut crate::gssapi_h::gss_name_t,
                        _: *mut crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_cred_id_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_process_context_token: Some(
                crate::src::krb5::iakerb::iakerb_gss_process_context_token
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_delete_sec_context: Some(
                crate::src::krb5::iakerb::iakerb_gss_delete_sec_context
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_context_time: Some(
                crate::src::krb5::iakerb::iakerb_gss_context_time
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: *mut crate::gssapi_h::OM_uint32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_get_mic: Some(
                crate::src::krb5::iakerb::iakerb_gss_get_mic
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_qop_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_verify_mic: Some(
                crate::src::krb5::iakerb::iakerb_gss_verify_mic
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_h::gss_qop_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_wrap: Some(
                crate::src::krb5::iakerb::iakerb_gss_wrap
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: i32,
                        _: crate::gssapi_h::gss_qop_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut i32,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_unwrap: Some(
                crate::src::krb5::iakerb::iakerb_gss_unwrap
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut i32,
                        _: *mut crate::gssapi_h::gss_qop_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_display_status: Some(
                crate::src::krb5::disp_status::krb5_gss_display_status
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::OM_uint32,
                        _: i32,
                        _: crate::gssapi_h::gss_OID,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_indicate_mechs: Some(
                crate::src::krb5::indicate_mechs::krb5_gss_indicate_mechs
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_OID_set,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_compare_name: Some(
                crate::src::krb5::compare_name::krb5_gss_compare_name
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_name_t,
                        _: *mut i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_display_name: Some(
                crate::src::krb5::disp_name::krb5_gss_display_name
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_h::gss_OID,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_import_name: Some(
                crate::src::krb5::import_name::krb5_gss_import_name
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_OID,
                        _: *mut crate::gssapi_h::gss_name_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_release_name: Some(
                crate::src::krb5::rel_name::krb5_gss_release_name
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_name_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_inquire_cred: Some(
                crate::src::krb5::inq_cred::krb5_gss_inquire_cred
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: *mut crate::gssapi_h::gss_name_t,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_cred_usage_t,
                        _: *mut crate::gssapi_h::gss_OID_set,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_add_cred: None,
            gss_export_sec_context: Some(
                crate::src::krb5::iakerb::iakerb_gss_export_sec_context
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_import_sec_context: Some(
                crate::src::krb5::iakerb::iakerb_gss_import_sec_context
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_h::gss_ctx_id_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_inquire_cred_by_mech: Some(
                crate::src::krb5::inq_cred::krb5_gss_inquire_cred_by_mech
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: crate::gssapi_h::gss_OID,
                        _: *mut crate::gssapi_h::gss_name_t,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_cred_usage_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_inquire_names_for_mech: Some(
                crate::src::krb5::inq_names::krb5_gss_inquire_names_for_mech
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_OID,
                        _: *mut crate::gssapi_h::gss_OID_set,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_inquire_context: Some(
                crate::src::krb5::iakerb::iakerb_gss_inquire_context
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: *mut crate::gssapi_h::gss_name_t,
                        _: *mut crate::gssapi_h::gss_name_t,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_OID,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut i32,
                        _: *mut i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_internal_release_oid: Some(
                crate::src::krb5::rel_oid::krb5_gss_internal_release_oid
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_OID,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_wrap_size_limit: Some(
                crate::src::krb5::iakerb::iakerb_gss_wrap_size_limit
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: i32,
                        _: crate::gssapi_h::gss_qop_t,
                        _: crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::OM_uint32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_localname: Some(
                krb5_gss_localname
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_const_OID,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gssspi_authorize_localname: Some(
                krb5_gss_authorize_localname
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_const_buffer_t,
                        _: crate::gssapi_h::gss_const_OID,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_export_name: Some(
                crate::src::krb5::export_name::krb5_gss_export_name
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_duplicate_name: Some(
                crate::src::krb5::duplicate_name::krb5_gss_duplicate_name
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: *mut crate::gssapi_h::gss_name_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_store_cred: Some(
                crate::src::krb5::store_cred::krb5_gss_store_cred
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: crate::gssapi_h::gss_cred_usage_t,
                        _: crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_OID_set,
                        _: *mut crate::gssapi_h::gss_cred_usage_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_inquire_sec_context_by_oid: Some(
                crate::src::krb5::iakerb::iakerb_gss_inquire_sec_context_by_oid
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_OID,
                        _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_inquire_cred_by_oid: Some(
                krb5_gss_inquire_cred_by_oid
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: crate::gssapi_h::gss_OID,
                        _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_set_sec_context_option: Some(
                crate::src::krb5::iakerb::iakerb_gss_set_sec_context_option
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gssspi_set_cred_option: Some(
                krb5_gssspi_set_cred_option
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_cred_id_t,
                        _: crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gssspi_mech_invoke: Some(
                krb5_gssspi_mech_invoke
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_wrap_aead: None,
            gss_unwrap_aead: None,
            gss_wrap_iov: Some(
                crate::src::krb5::iakerb::iakerb_gss_wrap_iov
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: i32,
                        _: crate::gssapi_h::gss_qop_t,
                        _: *mut i32,
                        _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                        _: i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_unwrap_iov: Some(
                crate::src::krb5::iakerb::iakerb_gss_unwrap_iov
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: *mut i32,
                        _: *mut crate::gssapi_h::gss_qop_t,
                        _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                        _: i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_wrap_iov_length: Some(
                crate::src::krb5::iakerb::iakerb_gss_wrap_iov_length
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: i32,
                        _: crate::gssapi_h::gss_qop_t,
                        _: *mut i32,
                        _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                        _: i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_complete_auth_token: None,
            gss_acquire_cred_impersonate_name: None,
            gss_add_cred_impersonate_name: None,
            gss_display_name_ext: None,
            gss_inquire_name: Some(
                crate::src::krb5::naming_exts::krb5_gss_inquire_name
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: *mut i32,
                        _: *mut crate::gssapi_h::gss_OID,
                        _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_get_name_attribute: Some(
                crate::src::krb5::naming_exts::krb5_gss_get_name_attribute
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut i32,
                        _: *mut i32,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_set_name_attribute: Some(
                crate::src::krb5::naming_exts::krb5_gss_set_name_attribute
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: i32,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_delete_name_attribute: Some(
                crate::src::krb5::naming_exts::krb5_gss_delete_name_attribute
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_export_name_composite: Some(
                crate::src::krb5::naming_exts::krb5_gss_export_name_composite
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_map_name_to_any: Some(
                crate::src::krb5::naming_exts::krb5_gss_map_name_to_any
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: i32,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_ext_h::gss_any_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_release_any_name_mapping: Some(
                crate::src::krb5::naming_exts::krb5_gss_release_any_name_mapping
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_ext_h::gss_any_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_pseudo_random: Some(
                crate::src::krb5::iakerb::iakerb_gss_pseudo_random
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: i32,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::stdlib::ssize_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_set_neg_mechs: None,
            gss_inquire_saslname_for_mech: Some(
                krb5_gss_inquire_saslname_for_mech
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_inquire_mech_for_saslname: Some(
                krb5_gss_inquire_mech_for_saslname
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_h::gss_OID,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_inquire_attrs_for_mech: Some(
                krb5_gss_inquire_attrs_for_mech
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_const_OID,
                        _: *mut crate::gssapi_h::gss_OID_set,
                        _: *mut crate::gssapi_h::gss_OID_set,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_acquire_cred_from: Some(
                crate::src::krb5::acquire_cred::krb5_gss_acquire_cred_from
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_OID_set,
                        _: crate::gssapi_h::gss_cred_usage_t,
                        _: crate::gssapi_ext_h::gss_const_key_value_set_t,
                        _: *mut crate::gssapi_h::gss_cred_id_t,
                        _: *mut crate::gssapi_h::gss_OID_set,
                        _: *mut crate::gssapi_h::OM_uint32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_store_cred_into: Some(
                crate::src::krb5::store_cred::krb5_gss_store_cred_into
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: crate::gssapi_h::gss_cred_usage_t,
                        _: crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_ext_h::gss_const_key_value_set_t,
                        _: *mut crate::gssapi_h::gss_OID_set,
                        _: *mut crate::gssapi_h::gss_cred_usage_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gssspi_acquire_cred_with_password: Some(
                crate::src::krb5::acquire_cred::iakerb_gss_acquire_cred_with_password
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_OID_set,
                        _: i32,
                        _: *mut crate::gssapi_h::gss_cred_id_t,
                        _: *mut crate::gssapi_h::gss_OID_set,
                        _: *mut crate::gssapi_h::OM_uint32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_export_cred: Some(
                crate::src::krb5::export_cred::krb5_gss_export_cred
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_import_cred: Some(
                crate::src::krb5::import_cred::krb5_gss_import_cred
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_h::gss_cred_id_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gssspi_import_sec_context_by_mech: None,
            gssspi_import_name_by_mech: None,
            gssspi_import_cred_by_mech: None,
            gss_get_mic_iov: Some(
                crate::src::krb5::iakerb::iakerb_gss_get_mic_iov
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_qop_t,
                        _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                        _: i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_verify_mic_iov: Some(
                crate::src::krb5::iakerb::iakerb_gss_verify_mic_iov
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: *mut crate::gssapi_h::gss_qop_t,
                        _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                        _: i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_get_mic_iov_length: Some(
                crate::src::krb5::iakerb::iakerb_gss_get_mic_iov_length
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_qop_t,
                        _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                        _: i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gssspi_query_meta_data: None,
            gssspi_exchange_meta_data: None,
            gssspi_query_mechanism_info: None,
        };
        init
    }
};

unsafe extern "C" fn gss_iakerbmechglue_init() -> i32 {
    let mut mech_iakerb = crate::mglueP_h::gss_mech_config {
        kmodName: 0 as *mut i8,
        uLibName: 0 as *mut i8,
        mechNameStr: 0 as *mut i8,
        optionStr: 0 as *mut i8,
        dl_handle: 0 as *mut libc::c_void,
        mech_type: 0 as *mut crate::gssapi_h::gss_OID_desc_struct,
        mech: 0 as *mut crate::mglueP_h::gss_config,
        priority: 0,
        freeMech: 0,
        is_interposer: 0,
        int_mech_type: 0 as *mut crate::gssapi_h::gss_OID_desc_struct,
        int_mech: 0 as *mut crate::mglueP_h::gss_config,
        next: 0 as *mut crate::mglueP_h::gss_mech_config,
    };
    crate::stdlib::memset(
        &mut mech_iakerb as *mut crate::mglueP_h::gss_mech_config as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::mglueP_h::gss_mech_config>(),
    );
    mech_iakerb.mech = &mut iakerb_mechanism;
    mech_iakerb.mechNameStr = b"iakerb\x00" as *const u8 as *mut i8;
    mech_iakerb.mech_type = gss_mech_iakerb;
    crate::src::mechglue::g_initialize::gssint_register_mechinfo(&mut mech_iakerb);
    return 0i32;
}

unsafe extern "C" fn gss_krb5mechglue_init() -> i32 {
    let mut mech_krb5 = crate::mglueP_h::gss_mech_config {
        kmodName: 0 as *mut i8,
        uLibName: 0 as *mut i8,
        mechNameStr: 0 as *mut i8,
        optionStr: 0 as *mut i8,
        dl_handle: 0 as *mut libc::c_void,
        mech_type: 0 as *mut crate::gssapi_h::gss_OID_desc_struct,
        mech: 0 as *mut crate::mglueP_h::gss_config,
        priority: 0,
        freeMech: 0,
        is_interposer: 0,
        int_mech_type: 0 as *mut crate::gssapi_h::gss_OID_desc_struct,
        int_mech: 0 as *mut crate::mglueP_h::gss_config,
        next: 0 as *mut crate::mglueP_h::gss_mech_config,
    };
    crate::stdlib::memset(
        &mut mech_krb5 as *mut crate::mglueP_h::gss_mech_config as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::mglueP_h::gss_mech_config>(),
    );
    mech_krb5.mech = &mut krb5_mechanism;
    mech_krb5.mechNameStr = b"kerberos_v5\x00" as *const u8 as *mut i8;
    mech_krb5.mech_type = gss_mech_krb5;
    crate::src::mechglue::g_initialize::gssint_register_mechinfo(&mut mech_krb5);
    mech_krb5.mechNameStr = b"kerberos_v5_old\x00" as *const u8 as *mut i8;
    mech_krb5.mech_type = gss_mech_krb5_old;
    crate::src::mechglue::g_initialize::gssint_register_mechinfo(&mut mech_krb5);
    mech_krb5.mechNameStr = b"mskrb\x00" as *const u8 as *mut i8;
    mech_krb5.mech_type = gss_mech_krb5_wrong;
    crate::src::mechglue::g_initialize::gssint_register_mechinfo(&mut mech_krb5);
    return 0i32;
}
/* _GSS_STATIC_LINK */
#[no_mangle]

pub unsafe extern "C" fn gss_krb5int_lib_init() -> i32 {
    let mut err: i32 = 0;
    crate::com_err_h::add_error_table(&crate::src::krb5::gssapi_err_krb5::et_k5g_error_table);
    err = k5_mutex_finish_init(&mut crate::src::krb5::acquire_cred::gssint_krb5_keytab_lock);
    if err != 0 {
        return err;
    }
    /* LEAN_CLIENT */
    err = crate::k5_thread_h::krb5int_key_register(
        crate::k5_thread_h::K5_KEY_GSS_KRB5_SET_CCACHE_OLD_NAME,
        Some(crate::stdlib::free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
    );
    if err != 0 {
        return err;
    }
    err = crate::k5_thread_h::krb5int_key_register(
        crate::k5_thread_h::K5_KEY_GSS_KRB5_CCACHE_NAME,
        Some(crate::stdlib::free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
    );
    if err != 0 {
        return err;
    }
    err = crate::k5_thread_h::krb5int_key_register(
        crate::k5_thread_h::K5_KEY_GSS_KRB5_ERROR_MESSAGE,
        Some(
            crate::src::krb5::disp_status::krb5_gss_delete_error_info
                as unsafe extern "C" fn(_: *mut libc::c_void) -> (),
        ),
    );
    if err != 0 {
        return err;
    }
    err = k5_mutex_finish_init(&mut crate::src::krb5::init_sec_context::kg_kdc_flag_mutex);
    if err != 0 {
        return err;
    }
    err = k5_mutex_finish_init(&mut kg_vdb.mutex);
    if err != 0 {
        return err;
    }
    err = gss_krb5mechglue_init();
    if err != 0 {
        return err;
    }
    err = gss_iakerbmechglue_init();
    if err != 0 {
        return err;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn gss_krb5int_lib_fini() {
    crate::com_err_h::remove_error_table(&crate::src::krb5::gssapi_err_krb5::et_k5g_error_table);
    crate::k5_thread_h::krb5int_key_delete(crate::k5_thread_h::K5_KEY_GSS_KRB5_SET_CCACHE_OLD_NAME);
    crate::k5_thread_h::krb5int_key_delete(crate::k5_thread_h::K5_KEY_GSS_KRB5_CCACHE_NAME);
    crate::k5_thread_h::krb5int_key_delete(crate::k5_thread_h::K5_KEY_GSS_KRB5_ERROR_MESSAGE);
    crate::k5_thread_h::k5_os_mutex_destroy(&mut kg_vdb.mutex);
    crate::k5_thread_h::k5_os_mutex_destroy(
        &mut crate::src::krb5::init_sec_context::kg_kdc_flag_mutex,
    );
    crate::k5_thread_h::k5_os_mutex_destroy(
        &mut crate::src::krb5::acquire_cred::gssint_krb5_keytab_lock,
    );
    /* LEAN_CLIENT */
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
/*
 * These take unglued krb5-mech-specific contexts.
 */
/* _GSS_STATIC_LINK */
#[no_mangle]

pub unsafe extern "C" fn gss_krb5int_initialize_library() -> crate::gssapi_h::OM_uint32 {
    return crate::src::mechglue::g_initialize::gssint_mechglue_initialize_library()
        as crate::gssapi_h::OM_uint32;
}
unsafe extern "C" fn run_static_initializers() {
    gss_mech_krb5 = &mut *(krb5_gss_oid_array.as_ptr() as crate::gssapi_h::gss_OID).offset(0isize)
        as *mut crate::gssapi_h::gss_OID_desc_struct;
    gss_mech_krb5_old = &mut *(krb5_gss_oid_array.as_ptr() as crate::gssapi_h::gss_OID)
        .offset(1isize) as *mut crate::gssapi_h::gss_OID_desc_struct;
    gss_mech_krb5_wrong = &mut *(krb5_gss_oid_array.as_ptr() as crate::gssapi_h::gss_OID)
        .offset(2isize) as *mut crate::gssapi_h::gss_OID_desc_struct;
    gss_mech_iakerb = &mut *(krb5_gss_oid_array.as_ptr() as crate::gssapi_h::gss_OID).offset(3isize)
        as *mut crate::gssapi_h::gss_OID_desc_struct;
    gss_nt_krb5_name = &mut *(krb5_gss_oid_array.as_ptr() as crate::gssapi_h::gss_OID)
        .offset(5isize) as *mut crate::gssapi_h::gss_OID_desc_struct;
    gss_nt_krb5_principal = &mut *(krb5_gss_oid_array.as_ptr() as crate::gssapi_h::gss_OID)
        .offset(6isize) as *mut crate::gssapi_h::gss_OID_desc_struct;
    GSS_KRB5_NT_PRINCIPAL_NAME = &mut *(krb5_gss_oid_array.as_ptr() as crate::gssapi_h::gss_OID)
        .offset(5isize)
        as *mut crate::gssapi_h::gss_OID_desc_struct;
    GSS_KRB5_CRED_NO_CI_FLAGS_X = &mut *(krb5_gss_oid_array.as_ptr() as crate::gssapi_h::gss_OID)
        .offset(7isize)
        as *mut crate::gssapi_h::gss_OID_desc_struct;
    GSS_KRB5_GET_CRED_IMPERSONATOR = &mut *(krb5_gss_oid_array.as_ptr() as crate::gssapi_h::gss_OID)
        .offset(8isize)
        as *mut crate::gssapi_h::gss_OID_desc_struct;
    GSS_KRB5_NT_ENTERPRISE_NAME = &mut *(krb5_gss_oid_array.as_ptr() as crate::gssapi_h::gss_OID)
        .offset(9isize)
        as *mut crate::gssapi_h::gss_OID_desc_struct;
    oidsets = [
        {
            let mut init = crate::gssapi_h::gss_OID_set_desc_struct {
                count: 1usize,
                elements: &mut *(krb5_gss_oid_array.as_ptr() as crate::gssapi_h::gss_OID)
                    .offset(0isize)
                    as *mut crate::gssapi_h::gss_OID_desc_struct,
            };
            init
        },
        {
            let mut init = crate::gssapi_h::gss_OID_set_desc_struct {
                count: 1usize,
                elements: &mut *(krb5_gss_oid_array.as_ptr() as crate::gssapi_h::gss_OID)
                    .offset(1isize)
                    as *mut crate::gssapi_h::gss_OID_desc_struct,
            };
            init
        },
        {
            let mut init = crate::gssapi_h::gss_OID_set_desc_struct {
                count: 3usize,
                elements: &mut *(krb5_gss_oid_array.as_ptr() as crate::gssapi_h::gss_OID)
                    .offset(0isize)
                    as *mut crate::gssapi_h::gss_OID_desc_struct,
            };
            init
        },
        {
            let mut init = crate::gssapi_h::gss_OID_set_desc_struct {
                count: 4usize,
                elements: &mut *(krb5_gss_oid_array.as_ptr() as crate::gssapi_h::gss_OID)
                    .offset(0isize)
                    as *mut crate::gssapi_h::gss_OID_desc_struct,
            };
            init
        },
    ];
    gss_mech_set_krb5 = &mut *(oidsets.as_ptr() as crate::gssapi_h::gss_OID_set).offset(0isize)
        as *mut crate::gssapi_h::gss_OID_set_desc_struct;
    gss_mech_set_krb5_old = &mut *(oidsets.as_ptr() as crate::gssapi_h::gss_OID_set).offset(1isize)
        as *mut crate::gssapi_h::gss_OID_set_desc_struct;
    gss_mech_set_krb5_both = &mut *(oidsets.as_ptr() as crate::gssapi_h::gss_OID_set).offset(2isize)
        as *mut crate::gssapi_h::gss_OID_set_desc_struct;
    kg_all_mechs = &mut *(oidsets.as_ptr() as crate::gssapi_h::gss_OID_set).offset(3isize)
        as *mut crate::gssapi_h::gss_OID_set_desc_struct
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
