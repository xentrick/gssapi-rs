use ::libc;

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

    pub unsafe extern "C" fn alloc_data(
        mut data: *mut crate::krb5_h::krb5_data,
        mut len: u32,
    ) -> crate::krb5_h::krb5_error_code {
        /* Allocate at least one byte since zero-byte allocs may return NULL. */
        let mut ptr =
            crate::stdlib::calloc(if len > 0u32 { len } else { 1u32 } as usize, 1usize) as *mut i8;
        if ptr.is_null() {
            return 12i32;
        }
        (*data).magic = -(1760647422 as isize) as crate::krb5_h::krb5_magic;
        (*data).data = ptr;
        (*data).length = len;
        return 0i32;
    }
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

    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}

pub mod gssapi_alloc_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
    /* To the extent possible under law, Painless Security, LLC has waived
     * all copyright and related or neighboring rights to GSS-API Memory
     * Management Header. This work is published from: United States.
     */
    /* not _WIN32 or DEBUG_GSSALLOC */
    /* Normal Unix case, just use free/malloc/calloc/realloc. */
    #[inline]

    pub unsafe extern "C" fn gssalloc_free(mut value: *mut libc::c_void) {
        crate::stdlib::free(value);
    }
    #[inline]

    pub unsafe extern "C" fn gssalloc_malloc(
        mut size: crate::stddef_h::size_t,
    ) -> *mut libc::c_void {
        return crate::stdlib::malloc(size);
    }
}
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__pthread_internal_list;
pub use crate::stdlib::__pthread_list_t;
pub use crate::stdlib::__pthread_mutex_s;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint64_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::pthread_mutex_t;

pub use crate::k5_err_h::errinfo;
pub use crate::k5_int_h::_kdb5_dal_handle;
pub use crate::k5_int_h::_kdb_log_context;
pub use crate::k5_int_h::_krb5_authdata_context;
pub use crate::k5_int_h::_krb5_authdata_context_module;
pub use crate::k5_int_h::_krb5_context;
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
pub use crate::k5_int_h::krb5int_arcfour_gsscrypt;
pub use crate::k5_int_h::krb5int_c_mandatory_cksumtype;
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
pub use crate::krb5_h::_krb5_address;
pub use crate::krb5_h::_krb5_ap_req;
pub use crate::krb5_h::_krb5_auth_context;
pub use crate::krb5_h::_krb5_authdata;
pub use crate::krb5_h::_krb5_crypto_iov;
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
pub use crate::krb5_h::krb5_c_block_size;
pub use crate::krb5_h::krb5_c_crypto_length;
pub use crate::krb5_h::krb5_c_random_make_octets;
pub use crate::krb5_h::krb5_cksumtype;
pub use crate::krb5_h::krb5_const_pointer;
pub use crate::krb5_h::krb5_const_principal;
pub use crate::krb5_h::krb5_context;
pub use crate::krb5_h::krb5_crypto_iov;
pub use crate::krb5_h::krb5_cryptotype;
pub use crate::krb5_h::krb5_data;
pub use crate::krb5_h::krb5_deltat;
pub use crate::krb5_h::krb5_enc_data;
pub use crate::krb5_h::krb5_enc_tkt_part;
pub use crate::krb5_h::krb5_enctype;
pub use crate::krb5_h::krb5_error_code;
pub use crate::krb5_h::krb5_flags;
pub use crate::krb5_h::krb5_free_data;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_k_create_key;
pub use crate::krb5_h::krb5_k_decrypt;
pub use crate::krb5_h::krb5_k_decrypt_iov;
pub use crate::krb5_h::krb5_k_encrypt;
pub use crate::krb5_h::krb5_k_encrypt_iov;
pub use crate::krb5_h::krb5_k_free_key;
pub use crate::krb5_h::krb5_key;
pub use crate::krb5_h::krb5_keyblock;
pub use crate::krb5_h::krb5_keyusage;
pub use crate::krb5_h::krb5_kvno;
pub use crate::krb5_h::krb5_magic;
pub use crate::krb5_h::krb5_octet;
pub use crate::krb5_h::krb5_pointer;
pub use crate::krb5_h::krb5_post_recv_fn;
pub use crate::krb5_h::krb5_pre_send_fn;
pub use crate::krb5_h::krb5_principal;
pub use crate::krb5_h::krb5_principal_data;
pub use crate::krb5_h::krb5_prompt_type;
pub use crate::krb5_h::krb5_ticket;
pub use crate::krb5_h::krb5_ticket_times;
pub use crate::krb5_h::krb5_timestamp;
pub use crate::krb5_h::krb5_trace_callback;
pub use crate::krb5_h::krb5_trace_info;
pub use crate::krb5_h::krb5_transited;
pub use crate::profile_h::profile_t;
pub use crate::src::krb5::util_crypt::k5_int_h::alloc_data;
pub use crate::src::krb5::util_crypt::k5_int_h::k5alloc;
pub use crate::src::krb5::util_crypt::k5_int_h::k5calloc;
pub use crate::src::krb5::util_crypt::k5_int_h::make_data;
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
pub use crate::gssapiP_generic_h::g_seqnum_state;
pub use crate::gssapiP_generic_h::g_seqnum_state_st;
pub use crate::gssapiP_krb5_h::_krb5_gss_ctx_id_rec;
pub use crate::gssapiP_krb5_h::_krb5_gss_name_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_name_t;
pub use crate::gssapiP_krb5_h::seal_alg;
pub use crate::gssapiP_krb5_h::sgn_alg;
pub use crate::gssapiP_krb5_h::SEAL_ALG_DES3KD;
pub use crate::gssapiP_krb5_h::SEAL_ALG_MICROSOFT_RC4;
pub use crate::gssapiP_krb5_h::SEAL_ALG_NONE;
pub use crate::gssapiP_krb5_h::SGN_ALG_HMAC_MD5;
pub use crate::gssapiP_krb5_h::SGN_ALG_HMAC_SHA1_DES3_KD;
pub use crate::gssapi_ext_h::gss_iov_buffer_desc;
pub use crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
pub use crate::gssapi_ext_h::gss_iov_buffer_t;
pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::src::krb5::util_crypt::gssapi_alloc_h::gssalloc_free;
pub use crate::src::krb5::util_crypt::gssapi_alloc_h::gssalloc_malloc;

