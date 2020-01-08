use ::libc;

pub mod k5_thread_h {

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

    pub unsafe extern "C" fn ts2tt(
        mut timestamp: crate::krb5_h::krb5_timestamp,
    ) -> crate::stdlib::time_t {
        return timestamp as crate::stdlib::time_t;
    }

    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}

pub mod gssapiP_krb5_h {

    #[inline]

    pub unsafe extern "C" fn data_to_gss(
        mut input_k5data: *mut crate::krb5_h::krb5_data,
        mut output_buffer: crate::gssapi_h::gss_buffer_t,
    ) -> crate::krb5_h::krb5_error_code {
        let mut code = 0i32;
        (*output_buffer).length = (*input_k5data).length as crate::stddef_h::size_t;
        (*output_buffer).value = (*input_k5data).data as *mut libc::c_void;
        *input_k5data = empty_data();
        return code;
    }

    use crate::src::krb5::export_cred::k5_int_h::empty_data;

    /* _GSSAPIP_KRB5_H_ */
}

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
pub use crate::k5_int_h::k5_rc_get_name;
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
pub use crate::krb5_h::krb5_cc_cursor;
pub use crate::krb5_h::krb5_cc_end_seq_get;
pub use crate::krb5_h::krb5_cc_get_full_name;
pub use crate::krb5_h::krb5_cc_get_principal;
pub use crate::krb5_h::krb5_cc_get_type;
pub use crate::krb5_h::krb5_cc_next_cred;
pub use crate::krb5_h::krb5_cc_start_seq_get;
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
pub use crate::krb5_h::krb5_free_context;
pub use crate::krb5_h::krb5_free_cred_contents;
pub use crate::krb5_h::krb5_free_principal;
pub use crate::krb5_h::krb5_free_unparsed_name;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_keyblock;
pub use crate::krb5_h::krb5_keytab;
pub use crate::krb5_h::krb5_keytab_entry;
pub use crate::krb5_h::krb5_keytab_entry_st;
pub use crate::krb5_h::krb5_kt_cursor;
pub use crate::krb5_h::krb5_kt_get_name;
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
pub use crate::krb5_h::krb5_unparse_name;
pub use crate::profile_h::profile_t;
pub use crate::src::krb5::export_cred::k5_int_h::empty_data;
pub use crate::src::krb5::export_cred::k5_int_h::make_data;
pub use crate::src::krb5::export_cred::k5_int_h::string2data;
pub use crate::src::krb5::export_cred::k5_int_h::ts2tt;
pub use crate::src::krb5::export_cred::k5_thread_h::k5_mutex_unlock;

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
pub use crate::k5_json_h::k5_json_array_add;
pub use crate::k5_json_h::k5_json_array_create;
pub use crate::k5_json_h::k5_json_array_fmt;
pub use crate::k5_json_h::k5_json_array_st;
pub use crate::k5_json_h::k5_json_encode;
pub use crate::k5_json_h::k5_json_null_create_val;
pub use crate::k5_json_h::k5_json_number;
pub use crate::k5_json_h::k5_json_number_create;
pub use crate::k5_json_h::k5_json_number_st;
pub use crate::k5_json_h::k5_json_release;
pub use crate::k5_json_h::k5_json_string;
pub use crate::k5_json_h::k5_json_string_create;
pub use crate::k5_json_h::k5_json_string_st;
pub use crate::k5_json_h::k5_json_value;
pub use crate::mglueP_h::gss_cred_id_struct;
pub use crate::src::krb5::export_cred::gssapiP_krb5_h::data_to_gss;
pub use crate::src::krb5::init_sec_context::krb5_gss_init_context;
pub use crate::src::krb5::val_cred::krb5_gss_validate_cred_1;

/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/gssapi/krb5/export_cred.c - krb5 export_cred implementation */
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
/* Return a JSON null or array value representing princ. */

