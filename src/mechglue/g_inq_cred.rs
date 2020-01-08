/* _GSSAPIP_GENERIC_H_ */
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__ssize_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::ssize_t;
pub use crate::stdlib::uint32_t;

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
pub use crate::mglueP_h::gss_mechanism;
pub use crate::mglueP_h::gss_name_struct;
pub use crate::mglueP_h::gss_union_cred_t;

pub use crate::src::mechglue::g_glue::gssint_convert_name_to_union_name;
pub use crate::src::mechglue::g_glue::gssint_get_mechanism_cred;
pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;
pub use crate::src::mechglue::g_initialize::gssint_get_public_oid;
pub use crate::src::mechglue::g_initialize::gssint_make_public_oid_set;
pub use crate::src::mechglue::g_initialize::gssint_select_mech_type;
pub use crate::src::mechglue::g_oid_ops::gss_add_oid_set_member;
pub use crate::src::mechglue::g_oid_ops::gss_create_empty_oid_set;
pub use crate::src::mechglue::g_rel_name::gss_release_name;
pub use crate::src::mechglue::g_rel_oid_set::gss_release_oid_set;
/* #pragma ident	"@(#)g_inquire_cred.c	1.16	04/02/23 SMI" */
/*
 * Copyright 1996 by Sun Microsystems, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of Sun Microsystems not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. Sun Microsystems makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * SUN MICROSYSTEMS DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL SUN MICROSYSTEMS BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
/*
 *  glue routine for gss_inquire_cred
 */
#[no_mangle]

pub unsafe extern "C" fn gss_inquire_cred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut name: *mut crate::gssapi_h::gss_name_t,
    mut lifetime: *mut crate::gssapi_h::OM_uint32,
    mut cred_usage: *mut i32,
    mut mechanisms: *mut crate::gssapi_h::gss_OID_set,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut temp_minor_status: crate::gssapi_h::OM_uint32 = 0;
    let mut union_cred = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    let mut mech_cred = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    let mut mech_name = 0 as *mut crate::mglueP_h::gss_name_struct;
    let mut mechs = 0 as crate::gssapi_h::gss_OID_set;
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !name.is_null() {
        *name = 0 as crate::gssapi_h::gss_name_t
    }
    if !mechanisms.is_null() {
        *mechanisms = 0 as crate::gssapi_h::gss_OID_set
    }
    /* Validate arguments. */
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    /*
     * XXX We should iterate over all mechanisms in the credential and
     * aggregate the results.  This requires a union name structure containing
     * multiple mechanism names, which we don't currently have.  For now,
     * inquire the first mechanism in the credential; this is consistent with
     * our historical behavior.
     */
    /* Determine mechanism and mechanism credential. */
    if !cred_handle.is_null() {
        union_cred = cred_handle;
        if (*union_cred).count <= 0i32 {
            return (10u32) << 16i32;
        }
        mech_cred = *(*union_cred).cred_array.offset(0isize);
        mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
            &mut *(*union_cred).mechs_array.offset(0isize)
                as *mut crate::gssapi_h::gss_OID_desc_struct
                as crate::gssapi_h::gss_const_OID,
        )
    } else {
        union_cred = 0 as crate::mglueP_h::gss_union_cred_t;
        mech_cred = 0 as crate::gssapi_h::gss_cred_id_t;
        mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
            0 as crate::gssapi_h::gss_const_OID,
        )
    }
    /* Skip the call into the mech if the caller doesn't care about any of the
     * values we would ask for. */
    if !name.is_null() || !lifetime.is_null() || !cred_usage.is_null() {
        if mech.is_null() {
            return (10u32) << 16i32;
        }
        if (*mech).gss_inquire_cred.is_none() {
            return (16u32) << 16i32;
        }
        status = (*mech).gss_inquire_cred.expect("non-null function pointer")(
            minor_status,
            mech_cred,
            if !name.is_null() {
                &mut mech_name
            } else {
                0 as *mut crate::gssapi_h::gss_name_t
            },
            lifetime,
            cred_usage,
            0 as *mut crate::gssapi_h::gss_OID_set,
        );
        if status != 0u32 {
            *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                *minor_status,
                &mut (*mech).mech_type,
            );
            return status;
        }
        if !name.is_null() {
            /* Convert mech_name into a union_name equivalent. */
            status = crate::src::mechglue::g_glue::gssint_convert_name_to_union_name(
                &mut temp_minor_status,
                mech,
                mech_name,
                name,
            );
            if status != 0u32 {
                *minor_status = temp_minor_status;
                *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                    *minor_status,
                    &mut (*mech).mech_type,
                );
                return status;
            }
        }
    }
    /*
     * copy the mechanism set in union_cred into an OID set and return in
     * the mechanisms parameter.
     */
    if !mechanisms.is_null() {
        if !union_cred.is_null() {
            status = crate::src::mechglue::g_initialize::gssint_make_public_oid_set(
                minor_status,
                (*union_cred).mechs_array,
                (*union_cred).count,
                &mut mechs,
            );
            if status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
                current_block = 8389588123125467076;
            } else {
                current_block = 9441801433784995173;
            }
        } else {
            status =
                crate::src::mechglue::g_oid_ops::gss_create_empty_oid_set(minor_status, &mut mechs);
            if status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
                current_block = 8389588123125467076;
            } else {
                status = crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(
                    minor_status,
                    &mut (*mech).mech_type,
                    &mut mechs,
                );
                if status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
                    current_block = 8389588123125467076;
                } else {
                    current_block = 9441801433784995173;
                }
            }
        }
        match current_block {
            8389588123125467076 => {
                if !mechs.is_null() {
                    crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(
                        &mut temp_minor_status,
                        &mut mechs,
                    );
                }
                if !name.is_null() && !(*name).is_null() {
                    crate::src::mechglue::g_rel_name::gss_release_name(
                        &mut temp_minor_status,
                        name,
                    );
                }
                return status;
            }
            _ => *mechanisms = mechs,
        }
    }
    return 0u32;
}
/* This is the gssapi.h prologue. */
/* no xom.h */
/* End of gssapi.h prologue. */
/* -*- mode: c; indent-tabs-mode: nil -*- */
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
 * Determine platform-dependent configuration.
 */
