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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:31"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:31"]
pub mod gssapi_h {
    #[c2rust::src_loc = "79:1"]
    pub type gss_name_t = *mut gss_name_struct;
    #[c2rust::src_loc = "82:1"]
    pub type gss_cred_id_t = *mut gss_cred_id_struct;
    #[c2rust::src_loc = "85:1"]
    pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
    /*
 * The following type must be defined as the smallest natural unsigned integer
 * supported by the platform that has at least 32 bits of precision.
 */
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    /* OM_STRING */
    /*
 * We can't use X/Open definitions, so roll our own.
 */
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
    /* OM_STRING */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:16"]
    pub struct gss_OID_set_desc_struct {
        pub count: size_t,
        pub elements: gss_OID,
    }
    #[c2rust::src_loc = "112:1"]
    pub type gss_OID_set_desc = gss_OID_set_desc_struct;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "122:16"]
    pub struct gss_channel_bindings_struct {
        pub initiator_addrtype: OM_uint32,
        pub initiator_address: gss_buffer_desc,
        pub acceptor_addrtype: OM_uint32,
        pub acceptor_address: gss_buffer_desc,
        pub application_data: gss_buffer_desc,
    }
    #[c2rust::src_loc = "122:1"]
    pub type gss_channel_bindings_t = *mut gss_channel_bindings_struct;
    #[c2rust::src_loc = "135:1"]
    pub type gss_cred_usage_t = libc::c_int;
    #[c2rust::src_loc = "850:1"]
    pub type gss_const_OID = *const gss_OID_desc;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
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
        #[c2rust::src_loc = "78:8"]
        pub type gss_name_struct;
        #[c2rust::src_loc = "81:8"]
        pub type gss_cred_id_struct;
        #[c2rust::src_loc = "84:8"]
        pub type gss_ctx_id_struct;
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
        #[no_mangle]
        #[c2rust::src_loc = "336:27"]
        pub static mut GSS_C_NT_USER_NAME: gss_OID;
        /* Function Prototypes */
        #[no_mangle]
        #[c2rust::src_loc = "421:1"]
        pub fn gss_acquire_cred(_: *mut OM_uint32, _: gss_name_t,
                                _: OM_uint32, _: gss_OID_set,
                                _: gss_cred_usage_t, _: *mut gss_cred_id_t,
                                _: *mut gss_OID_set, _: *mut OM_uint32)
         -> OM_uint32;
        /* time_rec */
        #[no_mangle]
        #[c2rust::src_loc = "432:1"]
        pub fn gss_release_cred(_: *mut OM_uint32, _: *mut gss_cred_id_t)
         -> OM_uint32;
        /* cred_handle */
        #[no_mangle]
        #[c2rust::src_loc = "437:1"]
        pub fn gss_init_sec_context(_: *mut OM_uint32, _: gss_cred_id_t,
                                    _: *mut gss_ctx_id_t, _: gss_name_t,
                                    _: gss_OID, _: OM_uint32, _: OM_uint32,
                                    _: gss_channel_bindings_t,
                                    _: gss_buffer_t, _: *mut gss_OID,
                                    _: gss_buffer_t, _: *mut OM_uint32,
                                    _: *mut OM_uint32) -> OM_uint32;
        /* time_rec */
        #[no_mangle]
        #[c2rust::src_loc = "453:1"]
        pub fn gss_accept_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_cred_id_t, _: gss_buffer_t,
                                      _: gss_channel_bindings_t,
                                      _: *mut gss_name_t, _: *mut gss_OID,
                                      _: gss_buffer_t, _: *mut OM_uint32,
                                      _: *mut OM_uint32,
                                      _: *mut gss_cred_id_t) -> OM_uint32;
        /* token_buffer */
        #[no_mangle]
        #[c2rust::src_loc = "474:1"]
        pub fn gss_delete_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_buffer_t) -> OM_uint32;
        /* output_name_type */
        #[no_mangle]
        #[c2rust::src_loc = "562:1"]
        pub fn gss_import_name(_: *mut OM_uint32, _: gss_buffer_t, _: gss_OID,
                               _: *mut gss_name_t) -> OM_uint32;
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
    }
}
#[c2rust::header_src = "/usr/include/string.h:28"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:29"]
pub mod assert_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn __assert_fail(__assertion: *const libc::c_char,
                             __file: *const libc::c_char,
                             __line: libc::c_uint,
                             __function: *const libc::c_char) -> !;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_ext.h:31"]