/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2001, 2008 by the Massachusetts Institute of Technology.
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

unsafe extern "C" fn kg_copy_keys(
    mut context: crate::krb5_h::krb5_context,
    mut ctx: *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec,
    mut subkey: crate::krb5_h::krb5_key,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    crate::krb5_h::krb5_k_free_key(context, (*ctx).enc);
    (*ctx).enc = 0 as crate::krb5_h::krb5_key;
    code = crate::krb5_h::krb5_k_create_key(context, &mut (*subkey).keyblock, &mut (*ctx).enc);
    if code != 0i32 {
        return code;
    }
    crate::krb5_h::krb5_k_free_key(context, (*ctx).seq);
    (*ctx).seq = 0 as crate::krb5_h::krb5_key;
    code = crate::krb5_h::krb5_k_create_key(context, &mut (*subkey).keyblock, &mut (*ctx).seq);
    if code != 0i32 {
        return code;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn kg_setup_keys(
    mut context: crate::krb5_h::krb5_context,
    mut ctx: *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec,
    mut subkey: crate::krb5_h::krb5_key,
    mut cksumtype: *mut crate::krb5_h::krb5_cksumtype,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    if !ctx.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"ctx != NULL\x00" as *const u8 as *const i8,
                      b"util_crypt.c\x00" as *const u8 as *const i8,
                      83u32,
                      (*::std::mem::transmute::<&[u8; 95],
                                                &[i8; 95]>(b"krb5_error_code kg_setup_keys(krb5_context, krb5_gss_ctx_id_rec *, krb5_key, krb5_cksumtype *)\x00")).as_ptr());
    }
    if !subkey.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"subkey != NULL\x00" as *const u8 as
                          *const i8,
                      b"util_crypt.c\x00" as *const u8 as *const i8,
                      84u32,
                      (*::std::mem::transmute::<&[u8; 95],
                                                &[i8; 95]>(b"krb5_error_code kg_setup_keys(krb5_context, krb5_gss_ctx_id_rec *, krb5_key, krb5_cksumtype *)\x00")).as_ptr());
    }
    *cksumtype = 0i32;
    (*ctx).proto = 0i32;
    if (*ctx).enc.is_null() {
        (*ctx).signalg = -(1i32);
        (*ctx).sealalg = -(1i32)
    }
    code = crate::k5_int_h::krb5int_c_mandatory_cksumtype(
        context,
        (*subkey).keyblock.enctype,
        cksumtype,
    );
    if code != 0i32 {
        return code;
    }
    match (*subkey).keyblock.enctype {
        16 => {
            code = kg_copy_keys(context, ctx, subkey);
            if code != 0i32 {
                return code;
            }
            (*(*ctx).enc).keyblock.enctype = 0x6i32;
            (*(*ctx).seq).keyblock.enctype = 0x6i32;
            (*ctx).signalg = crate::gssapiP_krb5_h::SGN_ALG_HMAC_SHA1_DES3_KD as i32;
            (*ctx).cksum_size = 20usize;
            (*ctx).sealalg = crate::gssapiP_krb5_h::SEAL_ALG_DES3KD as i32
        }
        23 | 24 => {
            /* RFC 4121 accidentally omits RC4-HMAC-EXP as a "not-newer" enctype,
             * even though RFC 4757 treats it as one. */
            code = kg_copy_keys(context, ctx, subkey);
            if code != 0i32 {
                return code;
            }
            (*ctx).signalg = crate::gssapiP_krb5_h::SGN_ALG_HMAC_MD5 as i32;
            (*ctx).cksum_size = 8usize;
            (*ctx).sealalg = crate::gssapiP_krb5_h::SEAL_ALG_MICROSOFT_RC4 as i32
        }
        _ => (*ctx).proto = 1i32,
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn kg_confounder_size(
    mut context: crate::krb5_h::krb5_context,
    mut enctype: crate::krb5_h::krb5_enctype,
) -> i32 {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut blocksize: crate::stddef_h::size_t = 0;
    /* We special case rc4*/
    if enctype == 0x17i32 || enctype == 0x18i32 {
        return 8i32;
    } /* XXX */
    code = crate::krb5_h::krb5_c_block_size(context, enctype, &mut blocksize);
    if code != 0 {
        return -(1i32);
    }
    return blocksize as i32;
}
#[no_mangle]

pub unsafe extern "C" fn kg_make_confounder(
    mut context: crate::krb5_h::krb5_context,
    mut enctype: crate::krb5_h::krb5_enctype,
    mut buf: *mut u8,
) -> crate::krb5_h::krb5_error_code {
    let mut confsize: i32 = 0;
    let mut lrandom = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    confsize = kg_confounder_size(context, enctype);
    if confsize < 0i32 {
        return -(1765328194 as isize) as crate::krb5_h::krb5_error_code;
    }
    lrandom.length = confsize as u32;
    lrandom.data = buf as *mut i8;
    return crate::krb5_h::krb5_c_random_make_octets(context, &mut lrandom);
}
/* Set *data_out to a krb5_data structure containing iv, or to NULL if iv is
 * NULL. */

unsafe extern "C" fn iv_to_state(
    mut context: crate::krb5_h::krb5_context,
    mut key: crate::krb5_h::krb5_key,
    mut iv: crate::krb5_h::krb5_pointer,
    mut data_out: *mut *mut crate::krb5_h::krb5_data,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut data = 0 as *mut crate::krb5_h::krb5_data;
    let mut blocksize: crate::stddef_h::size_t = 0;
    *data_out = 0 as *mut crate::krb5_h::krb5_data;
    if iv.is_null() {
        return 0i32;
    }
    code = crate::krb5_h::krb5_c_block_size(context, (*key).keyblock.enctype, &mut blocksize);
    if code != 0 {
        return code;
    }
    data = k5alloc(::std::mem::size_of::<crate::krb5_h::krb5_data>(), &mut code)
        as *mut crate::krb5_h::krb5_data;
    if data.is_null() {
        return code;
    }
    code = alloc_data(data, blocksize as u32);
    if code != 0 {
        crate::stdlib::free(data as *mut libc::c_void);
        return code;
    }
    crate::stdlib::memcpy(
        (*data).data as *mut libc::c_void,
        iv as *const libc::c_void,
        blocksize,
    );
    *data_out = data;
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn kg_encrypt(
    mut context: crate::krb5_h::krb5_context,
    mut key: crate::krb5_h::krb5_key,
    mut usage: i32,
    mut iv: crate::krb5_h::krb5_pointer,
    mut in_0: crate::krb5_h::krb5_const_pointer,
    mut out: crate::krb5_h::krb5_pointer,
    mut length: u32,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut state = 0 as *mut crate::krb5_h::krb5_data;
    let mut inputd = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    let mut outputd = crate::krb5_h::krb5_enc_data {
        magic: 0,
        enctype: 0,
        kvno: 0,
        ciphertext: crate::krb5_h::krb5_data {
            magic: 0,
            length: 0,
            data: 0 as *mut i8,
        },
    };
    code = iv_to_state(context, key, iv, &mut state);
    if code != 0 {
        return code;
    }
    inputd.length = length;
    inputd.data = in_0 as *mut i8;
    outputd.ciphertext.length = length;
    outputd.ciphertext.data = out as *mut i8;
    code = crate::krb5_h::krb5_k_encrypt(context, key, usage, state, &mut inputd, &mut outputd);
    crate::krb5_h::krb5_free_data(context, state);
    return code;
}
#[no_mangle]

pub unsafe extern "C" fn kg_encrypt_inplace(
    mut context: crate::krb5_h::krb5_context,
    mut key: crate::krb5_h::krb5_key,
    mut usage: i32,
    mut iv: crate::krb5_h::krb5_pointer,
    mut ptr: crate::krb5_h::krb5_pointer,
    mut length: u32,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut iov = crate::krb5_h::krb5_crypto_iov {
        flags: 0,
        data: crate::krb5_h::krb5_data {
            magic: 0,
            length: 0,
            data: 0 as *mut i8,
        },
    };
    let mut state = 0 as *mut crate::krb5_h::krb5_data;
    code = iv_to_state(context, key, iv, &mut state);
    if code != 0 {
        return code;
    }
    iov.flags = 2i32;
    iov.data = make_data(ptr, length);
    code = crate::krb5_h::krb5_k_encrypt_iov(context, key, usage, state, &mut iov, 1usize);
    crate::krb5_h::krb5_free_data(context, state);
    return code;
}
/* length is the length of the cleartext. */
#[no_mangle]

pub unsafe extern "C" fn kg_decrypt(
    mut context: crate::krb5_h::krb5_context,
    mut key: crate::krb5_h::krb5_key,
    mut usage: i32,
    mut iv: crate::krb5_h::krb5_pointer,
    mut in_0: crate::krb5_h::krb5_const_pointer,
    mut out: crate::krb5_h::krb5_pointer,
    mut length: u32,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut state = 0 as *mut crate::krb5_h::krb5_data;
    let mut outputd = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    let mut inputd = crate::krb5_h::krb5_enc_data {
        magic: 0,
        enctype: 0,
        kvno: 0,
        ciphertext: crate::krb5_h::krb5_data {
            magic: 0,
            length: 0,
            data: 0 as *mut i8,
        },
    };
    code = iv_to_state(context, key, iv, &mut state);
    if code != 0 {
        return code;
    }
    inputd.enctype = 0x1ffi32;
    inputd.ciphertext.length = length;
    inputd.ciphertext.data = in_0 as *mut i8;
    outputd.length = length;
    outputd.data = out as *mut i8;
    code = crate::krb5_h::krb5_k_decrypt(context, key, usage, state, &mut inputd, &mut outputd);
    crate::krb5_h::krb5_free_data(context, state);
    return code;
}
#[no_mangle]

pub unsafe extern "C" fn kg_arcfour_docrypt(
    mut keyblock: *const crate::krb5_h::krb5_keyblock,
    mut usage: i32,
    mut kd_data: *const u8,
    mut kd_data_len: crate::stddef_h::size_t,
    mut input_buf: *const u8,
    mut input_len: crate::stddef_h::size_t,
    mut output_buf: *mut u8,
) -> crate::krb5_h::krb5_error_code {
    let mut kd = make_data(kd_data as *mut libc::c_void, kd_data_len as u32);
    let mut kiov = crate::krb5_h::krb5_crypto_iov {
        flags: 0,
        data: crate::krb5_h::krb5_data {
            magic: 0,
            length: 0,
            data: 0 as *mut i8,
        },
    };
    crate::stdlib::memcpy(
        output_buf as *mut libc::c_void,
        input_buf as *const libc::c_void,
        input_len,
    );
    kiov.flags = 2i32;
    kiov.data = make_data(output_buf as *mut libc::c_void, input_len as u32);
    return crate::k5_int_h::krb5int_arcfour_gsscrypt(keyblock, usage, &mut kd, &mut kiov, 1usize);
}
/* AEAD */

unsafe extern "C" fn kg_translate_iov_v1(
    mut context: crate::krb5_h::krb5_context,
    mut enctype: crate::krb5_h::krb5_enctype,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
    mut pkiov: *mut *mut crate::krb5_h::krb5_crypto_iov,
    mut pkiov_count: *mut crate::stddef_h::size_t,
) -> crate::krb5_h::krb5_error_code {
    let mut header = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc;
    let mut trailer = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc;
    let mut i = 0i32;
    let mut j: i32 = 0;
    let mut kiov_count: crate::stddef_h::size_t = 0;
    let mut kiov = 0 as *mut crate::krb5_h::krb5_crypto_iov;
    let mut conf_len: crate::stddef_h::size_t = 0;
    *pkiov = 0 as *mut crate::krb5_h::krb5_crypto_iov;
    *pkiov_count = 0usize;
    conf_len = kg_confounder_size(context, enctype) as crate::stddef_h::size_t;
    header = kg_locate_iov(iov, iov_count, 2u32);
    if !header.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"header != NULL\x00" as *const u8 as
                          *const i8,
                      b"util_crypt.c\x00" as *const u8 as *const i8,
                      296u32,
                      (*::std::mem::transmute::<&[u8; 122],
                                                &[i8; 122]>(b"krb5_error_code kg_translate_iov_v1(krb5_context, krb5_enctype, gss_iov_buffer_desc *, int, krb5_crypto_iov **, size_t *)\x00")).as_ptr());
    }
    if (*header).buffer.length < conf_len {
        return -(1765328194 as isize) as crate::krb5_h::krb5_error_code;
    }
    trailer = kg_locate_iov(iov, iov_count, 7u32);
    if trailer.is_null() || (*trailer).buffer.length == 0usize {
    } else {
        crate::stdlib::__assert_fail(b"trailer == NULL || trailer->buffer.length == 0\x00" as
                          *const u8 as *const i8,
                      b"util_crypt.c\x00" as *const u8 as *const i8,
                      302u32,
                      (*::std::mem::transmute::<&[u8; 122],
                                                &[i8; 122]>(b"krb5_error_code kg_translate_iov_v1(krb5_context, krb5_enctype, gss_iov_buffer_desc *, int, krb5_crypto_iov **, size_t *)\x00")).as_ptr());
    }
    kiov_count = (3i32 + iov_count) as crate::stddef_h::size_t;
    kiov = crate::stdlib::malloc(
        kiov_count.wrapping_mul(::std::mem::size_of::<crate::krb5_h::krb5_crypto_iov>()),
    ) as *mut crate::krb5_h::krb5_crypto_iov;
    if kiov.is_null() {
        return 12i32;
    }
    /* For pre-CFX (raw enctypes) there is no krb5 header */
    (*kiov.offset(i as isize)).flags = 1i32;
    (*kiov.offset(i as isize)).data.length = 0u32;
    let ref mut fresh0 = (*kiov.offset(i as isize)).data.data;
    *fresh0 = 0 as *mut i8;
    i += 1;
    /* For pre-CFX, the confounder is at the end of the GSS header */
    (*kiov.offset(i as isize)).flags = 2i32;
    (*kiov.offset(i as isize)).data.length = conf_len as u32;
    let ref mut fresh1 = (*kiov.offset(i as isize)).data.data;
    *fresh1 = ((*header).buffer.value as *mut i8)
        .offset((*header).buffer.length as isize)
        .offset(-(conf_len as isize));
    i += 1;
    j = 0i32;
    while j < iov_count {
        (*kiov.offset(i as isize)).flags = kg_translate_flag_iov((*iov.offset(j as isize)).type_0);
        if !((*kiov.offset(i as isize)).flags == 0i32) {
            (*kiov.offset(i as isize)).data.length = (*iov.offset(j as isize)).buffer.length as u32;
            let ref mut fresh2 = (*kiov.offset(i as isize)).data.data;
            *fresh2 = (*iov.offset(j as isize)).buffer.value as *mut i8;
            i += 1
        }
        j += 1
    }
    (*kiov.offset(i as isize)).flags = 5i32;
    (*kiov.offset(i as isize)).data.length = 0u32;
    let ref mut fresh3 = (*kiov.offset(i as isize)).data.data;
    *fresh3 = 0 as *mut i8;
    i += 1;
    *pkiov = kiov;
    *pkiov_count = i as crate::stddef_h::size_t;
    return 0i32;
}
/*
 * DCE_STYLE indicates actual RRC is EC + RRC
 * EC is extra rotate count for DCE_STYLE, pad length otherwise
 * RRC is rotate count.
 */

unsafe extern "C" fn kg_translate_iov_v3(
    mut context: crate::krb5_h::krb5_context,
    mut dce_style: i32,
    mut ec: crate::stddef_h::size_t,
    mut rrc: crate::stddef_h::size_t,
    mut enctype: crate::krb5_h::krb5_enctype,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
    mut pkiov: *mut *mut crate::krb5_h::krb5_crypto_iov,
    mut pkiov_count: *mut crate::stddef_h::size_t,
) -> crate::krb5_h::krb5_error_code {
    let mut header = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
    let mut trailer = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
    let mut i = 0i32;
    let mut j: i32 = 0;
    let mut kiov_count: crate::stddef_h::size_t = 0;
    let mut kiov = 0 as *mut crate::krb5_h::krb5_crypto_iov;
    let mut k5_headerlen = 0u32;
    let mut k5_trailerlen = 0u32;
    let mut gss_headerlen: crate::stddef_h::size_t = 0;
    let mut gss_trailerlen: crate::stddef_h::size_t = 0;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    *pkiov = 0 as *mut crate::krb5_h::krb5_crypto_iov;
    *pkiov_count = 0usize;
    header = kg_locate_iov(iov, iov_count, 2u32);
    if !header.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"header != NULL\x00" as *const u8 as
                          *const i8,
                      b"util_crypt.c\x00" as *const u8 as *const i8,
                      366u32,
                      (*::std::mem::transmute::<&[u8; 143],
                                                &[i8; 143]>(b"krb5_error_code kg_translate_iov_v3(krb5_context, int, size_t, size_t, krb5_enctype, gss_iov_buffer_desc *, int, krb5_crypto_iov **, size_t *)\x00")).as_ptr());
    }
    trailer = kg_locate_iov(iov, iov_count, 7u32);
    if trailer.is_null() || rrc == 0usize {
    } else {
        crate::stdlib::__assert_fail(b"trailer == NULL || rrc == 0\x00" as *const u8 as
                          *const i8,
                      b"util_crypt.c\x00" as *const u8 as *const i8,
                      369u32,
                      (*::std::mem::transmute::<&[u8; 143],
                                                &[i8; 143]>(b"krb5_error_code kg_translate_iov_v3(krb5_context, int, size_t, size_t, krb5_enctype, gss_iov_buffer_desc *, int, krb5_crypto_iov **, size_t *)\x00")).as_ptr());
    }
    code = crate::krb5_h::krb5_c_crypto_length(context, enctype, 1i32, &mut k5_headerlen);
    if code != 0i32 {
        return code;
    }
    code = crate::krb5_h::krb5_c_crypto_length(context, enctype, 5i32, &mut k5_trailerlen);
    if code != 0i32 {
        return code;
    }
    /* Check header and trailer sizes */
    gss_headerlen = (16u32).wrapping_add(k5_headerlen) as crate::stddef_h::size_t; /* Kerb-Header */
    gss_trailerlen = ec
        .wrapping_add(16usize)
        .wrapping_add(k5_trailerlen as usize); /* Kerb-Trailer */
    /* If we're caller without a trailer, we must rotate by trailer length */
    if trailer.is_null() {
        let mut actual_rrc = rrc; /* compensate for Windows bug */
        if dce_style != 0 {
            actual_rrc = (actual_rrc).wrapping_add(ec)
        }
        if actual_rrc != gss_trailerlen {
            return -(1765328194 as isize) as crate::krb5_h::krb5_error_code;
        }
        gss_headerlen = (gss_headerlen).wrapping_add(gss_trailerlen);
        gss_trailerlen = 0usize
    } else if (*trailer).buffer.length != gss_trailerlen {
        return -(1765328194 as isize) as crate::krb5_h::krb5_error_code;
    }
    if (*header).buffer.length != gss_headerlen {
        return -(1765328194 as isize) as crate::krb5_h::krb5_error_code;
    }
    kiov_count = (3i32 + iov_count) as crate::stddef_h::size_t;
    kiov = crate::stdlib::malloc(
        kiov_count.wrapping_mul(::std::mem::size_of::<crate::krb5_h::krb5_crypto_iov>()),
    ) as *mut crate::krb5_h::krb5_crypto_iov;
    if kiov.is_null() {
        return 12i32;
    }
    /*
     * The krb5 header is located at the end of the GSS header.
     */
    (*kiov.offset(i as isize)).flags = 1i32;
    (*kiov.offset(i as isize)).data.length = k5_headerlen;
    let ref mut fresh4 = (*kiov.offset(i as isize)).data.data;
    *fresh4 = ((*header).buffer.value as *mut i8)
        .offset((*header).buffer.length as isize)
        .offset(-(k5_headerlen as isize));
    i += 1;
    j = 0i32;
    while j < iov_count {
        (*kiov.offset(i as isize)).flags = kg_translate_flag_iov((*iov.offset(j as isize)).type_0);
        if !((*kiov.offset(i as isize)).flags == 0i32) {
            (*kiov.offset(i as isize)).data.length = (*iov.offset(j as isize)).buffer.length as u32;
            let ref mut fresh5 = (*kiov.offset(i as isize)).data.data;
            *fresh5 = (*iov.offset(j as isize)).buffer.value as *mut i8;
            i += 1
        }
        j += 1
    }
    /*
     * The EC and encrypted GSS header are placed in the trailer, which may
     * be rotated directly after the plaintext header if no trailer buffer
     * is provided.
     */
    (*kiov.offset(i as isize)).flags = 2i32; /* E(Header) */
    (*kiov.offset(i as isize)).data.length = ec.wrapping_add(16usize) as u32;
    if trailer.is_null() {
        let ref mut fresh6 = (*kiov.offset(i as isize)).data.data;
        *fresh6 = ((*header).buffer.value as *mut i8).offset(16isize)
    } else {
        let ref mut fresh7 = (*kiov.offset(i as isize)).data.data;
        *fresh7 = (*trailer).buffer.value as *mut i8
    }
    i += 1;
    /*
     * The krb5 trailer is placed after the encrypted copy of the
     * krb5 header (which may be in the GSS header or trailer).
     */
    (*kiov.offset(i as isize)).flags = 5i32; /* E(Header) */
    (*kiov.offset(i as isize)).data.length = k5_trailerlen;
    let ref mut fresh8 = (*kiov.offset(i as isize)).data.data;
    *fresh8 = (*kiov.offset((i - 1i32) as isize))
        .data
        .data
        .offset(ec as isize)
        .offset(16isize);
    i += 1;
    *pkiov = kiov;
    *pkiov_count = i as crate::stddef_h::size_t;
    return 0i32;
}
/* PROTO is 1 if CFX, 0 if pre-CFX */

