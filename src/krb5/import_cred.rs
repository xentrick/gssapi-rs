use ::libc;

pub mod k5_thread_h {

    #[inline]

    pub unsafe extern "C" fn k5_mutex_init(mut m: *mut crate::k5_thread_h::k5_mutex_t) -> i32 {
        return crate::k5_thread_h::k5_os_mutex_init(m);
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

pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__pthread_internal_list;
pub use crate::stdlib::__pthread_list_t;
pub use crate::stdlib::__pthread_mutex_s;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::pthread_mutex_t;

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
pub use crate::k5_int_h::k5_rc_resolve;
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
pub use crate::k5_thread_h::k5_os_mutex_init;
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
pub use crate::krb5_h::krb5_cc_destroy;
pub use crate::krb5_h::krb5_cc_initialize;
pub use crate::krb5_h::krb5_cc_new_unique;
pub use crate::krb5_h::krb5_cc_resolve;
pub use crate::krb5_h::krb5_cc_store_cred;
pub use crate::krb5_h::krb5_ccache;
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
pub use crate::krb5_h::krb5_free_addresses;
pub use crate::krb5_h::krb5_free_authdata;
pub use crate::krb5_h::krb5_free_context;
pub use crate::krb5_h::krb5_free_cred_contents;
pub use crate::krb5_h::krb5_free_principal;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_keyblock;
pub use crate::krb5_h::krb5_keytab;
pub use crate::krb5_h::krb5_keytab_entry;
pub use crate::krb5_h::krb5_keytab_entry_st;
pub use crate::krb5_h::krb5_kt_cursor;
pub use crate::krb5_h::krb5_kt_resolve;
pub use crate::krb5_h::krb5_kvno;
pub use crate::krb5_h::krb5_magic;
pub use crate::krb5_h::krb5_octet;
pub use crate::krb5_h::krb5_parse_name;
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
pub use crate::src::krb5::import_cred::k5_int_h::k5alloc;
pub use crate::src::krb5::import_cred::k5_int_h::k5calloc;
pub use crate::src::krb5::import_cred::k5_int_h::k5memdup0;
pub use crate::src::krb5::import_cred::k5_thread_h::k5_mutex_init;
pub use crate::stdlib::uint32_t;
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
pub use crate::gssapiP_krb5_h::_krb5_gss_cred_id_rec;
pub use crate::gssapiP_krb5_h::_krb5_gss_name_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
pub use crate::gssapiP_krb5_h::krb5_gss_name_t;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_cred_id_t;
pub use crate::gssapi_h::gss_cred_usage_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::k5_json_h::k5_json_array;
pub use crate::k5_json_h::k5_json_array_get;
pub use crate::k5_json_h::k5_json_array_length;
pub use crate::k5_json_h::k5_json_array_st;
pub use crate::k5_json_h::k5_json_bool;
pub use crate::k5_json_h::k5_json_bool_st;
pub use crate::k5_json_h::k5_json_bool_value;
pub use crate::k5_json_h::k5_json_decode;
pub use crate::k5_json_h::k5_json_get_tid;
pub use crate::k5_json_h::k5_json_number;
pub use crate::k5_json_h::k5_json_number_st;
pub use crate::k5_json_h::k5_json_number_value;
pub use crate::k5_json_h::k5_json_release;
pub use crate::k5_json_h::k5_json_string;
pub use crate::k5_json_h::k5_json_string_st;
pub use crate::k5_json_h::k5_json_string_unbase64;
pub use crate::k5_json_h::k5_json_string_utf8;
pub use crate::k5_json_h::k5_json_tid;
pub use crate::k5_json_h::k5_json_value;
pub use crate::mglueP_h::gss_cred_id_struct;
pub use crate::src::krb5::init_sec_context::krb5_gss_init_context;
pub use crate::src::krb5::naming_exts::kg_release_name;
pub use crate::src::krb5::rel_cred::krb5_gss_release_cred;

/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/gssapi/krb5/import_cred.c - krb5 import_cred implementation */
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
/* Return the idx element of array if it is of type tid; otherwise return
 * NULL.  The caller is responsible for checking the array length. */

unsafe extern "C" fn check_element(
    mut array: crate::k5_json_h::k5_json_array,
    mut idx: crate::stddef_h::size_t,
    mut tid: crate::k5_json_h::k5_json_tid,
) -> crate::k5_json_h::k5_json_value {
    let mut v = 0 as *mut libc::c_void;
    v = crate::k5_json_h::k5_json_array_get(array, idx);
    return if crate::k5_json_h::k5_json_get_tid(v) == tid {
        v
    } else {
        0 as *mut libc::c_void
    };
}
/* All of the json_to_x functions return 0 on success, -1 on failure (either
 * from running out of memory or from defective input). */
/* Convert a JSON value to a C string or to NULL. */

unsafe extern "C" fn json_to_optional_string(
    mut v: crate::k5_json_h::k5_json_value,
    mut string_out: *mut *mut i8,
) -> i32 {
    *string_out = 0 as *mut i8;
    if crate::k5_json_h::k5_json_get_tid(v) == 1u32 {
        return 0i32;
    }
    if crate::k5_json_h::k5_json_get_tid(v) != 131u32 {
        return -(1i32);
    }
    *string_out = crate::stdlib::strdup(crate::k5_json_h::k5_json_string_utf8(
        v as crate::k5_json_h::k5_json_string,
    ));
    return if (*string_out).is_null() {
        -(1i32)
    } else {
        0i32
    };
}
/* Convert a JSON value to a principal or to NULL. */

unsafe extern "C" fn json_to_principal(
    mut context: crate::krb5_h::krb5_context,
    mut v: crate::k5_json_h::k5_json_value,
    mut princ_out: *mut crate::krb5_h::krb5_principal,
) -> i32 {
    *princ_out = 0 as crate::krb5_h::krb5_principal;
    if crate::k5_json_h::k5_json_get_tid(v) == 1u32 {
        return 0i32;
    }
    if crate::k5_json_h::k5_json_get_tid(v) != 131u32 {
        return -(1i32);
    }
    if crate::krb5_h::krb5_parse_name(
        context,
        crate::k5_json_h::k5_json_string_utf8(v as crate::k5_json_h::k5_json_string),
        princ_out,
    ) != 0
    {
        return -(1i32);
    }
    return 0i32;
}
/* Convert a JSON value to a zero-terminated enctypes list or to NULL. */

unsafe extern "C" fn json_to_etypes(
    mut v: crate::k5_json_h::k5_json_value,
    mut etypes_out: *mut *mut crate::krb5_h::krb5_enctype,
) -> i32 {
    let mut current_block: u64;
    let mut etypes = 0 as *mut crate::krb5_h::krb5_enctype;
    let mut array = 0 as *mut crate::k5_json_h::k5_json_array_st;
    let mut n = 0 as *mut crate::k5_json_h::k5_json_number_st;
    let mut len: crate::stddef_h::size_t = 0;
    let mut i: crate::stddef_h::size_t = 0;
    *etypes_out = 0 as *mut crate::krb5_h::krb5_enctype;
    if crate::k5_json_h::k5_json_get_tid(v) == 1u32 {
        return 0i32;
    }
    if crate::k5_json_h::k5_json_get_tid(v) != 129u32 {
        return -(1i32);
    }
    array = v as crate::k5_json_h::k5_json_array;
    len = crate::k5_json_h::k5_json_array_length(array);
    etypes = crate::stdlib::calloc(
        len.wrapping_add(1usize),
        ::std::mem::size_of::<crate::krb5_h::krb5_enctype>(),
    ) as *mut crate::krb5_h::krb5_enctype;
    i = 0usize;
    loop {
        if !(i < len) {
            current_block = 12599329904712511516;
            break;
        }
        n = check_element(array, i, 0u32) as crate::k5_json_h::k5_json_number;
        if n.is_null() {
            current_block = 2180654997443379353;
            break;
        }
        *etypes.offset(i as isize) =
            crate::k5_json_h::k5_json_number_value(n) as crate::krb5_h::krb5_enctype;
        i = i.wrapping_add(1)
    }
    match current_block {
        2180654997443379353 => {
            crate::stdlib::free(etypes as *mut libc::c_void);
            return -(1i32);
        }
        _ => {
            *etypes_out = etypes;
            return 0i32;
        }
    };
}
/* Convert a JSON value to a krb5 GSS name or to NULL. */

unsafe extern "C" fn json_to_kgname(
    mut context: crate::krb5_h::krb5_context,
    mut v: crate::k5_json_h::k5_json_value,
    mut name_out: *mut crate::gssapiP_krb5_h::krb5_gss_name_t,
) -> i32 {
    let mut array = 0 as *mut crate::k5_json_h::k5_json_array_st;
    let mut name = 0 as crate::gssapiP_krb5_h::krb5_gss_name_t;
    *name_out = 0 as crate::gssapiP_krb5_h::krb5_gss_name_t;
    if crate::k5_json_h::k5_json_get_tid(v) == 1u32 {
        return 0i32;
    }
    if crate::k5_json_h::k5_json_get_tid(v) != 129u32 {
        return -(1i32);
    }
    array = v as crate::k5_json_h::k5_json_array;
    if crate::k5_json_h::k5_json_array_length(array) != 3usize {
        return -(1i32);
    }
    name = crate::stdlib::calloc(
        1usize,
        ::std::mem::size_of::<crate::gssapiP_krb5_h::_krb5_gss_name_rec>(),
    ) as crate::gssapiP_krb5_h::krb5_gss_name_t;
    if name.is_null() {
        return -(1i32);
    }
    if k5_mutex_init(&mut (*name).lock) != 0 {
        crate::stdlib::free(name as *mut libc::c_void);
        return -(1i32);
    }
    if !(json_to_principal(
        context,
        crate::k5_json_h::k5_json_array_get(array, 0usize),
        &mut (*name).princ,
    ) != 0)
    {
        if !(json_to_optional_string(
            crate::k5_json_h::k5_json_array_get(array, 1usize),
            &mut (*name).service,
        ) != 0)
        {
            if !(json_to_optional_string(
                crate::k5_json_h::k5_json_array_get(array, 2usize),
                &mut (*name).host,
            ) != 0)
            {
                *name_out = name;
                return 0i32;
            }
        }
    }
    crate::src::krb5::naming_exts::kg_release_name(context, &mut name);
    return -(1i32);
}
/* Convert a JSON value to a keytab handle or to NULL. */

unsafe extern "C" fn json_to_keytab(
    mut context: crate::krb5_h::krb5_context,
    mut v: crate::k5_json_h::k5_json_value,
    mut keytab_out: *mut crate::krb5_h::krb5_keytab,
) -> i32 {
    *keytab_out = 0 as crate::krb5_h::krb5_keytab;
    if crate::k5_json_h::k5_json_get_tid(v) == 1u32 {
        return 0i32;
    }
    if crate::k5_json_h::k5_json_get_tid(v) != 131u32 {
        return -(1i32);
    }
    if crate::krb5_h::krb5_kt_resolve(
        context,
        crate::k5_json_h::k5_json_string_utf8(v as crate::k5_json_h::k5_json_string),
        keytab_out,
    ) != 0
    {
        return -(1i32);
    }
    return 0i32;
}
/* Convert a JSON value to an rcache handle or to NULL. */

unsafe extern "C" fn json_to_rcache(
    mut context: crate::krb5_h::krb5_context,
    mut v: crate::k5_json_h::k5_json_value,
    mut rcache_out: *mut crate::krb5_h::krb5_rcache,
) -> i32 {
    let mut rcache = 0 as *mut crate::krb5_h::krb5_rc_st;
    *rcache_out = 0 as crate::krb5_h::krb5_rcache;
    if crate::k5_json_h::k5_json_get_tid(v) == 1u32 {
        return 0i32;
    }
    if crate::k5_json_h::k5_json_get_tid(v) != 131u32 {
        return -(1i32);
    }
    if crate::k5_int_h::k5_rc_resolve(
        context,
        crate::k5_json_h::k5_json_string_utf8(v as crate::k5_json_h::k5_json_string) as *mut i8,
        &mut rcache,
    ) != 0
    {
        return -(1i32);
    }
    *rcache_out = rcache;
    return 0i32;
}
/* Convert a JSON value to a keyblock, filling in keyblock. */

unsafe extern "C" fn json_to_keyblock(
    mut v: crate::k5_json_h::k5_json_value,
    mut keyblock: *mut crate::krb5_h::krb5_keyblock,
) -> i32 {
    let mut array = 0 as *mut crate::k5_json_h::k5_json_array_st;
    let mut n = 0 as *mut crate::k5_json_h::k5_json_number_st;
    let mut s = 0 as *mut crate::k5_json_h::k5_json_string_st;
    let mut len: crate::stddef_h::size_t = 0;
    crate::stdlib::memset(
        keyblock as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::krb5_h::krb5_keyblock>(),
    );
    if crate::k5_json_h::k5_json_get_tid(v) != 129u32 {
        return -(1i32);
    }
    array = v as crate::k5_json_h::k5_json_array;
    if crate::k5_json_h::k5_json_array_length(array) != 2usize {
        return -(1i32);
    }
    n = check_element(array, 0usize, 0u32) as crate::k5_json_h::k5_json_number;
    if n.is_null() {
        return -(1i32);
    }
    (*keyblock).enctype = crate::k5_json_h::k5_json_number_value(n) as crate::krb5_h::krb5_enctype;
    s = check_element(array, 1usize, 131u32) as crate::k5_json_h::k5_json_string;
    if s.is_null() {
        return -(1i32);
    }
    if crate::k5_json_h::k5_json_string_unbase64(s, &mut (*keyblock).contents, &mut len) != 0 {
        return -(1i32);
    }
    (*keyblock).length = len as u32;
    (*keyblock).magic = -(1760647421 as isize) as crate::krb5_h::krb5_magic;
    return 0i32;
}
/* Convert a JSON value to a krb5 address. */

unsafe extern "C" fn json_to_address(
    mut v: crate::k5_json_h::k5_json_value,
    mut addr_out: *mut *mut crate::krb5_h::krb5_address,
) -> i32 {
    let mut array = 0 as *mut crate::k5_json_h::k5_json_array_st;
    let mut addr = 0 as *mut crate::krb5_h::krb5_address;
    let mut n = 0 as *mut crate::k5_json_h::k5_json_number_st;
    let mut s = 0 as *mut crate::k5_json_h::k5_json_string_st;
    let mut len: crate::stddef_h::size_t = 0;
    *addr_out = 0 as *mut crate::krb5_h::krb5_address;
    if crate::k5_json_h::k5_json_get_tid(v) != 129u32 {
        return -(1i32);
    }
    array = v as crate::k5_json_h::k5_json_array;
    if crate::k5_json_h::k5_json_array_length(array) != 2usize {
        return -(1i32);
    }
    n = check_element(array, 0usize, 0u32) as crate::k5_json_h::k5_json_number;
    if n.is_null() {
        return -(1i32);
    }
    s = check_element(array, 1usize, 131u32) as crate::k5_json_h::k5_json_string;
    if s.is_null() {
        return -(1i32);
    }
    addr = crate::stdlib::malloc(::std::mem::size_of::<crate::krb5_h::krb5_address>())
        as *mut crate::krb5_h::krb5_address;
    if addr.is_null() {
        return -(1i32);
    }
    (*addr).addrtype = crate::k5_json_h::k5_json_number_value(n) as crate::krb5_h::krb5_addrtype;
    if crate::k5_json_h::k5_json_string_unbase64(s, &mut (*addr).contents, &mut len) != 0 {
        crate::stdlib::free(addr as *mut libc::c_void);
        return -(1i32);
    }
    (*addr).length = len as u32;
    (*addr).magic = -(1760647390 as isize) as crate::krb5_h::krb5_magic;
    *addr_out = addr;
    return 0i32;
}
/* Convert a JSON value to a null-terminated list of krb5 addresses or to
 * NULL. */

unsafe extern "C" fn json_to_addresses(
    mut context: crate::krb5_h::krb5_context,
    mut v: crate::k5_json_h::k5_json_value,
    mut addresses_out: *mut *mut *mut crate::krb5_h::krb5_address,
) -> i32 {
    let mut current_block: u64;
    let mut array = 0 as *mut crate::k5_json_h::k5_json_array_st;
    let mut addrs = 0 as *mut *mut crate::krb5_h::krb5_address;
    let mut len: crate::stddef_h::size_t = 0;
    let mut i: crate::stddef_h::size_t = 0;
    *addresses_out = 0 as *mut *mut crate::krb5_h::krb5_address;
    if crate::k5_json_h::k5_json_get_tid(v) == 1u32 {
        return 0i32;
    }
    if crate::k5_json_h::k5_json_get_tid(v) != 129u32 {
        return -(1i32);
    }
    array = v as crate::k5_json_h::k5_json_array;
    len = crate::k5_json_h::k5_json_array_length(array);
    addrs = crate::stdlib::calloc(
        len.wrapping_add(1usize),
        ::std::mem::size_of::<*mut crate::krb5_h::krb5_address>(),
    ) as *mut *mut crate::krb5_h::krb5_address;
    i = 0usize;
    loop {
        if !(i < len) {
            current_block = 7651349459974463963;
            break;
        }
        if json_to_address(
            crate::k5_json_h::k5_json_array_get(array, i),
            &mut *addrs.offset(i as isize),
        ) != 0
        {
            current_block = 14641784374299874002;
            break;
        }
        i = i.wrapping_add(1)
    }
    match current_block {
        14641784374299874002 => {
            crate::krb5_h::krb5_free_addresses(context, addrs);
            return -(1i32);
        }
        _ => {
            let ref mut fresh0 = *addrs.offset(i as isize);
            *fresh0 = 0 as *mut crate::krb5_h::krb5_address;
            *addresses_out = addrs;
            return 0i32;
        }
    };
}
/* Convert a JSON value to an authdata element. */

unsafe extern "C" fn json_to_authdata_element(
    mut v: crate::k5_json_h::k5_json_value,
    mut ad_out: *mut *mut crate::krb5_h::krb5_authdata,
) -> i32 {
    let mut array = 0 as *mut crate::k5_json_h::k5_json_array_st;
    let mut ad = 0 as *mut crate::krb5_h::krb5_authdata;
    let mut n = 0 as *mut crate::k5_json_h::k5_json_number_st;
    let mut s = 0 as *mut crate::k5_json_h::k5_json_string_st;
    let mut len: crate::stddef_h::size_t = 0;
    *ad_out = 0 as *mut crate::krb5_h::krb5_authdata;
    if crate::k5_json_h::k5_json_get_tid(v) != 129u32 {
        return -(1i32);
    }
    array = v as crate::k5_json_h::k5_json_array;
    if crate::k5_json_h::k5_json_array_length(array) != 2usize {
        return -(1i32);
    }
    n = check_element(array, 0usize, 0u32) as crate::k5_json_h::k5_json_number;
    if n.is_null() {
        return -(1i32);
    }
    s = check_element(array, 1usize, 131u32) as crate::k5_json_h::k5_json_string;
    if s.is_null() {
        return -(1i32);
    }
    ad = crate::stdlib::malloc(::std::mem::size_of::<crate::krb5_h::krb5_authdata>())
        as *mut crate::krb5_h::krb5_authdata;
    if ad.is_null() {
        return -(1i32);
    }
    (*ad).ad_type = crate::k5_json_h::k5_json_number_value(n) as crate::krb5_h::krb5_authdatatype;
    if crate::k5_json_h::k5_json_string_unbase64(s, &mut (*ad).contents, &mut len) != 0 {
        crate::stdlib::free(ad as *mut libc::c_void);
        return -(1i32);
    }
    (*ad).length = len as u32;
    (*ad).magic = -(1760647414 as isize) as crate::krb5_h::krb5_magic;
    *ad_out = ad;
    return 0i32;
}
/* Convert a JSON value to a null-terminated authdata list or to NULL. */

unsafe extern "C" fn json_to_authdata(
    mut context: crate::krb5_h::krb5_context,
    mut v: crate::k5_json_h::k5_json_value,
    mut authdata_out: *mut *mut *mut crate::krb5_h::krb5_authdata,
) -> i32 {
    let mut current_block: u64;
    let mut array = 0 as *mut crate::k5_json_h::k5_json_array_st;
    let mut authdata = 0 as *mut *mut crate::krb5_h::krb5_authdata;
    let mut len: crate::stddef_h::size_t = 0;
    let mut i: crate::stddef_h::size_t = 0;
    *authdata_out = 0 as *mut *mut crate::krb5_h::krb5_authdata;
    if crate::k5_json_h::k5_json_get_tid(v) == 1u32 {
        return 0i32;
    }
    if crate::k5_json_h::k5_json_get_tid(v) != 129u32 {
        return -(1i32);
    }
    array = v as crate::k5_json_h::k5_json_array;
    len = crate::k5_json_h::k5_json_array_length(array);
    authdata = crate::stdlib::calloc(
        len.wrapping_add(1usize),
        ::std::mem::size_of::<*mut crate::krb5_h::krb5_authdata>(),
    ) as *mut *mut crate::krb5_h::krb5_authdata;
    i = 0usize;
    loop {
        if !(i < len) {
            current_block = 7651349459974463963;
            break;
        }
        if json_to_authdata_element(
            crate::k5_json_h::k5_json_array_get(array, i),
            &mut *authdata.offset(i as isize),
        ) != 0
        {
            current_block = 1764916337490345069;
            break;
        }
        i = i.wrapping_add(1)
    }
    match current_block {
        1764916337490345069 => {
            crate::krb5_h::krb5_free_authdata(context, authdata);
            return -(1i32);
        }
        _ => {
            let ref mut fresh1 = *authdata.offset(i as isize);
            *fresh1 = 0 as *mut crate::krb5_h::krb5_authdata;
            *authdata_out = authdata;
            return 0i32;
        }
    };
}
/* Convert a JSON value to a krb5 credential structure, filling in creds. */

unsafe extern "C" fn json_to_creds(
    mut context: crate::krb5_h::krb5_context,
    mut v: crate::k5_json_h::k5_json_value,
    mut creds: *mut crate::krb5_h::krb5_creds,
) -> i32 {
    let mut array = 0 as *mut crate::k5_json_h::k5_json_array_st;
    let mut n = 0 as *mut crate::k5_json_h::k5_json_number_st;
    let mut b = 0 as *mut crate::k5_json_h::k5_json_bool_st;
    let mut s = 0 as *mut crate::k5_json_h::k5_json_string_st;
    let mut data = 0 as *mut u8;
    let mut len: crate::stddef_h::size_t = 0;
    crate::stdlib::memset(
        creds as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::krb5_h::krb5_creds>(),
    );
    if crate::k5_json_h::k5_json_get_tid(v) != 129u32 {
        return -(1i32);
    }
    array = v as crate::k5_json_h::k5_json_array;
    if crate::k5_json_h::k5_json_array_length(array) != 13usize {
        return -(1i32);
    }
    if !(json_to_principal(
        context,
        crate::k5_json_h::k5_json_array_get(array, 0usize),
        &mut (*creds).client,
    ) != 0)
    {
        if !(json_to_principal(
            context,
            crate::k5_json_h::k5_json_array_get(array, 1usize),
            &mut (*creds).server,
        ) != 0)
        {
            if !(json_to_keyblock(
                crate::k5_json_h::k5_json_array_get(array, 2usize),
                &mut (*creds).keyblock,
            ) != 0)
            {
                n = check_element(array, 3usize, 0u32) as crate::k5_json_h::k5_json_number;
                if !n.is_null() {
                    (*creds).times.authtime =
                        crate::k5_json_h::k5_json_number_value(n) as crate::krb5_h::krb5_timestamp;
                    n = check_element(array, 4usize, 0u32) as crate::k5_json_h::k5_json_number;
                    if !n.is_null() {
                        (*creds).times.starttime = crate::k5_json_h::k5_json_number_value(n)
                            as crate::krb5_h::krb5_timestamp;
                        n = check_element(array, 5usize, 0u32) as crate::k5_json_h::k5_json_number;
                        if !n.is_null() {
                            (*creds).times.endtime = crate::k5_json_h::k5_json_number_value(n)
                                as crate::krb5_h::krb5_timestamp;
                            n = check_element(array, 6usize, 0u32)
                                as crate::k5_json_h::k5_json_number;
                            if !n.is_null() {
                                (*creds).times.renew_till =
                                    crate::k5_json_h::k5_json_number_value(n)
                                        as crate::krb5_h::krb5_timestamp;
                                b = check_element(array, 7usize, 2u32)
                                    as crate::k5_json_h::k5_json_bool;
                                if !b.is_null() {
                                    (*creds).is_skey = crate::k5_json_h::k5_json_bool_value(b)
                                        as crate::krb5_h::krb5_boolean;
                                    n = check_element(array, 8usize, 0u32)
                                        as crate::k5_json_h::k5_json_number;
                                    if !n.is_null() {
                                        (*creds).ticket_flags =
                                            crate::k5_json_h::k5_json_number_value(n)
                                                as crate::krb5_h::krb5_flags;
                                        if !(json_to_addresses(
                                            context,
                                            crate::k5_json_h::k5_json_array_get(array, 9usize),
                                            &mut (*creds).addresses,
                                        ) != 0)
                                        {
                                            s = check_element(array, 10usize, 131u32)
                                                as crate::k5_json_h::k5_json_string;
                                            if !s.is_null() {
                                                if !(crate::k5_json_h::k5_json_string_unbase64(
                                                    s, &mut data, &mut len,
                                                ) != 0)
                                                {
                                                    (*creds).ticket.data = data as *mut i8;
                                                    (*creds).ticket.length = len as u32;
                                                    s = check_element(array, 11usize, 131u32)
                                                        as crate::k5_json_h::k5_json_string;
                                                    if !s.is_null() {
                                                        if !(crate::k5_json_h::k5_json_string_unbase64(s,
                                                                                     &mut data,
                                                                                     &mut len)
                                                                 != 0) {
                                                            (*creds).second_ticket.data
                                                                =
                                                                data as
                                                                    *mut i8;
                                                            (*creds).second_ticket.length
                                                                =
                                                                len as
                                                                    u32;
                                                            if !(json_to_authdata(context,
                                                                                  crate::k5_json_h::k5_json_array_get(array,
                                                                                                    12usize),
                                                                                  &mut (*creds).authdata)
                                                                     != 0) {
                                                                (*creds).magic
                                                                    =
                                                                    -(1760647408
                                                                          as
                                                                          isize)
                                                                        as
                                                                        crate::krb5_h::krb5_magic;
                                                                return 0i32
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
    crate::krb5_h::krb5_free_cred_contents(context, creds);
    crate::stdlib::memset(
        creds as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::krb5_h::krb5_creds>(),
    );
    return -(1i32);
}
/* Convert a JSON value to a ccache handle or to NULL.  Set *new_out to true if
 * the ccache handle is a newly created memory ccache, false otherwise. */

unsafe extern "C" fn json_to_ccache(
    mut context: crate::krb5_h::krb5_context,
    mut v: crate::k5_json_h::k5_json_value,
    mut ccache_out: *mut crate::krb5_h::krb5_ccache,
    mut new_out: *mut crate::krb5_h::krb5_boolean,
) -> i32 {
    let mut current_block: u64;
    let mut ret: crate::krb5_h::krb5_error_code = 0;
    let mut ccache = 0 as crate::krb5_h::krb5_ccache;
    let mut princ = 0 as *mut crate::krb5_h::krb5_principal_data;
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
    let mut array = 0 as *mut crate::k5_json_h::k5_json_array_st;
    let mut i: crate::stddef_h::size_t = 0;
    let mut len: crate::stddef_h::size_t = 0;
    *ccache_out = 0 as crate::krb5_h::krb5_ccache;
    *new_out = 0u32;
    if crate::k5_json_h::k5_json_get_tid(v) == 1u32 {
        return 0i32;
    }
    if crate::k5_json_h::k5_json_get_tid(v) == 131u32 {
        /* We got a reference to an external ccache; just resolve it. */
        return if crate::krb5_h::krb5_cc_resolve(
            context,
            crate::k5_json_h::k5_json_string_utf8(v as crate::k5_json_h::k5_json_string),
            ccache_out,
        ) != 0
        {
            -(1i32)
        } else {
            0i32
        };
    }
    /* We got the contents of a memory ccache. */
    if crate::k5_json_h::k5_json_get_tid(v) != 129u32 {
        return -(1i32);
    }
    array = v as crate::k5_json_h::k5_json_array;
    len = crate::k5_json_h::k5_json_array_length(array);
    if len < 1usize {
        return -(1i32);
    }
    /* Initialize a new memory ccache using the principal in the first array
     * entry.*/
    if crate::krb5_h::krb5_cc_new_unique(
        context,
        b"MEMORY\x00" as *const u8 as *const i8,
        0 as *const i8,
        &mut ccache,
    ) != 0
    {
        return -(1i32);
    }
    if !(json_to_principal(
        context,
        crate::k5_json_h::k5_json_array_get(array, 0usize),
        &mut princ,
    ) != 0)
    {
        ret = crate::krb5_h::krb5_cc_initialize(context, ccache, princ);
        crate::krb5_h::krb5_free_principal(context, princ);
        if !(ret != 0) {
            /* Add remaining array entries to the ccache as credentials. */
            i = 1usize;
            loop {
                if !(i < len) {
                    current_block = 2232869372362427478;
                    break;
                }
                if json_to_creds(
                    context,
                    crate::k5_json_h::k5_json_array_get(array, i),
                    &mut creds,
                ) != 0
                {
                    current_block = 5281045310412464552;
                    break;
                }
                ret = crate::krb5_h::krb5_cc_store_cred(context, ccache, &mut creds);
                crate::krb5_h::krb5_free_cred_contents(context, &mut creds);
                if ret != 0 {
                    current_block = 5281045310412464552;
                    break;
                }
                i = i.wrapping_add(1)
            }
            match current_block {
                5281045310412464552 => {}
                _ => {
                    *ccache_out = ccache;
                    *new_out = 1u32;
                    return 0i32;
                }
            }
        }
    }
    crate::krb5_h::krb5_cc_destroy(context, ccache);
    return -(1i32);
}
/* Convert a JSON array value to a krb5 GSS credential. */

unsafe extern "C" fn json_to_kgcred(
    mut context: crate::krb5_h::krb5_context,
    mut array: crate::k5_json_h::k5_json_array,
    mut cred_out: *mut crate::gssapiP_krb5_h::krb5_gss_cred_id_t,
) -> i32 {
    let mut cred = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_cred_id_rec;
    let mut n = 0 as *mut crate::k5_json_h::k5_json_number_st;
    let mut b = 0 as *mut crate::k5_json_h::k5_json_bool_st;
    let mut is_new: crate::krb5_h::krb5_boolean = 0;
    let mut tmp: crate::gssapi_h::OM_uint32 = 0;
    *cred_out = 0 as crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
    if crate::k5_json_h::k5_json_array_length(array) != 14usize {
        return -(1i32);
    }
    cred = crate::stdlib::calloc(
        1usize,
        ::std::mem::size_of::<crate::gssapiP_krb5_h::_krb5_gss_cred_id_rec>(),
    ) as crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
    if cred.is_null() {
        return -(1i32);
    }
    if k5_mutex_init(&mut (*cred).lock) != 0 {
        crate::stdlib::free(cred as *mut libc::c_void);
        return -(1i32);
    }
    n = check_element(array, 0usize, 0u32) as crate::k5_json_h::k5_json_number;
    if !n.is_null() {
        (*cred).usage =
            crate::k5_json_h::k5_json_number_value(n) as crate::gssapi_h::gss_cred_usage_t;
        if !(json_to_kgname(
            context,
            crate::k5_json_h::k5_json_array_get(array, 1usize),
            &mut (*cred).name,
        ) != 0)
        {
            if !(json_to_principal(
                context,
                crate::k5_json_h::k5_json_array_get(array, 2usize),
                &mut (*cred).impersonator,
            ) != 0)
            {
                b = check_element(array, 3usize, 2u32) as crate::k5_json_h::k5_json_bool;
                if !b.is_null() {
                    (*cred).set_default_identity(crate::k5_json_h::k5_json_bool_value(b) as u32);
                    b = check_element(array, 4usize, 2u32) as crate::k5_json_h::k5_json_bool;
                    if !b.is_null() {
                        (*cred).set_iakerb_mech(crate::k5_json_h::k5_json_bool_value(b) as u32);
                        if !(json_to_keytab(
                            context,
                            crate::k5_json_h::k5_json_array_get(array, 5usize),
                            &mut (*cred).keytab,
                        ) != 0)
                        {
                            if !(json_to_rcache(
                                context,
                                crate::k5_json_h::k5_json_array_get(array, 6usize),
                                &mut (*cred).rcache,
                            ) != 0)
                            {
                                if !(json_to_ccache(
                                    context,
                                    crate::k5_json_h::k5_json_array_get(array, 7usize),
                                    &mut (*cred).ccache,
                                    &mut is_new,
                                ) != 0)
                                {
                                    (*cred).set_destroy_ccache(is_new);
                                    if !(json_to_keytab(
                                        context,
                                        crate::k5_json_h::k5_json_array_get(array, 8usize),
                                        &mut (*cred).client_keytab,
                                    ) != 0)
                                    {
                                        b = check_element(array, 9usize, 2u32)
                                            as crate::k5_json_h::k5_json_bool;
                                        if !b.is_null() {
                                            (*cred).have_tgt =
                                                crate::k5_json_h::k5_json_bool_value(b)
                                                    as crate::krb5_h::krb5_boolean;
                                            n = check_element(array, 10usize, 0u32)
                                                as crate::k5_json_h::k5_json_number;
                                            if !n.is_null() {
                                                (*cred).expire =
                                                    crate::k5_json_h::k5_json_number_value(n)
                                                        as crate::krb5_h::krb5_timestamp;
                                                n = check_element(array, 11usize, 0u32)
                                                    as crate::k5_json_h::k5_json_number;
                                                if !n.is_null() {
                                                    (*cred).refresh_time =
                                                        crate::k5_json_h::k5_json_number_value(n)
                                                            as crate::krb5_h::krb5_timestamp;
                                                    if !(json_to_etypes(
                                                        crate::k5_json_h::k5_json_array_get(
                                                            array, 12usize,
                                                        ),
                                                        &mut (*cred).req_enctypes,
                                                    ) != 0)
                                                    {
                                                        if !(json_to_optional_string(
                                                            crate::k5_json_h::k5_json_array_get(
                                                                array, 13usize,
                                                            ),
                                                            &mut (*cred).password,
                                                        ) != 0)
                                                        {
                                                            *cred_out = cred;
                                                            return 0i32;
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
    crate::src::krb5::rel_cred::krb5_gss_release_cred(
        &mut tmp,
        &mut cred as *mut crate::gssapiP_krb5_h::krb5_gss_cred_id_t
            as *mut crate::gssapi_h::gss_cred_id_t,
    );
    return -(1i32);
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
/* minor_status */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* cred_store */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* minor_status */
/* input_cred_handle */
/* input_usage */
/* desired_mech */
/* overwrite_cred */
/* default_cred */
/* cred_store */
/* elements_stored */
/* cred_usage_stored */
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_import_cred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut token: crate::gssapi_h::gss_buffer_t,
    mut cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut status = 0u32;
    let mut context = 0 as *mut crate::k5_int_h::_krb5_context;
    let mut ret: crate::krb5_h::krb5_error_code = 0;
    let mut cred = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_cred_id_rec;
    let mut v = 0 as *mut libc::c_void;
    let mut array = 0 as *mut crate::k5_json_h::k5_json_array_st;
    let mut str = 0 as *mut crate::k5_json_h::k5_json_string_st;
    let mut copy = 0 as *mut i8;
    ret = crate::src::krb5::init_sec_context::krb5_gss_init_context(&mut context);
    if ret != 0 {
        *minor_status = ret as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    /* Decode token. */
    copy = k5memdup0((*token).value, (*token).length, &mut ret) as *mut i8;
    if copy.is_null() {
        status = (13u32) << 16i32;
        *minor_status = ret as crate::gssapi_h::OM_uint32
    } else {
        if crate::k5_json_h::k5_json_decode(copy, &mut v) != 0 {
            current_block = 18177931904574916874;
        } else if crate::k5_json_h::k5_json_get_tid(v) != 129u32 {
            current_block = 18177931904574916874;
        } else {
            array = v as crate::k5_json_h::k5_json_array;
            if crate::k5_json_h::k5_json_array_length(array) != 2usize {
                current_block = 18177931904574916874;
            } else {
                str = check_element(array, 0usize, 131u32) as crate::k5_json_h::k5_json_string;
                if str.is_null()
                    || crate::stdlib::strcmp(
                        crate::k5_json_h::k5_json_string_utf8(str),
                        b"K5C1\x00" as *const u8 as *const i8,
                    ) != 0i32
                {
                    current_block = 18177931904574916874;
                } else if json_to_kgcred(
                    context,
                    crate::k5_json_h::k5_json_array_get(array, 1usize)
                        as crate::k5_json_h::k5_json_array,
                    &mut cred,
                ) != 0
                {
                    current_block = 18177931904574916874;
                } else {
                    *cred_handle = cred as crate::gssapi_h::gss_cred_id_t;
                    current_block = 4440839828259268565;
                }
            }
        }
        match current_block {
            4440839828259268565 => {}
            _ => status = (9u32) << 16i32,
        }
    }
    crate::stdlib::free(copy as *mut libc::c_void);
    crate::k5_json_h::k5_json_release(v);
    crate::krb5_h::krb5_free_context(context);
    return status;
}
/* Decode the CRED_EXPORT_MAGIC array wrapper. */