pub mod gssapi_ext_h {
    use super::gssapi_h::{OM_uint32, gss_name_struct, gss_name_t,
                          gss_buffer_desc_struct, gss_buffer_t,
                          gss_OID_set_desc_struct, gss_OID_set,
                          gss_cred_usage_t, gss_cred_id_t, gss_OID_desc,
                          gss_const_OID};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "104:1"]
        pub fn gss_acquire_cred_with_password(_: *mut OM_uint32,
                                              _: gss_name_t, _: gss_buffer_t,
                                              _: OM_uint32, _: gss_OID_set,
                                              _: gss_cred_usage_t,
                                              _: *mut gss_cred_id_t,
                                              _: *mut gss_OID_set,
                                              _: *mut OM_uint32) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "522:1"]
        pub fn gss_oid_equal(_: gss_const_OID, _: gss_const_OID)
         -> libc::c_int;
    }
    /* GSSAPI_EXT_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/tests/gssapi/common.h:31"]
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
        #[c2rust::src_loc = "40:21"]
        pub static mut mech_iakerb: gss_OID_desc;
        #[no_mangle]
        #[c2rust::src_loc = "39:21"]
        pub static mut mech_spnego: gss_OID_desc;
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
                         gss_OID, gss_OID_set_desc_struct, gss_OID_set_desc,
                         gss_OID_set, gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_channel_bindings_struct,
                         gss_channel_bindings_t, gss_cred_usage_t,
                         gss_const_OID, gss_name_struct, gss_cred_id_struct,
                         gss_ctx_id_struct, GSS_C_NT_USER_NAME,
                         gss_acquire_cred, gss_release_cred,
                         gss_init_sec_context, gss_accept_sec_context,
                         gss_delete_sec_context, gss_import_name,
                         gss_release_name, gss_release_buffer,
                         gss_inquire_context};
use self::stdio_h::{stderr, fprintf};
use self::string_h::strlen;
use self::assert_h::__assert_fail;
use self::gssapi_ext_h::{gss_acquire_cred_with_password, gss_oid_equal};
use self::common_h::{establish_contexts, import_name, check_gsserr,
                     mech_iakerb, mech_spnego, mech_krb5};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2015 Red Hat, Inc.
 *
 * Permission is hereby granted, free of charge, to any person
 * obtaining a copy of this software and associated documentation files
 * (the "Software"), to deal in the Software without restriction,
 * including without limitation the rights to use, copy, modify, merge,
 * publish, distribute, sublicense, and/or sell copies of the Software,
 * and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
/*
 * Test program for inquiring about a security context, intented to be run from
 * a Python test script.  Partially establishes a context to test inquiring
 * about an incomplete context, and then establishes full contexts and inquires
 * them.  Exits with status 0 if all operations are successful, or 1 if not.
 *
 * Usage: ./t_inq_ctx target_name
 */
#[c2rust::src_loc = "43:1"]
unsafe extern "C" fn check_inq_context(mut context: gss_ctx_id_t,
                                       mut incomplete: libc::c_int,
                                       mut expected_mech: gss_OID,
                                       mut expected_flags: OM_uint32,
                                       mut expected_locally_init:
                                           libc::c_int) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut out_init_name: gss_name_t = 0 as *mut gss_name_struct;
    let mut out_accept_name: gss_name_t = 0 as *mut gss_name_struct;
    let mut out_lifetime: OM_uint32 = 0;
    let mut out_mech_type: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut out_flags: OM_uint32 = 0;
    let mut out_locally_init: libc::c_int = 0;
    let mut out_open: libc::c_int = 0;
    major =
        gss_inquire_context(&mut minor, context, &mut out_init_name,
                            &mut out_accept_name, &mut out_lifetime,
                            &mut out_mech_type, &mut out_flags,
                            &mut out_locally_init, &mut out_open);
    check_gsserr(b"gss_inquire_context\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    if gss_oid_equal(out_mech_type as gss_const_OID,
                     expected_mech as gss_const_OID) != 0 {
    } else {
        __assert_fail(b"gss_oid_equal(out_mech_type, expected_mech)\x00" as
                          *const u8 as *const libc::c_char,
                      b"t_inq_ctx.c\x00" as *const u8 as *const libc::c_char,
                      61 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 67],
                                                &[libc::c_char; 67]>(b"void check_inq_context(gss_ctx_id_t, int, gss_OID, OM_uint32, int)\x00")).as_ptr());
    }
    if out_flags == expected_flags {
    } else {
        __assert_fail(b"out_flags == expected_flags\x00" as *const u8 as
                          *const libc::c_char,
                      b"t_inq_ctx.c\x00" as *const u8 as *const libc::c_char,
                      62 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 67],
                                                &[libc::c_char; 67]>(b"void check_inq_context(gss_ctx_id_t, int, gss_OID, OM_uint32, int)\x00")).as_ptr());
    }
    if out_locally_init == expected_locally_init {
    } else {
        __assert_fail(b"out_locally_init == expected_locally_init\x00" as
                          *const u8 as *const libc::c_char,
                      b"t_inq_ctx.c\x00" as *const u8 as *const libc::c_char,
                      63 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 67],
                                                &[libc::c_char; 67]>(b"void check_inq_context(gss_ctx_id_t, int, gss_OID, OM_uint32, int)\x00")).as_ptr());
    }
    if incomplete != 0 {
        if out_open == 0 {
        } else {
            __assert_fail(b"!out_open\x00" as *const u8 as
                              *const libc::c_char,
                          b"t_inq_ctx.c\x00" as *const u8 as
                              *const libc::c_char,
                          65 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 67],
                                                    &[libc::c_char; 67]>(b"void check_inq_context(gss_ctx_id_t, int, gss_OID, OM_uint32, int)\x00")).as_ptr());
        }
        if out_lifetime == 0 as libc::c_int as libc::c_uint {
        } else {
            __assert_fail(b"out_lifetime == 0\x00" as *const u8 as
                              *const libc::c_char,
                          b"t_inq_ctx.c\x00" as *const u8 as
                              *const libc::c_char,
                          66 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 67],
                                                    &[libc::c_char; 67]>(b"void check_inq_context(gss_ctx_id_t, int, gss_OID, OM_uint32, int)\x00")).as_ptr());
        }
        if out_init_name.is_null() {
        } else {
            __assert_fail(b"out_init_name == GSS_C_NO_NAME\x00" as *const u8
                              as *const libc::c_char,
                          b"t_inq_ctx.c\x00" as *const u8 as
                              *const libc::c_char,
                          67 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 67],
                                                    &[libc::c_char; 67]>(b"void check_inq_context(gss_ctx_id_t, int, gss_OID, OM_uint32, int)\x00")).as_ptr());
        }
        if out_accept_name.is_null() {
        } else {
            __assert_fail(b"out_accept_name == GSS_C_NO_NAME\x00" as *const u8
                              as *const libc::c_char,
                          b"t_inq_ctx.c\x00" as *const u8 as
                              *const libc::c_char,
                          68 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 67],
                                                    &[libc::c_char; 67]>(b"void check_inq_context(gss_ctx_id_t, int, gss_OID, OM_uint32, int)\x00")).as_ptr());
        }
    } else {
        if out_open != 0 {
        } else {
            __assert_fail(b"out_open\x00" as *const u8 as *const libc::c_char,
                          b"t_inq_ctx.c\x00" as *const u8 as
                              *const libc::c_char,
                          70 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 67],
                                                    &[libc::c_char; 67]>(b"void check_inq_context(gss_ctx_id_t, int, gss_OID, OM_uint32, int)\x00")).as_ptr());
        }
        if out_lifetime > 0 as libc::c_int as libc::c_uint {
        } else {
            __assert_fail(b"out_lifetime > 0\x00" as *const u8 as
                              *const libc::c_char,
                          b"t_inq_ctx.c\x00" as *const u8 as
                              *const libc::c_char,
                          71 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 67],
                                                    &[libc::c_char; 67]>(b"void check_inq_context(gss_ctx_id_t, int, gss_OID, OM_uint32, int)\x00")).as_ptr());
        }
        if !out_init_name.is_null() {
        } else {
            __assert_fail(b"out_init_name != GSS_C_NO_NAME\x00" as *const u8
                              as *const libc::c_char,
                          b"t_inq_ctx.c\x00" as *const u8 as
                              *const libc::c_char,
                          72 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 67],
                                                    &[libc::c_char; 67]>(b"void check_inq_context(gss_ctx_id_t, int, gss_OID, OM_uint32, int)\x00")).as_ptr());
        }
        if !out_accept_name.is_null() {
        } else {
            __assert_fail(b"out_accept_name != GSS_C_NO_NAME\x00" as *const u8
                              as *const libc::c_char,
                          b"t_inq_ctx.c\x00" as *const u8 as
                              *const libc::c_char,
                          73 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 67],
                                                    &[libc::c_char; 67]>(b"void check_inq_context(gss_ctx_id_t, int, gss_OID, OM_uint32, int)\x00")).as_ptr());
        }
    }
    gss_release_name(&mut minor, &mut out_accept_name);
    gss_release_name(&mut minor, &mut out_init_name);
}
/* Call gss_init_sec_context() once to create an initiator context (which will
 * be partial if flags includes GSS_C_MUTUAL_FLAG and the mech is krb5). */
#[c2rust::src_loc = "82:1"]
unsafe extern "C" fn start_init_context(mut mech: gss_OID,
                                        mut cred: gss_cred_id_t,
                                        mut tname: gss_name_t,
                                        mut flags: OM_uint32,
                                        mut ctx: *mut gss_ctx_id_t) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut itok: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    *ctx = 0 as gss_ctx_id_t;
    major =
        gss_init_sec_context(&mut minor, cred, ctx, tname, mech, flags,
                             0xffffffff as libc::c_ulong as OM_uint32,
                             0 as gss_channel_bindings_t, 0 as gss_buffer_t,
                             0 as *mut gss_OID, &mut itok,
                             0 as *mut OM_uint32, 0 as *mut OM_uint32);
    check_gsserr(b"gss_init_sec_context\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    gss_release_buffer(&mut minor, &mut itok);
}
/* Call gss_init_sec_context() and gss_accept_sec_context() once to create an
 * acceptor context. */
#[c2rust::src_loc = "99:1"]
unsafe extern "C" fn start_accept_context(mut mech: gss_OID,
                                          mut icred: gss_cred_id_t,
                                          mut acred: gss_cred_id_t,
                                          mut tname: gss_name_t,
                                          mut flags: OM_uint32,
                                          mut ctx: *mut gss_ctx_id_t) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut ictx: gss_ctx_id_t = 0 as gss_ctx_id_t;
    let mut itok: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut atok: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    major =
        gss_init_sec_context(&mut minor, icred, &mut ictx, tname, mech, flags,
                             0xffffffff as libc::c_ulong as OM_uint32,
                             0 as gss_channel_bindings_t, 0 as gss_buffer_t,
                             0 as *mut gss_OID, &mut itok,
                             0 as *mut OM_uint32, 0 as *mut OM_uint32);
    check_gsserr(b"gss_init_sec_context\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    *ctx = 0 as gss_ctx_id_t;
    major =
        gss_accept_sec_context(&mut minor, ctx, acred, &mut itok,
                               0 as gss_channel_bindings_t,
                               0 as *mut gss_name_t, 0 as *mut gss_OID,
                               &mut atok, 0 as *mut OM_uint32,
                               0 as *mut OM_uint32, 0 as *mut gss_cred_id_t);
    check_gsserr(b"gss_accept_sec_context\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    gss_release_buffer(&mut minor, &mut itok);
    gss_release_buffer(&mut minor, &mut atok);
    gss_delete_sec_context(&mut minor, &mut ictx, 0 as gss_buffer_t);
}
#[c2rust::src_loc = "123:1"]
unsafe extern "C" fn partial_iakerb_acceptor(mut username:
                                                 *const libc::c_char,
                                             mut password:
                                                 *const libc::c_char,
                                             mut tname: gss_name_t,
                                             mut flags: OM_uint32,
                                             mut ctx: *mut gss_ctx_id_t) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut name: gss_name_t = 0 as *mut gss_name_struct;
    let mut ubuf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut pwbuf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut mechlist: gss_OID_set_desc =
        gss_OID_set_desc{count: 0, elements: 0 as *mut gss_OID_desc_struct,};
    let mut icred: gss_cred_id_t = 0 as *mut gss_cred_id_struct;
    let mut acred: gss_cred_id_t = 0 as *mut gss_cred_id_struct;
    mechlist.count = 1 as libc::c_int as size_t;
    mechlist.elements = &mut mech_iakerb;
    /* Import the username. */
    ubuf.value = username as *mut libc::c_void;
    ubuf.length = strlen(username);
    major =
        gss_import_name(&mut minor, &mut ubuf, GSS_C_NT_USER_NAME, &mut name);
    check_gsserr(b"gss_import_name\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    /* Create an IAKERB initiator cred with the username and password. */
    pwbuf.value = password as *mut libc::c_void;
    pwbuf.length = strlen(password);
    major =
        gss_acquire_cred_with_password(&mut minor, name, &mut pwbuf,
                                       0 as libc::c_int as OM_uint32,
                                       &mut mechlist, 1 as libc::c_int,
                                       &mut icred, 0 as *mut gss_OID_set,
                                       0 as *mut OM_uint32);
    check_gsserr(b"gss_acquire_cred_with_password\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    /* Create an acceptor cred with support for IAKERB. */
    major =
        gss_acquire_cred(&mut minor, 0 as gss_name_t,
                         0xffffffff as libc::c_ulong as OM_uint32,
                         &mut mechlist, 2 as libc::c_int, &mut acred,
                         0 as *mut gss_OID_set, 0 as *mut OM_uint32);
    check_gsserr(b"gss_acquire_cred\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    /* Begin context establishment to get a partial acceptor context. */
    start_accept_context(&mut mech_iakerb, icred, acred, tname, flags, ctx);
    gss_release_name(&mut minor, &mut name);
    gss_release_cred(&mut minor, &mut icred);
    gss_release_cred(&mut minor, &mut acred);
}
/* Create a partially established SPNEGO acceptor. */
#[c2rust::src_loc = "164:1"]
unsafe extern "C" fn partial_spnego_acceptor(mut tname: gss_name_t,
                                             mut ctx: *mut gss_ctx_id_t) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut itok: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut atok: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    /*
     * We could construct a fixed SPNEGO initiator token which forces a
     * renegotiation, but a simpler approach is to pass an empty token to
     * gss_accept_sec_context(), taking advantage of our compatibility support
     * for SPNEGO NegHints.
     */
    *ctx = 0 as gss_ctx_id_t;
    major =
        gss_accept_sec_context(&mut minor, ctx, 0 as gss_cred_id_t, &mut itok,
                               0 as gss_channel_bindings_t,
                               0 as *mut gss_name_t, 0 as *mut gss_OID,
                               &mut atok, 0 as *mut OM_uint32,
                               0 as *mut OM_uint32, 0 as *mut gss_cred_id_t);
    check_gsserr(b"gss_accept_sec_context(neghints)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    gss_release_buffer(&mut minor, &mut atok);
}
#[c2rust::src_loc = "185:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut minor: OM_uint32 = 0;
    let mut flags: OM_uint32 = 0;
    let mut dce_flags: OM_uint32 = 0;
    let mut tname: gss_name_t = 0 as *mut gss_name_struct;
    let mut ictx: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut actx: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut username: *const libc::c_char = 0 as *const libc::c_char;
    let mut password: *const libc::c_char = 0 as *const libc::c_char;
    if argc != 4 as libc::c_int {
        fprintf(stderr,
                b"Usage: %s username password targetname\n\x00" as *const u8
                    as *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize));
        return 1 as libc::c_int
    }
    username = *argv.offset(1 as libc::c_int as isize);
    password = *argv.offset(2 as libc::c_int as isize);
    tname = import_name(*argv.offset(3 as libc::c_int as isize));
    flags =
        (8 as libc::c_int | 2 as libc::c_int | 16 as libc::c_int |
             32 as libc::c_int) as OM_uint32;
    start_init_context(&mut mech_krb5, 0 as gss_cred_id_t, tname, flags,
                       &mut ictx);
    check_inq_context(ictx, 1 as libc::c_int, &mut mech_krb5,
                      flags | 256 as libc::c_int as libc::c_uint,
                      1 as libc::c_int);
    gss_delete_sec_context(&mut minor, &mut ictx, 0 as gss_buffer_t);
    start_init_context(&mut mech_iakerb, 0 as gss_cred_id_t, tname, flags,
                       &mut ictx);
    check_inq_context(ictx, 1 as libc::c_int, &mut mech_iakerb, flags,
                      1 as libc::c_int);
    gss_delete_sec_context(&mut minor, &mut ictx, 0 as gss_buffer_t);
    start_init_context(&mut mech_spnego, 0 as gss_cred_id_t, tname, flags,
                       &mut ictx);
    check_inq_context(ictx, 1 as libc::c_int, &mut mech_spnego, flags,
                      1 as libc::c_int);
    gss_delete_sec_context(&mut minor, &mut ictx, 0 as gss_buffer_t);
    dce_flags = flags | 0x1000 as libc::c_int as libc::c_uint;
    start_accept_context(&mut mech_krb5, 0 as gss_cred_id_t,
                         0 as gss_cred_id_t, tname, dce_flags, &mut actx);
    check_inq_context(actx, 1 as libc::c_int, &mut mech_krb5,
                      dce_flags | 256 as libc::c_int as libc::c_uint,
                      0 as libc::c_int);
    gss_delete_sec_context(&mut minor, &mut actx, 0 as gss_buffer_t);
    partial_iakerb_acceptor(username, password, tname, flags, &mut actx);
    check_inq_context(actx, 1 as libc::c_int, &mut mech_iakerb,
                      0 as libc::c_int as OM_uint32, 0 as libc::c_int);
    gss_delete_sec_context(&mut minor, &mut actx, 0 as gss_buffer_t);
    partial_spnego_acceptor(tname, &mut actx);
    check_inq_context(actx, 1 as libc::c_int, &mut mech_spnego,
                      0 as libc::c_int as OM_uint32, 0 as libc::c_int);
    gss_delete_sec_context(&mut minor, &mut actx, 0 as gss_buffer_t);
    establish_contexts(&mut mech_krb5, 0 as gss_cred_id_t, 0 as gss_cred_id_t,
                       tname, flags, &mut ictx, &mut actx,
                       0 as *mut gss_name_t, 0 as *mut gss_OID,
                       0 as *mut gss_cred_id_t);
    check_inq_context(ictx, 0 as libc::c_int, &mut mech_krb5,
                      flags | 256 as libc::c_int as libc::c_uint,
                      1 as libc::c_int);
    check_inq_context(actx, 0 as libc::c_int, &mut mech_krb5,
                      flags | 256 as libc::c_int as libc::c_uint |
                          128 as libc::c_int as libc::c_uint,
                      0 as libc::c_int);
    gss_delete_sec_context(&mut minor, &mut ictx, 0 as gss_buffer_t);
    gss_delete_sec_context(&mut minor, &mut actx, 0 as gss_buffer_t);
    gss_release_name(&mut minor, &mut tname);
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
