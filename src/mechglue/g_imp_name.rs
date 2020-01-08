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

    use crate::src::mechglue::g_imp_name::byteswap_h::__bswap_32;
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
pub use crate::mglueP_h::gss_union_name_desc;
pub use crate::mglueP_h::gss_union_name_t;
pub use crate::src::generic::gssapi_generic::GSS_C_NT_ANONYMOUS;
pub use crate::src::generic::gssapi_generic::GSS_C_NT_COMPOSITE_EXPORT;
pub use crate::src::generic::gssapi_generic::GSS_C_NT_EXPORT_NAME;
pub use crate::src::mechglue::g_glue::gssint_create_copy_buffer;
pub use crate::src::mechglue::g_glue::gssint_get_der_length;
pub use crate::src::mechglue::g_glue::gssint_release_internal_name;
pub use crate::src::mechglue::g_imp_name::byteswap_h::__bswap_32;
pub use crate::src::mechglue::g_imp_name::k5_platform_h::load_32_be;
pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;

unsafe extern "C" fn val_imp_name_args(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut input_name_buffer: crate::gssapi_h::gss_buffer_t,
    mut input_name_type: crate::gssapi_h::gss_OID,
    mut output_name: *mut crate::gssapi_h::gss_name_t,
) -> crate::gssapi_h::OM_uint32 {
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !output_name.is_null() {
        *output_name = 0 as crate::gssapi_h::gss_name_t
    }
    /* Validate arguments. */
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    if output_name.is_null() {
        return (2u32) << 24i32;
    }
    if input_name_buffer.is_null() {
        return (1u32) << 24i32 | (2u32) << 16i32;
    }
    if input_name_type.is_null()
        || !((*input_name_type).length
            == (*crate::src::generic::gssapi_generic::GSS_C_NT_ANONYMOUS).length
            && crate::stdlib::memcmp(
                (*input_name_type).elements,
                (*crate::src::generic::gssapi_generic::GSS_C_NT_ANONYMOUS).elements,
                (*input_name_type).length as usize,
            ) == 0i32)
    {
        if (*input_name_buffer).length == 0usize {
            return (2u32) << 16i32;
        }
        if (*input_name_buffer).value.is_null() {
            return (1u32) << 24i32 | (2u32) << 16i32;
        }
    }
    return 0u32;
}

static mut emptyNameBuffer: crate::gssapi_h::gss_buffer_desc = crate::gssapi_h::gss_buffer_desc {
    length: 0,
    value: 0 as *mut libc::c_void,
};
#[no_mangle]

