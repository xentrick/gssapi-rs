use ::libc;

pub mod k5_platform_h {

    /* -*- mode: c; indent-tabs-mode: nil -*- */
    /* include/k5-platform.h */
    /*
     * Copyright 2003, 2004, 2005, 2007, 2008, 2009 Massachusetts Institute of Technology.
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
     * Some platform-dependent definitions to sync up the C support level.
     * Some to a C99-ish level, some related utility code.
     *
     * Currently:
     * + [u]int{8,16,32}_t types
     * + 64-bit types and load/store code
     * + SIZE_MAX
     * + shared library init/fini hooks
     * + consistent getpwnam/getpwuid interfaces
     * + va_copy fudged if not provided
     * + strlcpy/strlcat
     * + fnmatch
     * + [v]asprintf
     * + strerror_r
     * + mkstemp
     * + zap (support function and macro)
     * + constant time memory comparison
     * + path manipulation
     * + _, N_, dgettext, bindtextdomain (for localization)
     * + getopt_long
     * + secure_getenv
     * + fetching filenames from a directory
     */
    /* This attribute prevents unused function warnings in gcc and clang. */
    /* Initialization and finalization function support for libraries.

    At top level, before the functions are defined or even declared:
    MAKE_INIT_FUNCTION(init_fn);
    MAKE_FINI_FUNCTION(fini_fn);
    Then:
    int init_fn(void) { ... }
    void fini_fn(void) { if (INITIALIZER_RAN(init_fn)) ... }
    In code, in the same file:
    err = CALL_INIT_FUNCTION(init_fn);

    To trigger or verify the initializer invocation from another file,
    a helper function must be created.

    This model handles both the load-time execution (Windows) and
    delayed execution (pthread_once) approaches, and should be able to
    guarantee in both cases that the init function is run once, in one
    thread, before other stuff in the library is done; furthermore, the
    finalization code should only run if the initialization code did.
    (Maybe I could've made the "if INITIALIZER_RAN" test implicit, via
    another function hidden in macros, but this is hairy enough
    already.)

    The init_fn and fini_fn names should be chosen such that any
    exported names staring with those names, and optionally followed by
    additional characters, fits in with any namespace constraints on
    the library in question.


    There's also PROGRAM_EXITING() currently always defined as zero.
    If there's some trivial way to find out if the fini function is
    being called because the program that the library is linked into is
    exiting, we can just skip all the work because the resources are
    about to be freed up anyways.  Generally this is likely to be the
    same as distinguishing whether the library was loaded dynamically
    while the program was running, or loaded as part of program
    startup.  On most platforms, I don't think we can distinguish these
    cases easily, and it's probably not worth expending any significant
    effort.  (Note in particular that atexit() won't do, because if the
    library is explicitly loaded and unloaded, it would have to be able
    to deregister the atexit callback function.  Also, the system limit
    on atexit callbacks may be small.)


    Implementation outline:

    Windows: MAKE_FINI_FUNCTION creates a symbol with a magic name that
    is sought at library build time, and code is added to invoke the
    function when the library is unloaded.  MAKE_INIT_FUNCTION does
    likewise, but the function is invoked when the library is loaded,
    and an extra variable is declared to hold an error code and a "yes
    the initializer ran" flag.  CALL_INIT_FUNCTION blows up if the flag
    isn't set, otherwise returns the error code.

    UNIX: MAKE_INIT_FUNCTION creates and initializes a variable with a
    name derived from the function name, containing a k5_once_t
    (pthread_once_t or int), an error code, and a pointer to the
    function.  The function itself is declared static, but the
    associated variable has external linkage.  CALL_INIT_FUNCTION
    ensures thath the function is called exactly once (pthread_once or
    just check the flag) and returns the stored error code (or the
    pthread_once error).

    (That's the basic idea.  With some debugging assert() calls and
    such, it's a bit more complicated.  And we also need to handle
    doing the pthread test at run time on systems where that works, so
    we use the k5_once_t stuff instead.)

    UNIX, with compiler support: MAKE_FINI_FUNCTION declares the
    function as a destructor, and the run time linker support or
    whatever will cause it to be invoked when the library is unloaded,
    the program ends, etc.

    UNIX, with linker support: MAKE_FINI_FUNCTION creates a symbol with
    a magic name that is sought at library build time, and linker
    options are used to mark it as a finalization function for the
    library.  The symbol must be exported.

    UNIX, no library finalization support: The finalization function
    never runs, and we leak memory.  Tough.

    DELAY_INITIALIZER will be defined by the configure script if we
    want to use k5_once instead of load-time initialization.  That'll
    be the preferred method on most systems except Windows, where we
    have to initialize some mutexes.




    For maximum flexibility in defining the macros, the function name
    parameter should be a simple name, not even a macro defined as
    another name.  The function should have a unique name, and should
    conform to whatever namespace is used by the library in question.
    (We do have export lists, but (1) they're not used for all
    platforms, and (2) they're not used for static libraries.)

    If the macro expansion needs the function to have been declared, it
    must include a declaration.  If it is not necessary for the symbol
    name to be exported from the object file, the macro should declare
    it as "static".  Hence the signature must exactly match "void
    foo(void)".  (ANSI C allows a static declaration followed by a
    non-static one; the result is internal linkage.)  The macro
    expansion has to come before the function, because gcc apparently
    won't act on "__attribute__((constructor))" if it comes after the
    function definition.

    This is going to be compiler- and environment-specific, and may
    require some support at library build time, and/or "asm"
    statements.  But through macro expansion and auxiliary functions,
    we should be able to handle most things except #pragma.

    It's okay for this code to require that the library be built
    with the same compiler and compiler options throughout, but
    we shouldn't require that the library and application use the
    same compiler.

    For static libraries, we don't really care about cleanup too much,
    since it's all memory handling and mutex allocation which will all
    be cleaned up when the program exits.  Thus, it's okay if gcc-built
    static libraries don't play nicely with cc-built executables when
    it comes to static constructors, just as long as it doesn't cause
    linking to fail.

    For dynamic libraries on UNIX, we'll use pthread_once-type support
    to do delayed initialization, so if finalization can't be made to
    work, we'll only have memory leaks in a load/use/unload cycle.  If
    anyone (like, say, the OS vendor) complains about this, they can
    tell us how to get a shared library finalization function invoked
    automatically.

    Currently there's --disable-delayed-initialization for preventing
    the initialization from being delayed on UNIX, but that's mainly
    just for testing the linker options for initialization, and will
    probably be removed at some point.  */
    /* Helper macros.  */
    /* XXX Should test USE_LINKER_INIT_OPTION early, and if it's set,
    always provide a function by the expected name, even if we're
    delaying initialization.  */
    /* Run the initialization code during program execution, at the latest
    possible moment.  This means multiple threads may be active.  */
    /* Do it in macro form so we get the file/line of the invocation if
    the assertion fails.  */
    /* forward declaration for use in initializer */
    /* so ';' following macro use won't get error */
    /* This should be called in finalization only, so we shouldn't have
    multiple active threads mucking around in our library at this
    point.  So ignore the once_t object and just look at the flag.

    XXX Could we have problems with memory coherence between processors
    if we don't invoke mutex/once routines?  Probably not, the
    application code should already be coordinating things such that
    the library code is not in use by this point, and memory
    synchronization will be needed there.  */
    /* If we're using gcc, if the C++ support works, the compiler should
    build executables and shared libraries that support the use of
    static constructors and destructors.  The C compiler supports a
    function attribute that makes use of the same facility as C++.

    XXX How do we know if the C++ support actually works?  */
    /* Read and write integer values as (unaligned) octet strings in
    specific byte orders.  Add per-platform optimizations as
    needed.  */
    /* Check for BIG/LITTLE_ENDIAN macros.  If exactly one is defined, use
    it.  If both are defined, then BYTE_ORDER should be defined and
    match one of them.  Try those symbols, then try again with an
    underscore prefix.  */
    /* Optimize for GCC on platforms with known byte orders.

    GCC's packed structures can be written to with any alignment; the
    compiler will use byte operations, unaligned-word operations, or
    normal memory ops as appropriate for the architecture.

    This assumes the availability of uint##_t types, which should work
    on most of our platforms except Windows, where we're not using
    GCC.  */
    /* To do: Define SWAP16, SWAP32, SWAP64 macros to byte-swap values
    with the indicated numbers of bits.

    Linux: byteswap.h, bswap_16 etc.
    Solaris 10: none
    macOS: machine/endian.h or byte_order.h, NXSwap{Short,Int,LongLong}
    NetBSD: sys/bswap.h, bswap16 etc.  */
    /* Note that on Windows at least this file can be included from C++
    source, so casts *from* void* are required.  */
    #[inline]

