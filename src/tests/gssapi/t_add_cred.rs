use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:40"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:40"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:43"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:43"]
pub mod gssapi_h {
    #[c2rust::src_loc = "79:1"]
    pub type gss_name_t = *mut gss_name_struct;
    #[c2rust::src_loc = "82:1"]
    pub type gss_cred_id_t = *mut gss_cred_id_struct;
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
    pub type gss_OID_set = *mut gss_OID_set_desc_struct;
    #[c2rust::src_loc = "135:1"]
    pub type gss_cred_usage_t = libc::c_int;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "78:8"]
        pub type gss_name_struct;
        #[c2rust::src_loc = "81:8"]
        pub type gss_cred_id_struct;
        /* time_rec */
        #[no_mangle]
        #[c2rust::src_loc = "432:1"]
        pub fn gss_release_cred(_: *mut OM_uint32, _: *mut gss_cred_id_t)
         -> OM_uint32;
        /* output_name */
        #[no_mangle]
        #[c2rust::src_loc = "569:1"]
        pub fn gss_release_name(_: *mut OM_uint32, _: *mut gss_name_t)
         -> OM_uint32;
        /* output_name */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "633:1"]
        pub fn gss_add_cred(_: *mut OM_uint32, _: gss_cred_id_t,
                            _: gss_name_t, _: gss_OID, _: gss_cred_usage_t,
                            _: OM_uint32, _: OM_uint32, _: *mut gss_cred_id_t,
                            _: *mut gss_OID_set, _: *mut OM_uint32,
                            _: *mut OM_uint32) -> OM_uint32;
        /* acceptor_time_rec */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "648:1"]
        pub fn gss_inquire_cred_by_mech(_: *mut OM_uint32, _: gss_cred_id_t,
                                        _: gss_OID, _: *mut gss_name_t,
                                        _: *mut OM_uint32, _: *mut OM_uint32,
                                        _: *mut gss_cred_usage_t)
         -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src = "/usr/include/assert.h:41"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/tests/gssapi/common.h:43"]
pub mod common_h {
    use super::gssapi_h::{gss_name_t, gss_OID_desc, OM_uint32};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "56:1"]
        pub fn import_name(str: *const libc::c_char) -> gss_name_t;
        #[no_mangle]
        #[c2rust::src_loc = "40:21"]
        pub static mut mech_iakerb: gss_OID_desc;
        #[no_mangle]
        #[c2rust::src_loc = "38:21"]
        pub static mut mech_krb5: gss_OID_desc;
    }
    /* COMMON_H */
}
pub use self::stddef_h::size_t;
pub use self::types_h::__uint32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssapi_h::{gss_name_t, gss_cred_id_t, gss_uint32, OM_uint32,
                         gss_OID_desc_struct, gss_OID_desc, gss_OID,
                         gss_OID_set_desc_struct, gss_OID_set,
                         gss_cred_usage_t, gss_name_struct,
                         gss_cred_id_struct, gss_release_cred,
                         gss_release_name, gss_add_cred,
                         gss_inquire_cred_by_mech};
use self::assert_h::__assert_fail;
use self::common_h::{import_name, mech_iakerb, mech_krb5};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/gssapi/t_add_cred.c - gss_add_cred() tests */
/*
 * Copyright (C) 2018 by the Massachusetts Institute of Technology.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * * Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.
 *
 * * Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in
 *   the documentation and/or other materials provided with the
 *   distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/*
 * This program tests the mechglue behavior of gss_add_cred().  It relies on a
 * krb5 keytab and credentials being present so that initiator and acceptor
 * credentials can be acquired, but does not use them to initiate or accept any
 * requests.
 */