/* __cplusplus */
/*
 * First, include stddef.h to get size_t defined.
 */
/*
 * POSIX says that sys/types.h is where size_t is defined.
 */
/*
 * $Id$
 */
/*
 * First, define the three platform-dependent pointer types.
 */
/*
 * The following type must be defined as the smallest natural unsigned integer
 * supported by the platform that has at least 32 bits of precision.
 */
/* OM_STRING */
/*
 * We can't use X/Open definitions, so roll our own.
 */
/* OM_STRING */
/*
 * For now, define a QOP-type as an OM_uint32 (pending resolution of ongoing
 * discussions).
 */
/*
 * Flag bits for context-level services.
 */
/*
 * Credential usage options
 */
/*
 * Status code types for gss_display_status
 */
/*
 * The constant definitions for channel-bindings address families
 */
/*
 * Various Null values.
 */
/*
 * Some alternate names for a couple of the above values.  These are defined
 * for V1 compatibility.
 */
/*
 * Define the default Quality of Protection for per-message services.  Note
 * that an implementation that offers multiple levels of QOP may either reserve
 * a value (for example zero, as assumed here) to mean "default protection", or
 * alternatively may simply equate GSS_C_QOP_DEFAULT to a specific explicit
 * QOP value.  However a value of 0 should always be interpreted by a GSSAPI
 * implementation as a request for the default protection level.
 */
/*
 * Expiration time of 2^32-1 seconds means infinite lifetime for a
 * credential or security context
 */
/* Major status codes */
/*
 * Some "helper" definitions to make the status code macros obvious.
 */
/*
 * The macros that test status codes for error conditions.  Note that the
 * GSS_ERROR() macro has changed slightly from the V1 GSSAPI so that it now
 * evaluates its argument only once.
 */
/*
 * Now the actual status code definitions
 */
/*
 * Calling errors:
 */
/*
 * Routine errors:
 */
/*
 * Supplementary info bits:
 */
/*
 * Finally, function prototypes for the GSSAPI routines.
 */
