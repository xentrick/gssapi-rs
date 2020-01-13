use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:21"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:21"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:21"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/mechglue/mglueP.h:21"]
pub mod mglueP_h {
    /* lib/gssapi/mechglue/mglueP.h */
    /*
 * Copyright (c) 1995, by Sun Microsystems, Inc.
 * All rights reserved.
 */
    /* This header contains the private mechglue definitions. */
    /*
 * Array of context IDs typed by mechanism OID
 */
    /*
 * Generic GSSAPI names.  A name can either be a generic name, or a
 * mechanism specific name....
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:16"]
    pub struct gss_name_struct {
        pub loopback: *mut gss_name_struct,
        pub name_type: gss_OID,
        pub external_name: gss_buffer_t,
        pub mech_type: gss_OID,
        pub mech_name: gss_name_t,
    }
    #[c2rust::src_loc = "36:1"]
    pub type gss_union_name_t = *mut gss_name_struct;
    #[c2rust::src_loc = "36:1"]
    pub type gss_union_name_desc = gss_name_struct;
    use super::gssapi_h::{gss_OID, gss_buffer_t, gss_name_t, OM_uint32,
                          gss_OID_desc_struct, gss_buffer_desc_struct,
                          gss_OID_desc, gss_const_OID};
    extern "C" {
        /*
 * Rudimentary pointer validation macro to check whether the
 * "loopback" field of an opaque struct points back to itself.  This
 * field also catches some programming errors where an opaque pointer
 * is passed to a function expecting the address of the opaque
 * pointer.
 */
        /* *******************************************************/
/* The Mechanism Dispatch Table -- a mechanism needs to */
/* define one of these and provide a function to return */
/* it to initialize the GSSAPI library		  */
        /*
 * This is the definition of the mechs_array struct, which is used to
 * define the mechs array table. This table is used to indirectly
 * access mechanism specific versions of the gssapi routines through
 * the routines in the glue module (gssd_mech_glue.c)
 *
 * This contants all of the functions defined in gssapi.h except for
 * gss_release_buffer() and gss_release_oid_set(), which I am
 * assuming, for now, to be equal across mechanisms.
 */
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
        /* token_buffer */
        /* minor_status */
        /* context_handle */
        /* output_token */
        /* minor_status */
        /* context_handle */
        /* time_rec */
        /* minor_status */
        /* context_handle */
        /* qop_req */
        /* message_buffer */
        /* message_token */
        /* minor_status */
        /* context_handle */
        /* message_buffer */
        /* token_buffer */
        /* qop_state */
        /* minor_status */
        /* context_handle */
        /* conf_req_flag */
        /* qop_req */
        /* input_message_buffer */
        /* conf_state */
        /* output_message_buffer */
        /* minor_status */
        /* context_handle */
        /* input_message_buffer */
        /* output_message_buffer */
        /* conf_state */
        /* qop_state */
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
        /* minor_status */
        /* context_handle */
        /* interprocess_token */
        /* minor_status */
        /* interprocess_token */
        /* context_handle */
        /* minor_status */
        /* cred_handle */
        /* mech_type */
        /* name */
        /* initiator_lifetime */
        /* acceptor_lifetime */
        /* cred_usage */
        /* minor_status */
        /* mechanism */
        /* name_types */
        /* minor_status */
        /* context_handle */
        /* src_name */
        /* targ_name */
        /* lifetime_rec */
        /* mech_type */
        /* ctx_flags */
        /* locally_initiated */
        /* open */
        /* minor_status */
        /* OID */
        /* minor_status */
        /* context_handle */
        /* conf_req_flag */
        /* qop_req */
        /* req_output_size */
        /* max_input_size */
        /* minor */
        /* name */
        /* mech_type */
        /* localname */
        /* minor_status */
        /* pname */
        /* local user */
        /* local nametype */
        /* */
        /* minor_status */
        /* input_name */
        /* exported_name */
        /* */
        /* minor_status */
        /* input_name */
        /* output_name */
        /* */
        /* minor_status */
        /* input_cred */
        /* cred_usage */
        /* desired_mech */
        /* overwrite_cred */
        /* default_cred */
        /* elements_stored */
        /* cred_usage_stored */
        /* */
        /* GGF extensions */
        /* minor_status */
        /* context_handle */
        /* OID */
        /* data_set */
        /* minor_status */
        /* cred_handle */
        /* OID */
        /* data_set */
        /* minor_status */
        /* context_handle */
        /* OID */
        /* value */
        /* minor_status */
        /* cred_handle */
        /* OID */
        /* value */
        /* minor_status */
        /* mech OID */
        /* OID */
        /* value */
        /* AEAD extensions */
        /* minor_status */
        /* context_handle */
        /* conf_req_flag */
        /* qop_req */
        /* input_assoc_buffer */
        /* input_payload_buffer */
        /* conf_state */
        /* output_message_buffer */
        /* */
        /* minor_status */
        /* context_handle */
        /* input_message_buffer */
        /* input_assoc_buffer */
        /* output_payload_buffer */
        /* conf_state */
        /* qop_state */
        /* */
        /* SSPI extensions */
        /* minor_status */
        /* context_handle */
        /* conf_req_flag */
        /* qop_req */
        /* conf_state */
        /* iov */
        /* iov_count */
        /* */
        /* minor_status */
        /* context_handle */
        /* conf_state */
        /* qop_state */
        /* iov */
        /* iov_count */
        /* */
        /* minor_status */
        /* context_handle */
        /* conf_req_flag*/
        /* qop_req */
        /* conf_state */
        /* iov */
        /* iov_count */
        /* */
        /* minor_status */
        /* context_handle */
        /* input_message_buffer */
        /* New for 1.8 */
        /* minor_status */
        /* impersonator_cred_handle */
        /* desired_name */
        /* time_req */
        /* desired_mechs */
        /* cred_usage */
        /* output_cred_handle */
        /* actual_mechs */
        /* time_rec */
        /* */
        /* minor_status */
        /* input_cred_handle */
        /* impersonator_cred_handle */
        /* desired_name */
        /* desired_mech */
        /* cred_usage */
        /* initiator_time_req */
        /* acceptor_time_req */
        /* output_cred_handle */
        /* actual_mechs */
        /* initiator_time_rec */
        /* acceptor_time_rec */
        /* */
        /* minor_status */
        /* name */
        /* display_as_name_type */
        /* display_name */
        /* */
        /* minor_status */
        /* name */
        /* name_is_MN */
        /* MN_mech */
        /* attrs */
        /* */
        /* minor_status */
        /* name */
        /* attr */
        /* authenticated */
        /* complete */
        /* value */
        /* display_value */
        /* more */
        /* */
        /* minor_status */
        /* name */
        /* complete */
        /* attr */
        /* value */
        /* */
        /* minor_status */
        /* name */
        /* attr */
        /* */
        /* minor_status */
        /* name */
        /* exp_composite_name */
        /* */
        /* minor_status */
        /* name */
        /* authenticated */
        /* type_id */
        /* output */
        /* */
        /* minor_status */
        /* name */
        /* type_id */
        /* input */
        /* */
        /* minor_status */
        /* context */
        /* prf_key */
        /* prf_in */
        /* desired_output_len */
        /* prf_out */
        /* */
        /* minor_status */
        /* cred_handle */
        /* mech_set */
        /* */
        /* minor_status */
        /* desired_mech */
        /* sasl_mech_name */
        /* mech_name */
        /* mech_description */
        /* */
        /* minor_status */
        /* sasl_mech_name */
        /* mech_type */
        /* */
        /* minor_status */
        /* mech */
        /* mech_attrs */
        /* known_mech_attrs */
        /* */
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
        /* */
        /* minor_status */
        /* input_cred_handle */
        /* input_usage */
        /* desired_mech */
        /* overwrite_cred */
        /* default_cred */
        /* cred_store */
        /* elements_stored */
        /* cred_usage_stored */
        /* */
        /* minor_status */
        /* desired_name */
        /* password */
        /* time_req */
        /* desired_mechs */
        /* cred_usage */
        /* output_cred_handle */
        /* actual_mechs */
        /* time_rec */
        /* */
        /* minor_status */
        /* cred_handle */
        /* token */
        /* */
        /* minor_status */
        /* token */
        /* cred_handle */
        /* */
        /* minor_status */
        /* desired_mech */
        /* interprocess_token */
        /* context_handle */
        /* */
        /* minor_status */
        /* mech_type */
        /* input_name_buffer */
        /* input_name_type */
        /* output_name */
        /* */
        /* minor_status */
        /* mech_type */
        /* token */
        /* cred_handle */
        /* */
        /* get_mic_iov extensions, added in 1.12 */
        /* minor_status */
        /* context_handle */
        /* qop_req */
        /* iov */
        /* iov_count */
        /* minor_status */
        /* context_handle */
        /* qop_state */
        /* iov */
        /* iov_count */
        /* minor_status */
        /* context_handle */
        /* qop_req */
        /* iov */
        /* iov_count */
        /* NegoEx extensions added in 1.18 */
        /* minor_status */
        /* mech_oid */
        /* cred_handle */
        /* context_handle */
        /* targ_name */
        /* req_flags */
        /* meta_data */
        /* */
        /* minor_status */
        /* mech_oid */
        /* cred_handle */
        /* context_handle */
        /* targ_name */
        /* req_flags */
        /* meta_data */
        /* */
        /* minor_status */
        /* mech_oid */
        /* auth_scheme */
        /* */
        /*
 * In the user space we use a wrapper structure to encompass the
 * mechanism entry points.  The wrapper contain the mechanism
 * entry points and other data which is only relevant to the gss-api
 * layer.  In the kernel we use only the gss_config strucutre because
 * the kernal does not cantain any of the extra gss-api specific data.
 */
        /* kernel module name */
        /* user library name */
        /* mechanism string name */
        /* optional mech parameters */
        /* RTLD object handle for the mech */
        /* mechanism oid */
        /* mechanism initialization struct */
        /* mechanism preference order */
        /* free mech table */
        /* interposer mechanism flag */
        /* points to the interposer OID */
        /* points to the interposer mech */
        /* next element in the list */
        /* *******************************************************/
/* Internal mechglue routines */
        #[no_mangle]
        #[c2rust::src_loc = "779:1"]
        pub fn gssint_release_internal_name(_: *mut OM_uint32, _: gss_OID,
                                            _: *mut gss_name_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "773:1"]
        pub fn gssint_import_internal_name(_: *mut OM_uint32, _: gss_OID,
                                           _: gss_union_name_t,
                                           _: *mut gss_name_t) -> OM_uint32;
        /* minor_status */
        /* mech */
        /* internal_name */
        /* external_name */
        /* union_cred */
        /* mech_type */
        #[no_mangle]
        #[c2rust::src_loc = "797:1"]
        pub fn gssint_create_copy_buffer(_: gss_buffer_t,
                                         _: *mut gss_buffer_t, _: libc::c_int)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "764:1"]
        pub fn gssint_select_mech_type(minor: *mut OM_uint32,
                                       in_oid: gss_const_OID,
                                       selected_oid: *mut gss_OID)
         -> OM_uint32;
    }
    /* _GSS_MECHGLUEP_H */
    /* Use this to map an errno value or com_err error code being
   generated within the mechglue code (e.g., by calling generic oid
   ops).  Any errno or com_err values produced by mech operations
   should be processed with map_error.  This means they'll be stored
   separately even if the mech uses com_err, because we can't assume
   that it will use com_err.  */
    /* Use this to map an error code that was returned from a mech
   operation; the mech will be asked to produce the associated error
   messages.

   Remember that if the minor status code cannot be returned to the
   caller (e.g., if it's stuffed in an automatic variable and then
   ignored), then we don't care about producing a mapping.  */
    /* qop_state */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:21"]