#[c2rust::src_loc = "45:1"]
unsafe fn main_0() -> libc::c_int {
    let mut minor: OM_uint32 = 0;
    let mut major: OM_uint32 = 0;
    let mut cred1: gss_cred_id_t = 0 as *mut gss_cred_id_struct;
    let mut cred2: gss_cred_id_t = 0 as *mut gss_cred_id_struct;
    let mut usage: gss_cred_usage_t = 0;
    let mut name: gss_name_t = 0 as *mut gss_name_struct;
    /* Check that we get the expected error if we pass neither an input nor an
     * output cred handle. */
    major =
        gss_add_cred(&mut minor, 0 as gss_cred_id_t, 0 as gss_name_t,
                     &mut mech_krb5, 1 as libc::c_int,
                     0xffffffff as libc::c_ulong as OM_uint32,
                     0xffffffff as libc::c_ulong as OM_uint32,
                     0 as *mut gss_cred_id_t, 0 as *mut gss_OID_set,
                     0 as *mut OM_uint32, 0 as *mut OM_uint32);
    if major ==
           (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
               (7 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
    } else {
        __assert_fail(b"major == (GSS_S_CALL_INACCESSIBLE_WRITE | GSS_S_NO_CRED)\x00"
                          as *const u8 as *const libc::c_char,
                      b"t_add_cred.c\x00" as *const u8 as *const libc::c_char,
                      58 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"int main()\x00")).as_ptr());
    }
    /* Regression test for #8737: make sure that desired_name is honored when
     * creating a credential by passing in a non-matching name. */
    name =
        import_name(b"p:does/not/match@WRONG_REALM\x00" as *const u8 as
                        *const libc::c_char);
    major =
        gss_add_cred(&mut minor, 0 as gss_cred_id_t, name, &mut mech_krb5,
                     1 as libc::c_int,
                     0xffffffff as libc::c_ulong as OM_uint32,
                     0xffffffff as libc::c_ulong as OM_uint32, &mut cred1,
                     0 as *mut gss_OID_set, 0 as *mut OM_uint32,
                     0 as *mut OM_uint32);
    if major == (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
    } else {
        __assert_fail(b"major == GSS_S_CRED_UNAVAIL\x00" as *const u8 as
                          *const libc::c_char,
                      b"t_add_cred.c\x00" as *const u8 as *const libc::c_char,
                      66 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"int main()\x00")).as_ptr());
    }
    gss_release_name(&mut minor, &mut name);
    /* Create cred1 with a krb5 initiator cred by passing an output handle but
     * no input handle. */
    major =
        gss_add_cred(&mut minor, 0 as gss_cred_id_t, 0 as gss_name_t,
                     &mut mech_krb5, 1 as libc::c_int,
                     0xffffffff as libc::c_ulong as OM_uint32,
                     0xffffffff as libc::c_ulong as OM_uint32, &mut cred1,
                     0 as *mut gss_OID_set, 0 as *mut OM_uint32,
                     0 as *mut OM_uint32);
    if major == 0 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"major == GSS_S_COMPLETE\x00" as *const u8 as
                          *const libc::c_char,
                      b"t_add_cred.c\x00" as *const u8 as *const libc::c_char,
                      74 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"int main()\x00")).as_ptr());
    }
    /* Verify that cred1 has the expected mechanism creds. */
    major =
        gss_inquire_cred_by_mech(&mut minor, cred1, &mut mech_krb5,
                                 0 as *mut gss_name_t, 0 as *mut OM_uint32,
                                 0 as *mut OM_uint32, &mut usage);
    if major == 0 as libc::c_int as libc::c_uint && usage == 1 as libc::c_int
       {
    } else {
        __assert_fail(b"major == GSS_S_COMPLETE && usage == GSS_C_INITIATE\x00"
                          as *const u8 as *const libc::c_char,
                      b"t_add_cred.c\x00" as *const u8 as *const libc::c_char,
                      79 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"int main()\x00")).as_ptr());
    }
    major =
        gss_inquire_cred_by_mech(&mut minor, cred1, &mut mech_iakerb,
                                 0 as *mut gss_name_t, 0 as *mut OM_uint32,
                                 0 as *mut OM_uint32, &mut usage);
    if major == (7 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
    } else {
        __assert_fail(b"major == GSS_S_NO_CRED\x00" as *const u8 as
                          *const libc::c_char,
                      b"t_add_cred.c\x00" as *const u8 as *const libc::c_char,
                      82 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"int main()\x00")).as_ptr());
    }
    /* Check that we get the expected error if we try to add another krb5 mech
     * cred to cred1. */
    major =
        gss_add_cred(&mut minor, cred1, 0 as gss_name_t, &mut mech_krb5,
                     1 as libc::c_int,
                     0xffffffff as libc::c_ulong as OM_uint32,
                     0xffffffff as libc::c_ulong as OM_uint32,
                     0 as *mut gss_cred_id_t, 0 as *mut gss_OID_set,
                     0 as *mut OM_uint32, 0 as *mut OM_uint32);
    if major == (17 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
    } else {
        __assert_fail(b"major == GSS_S_DUPLICATE_ELEMENT\x00" as *const u8 as
                          *const libc::c_char,
                      b"t_add_cred.c\x00" as *const u8 as *const libc::c_char,
                      89 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"int main()\x00")).as_ptr());
    }
    /* Add an IAKERB acceptor mech cred to cred1. */
    major =
        gss_add_cred(&mut minor, cred1, 0 as gss_name_t, &mut mech_iakerb,
                     2 as libc::c_int,
                     0xffffffff as libc::c_ulong as OM_uint32,
                     0xffffffff as libc::c_ulong as OM_uint32,
                     0 as *mut gss_cred_id_t, 0 as *mut gss_OID_set,
                     0 as *mut OM_uint32, 0 as *mut OM_uint32);
    if major == 0 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"major == GSS_S_COMPLETE\x00" as *const u8 as
                          *const libc::c_char,
                      b"t_add_cred.c\x00" as *const u8 as *const libc::c_char,
                      95 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"int main()\x00")).as_ptr());
    }
    /* Verify cred1 mechanism creds. */
    major =
        gss_inquire_cred_by_mech(&mut minor, cred1, &mut mech_krb5,
                                 0 as *mut gss_name_t, 0 as *mut OM_uint32,
                                 0 as *mut OM_uint32, &mut usage);
    if major == 0 as libc::c_int as libc::c_uint && usage == 1 as libc::c_int
       {
    } else {
        __assert_fail(b"major == GSS_S_COMPLETE && usage == GSS_C_INITIATE\x00"
                          as *const u8 as *const libc::c_char,
                      b"t_add_cred.c\x00" as *const u8 as *const libc::c_char,
                      100 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"int main()\x00")).as_ptr());
    }
    major =
        gss_inquire_cred_by_mech(&mut minor, cred1, &mut mech_iakerb,
                                 0 as *mut gss_name_t, 0 as *mut OM_uint32,
                                 0 as *mut OM_uint32, &mut usage);
    if major == 0 as libc::c_int as libc::c_uint && usage == 2 as libc::c_int
       {
    } else {
        __assert_fail(b"major == GSS_S_COMPLETE && usage == GSS_C_ACCEPT\x00"
                          as *const u8 as *const libc::c_char,
                      b"t_add_cred.c\x00" as *const u8 as *const libc::c_char,
                      103 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"int main()\x00")).as_ptr());
    }
    /* Start over with another new cred. */
    gss_release_cred(&mut minor, &mut cred1);
    major =
        gss_add_cred(&mut minor, 0 as gss_cred_id_t, 0 as gss_name_t,
                     &mut mech_krb5, 2 as libc::c_int,
                     0xffffffff as libc::c_ulong as OM_uint32,
                     0xffffffff as libc::c_ulong as OM_uint32, &mut cred1,
                     0 as *mut gss_OID_set, 0 as *mut OM_uint32,
                     0 as *mut OM_uint32);
    if major == 0 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"major == GSS_S_COMPLETE\x00" as *const u8 as
                          *const libc::c_char,
                      b"t_add_cred.c\x00" as *const u8 as *const libc::c_char,
                      110 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"int main()\x00")).as_ptr());
    }
    /* Create an expanded cred by passing both an output handle and an input
     * handle. */
    major =
        gss_add_cred(&mut minor, cred1, 0 as gss_name_t, &mut mech_iakerb,
                     1 as libc::c_int,
                     0xffffffff as libc::c_ulong as OM_uint32,
                     0xffffffff as libc::c_ulong as OM_uint32, &mut cred2,
                     0 as *mut gss_OID_set, 0 as *mut OM_uint32,
                     0 as *mut OM_uint32);
    if major == 0 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"major == GSS_S_COMPLETE\x00" as *const u8 as
                          *const libc::c_char,
                      b"t_add_cred.c\x00" as *const u8 as *const libc::c_char,
                      117 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"int main()\x00")).as_ptr());
    }
    /* Verify mechanism creds in cred1 and cred2. */
    major =
        gss_inquire_cred_by_mech(&mut minor, cred1, &mut mech_krb5,
                                 0 as *mut gss_name_t, 0 as *mut OM_uint32,
                                 0 as *mut OM_uint32, &mut usage);
    if major == 0 as libc::c_int as libc::c_uint && usage == 2 as libc::c_int
       {
    } else {
        __assert_fail(b"major == GSS_S_COMPLETE && usage == GSS_C_ACCEPT\x00"
                          as *const u8 as *const libc::c_char,
                      b"t_add_cred.c\x00" as *const u8 as *const libc::c_char,
                      122 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"int main()\x00")).as_ptr());
    }
    major =
        gss_inquire_cred_by_mech(&mut minor, cred1, &mut mech_iakerb,
                                 0 as *mut gss_name_t, 0 as *mut OM_uint32,
                                 0 as *mut OM_uint32, &mut usage);
    if major == (7 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
    } else {
        __assert_fail(b"major == GSS_S_NO_CRED\x00" as *const u8 as
                          *const libc::c_char,
                      b"t_add_cred.c\x00" as *const u8 as *const libc::c_char,
                      125 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"int main()\x00")).as_ptr());
    }
    major =
        gss_inquire_cred_by_mech(&mut minor, cred2, &mut mech_krb5,
                                 0 as *mut gss_name_t, 0 as *mut OM_uint32,
                                 0 as *mut OM_uint32, &mut usage);
    if major == 0 as libc::c_int as libc::c_uint && usage == 2 as libc::c_int
       {
    } else {
        __assert_fail(b"major == GSS_S_COMPLETE && usage == GSS_C_ACCEPT\x00"
                          as *const u8 as *const libc::c_char,
                      b"t_add_cred.c\x00" as *const u8 as *const libc::c_char,
                      128 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"int main()\x00")).as_ptr());
    }
    major =
        gss_inquire_cred_by_mech(&mut minor, cred2, &mut mech_iakerb,
                                 0 as *mut gss_name_t, 0 as *mut OM_uint32,
                                 0 as *mut OM_uint32, &mut usage);
    if major == 0 as libc::c_int as libc::c_uint && usage == 1 as libc::c_int
       {
    } else {
        __assert_fail(b"major == GSS_S_COMPLETE && usage == GSS_C_INITIATE\x00"
                          as *const u8 as *const libc::c_char,
                      b"t_add_cred.c\x00" as *const u8 as *const libc::c_char,
                      131 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"int main()\x00")).as_ptr());
    }
    gss_release_cred(&mut minor, &mut cred1);
    gss_release_cred(&mut minor, &mut cred2);
    return 0 as libc::c_int;
}
#[main]
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }
