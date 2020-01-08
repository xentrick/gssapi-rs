use ::libc;

pub mod k5_platform_h {

    #[inline]

    pub unsafe extern "C" fn store_32_be(mut val: u32, mut vp: *mut libc::c_void) {
        let mut p = vp as *mut u8;
        (*(p as *mut crate::k5_platform_h::C2RustUnnamed_6)).i = __bswap_32(val);
    }

    use crate::src::krb5::inq_context::byteswap_h::__bswap_32;
    /* K5_PLATFORM_H */
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

    /* Return the delta between two timestamps (a - b) as a signed 32-bit value,
     * without relying on undefined behavior. */
    #[inline]

    pub unsafe extern "C" fn ts_delta(
        mut a: crate::krb5_h::krb5_timestamp,
        mut b: crate::krb5_h::krb5_timestamp,
    ) -> crate::krb5_h::krb5_deltat {
        return (a as crate::stdlib::uint32_t).wrapping_sub(b as crate::stdlib::uint32_t)
            as crate::krb5_h::krb5_deltat;
    }

    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}

pub mod byteswap_h {
    #[inline]

    pub unsafe extern "C" fn __bswap_32(
        mut __bsx: crate::stdlib::__uint32_t,
    ) -> crate::stdlib::__uint32_t {
        return (__bsx & 0xff000000u32) >> 24i32
            | (__bsx & 0xff0000u32) >> 8i32
            | (__bsx & 0xff00u32) << 8i32
            | (__bsx & 0xffu32) << 24i32;
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
pub use crate::k5_int_h::k5_enctype_to_ssf;
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
pub use crate::k5_platform_h::C2RustUnnamed_6;
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
pub use crate::krb5_h::krb5_ap_req;
pub use crate::krb5_h::krb5_auth_context;
pub use crate::krb5_h::krb5_authdata;
pub use crate::krb5_h::krb5_authdatatype;
pub use crate::krb5_h::krb5_boolean;
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
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_key;
pub use crate::krb5_h::krb5_keyblock;
pub use crate::krb5_h::krb5_kvno;
pub use crate::krb5_h::krb5_magic;
pub use crate::krb5_h::krb5_octet;
pub use crate::krb5_h::krb5_post_recv_fn;
pub use crate::krb5_h::krb5_pre_send_fn;
pub use crate::krb5_h::krb5_principal;
pub use crate::krb5_h::krb5_principal_data;
pub use crate::krb5_h::krb5_prompt_type;
pub use crate::krb5_h::krb5_ticket;
pub use crate::krb5_h::krb5_ticket_times;
pub use crate::krb5_h::krb5_timeofday;
pub use crate::krb5_h::krb5_timestamp;
pub use crate::krb5_h::krb5_trace_callback;
pub use crate::krb5_h::krb5_trace_info;
pub use crate::krb5_h::krb5_transited;
pub use crate::profile_h::profile_t;
pub use crate::src::krb5::inq_context::k5_int_h::ts_delta;
pub use crate::src::krb5::inq_context::k5_platform_h::store_32_be;
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
pub use crate::gssapi_ext_h::gss_buffer_set_desc_struct;
pub use crate::gssapi_ext_h::gss_buffer_set_t;
pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_ctx_id_struct;
pub use crate::gssapi_h::gss_ctx_id_t;
pub use crate::gssapi_h::gss_name_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::mglueP_h::gss_name_struct;
pub use crate::src::generic::oid_ops::generic_gss_oid_compose;
pub use crate::src::generic::oid_ops::generic_gss_oid_decompose;
pub use crate::src::generic::util_buffer_set::generic_gss_add_buffer_set_member;
pub use crate::src::generic::util_buffer_set::generic_gss_release_buffer_set;
pub use crate::src::krb5::disp_status::krb5_gss_save_error_info;
pub use crate::src::krb5::inq_context::byteswap_h::__bswap_32;
pub use crate::src::krb5::naming_exts::kg_duplicate_name;
pub use crate::src::krb5::naming_exts::kg_release_name;
pub use crate::src::mechglue::g_buffer_set::gss_release_buffer_set;

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
 * Copyright (c) 2006-2008, Novell, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *   * Redistributions of source code must retain the above copyright notice,
 *       this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in the
 *       documentation and/or other materials provided with the distribution.
 *   * The copyright holder's name is not used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */
/*
 * Copyright (c) 2006-2008, Novell, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *   * Redistributions of source code must retain the above copyright notice,
 *       this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in the
 *       documentation and/or other materials provided with the distribution.
 *   * The copyright holder's name is not used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_inquire_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut initiator_name: *mut crate::gssapi_h::gss_name_t,
    mut acceptor_name: *mut crate::gssapi_h::gss_name_t,
    mut lifetime_rec: *mut crate::gssapi_h::OM_uint32,
    mut mech_type: *mut crate::gssapi_h::gss_OID,
    mut ret_flags: *mut crate::gssapi_h::OM_uint32,
    mut locally_initiated: *mut i32,
    mut opened: *mut i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut context = 0 as *mut crate::k5_int_h::_krb5_context;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut ctx = 0 as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    let mut initiator = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_name_rec;
    let mut acceptor = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_name_rec;
    let mut now: crate::krb5_h::krb5_timestamp = 0;
    let mut lifetime: crate::krb5_h::krb5_deltat = 0;
    if !initiator_name.is_null() {
        *initiator_name = 0 as crate::gssapi_h::gss_name_t
    }
    if !acceptor_name.is_null() {
        *acceptor_name = 0 as crate::gssapi_h::gss_name_t
    }
    ctx = context_handle as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    context = (*ctx).k5_context;
    /* RFC 2743 states that a partially completed context must return
     * flags, locally_initiated, and open, and *may* return mech_type. */
    if (*ctx).established() != 0 {
        initiator = 0 as crate::gssapiP_krb5_h::krb5_gss_name_t;
        acceptor = 0 as crate::gssapiP_krb5_h::krb5_gss_name_t;
        code = crate::krb5_h::krb5_timeofday(context, &mut now);
        if code != 0 {
            *minor_status = code as crate::gssapi_h::OM_uint32;
            crate::src::krb5::disp_status::krb5_gss_save_error_info(*minor_status, context);
            return (13u32) << 16i32;
        }
        /* Add the maximum allowable clock skew as a grace period for context
         * expiration, just as we do for the ticket during authentication. */
        lifetime = ts_delta((*ctx).krb_times.endtime, now);
        if (*ctx).initiate() == 0 {
            lifetime += (*context).clockskew
        }
        if lifetime < 0i32 {
            lifetime = 0i32
        }
        if !initiator_name.is_null() {
            code = crate::src::krb5::naming_exts::kg_duplicate_name(
                context,
                if (*ctx).initiate() as i32 != 0 {
                    (*ctx).here
                } else {
                    (*ctx).there
                },
                &mut initiator,
            );
            if code != 0 {
                *minor_status = code as crate::gssapi_h::OM_uint32;
                crate::src::krb5::disp_status::krb5_gss_save_error_info(*minor_status, context);
                return (13u32) << 16i32;
            }
        }
        if !acceptor_name.is_null() {
            code = crate::src::krb5::naming_exts::kg_duplicate_name(
                context,
                if (*ctx).initiate() as i32 != 0 {
                    (*ctx).there
                } else {
                    (*ctx).here
                },
                &mut acceptor,
            );
            if code != 0 {
                if !initiator.is_null() {
                    crate::src::krb5::naming_exts::kg_release_name(context, &mut initiator);
                }
                *minor_status = code as crate::gssapi_h::OM_uint32;
                crate::src::krb5::disp_status::krb5_gss_save_error_info(*minor_status, context);
                return (13u32) << 16i32;
            }
        }
        if !initiator_name.is_null() {
            *initiator_name = initiator as crate::gssapi_h::gss_name_t
        }
        if !acceptor_name.is_null() {
            *acceptor_name = acceptor as crate::gssapi_h::gss_name_t
        }
        if !lifetime_rec.is_null() {
            *lifetime_rec = lifetime as crate::gssapi_h::OM_uint32
        }
    } else {
        lifetime = 0i32;
        if !initiator_name.is_null() {
            *initiator_name = 0 as crate::gssapi_h::gss_name_t
        }
        if !acceptor_name.is_null() {
            *acceptor_name = 0 as crate::gssapi_h::gss_name_t
        }
        if !lifetime_rec.is_null() {
            *lifetime_rec = 0u32
        }
    }
    if !mech_type.is_null() {
        *mech_type = (*ctx).mech_used
    }
    if !ret_flags.is_null() {
        *ret_flags = (*ctx).gss_flags
    }
    if !locally_initiated.is_null() {
        *locally_initiated = (*ctx).initiate() as i32
    }
    if !opened.is_null() {
        *opened = (*ctx).established() as i32
    }
    *minor_status = 0u32;
    if (*ctx).established() != 0 {
        return if lifetime == 0i32 {
            (12u32) << 16i32
        } else {
            0u32
        };
    } else {
        return 0u32;
    };
}
#[no_mangle]

pub unsafe extern "C" fn gss_krb5int_inq_session_key(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    context_handle: crate::gssapi_h::gss_ctx_id_t,
    _desired_object: crate::gssapi_h::gss_OID,
    mut data_set: *mut crate::gssapi_ext_h::gss_buffer_set_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = 0 as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    let mut key = 0 as *mut crate::k5_int_h::krb5_key_st;
    let mut keyvalue = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    let mut keyinfo = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut minor: crate::gssapi_h::OM_uint32 = 0;
    let mut oid_buf: [u8; 16] = [0; 16];
    let mut oid = crate::gssapi_h::gss_OID_desc {
        length: 0,
        elements: 0 as *mut libc::c_void,
    };
    ctx = context_handle as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    key = if (*ctx).have_acceptor_subkey() as i32 != 0 {
        (*ctx).acceptor_subkey
    } else {
        (*ctx).subkey
    };
    keyvalue.value = (*key).keyblock.contents as *mut libc::c_void;
    keyvalue.length = (*key).keyblock.length as crate::stddef_h::size_t;
    major_status = crate::src::generic::util_buffer_set::generic_gss_add_buffer_set_member(
        minor_status,
        &mut keyvalue,
        data_set,
    );
    if !(major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0) {
        oid.elements = oid_buf.as_mut_ptr() as *mut libc::c_void;
        oid.length = ::std::mem::size_of::<[u8; 16]>() as crate::gssapi_h::OM_uint32;
        major_status = crate::src::generic::oid_ops::generic_gss_oid_compose(
            minor_status,
            b"*\x86H\x86\xf7\x12\x01\x02\x02\x04\x00" as *const u8 as *const i8,
            10usize,
            (*key).keyblock.enctype,
            &mut oid,
        );
        if !(major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0) {
            keyinfo.value = oid.elements;
            keyinfo.length = oid.length as crate::stddef_h::size_t;
            major_status = crate::src::generic::util_buffer_set::generic_gss_add_buffer_set_member(
                minor_status,
                &mut keyinfo,
                data_set,
            );
            if !(major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0) {
                return 0u32;
            }
        }
    }
    if !(*data_set).is_null() {
        if (**data_set).count != 0usize {
            crate::stdlib::memset(
                (*(**data_set).elements.offset(0isize)).value,
                0i32,
                (*(**data_set).elements.offset(0isize)).length,
            );
        }
        crate::src::mechglue::g_buffer_set::gss_release_buffer_set(&mut minor, data_set);
    }
    return major_status;
}
#[no_mangle]

pub unsafe extern "C" fn gss_krb5int_extract_authz_data_from_sec_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    context_handle: crate::gssapi_h::gss_ctx_id_t,
    desired_object: crate::gssapi_h::gss_OID,
    mut data_set: *mut crate::gssapi_ext_h::gss_buffer_set_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut ctx = 0 as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    let mut ad_type = 0i32;
    let mut i: crate::stddef_h::size_t = 0;
    *data_set = 0 as crate::gssapi_ext_h::gss_buffer_set_t;
    ctx = context_handle as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    major_status = crate::src::generic::oid_ops::generic_gss_oid_decompose(
        minor_status,
        b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\n\x00" as *const u8 as *const i8,
        11usize,
        desired_object,
        &mut ad_type,
    );
    if major_status != 0u32 || ad_type == 0i32 {
        *minor_status = 2u32;
        return (13u32) << 16i32;
    }
    if !(*ctx).authdata.is_null() {
        i = 0usize;
        while !(*(*ctx).authdata.offset(i as isize)).is_null() {
            if (**(*ctx).authdata.offset(i as isize)).ad_type == ad_type {
                let mut ad_data = crate::gssapi_h::gss_buffer_desc {
                    length: 0,
                    value: 0 as *mut libc::c_void,
                };
                ad_data.length =
                    (**(*ctx).authdata.offset(i as isize)).length as crate::stddef_h::size_t;
                ad_data.value =
                    (**(*ctx).authdata.offset(i as isize)).contents as *mut libc::c_void;
                major_status =
                    crate::src::generic::util_buffer_set::generic_gss_add_buffer_set_member(
                        minor_status,
                        &mut ad_data,
                        data_set,
                    );
                if major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
                    break;
                }
            }
            i = i.wrapping_add(1)
        }
    }
    if major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
        let mut tmp: crate::gssapi_h::OM_uint32 = 0;
        crate::src::generic::util_buffer_set::generic_gss_release_buffer_set(&mut tmp, data_set);
    }
    return major_status;
}
#[no_mangle]

