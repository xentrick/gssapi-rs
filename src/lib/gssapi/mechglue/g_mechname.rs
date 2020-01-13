use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:9"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:9"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:9"]
pub mod gssapi_h {
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
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID_desc = gss_OID_desc_struct;
    use super::stdint_uintn_h::uint32_t;
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/mechglue/mglueP.h:9"]
pub mod mglueP_h {
    #[c2rust::src_loc = "51:1"]
    pub type gss_mech_spec_name = *mut gss_mech_spec_name_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "51:16"]
    pub struct gss_mech_spec_name_t {
        pub name_type: gss_OID,
        pub mech: gss_OID,
        pub next: *mut gss_mech_spec_name_t,
        pub prev: *mut gss_mech_spec_name_t,
    }
    #[c2rust::src_loc = "51:1"]
    pub type gss_mech_spec_name_desc = gss_mech_spec_name_t;
    use super::gssapi_h::gss_OID;
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
#[c2rust::header_src = "/usr/include/string.h:9"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:9"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapiP_generic.h:9"]
pub mod gssapiP_generic_h {
    use super::gssapi_h::{OM_uint32, gss_OID_desc, gss_OID};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "263:1"]
        pub fn gssint_mecherrmap_map_errcode(errcode: OM_uint32) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "206:1"]
        pub fn generic_gss_copy_oid(_: *mut OM_uint32, _: *const gss_OID_desc,
                                    _: *mut gss_OID) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "201:1"]
        pub fn generic_gss_release_oid(_: *mut OM_uint32, _: *mut gss_OID)
         -> OM_uint32;
    }
    /* _GSSAPIP_GENERIC_H_ */
}
pub use self::types_h::__uint32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssapi_h::{gss_OID, gss_OID_desc_struct, OM_uint32, gss_uint32,
                         gss_OID_desc};
pub use self::mglueP_h::{gss_mech_spec_name, gss_mech_spec_name_t,
                         gss_mech_spec_name_desc};
use self::string_h::memcmp;
use self::stdlib_h::{malloc, free};
use self::gssapiP_generic_h::{gssint_mecherrmap_map_errcode,
                              generic_gss_copy_oid, generic_gss_release_oid};
/*
 * g_mechname.c --- registry of mechanism-specific name types
 *
 * This file contains a registry of mechanism-specific name types.  It
 * is used to determine which name types not should be lazy evaluated,
 * but rather evaluated on the spot.
 */
#[c2rust::src_loc = "18:27"]
static mut name_list: gss_mech_spec_name =
    0 as *const gss_mech_spec_name_t as gss_mech_spec_name;
/*
 * generic searching helper function.
 */
#[c2rust::src_loc = "23:1"]
unsafe extern "C" fn search_mech_spec(mut name_type: gss_OID)
 -> gss_mech_spec_name {
    let mut p: gss_mech_spec_name = 0 as *mut gss_mech_spec_name_t;
    p = name_list;
    while !p.is_null() {
        if (*name_type).length == (*(*p).name_type).length &&
               memcmp((*name_type).elements, (*(*p).name_type).elements,
                      (*name_type).length as libc::c_ulong) ==
                   0 as libc::c_int {
            return p
        }
        p = (*p).next
    }
    return 0 as gss_mech_spec_name;
}
/*
 * Given a name_type, if it is specific to a mechanism, return the
 * mechanism OID.  Otherwise, return NULL.
 */
#[no_mangle]
#[c2rust::src_loc = "39:1"]
pub unsafe extern "C" fn gss_find_mechanism_from_name_type(mut name_type:
                                                               gss_OID)
 -> gss_OID {
    let mut p: gss_mech_spec_name = 0 as *mut gss_mech_spec_name_t;
    p = search_mech_spec(name_type);
    if p.is_null() { return 0 as gss_OID }
    return (*p).mech;
}
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
/* minor_status */
/* mech */
/* internal_name */
/* external_name */
/* union_cred */
/* mech_type */
/* src buffer */
/* destination buffer */
/* NULL terminate buffer ? */
/* minor_status */
/* mech_oid */
/* ctx_out */
/* minor_status */
/* oid set */
/* new oid set */
/* name_type */
/*
 * This function adds a (name_type, mechanism) pair to the
 * mechanism-specific name type registry.  If an entry for the
 * name_type already exists, then zero out the mechanism entry.
 * Otherwise, enter the pair into the registry.
 */
#[no_mangle]
#[c2rust::src_loc = "56:1"]
pub unsafe extern "C" fn gss_add_mech_name_type(mut minor_status:
                                                    *mut OM_uint32,
                                                mut name_type: gss_OID,
                                                mut mech: gss_OID)
 -> OM_uint32 {
    let mut major_status: OM_uint32 = 0;
    let mut tmp: OM_uint32 = 0;
    let mut p: gss_mech_spec_name = 0 as *mut gss_mech_spec_name_t;
    p = search_mech_spec(name_type);
    if !p.is_null() {
        /*
	 * We found an entry for this name type; mark it as not being
	 * a mechanism-specific name type.
	 */
        if !(*p).mech.is_null() {
            if !((*mech).length == (*(*p).mech).length &&
                     memcmp((*mech).elements, (*(*p).mech).elements,
                            (*mech).length as libc::c_ulong) ==
                         0 as libc::c_int) {
                generic_gss_release_oid(minor_status, &mut (*p).mech);
                (*p).mech = 0 as gss_OID
            }
        }
        return 0 as libc::c_int as OM_uint32
    }
    p =
        malloc(::std::mem::size_of::<gss_mech_spec_name_desc>() as
                   libc::c_ulong) as gss_mech_spec_name;
    if p.is_null() {
        *minor_status = 12 as libc::c_int as OM_uint32;
        *minor_status = gssint_mecherrmap_map_errcode(*minor_status)
    } else {
        (*p).name_type = 0 as gss_OID;
        (*p).mech = 0 as gss_OID;
        major_status =
            generic_gss_copy_oid(minor_status,
                                 name_type as *const gss_OID_desc,
                                 &mut (*p).name_type);
        if major_status != 0 {
            *minor_status = gssint_mecherrmap_map_errcode(*minor_status)
        } else {
            major_status =
                generic_gss_copy_oid(minor_status,
                                     mech as *const gss_OID_desc,
                                     &mut (*p).mech);
            if major_status != 0 {
                *minor_status = gssint_mecherrmap_map_errcode(*minor_status)
            } else {
                (*p).next = name_list;
                (*p).prev = 0 as *mut gss_mech_spec_name_t;
                name_list = p;
                return 0 as libc::c_int as OM_uint32
            }
        }
    }
    if !p.is_null() {
        if !(*p).mech.is_null() {
            generic_gss_release_oid(&mut tmp, &mut (*p).mech);
        }
        if !(*p).name_type.is_null() {
            generic_gss_release_oid(&mut tmp, &mut (*p).name_type);
        }
        free(p as *mut libc::c_void);
    }
    return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
}