unsafe extern "C" fn json_principal(
    mut context: crate::krb5_h::krb5_context,
    mut princ: crate::krb5_h::krb5_principal,
    mut val_out: *mut crate::k5_json_h::k5_json_value,
) -> crate::krb5_h::krb5_error_code {
    let mut ret: crate::krb5_h::krb5_error_code = 0;
    let mut str = 0 as crate::k5_json_h::k5_json_string;
    let mut princname = 0 as *mut i8;
    *val_out = 0 as *mut libc::c_void;
    if princ.is_null() {
        return crate::k5_json_h::k5_json_null_create_val(val_out);
    }
    ret = crate::krb5_h::krb5_unparse_name(
        context,
        princ as crate::krb5_h::krb5_const_principal,
        &mut princname,
    );
    if ret != 0 {
        return ret;
    }
    ret = crate::k5_json_h::k5_json_string_create(princname, &mut str);
    crate::krb5_h::krb5_free_unparsed_name(context, princname);
    *val_out = str as crate::k5_json_h::k5_json_value;
    return ret;
}
/* Return a json null or array value representing etypes. */

unsafe extern "C" fn json_etypes(
    mut etypes: *mut crate::krb5_h::krb5_enctype,
    mut val_out: *mut crate::k5_json_h::k5_json_value,
) -> crate::krb5_h::krb5_error_code {
    let mut current_block: u64;
    let mut ret: crate::krb5_h::krb5_error_code = 0;
    let mut num = 0 as *mut crate::k5_json_h::k5_json_number_st;
    let mut array = 0 as *mut crate::k5_json_h::k5_json_array_st;
    *val_out = 0 as *mut libc::c_void;
    if etypes.is_null() {
        return crate::k5_json_h::k5_json_null_create_val(val_out);
    }
    ret = crate::k5_json_h::k5_json_array_create(&mut array);
    if ret != 0 {
        return ret;
    }
    loop {
        if !(*etypes != 0i32) {
            current_block = 12800627514080957624;
            break;
        }
        ret = crate::k5_json_h::k5_json_number_create(*etypes as i64, &mut num);
        if ret != 0 {
            current_block = 14866461976285342911;
            break;
        }
        ret = crate::k5_json_h::k5_json_array_add(array, num as crate::k5_json_h::k5_json_value);
        crate::k5_json_h::k5_json_release(num as crate::k5_json_h::k5_json_value);
        if ret != 0 {
            current_block = 14866461976285342911;
            break;
        }
        etypes = etypes.offset(1)
    }
    match current_block {
        14866461976285342911 => {
            crate::k5_json_h::k5_json_release(array as crate::k5_json_h::k5_json_value);
            return ret;
        }
        _ => {
            *val_out = array as crate::k5_json_h::k5_json_value;
            return 0i32;
        }
    };
}
/* Return a JSON null or array value representing name. */

unsafe extern "C" fn json_kgname(
    mut context: crate::krb5_h::krb5_context,
    mut name: crate::gssapiP_krb5_h::krb5_gss_name_t,
    mut val_out: *mut crate::k5_json_h::k5_json_value,
) -> crate::krb5_h::krb5_error_code {
    let mut ret: crate::krb5_h::krb5_error_code = 0;
    let mut array = 0 as crate::k5_json_h::k5_json_array;
    let mut princ = 0 as *mut libc::c_void;
    *val_out = 0 as *mut libc::c_void;
    if name.is_null() {
        return crate::k5_json_h::k5_json_null_create_val(val_out);
    }
    ret = json_principal(context, (*name).princ, &mut princ);
    if ret != 0 {
        return ret;
    }
    ret = crate::k5_json_h::k5_json_array_fmt(
        &mut array as *mut crate::k5_json_h::k5_json_array,
        b"vss\x00" as *const u8 as *const i8,
        princ,
        (*name).service,
        (*name).host,
    );
    crate::k5_json_h::k5_json_release(princ);
    *val_out = array as crate::k5_json_h::k5_json_value;
    return ret;
}
/* Return a JSON null or string value representing keytab. */

