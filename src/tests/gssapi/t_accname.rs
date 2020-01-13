use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:26"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:26"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:26"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    #[c2rust::src_loc = "43:1"]
    pub type _IO_lock_t = ();
    use super::types_h::{__off_t, __off64_t};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "38:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "37:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "36:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:26"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:29"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:29"]
pub mod gssapi_h {
    #[c2rust::src_loc = "79:1"]
    pub type gss_name_t = *mut gss_name_struct;
    #[c2rust::src_loc = "82:1"]
    pub type gss_cred_id_t = *mut gss_cred_id_struct;
    #[c2rust::src_loc = "85:1"]
    pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "106:16"]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID_desc = gss_OID_desc_struct;
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:16"]
    pub struct gss_OID_set_desc_struct {
        pub count: size_t,
        pub elements: gss_OID,
    }
    #[c2rust::src_loc = "112:1"]
    pub type gss_OID_set = *mut gss_OID_set_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_desc = gss_buffer_desc_struct;
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    #[c2rust::src_loc = "135:1"]
    pub type gss_cred_usage_t = libc::c_int;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "78:8"]
        pub type gss_name_struct;
        #[c2rust::src_loc = "81:8"]
        pub type gss_cred_id_struct;
        #[c2rust::src_loc = "84:8"]
        pub type gss_ctx_id_struct;
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
        #[no_mangle]
        #[c2rust::src_loc = "474:1"]
        pub fn gss_delete_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_buffer_t) -> OM_uint32;
        /* name_equal */
        #[no_mangle]
        #[c2rust::src_loc = "554:1"]
        pub fn gss_display_name(_: *mut OM_uint32, _: gss_name_t,
                                _: gss_buffer_t, _: *mut gss_OID)
         -> OM_uint32;
        /* output_name */
        #[no_mangle]
        #[c2rust::src_loc = "569:1"]
        pub fn gss_release_name(_: *mut OM_uint32, _: *mut gss_name_t)
         -> OM_uint32;
        /* input_name */
        #[no_mangle]
        #[c2rust::src_loc = "574:1"]
        pub fn gss_release_buffer(_: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
        /* mechanisms */
        /* Last argument new for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "594:1"]
        pub fn gss_inquire_context(_: *mut OM_uint32, _: gss_ctx_id_t,
                                   _: *mut gss_name_t, _: *mut gss_name_t,
                                   _: *mut OM_uint32, _: *mut gss_OID,
                                   _: *mut OM_uint32, _: *mut libc::c_int,
                                   _: *mut libc::c_int) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "432:1"]
        pub fn gss_release_cred(_: *mut OM_uint32, _: *mut gss_cred_id_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "421:1"]
        pub fn gss_acquire_cred(_: *mut OM_uint32, _: gss_name_t,
                                _: OM_uint32, _: gss_OID_set,
                                _: gss_cred_usage_t, _: *mut gss_cred_id_t,
                                _: *mut gss_OID_set, _: *mut OM_uint32)
         -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src = "/usr/include/stdio.h:26"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/tests/gssapi/common.h:29"]
pub mod common_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID, gss_cred_id_struct,
                          gss_cred_id_t, gss_name_struct, gss_name_t,
                          OM_uint32, gss_ctx_id_t, gss_OID_desc};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "59:1"]
        pub fn establish_contexts(imech: gss_OID, icred: gss_cred_id_t,
                                  acred: gss_cred_id_t, tname: gss_name_t,
                                  flags: OM_uint32, ictx: *mut gss_ctx_id_t,
                                  actx: *mut gss_ctx_id_t,
                                  src_name: *mut gss_name_t,
                                  amech: *mut gss_OID,
                                  deleg_cred: *mut gss_cred_id_t);
        #[no_mangle]
        #[c2rust::src_loc = "56:1"]
        pub fn import_name(str: *const libc::c_char) -> gss_name_t;
        #[no_mangle]
        #[c2rust::src_loc = "46:1"]
        pub fn check_gsserr(msg: *const libc::c_char, major: OM_uint32,
                            minor: OM_uint32);
        #[no_mangle]
        #[c2rust::src_loc = "38:21"]
        pub static mut mech_krb5: gss_OID_desc;
    }
    /* COMMON_H */
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint32_t, __off_t, __off64_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssapi_h::{gss_name_t, gss_cred_id_t, gss_ctx_id_t, gss_uint32,
                         OM_uint32, gss_OID_desc_struct, gss_OID_desc,
                         gss_OID, gss_OID_set_desc_struct, gss_OID_set,
                         gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_cred_usage_t, gss_name_struct,
                         gss_cred_id_struct, gss_ctx_id_struct,
                         gss_delete_sec_context, gss_display_name,
                         gss_release_name, gss_release_buffer,
                         gss_inquire_context, gss_release_cred,
                         gss_acquire_cred};
use self::stdio_h::{stderr, fprintf, printf};
use self::common_h::{establish_contexts, import_name, check_gsserr,
                     mech_krb5};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2011 by the Massachusetts Institute of Technology.
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
/*
 * Test program for acceptor names, intended to be run from a Python test
 * script.  Establishes contexts with the default initiator name, a specified
 * principal name as target name, and a specified host-based name as acceptor
 * name (or GSS_C_NO_NAME if no acceptor name is given).  If the exchange is
 * successful, queries the context for the acceptor name and prints it.  If any
 * call is unsuccessful, displays an error message.  Exits with status 0 if all
 * operations are successful, or 1 if not.
 *
 * Usage: ./t_accname targetname [acceptorname]
 */
#[c2rust::src_loc = "43:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut minor: OM_uint32 = 0;
    let mut major: OM_uint32 = 0;
    let mut flags: OM_uint32 = 0;
    let mut acceptor_cred: gss_cred_id_t = 0 as *mut gss_cred_id_struct;
    let mut target_name: gss_name_t = 0 as *mut gss_name_struct;
    let mut acceptor_name: gss_name_t = 0 as gss_name_t;
    let mut real_acceptor_name: gss_name_t = 0 as *mut gss_name_struct;
    let mut namebuf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut initiator_context: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut acceptor_context: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    if argc < 2 as libc::c_int || argc > 3 as libc::c_int {
        fprintf(stderr,
                b"Usage: %s targetname [acceptorname]\n\x00" as *const u8 as
                    *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize));
        return 1 as libc::c_int
    }
    /* Import target and acceptor names. */
    target_name = import_name(*argv.offset(1 as libc::c_int as isize));
    if argc >= 3 as libc::c_int {
        acceptor_name = import_name(*argv.offset(2 as libc::c_int as isize))
    }
    /* Get acceptor cred. */
    major =
        gss_acquire_cred(&mut minor, acceptor_name,
                         0xffffffff as libc::c_ulong as OM_uint32,
                         0 as gss_OID_set, 2 as libc::c_int,
                         &mut acceptor_cred, 0 as *mut gss_OID_set,
                         0 as *mut OM_uint32);
    check_gsserr(b"gss_acquire_cred\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    flags = (4 as libc::c_int | 8 as libc::c_int) as OM_uint32;
    establish_contexts(&mut mech_krb5, 0 as gss_cred_id_t, acceptor_cred,
                       target_name, flags, &mut initiator_context,
                       &mut acceptor_context, 0 as *mut gss_name_t,
                       0 as *mut gss_OID, 0 as *mut gss_cred_id_t);
    major =
        gss_inquire_context(&mut minor, acceptor_context,
                            0 as *mut gss_name_t, &mut real_acceptor_name,
                            0 as *mut OM_uint32, 0 as *mut gss_OID,
                            0 as *mut OM_uint32, 0 as *mut libc::c_int,
                            0 as *mut libc::c_int);
    check_gsserr(b"gss_inquire_context\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    namebuf.value = 0 as *mut libc::c_void;
    namebuf.length = 0 as libc::c_int as size_t;
    major =
        gss_display_name(&mut minor, real_acceptor_name, &mut namebuf,
                         0 as *mut gss_OID);
    check_gsserr(b"gss_display_name\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    printf(b"%.*s\n\x00" as *const u8 as *const libc::c_char,
           namebuf.length as libc::c_int, namebuf.value as *mut libc::c_char);
    gss_release_name(&mut minor, &mut target_name);
    gss_release_name(&mut minor, &mut acceptor_name);
    gss_release_name(&mut minor, &mut real_acceptor_name);
    gss_release_cred(&mut minor, &mut acceptor_cred);
    gss_delete_sec_context(&mut minor, &mut initiator_context,
                           0 as gss_buffer_t);
    gss_delete_sec_context(&mut minor, &mut acceptor_context,
                           0 as gss_buffer_t);
    gss_release_buffer(&mut minor, &mut namebuf);
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