pub unsafe extern "C" fn gss_krb5int_extract_authtime_from_sec_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    context_handle: crate::gssapi_h::gss_ctx_id_t,
    _desired_oid: crate::gssapi_h::gss_OID,
    mut data_set: *mut crate::gssapi_ext_h::gss_buffer_set_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = 0 as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    let mut rep = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    ctx = context_handle as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    rep.value =
        &mut (*ctx).krb_times.authtime as *mut crate::krb5_h::krb5_timestamp as *mut libc::c_void;
    rep.length = ::std::mem::size_of::<crate::krb5_h::krb5_timestamp>();
    return crate::src::generic::util_buffer_set::generic_gss_add_buffer_set_member(
        minor_status,
        &mut rep,
        data_set,
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
/*
 * These take unglued krb5-mech-specific contexts.
 */
#[no_mangle]

pub unsafe extern "C" fn gss_krb5int_sec_context_sasl_ssf(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    context_handle: crate::gssapi_h::gss_ctx_id_t,
    _desired_object: crate::gssapi_h::gss_OID,
    mut data_set: *mut crate::gssapi_ext_h::gss_buffer_set_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = 0 as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    let mut key = 0 as *mut crate::k5_int_h::krb5_key_st;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut ssfbuf = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    let mut ssf: u32 = 0;
    let mut buf: [crate::stdlib::uint8_t; 4] = [0; 4];
    ctx = context_handle as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    key = if (*ctx).have_acceptor_subkey() as i32 != 0 {
        (*ctx).acceptor_subkey
    } else {
        (*ctx).subkey
    };
    code = crate::k5_int_h::k5_enctype_to_ssf((*key).keyblock.enctype, &mut ssf);
    if code != 0 {
        return (13u32) << 16i32;
    }
    store_32_be(ssf, buf.as_mut_ptr() as *mut libc::c_void);
    ssfbuf.value = buf.as_mut_ptr() as *mut libc::c_void;
    ssfbuf.length = ::std::mem::size_of::<[crate::stdlib::uint8_t; 4]>();
    return crate::src::generic::util_buffer_set::generic_gss_add_buffer_set_member(
        minor_status,
        &mut ssfbuf,
        data_set,
    );
}