unsafe extern "C" fn json_keytab(
    mut context: crate::krb5_h::krb5_context,
    mut keytab: crate::krb5_h::krb5_keytab,
    mut val_out: *mut crate::k5_json_h::k5_json_value,
) -> crate::krb5_h::krb5_error_code {
    let mut ret: crate::krb5_h::krb5_error_code = 0;
    let mut str = 0 as *mut crate::k5_json_h::k5_json_string_st;
    let mut name: [i8; 1024] = [0; 1024];
    *val_out = 0 as *mut libc::c_void;
    if keytab.is_null() {
        return crate::k5_json_h::k5_json_null_create_val(val_out);
    }
    ret = crate::krb5_h::krb5_kt_get_name(
        context,
        keytab,
        name.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as u32,
    );
    if ret != 0 {
        return ret;
    }
    ret = crate::k5_json_h::k5_json_string_create(name.as_mut_ptr(), &mut str);
    *val_out = str as crate::k5_json_h::k5_json_value;
    return ret;
}
/* Return a JSON null or string value representing rcache. */

unsafe extern "C" fn json_rcache(
    mut context: crate::krb5_h::krb5_context,
    mut rcache: crate::krb5_h::krb5_rcache,
    mut val_out: *mut crate::k5_json_h::k5_json_value,
) -> crate::krb5_h::krb5_error_code {
    let mut ret: crate::krb5_h::krb5_error_code = 0;
    let mut str = 0 as crate::k5_json_h::k5_json_string;
    if rcache.is_null() {
        return crate::k5_json_h::k5_json_null_create_val(val_out);
    }
    ret = crate::k5_json_h::k5_json_string_create(
        crate::k5_int_h::k5_rc_get_name(context, rcache),
        &mut str,
    );
    *val_out = str as crate::k5_json_h::k5_json_value;
    return ret;
}
/* Return a JSON array value representing keyblock. */

unsafe extern "C" fn json_keyblock(
    mut kb: *mut crate::krb5_h::krb5_keyblock,
    mut val_out: *mut crate::k5_json_h::k5_json_value,
) -> crate::krb5_h::krb5_error_code {
    let mut ret: crate::krb5_h::krb5_error_code = 0;
    let mut array = 0 as *mut crate::k5_json_h::k5_json_array_st;
    *val_out = 0 as *mut libc::c_void;
    ret = crate::k5_json_h::k5_json_array_fmt(
        &mut array as *mut crate::k5_json_h::k5_json_array,
        b"iB\x00" as *const u8 as *const i8,
        (*kb).enctype,
        (*kb).contents as *mut libc::c_void,
        (*kb).length as crate::stddef_h::size_t,
    );
    if ret != 0 {
        return ret;
    }
    *val_out = array as crate::k5_json_h::k5_json_value;
    return 0i32;
}
/* Return a JSON array value representing addr. */

unsafe extern "C" fn json_address(
    mut addr: *mut crate::krb5_h::krb5_address,
    mut val_out: *mut crate::k5_json_h::k5_json_value,
) -> crate::krb5_h::krb5_error_code {
    let mut ret: crate::krb5_h::krb5_error_code = 0;
    let mut array = 0 as *mut crate::k5_json_h::k5_json_array_st;
    *val_out = 0 as *mut libc::c_void;
    ret = crate::k5_json_h::k5_json_array_fmt(
        &mut array as *mut crate::k5_json_h::k5_json_array,
        b"iB\x00" as *const u8 as *const i8,
        (*addr).addrtype,
        (*addr).contents as *mut libc::c_void,
        (*addr).length as crate::stddef_h::size_t,
    );
    if ret != 0 {
        return ret;
    }
    *val_out = array as crate::k5_json_h::k5_json_value;
    return 0i32;
}
/* Return a JSON null or array value representing addrs. */