pub mod gssapi_h {
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
    #[c2rust::src_loc = "79:1"]
    pub type gss_name_t = *mut gss_name_struct;
    /*
 * The following type must be defined as the smallest natural unsigned integer
 * supported by the platform that has at least 32 bits of precision.
 */
    /* OM_STRING */
    /*
 * We can't use X/Open definitions, so roll our own.
 */
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "106:16"]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    /* OM_STRING */
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID_desc = gss_OID_desc_struct;
    /* output_name */
    /* RFC 4401 */
    /* minor_status */
    /* context */
    /* prf_key */
    /* prf_in */
    /* desired_output_len */
    /* prf_out */
    /* minor_status */
    /* input_cred_handle */
    /* input_usage */
    /* desired_mech */
    /* overwrite_cred */
    /* default_cred */
    /* elements_stored */
    /* cred_usage_stored */
    /* minor_status */
    /* cred_handle */
    /* mech_set */
    /* XXXX these are not part of the GSSAPI C bindings!  (but should be) */
    /* XXXX This is a necessary evil until the spec is fixed */
    /*
 * RFC 5587
 */
    #[c2rust::src_loc = "850:1"]
    pub type gss_const_OID = *const gss_OID_desc;
    use super::mglueP_h::gss_name_struct;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        /* output_name */
        #[no_mangle]
        #[c2rust::src_loc = "569:1"]
        pub fn gss_release_name(_: *mut OM_uint32, _: *mut gss_name_t)
         -> OM_uint32;
        /* context_handle */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "673:1"]
        pub fn gss_release_oid(_: *mut OM_uint32, _: *mut gss_OID)
         -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapiP_generic.h:21"]