unsafe extern "C" fn kg_translate_iov(
    mut context: crate::krb5_h::krb5_context,
    mut proto: i32,
    mut dce_style: i32,
    mut ec: crate::stddef_h::size_t,
    mut rrc: crate::stddef_h::size_t,
    mut enctype: crate::krb5_h::krb5_enctype,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
    mut pkiov: *mut *mut crate::krb5_h::krb5_crypto_iov,
    mut pkiov_count: *mut crate::stddef_h::size_t,
) -> crate::krb5_h::krb5_error_code {
    return if proto != 0 {
        kg_translate_iov_v3(
            context,
            dce_style,
            ec,
            rrc,
            enctype,
            iov,
            iov_count,
            pkiov,
            pkiov_count,
        )
    } else {
        kg_translate_iov_v1(context, enctype, iov, iov_count, pkiov, pkiov_count)
    };
}
#[no_mangle]

pub unsafe extern "C" fn kg_encrypt_iov(
    mut context: crate::krb5_h::krb5_context,
    mut proto: i32,
    mut dce_style: i32,
    mut ec: crate::stddef_h::size_t,
    mut rrc: crate::stddef_h::size_t,
    mut key: crate::krb5_h::krb5_key,
    mut usage: i32,
    mut iv: crate::krb5_h::krb5_pointer,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut state = 0 as *mut crate::krb5_h::krb5_data;
    let mut kiov_len: crate::stddef_h::size_t = 0;
    let mut kiov = 0 as *mut crate::krb5_h::krb5_crypto_iov;
    code = iv_to_state(context, key, iv, &mut state);
    if code != 0 {
        return code;
    }
    code = kg_translate_iov(
        context,
        proto,
        dce_style,
        ec,
        rrc,
        (*key).keyblock.enctype,
        iov,
        iov_count,
        &mut kiov,
        &mut kiov_len,
    );
    if code == 0i32 {
        code = crate::krb5_h::krb5_k_encrypt_iov(context, key, usage, state, kiov, kiov_len);
        crate::stdlib::free(kiov as *mut libc::c_void);
    }
    crate::krb5_h::krb5_free_data(context, state);
    return code;
}
/* length is the length of the cleartext. */
#[no_mangle]

