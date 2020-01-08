/* _GSSAPI_KRB5_H_ */

/* __cplusplus */
pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_OID_set;
pub use crate::gssapi_h::gss_OID_set_desc_struct;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;

pub use crate::stddef_h::size_t;
pub use crate::stdlib::__uint32_t;

pub use crate::stdlib::uint32_t;

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
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/gssapi/krb5/inq_names.c - Return nametypes supported by krb5 mech */
/*
 * Copyright 1995 by the Massachusetts Institute of Technology.
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
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_inquire_names_for_mech(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut mechanism: crate::gssapi_h::gss_OID,
    mut name_types: *mut crate::gssapi_h::gss_OID_set,
) -> crate::gssapi_h::OM_uint32 {
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut minor: crate::gssapi_h::OM_uint32 = 0;
    /*
     * We only know how to handle our own mechanism.
     */
    if !mechanism.is_null()
        && !((*crate::src::krb5::gssapi_krb5::gss_mech_krb5).length == (*mechanism).length
            && crate::stdlib::memcmp(
                (*crate::src::krb5::gssapi_krb5::gss_mech_krb5).elements,
                (*mechanism).elements,
                (*crate::src::krb5::gssapi_krb5::gss_mech_krb5).length as usize,
            ) == 0i32)
        && !((*crate::src::krb5::gssapi_krb5::gss_mech_krb5_old).length == (*mechanism).length
            && crate::stdlib::memcmp(
                (*crate::src::krb5::gssapi_krb5::gss_mech_krb5_old).elements,
                (*mechanism).elements,
                (*crate::src::krb5::gssapi_krb5::gss_mech_krb5_old).length as usize,
            ) == 0i32)
        && !((*crate::src::krb5::gssapi_krb5::gss_mech_krb5_wrong).length == (*mechanism).length
            && crate::stdlib::memcmp(
                (*crate::src::krb5::gssapi_krb5::gss_mech_krb5_wrong).elements,
                (*mechanism).elements,
                (*crate::src::krb5::gssapi_krb5::gss_mech_krb5_wrong).length as usize,
            ) == 0i32)
        && !((*crate::src::krb5::gssapi_krb5::gss_mech_iakerb).length == (*mechanism).length
            && crate::stdlib::memcmp(
                (*crate::src::krb5::gssapi_krb5::gss_mech_iakerb).elements,
                (*mechanism).elements,
                (*crate::src::krb5::gssapi_krb5::gss_mech_iakerb).length as usize,
            ) == 0i32)
    {
        *minor_status = 0u32;
        return (1u32) << 16i32;
    }
    /* We're okay.  Create an empty OID set */
    major =
        crate::src::generic::oid_ops::generic_gss_create_empty_oid_set(minor_status, name_types);
    if major == 0u32 {
        /* Now add our members. */
        major = crate::src::generic::oid_ops::generic_gss_add_oid_set_member(
            minor_status,
            crate::src::generic::gssapi_generic::gss_nt_user_name
                as *const crate::gssapi_h::gss_OID_desc,
            name_types,
        );
        if major == 0u32
            && {
                major = crate::src::generic::oid_ops::generic_gss_add_oid_set_member(
                    minor_status,
                    crate::src::generic::gssapi_generic::gss_nt_machine_uid_name
                        as *const crate::gssapi_h::gss_OID_desc,
                    name_types,
                );
                (major) == 0u32
            }
            && {
                major = crate::src::generic::oid_ops::generic_gss_add_oid_set_member(
                    minor_status,
                    crate::src::generic::gssapi_generic::gss_nt_string_uid_name
                        as *const crate::gssapi_h::gss_OID_desc,
                    name_types,
                );
                (major) == 0u32
            }
            && {
                major = crate::src::generic::oid_ops::generic_gss_add_oid_set_member(
                    minor_status,
                    crate::src::generic::gssapi_generic::gss_nt_service_name
                        as *const crate::gssapi_h::gss_OID_desc,
                    name_types,
                );
                (major) == 0u32
            }
            && {
                major = crate::src::generic::oid_ops::generic_gss_add_oid_set_member(
                    minor_status,
                    crate::src::generic::gssapi_generic::gss_nt_service_name_v2
                        as *const crate::gssapi_h::gss_OID_desc,
                    name_types,
                );
                (major) == 0u32
            }
            && {
                major = crate::src::generic::oid_ops::generic_gss_add_oid_set_member(
                    minor_status,
                    crate::src::generic::gssapi_generic::gss_nt_exported_name
                        as *const crate::gssapi_h::gss_OID_desc,
                    name_types,
                );
                (major) == 0u32
            }
            && {
                major = crate::src::generic::oid_ops::generic_gss_add_oid_set_member(
                    minor_status,
                    crate::src::krb5::gssapi_krb5::gss_nt_krb5_name
                        as *const crate::gssapi_h::gss_OID_desc,
                    name_types,
                );
                (major) == 0u32
            }
            && {
                major = crate::src::generic::oid_ops::generic_gss_add_oid_set_member(
                    minor_status,
                    crate::src::generic::gssapi_generic::GSS_C_NT_COMPOSITE_EXPORT
                        as *const crate::gssapi_h::gss_OID_desc,
                    name_types,
                );
                (major) == 0u32
            }
        {
            major = crate::src::generic::oid_ops::generic_gss_add_oid_set_member(
                minor_status,
                crate::src::krb5::gssapi_krb5::gss_nt_krb5_principal
                    as *const crate::gssapi_h::gss_OID_desc,
                name_types,
            )
        }
        /*
         * If we choked, then release the set, but don't overwrite the minor
         * status with the release call.
         */
        if major != 0u32 {
            crate::src::generic::rel_oid_set::generic_gss_release_oid_set(&mut minor, name_types);
        }
    }
    return major;
}