unsafe extern "C" fn json_addresses(
    mut addrs: *mut *mut crate::krb5_h::krb5_address,
    mut val_out: *mut crate::k5_json_h::k5_json_value,
) -> crate::krb5_h::krb5_error_code {
    let mut current_block: u64;
    let mut ret: crate::krb5_h::krb5_error_code = 0;
    let mut array = 0 as *mut crate::k5_json_h::k5_json_array_st;
    let mut val = 0 as *mut libc::c_void;
    *val_out = 0 as *mut libc::c_void;
    if addrs.is_null() {
        return crate::k5_json_h::k5_json_null_create_val(val_out);
    }
    ret = crate::k5_json_h::k5_json_array_create(&mut array);
    if ret != 0 {
        return ret;
    }
    loop {
        if (*addrs).is_null() {
            current_block = 12800627514080957624;
            break;
        }
        ret = json_address(*addrs, &mut val);
        if ret != 0 {
            current_block = 5381337261603195026;
            break;
        }
        ret = crate::k5_json_h::k5_json_array_add(array, val);
        crate::k5_json_h::k5_json_release(val);
        if ret != 0 {
            current_block = 5381337261603195026;
            break;
        }
        addrs = addrs.offset(1)
    }
    match current_block {
        5381337261603195026 => {
            crate::k5_json_h::k5_json_release(array as crate::k5_json_h::k5_json_value);
            return ret;
        }
        _ => {
            *val_out = array as crate::k5_json_h::k5_json_value;
            return 0i32;
        }
    };
}
/* Return a JSON array value representing ad. */

unsafe extern "C" fn json_authdata_element(
    mut ad: *mut crate::krb5_h::krb5_authdata,
    mut val_out: *mut crate::k5_json_h::k5_json_value,
) -> crate::krb5_h::krb5_error_code {
    let mut ret: crate::krb5_h::krb5_error_code = 0;
    let mut array = 0 as *mut crate::k5_json_h::k5_json_array_st;
    *val_out = 0 as *mut libc::c_void;
    ret = crate::k5_json_h::k5_json_array_fmt(
        &mut array as *mut crate::k5_json_h::k5_json_array,
        b"iB\x00" as *const u8 as *const i8,
        (*ad).ad_type,
        (*ad).contents as *mut libc::c_void,
        (*ad).length as crate::stddef_h::size_t,
    );
    if ret != 0 {
        return ret;
    }
    *val_out = array as crate::k5_json_h::k5_json_value;
    return 0i32;
}
/* Return a JSON null or array value representing authdata. */

unsafe extern "C" fn json_authdata(
    mut authdata: *mut *mut crate::krb5_h::krb5_authdata,
    mut val_out: *mut crate::k5_json_h::k5_json_value,
) -> crate::krb5_h::krb5_error_code {
    let mut current_block: u64;
    let mut ret: crate::krb5_h::krb5_error_code = 0;
    let mut array = 0 as *mut crate::k5_json_h::k5_json_array_st;
    let mut val = 0 as *mut libc::c_void;
    *val_out = 0 as *mut libc::c_void;
    if authdata.is_null() {
        return crate::k5_json_h::k5_json_null_create_val(val_out);
    }
    ret = crate::k5_json_h::k5_json_array_create(&mut array);
    if ret != 0 {
        return ret;
    }
    loop {
        if (*authdata).is_null() {
            current_block = 12800627514080957624;
            break;
        }
        ret = json_authdata_element(*authdata, &mut val);
        if ret != 0 {
            current_block = 5463299119574025126;
            break;
        }
        ret = crate::k5_json_h::k5_json_array_add(array, val);
        crate::k5_json_h::k5_json_release(val);
        if ret != 0 {
            current_block = 5463299119574025126;
            break;
        }
        authdata = authdata.offset(1)
    }
    match current_block {
        5463299119574025126 => {
            crate::k5_json_h::k5_json_release(array as crate::k5_json_h::k5_json_value);
            return ret;
        }
        _ => {
            *val_out = array as crate::k5_json_h::k5_json_value;
            return 0i32;
        }
    };
}
/* Return a JSON array value representing creds. */