pub unsafe extern "C" fn kg_decrypt_iov(
    mut context: crate::krb5_h::krb5_context,
    mut proto: i32,
    mut dce_style: i32,
    mut ec: crate::stddef_h::size_t,
    mut rrc: crate::stddef_h::size_t,
    mut key: crate::krb5_h::krb5_key,
    mut usage: i32,
    mut iv: crate::krb5_h::krb5_pointer,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut state = 0 as *mut crate::krb5_h::krb5_data;
    let mut kiov_len: crate::stddef_h::size_t = 0;
    let mut kiov = 0 as *mut crate::krb5_h::krb5_crypto_iov;
    code = iv_to_state(context, key, iv, &mut state);
    if code != 0 {
        return code;
    }
    code = kg_translate_iov(
        context,
        proto,
        dce_style,
        ec,
        rrc,
        (*key).keyblock.enctype,
        iov,
        iov_count,
        &mut kiov,
        &mut kiov_len,
    );
    if code == 0i32 {
        code = crate::krb5_h::krb5_k_decrypt_iov(context, key, usage, state, kiov, kiov_len);
        crate::stdlib::free(kiov as *mut libc::c_void);
    }
    crate::krb5_h::krb5_free_data(context, state);
    return code;
}
#[no_mangle]

pub unsafe extern "C" fn kg_arcfour_docrypt_iov(
    mut context: crate::krb5_h::krb5_context,
    mut keyblock: *const crate::krb5_h::krb5_keyblock,
    mut usage: i32,
    mut kd_data: *const u8,
    mut kd_data_len: crate::stddef_h::size_t,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut kd = make_data(kd_data as *mut libc::c_void, kd_data_len as u32);
    let mut kiov = 0 as *mut crate::krb5_h::krb5_crypto_iov;
    let mut kiov_len = 0usize;
    code = kg_translate_iov(
        context,
        0i32,
        0i32,
        0usize,
        0usize,
        (*keyblock).enctype,
        iov,
        iov_count,
        &mut kiov,
        &mut kiov_len,
    );
    if code != 0 {
        return code;
    }
    code = crate::k5_int_h::krb5int_arcfour_gsscrypt(keyblock, usage, &mut kd, kiov, kiov_len);
    crate::stdlib::free(kiov as *mut libc::c_void);
    return code;
}
#[no_mangle]

