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

/* _GSSAPI_KRB5_H_ */

/* __cplusplus */
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__gid_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__pthread_internal_list;
pub use crate::stdlib::__pthread_list_t;
pub use crate::stdlib::__pthread_mutex_s;
pub use crate::stdlib::__ssize_t;
pub use crate::stdlib::__uid_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::pthread_mutex_t;
pub use crate::stdlib::ssize_t;
pub use crate::stdlib::uid_t;

pub use crate::k5_err_h::errinfo;
pub use crate::k5_int_h::_kdb5_dal_handle;
pub use crate::k5_int_h::_kdb_log_context;
pub use crate::k5_int_h::_krb5_authdata_context;
pub use crate::k5_int_h::_krb5_authdata_context_module;
pub use crate::k5_int_h::_krb5_context;
pub use crate::k5_int_h::_krb5_os_context;
pub use crate::k5_int_h::ccselect_module_handle;
pub use crate::k5_int_h::dns_canonhost;
pub use crate::k5_int_h::hostrealm_module_handle;
pub use crate::k5_int_h::k5_tls_vtable_st;
pub use crate::k5_int_h::kdb5_dal_handle;
pub use crate::k5_int_h::krb5_authdata_context;
pub use crate::k5_int_h::krb5_authdata_context_free;
pub use crate::k5_int_h::krb5_authdata_context_init;
pub use crate::k5_int_h::krb5_authdata_import_attributes;
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
pub use crate::krb5_h::_krb5_address;
pub use crate::krb5_h::_krb5_ap_req;
pub use crate::krb5_h::_krb5_auth_context;
pub use crate::krb5_h::_krb5_authdata;
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
pub use crate::krb5_h::krb5_anonymous_principal;
pub use crate::krb5_h::krb5_ap_req;
pub use crate::krb5_h::krb5_auth_context;
pub use crate::krb5_h::krb5_authdata;
pub use crate::krb5_h::krb5_authdatatype;
pub use crate::krb5_h::krb5_boolean;
pub use crate::krb5_h::krb5_const_principal;
pub use crate::krb5_h::krb5_context;
pub use crate::krb5_h::krb5_copy_principal;
pub use crate::krb5_h::krb5_data;
pub use crate::krb5_h::krb5_deltat;
pub use crate::krb5_h::krb5_enc_data;
pub use crate::krb5_h::krb5_enc_tkt_part;
pub use crate::krb5_h::krb5_enctype;
pub use crate::krb5_h::krb5_error_code;
pub use crate::krb5_h::krb5_flags;
pub use crate::krb5_h::krb5_free_context;
pub use crate::krb5_h::krb5_free_principal;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_keyblock;
pub use crate::krb5_h::krb5_kvno;
pub use crate::krb5_h::krb5_magic;
pub use crate::krb5_h::krb5_octet;
pub use crate::krb5_h::krb5_parse_name_flags;
pub use crate::krb5_h::krb5_post_recv_fn;
pub use crate::krb5_h::krb5_pre_send_fn;
pub use crate::krb5_h::krb5_principal;
pub use crate::krb5_h::krb5_principal_data;
pub use crate::krb5_h::krb5_prompt_type;
pub use crate::krb5_h::krb5_sname_to_principal;
pub use crate::krb5_h::krb5_ticket;
pub use crate::krb5_h::krb5_ticket_times;
pub use crate::krb5_h::krb5_timestamp;
pub use crate::krb5_h::krb5_trace_callback;
pub use crate::krb5_h::krb5_trace_info;
pub use crate::krb5_h::krb5_transited;
pub use crate::profile_h::profile_t;
pub use crate::src::krb5::import_name::k5_int_h::k5alloc;
pub use crate::src::krb5::import_name::k5_int_h::k5calloc;
pub use crate::src::krb5::import_name::k5_int_h::k5memdup0;
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
pub use crate::gssapiP_krb5_h::_krb5_gss_name_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_name_t;
pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_name_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::mglueP_h::gss_name_struct;

pub use crate::src::generic::gssapi_generic::GSS_C_NT_ANONYMOUS;

pub use crate::src::krb5::disp_status::krb5_gss_save_error_info;

pub use crate::src::krb5::init_sec_context::krb5_gss_init_context;
pub use crate::src::krb5::naming_exts::kg_init_name;

pub use crate::stdlib::getpwuid_r;

pub use crate::stdlib::passwd;

/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
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
 * $Id$
 */
/*
 * errors:
 * GSS_S_BAD_NAMETYPE   if the type is bogus
 * GSS_S_BAD_NAME       if the type is good but the name is bogus
 * GSS_S_FAILURE        if memory allocation fails
 */
/*
 * Import serialized authdata context
 */

unsafe extern "C" fn import_name_composite(
    mut context: crate::krb5_h::krb5_context,
    mut enc_data: *mut u8,
    mut enc_length: crate::stddef_h::size_t,
    mut pad_context: *mut crate::k5_int_h::krb5_authdata_context,
) -> crate::krb5_h::krb5_error_code {
    let mut ad_context = 0 as *mut crate::k5_int_h::_krb5_authdata_context;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut data = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    if enc_length == 0usize {
        return 0i32;
    }
    code = crate::k5_int_h::krb5_authdata_context_init(context, &mut ad_context);
    if code != 0i32 {
        return code;
    }
    data.data = enc_data as *mut i8;
    data.length = enc_length as u32;
    code =
        crate::k5_int_h::krb5_authdata_import_attributes(context, ad_context, 0x2fi32, &mut data);
    if code != 0i32 {
        crate::k5_int_h::krb5_authdata_context_free(context, ad_context);
        return code;
    }
    *pad_context = ad_context;
    return 0i32;
}
/* Split a host-based name "service[@host]" into allocated strings
 * placed in *service_out and *host_out (possibly NULL). */

unsafe extern "C" fn parse_hostbased(
    mut str: *const i8,
    mut len: crate::stddef_h::size_t,
    mut service_out: *mut *mut i8,
    mut host_out: *mut *mut i8,
) -> crate::krb5_h::krb5_error_code {
    let mut at = 0 as *const i8;
    let mut servicelen: crate::stddef_h::size_t = 0;
    let mut hostlen: crate::stddef_h::size_t = 0;
    let mut service = 0 as *mut i8;
    let mut host = 0 as *mut i8;
    *host_out = 0 as *mut i8;
    *service_out = *host_out;
    /* Find the bound of the service name and copy it. */
    at = crate::stdlib::memchr(str as *const libc::c_void, '@' as i32, len) as *const i8;
    servicelen = if at.is_null() {
        len
    } else {
        at.wrapping_offset_from(str) as crate::stddef_h::size_t
    };
    service = crate::stdlib::malloc(servicelen.wrapping_add(1usize)) as *mut i8;
    if service.is_null() {
        return 12i32;
    }
    crate::stdlib::memcpy(
        service as *mut libc::c_void,
        str as *const libc::c_void,
        servicelen,
    );
    *service.offset(servicelen as isize) = '\u{0}' as i8;
    /* If present, copy the hostname. */
    if !at.is_null() {
        hostlen = len.wrapping_sub(servicelen).wrapping_sub(1usize);
        host = crate::stdlib::malloc(hostlen.wrapping_add(1usize)) as *mut i8;
        if host.is_null() {
            crate::stdlib::free(service as *mut libc::c_void);
            return 12i32;
        }
        crate::stdlib::memcpy(
            host as *mut libc::c_void,
            at.offset(1isize) as *const libc::c_void,
            hostlen,
        );
        *host.offset(hostlen as isize) = '\u{0}' as i8
    }
    *service_out = service;
    *host_out = host;
    return 0i32;
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
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_import_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut input_name_buffer: crate::gssapi_h::gss_buffer_t,
    mut input_name_type: crate::gssapi_h::gss_OID,
    mut output_name: *mut crate::gssapi_h::gss_name_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut context = 0 as *mut crate::k5_int_h::_krb5_context;
    let mut princ = 0 as crate::krb5_h::krb5_principal;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut cp = 0 as *mut u8;
    let mut end = 0 as *mut u8;
    let mut tmp = 0 as *mut i8;
    let mut tmp2 = 0 as *mut i8;
    let mut service = 0 as *mut i8;
    let mut host = 0 as *mut i8;
    let mut stringrep = 0 as *mut i8;
    let mut length: crate::stdlib::ssize_t = 0;
    let mut pw = 0 as *mut crate::stdlib::passwd;
    let mut is_composite = 0i32;
    let mut ad_context = 0 as crate::k5_int_h::krb5_authdata_context;
    let mut status = (13u32) << 16i32;
    let mut name = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_name_rec;
    let mut flags = 0i32;
    *output_name = 0 as crate::gssapi_h::gss_name_t;
    *minor_status = 0u32;
    code = crate::src::krb5::init_sec_context::krb5_gss_init_context(&mut context);
    if !(code != 0) {
        if !input_name_type.is_null()
            && ((*input_name_type).length
                == (*crate::src::generic::gssapi_generic::gss_nt_service_name).length
                && crate::stdlib::memcmp(
                    (*input_name_type).elements,
                    (*crate::src::generic::gssapi_generic::gss_nt_service_name).elements,
                    (*input_name_type).length as usize,
                ) == 0i32
                || (*input_name_type).length
                    == (*crate::src::generic::gssapi_generic::gss_nt_service_name_v2).length
                    && crate::stdlib::memcmp(
                        (*input_name_type).elements,
                        (*crate::src::generic::gssapi_generic::gss_nt_service_name_v2).elements,
                        (*input_name_type).length as usize,
                    ) == 0i32)
        {
            /* Split the name into service and host (or NULL). */
            code = parse_hostbased(
                (*input_name_buffer).value as *const i8,
                (*input_name_buffer).length,
                &mut service,
                &mut host,
            );
            if code != 0 {
                current_block = 5854819196614252297;
            } else {
                /*
                 * Compute the initiator target name.  In some cases this is a waste of
                 * getaddrinfo/getnameinfo queries, but computing the name when we need
                 * it would require a lot of code changes.
                 */
                code = crate::krb5_h::krb5_sname_to_principal(
                    context, host, service, 3i32, &mut princ,
                );
                if code != 0 {
                    current_block = 5854819196614252297;
                } else {
                    current_block = 12079920068676227593;
                }
            }
        } else if !input_name_type.is_null()
            && ((*input_name_type).length
                == (*crate::src::krb5::gssapi_krb5::gss_nt_krb5_principal).length
                && crate::stdlib::memcmp(
                    (*input_name_type).elements,
                    (*crate::src::krb5::gssapi_krb5::gss_nt_krb5_principal).elements,
                    (*input_name_type).length as usize,
                ) == 0i32)
        {
            let mut input = 0 as *mut crate::krb5_h::krb5_principal_data;
            if (*input_name_buffer).length != ::std::mem::size_of::<crate::krb5_h::krb5_principal>()
            {
                code = -(2045022970 as isize) as crate::krb5_h::krb5_error_code;
                status = (2u32) << 16i32;
                current_block = 5854819196614252297;
            } else {
                input = *((*input_name_buffer).value as *mut crate::krb5_h::krb5_principal);
                code = crate::krb5_h::krb5_copy_principal(
                    context,
                    input as crate::krb5_h::krb5_const_principal,
                    &mut princ,
                );
                if code != 0 {
                    current_block = 5854819196614252297;
                } else {
                    current_block = 12079920068676227593;
                }
            }
        } else if !input_name_type.is_null()
            && ((*input_name_type).length
                == (*crate::src::generic::gssapi_generic::GSS_C_NT_ANONYMOUS).length
                && crate::stdlib::memcmp(
                    (*input_name_type).elements,
                    (*crate::src::generic::gssapi_generic::GSS_C_NT_ANONYMOUS).elements,
                    (*input_name_type).length as usize,
                ) == 0i32)
        {
            code = crate::krb5_h::krb5_copy_principal(
                context,
                crate::krb5_h::krb5_anonymous_principal(),
                &mut princ,
            );
            if code != 0 {
                current_block = 5854819196614252297;
            } else {
                current_block = 12079920068676227593;
            }
        } else {
            let mut uid: crate::stdlib::uid_t = 0;
            let mut pwx = crate::stdlib::passwd {
                pw_name: 0 as *mut i8,
                pw_passwd: 0 as *mut i8,
                pw_uid: 0,
                pw_gid: 0,
                pw_gecos: 0 as *mut i8,
                pw_dir: 0 as *mut i8,
                pw_shell: 0 as *mut i8,
            };
            let mut pwbuf: [i8; 8192] = [0; 8192];
            stringrep = 0 as *mut i8;
            tmp = k5memdup0(
                (*input_name_buffer).value,
                (*input_name_buffer).length,
                &mut code,
            ) as *mut i8;
            if tmp.is_null() {
                current_block = 5854819196614252297;
            } else {
                tmp2 = 0 as *mut i8;
                /* Find the appropriate string rep to pass into parse_name. */
                if input_name_type.is_null()
                    || (*input_name_type).length
                        == (*crate::src::krb5::gssapi_krb5::gss_nt_krb5_name).length
                        && crate::stdlib::memcmp(
                            (*input_name_type).elements,
                            (*crate::src::krb5::gssapi_krb5::gss_nt_krb5_name).elements,
                            (*input_name_type).length as usize,
                        ) == 0i32
                    || (*input_name_type).length
                        == (*crate::src::generic::gssapi_generic::gss_nt_user_name).length
                        && crate::stdlib::memcmp(
                            (*input_name_type).elements,
                            (*crate::src::generic::gssapi_generic::gss_nt_user_name).elements,
                            (*input_name_type).length as usize,
                        ) == 0i32
                {
                    stringrep = tmp;
                    current_block = 2652804691515851435;
                } else if (*input_name_type).length
                    == (*crate::src::krb5::gssapi_krb5::GSS_KRB5_NT_ENTERPRISE_NAME).length
                    && crate::stdlib::memcmp(
                        (*input_name_type).elements,
                        (*crate::src::krb5::gssapi_krb5::GSS_KRB5_NT_ENTERPRISE_NAME).elements,
                        (*input_name_type).length as usize,
                    ) == 0i32
                {
                    stringrep = tmp;
                    flags |= 0x4i32;
                    current_block = 2652804691515851435;
                } else {
                    if (*input_name_type).length
                        == (*crate::src::generic::gssapi_generic::gss_nt_machine_uid_name).length
                        && crate::stdlib::memcmp(
                            (*input_name_type).elements,
                            (*crate::src::generic::gssapi_generic::gss_nt_machine_uid_name)
                                .elements,
                            (*input_name_type).length as usize,
                        ) == 0i32
                    {
                        uid = *((*input_name_buffer).value as *mut crate::stdlib::uid_t);
                        current_block = 5042286318545367694;
                    } else if (*input_name_type).length
                        == (*crate::src::generic::gssapi_generic::gss_nt_string_uid_name).length
                        && crate::stdlib::memcmp(
                            (*input_name_type).elements,
                            (*crate::src::generic::gssapi_generic::gss_nt_string_uid_name).elements,
                            (*input_name_type).length as usize,
                        ) == 0i32
                    {
                        uid = crate::stdlib::atoi(tmp) as crate::stdlib::uid_t;
                        current_block = 5042286318545367694;
                    } else if (*input_name_type).length
                        == (*crate::src::generic::gssapi_generic::gss_nt_exported_name).length
                        && crate::stdlib::memcmp(
                            (*input_name_type).elements,
                            (*crate::src::generic::gssapi_generic::gss_nt_exported_name).elements,
                            (*input_name_type).length as usize,
                        ) == 0i32
                        || (*input_name_type).length
                            == (*crate::src::generic::gssapi_generic::GSS_C_NT_COMPOSITE_EXPORT)
                                .length
                            && crate::stdlib::memcmp(
                                (*input_name_type).elements,
                                (*crate::src::generic::gssapi_generic::GSS_C_NT_COMPOSITE_EXPORT)
                                    .elements,
                                (*input_name_type).length as usize,
                            ) == 0i32
                    {
                        cp = tmp as *mut u8;
                        end = cp.offset((*input_name_buffer).length as isize);
                        if (end.wrapping_offset_from(cp)) < 2isize {
                            current_block = 16859350023670137067;
                        } else {
                            let fresh0 = cp;
                            cp = cp.offset(1);
                            if *fresh0 as i32 != 0x4i32 {
                                current_block = 16859350023670137067;
                            } else {
                                let fresh1 = cp;
                                cp = cp.offset(1);
                                match *fresh1 as i32 {
                                    1 => {
                                        current_block = 6072622540298447352;
                                        match current_block {
                                            2073638450139208143 => is_composite += 1,
                                            _ => {}
                                        }
                                        if (end.wrapping_offset_from(cp)) < 2isize {
                                            current_block = 16859350023670137067;
                                        } else {
                                            let fresh2 = cp;
                                            cp = cp.offset(1);
                                            if *fresh2 as i32 != 0i32 {
                                                current_block = 16859350023670137067;
                                            } else {
                                                let fresh3 = cp;
                                                cp = cp.offset(1);
                                                length = *fresh3 as crate::stdlib::ssize_t;
                                                if length !=
                                                       (*crate::src::krb5::gssapi_krb5::gss_mech_krb5).length
                                                           as crate::stdlib::ssize_t +
                                                           2isize {
                                                    current_block =
                                                        16859350023670137067;
                                                } else if (end.wrapping_offset_from(cp)) <
                                                              2isize
                                                 {
                                                    current_block =
                                                        16859350023670137067;
                                                } else {
                                                    let fresh4 = cp;
                                                    cp = cp.offset(1);
                                                    if *fresh4 as i32
                                                           !=
                                                           0x6i32
                                                       {
                                                        current_block =
                                                            16859350023670137067;
                                                    } else {
                                                        let fresh5 = cp;
                                                        cp = cp.offset(1);
                                                        length =
                                                            *fresh5 as
                                                                crate::stdlib::ssize_t;
                                                        if length !=
                                                               (*crate::src::krb5::gssapi_krb5::gss_mech_krb5).length
                                                                   as crate::stdlib::ssize_t
                                                           {
                                                            current_block =
                                                                16859350023670137067;
                                                        } else if (end.wrapping_offset_from(cp))
                                                                      < length
                                                         {
                                                            current_block =
                                                                16859350023670137067;
                                                        } else if crate::stdlib::memcmp(cp as
                                                                             *const libc::c_void,
                                                                         (*crate::src::krb5::gssapi_krb5::gss_mech_krb5).elements,
                                                                         length
                                                                             as
                                                                             usize)
                                                                      !=
                                                                      0i32
                                                         {
                                                            current_block =
                                                                16859350023670137067;
                                                        } else {
                                                            cp =
                                                                cp.offset(length);
                                                            if (end.wrapping_offset_from(cp))
                                                                   <
                                                                   4isize
                                                               {
                                                                current_block
                                                                    =
                                                                    16859350023670137067;
                                                            } else {
                                                                let fresh6 =
                                                                    cp;
                                                                cp =
                                                                    cp.offset(1);
                                                                length =
                                                                    *fresh6 as
                                                                        crate::stdlib::ssize_t;
                                                                let fresh7 =
                                                                    cp;
                                                                cp =
                                                                    cp.offset(1);
                                                                length =
                                                                    length <<
                                                                        8i32
                                                                        |
                                                                        *fresh7
                                                                            as
                                                                            isize;
                                                                let fresh8 =
                                                                    cp;
                                                                cp =
                                                                    cp.offset(1);
                                                                length =
                                                                    length <<
                                                                        8i32
                                                                        |
                                                                        *fresh8
                                                                            as
                                                                            isize;
                                                                let fresh9 =
                                                                    cp;
                                                                cp =
                                                                    cp.offset(1);
                                                                length =
                                                                    length <<
                                                                        8i32
                                                                        |
                                                                        *fresh9
                                                                            as
                                                                            isize;
                                                                if (end.wrapping_offset_from(cp))
                                                                       <
                                                                       length
                                                                   {
                                                                    current_block
                                                                        =
                                                                        16859350023670137067;
                                                                } else {
                                                                    tmp2 =
                                                                        k5alloc((length
                                                                                     +
                                                                                     1isize)
                                                                                    as
                                                                                    crate::stddef_h::size_t,
                                                                                &mut code)
                                                                            as
                                                                            *mut i8;
                                                                    if tmp2.is_null()
                                                                       {
                                                                        current_block
                                                                            =
                                                                            5854819196614252297;
                                                                    } else {
                                                                        crate::stdlib::strncpy(tmp2,
                                                                                cp
                                                                                    as
                                                                                    *mut i8,
                                                                                length
                                                                                    as
                                                                                    usize);
                                                                        *tmp2.offset(length)
                                                                            =
                                                                            0i8;
                                                                        stringrep
                                                                            =
                                                                            tmp2;
                                                                        cp =
                                                                            cp.offset(length);
                                                                        if is_composite
                                                                               !=
                                                                               0
                                                                           {
                                                                            if (end.wrapping_offset_from(cp))
                                                                                   <
                                                                                   4isize
                                                                               {
                                                                                current_block
                                                                                    =
                                                                                    16859350023670137067;
                                                                            } else {
                                                                                let fresh10 =
                                                                                    cp;
                                                                                cp
                                                                                    =
                                                                                    cp.offset(1);
                                                                                length
                                                                                    =
                                                                                    *fresh10
                                                                                        as
                                                                                        crate::stdlib::ssize_t;
                                                                                let fresh11 =
                                                                                    cp;
                                                                                cp
                                                                                    =
                                                                                    cp.offset(1);
                                                                                length
                                                                                    =
                                                                                    length
                                                                                        <<
                                                                                        8i32
                                                                                        |
                                                                                        *fresh11
                                                                                            as
                                                                                            isize;
                                                                                let fresh12 =
                                                                                    cp;
                                                                                cp
                                                                                    =
                                                                                    cp.offset(1);
                                                                                length
                                                                                    =
                                                                                    length
                                                                                        <<
                                                                                        8i32
                                                                                        |
                                                                                        *fresh12
                                                                                            as
                                                                                            isize;
                                                                                let fresh13 =
                                                                                    cp;
                                                                                cp
                                                                                    =
                                                                                    cp.offset(1);
                                                                                length
                                                                                    =
                                                                                    length
                                                                                        <<
                                                                                        8i32
                                                                                        |
                                                                                        *fresh13
                                                                                            as
                                                                                            isize;
                                                                                if (end.wrapping_offset_from(cp))
                                                                                       <
                                                                                       length
                                                                                   {
                                                                                    current_block
                                                                                        =
                                                                                        16859350023670137067;
                                                                                } else {
                                                                                    code
                                                                                        =
                                                                                        import_name_composite(context,
                                                                                                              cp,
                                                                                                              length
                                                                                                                  as
                                                                                                                  crate::stddef_h::size_t,
                                                                                                              &mut ad_context);
                                                                                    if code
                                                                                           !=
                                                                                           0i32
                                                                                       {
                                                                                        current_block
                                                                                            =
                                                                                            16859350023670137067;
                                                                                    } else {
                                                                                        cp
                                                                                            =
                                                                                            cp.offset(length);
                                                                                        current_block
                                                                                            =
                                                                                            13003737910779602957;
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            current_block
                                                                                =
                                                                                13003737910779602957;
                                                                        }
                                                                        match current_block
                                                                            {
                                                                            16859350023670137067
                                                                            =>
                                                                            {
                                                                            }
                                                                            _
                                                                            =>
                                                                            {
                                                                                if cp
                                                                                       ==
                                                                                       end
                                                                                   {
                                                                                } else {
                                                                                    crate::stdlib::__assert_fail(b"cp == end\x00"
                                                                                                      as
                                                                                                      *const u8
                                                                                                      as
                                                                                                      *const i8,
                                                                                                  b"import_name.c\x00"
                                                                                                      as
                                                                                                      *const u8
                                                                                                      as
                                                                                                      *const i8,
                                                                                                  295u32,
                                                                                                  (*::std::mem::transmute::<&[u8; 33],
                                                                                                                            &[i8; 33]>(b"OM_uint32 krb5_gss_import_name()\x00")).as_ptr());
                                                                                }
                                                                                current_block
                                                                                    =
                                                                                    2652804691515851435;
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
                                    2 => {
                                        current_block = 2073638450139208143;
                                        match current_block {
                                            2073638450139208143 => is_composite += 1,
                                            _ => {}
                                        }
                                        if (end.wrapping_offset_from(cp)) < 2isize {
                                            current_block = 16859350023670137067;
                                        } else {
                                            let fresh2 = cp;
                                            cp = cp.offset(1);
                                            if *fresh2 as i32 != 0i32 {
                                                current_block = 16859350023670137067;
                                            } else {
                                                let fresh3 = cp;
                                                cp = cp.offset(1);
                                                length = *fresh3 as crate::stdlib::ssize_t;
                                                if length !=
                                                       (*crate::src::krb5::gssapi_krb5::gss_mech_krb5).length
                                                           as crate::stdlib::ssize_t +
                                                           2isize {
                                                    current_block =
                                                        16859350023670137067;
                                                } else if (end.wrapping_offset_from(cp)) <
                                                              2isize
                                                 {
                                                    current_block =
                                                        16859350023670137067;
                                                } else {
                                                    let fresh4 = cp;
                                                    cp = cp.offset(1);
                                                    if *fresh4 as i32
                                                           !=
                                                           0x6i32
                                                       {
                                                        current_block =
                                                            16859350023670137067;
                                                    } else {
                                                        let fresh5 = cp;
                                                        cp = cp.offset(1);
                                                        length =
                                                            *fresh5 as
                                                                crate::stdlib::ssize_t;
                                                        if length !=
                                                               (*crate::src::krb5::gssapi_krb5::gss_mech_krb5).length
                                                                   as crate::stdlib::ssize_t
                                                           {
                                                            current_block =
                                                                16859350023670137067;
                                                        } else if (end.wrapping_offset_from(cp))
                                                                      < length
                                                         {
                                                            current_block =
                                                                16859350023670137067;
                                                        } else if crate::stdlib::memcmp(cp as
                                                                             *const libc::c_void,
                                                                         (*crate::src::krb5::gssapi_krb5::gss_mech_krb5).elements,
                                                                         length
                                                                             as
                                                                             usize)
                                                                      !=
                                                                      0i32
                                                         {
                                                            current_block =
                                                                16859350023670137067;
                                                        } else {
                                                            cp =
                                                                cp.offset(length);
                                                            if (end.wrapping_offset_from(cp))
                                                                   <
                                                                   4isize
                                                               {
                                                                current_block
                                                                    =
                                                                    16859350023670137067;
                                                            } else {
                                                                let fresh6 =
                                                                    cp;
                                                                cp =
                                                                    cp.offset(1);
                                                                length =
                                                                    *fresh6 as
                                                                        crate::stdlib::ssize_t;
                                                                let fresh7 =
                                                                    cp;
                                                                cp =
                                                                    cp.offset(1);
                                                                length =
                                                                    length <<
                                                                        8i32
                                                                        |
                                                                        *fresh7
                                                                            as
                                                                            isize;
                                                                let fresh8 =
                                                                    cp;
                                                                cp =
                                                                    cp.offset(1);
                                                                length =
                                                                    length <<
                                                                        8i32
                                                                        |
                                                                        *fresh8
                                                                            as
                                                                            isize;
                                                                let fresh9 =
                                                                    cp;
                                                                cp =
                                                                    cp.offset(1);
                                                                length =
                                                                    length <<
                                                                        8i32
                                                                        |
                                                                        *fresh9
                                                                            as
                                                                            isize;
                                                                if (end.wrapping_offset_from(cp))
                                                                       <
                                                                       length
                                                                   {
                                                                    current_block
                                                                        =
                                                                        16859350023670137067;
                                                                } else {
                                                                    tmp2 =
                                                                        k5alloc((length
                                                                                     +
                                                                                     1isize)
                                                                                    as
                                                                                    crate::stddef_h::size_t,
                                                                                &mut code)
                                                                            as
                                                                            *mut i8;
                                                                    if tmp2.is_null()
                                                                       {
                                                                        current_block
                                                                            =
                                                                            5854819196614252297;
                                                                    } else {
                                                                        crate::stdlib::strncpy(tmp2,
                                                                                cp
                                                                                    as
                                                                                    *mut i8,
                                                                                length
                                                                                    as
                                                                                    usize);
                                                                        *tmp2.offset(length)
                                                                            =
                                                                            0i8;
                                                                        stringrep
                                                                            =
                                                                            tmp2;
                                                                        cp =
                                                                            cp.offset(length);
                                                                        if is_composite
                                                                               !=
                                                                               0
                                                                           {
                                                                            if (end.wrapping_offset_from(cp))
                                                                                   <
                                                                                   4isize
                                                                               {
                                                                                current_block
                                                                                    =
                                                                                    16859350023670137067;
                                                                            } else {
                                                                                let fresh10 =
                                                                                    cp;
                                                                                cp
                                                                                    =
                                                                                    cp.offset(1);
                                                                                length
                                                                                    =
                                                                                    *fresh10
                                                                                        as
                                                                                        crate::stdlib::ssize_t;
                                                                                let fresh11 =
                                                                                    cp;
                                                                                cp
                                                                                    =
                                                                                    cp.offset(1);
                                                                                length
                                                                                    =
                                                                                    length
                                                                                        <<
                                                                                        8i32
                                                                                        |
                                                                                        *fresh11
                                                                                            as
                                                                                            isize;
                                                                                let fresh12 =
                                                                                    cp;
                                                                                cp
                                                                                    =
                                                                                    cp.offset(1);
                                                                                length
                                                                                    =
                                                                                    length
                                                                                        <<
                                                                                        8i32
                                                                                        |
                                                                                        *fresh12
                                                                                            as
                                                                                            isize;
                                                                                let fresh13 =
                                                                                    cp;
                                                                                cp
                                                                                    =
                                                                                    cp.offset(1);
                                                                                length
                                                                                    =
                                                                                    length
                                                                                        <<
                                                                                        8i32
                                                                                        |
                                                                                        *fresh13
                                                                                            as
                                                                                            isize;
                                                                                if (end.wrapping_offset_from(cp))
                                                                                       <
                                                                                       length
                                                                                   {
                                                                                    current_block
                                                                                        =
                                                                                        16859350023670137067;
                                                                                } else {
                                                                                    code
                                                                                        =
                                                                                        import_name_composite(context,
                                                                                                              cp,
                                                                                                              length
                                                                                                                  as
                                                                                                                  crate::stddef_h::size_t,
                                                                                                              &mut ad_context);
                                                                                    if code
                                                                                           !=
                                                                                           0i32
                                                                                       {
                                                                                        current_block
                                                                                            =
                                                                                            16859350023670137067;
                                                                                    } else {
                                                                                        cp
                                                                                            =
                                                                                            cp.offset(length);
                                                                                        current_block
                                                                                            =
                                                                                            13003737910779602957;
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            current_block
                                                                                =
                                                                                13003737910779602957;
                                                                        }
                                                                        match current_block
                                                                            {
                                                                            16859350023670137067
                                                                            =>
                                                                            {
                                                                            }
                                                                            _
                                                                            =>
                                                                            {
                                                                                if cp
                                                                                       ==
                                                                                       end
                                                                                   {
                                                                                } else {
                                                                                    crate::stdlib::__assert_fail(b"cp == end\x00"
                                                                                                      as
                                                                                                      *const u8
                                                                                                      as
                                                                                                      *const i8,
                                                                                                  b"import_name.c\x00"
                                                                                                      as
                                                                                                      *const u8
                                                                                                      as
                                                                                                      *const i8,
                                                                                                  295u32,
                                                                                                  (*::std::mem::transmute::<&[u8; 33],
                                                                                                                            &[i8; 33]>(b"OM_uint32 krb5_gss_import_name()\x00")).as_ptr());
                                                                                }
                                                                                current_block
                                                                                    =
                                                                                    2652804691515851435;
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
                                    _ => {
                                        current_block = 16859350023670137067;
                                    }
                                }
                            }
                        }
                    } else {
                        status = (3u32) << 16i32;
                        current_block = 5854819196614252297;
                    }
                    match current_block {
                        5854819196614252297 => {}
                        2652804691515851435 => {}
                        16859350023670137067 => {}
                        _ => {
                            if (if crate::stdlib::getpwuid_r(
                                uid,
                                &mut pwx,
                                pwbuf.as_mut_ptr(),
                                ::std::mem::size_of::<[i8; 8192]>(),
                                &mut pw,
                            ) == 0i32
                            {
                                (if pw.is_null() { -(1i32) } else { 0i32 })
                            } else {
                                -(1i32)
                            }) == 0i32
                            {
                                stringrep = (*pw).pw_name
                            } else {
                                code = -(2045022974 as isize) as crate::krb5_h::krb5_error_code
                            }
                            current_block = 2652804691515851435;
                        }
                    }
                }
                match current_block {
                    5854819196614252297 => {}
                    _ => {
                        match current_block {
                            2652804691515851435 =>
                            /* At this point, stringrep is set, or if not, code is. */
                            {
                                if !stringrep.is_null() {
                                    code = crate::krb5_h::krb5_parse_name_flags(
                                        context, stringrep, flags, &mut princ,
                                    );
                                    if code != 0 {
                                        current_block = 5854819196614252297;
                                    } else {
                                        current_block = 12079920068676227593;
                                    }
                                } else {
                                    current_block = 16859350023670137067;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            5854819196614252297 => {}
                            12079920068676227593 => {}
                            _ => {
                                status = (2u32) << 16i32;
                                current_block = 5854819196614252297;
                            }
                        }
                    }
                }
            }
        }
        match current_block {
            5854819196614252297 => {}
            _ => {
                /* Create a name and save it in the validation database. */
                code = crate::src::krb5::naming_exts::kg_init_name(
                    context, princ, service, host, ad_context, 0x1i32, &mut name,
                );
                if !(code != 0) {
                    princ = 0 as crate::krb5_h::krb5_principal;
                    ad_context = 0 as crate::k5_int_h::krb5_authdata_context;
                    host = 0 as *mut i8;
                    service = host;
                    *output_name = name as crate::gssapi_h::gss_name_t;
                    status = 0u32
                }
            }
        }
    }
    *minor_status = code as crate::gssapi_h::OM_uint32;
    if *minor_status != 0 {
        crate::src::krb5::disp_status::krb5_gss_save_error_info(*minor_status, context);
    }
    crate::krb5_h::krb5_free_principal(context, princ);
    crate::k5_int_h::krb5_authdata_context_free(context, ad_context);
    crate::krb5_h::krb5_free_context(context);
    crate::stdlib::free(tmp as *mut libc::c_void);
    crate::stdlib::free(tmp2 as *mut libc::c_void);
    crate::stdlib::free(service as *mut libc::c_void);
    crate::stdlib::free(host as *mut libc::c_void);
    return status;
}