    pub unsafe extern "C" fn load_32_be(mut cvp: *const libc::c_void) -> u32 {
        let mut p = cvp as *const u8;
        return __bswap_32((*(p as *const crate::k5_platform_h::C2RustUnnamed_6)).i);
    }

    use crate::src::mechglue::g_imp_cred::byteswap_h::__bswap_32;
    /* K5_PLATFORM_H */
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
pub use crate::k5_platform_h::C2RustUnnamed_6;
pub use crate::mglueP_h::gss_config;
pub use crate::mglueP_h::gss_cred_id_struct;
pub use crate::mglueP_h::gss_mechanism;
pub use crate::mglueP_h::gss_name_struct;
pub use crate::mglueP_h::gss_union_cred_t;

pub use crate::src::mechglue::g_imp_cred::byteswap_h::__bswap_32;
pub use crate::src::mechglue::g_imp_cred::k5_platform_h::load_32_be;
pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;
pub use crate::src::mechglue::g_initialize::gssint_get_public_oid;
pub use crate::src::mechglue::g_initialize::gssint_select_mech_type;
pub use crate::src::mechglue::g_rel_cred::gss_release_cred;

/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/gssapi/mechglue/g_imp_cred.c - gss_import_cred definition */
/*
 * Copyright (C) 2012 by the Massachusetts Institute of Technology.
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

unsafe extern "C" fn val_imp_cred_args(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut token: crate::gssapi_h::gss_buffer_t,
    mut cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
) -> crate::gssapi_h::OM_uint32 {
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !cred_handle.is_null() {
        *cred_handle = 0 as crate::gssapi_h::gss_cred_id_t
    }
    /* Validate arguments. */
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    if cred_handle.is_null() {
        return (2u32) << 24i32;
    }
    if token.is_null() {
        return (1u32) << 24i32 | (9u32) << 16i32;
    }
    if token.is_null() || (*token).value.is_null() || (*token).length == 0usize {
        return (9u32) << 16i32;
    }
    return 0u32;
}
/* Populate mech_oid and mech_token with the next entry in token, using aliased
 * memory from token.  Advance token by the amount consumed. */