pub unsafe extern "C" fn kg_translate_flag_iov(
    mut type_0: crate::gssapi_h::OM_uint32,
) -> crate::krb5_h::krb5_cryptotype {
    let mut ktype: crate::krb5_h::krb5_cryptotype = 0;
    match type_0 & !(0xffff0000u32) {
        1 | 9 => ktype = 2i32,
        11 => ktype = 3i32,
        _ => ktype = 0i32,
    }
    return ktype;
}
#[no_mangle]

pub unsafe extern "C" fn kg_locate_iov(
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
    mut type_0: crate::gssapi_h::OM_uint32,
) -> crate::gssapi_ext_h::gss_iov_buffer_t {
    let mut i: i32 = 0;
    let mut p = 0 as crate::gssapi_ext_h::gss_iov_buffer_t;
    if iov.is_null() {
        return 0 as crate::gssapi_ext_h::gss_iov_buffer_t;
    }
    i = iov_count - 1i32;
    while i >= 0i32 {
        if (*iov.offset(i as isize)).type_0 & !(0xffff0000u32) == type_0 {
            if p.is_null() {
                p = &mut *iov.offset(i as isize) as *mut crate::gssapi_ext_h::gss_iov_buffer_desc
            } else {
                return 0 as crate::gssapi_ext_h::gss_iov_buffer_t;
            }
        }
        i -= 1
    }
    return p;
}
/* Return the IOV where the GSSAPI token header should be placed (and possibly
 * the checksum as well, depending on the token type). */
#[no_mangle]

pub unsafe extern "C" fn kg_locate_header_iov(
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
    mut toktype: i32,
) -> crate::gssapi_ext_h::gss_iov_buffer_t {
    if toktype == 0x101i32 {
        return kg_locate_iov(iov, iov_count, 12u32);
    } else {
        return kg_locate_iov(iov, iov_count, 2u32);
    };
}
#[no_mangle]

pub unsafe extern "C" fn kg_iov_msglen(
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
    mut data_length_p: *mut crate::stddef_h::size_t,
    mut assoc_data_length_p: *mut crate::stddef_h::size_t,
) {
    let mut i: i32 = 0;
    let mut data_length = 0usize;
    let mut assoc_data_length = 0usize;
    if !iov.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"iov != GSS_C_NO_IOV_BUFFER\x00" as *const u8 as *const i8,
            b"util_crypt.c\x00" as *const u8 as *const i8,
            604u32,
            (*::std::mem::transmute::<&[u8; 67], &[i8; 67]>(
                b"void kg_iov_msglen(gss_iov_buffer_desc *, int, size_t *, size_t *)\x00",
            ))
            .as_ptr(),
        );
    }
    *assoc_data_length_p = 0usize;
    *data_length_p = *assoc_data_length_p;
    i = 0i32;
    while i < iov_count {
        let mut type_0 = (*iov.offset(i as isize)).type_0 & !(0xffff0000u32);
        if type_0 == 11u32 {
            assoc_data_length =
                (assoc_data_length).wrapping_add((*iov.offset(i as isize)).buffer.length)
        }
        if type_0 == 1u32 || type_0 == 11u32 {
            data_length = (data_length).wrapping_add((*iov.offset(i as isize)).buffer.length)
        }
        i += 1
    }
    *data_length_p = data_length;
    *assoc_data_length_p = assoc_data_length;
}
#[no_mangle]

