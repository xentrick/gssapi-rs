use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:29"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:29"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:29"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:29"]
pub mod gssapi_h {
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
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "85:1"]
    pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "84:8"]
        pub type gss_ctx_id_struct;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/mechglue/mglueP.h:29"]
pub mod mglueP_h {
    #[c2rust::src_loc = "26:1"]
    pub type gss_union_ctx_id_t = *mut gss_union_ctx_id_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "26:16"]
    pub struct gss_union_ctx_id_struct {
        pub loopback: *mut gss_union_ctx_id_struct,
        pub mech_type: gss_OID,
        pub internal_ctx_id: gss_ctx_id_t,
    }
    use super::gssapi_h::{gss_OID, gss_ctx_id_t, OM_uint32,
                          gss_OID_desc_struct, gss_buffer_desc_struct,
                          gss_buffer_t};
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
        #[c2rust::src_loc = "780:1"]
        pub fn gssint_delete_internal_sec_context(_: *mut OM_uint32,
                                                  _: gss_OID,
                                                  _: *mut gss_ctx_id_t,
                                                  _: gss_buffer_t)
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
#[c2rust::header_src = "/usr/include/stdlib.h:29"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
pub use self::types_h::__uint32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::stddef_h::size_t;
pub use self::gssapi_h::{gss_OID, gss_OID_desc_struct, OM_uint32, gss_uint32,
                         gss_buffer_t, gss_buffer_desc_struct, gss_ctx_id_t,
                         gss_ctx_id_struct};
pub use self::mglueP_h::{gss_union_ctx_id_t, gss_union_ctx_id_struct,
                         gssint_delete_internal_sec_context};
use self::stdlib_h::free;
/* #pragma ident	"@(#)g_delete_sec_context.c	1.11	97/11/09 SMI" */
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
 *  glue routine for gss_delete_sec_context
 */
#[c2rust::src_loc = "35:1"]
unsafe extern "C" fn val_del_sec_ctx_args(mut minor_status: *mut OM_uint32,
                                          mut context_handle:
                                              *mut gss_ctx_id_t,
                                          mut output_token: gss_buffer_t)
 -> OM_uint32 {
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0 as libc::c_int as OM_uint32
    }
    if !output_token.is_null() {
        (*output_token).length = 0 as libc::c_int as size_t;
        (*output_token).value = 0 as *mut libc::c_void
    }
    /* Validate arguments. */
    if minor_status.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    if context_handle.is_null() || (*context_handle).is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                   (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    return 0 as libc::c_int as OM_uint32;
}
#[no_mangle]
#[c2rust::src_loc = "64:1"]
pub unsafe extern "C" fn gss_delete_sec_context(mut minor_status:
                                                    *mut OM_uint32,
                                                mut context_handle:
                                                    *mut gss_ctx_id_t,
                                                mut output_token:
                                                    gss_buffer_t)
 -> OM_uint32 {
    let mut status: OM_uint32 = 0;
    let mut ctx: gss_union_ctx_id_t = 0 as *mut gss_union_ctx_id_struct;
    status = val_del_sec_ctx_args(minor_status, context_handle, output_token);
    if status != 0 as libc::c_int as libc::c_uint { return status }
    /*
     * select the approprate underlying mechanism routine and
     * call it.
     */
    ctx = *context_handle as gss_union_ctx_id_t;
    if !(!ctx.is_null() && (*ctx).loopback == ctx) {
        return (1 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                   (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if !(*ctx).internal_ctx_id.is_null() {
        status =
            gssint_delete_internal_sec_context(minor_status, (*ctx).mech_type,
                                               &mut (*ctx).internal_ctx_id,
                                               output_token);
        if status != 0 { return status }
    }
    /* now free up the space for the union context structure */
    free((*(*ctx).mech_type).elements);
    free((*ctx).mech_type as *mut libc::c_void);
    free(*context_handle as *mut libc::c_void);
    *context_handle = 0 as gss_ctx_id_t;
    return 0 as libc::c_int as OM_uint32;
}