/* Reserved static storage for GSS_oids.  Comments are quotes from RFC 2744.
 *
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {10, (void *)"\x2a\x86\x48\x86\xf7\x12\x01\x02\x01\x01"},
 * corresponding to an object-identifier value of
 * {iso(1) member-body(2) United States(840) mit(113554)
 * infosys(1) gssapi(2) generic(1) user_name(1)}.  The constant
 * GSS_C_NT_USER_NAME should be initialized to point
 * to that gss_OID_desc.
 */
/*
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {10, (void *)"\x2a\x86\x48\x86\xf7\x12\x01\x02\x01\x02"},
 * corresponding to an object-identifier value of
 * {iso(1) member-body(2) United States(840) mit(113554)
 * infosys(1) gssapi(2) generic(1) machine_uid_name(2)}.
 * The constant GSS_C_NT_MACHINE_UID_NAME should be
 * initialized to point to that gss_OID_desc.
 */
/*
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {10, (void *)"\x2a\x86\x48\x86\xf7\x12\x01\x02\x01\x03"},
 * corresponding to an object-identifier value of
 * {iso(1) member-body(2) United States(840) mit(113554)
 * infosys(1) gssapi(2) generic(1) string_uid_name(3)}.
 * The constant GSS_C_NT_STRING_UID_NAME should be
 * initialized to point to that gss_OID_desc.
 */
/*
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {6, (void *)"\x2b\x06\x01\x05\x06\x02"},
 * corresponding to an object-identifier value of
 * {iso(1) org(3) dod(6) internet(1) security(5)
 * nametypes(6) gss-host-based-services(2)).  The constant
 * GSS_C_NT_HOSTBASED_SERVICE_X should be initialized to point
 * to that gss_OID_desc.  This is a deprecated OID value, and
 * implementations wishing to support hostbased-service names
 * should instead use the GSS_C_NT_HOSTBASED_SERVICE OID,
 * defined below, to identify such names;
 * GSS_C_NT_HOSTBASED_SERVICE_X should be accepted a synonym
 * for GSS_C_NT_HOSTBASED_SERVICE when presented as an input
 * parameter, but should not be emitted by GSS-API
 * implementations
 */
/*
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {10, (void *)"\x2a\x86\x48\x86\xf7\x12"
 *              "\x01\x02\x01\x04"}, corresponding to an
 * object-identifier value of {iso(1) member-body(2)
 * Unites States(840) mit(113554) infosys(1) gssapi(2)
 * generic(1) service_name(4)}.  The constant
 * GSS_C_NT_HOSTBASED_SERVICE should be initialized
 * to point to that gss_OID_desc.
 */
/*
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {6, (void *)"\x2b\x06\01\x05\x06\x03"},
 * corresponding to an object identifier value of
 * {1(iso), 3(org), 6(dod), 1(internet), 5(security),
 * 6(nametypes), 3(gss-anonymous-name)}.  The constant
 * and GSS_C_NT_ANONYMOUS should be initialized to point
 * to that gss_OID_desc.
 */
/*
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {6, (void *)"\x2b\x06\x01\x05\x06\x04"},
 * corresponding to an object-identifier value of
 * {1(iso), 3(org), 6(dod), 1(internet), 5(security),
 * 6(nametypes), 4(gss-api-exported-name)}.  The constant
 * GSS_C_NT_EXPORT_NAME should be initialized to point
 * to that gss_OID_desc.
 */