pub mod gssapiP_generic_h {
    use super::gssapi_h::{OM_uint32, gss_OID_desc, gss_OID};
    extern "C" {
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
 * $Id$
 */
        /* * helper macros **/
        /* this code knows that an int on the wire is 32 bits.  The type of
   num should be at least this big, or the extra shifts may do weird
   things */
        /* * malloc wrappers; these may actually do something later */
        /* * helper functions **/
        /* hide names from applications, especially glib applications */
        /* flags for g_verify_token_header() */
        /* * declarations of internal name mechanism functions **/
        /* minor_status */
        /* buffer */
        /* minor_status */
        /* set */
        /* minor_status */
        /* set */
        /* minor_status */
        /* oid */
        /* new_oid */
        /* minor_status */
        /* oid_set */
        /* minor_status */
        /* member_oid */
        /* oid_set */
        /* minor_status */
        /* member */
        /* set */
        /* present */
        /* minor_status */
        /* oid */
        /* oid_str */
        /* minor_status */
        /* oid_str */
        /* oid */
        /* minor_status */
        /* prefix */
        /* prefix_len */
        /* suffix */
        /* oid */
        /* minor_status */
        /*prefix */
        /* prefix_len */
        /* oid */
        /* suffix */
        #[no_mangle]
        #[c2rust::src_loc = "263:1"]
        pub fn gssint_mecherrmap_map_errcode(errcode: OM_uint32) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "206:1"]
        pub fn generic_gss_copy_oid(_: *mut OM_uint32, _: *const gss_OID_desc,
                                    _: *mut gss_OID) -> OM_uint32;
    }
    /* _GSSAPIP_GENERIC_H_ */
}
#[c2rust::header_src = "/usr/include/stdlib.h:21"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/string.h:21"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
    }
}
pub use self::types_h::__uint32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::stddef_h::size_t;
pub use self::mglueP_h::{gss_name_struct, gss_union_name_t,
                         gss_union_name_desc, gssint_release_internal_name,
                         gssint_import_internal_name,
                         gssint_create_copy_buffer, gssint_select_mech_type};