unsafe extern "C" fn get_entry(
    mut _minor_status: *mut crate::gssapi_h::OM_uint32,
    mut token: crate::gssapi_h::gss_buffer_t,
    mut mech_oid: crate::gssapi_h::gss_OID,
    mut mech_token: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut len: crate::gssapi_h::OM_uint32 = 0;
    /* Get the mechanism OID. */
    if (*token).length < 4usize {
        return (9u32) << 16i32;
    }
    len = load_32_be((*token).value);
    if (*token).length.wrapping_sub(4usize) < len as usize {
        return (9u32) << 16i32;
    }
    (*mech_oid).length = len;
    (*mech_oid).elements = ((*token).value as *mut i8).offset(4isize) as *mut libc::c_void;
    (*token).value = ((*token).value as *mut i8)
        .offset(4isize)
        .offset(len as isize) as *mut libc::c_void;
    (*token).length = ((*token).length).wrapping_sub((4u32).wrapping_add(len) as usize);
    /* Get the mechanism token. */
    if (*token).length < 4usize {
        return (9u32) << 16i32;
    }
    len = load_32_be((*token).value);
    if (*token).length.wrapping_sub(4usize) < len as usize {
        return (9u32) << 16i32;
    }
    (*mech_token).length = len as crate::stddef_h::size_t;
    (*mech_token).value = ((*token).value as *mut i8).offset(4isize) as *mut libc::c_void;
    (*token).value = ((*token).value as *mut i8)
        .offset(4isize)
        .offset(len as isize) as *mut libc::c_void;
    (*token).length = ((*token).length).wrapping_sub((4u32).wrapping_add(len) as usize);
    return 0u32;
}
#[no_mangle]