pub unsafe extern "C" fn gss_import_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut input_name_buffer: crate::gssapi_h::gss_buffer_t,
    mut input_name_type: crate::gssapi_h::gss_OID,
    mut output_name: *mut crate::gssapi_h::gss_name_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut union_name = 0 as *mut crate::mglueP_h::gss_name_struct;
    let mut tmp: crate::gssapi_h::OM_uint32 = 0;
    let mut major_status = (13u32) << 16i32;
    if input_name_buffer.is_null() {
        input_name_buffer = &mut emptyNameBuffer
    }
    major_status = val_imp_name_args(
        minor_status,
        input_name_buffer,
        input_name_type,
        output_name,
    );
    if major_status != 0u32 {
        return major_status;
    }
    /*
     * First create the union name struct that will hold the external
     * name and the name type.
     */
    union_name = crate::stdlib::malloc(::std::mem::size_of::<crate::mglueP_h::gss_union_name_desc>())
        as crate::mglueP_h::gss_union_name_t;
    if union_name.is_null() {
        return (13u32) << 16i32;
    }
    (*union_name).loopback = 0 as *mut crate::mglueP_h::gss_name_struct;
    (*union_name).mech_type = 0 as crate::gssapi_h::gss_OID;
    (*union_name).mech_name = 0 as crate::gssapi_h::gss_name_t;
    (*union_name).name_type = 0 as crate::gssapi_h::gss_OID;
    (*union_name).external_name = 0 as crate::gssapi_h::gss_buffer_t;
    /*
     * All we do here is record the external name and name_type.
     * When the name is actually used, the underlying gss_import_name()
     * is called for the appropriate mechanism.  The exception to this
     * rule is when the name of GSS_C_NT_EXPORT_NAME type.  If that is
     * the case, then we make it MN in this call.
     */
    major_status = crate::src::mechglue::g_glue::gssint_create_copy_buffer(
        input_name_buffer,
        &mut (*union_name).external_name,
        0i32,
    );
    if major_status != 0u32 {
        crate::stdlib::free(union_name as *mut libc::c_void);
        return major_status;
    }
    if !input_name_type.is_null() {
        major_status = crate::src::generic::oid_ops::generic_gss_copy_oid(
            minor_status,
            input_name_type as *const crate::gssapi_h::gss_OID_desc,
            &mut (*union_name).name_type,
        );
        if major_status != 0u32 {
            *minor_status =
                crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status);
            current_block = 13352846991222757387;
        } else {
            current_block = 11584701595673473500;
        }
    } else {
        current_block = 11584701595673473500;
    }
    match current_block {
        11584701595673473500 =>
        /*
         * In MIT Distribution the mechanism is determined from the nametype;
         * This is not a good idea - first mechanism that supports a given
         * name type is picked up; later on the caller can request a
         * different mechanism. So we don't determine the mechanism here. Now
         * the user level and kernel level import_name routine looks similar
         * except the kernel routine makes a copy of the nametype structure. We
         * do however make this an MN for names of GSS_C_NT_EXPORT_NAME type.
         */
        {
            if !input_name_type.is_null()
                && ((*input_name_type).length
                    == (*crate::src::generic::gssapi_generic::GSS_C_NT_EXPORT_NAME).length
                    && crate::stdlib::memcmp(
                        (*input_name_type).elements,
                        (*crate::src::generic::gssapi_generic::GSS_C_NT_EXPORT_NAME).elements,
                        (*input_name_type).length as usize,
                    ) == 0i32
                    || (*input_name_type).length
                        == (*crate::src::generic::gssapi_generic::GSS_C_NT_COMPOSITE_EXPORT).length
                        && crate::stdlib::memcmp(
                            (*input_name_type).elements,
                            (*crate::src::generic::gssapi_generic::GSS_C_NT_COMPOSITE_EXPORT)
                                .elements,
                            (*input_name_type).length as usize,
                        ) == 0i32)
            {
                major_status = importExportName(minor_status, union_name, input_name_type);
                if major_status != 0u32 {
                    current_block = 13352846991222757387;
                } else {
                    current_block = 10043043949733653460;
                }
            } else {
                current_block = 10043043949733653460;
            }
            match current_block {
                13352846991222757387 => {}
                _ => {
                    (*union_name).loopback = union_name;
                    *output_name = union_name;
                    return 0u32;
                }
            }
        }
        _ => {}
    }
    if !union_name.is_null() {
        if !(*union_name).external_name.is_null() {
            if !(*(*union_name).external_name).value.is_null() {
                crate::stdlib::free((*(*union_name).external_name).value);
            }
            crate::stdlib::free((*union_name).external_name as *mut libc::c_void);
        }
        if !(*union_name).name_type.is_null() {
            crate::src::generic::oid_ops::generic_gss_release_oid(
                &mut tmp,
                &mut (*union_name).name_type,
            );
        }
        if !(*union_name).mech_name.is_null() {
            crate::src::mechglue::g_glue::gssint_release_internal_name(
                minor_status,
                (*union_name).mech_type,
                &mut (*union_name).mech_name,
            );
        }
        if !(*union_name).mech_type.is_null() {
            crate::src::generic::oid_ops::generic_gss_release_oid(
                &mut tmp,
                &mut (*union_name).mech_type,
            );
        }
        crate::stdlib::free(union_name as *mut libc::c_void);
    }
    return major_status;
}
/*
 * GSS export name constants
 */

static mut expNameTokIdLen: u32 = 2u32;

static mut mechOidLenLen: u32 = 2u32;

static mut nameTypeLenLen: u32 = 2u32;
/* #pragma ident	"@(#)g_imp_name.c	1.26	04/02/23 SMI" */
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
 *  glue routine gss_import_name
 *
 */
/* local function to import GSS_C_EXPORT_NAME names */

unsafe extern "C" fn importExportName(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut unionName: crate::mglueP_h::gss_union_name_t,
    mut inputNameType: crate::gssapi_h::gss_OID,
) -> crate::gssapi_h::OM_uint32 {
    let mut mechOid = crate::gssapi_h::gss_OID_desc {
        length: 0,
        elements: 0 as *mut libc::c_void,
    };
    let mut expName = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    let mut buf = 0 as *mut u8;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut mechOidLen: crate::gssapi_h::OM_uint32 = 0;
    let mut nameLen: crate::gssapi_h::OM_uint32 = 0;
    let mut curLength: crate::gssapi_h::OM_uint32 = 0;
    let mut bytes: u32 = 0;
    expName.value = (*(*unionName).external_name).value;
    expName.length = (*(*unionName).external_name).length;
    curLength = expNameTokIdLen.wrapping_add(mechOidLenLen);
    if expName.length < curLength as usize {
        return (9u32) << 16i32;
    }
    buf = expName.value as *mut u8;
    if *buf.offset(0isize) as i32 != 0x4i32 {
        return (9u32) << 16i32;
    }
    if *buf.offset(1isize) as i32 != 0x1i32 && *buf.offset(1isize) as i32 != 0x2i32 {
        /* allow composite names */
        return (9u32) << 16i32;
    }
    buf = buf.offset(expNameTokIdLen as isize);
    /* extract the mechanism oid length */
    let fresh0 = buf;
    buf = buf.offset(1);
    mechOidLen = ((*fresh0 as i32) << 8i32) as crate::gssapi_h::OM_uint32;
    let fresh1 = buf;
    buf = buf.offset(1);
    mechOidLen |= *fresh1 as u32;
    curLength = (curLength).wrapping_add(mechOidLen);
    if expName.length < curLength as usize {
        return (9u32) << 16i32;
    }
    /*
     * The mechOid itself is encoded in DER format, OID Tag (0x06)
     * length and the value of mech_OID
     */
    let fresh2 = buf;
    buf = buf.offset(1);
    if *fresh2 as i32 != 0x6i32 {
        return (9u32) << 16i32;
    }
    /*
     * mechoid Length is encoded twice; once in 2 bytes as
     * explained in RFC2743 (under mechanism independent exported
     * name object format) and once using DER encoding
     *
     * We verify both lengths.
     */
    mechOid.length = crate::src::mechglue::g_glue::gssint_get_der_length(
        &mut buf,
        expName.length.wrapping_sub(curLength as usize) as u32,
        &mut bytes,
    ) as crate::gssapi_h::OM_uint32;
    mechOid.elements = buf as *mut libc::c_void;
    /*
     * 'bytes' is the length of the DER length, '1' is for the DER
     * tag for OID
     */
    if bytes.wrapping_add(mechOid.length).wrapping_add(1u32) != mechOidLen {
        return (9u32) << 16i32;
    }
    buf = buf.offset(mechOid.length as isize);
    mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
        &mut mechOid as *mut crate::gssapi_h::gss_OID_desc as crate::gssapi_h::gss_const_OID,
    );
    if mech.is_null() {
        return (1u32) << 16i32;
    }
    if (*mech).gssspi_import_name_by_mech.is_none() && (*mech).gss_import_name.is_none() {
        return (16u32) << 16i32;
    }
    /*
     * we must now determine if we should unwrap the name ourselves
     * or make the mechanism do it - we should only unwrap it
     * if we create it; so if mech->gss_export_name == NULL, we must
     * have created it.
     */
    if (*mech).gss_export_name.is_some() {
        if (*mech).gssspi_import_name_by_mech.is_some() {
            major = (*mech)
                .gssspi_import_name_by_mech
                .expect("non-null function pointer")(
                minor,
                &mut mechOid,
                &mut expName,
                inputNameType,
                &mut (*unionName).mech_name,
            )
        } else {
            major = (*mech).gss_import_name.expect("non-null function pointer")(
                minor,
                &mut expName,
                inputNameType,
                &mut (*unionName).mech_name,
            )
        }
        if major != 0u32 {
            *minor = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                *minor,
                &mut (*mech).mech_type,
            )
        } else {
            major = crate::src::generic::oid_ops::generic_gss_copy_oid(
                minor,
                &mut mechOid,
                &mut (*unionName).mech_type,
            );
            if major != 0u32 {
                *minor = crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor)
            }
        }
        return major;
    }
    /*
     * we must have exported the name - so we now need to reconstruct it
     * and call the mechanism to create it
     *
     * WARNING:	Older versions of gssint_export_internal_name() did
     *		not export names correctly, but now it does.  In
     *		order to stay compatible with existing exported
     *		names we must support names exported the broken
     *		way.
     *
     * Specifically, gssint_export_internal_name() used to include
     * the name type OID in the encoding of the exported MN.
     * Additionally, the Kerberos V mech used to make display names
     * that included a null terminator which was counted in the
     * display name gss_buffer_desc.
     */
    curLength = (curLength).wrapping_add(4u32); /* 4 bytes for name len */
    if expName.length < curLength as usize {
        return (9u32) << 16i32;
    }
    /* next 4 bytes in the name are the name length */
    nameLen = load_32_be(buf as *const libc::c_void);
    buf = buf.offset(4isize);
    /*
     * we use < here because bad code in rpcsec_gss rounds up exported
     * name token lengths and pads with nulls, otherwise != would be
     * appropriate
     */
    curLength = (curLength).wrapping_add(nameLen); /* this is the total length */
    if expName.length < curLength as usize {
        return (9u32) << 16i32;
    }
    /*
     * We detect broken exported names here: they always start with
     * a two-octet network-byte order OID length, which is always
     * less than 256 bytes, so the first octet of the length is
     * always '\0', which is not allowed in GSS-API display names
     * (or never occurs in them anyways).  Of course, the OID
     * shouldn't be there, but it is.  After the OID (sans DER tag
     * and length) there's the name itself, though null-terminated;
     * this null terminator should also not be there, but it is.
     */
    if nameLen > 0u32 && *buf as i32 == '\u{0}' as i32 {
        let mut nameTypeLen: crate::gssapi_h::OM_uint32 = 0;
        /* next two bytes are the name oid */
        if nameLen < nameTypeLenLen {
            return (9u32) << 16i32;
        }
        nameLen = (nameLen).wrapping_sub(nameTypeLenLen);
        let fresh3 = buf;
        buf = buf.offset(1);
        nameTypeLen = ((*fresh3 as i32) << 8i32) as crate::gssapi_h::OM_uint32;
        let fresh4 = buf;
        buf = buf.offset(1);
        nameTypeLen |= *fresh4 as u32;
        if nameLen < nameTypeLen {
            return (9u32) << 16i32;
        }
        buf = buf.offset(nameTypeLen as isize);
        nameLen = (nameLen).wrapping_sub(nameTypeLen);
        /*
         * adjust for expected null terminator that should
         * really not be there
         */
        if nameLen > 0u32
            && *buf.offset(nameLen as isize).offset(-(1isize)) as i32 == '\u{0}' as i32
        {
            nameLen = nameLen.wrapping_sub(1)
        }
    }
    /*
     * Can a name be null?  Let the mech decide.
     *
     * NOTE: We use GSS_C_NULL_OID as the name type when importing
     *	 the unwrapped name.  Presumably the exported name had,
     *	 prior to being exported been obtained in such a way
     *	 that it has been properly perpared ("canonicalized," in
     *	 GSS-API terms) accroding to some name type; we cannot
     *	 tell what that name type was now, but the name should
     *	 need no further preparation other than the lowest
     *	 common denominator afforded by the mech to names
     *	 imported with GSS_C_NULL_OID.  For the Kerberos V mech
     *	 this means doing less busywork too (particularly once
     *	 IDN is thrown in with Kerberos V extensions).
     */
    expName.length = nameLen as crate::stddef_h::size_t;
    expName.value = if nameLen != 0 {
        buf as *mut libc::c_void
    } else {
        0 as *mut libc::c_void
    };
    if (*mech).gssspi_import_name_by_mech.is_some() {
        major = (*mech)
            .gssspi_import_name_by_mech
            .expect("non-null function pointer")(
            minor,
            &mut mechOid,
            &mut expName,
            0 as crate::gssapi_h::gss_OID,
            &mut (*unionName).mech_name,
        )
    } else {
        major = (*mech).gss_import_name.expect("non-null function pointer")(
            minor,
            &mut expName,
            0 as crate::gssapi_h::gss_OID,
            &mut (*unionName).mech_name,
        )
    }
    if major != 0u32 {
        *minor =
            crate::src::generic::util_errmap::gssint_mecherrmap_map(*minor, &mut (*mech).mech_type);
        return major;
    }
    major = crate::src::generic::oid_ops::generic_gss_copy_oid(
        minor,
        &mut mechOid,
        &mut (*unionName).mech_type,
    );
    if major != 0u32 {
        *minor = crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor)
    }
    return major;
}
/* importExportName */