unsafe extern "C" fn json_creds(
    mut context: crate::krb5_h::krb5_context,
    mut creds: *mut crate::krb5_h::krb5_creds,
    mut val_out: *mut crate::k5_json_h::k5_json_value,
) -> crate::krb5_h::krb5_error_code {
    let mut ret: crate::krb5_h::krb5_error_code = 0;
    let mut array = 0 as *mut crate::k5_json_h::k5_json_array_st;
    let mut client = 0 as *mut libc::c_void;
    let mut server = 0 as *mut libc::c_void;
    let mut keyblock = 0 as *mut libc::c_void;
    let mut addrs = 0 as *mut libc::c_void;
    let mut authdata = 0 as *mut libc::c_void;
    *val_out = 0 as *mut libc::c_void;
    ret = json_principal(context, (*creds).client, &mut client);
    if !(ret != 0) {
        ret = json_principal(context, (*creds).server, &mut server);
        if !(ret != 0) {
            ret = json_keyblock(&mut (*creds).keyblock, &mut keyblock);
            if !(ret != 0) {
                ret = json_addresses((*creds).addresses, &mut addrs);
                if !(ret != 0) {
                    ret = json_authdata((*creds).authdata, &mut authdata);
                    if !(ret != 0) {
                        ret = crate::k5_json_h::k5_json_array_fmt(
                            &mut array as *mut crate::k5_json_h::k5_json_array,
                            b"vvviiiibivBBv\x00" as *const u8 as *const i8,
                            client,
                            server,
                            keyblock,
                            (*creds).times.authtime,
                            (*creds).times.starttime,
                            (*creds).times.endtime,
                            (*creds).times.renew_till,
                            (*creds).is_skey,
                            (*creds).ticket_flags,
                            addrs,
                            (*creds).ticket.data as *mut libc::c_void,
                            (*creds).ticket.length as crate::stddef_h::size_t,
                            (*creds).second_ticket.data as *mut libc::c_void,
                            (*creds).second_ticket.length as crate::stddef_h::size_t,
                            authdata,
                        );
                        if !(ret != 0) {
                            *val_out = array as crate::k5_json_h::k5_json_value
                        }
                    }
                }
            }
        }
    }
    crate::k5_json_h::k5_json_release(client);
    crate::k5_json_h::k5_json_release(server);
    crate::k5_json_h::k5_json_release(keyblock);
    crate::k5_json_h::k5_json_release(addrs);
    crate::k5_json_h::k5_json_release(authdata);
    return ret;
}
/* Return a JSON array value representing the contents of ccache. */