pub unsafe extern "C" fn gss_import_cred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut token: crate::gssapi_h::gss_buffer_t,
    mut cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut count: crate::gssapi_h::OM_uint32 = 0;
    let mut cred = 0 as crate::mglueP_h::gss_union_cred_t;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    let mut tok = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    let mut mech_token = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    let mut mech_oid = crate::gssapi_h::gss_OID_desc {
        length: 0,
        elements: 0 as *mut libc::c_void,
    };
    let mut selected_mech = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut mech_cred = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    let mut elemcopy = 0 as *mut libc::c_void;
    status = val_imp_cred_args(minor_status, token, cred_handle);
    if status != 0u32 {
        return status;
    }
    /* Count the entries in token. */
    tok = *token;
    count = 0u32;
    while tok.length > 0usize {
        status = get_entry(minor_status, &mut tok, &mut mech_oid, &mut mech_token);
        if status != 0u32 {
            return status;
        }
        count = count.wrapping_add(1)
    }
    /* Allocate a union credential. */
    cred = crate::stdlib::calloc(
        1usize,
        ::std::mem::size_of::<crate::mglueP_h::gss_cred_id_struct>(),
    ) as crate::mglueP_h::gss_union_cred_t;
    if cred.is_null() {
        current_block = 15478314596181924773;
    } else {
        (*cred).loopback = cred;
        (*cred).count = 0i32;
        (*cred).mechs_array = crate::stdlib::calloc(
            count as usize,
            ::std::mem::size_of::<crate::gssapi_h::gss_OID_desc_struct>(),
        ) as crate::gssapi_h::gss_OID;
        if (*cred).mechs_array.is_null() {
            current_block = 15478314596181924773;
        } else {
            (*cred).cred_array = crate::stdlib::calloc(
                count as usize,
                ::std::mem::size_of::<crate::gssapi_h::gss_cred_id_t>(),
            ) as *mut crate::gssapi_h::gss_cred_id_t;
            if (*cred).cred_array.is_null() {
                current_block = 15478314596181924773;
            } else {
                tok = *token;
                loop {
                    if !(tok.length > 0usize) {
                        current_block = 721385680381463314;
                        break;
                    }
                    get_entry(minor_status, &mut tok, &mut mech_oid, &mut mech_token);
                    /* Import this entry's mechanism token. */
                    status = crate::src::mechglue::g_initialize::gssint_select_mech_type(
                        minor_status,
                        &mut mech_oid as *mut crate::gssapi_h::gss_OID_desc
                            as crate::gssapi_h::gss_const_OID,
                        &mut selected_mech,
                    );
                    if status != 0u32 {
                        current_block = 14614737902163209062;
                        break;
                    }
                    mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
                        selected_mech as crate::gssapi_h::gss_const_OID,
                    );
                    if mech.is_null()
                        || (*mech).gss_import_cred.is_none()
                            && (*mech).gssspi_import_cred_by_mech.is_none()
                    {
                        status = (9u32) << 16i32;
                        current_block = 14614737902163209062;
                        break;
                    } else {
                        if (*mech).gssspi_import_cred_by_mech.is_some() {
                            status = (*mech)
                                .gssspi_import_cred_by_mech
                                .expect("non-null function pointer")(
                                minor_status,
                                crate::src::mechglue::g_initialize::gssint_get_public_oid(
                                    selected_mech as crate::gssapi_h::gss_const_OID,
                                ),
                                &mut mech_token,
                                &mut mech_cred,
                            )
                        } else {
                            status = (*mech).gss_import_cred.expect("non-null function pointer")(
                                minor_status,
                                &mut mech_token,
                                &mut mech_cred,
                            )
                        }
                        if status != 0u32 {
                            *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                                *minor_status,
                                &mut (*mech).mech_type,
                            );
                            current_block = 14614737902163209062;
                            break;
                        } else {
                            /* Add the resulting mechanism cred to the union cred. */
                            elemcopy = crate::stdlib::malloc((*selected_mech).length as usize);
                            if elemcopy.is_null() {
                                if (*mech).gss_release_cred.is_some() {
                                    (*mech).gss_release_cred.expect("non-null function pointer")(
                                        &mut tmpmin,
                                        &mut mech_cred,
                                    );
                                }
                                current_block = 15478314596181924773;
                                break;
                            } else {
                                crate::stdlib::memcpy(
                                    elemcopy,
                                    (*selected_mech).elements,
                                    (*selected_mech).length as usize,
                                );
                                (*(*cred).mechs_array.offset((*cred).count as isize)).length =
                                    (*selected_mech).length;
                                let ref mut fresh0 =
                                    (*(*cred).mechs_array.offset((*cred).count as isize)).elements;
                                *fresh0 = elemcopy;
                                let fresh1 = (*cred).count;
                                (*cred).count = (*cred).count + 1;
                                let ref mut fresh2 = *(*cred).cred_array.offset(fresh1 as isize);
                                *fresh2 = mech_cred
                            }
                        }
                    }
                }
                match current_block {
                    15478314596181924773 => {}
                    14614737902163209062 => {}
                    _ => {
                        *cred_handle = cred;
                        return 0u32;
                    }
                }
            }
        }
    }
    match current_block {
        15478314596181924773 => {
            status = (13u32) << 16i32;
            *minor_status = 12u32
        }
        _ => {}
    }
    crate::src::mechglue::g_rel_cred::gss_release_cred(&mut tmpmin, &mut cred);
    return status;
}