pub use self::gssapi_h::{gss_name_t, gss_OID, gss_OID_desc_struct, OM_uint32,
                         gss_uint32, gss_buffer_t, gss_buffer_desc_struct,
                         gss_OID_desc, gss_const_OID, gss_release_name,
                         gss_release_oid};
use self::gssapiP_generic_h::{gssint_mecherrmap_map_errcode,
                              generic_gss_copy_oid};
use self::stdlib_h::malloc;
use self::string_h::memcmp;
/*
 * Copyright 2004 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
/* #pragma ident	"@(#)g_canon_name.c	1.15	04/02/23 SMI" */
/*
 * routine gss_canonicalize_name
 *
 * This routine is used to produce a mechanism specific
 * representation of name that has been previously
 * imported with gss_import_name.  The routine uses the mechanism
 * specific implementation of gss_import_name to implement this
 * function.
 *
 * We allow a NULL output_name, in which case we modify the
 * input_name to include the mechanism specific name.
 */
#[c2rust::src_loc = "28:1"]
unsafe extern "C" fn val_canon_name_args(mut minor_status: *mut OM_uint32,
                                         input_name: gss_name_t,
                                         mech_type: gss_OID,
                                         mut output_name: *mut gss_name_t)
 -> OM_uint32 {
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0 as libc::c_int as OM_uint32
    }
    if !output_name.is_null() { *output_name = 0 as gss_name_t }
    /* Validate arguments. */
    if minor_status.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    if input_name.is_null() || mech_type.is_null() {
        return (1 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    return 0 as libc::c_int as OM_uint32;
}
/* dest_name */
/* New for V2 */
#[no_mangle]
#[c2rust::src_loc = "56:1"]
pub unsafe extern "C" fn gss_canonicalize_name(mut minor_status:
                                                   *mut OM_uint32,
                                               input_name: gss_name_t,
                                               mech_type: gss_OID,
                                               mut output_name:
                                                   *mut gss_name_t)
 -> OM_uint32 {
    let mut current_block: u64;
    let mut in_union: gss_union_name_t = 0 as *mut gss_name_struct;
    let mut out_union: gss_union_name_t = 0 as gss_union_name_t;
    let mut dest_union: gss_union_name_t = 0 as gss_union_name_t;
    let mut major_status: OM_uint32 =
        (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
    let mut tmpmin: OM_uint32 = 0;
    let mut selected_mech: gss_OID = 0 as *mut gss_OID_desc_struct;
    major_status =
        val_canon_name_args(minor_status, input_name, mech_type, output_name);
    if major_status != 0 as libc::c_int as libc::c_uint {
        return major_status
    }
    major_status =
        gssint_select_mech_type(minor_status, mech_type as gss_const_OID,
                                &mut selected_mech);
    if major_status != 0 as libc::c_int as libc::c_uint {
        return major_status
    }
    /* Initial value needed below. */
    major_status = (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
    in_union = input_name as gss_union_name_t;
    /*
	 * If the caller wants to reuse the name, and the name has already
	 * been converted, then there is nothing for us to do.
	 */
    if output_name.is_null() && !(*in_union).mech_type.is_null() &&
           ((*(*in_union).mech_type).length == (*selected_mech).length &&
                memcmp((*(*in_union).mech_type).elements,
                       (*selected_mech).elements,
                       (*(*in_union).mech_type).length as libc::c_ulong) ==
                    0 as libc::c_int) {
        return 0 as libc::c_int as OM_uint32
    }
    /* ok, then we need to do something - start by creating data struct */
    if !output_name.is_null() {
        out_union =
            malloc(::std::mem::size_of::<gss_union_name_desc>() as
                       libc::c_ulong) as gss_union_name_t;
        if out_union.is_null() {
            current_block = 5332791911318817343;
        } else {
            (*out_union).mech_type = 0 as gss_OID;
            (*out_union).mech_name = 0 as gss_name_t;
            (*out_union).name_type = 0 as gss_OID;
            (*out_union).external_name = 0 as gss_buffer_t;
            (*out_union).loopback = out_union;
            /* Allocate the buffer for the user specified representation */
            if gssint_create_copy_buffer((*in_union).external_name,
                                         &mut (*out_union).external_name,
                                         1 as libc::c_int) != 0 {
                current_block = 5332791911318817343;
            } else if !(*in_union).name_type.is_null() {
                major_status =
                    generic_gss_copy_oid(minor_status,
                                         (*in_union).name_type as
                                             *const gss_OID_desc,
                                         &mut (*out_union).name_type);
                if major_status != 0 {
                    *minor_status =
                        gssint_mecherrmap_map_errcode(*minor_status);
                    current_block = 5332791911318817343;
                } else { current_block = 11307063007268554308; }
            } else { current_block = 11307063007268554308; }
        }
    } else { current_block = 11307063007268554308; }
    match current_block {
        11307063007268554308 => {
            /*
	 * might need to delete any old mechanism names if we are
	 * reusing the buffer.
	 */
            if output_name.is_null() {
                if !(*in_union).mech_type.is_null() {
                    gssint_release_internal_name(minor_status,
                                                 (*in_union).mech_type,
                                                 &mut (*in_union).mech_name);
                    gss_release_oid(minor_status, &mut (*in_union).mech_type);
                    (*in_union).mech_type = 0 as gss_OID
                }
                dest_union = in_union
            } else { dest_union = out_union }
            /* now let's create the new mech name */
            major_status =
                generic_gss_copy_oid(minor_status,
                                     selected_mech as *const gss_OID_desc,
                                     &mut (*dest_union).mech_type);
            if major_status != 0 {
                *minor_status = gssint_mecherrmap_map_errcode(*minor_status)
            } else {
                major_status =
                    gssint_import_internal_name(minor_status, selected_mech,
                                                in_union,
                                                &mut (*dest_union).mech_name);
                if !(major_status != 0) {
                    if !output_name.is_null() {
                        *output_name = dest_union as gss_name_t
                    }
                    return 0 as libc::c_int as OM_uint32
                }
            }
        }
        _ => { }
    }
    if !out_union.is_null() {
        /* Release the partly constructed out_union. */
        let mut name: gss_name_t = out_union as gss_name_t;
        gss_release_name(&mut tmpmin, &mut name);
    } else if output_name.is_null() {
        /* Release only the mech name fields in in_union. */
        if !(*in_union).mech_name.is_null() {
            gssint_release_internal_name(&mut tmpmin, (*dest_union).mech_type,
                                         &mut (*dest_union).mech_name);
        }
        if !(*in_union).mech_type.is_null() {
            gss_release_oid(&mut tmpmin, &mut (*dest_union).mech_type);
        }
    }
    return major_status;
}
/* *********  gss_canonicalize_name ********/