unsafe extern "C" fn json_ccache_contents(
    mut context: crate::krb5_h::krb5_context,
    mut ccache: crate::krb5_h::krb5_ccache,
    mut val_out: *mut crate::k5_json_h::k5_json_value,
) -> crate::krb5_h::krb5_error_code {
    let mut ret: crate::krb5_h::krb5_error_code = 0;
    let mut princ = 0 as *mut crate::krb5_h::krb5_principal_data;
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
    let mut array = 0 as *mut crate::k5_json_h::k5_json_array_st;
    let mut val = 0 as *mut libc::c_void;
    *val_out = 0 as *mut libc::c_void;
    ret = crate::k5_json_h::k5_json_array_create(&mut array);
    if ret != 0 {
        return ret;
    }
    /* Put the principal in the first array entry. */
    ret = crate::krb5_h::krb5_cc_get_principal(context, ccache, &mut princ);
    if !(ret != 0) {
        ret = json_principal(context, princ, &mut val);
        crate::krb5_h::krb5_free_principal(context, princ);
        if !(ret != 0) {
            ret = crate::k5_json_h::k5_json_array_add(array, val);
            crate::k5_json_h::k5_json_release(val);
            if !(ret != 0) {
                /* Put credentials in the remaining array entries. */
                ret = crate::krb5_h::krb5_cc_start_seq_get(context, ccache, &mut cursor);
                if !(ret != 0) {
                    loop {
                        ret = crate::krb5_h::krb5_cc_next_cred(
                            context,
                            ccache,
                            &mut cursor,
                            &mut creds,
                        );
                        if !(ret == 0i32) {
                            break;
                        }
                        ret = json_creds(context, &mut creds, &mut val);
                        crate::krb5_h::krb5_free_cred_contents(context, &mut creds);
                        if ret != 0 {
                            break;
                        }
                        ret = crate::k5_json_h::k5_json_array_add(array, val);
                        crate::k5_json_h::k5_json_release(val);
                        if ret != 0 {
                            break;
                        }
                    }
                    crate::krb5_h::krb5_cc_end_seq_get(context, ccache, &mut cursor);
                    if !(ret as isize != -(1765328242 as isize)) {
                        *val_out = array as crate::k5_json_h::k5_json_value;
                        return 0i32;
                    }
                }
            }
        }
    }
    crate::k5_json_h::k5_json_release(array as crate::k5_json_h::k5_json_value);
    return ret;
}
/* Return a JSON null, string, or array value representing ccache. */

unsafe extern "C" fn json_ccache(
    mut context: crate::krb5_h::krb5_context,
    mut ccache: crate::krb5_h::krb5_ccache,
    mut val_out: *mut crate::k5_json_h::k5_json_value,
) -> crate::krb5_h::krb5_error_code {
    let mut ret: crate::krb5_h::krb5_error_code = 0;
    let mut str = 0 as *mut crate::k5_json_h::k5_json_string_st;
    let mut name = 0 as *mut i8;
    *val_out = 0 as *mut libc::c_void;
    if ccache.is_null() {
        return crate::k5_json_h::k5_json_null_create_val(val_out);
    }
    if crate::stdlib::strcmp(
        crate::krb5_h::krb5_cc_get_type(context, ccache),
        b"MEMORY\x00" as *const u8 as *const i8,
    ) == 0i32
    {
        return json_ccache_contents(context, ccache, val_out);
    } else {
        ret = crate::krb5_h::krb5_cc_get_full_name(context, ccache, &mut name);
        if ret != 0 {
            return ret;
        }
        ret = crate::k5_json_h::k5_json_string_create(name, &mut str);
        crate::stdlib::free(name as *mut libc::c_void);
        *val_out = str as crate::k5_json_h::k5_json_value;
        return ret;
    };
}
/* Return a JSON array value representing cred. */