pub unsafe extern "C" fn kg_release_iov(
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
) {
    let mut i: i32 = 0;
    if !iov.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"iov != GSS_C_NO_IOV_BUFFER\x00" as *const u8 as *const i8,
            b"util_crypt.c\x00" as *const u8 as *const i8,
            628u32,
            (*::std::mem::transmute::<&[u8; 48], &[i8; 48]>(
                b"void kg_release_iov(gss_iov_buffer_desc *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    i = 0i32;
    while i < iov_count {
        if (*iov.offset(i as isize)).type_0 & 0x20000u32 != 0 {
            gssalloc_free((*iov.offset(i as isize)).buffer.value);
            (*iov.offset(i as isize)).buffer.length = 0usize;
            let ref mut fresh9 = (*iov.offset(i as isize)).buffer.value;
            *fresh9 = 0 as *mut libc::c_void;
            let ref mut fresh10 = (*iov.offset(i as isize)).type_0;
            *fresh10 &= !(0x20000i32) as u32
        }
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn kg_fixup_padding_iov(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut padding = 0 as crate::gssapi_ext_h::gss_iov_buffer_t;
    let mut data = 0 as crate::gssapi_ext_h::gss_iov_buffer_t;
    let mut padlength: crate::stddef_h::size_t = 0;
    let mut relative_padlength: crate::stddef_h::size_t = 0;
    let mut p = 0 as *mut u8;
    data = kg_locate_iov(iov, iov_count, 1u32);
    padding = kg_locate_iov(iov, iov_count, 9u32);
    if data.is_null() {
        *minor_status = 0u32;
        return 0u32;
    }
    if padding.is_null() || (*padding).buffer.length == 0usize {
        *minor_status = 22u32;
        return (13u32) << 16i32;
    }
    p = (*padding).buffer.value as *mut u8;
    padlength = *p.offset((*padding).buffer.length.wrapping_sub(1usize) as isize)
        as crate::stddef_h::size_t;
    if (*data).buffer.length.wrapping_add((*padding).buffer.length) < padlength
        || padlength == 0usize
    {
        *minor_status = -(1765328194 as isize) as crate::gssapi_h::OM_uint32;
        return (9u32) << 16i32;
    }
    /*
     * kg_unseal_stream_iov() will place one byte of padding in the
     * padding buffer; its true value is unknown until after decryption.
     *
     * relative_padlength contains the number of bytes to compensate the
     * padding and data buffers by; it will be zero if the caller manages
     * the padding length.
     *
     * If the caller manages the padding length, then relative_padlength
     * wil be zero.
     *
     * eg. if the buffers are structured as follows:
     *
     *      +---DATA---+-PAD-+
     *      | ABCDE444 | 4   |
     *      +----------+-----+
     *
     * after compensation they would look like:
     *
     *      +-DATA--+-PAD--+
     *      | ABCDE | NULL |
     *      +-------+------+
     */
    relative_padlength = padlength.wrapping_sub((*padding).buffer.length);
    if (*data).buffer.length >= relative_padlength {
    } else {
        crate::stdlib::__assert_fail(
            b"data->buffer.length >= relative_padlength\x00" as *const u8 as *const i8,
            b"util_crypt.c\x00" as *const u8 as *const i8,
            696u32,
            (*::std::mem::transmute::<&[u8; 72], &[i8; 72]>(
                b"OM_uint32 kg_fixup_padding_iov(OM_uint32 *, gss_iov_buffer_desc *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    (*data).buffer.length = ((*data).buffer.length).wrapping_sub(relative_padlength);
    kg_release_iov(padding, 1i32);
    (*padding).buffer.length = 0usize;
    (*padding).buffer.value = 0 as *mut libc::c_void;
    return 0u32;
}
#[no_mangle]

pub unsafe extern "C" fn kg_integ_only_iov(
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
) -> crate::krb5_h::krb5_boolean {
    let mut i: i32 = 0;
    let mut has_conf_data = 0u32;
    if !iov.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"iov != GSS_C_NO_IOV_BUFFER\x00" as *const u8 as *const i8,
            b"util_crypt.c\x00" as *const u8 as *const i8,
            713u32,
            (*::std::mem::transmute::<&[u8; 59], &[i8; 59]>(
                b"krb5_boolean kg_integ_only_iov(gss_iov_buffer_desc *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    i = 0i32;
    while i < iov_count {
        if (*iov.offset(i as isize)).type_0 & !(0xffff0000u32) == 1u32 {
            has_conf_data = 1u32;
            break;
        } else {
            i += 1
        }
    }
    return (has_conf_data == 0u32) as crate::krb5_h::krb5_boolean;
}
#[no_mangle]

pub unsafe extern "C" fn kg_allocate_iov(
    mut iov: crate::gssapi_ext_h::gss_iov_buffer_t,
    mut size: crate::stddef_h::size_t,
) -> crate::krb5_h::krb5_error_code {
    if !iov.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"iov != GSS_C_NO_IOV_BUFFER\x00" as *const u8 as *const i8,
            b"util_crypt.c\x00" as *const u8 as *const i8,
            728u32,
            (*::std::mem::transmute::<&[u8; 58], &[i8; 58]>(
                b"krb5_error_code kg_allocate_iov(gss_iov_buffer_t, size_t)\x00",
            ))
            .as_ptr(),
        );
    }
    if (*iov).type_0 & 0x10000u32 != 0 {
    } else {
        crate::stdlib::__assert_fail(
            b"iov->type & GSS_IOV_BUFFER_FLAG_ALLOCATE\x00" as *const u8 as *const i8,
            b"util_crypt.c\x00" as *const u8 as *const i8,
            729u32,
            (*::std::mem::transmute::<&[u8; 58], &[i8; 58]>(
                b"krb5_error_code kg_allocate_iov(gss_iov_buffer_t, size_t)\x00",
            ))
            .as_ptr(),
        );
    }
    (*iov).buffer.length = size;
    (*iov).buffer.value = gssalloc_malloc(size);
    if (*iov).buffer.value.is_null() {
        (*iov).buffer.length = 0usize;
        return 12i32;
    }
    (*iov).type_0 |= 0x20000u32;
    return 0i32;
}