/* Function Prototypes */
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
/* mech_type (used to be const) */
/* req_flags */
/* time_req */
/* input_chan_bindings */
/* input_token */
/* actual_mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* minor_status */
/* context_handle */
/* acceptor_cred_handle */
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
/* token_buffer */
/* minor_status */
/* context_handle */
/* output_token */
/* minor_status */
/* context_handle */
/* time_rec */
/* New for V2 */
/* minor_status */
/* context_handle */
/* qop_req */
/* message_buffer */
/* message_token */
/* New for V2 */
/* minor_status */
/* context_handle */
/* message_buffer */
/* message_token */
/* qop_state */
/* New for V2 */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* input_message_buffer */
/* conf_state */
/* output_message_buffer */
/* New for V2 */
/* minor_status */
/* context_handle */
/* input_message_buffer */
/* output_message_buffer */
/* conf_state */
/* qop_state */
/* minor_status */
/* status_value */
/* status_type */
/* mech_type (used to be const) */
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
/* input_name_type(used to be const) */
/* output_name */
/* minor_status */
/* input_name */
/* minor_status */
/* buffer */
/* minor_status */
/* set */
/* minor_status */
/* cred_handle */
/* name */
/* lifetime */
/* cred_usage */
/* mechanisms */
/* Last argument new for V2 */
/* minor_status */
/* context_handle */
/* src_name */
/* targ_name */
/* lifetime_rec */
/* mech_type */
/* ctx_flags */
/* locally_initiated */
/* open */
/* New for V2 */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* req_output_size */
/* max_input_size */
/* New for V2 */
/* minor_status */
/* input_name */
/* input_name_type */
/* output_name */
/* New for V2 */
/* minor_status */
/* input_name */
/* desired_name_type */
/* output_name */
/* New for V2 */
/* minor_status */
/* input_cred_handle */
/* desired_name */
/* desired_mech */
/* cred_usage */
/* initiator_time_req */
/* acceptor_time_req */
/* output_cred_handle */
/* actual_mechs */
/* initiator_time_rec */
/* acceptor_time_rec */
/* New for V2 */
#[no_mangle]

pub unsafe extern "C" fn gss_inquire_cred_by_mech(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut mech_type: crate::gssapi_h::gss_OID,
    mut name: *mut crate::gssapi_h::gss_name_t,
    mut initiator_lifetime: *mut crate::gssapi_h::OM_uint32,
    mut acceptor_lifetime: *mut crate::gssapi_h::OM_uint32,
    mut cred_usage: *mut crate::gssapi_h::gss_cred_usage_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut union_cred = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    let mut mech_cred = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut temp_minor_status: crate::gssapi_h::OM_uint32 = 0;
    let mut internal_name = 0 as *mut crate::mglueP_h::gss_name_struct;
    let mut selected_mech = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut public_mech = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !name.is_null() {
        *name = 0 as crate::gssapi_h::gss_name_t
    }
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    status = crate::src::mechglue::g_initialize::gssint_select_mech_type(
        minor_status,
        mech_type as crate::gssapi_h::gss_const_OID,
        &mut selected_mech,
    );
    if status != 0u32 {
        return status;
    }
    mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
        selected_mech as crate::gssapi_h::gss_const_OID,
    );
    if mech.is_null() {
        return (1u32) << 16i32;
    }
    if (*mech).gss_inquire_cred_by_mech.is_none() {
        return (4u32) << 16i32;
    }
    union_cred = cred_handle;
    mech_cred = crate::src::mechglue::g_glue::gssint_get_mechanism_cred(union_cred, selected_mech);
    if !cred_handle.is_null() && mech_cred.is_null() {
        return (7u32) << 16i32;
    }
    public_mech = crate::src::mechglue::g_initialize::gssint_get_public_oid(
        selected_mech as crate::gssapi_h::gss_const_OID,
    );
    status = (*mech)
        .gss_inquire_cred_by_mech
        .expect("non-null function pointer")(
        minor_status,
        mech_cred,
        public_mech,
        if !name.is_null() {
            &mut internal_name
        } else {
            0 as *mut crate::gssapi_h::gss_name_t
        },
        initiator_lifetime,
        acceptor_lifetime,
        cred_usage,
    );
    if status != 0u32 {
        *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
            *minor_status,
            &mut (*mech).mech_type,
        );
        return status;
    }
    if !name.is_null() {
        /*
         * Convert internal_name into a union_name equivalent.
         */
        status = crate::src::mechglue::g_glue::gssint_convert_name_to_union_name(
            &mut temp_minor_status,
            mech,
            internal_name,
            name,
        );
        if status != 0u32 {
            *minor_status = temp_minor_status;
            *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                *minor_status,
                &mut (*mech).mech_type,
            );
            return status;
        }
    }
    return 0u32;
}