unsafe extern "C" fn json_kgcred(
    mut context: crate::krb5_h::krb5_context,
    mut cred: crate::gssapiP_krb5_h::krb5_gss_cred_id_t,
    mut val_out: *mut crate::k5_json_h::k5_json_value,
) -> crate::krb5_h::krb5_error_code {
    let mut ret: crate::krb5_h::krb5_error_code = 0;
    let mut array = 0 as *mut crate::k5_json_h::k5_json_array_st;
    let mut name = 0 as *mut libc::c_void;
    let mut imp = 0 as *mut libc::c_void;
    let mut keytab = 0 as *mut libc::c_void;
    let mut rcache = 0 as *mut libc::c_void;
    let mut ccache = 0 as *mut libc::c_void;
    let mut ckeytab = 0 as *mut libc::c_void;
    let mut etypes = 0 as *mut libc::c_void;
    *val_out = 0 as *mut libc::c_void;
    ret = json_kgname(context, (*cred).name, &mut name);
    if !(ret != 0) {
        ret = json_principal(context, (*cred).impersonator, &mut imp);
        if !(ret != 0) {
            ret = json_keytab(context, (*cred).keytab, &mut keytab);
            if !(ret != 0) {
                ret = json_rcache(context, (*cred).rcache, &mut rcache);
                if !(ret != 0) {
                    ret = json_ccache(context, (*cred).ccache, &mut ccache);
                    if !(ret != 0) {
                        ret = json_keytab(context, (*cred).client_keytab, &mut ckeytab);
                        if !(ret != 0) {
                            ret = json_etypes((*cred).req_enctypes, &mut etypes);
                            if !(ret != 0) {
                                ret = crate::k5_json_h::k5_json_array_fmt(
                                    &mut array as *mut crate::k5_json_h::k5_json_array,
                                    b"ivvbbvvvvbLLvs\x00" as *const u8 as *const i8,
                                    (*cred).usage,
                                    name,
                                    imp,
                                    (*cred).default_identity() as i32,
                                    (*cred).iakerb_mech() as i32,
                                    keytab,
                                    rcache,
                                    ccache,
                                    ckeytab,
                                    (*cred).have_tgt,
                                    ts2tt((*cred).expire) as i64,
                                    ts2tt((*cred).refresh_time) as i64,
                                    etypes,
                                    (*cred).password,
                                );
                                if !(ret != 0) {
                                    *val_out = array as crate::k5_json_h::k5_json_value
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    crate::k5_json_h::k5_json_release(name);
    crate::k5_json_h::k5_json_release(imp);
    crate::k5_json_h::k5_json_release(keytab);
    crate::k5_json_h::k5_json_release(rcache);
    crate::k5_json_h::k5_json_release(ccache);
    crate::k5_json_h::k5_json_release(ckeytab);
    crate::k5_json_h::k5_json_release(etypes);
    return ret;
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

pub unsafe extern "C" fn krb5_gss_export_cred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut token: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut status = 0u32;
    let mut context = 0 as *mut crate::k5_int_h::_krb5_context;
    let mut ret: crate::krb5_h::krb5_error_code = 0;
    let mut cred = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_cred_id_rec;
    let mut array = 0 as crate::k5_json_h::k5_json_array;
    let mut jcred = 0 as *mut libc::c_void;
    let mut str = 0 as *mut i8;
    let mut d = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    ret = crate::src::krb5::init_sec_context::krb5_gss_init_context(&mut context);
    if ret != 0 {
        *minor_status = ret as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    /* Validate and lock cred_handle. */
    status =
        crate::src::krb5::val_cred::krb5_gss_validate_cred_1(minor_status, cred_handle, context);
    if status != 0u32 {
        return status;
    }
    cred = cred_handle as crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
    if json_kgcred(context, cred, &mut jcred) != 0 {
        current_block = 18177931904574916874;
    } else if crate::k5_json_h::k5_json_array_fmt(
        &mut array as *mut crate::k5_json_h::k5_json_array,
        b"sv\x00" as *const u8 as *const i8,
        b"K5C1\x00" as *const u8 as *const i8,
        jcred,
    ) != 0
    {
        current_block = 18177931904574916874;
    } else if crate::k5_json_h::k5_json_encode(array as crate::k5_json_h::k5_json_value, &mut str)
        != 0
    {
        current_block = 18177931904574916874;
    } else {
        d = string2data(str);
        if data_to_gss(&mut d, token) != 0 {
            current_block = 18177931904574916874;
        } else {
            str = 0 as *mut i8;
            current_block = 2500679475800559546;
        }
    }
    match current_block {
        18177931904574916874 => {
            *minor_status = 12u32;
            status = (13u32) << 16i32
        }
        _ => {}
    }
    crate::stdlib::free(str as *mut libc::c_void);
    k5_mutex_unlock(&mut (*cred).lock);
    crate::k5_json_h::k5_json_release(array as crate::k5_json_h::k5_json_value);
    crate::k5_json_h::k5_json_release(jcred);
    crate::krb5_h::krb5_free_context(context);
    return status;
}
