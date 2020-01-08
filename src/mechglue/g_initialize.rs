use ::libc;

pub mod k5_thread_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
    /* include/k5-thread.h - Preliminary portable thread support */
    /*
     * Copyright 2004,2005,2006,2007,2008 by the Massachusetts Institute of Technology.
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
    /* Interface (tentative):

    Mutex support:

    // Between these two, we should be able to do pure compile-time
    // and pure run-time initialization.
    //   POSIX:   partial initializer is PTHREAD_MUTEX_INITIALIZER,
    //            finish does nothing
    //   Windows: partial initializer is an invalid handle,
    //            finish does the real initialization work
    k5_mutex_t foo_mutex = K5_MUTEX_PARTIAL_INITIALIZER;
    int k5_mutex_finish_init(k5_mutex_t *);
    // for dynamic allocation
    int k5_mutex_init(k5_mutex_t *);
    // Must work for both kinds of alloc, even if it means adding flags.
    int k5_mutex_destroy(k5_mutex_t *);

    // As before.
    int k5_mutex_lock(k5_mutex_t *);
    int k5_mutex_unlock(k5_mutex_t *);

    In each library, one new function to finish the static mutex init,
    and any other library-wide initialization that might be desired.
    On POSIX, this function would be called via the second support
    function (see below).  On Windows, it would be called at library
    load time.  These functions, or functions they calls, should be the
    only places that k5_mutex_finish_init gets called.

    A second function or macro called at various possible "first" entry
    points which either calls pthread_once on the first function
    (POSIX), or checks some flag set by the first function (Windows),
    and possibly returns an error.  (In the non-threaded case, a simple
    flag can be used to avoid multiple invocations, and the mutexes
    don't need run-time initialization anyways.)

    A third function for library termination calls mutex_destroy on
    each mutex for the library.  This function would be called
    automatically at library unload time.  If it turns out to be needed
    at exit time for libraries that don't get unloaded, perhaps we
    should also use atexit().  Any static mutexes should be cleaned up
    with k5_mutex_destroy here.

    How does that second support function invoke the first support
    function only once?  Through something modelled on pthread_once
    that I haven't written up yet.  Probably:

    k5_once_t foo_once = K5_ONCE_INIT;
    k5_once(k5_once_t *, void (*)(void));

    For POSIX: Map onto pthread_once facility.
    For non-threaded case: A simple flag.
    For Windows: Not needed; library init code takes care of it.

    XXX: A general k5_once mechanism isn't possible for Windows,
    without faking it through named mutexes or mutexes initialized at
    startup.  I was only using it in one place outside these headers,
    so I'm dropping the general scheme.  Eventually the existing uses
    in k5-thread.h and k5-platform.h will be converted to pthread_once
    or static variables.


    Thread-specific data:

    // TSD keys are limited in number in gssapi/krb5/com_err; enumerate
    // them all.  This allows support code init to allocate the
    // necessary storage for pointers all at once, and avoids any
    // possible error in key creation.
    enum { ... } k5_key_t;
    // Register destructor function.  Called in library init code.
    int k5_key_register(k5_key_t, void (*destructor)(void *));
    // Returns NULL or data.
    void *k5_getspecific(k5_key_t);
    // Returns error if key out of bounds, or the pointer table can't
    // be allocated.  A call to k5_key_register must have happened first.
    // This may trigger the calling of pthread_setspecific on POSIX.
    int k5_setspecific(k5_key_t, void *);
    // Called in library termination code.
    // Trashes data in all threads, calling the registered destructor
    // (but calling it from the current thread).
    int k5_key_delete(k5_key_t);

    For the non-threaded version, the support code will have a static
    array indexed by k5_key_t values, and get/setspecific simply access
    the array elements.

    The TSD destructor table is global state, protected by a mutex if
    threads are enabled.


    Any actual external symbols will use the krb5int_ prefix.  The k5_
    names will be simple macros or inline functions to rename the
    external symbols, or slightly more complex ones to expand the
    implementation inline (e.g., map to POSIX versions and/or debug
    code using __FILE__ and the like).


    More to be added, perhaps.  */

    #[inline]

    pub unsafe extern "C" fn k5_mutex_unlock(mut m: *mut crate::k5_thread_h::k5_mutex_t) {
        let mut r = crate::k5_thread_h::k5_os_mutex_unlock(m);
        if r != 0i32 {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"k5_mutex_unlock: Received error %d (%s)\n\x00" as *const u8 as *const i8,
                r,
                crate::stdlib::strerror(r),
            );
        }
        if r == 0i32 {
        } else {
            crate::stdlib::__assert_fail(
                b"r == 0\x00" as *const u8 as *const i8,
                b"../../../include/k5-thread.h\x00" as *const u8 as *const i8,
                388u32,
                (*::std::mem::transmute::<&[u8; 35], &[i8; 35]>(
                    b"void k5_mutex_unlock(k5_mutex_t *)\x00",
                ))
                .as_ptr(),
            );
        };
    }

    #[inline]
    pub unsafe extern "C" fn k5_mutex_lock(mut m: *mut crate::k5_thread_h::k5_mutex_t) {
        let mut r = crate::k5_thread_h::k5_os_mutex_lock(m);
        if r != 0i32 {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"k5_mutex_lock: Received error %d (%s)\n\x00" as *const u8 as *const i8,
                r,
                crate::stdlib::strerror(r),
            );
        }
        if r == 0i32 {
        } else {
            crate::stdlib::__assert_fail(
                b"r == 0\x00" as *const u8 as *const i8,
                b"../../../include/k5-thread.h\x00" as *const u8 as *const i8,
                376u32,
                (*::std::mem::transmute::<&[u8; 33], &[i8; 33]>(
                    b"void k5_mutex_lock(k5_mutex_t *)\x00",
                ))
                .as_ptr(),
            );
        };
    }

    #[inline]
    pub unsafe extern "C" fn k5_mutex_finish_init(
        mut _m: *mut crate::k5_thread_h::k5_mutex_t,
    ) -> i32 {
        return 0i32;
    }

    /* multiple inclusion? */
    /* In time, many of the definitions above should move into the support
    library, and this file should be greatly simplified.  For type
    definitions, that'll take some work, since other data structures
    incorporate mutexes directly, and our mutex type is dependent on
    configuration options and system attributes.  For most functions,
    though, it should be relatively easy.

    For now, plugins should use the exported functions, and not the
    above macros, and use krb5int_mutex_alloc for allocations.  */
}

pub mod gssapi_alloc_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
    /* To the extent possible under law, Painless Security, LLC has waived
     * all copyright and related or neighboring rights to GSS-API Memory
     * Management Header. This work is published from: United States.
     */
    /* not _WIN32 or DEBUG_GSSALLOC */
    /* Normal Unix case, just use free/malloc/calloc/realloc. */
    #[inline]

    pub unsafe extern "C" fn gssalloc_free(mut value: *mut libc::c_void) {
        crate::stdlib::free(value);
    }
}

pub mod k5_int_h {
    #[inline]

    pub unsafe extern "C" fn zapfree(mut ptr: *mut libc::c_void, mut len: crate::stddef_h::size_t) {
        if !ptr.is_null() {
            crate::stdlib::explicit_bzero(ptr, len);
            crate::stdlib::free(ptr);
        };
    }

    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}

/* _GSSAPIP_SPNEGO_H_ */
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__blkcnt_t;
pub use crate::stdlib::__blksize_t;
pub use crate::stdlib::__dev_t;
pub use crate::stdlib::__gid_t;
pub use crate::stdlib::__ino_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__mode_t;
pub use crate::stdlib::__nlink_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__ssize_t;
pub use crate::stdlib::__syscall_slong_t;
pub use crate::stdlib::__time_t;
pub use crate::stdlib::__uid_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::ssize_t;
pub use crate::stdlib::time_t;
pub use crate::stdlib::timespec;
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
pub use crate::gssapi_h::gss_OID_set_desc;
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
pub use crate::mglueP_h::gss_mech_config;
pub use crate::mglueP_h::gss_mech_info;
pub use crate::mglueP_h::gss_mechanism;
pub use crate::mglueP_h::gss_name_struct;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_DEPRECATED;
pub use crate::src::mechglue::g_accept_sec_context::gss_accept_sec_context;
pub use crate::src::mechglue::g_acquire_cred::gss_acquire_cred;
pub use crate::src::mechglue::g_acquire_cred::gss_acquire_cred_from;
pub use crate::src::mechglue::g_acquire_cred::gss_add_cred;
pub use crate::src::mechglue::g_acquire_cred_imp_name::gss_acquire_cred_impersonate_name;
pub use crate::src::mechglue::g_acquire_cred_imp_name::gss_add_cred_impersonate_name;
pub use crate::src::mechglue::g_compare_name::gss_compare_name;
pub use crate::src::mechglue::g_complete_auth_token::gss_complete_auth_token;
pub use crate::src::mechglue::g_context_time::gss_context_time;
pub use crate::src::mechglue::g_del_name_attr::gss_delete_name_attribute;
pub use crate::src::mechglue::g_delete_sec_context::gss_delete_sec_context;
pub use crate::src::mechglue::g_dsp_name::gss_display_name;
pub use crate::src::mechglue::g_dsp_name_ext::gss_display_name_ext;
pub use crate::src::mechglue::g_dsp_status::gss_display_status;
pub use crate::src::mechglue::g_dup_name::gss_duplicate_name;
pub use crate::src::mechglue::g_exp_sec_context::gss_export_sec_context;
pub use crate::src::mechglue::g_export_cred::gss_export_cred;
pub use crate::src::mechglue::g_export_name::gss_export_name;
pub use crate::src::mechglue::g_export_name_comp::gss_export_name_composite;
pub use crate::src::mechglue::g_get_name_attr::gss_get_name_attribute;
pub use crate::src::mechglue::g_imp_cred::gss_import_cred;
pub use crate::src::mechglue::g_imp_name::gss_import_name;
pub use crate::src::mechglue::g_imp_sec_context::gss_import_sec_context;
pub use crate::src::mechglue::g_init_sec_context::gss_init_sec_context;
pub use crate::src::mechglue::g_inq_context::gss_inquire_context;
pub use crate::src::mechglue::g_inq_context_oid::gss_inquire_sec_context_by_oid;
pub use crate::src::mechglue::g_inq_cred::gss_inquire_cred;
pub use crate::src::mechglue::g_inq_cred::gss_inquire_cred_by_mech;
pub use crate::src::mechglue::g_inq_cred_oid::gss_inquire_cred_by_oid;
pub use crate::src::mechglue::g_inq_name::gss_inquire_name;
pub use crate::src::mechglue::g_inq_names::gss_inquire_names_for_mech;
pub use crate::src::mechglue::g_map_name_to_any::gss_map_name_to_any;
pub use crate::src::mechglue::g_mech_invoke::gssspi_mech_invoke;
pub use crate::src::mechglue::g_mechattr::gss_inquire_attrs_for_mech;
pub use crate::src::mechglue::g_negoex::gssspi_exchange_meta_data;
pub use crate::src::mechglue::g_negoex::gssspi_query_mechanism_info;
pub use crate::src::mechglue::g_negoex::gssspi_query_meta_data;
pub use crate::src::mechglue::g_oid_ops::gss_test_oid_set_member;
pub use crate::src::mechglue::g_prf::gss_pseudo_random;
pub use crate::src::mechglue::g_process_context::gss_process_context_token;
pub use crate::src::mechglue::g_rel_cred::gss_release_cred;
pub use crate::src::mechglue::g_rel_name::gss_release_name;
pub use crate::src::mechglue::g_rel_name_mapping::gss_release_any_name_mapping;
pub use crate::src::mechglue::g_rel_oid_set::gss_release_oid_set;
pub use crate::src::mechglue::g_saslname::gss_inquire_mech_for_saslname;
pub use crate::src::mechglue::g_saslname::gss_inquire_saslname_for_mech;
pub use crate::src::mechglue::g_seal::gss_wrap;
pub use crate::src::mechglue::g_seal::gss_wrap_size_limit;
pub use crate::src::mechglue::g_set_context_option::gss_set_sec_context_option;
pub use crate::src::mechglue::g_set_name_attr::gss_set_name_attribute;
pub use crate::src::mechglue::g_set_neg_mechs::gss_set_neg_mechs;
pub use crate::src::mechglue::g_sign::gss_get_mic;
pub use crate::src::mechglue::g_store_cred::gss_store_cred;
pub use crate::src::mechglue::g_store_cred::gss_store_cred_into;
pub use crate::src::mechglue::g_unseal::gss_unwrap;
pub use crate::src::mechglue::g_unwrap_aead::gss_unwrap_aead;
pub use crate::src::mechglue::g_unwrap_iov::gss_unwrap_iov;
pub use crate::src::mechglue::g_verify::gss_verify_mic;
pub use crate::src::mechglue::g_wrap_aead::gss_wrap_aead;
pub use crate::src::mechglue::g_wrap_iov::gss_wrap_iov;
pub use crate::src::mechglue::g_wrap_iov::gss_wrap_iov_length;
pub use crate::src::mechglue::gssd_pname_to_uid::gss_localname;
pub use crate::stdlib::__pthread_internal_list;
pub use crate::stdlib::__pthread_list_t;
pub use crate::stdlib::__pthread_mutex_s;
pub use crate::stdlib::pthread_mutex_t;
pub use crate::stdlib::pthread_once_t;
pub use crate::stdlib::FILE;

pub use crate::k5_err_h::errinfo;
pub use crate::k5_err_h::k5_clear_error;
pub use crate::k5_platform_h::k5_init_t;

pub use crate::k5_thread_h::k5_mutex_t;
pub use crate::k5_thread_h::k5_once;
pub use crate::k5_thread_h::k5_once_t;
pub use crate::k5_thread_h::k5_os_mutex;
pub use crate::k5_thread_h::k5_os_mutex_lock;
pub use crate::k5_thread_h::k5_os_mutex_unlock;
pub use crate::k5_thread_h::k5_os_nothread_once_t;
pub use crate::krb5_h::_krb5_data;
pub use crate::krb5_h::krb5_data;
pub use crate::krb5_h::krb5_error_code;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_magic;
pub use crate::src::mechglue::g_initialize::k5_thread_h::k5_mutex_finish_init;
pub use crate::src::mechglue::g_initialize::k5_thread_h::k5_mutex_lock;
pub use crate::src::mechglue::g_initialize::k5_thread_h::k5_mutex_unlock;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::_ISalpha;
pub use crate::stdlib::_ISblank;
pub use crate::stdlib::_ISdigit;
pub use crate::stdlib::_ISgraph;
pub use crate::stdlib::_ISlower;
pub use crate::stdlib::_ISprint;
pub use crate::stdlib::_ISupper;
pub use crate::stdlib::_ISxdigit;
pub use crate::stdlib::__ctype_b_loc;
pub use crate::stdlib::__size_t;
pub use crate::stdlib::dirent;
pub use crate::stdlib::glob;
pub use crate::stdlib::glob_t;
pub use crate::stdlib::globfree;
pub use crate::stdlib::stat;
pub use crate::stdlib::C2RustUnnamed_7;
pub use crate::stdlib::_ISalnum;
pub use crate::stdlib::_IScntrl;
pub use crate::stdlib::_ISpunct;
pub use crate::stdlib::_ISspace;
pub use crate::stdlib::_IO_FILE;

pub use crate::com_err_h::add_error_table;
pub use crate::com_err_h::errcode_t;
pub use crate::com_err_h::error_table;
pub use crate::src::mechglue::g_initialize::gssapi_alloc_h::gssalloc_free;

pub use crate::src::mechglue::g_initialize::k5_int_h::zapfree;

/*
 * list of mechanism libraries and their entry points.
 * the list also maintains state of the mech libraries (loaded or not).
 */

static mut g_mechList: crate::mglueP_h::gss_mech_info = 0 as crate::mglueP_h::gss_mech_info;

static mut g_mechListTail: crate::mglueP_h::gss_mech_info = 0 as crate::mglueP_h::gss_mech_info;

static mut g_mechListLock: crate::k5_thread_h::k5_mutex_t = crate::stdlib::pthread_mutex_t {
    __data: {
        let mut init = crate::stdlib::__pthread_mutex_s {
            __lock: 0i32,
            __count: 0u32,
            __owner: 0i32,
            __nusers: 0u32,
            __kind: 0i32,
            __spins: 0i16,
            __elision: 0i16,
            __list: {
                let mut init = crate::stdlib::__pthread_internal_list {
                    __prev: 0 as *mut crate::stdlib::__pthread_internal_list,
                    __next: 0 as *mut crate::stdlib::__pthread_internal_list,
                };
                init
            },
        };
        init
    },
};

static mut g_confFileModTime: crate::stdlib::time_t = 0isize;

static mut g_confLastCall: crate::stdlib::time_t = 0isize;

static mut g_mechSet: crate::gssapi_h::gss_OID_set_desc = {
    let mut init = crate::gssapi_h::gss_OID_set_desc_struct {
        count: 0usize,
        elements: 0 as crate::gssapi_h::gss_OID,
    };
    init
};

static mut g_mechSetLock: crate::k5_thread_h::k5_mutex_t = crate::stdlib::pthread_mutex_t {
    __data: {
        let mut init = crate::stdlib::__pthread_mutex_s {
            __lock: 0i32,
            __count: 0u32,
            __owner: 0i32,
            __nusers: 0u32,
            __kind: 0i32,
            __spins: 0i16,
            __elision: 0i16,
            __list: {
                let mut init = crate::stdlib::__pthread_internal_list {
                    __prev: 0 as *mut crate::stdlib::__pthread_internal_list,
                    __next: 0 as *mut crate::stdlib::__pthread_internal_list,
                };
                init
            },
        };
        init
    },
};

static mut gssint_mechglue_init__once: crate::k5_platform_h::k5_init_t = {
    {
        let mut init = crate::k5_platform_h::k5_init_t {
            once: {
                let mut init = crate::k5_thread_h::k5_once_t { o: 0i32, n: 2u8 };
                init
            },
            error: 0i32,
            did_run: 0i32,
            fn_0: Some(gssint_mechglue_init__aux as unsafe extern "C" fn() -> ()),
        };
        init
    }
};

unsafe extern "C" fn gssint_mechglue_init__aux() {
    gssint_mechglue_init__once.did_run = 1i32;
    gssint_mechglue_init__once.error = gssint_mechglue_init();
}

unsafe extern "C" fn gssint_mechglue_init() -> i32 {
    let mut err: i32 = 0;
    crate::com_err_h::add_error_table(
        &crate::src::generic::gssapi_err_generic::et_ggss_error_table,
    );
    err = k5_mutex_finish_init(&mut g_mechSetLock);
    err = k5_mutex_finish_init(&mut g_mechListLock);
    err = crate::src::krb5::gssapi_krb5::gss_krb5int_lib_init();
    err = crate::src::spnego::spnego_mech::gss_spnegoint_lib_init();
    err = crate::src::generic::util_errmap::gssint_mecherrmap_init();
    return err;
}
#[no_mangle]

pub unsafe extern "C" fn gssint_mechglue_initialize_library() -> i32 {
    return {
        let mut k5int_i: *mut crate::k5_platform_h::k5_init_t = &mut gssint_mechglue_init__once;
        let mut k5int_err = crate::k5_thread_h::k5_once(&mut (*k5int_i).once, (*k5int_i).fn_0);
        if k5int_err != 0 {
            k5int_err
        } else {
            if (*k5int_i).did_run != 0i32 {
            } else {
                crate::stdlib::__assert_fail(
                    b"k5int_i->did_run != 0\x00" as *const u8 as *const i8,
                    b"g_initialize.c\x00" as *const u8 as *const i8,
                    156u32,
                    (*::std::mem::transmute::<&[u8; 45], &[i8; 45]>(
                        b"int gssint_mechglue_initialize_library(void)\x00",
                    ))
                    .as_ptr(),
                );
            }
            (*k5int_i).error
        }
    };
}
/*
 * function used to reclaim the memory used by a gss_OID structure.
 * This routine requires direct access to the mechList.
 */
#[no_mangle]

pub unsafe extern "C" fn gss_release_oid(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut oid: *mut crate::gssapi_h::gss_OID,
) -> crate::gssapi_h::OM_uint32 {
    let mut major: crate::gssapi_h::OM_uint32 = 0; /* while */
    let mut aMech = 0 as *mut crate::mglueP_h::gss_mech_config;
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if minor_status.is_null() || oid.is_null() {
        return (2u32) << 24i32;
    }
    *minor_status = gssint_mechglue_initialize_library() as crate::gssapi_h::OM_uint32;
    if *minor_status != 0u32 {
        return (13u32) << 16i32;
    }
    k5_mutex_lock(&mut g_mechListLock);
    aMech = g_mechList;
    while !aMech.is_null() {
        /*
         * look through the loaded mechanism libraries for
         * gss_internal_release_oid until one returns success.
         * gss_internal_release_oid will only return success when
         * the OID was recognized as an internal mechanism OID. if no
         * mechanisms recognize the OID, then call the generic version.
         */
        if !(*aMech).mech.is_null() && (*(*aMech).mech).gss_internal_release_oid.is_some() {
            major = (*(*aMech).mech)
                .gss_internal_release_oid
                .expect("non-null function pointer")(minor_status, oid);
            if major == 0u32 {
                k5_mutex_unlock(&mut g_mechListLock);
                return 0u32;
            }
            *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                *minor_status,
                &mut (*(*aMech).mech).mech_type,
            )
        }
        aMech = (*aMech).next
    }
    k5_mutex_unlock(&mut g_mechListLock);
    return crate::src::generic::oid_ops::generic_gss_release_oid(minor_status, oid);
}
/* gss_release_oid */
/*
 * Wrapper around inquire_attrs_for_mech to determine whether a mechanism has
 * the deprecated attribute.  Must be called without g_mechSetLock since it
 * will call into the mechglue.
 */

unsafe extern "C" fn is_deprecated(mut element: crate::gssapi_h::gss_OID) -> i32 {
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut minor: crate::gssapi_h::OM_uint32 = 0;
    let mut mech_attrs = 0 as crate::gssapi_h::gss_OID_set;
    let mut deprecated = 0i32;
    major = crate::src::mechglue::g_mechattr::gss_inquire_attrs_for_mech(
        &mut minor,
        element as crate::gssapi_h::gss_const_OID,
        &mut mech_attrs,
        0 as *mut crate::gssapi_h::gss_OID_set,
    );
    if major == 0u32 {
        crate::src::mechglue::g_oid_ops::gss_test_oid_set_member(
            &mut minor,
            crate::src::generic::gssapi_generic::GSS_C_MA_DEPRECATED as crate::gssapi_h::gss_OID,
            mech_attrs,
            &mut deprecated,
        );
    }
    if !mech_attrs.is_null() {
        crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(&mut minor, &mut mech_attrs);
    }
    return deprecated;
}
/*
 * Removes mechs with the deprecated attribute from an OID set.  Must be
 * called without g_mechSetLock held since it calls into the mechglue.
 */

unsafe extern "C" fn prune_deprecated(mut mech_set: crate::gssapi_h::gss_OID_set) {
    let mut i: crate::gssapi_h::OM_uint32 = 0;
    let mut j: crate::gssapi_h::OM_uint32 = 0;
    j = 0u32;
    i = 0u32;
    while (i as usize) < (*mech_set).count {
        if is_deprecated(&mut *(*mech_set).elements.offset(i as isize)) == 0 {
            let fresh0 = j;
            j = j.wrapping_add(1);
            *(*mech_set).elements.offset(fresh0 as isize) = *(*mech_set).elements.offset(i as isize)
        } else {
            gssalloc_free((*(*mech_set).elements.offset(i as isize)).elements);
        }
        i = i.wrapping_add(1)
    }
    (*mech_set).count = j as crate::stddef_h::size_t;
}
/*
 * this function will return an oid set indicating available mechanisms.
 * The set returned is based on configuration file entries and
 * NOT on the loaded mechanisms.  This function does not check if any
 * of these can actually be loaded.
 * Deprecated mechanisms will not be returned.
 * This routine needs direct access to the mechanism list.
 * To avoid reading the configuration file each call, we will save a
 * a mech oid set, and only update it once the file has changed.
 */
#[no_mangle]

pub unsafe extern "C" fn gss_indicate_mechs(
    mut minorStatus: *mut crate::gssapi_h::OM_uint32,
    mut mechSet_out: *mut crate::gssapi_h::gss_OID_set,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    /* Initialize outputs. */
    if !minorStatus.is_null() {
        *minorStatus = 0u32
    }
    if !mechSet_out.is_null() {
        *mechSet_out = 0 as crate::gssapi_h::gss_OID_set
    }
    /* Validate arguments. */
    if minorStatus.is_null() || mechSet_out.is_null() {
        return (2u32) << 24i32;
    }
    *minorStatus = gssint_mechglue_initialize_library() as crate::gssapi_h::OM_uint32;
    if *minorStatus != 0u32 {
        return (13u32) << 16i32;
    }
    if build_mechSet() != 0 {
        return (13u32) << 16i32;
    }
    /*
     * need to lock the g_mechSet in case someone tries to update it while
     * I'm copying it.
     */
    k5_mutex_lock(&mut g_mechSetLock);
    status = crate::src::generic::oid_ops::generic_gss_copy_oid_set(
        minorStatus,
        &mut g_mechSet,
        mechSet_out,
    );
    k5_mutex_unlock(&mut g_mechSetLock);
    if !(*mechSet_out).is_null() {
        prune_deprecated(*mechSet_out);
    }
    return status;
}
/* gss_indicate_mechs */
/* Call with g_mechSetLock held, or during final cleanup.  */

unsafe extern "C" fn free_mechSet() {
    let mut i: u32 = 0;
    if g_mechSet.count != 0usize {
        i = 0u32;
        while (i as usize) < g_mechSet.count {
            crate::stdlib::free((*g_mechSet.elements.offset(i as isize)).elements);
            i = i.wrapping_add(1)
        }
        crate::stdlib::free(g_mechSet.elements as *mut libc::c_void);
        g_mechSet.elements = 0 as crate::gssapi_h::gss_OID;
        g_mechSet.count = 0usize
    };
}

unsafe extern "C" fn build_mechSet() -> crate::gssapi_h::OM_uint32 {
    let mut mList = 0 as *mut crate::mglueP_h::gss_mech_config;
    let mut i: crate::stddef_h::size_t = 0;
    let mut count: crate::stddef_h::size_t = 0;
    let mut curItem = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    /*
     * lock the mutex since we will be updating
     * the mechList structure
     * we need to keep the lock while we build the mechanism list
     * since we are accessing parts of the mechList which could be
     * modified.
     */
    k5_mutex_lock(&mut g_mechListLock);
    updateMechList();
    /*
     * we need to lock the mech set so that no one else will
     * try to read it as we are re-creating it
     */
    k5_mutex_lock(&mut g_mechSetLock);
    /* if the oid list already exists we must free it first */
    free_mechSet();
    /* determine how many elements to have in the list */
    mList = g_mechList;
    count = 0usize;
    while !mList.is_null() {
        count = count.wrapping_add(1);
        mList = (*mList).next
    }
    /* this should always be true, but.... */
    if count > 0usize {
        g_mechSet.elements = crate::stdlib::calloc(
            count,
            ::std::mem::size_of::<crate::gssapi_h::gss_OID_desc>(),
        ) as crate::gssapi_h::gss_OID;
        if g_mechSet.elements.is_null() {
            k5_mutex_unlock(&mut g_mechSetLock);
            k5_mutex_unlock(&mut g_mechListLock);
            return (13u32) << 16i32;
        }
        crate::stdlib::memset(
            g_mechSet.elements as *mut libc::c_void,
            0i32,
            count.wrapping_mul(::std::mem::size_of::<crate::gssapi_h::gss_OID_desc>()),
        );
        /* now copy each oid element */
        count = 0usize;
        mList = g_mechList;
        while !mList.is_null() {
            /* Don't expose interposer mechanisms. */
            if !((*mList).is_interposer != 0) {
                curItem = &mut *g_mechSet.elements.offset(count as isize)
                    as *mut crate::gssapi_h::gss_OID_desc_struct;
                (*curItem).elements = crate::stdlib::malloc((*(*mList).mech_type).length as usize);
                if (*curItem).elements.is_null() {
                    /*
                     * this is nasty - we must delete the
                     * part of the array already copied
                     */
                    i = 0usize;
                    while i < count {
                        crate::stdlib::free((*g_mechSet.elements.offset(i as isize)).elements);
                        i = i.wrapping_add(1)
                    }
                    crate::stdlib::free(g_mechSet.elements as *mut libc::c_void);
                    g_mechSet.count = 0usize;
                    g_mechSet.elements = 0 as crate::gssapi_h::gss_OID;
                    k5_mutex_unlock(&mut g_mechSetLock);
                    k5_mutex_unlock(&mut g_mechListLock);
                    return (13u32) << 16i32;
                }
                crate::stdlib::memcpy(
                    (*curItem).elements,
                    (*(*mList).mech_type).elements,
                    (*(*mList).mech_type).length as usize,
                );
                (*curItem).length = (*(*mList).mech_type).length;
                count = count.wrapping_add(1)
            }
            mList = (*mList).next
        }
        g_mechSet.count = count
    }
    k5_mutex_unlock(&mut g_mechSetLock);
    k5_mutex_unlock(&mut g_mechListLock);
    return 0u32;
}
/*
 * this function has been added for use by modules that need to
 * know what (if any) optional parameters are supplied in the
 * config file (MECH_CONF).
 * It will return the option string for a specified mechanism.
 * caller is responsible for freeing the memory
 */
#[no_mangle]

pub unsafe extern "C" fn gssint_get_modOptions(oid: crate::gssapi_h::gss_OID) -> *mut i8 {
    let mut aMech = 0 as *mut crate::mglueP_h::gss_mech_config;
    let mut modOptions = 0 as *mut i8;
    if gssint_mechglue_initialize_library() != 0i32 {
        return 0 as *mut i8;
    }
    /* make sure we have fresh data */
    k5_mutex_lock(&mut g_mechListLock);
    updateMechList();
    aMech = searchMechList(oid as crate::gssapi_h::gss_const_OID);
    if aMech.is_null() || (*aMech).optionStr.is_null() {
        k5_mutex_unlock(&mut g_mechListLock);
        return 0 as *mut i8;
    }
    if !(*aMech).optionStr.is_null() {
        modOptions = crate::stdlib::strdup((*aMech).optionStr)
    }
    k5_mutex_unlock(&mut g_mechListLock);
    return modOptions;
}
/* gssint_get_modOptions */
/* Return the mtime of filename or its eventual symlink target (if it is a
 * symlink), whichever is larger.  Return (time_t)-1 if lstat or stat fails. */

unsafe extern "C" fn check_link_mtime(
    mut filename: *const i8,
    mut _mtime_out: *mut crate::stdlib::time_t,
) -> crate::stdlib::time_t {
    let mut st1 = crate::stdlib::stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: crate::stdlib::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: crate::stdlib::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: crate::stdlib::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut st2 = crate::stdlib::stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: crate::stdlib::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: crate::stdlib::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: crate::stdlib::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if crate::stdlib::lstat(filename, &mut st1) != 0i32 {
        return -1isize;
    }
    if !(st1.st_mode & 0o170000u32 == 0o120000u32) {
        return st1.st_mtim.tv_sec;
    }
    if crate::stdlib::stat(filename, &mut st2) != 0i32 {
        return -1isize;
    }
    return if st1.st_mtim.tv_sec > st2.st_mtim.tv_sec {
        st1.st_mtim.tv_sec
    } else {
        st2.st_mtim.tv_sec
    };
}
/* Load pathname if it is newer than last.  Update *highest to the maximum of
 * its current value and pathname's mod time. */

unsafe extern "C" fn load_if_changed(
    mut pathname: *const i8,
    mut last: crate::stdlib::time_t,
    mut highest: *mut crate::stdlib::time_t,
) {
    let mut mtime: crate::stdlib::time_t = 0;
    mtime = check_link_mtime(pathname, &mut mtime);
    if mtime == -1isize {
        return;
    }
    if mtime > *highest {
        *highest = mtime
    }
    if mtime > last {
        loadConfigFile(pathname);
    };
}
/* Try to load any config files which have changed since the last call.  Config
 * files are MECH_CONF and any files matching MECH_CONF_PATTERN. */

unsafe extern "C" fn loadConfigFiles() {
    let mut globbuf = crate::stdlib::glob_t {
        gl_pathc: 0,
        gl_pathv: 0 as *mut *mut i8,
        gl_offs: 0,
        gl_flags: 0,
        gl_closedir: None,
        gl_readdir: None,
        gl_opendir: None,
        gl_lstat: None,
        gl_stat: None,
    };
    let mut highest = 0isize;
    let mut now: crate::stdlib::time_t = 0;
    let mut path = 0 as *mut *mut i8;
    let mut val = 0 as *const i8;
    /* Don't glob and stat more than once per second. */
    if crate::stdlib::time(&mut now) == -1isize || now == g_confLastCall {
        return;
    }
    g_confLastCall = now;
    val = crate::stdlib::secure_getenv(b"GSS_MECH_CONFIG\x00" as *const u8 as *const i8);
    if !val.is_null() {
        load_if_changed(val, g_confFileModTime, &mut g_confFileModTime);
        return;
    }
    load_if_changed(
        b"/usr/local/etc/gss/mech\x00" as *const u8 as *const i8,
        g_confFileModTime,
        &mut highest,
    );
    crate::stdlib::memset(
        &mut globbuf as *mut crate::stdlib::glob_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::stdlib::glob_t>(),
    );
    if crate::stdlib::glob(
        b"/usr/local/etc/gss/mech.d/*.conf\x00" as *const u8 as *const i8,
        0i32,
        None,
        &mut globbuf,
    ) == 0i32
    {
        path = globbuf.gl_pathv;
        while !(*path).is_null() {
            load_if_changed(*path, g_confFileModTime, &mut highest);
            path = path.offset(1)
        }
    }
    crate::stdlib::globfree(&mut globbuf);
    g_confFileModTime = highest;
}
/*
 * determines if the mechList needs to be updated from file
 * and performs the update.
 * this functions must be called with a lock of g_mechListLock
 */

unsafe extern "C" fn updateMechList() {
    let mut minfo = 0 as *mut crate::mglueP_h::gss_mech_config;
    /* _WIN32 */
    loadConfigFiles();
    /* !_WIN32 */
    /* Load any unloaded interposer mechanisms immediately, to make sure we
     * interpose other mechanisms before they are used. */
    minfo = g_mechList;
    while !minfo.is_null() {
        if (*minfo).is_interposer != 0 && (*minfo).mech.is_null() {
            loadInterMech(minfo);
        }
        minfo = (*minfo).next
    }
}
/* updateMechList */
/* Update the mech list from system configuration if we have never done so.
 * Must be invoked with the g_mechListLock mutex held. */

unsafe extern "C" fn initMechList() {
    static mut lazy_init: i32 = 0i32;
    if lazy_init == 0i32 {
        updateMechList();
        lazy_init = 1i32
    };
}

unsafe extern "C" fn releaseMechInfo(mut pCf: *mut crate::mglueP_h::gss_mech_info) {
    let mut cf = 0 as *mut crate::mglueP_h::gss_mech_config;
    let mut minor_status: crate::gssapi_h::OM_uint32 = 0;
    if (*pCf).is_null() {
        return;
    }
    cf = *pCf;
    if !(*cf).kmodName.is_null() {
        crate::stdlib::free((*cf).kmodName as *mut libc::c_void);
    }
    if !(*cf).uLibName.is_null() {
        crate::stdlib::free((*cf).uLibName as *mut libc::c_void);
    }
    if !(*cf).mechNameStr.is_null() {
        crate::stdlib::free((*cf).mechNameStr as *mut libc::c_void);
    }
    if !(*cf).optionStr.is_null() {
        crate::stdlib::free((*cf).optionStr as *mut libc::c_void);
    }
    if !(*cf).mech_type.is_null()
        && (*cf).mech_type != &mut (*(*cf).mech).mech_type as *mut crate::gssapi_h::gss_OID_desc
    {
        crate::src::generic::oid_ops::generic_gss_release_oid(
            &mut minor_status,
            &mut (*cf).mech_type,
        );
    }
    if (*cf).freeMech != 0 {
        zapfree(
            (*cf).mech as *mut libc::c_void,
            ::std::mem::size_of::<crate::mglueP_h::gss_config>(),
        );
    }
    if !(*cf).dl_handle.is_null() {
        crate::k5_plugin_h::krb5int_close_plugin(
            (*cf).dl_handle as *mut crate::k5_plugin_h::plugin_file_handle,
        );
    }
    if !(*cf).int_mech_type.is_null() {
        crate::src::generic::oid_ops::generic_gss_release_oid(
            &mut minor_status,
            &mut (*cf).int_mech_type,
        );
    }
    crate::stdlib::memset(
        cf as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::mglueP_h::gss_mech_config>(),
    );
    crate::stdlib::free(cf as *mut libc::c_void);
    *pCf = 0 as crate::mglueP_h::gss_mech_info;
}
/*
 * Register a mechanism.  Called with g_mechListLock held.
 */
#[no_mangle]

pub unsafe extern "C" fn gssint_register_mechinfo(
    mut template: crate::mglueP_h::gss_mech_info,
) -> i32 {
    let mut cf = 0 as *mut crate::mglueP_h::gss_mech_config;
    let mut new_cf = 0 as *mut crate::mglueP_h::gss_mech_config;
    new_cf = crate::stdlib::calloc(
        1usize,
        ::std::mem::size_of::<crate::mglueP_h::gss_mech_config>(),
    ) as crate::mglueP_h::gss_mech_info;
    if new_cf.is_null() {
        return 12i32;
    }
    (*new_cf).dl_handle = (*template).dl_handle;
    /* copy mech so we can rewrite canonical mechanism OID */
    (*new_cf).mech =
        crate::stdlib::calloc(1usize, ::std::mem::size_of::<crate::mglueP_h::gss_config>())
            as crate::mglueP_h::gss_mechanism;
    if (*new_cf).mech.is_null() {
        releaseMechInfo(&mut new_cf);
        return 12i32;
    }
    *(*new_cf).mech = *(*template).mech;
    if !(*template).mech_type.is_null() {
        (*(*new_cf).mech).mech_type = *(*template).mech_type
    }
    (*new_cf).mech_type = &mut (*(*new_cf).mech).mech_type;
    (*new_cf).priority = (*template).priority;
    (*new_cf).freeMech = 1i32;
    (*new_cf).next = 0 as *mut crate::mglueP_h::gss_mech_config;
    if !(*template).kmodName.is_null() {
        (*new_cf).kmodName = crate::stdlib::strdup((*template).kmodName);
        if (*new_cf).kmodName.is_null() {
            releaseMechInfo(&mut new_cf);
            return 12i32;
        }
    }
    if !(*template).uLibName.is_null() {
        (*new_cf).uLibName = crate::stdlib::strdup((*template).uLibName);
        if (*new_cf).uLibName.is_null() {
            releaseMechInfo(&mut new_cf);
            return 12i32;
        }
    }
    if !(*template).mechNameStr.is_null() {
        (*new_cf).mechNameStr = crate::stdlib::strdup((*template).mechNameStr);
        if (*new_cf).mechNameStr.is_null() {
            releaseMechInfo(&mut new_cf);
            return 12i32;
        }
    }
    if !(*template).optionStr.is_null() {
        (*new_cf).optionStr = crate::stdlib::strdup((*template).optionStr);
        if (*new_cf).optionStr.is_null() {
            releaseMechInfo(&mut new_cf);
            return 12i32;
        }
    }
    if g_mechList.is_null() {
        g_mechList = new_cf;
        g_mechListTail = new_cf;
        return 0i32;
    } else {
        if (*new_cf).priority < (*g_mechList).priority {
            (*new_cf).next = g_mechList;
            g_mechList = new_cf;
            return 0i32;
        }
    }
    cf = g_mechList;
    while !cf.is_null() {
        if (*cf).next.is_null() || (*new_cf).priority < (*(*cf).next).priority {
            (*new_cf).next = (*cf).next;
            (*cf).next = new_cf;
            if g_mechListTail == cf {
                g_mechListTail = new_cf
            }
            break;
        } else {
            cf = (*cf).next
        }
    }
    return 0i32;
}
/* _GSS_STATIC_LINK */
/*
 * If _symbol is undefined in the shared object but the shared object
 * is linked against the mechanism glue, it's possible for dlsym() to
 * return the mechanism glue implementation. Guard against that.
 */

unsafe extern "C" fn build_dynamicMech(
    mut dl: *mut libc::c_void,
    mech_type: crate::gssapi_h::gss_OID,
) -> crate::mglueP_h::gss_mechanism {
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    mech = crate::stdlib::calloc(1usize, ::std::mem::size_of::<crate::mglueP_h::gss_config>())
        as crate::mglueP_h::gss_mechanism;
    if mech.is_null() {
        return 0 as crate::mglueP_h::gss_mechanism;
    }
    let mut errinfo = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_acquire_cred\x00" as *const u8 as *const i8,
        &mut (*mech).gss_acquire_cred
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID_set,
                    _: i32,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo,
    ) != 0
        || errinfo.code != 0
    {
        (*mech).gss_acquire_cred = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo);
    }
    if (*mech).gss_acquire_cred
        == Some(
            crate::src::mechglue::g_acquire_cred::gss_acquire_cred
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID_set,
                    _: crate::gssapi_h::gss_cred_usage_t,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_acquire_cred = None
    }
    let mut errinfo_0 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_0 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_release_cred\x00" as *const u8 as *const i8,
        &mut (*mech).gss_release_cred
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_0,
    ) != 0
        || errinfo_0.code != 0
    {
        (*mech).gss_release_cred = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_0);
    }
    if (*mech).gss_release_cred
        == Some(
            crate::src::mechglue::g_rel_cred::gss_release_cred
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_release_cred = None
    }
    let mut errinfo_1 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_1 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_init_sec_context\x00" as *const u8 as *const i8,
        &mut (*mech).gss_init_sec_context
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_channel_bindings_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_1,
    ) != 0
        || errinfo_1.code != 0
    {
        (*mech).gss_init_sec_context = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_1);
    }
    if &(*mech).gss_init_sec_context as *const _ as *const ()
        == &gss_init_sec_context as *const _ as *const ()
    {
        (*mech).gss_init_sec_context = None
    }
    // if (*mech).gss_init_sec_context
    //     == Some(
    //         crate::src::mechglue::g_init_sec_context::gss_init_sec_context
    //             as unsafe extern "C" fn(
    //                 _: *mut crate::gssapi_h::OM_uint32,
    //                 _: crate::gssapi_h::gss_cred_id_t,
    //                 _: *mut crate::gssapi_h::gss_ctx_id_t,
    //                 _: crate::gssapi_h::gss_name_t,
    //                 _: crate::gssapi_h::gss_OID,
    //                 _: crate::gssapi_h::OM_uint32,
    //                 _: crate::gssapi_h::OM_uint32,
    //                 _: crate::gssapi_h::gss_channel_bindings_t,
    //                 _: crate::gssapi_h::gss_buffer_t,
    //                 _: *mut crate::gssapi_h::gss_OID,
    //                 _: crate::gssapi_h::gss_buffer_t,
    //                 _: *mut crate::gssapi_h::OM_uint32,
    //                 _: *mut crate::gssapi_h::OM_uint32,
    //             ) -> crate::gssapi_h::OM_uint32,
    //     )
    // {
    //     (*mech).gss_init_sec_context = None
    // }
    let mut errinfo_2 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_2 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_accept_sec_context\x00" as *const u8 as *const i8,
        &mut (*mech).gss_accept_sec_context
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_channel_bindings_t,
                    _: *mut crate::gssapi_h::gss_name_t,
                    _: *mut crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_2,
    ) != 0
        || errinfo_2.code != 0
    {
        (*mech).gss_accept_sec_context = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_2);
    }
    if (*mech).gss_accept_sec_context
        == Some(
            crate::src::mechglue::g_accept_sec_context::gss_accept_sec_context
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_channel_bindings_t,
                    _: *mut crate::gssapi_h::gss_name_t,
                    _: *mut crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_accept_sec_context = None
    }
    let mut errinfo_3 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_3 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_process_context_token\x00" as *const u8 as *const i8,
        &mut (*mech).gss_process_context_token
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_3,
    ) != 0
        || errinfo_3.code != 0
    {
        (*mech).gss_process_context_token = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_3);
    }
    if (*mech).gss_process_context_token
        == Some(
            crate::src::mechglue::g_process_context::gss_process_context_token
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_process_context_token = None
    }
    let mut errinfo_4 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_4 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_delete_sec_context\x00" as *const u8 as *const i8,
        &mut (*mech).gss_delete_sec_context
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_4,
    ) != 0
        || errinfo_4.code != 0
    {
        (*mech).gss_delete_sec_context = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_4);
    }
    if (*mech).gss_delete_sec_context
        == Some(
            crate::src::mechglue::g_delete_sec_context::gss_delete_sec_context
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_delete_sec_context = None
    }
    let mut errinfo_5 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_5 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_context_time\x00" as *const u8 as *const i8,
        &mut (*mech).gss_context_time
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_5,
    ) != 0
        || errinfo_5.code != 0
    {
        (*mech).gss_context_time = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_5);
    }
    if (*mech).gss_context_time
        == Some(
            crate::src::mechglue::g_context_time::gss_context_time
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_context_time = None
    }
    let mut errinfo_6 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_6 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_get_mic\x00" as *const u8 as *const i8,
        &mut (*mech).gss_get_mic
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_qop_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_6,
    ) != 0
        || errinfo_6.code != 0
    {
        (*mech).gss_get_mic = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_6);
    }
    if (*mech).gss_get_mic
        == Some(
            crate::src::mechglue::g_sign::gss_get_mic
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_qop_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_get_mic = None
    }
    let mut errinfo_7 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_7 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_verify_mic\x00" as *const u8 as *const i8,
        &mut (*mech).gss_verify_mic
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::gss_qop_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_7,
    ) != 0
        || errinfo_7.code != 0
    {
        (*mech).gss_verify_mic = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_7);
    }
    if (*mech).gss_verify_mic
        == Some(
            crate::src::mechglue::g_verify::gss_verify_mic
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::gss_qop_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_verify_mic = None
    }
    let mut errinfo_8 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_8 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_wrap\x00" as *const u8 as *const i8,
        &mut (*mech).gss_wrap
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: i32,
                    _: crate::gssapi_h::gss_qop_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut i32,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_8,
    ) != 0
        || errinfo_8.code != 0
    {
        (*mech).gss_wrap = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_8);
    }
    if (*mech).gss_wrap
        == Some(
            crate::src::mechglue::g_seal::gss_wrap
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: i32,
                    _: crate::gssapi_h::gss_qop_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut i32,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_wrap = None
    }
    let mut errinfo_9 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_9 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_unwrap\x00" as *const u8 as *const i8,
        &mut (*mech).gss_unwrap
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut i32,
                    _: *mut crate::gssapi_h::gss_qop_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_9,
    ) != 0
        || errinfo_9.code != 0
    {
        (*mech).gss_unwrap = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_9);
    }
    if (*mech).gss_unwrap
        == Some(
            crate::src::mechglue::g_unseal::gss_unwrap
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut i32,
                    _: *mut crate::gssapi_h::gss_qop_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_unwrap = None
    }
    let mut errinfo_10 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_10 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_display_status\x00" as *const u8 as *const i8,
        &mut (*mech).gss_display_status
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::OM_uint32,
                    _: i32,
                    _: crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_10,
    ) != 0
        || errinfo_10.code != 0
    {
        (*mech).gss_display_status = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_10);
    }
    if (*mech).gss_display_status
        == Some(
            crate::src::mechglue::g_dsp_status::gss_display_status
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::OM_uint32,
                    _: i32,
                    _: crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_display_status = None
    }
    let mut errinfo_11 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_11 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_indicate_mechs\x00" as *const u8 as *const i8,
        &mut (*mech).gss_indicate_mechs
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_OID_set,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_11,
    ) != 0
        || errinfo_11.code != 0
    {
        (*mech).gss_indicate_mechs = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_11);
    }
    if (*mech).gss_indicate_mechs
        == ::std::mem::transmute::<
            Option<unsafe extern "C" fn() -> crate::gssapi_h::OM_uint32>,
            Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_OID_set,
                ) -> crate::gssapi_h::OM_uint32,
            >,
        >(Some(::std::mem::transmute::<
            unsafe extern "C" fn(
                _: *mut crate::gssapi_h::OM_uint32,
                _: *mut crate::gssapi_h::gss_OID_set,
            ) -> crate::gssapi_h::OM_uint32,
            unsafe extern "C" fn() -> crate::gssapi_h::OM_uint32,
        >(gss_indicate_mechs)))
    {
        (*mech).gss_indicate_mechs = None
    }
    let mut errinfo_12 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_12 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_compare_name\x00" as *const u8 as *const i8,
        &mut (*mech).gss_compare_name
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_name_t,
                    _: *mut i32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_12,
    ) != 0
        || errinfo_12.code != 0
    {
        (*mech).gss_compare_name = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_12);
    }
    if (*mech).gss_compare_name
        == Some(
            crate::src::mechglue::g_compare_name::gss_compare_name
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_name_t,
                    _: *mut i32,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_compare_name = None
    }
    let mut errinfo_13 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_13 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_display_name\x00" as *const u8 as *const i8,
        &mut (*mech).gss_display_name
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::gss_OID,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_13,
    ) != 0
        || errinfo_13.code != 0
    {
        (*mech).gss_display_name = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_13);
    }
    if (*mech).gss_display_name
        == Some(
            crate::src::mechglue::g_dsp_name::gss_display_name
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::gss_OID,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_display_name = None
    }
    let mut errinfo_14 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_14 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_import_name\x00" as *const u8 as *const i8,
        &mut (*mech).gss_import_name
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_h::gss_name_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_14,
    ) != 0
        || errinfo_14.code != 0
    {
        (*mech).gss_import_name = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_14);
    }
    if (*mech).gss_import_name
        == Some(
            crate::src::mechglue::g_imp_name::gss_import_name
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_h::gss_name_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_import_name = None
    }
    let mut errinfo_15 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_15 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_release_name\x00" as *const u8 as *const i8,
        &mut (*mech).gss_release_name
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_name_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_15,
    ) != 0
        || errinfo_15.code != 0
    {
        (*mech).gss_release_name = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_15);
    }
    if (*mech).gss_release_name
        == Some(
            crate::src::mechglue::g_rel_name::gss_release_name
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_name_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_release_name = None
    }
    let mut errinfo_16 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_16 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_inquire_cred\x00" as *const u8 as *const i8,
        &mut (*mech).gss_inquire_cred
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_name_t,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut i32,
                    _: *mut crate::gssapi_h::gss_OID_set,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_16,
    ) != 0
        || errinfo_16.code != 0
    {
        (*mech).gss_inquire_cred = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_16);
    }
    if (*mech).gss_inquire_cred
        == Some(
            crate::src::mechglue::g_inq_cred::gss_inquire_cred
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_name_t,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_cred_usage_t,
                    _: *mut crate::gssapi_h::gss_OID_set,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_inquire_cred = None
    }
    let mut errinfo_17 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_17 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_add_cred\x00" as *const u8 as *const i8,
        &mut (*mech).gss_add_cred
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_cred_usage_t,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_17,
    ) != 0
        || errinfo_17.code != 0
    {
        (*mech).gss_add_cred = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_17);
    }
    if (*mech).gss_add_cred
        == Some(
            crate::src::mechglue::g_acquire_cred::gss_add_cred
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_cred_usage_t,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_add_cred = None
    }
    let mut errinfo_18 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_18 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_export_sec_context\x00" as *const u8 as *const i8,
        &mut (*mech).gss_export_sec_context
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_18,
    ) != 0
        || errinfo_18.code != 0
    {
        (*mech).gss_export_sec_context = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_18);
    }
    if (*mech).gss_export_sec_context
        == Some(
            crate::src::mechglue::g_exp_sec_context::gss_export_sec_context
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_export_sec_context = None
    }
    let mut errinfo_19 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_19 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_import_sec_context\x00" as *const u8 as *const i8,
        &mut (*mech).gss_import_sec_context
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::gss_ctx_id_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_19,
    ) != 0
        || errinfo_19.code != 0
    {
        (*mech).gss_import_sec_context = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_19);
    }
    if (*mech).gss_import_sec_context
        == Some(
            crate::src::mechglue::g_imp_sec_context::gss_import_sec_context
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::gss_ctx_id_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_import_sec_context = None
    }
    let mut errinfo_20 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_20 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_inquire_cred_by_mech\x00" as *const u8 as *const i8,
        &mut (*mech).gss_inquire_cred_by_mech
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_h::gss_name_t,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_cred_usage_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_20,
    ) != 0
        || errinfo_20.code != 0
    {
        (*mech).gss_inquire_cred_by_mech = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_20);
    }
    if (*mech).gss_inquire_cred_by_mech
        == Some(
            crate::src::mechglue::g_inq_cred::gss_inquire_cred_by_mech
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_h::gss_name_t,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_cred_usage_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_inquire_cred_by_mech = None
    }
    let mut errinfo_21 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_21 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_inquire_names_for_mech\x00" as *const u8 as *const i8,
        &mut (*mech).gss_inquire_names_for_mech
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_h::gss_OID_set,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_21,
    ) != 0
        || errinfo_21.code != 0
    {
        (*mech).gss_inquire_names_for_mech = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_21);
    }
    if (*mech).gss_inquire_names_for_mech
        == Some(
            crate::src::mechglue::g_inq_names::gss_inquire_names_for_mech
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_h::gss_OID_set,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_inquire_names_for_mech = None
    }
    let mut errinfo_22 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_22 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_inquire_context\x00" as *const u8 as *const i8,
        &mut (*mech).gss_inquire_context
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: *mut crate::gssapi_h::gss_name_t,
                    _: *mut crate::gssapi_h::gss_name_t,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut i32,
                    _: *mut i32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_22,
    ) != 0
        || errinfo_22.code != 0
    {
        (*mech).gss_inquire_context = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_22);
    }
    if (*mech).gss_inquire_context
        == Some(
            crate::src::mechglue::g_inq_context::gss_inquire_context
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: *mut crate::gssapi_h::gss_name_t,
                    _: *mut crate::gssapi_h::gss_name_t,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut i32,
                    _: *mut i32,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_inquire_context = None
    }
    let mut errinfo_23 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_23 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_internal_release_oid\x00" as *const u8 as *const i8,
        &mut (*mech).gss_internal_release_oid
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_OID,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_23,
    ) != 0
        || errinfo_23.code != 0
    {
        (*mech).gss_internal_release_oid = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_23);
    }
    let mut errinfo_24 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_24 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_wrap_size_limit\x00" as *const u8 as *const i8,
        &mut (*mech).gss_wrap_size_limit
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: i32,
                    _: crate::gssapi_h::gss_qop_t,
                    _: crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_24,
    ) != 0
        || errinfo_24.code != 0
    {
        (*mech).gss_wrap_size_limit = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_24);
    }
    if (*mech).gss_wrap_size_limit
        == Some(
            crate::src::mechglue::g_seal::gss_wrap_size_limit
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: i32,
                    _: crate::gssapi_h::gss_qop_t,
                    _: crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_wrap_size_limit = None
    }
    let mut errinfo_25 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_25 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_localname\x00" as *const u8 as *const i8,
        &mut (*mech).gss_localname
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_const_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_25,
    ) != 0
        || errinfo_25.code != 0
    {
        (*mech).gss_localname = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_25);
    }
    if (*mech).gss_localname
        == Some(
            crate::src::mechglue::gssd_pname_to_uid::gss_localname
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_const_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_localname = None
    }
    let mut errinfo_26 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_26 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssspi_authorize_localname\x00" as *const u8 as *const i8,
        &mut (*mech).gssspi_authorize_localname
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_const_buffer_t,
                    _: crate::gssapi_h::gss_const_OID,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_26,
    ) != 0
        || errinfo_26.code != 0
    {
        (*mech).gssspi_authorize_localname = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_26);
    }
    let mut errinfo_27 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_27 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_export_name\x00" as *const u8 as *const i8,
        &mut (*mech).gss_export_name
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_27,
    ) != 0
        || errinfo_27.code != 0
    {
        (*mech).gss_export_name = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_27);
    }
    if (*mech).gss_export_name
        == Some(
            crate::src::mechglue::g_export_name::gss_export_name
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_export_name = None
    }
    let mut errinfo_28 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_28 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_duplicate_name\x00" as *const u8 as *const i8,
        &mut (*mech).gss_duplicate_name
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: *mut crate::gssapi_h::gss_name_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_28,
    ) != 0
        || errinfo_28.code != 0
    {
        (*mech).gss_duplicate_name = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_28);
    }
    if (*mech).gss_duplicate_name
        == Some(
            crate::src::mechglue::g_dup_name::gss_duplicate_name
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: *mut crate::gssapi_h::gss_name_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_duplicate_name = None
    }
    let mut errinfo_29 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_29 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_store_cred\x00" as *const u8 as *const i8,
        &mut (*mech).gss_store_cred
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_cred_usage_t,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::gss_cred_usage_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_29,
    ) != 0
        || errinfo_29.code != 0
    {
        (*mech).gss_store_cred = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_29);
    }
    if (*mech).gss_store_cred
        == Some(
            crate::src::mechglue::g_store_cred::gss_store_cred
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_cred_usage_t,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::gss_cred_usage_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_store_cred = None
    }
    let mut errinfo_30 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_30 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_inquire_sec_context_by_oid\x00" as *const u8 as *const i8,
        &mut (*mech).gss_inquire_sec_context_by_oid
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_30,
    ) != 0
        || errinfo_30.code != 0
    {
        (*mech).gss_inquire_sec_context_by_oid = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_30);
    }
    if (*mech).gss_inquire_sec_context_by_oid
        == Some(
            crate::src::mechglue::g_inq_context_oid::gss_inquire_sec_context_by_oid
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_inquire_sec_context_by_oid = None
    }
    let mut errinfo_31 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_31 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_inquire_cred_by_oid\x00" as *const u8 as *const i8,
        &mut (*mech).gss_inquire_cred_by_oid
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_31,
    ) != 0
        || errinfo_31.code != 0
    {
        (*mech).gss_inquire_cred_by_oid = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_31);
    }
    if (*mech).gss_inquire_cred_by_oid
        == Some(
            crate::src::mechglue::g_inq_cred_oid::gss_inquire_cred_by_oid
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_inquire_cred_by_oid = None
    }
    let mut errinfo_32 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_32 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_set_sec_context_option\x00" as *const u8 as *const i8,
        &mut (*mech).gss_set_sec_context_option
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_32,
    ) != 0
        || errinfo_32.code != 0
    {
        (*mech).gss_set_sec_context_option = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_32);
    }
    if (*mech).gss_set_sec_context_option
        == Some(
            crate::src::mechglue::g_set_context_option::gss_set_sec_context_option
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_set_sec_context_option = None
    }
    let mut errinfo_33 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_33 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssspi_set_cred_option\x00" as *const u8 as *const i8,
        &mut (*mech).gssspi_set_cred_option
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_33,
    ) != 0
        || errinfo_33.code != 0
    {
        (*mech).gssspi_set_cred_option = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_33);
    }
    let mut errinfo_34 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_34 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssspi_mech_invoke\x00" as *const u8 as *const i8,
        &mut (*mech).gssspi_mech_invoke
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_34,
    ) != 0
        || errinfo_34.code != 0
    {
        (*mech).gssspi_mech_invoke = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_34);
    }
    if (*mech).gssspi_mech_invoke
        == Some(
            crate::src::mechglue::g_mech_invoke::gssspi_mech_invoke
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gssspi_mech_invoke = None
    }
    let mut errinfo_35 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_35 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_wrap_aead\x00" as *const u8 as *const i8,
        &mut (*mech).gss_wrap_aead
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: i32,
                    _: crate::gssapi_h::gss_qop_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut i32,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_35,
    ) != 0
        || errinfo_35.code != 0
    {
        (*mech).gss_wrap_aead = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_35);
    }
    if (*mech).gss_wrap_aead
        == Some(
            crate::src::mechglue::g_wrap_aead::gss_wrap_aead
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: i32,
                    _: crate::gssapi_h::gss_qop_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut i32,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_wrap_aead = None
    }
    let mut errinfo_36 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_36 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_unwrap_aead\x00" as *const u8 as *const i8,
        &mut (*mech).gss_unwrap_aead
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut i32,
                    _: *mut crate::gssapi_h::gss_qop_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_36,
    ) != 0
        || errinfo_36.code != 0
    {
        (*mech).gss_unwrap_aead = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_36);
    }
    if (*mech).gss_unwrap_aead
        == Some(
            crate::src::mechglue::g_unwrap_aead::gss_unwrap_aead
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut i32,
                    _: *mut crate::gssapi_h::gss_qop_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_unwrap_aead = None
    }
    let mut errinfo_37 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_37 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_wrap_iov\x00" as *const u8 as *const i8,
        &mut (*mech).gss_wrap_iov
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: i32,
                    _: crate::gssapi_h::gss_qop_t,
                    _: *mut i32,
                    _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                    _: i32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_37,
    ) != 0
        || errinfo_37.code != 0
    {
        (*mech).gss_wrap_iov = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_37);
    }
    if (*mech).gss_wrap_iov
        == Some(
            crate::src::mechglue::g_wrap_iov::gss_wrap_iov
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: i32,
                    _: crate::gssapi_h::gss_qop_t,
                    _: *mut i32,
                    _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                    _: i32,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_wrap_iov = None
    }
    let mut errinfo_38 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_38 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_unwrap_iov\x00" as *const u8 as *const i8,
        &mut (*mech).gss_unwrap_iov
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: *mut i32,
                    _: *mut crate::gssapi_h::gss_qop_t,
                    _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                    _: i32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_38,
    ) != 0
        || errinfo_38.code != 0
    {
        (*mech).gss_unwrap_iov = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_38);
    }
    if (*mech).gss_unwrap_iov
        == Some(
            crate::src::mechglue::g_unwrap_iov::gss_unwrap_iov
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: *mut i32,
                    _: *mut crate::gssapi_h::gss_qop_t,
                    _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                    _: i32,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_unwrap_iov = None
    }
    let mut errinfo_39 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_39 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_wrap_iov_length\x00" as *const u8 as *const i8,
        &mut (*mech).gss_wrap_iov_length
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: i32,
                    _: crate::gssapi_h::gss_qop_t,
                    _: *mut i32,
                    _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                    _: i32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_39,
    ) != 0
        || errinfo_39.code != 0
    {
        (*mech).gss_wrap_iov_length = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_39);
    }
    if (*mech).gss_wrap_iov_length
        == Some(
            crate::src::mechglue::g_wrap_iov::gss_wrap_iov_length
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: i32,
                    _: crate::gssapi_h::gss_qop_t,
                    _: *mut i32,
                    _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                    _: i32,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_wrap_iov_length = None
    }
    let mut errinfo_40 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_40 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_complete_auth_token\x00" as *const u8 as *const i8,
        &mut (*mech).gss_complete_auth_token
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_40,
    ) != 0
        || errinfo_40.code != 0
    {
        (*mech).gss_complete_auth_token = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_40);
    }
    if (*mech).gss_complete_auth_token
        == Some(
            crate::src::mechglue::g_complete_auth_token::gss_complete_auth_token
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_complete_auth_token = None
    }
    /* Services4User (introduced in 1.8) */
    let mut errinfo_41 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_41 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_acquire_cred_impersonate_name\x00" as *const u8 as *const i8,
        &mut (*mech).gss_acquire_cred_impersonate_name
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID_set,
                    _: crate::gssapi_h::gss_cred_usage_t,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_41,
    ) != 0
        || errinfo_41.code != 0
    {
        (*mech).gss_acquire_cred_impersonate_name = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_41);
    }
    if (*mech).gss_acquire_cred_impersonate_name
        == Some(
            crate::src::mechglue::g_acquire_cred_imp_name::gss_acquire_cred_impersonate_name
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID_set,
                    _: crate::gssapi_h::gss_cred_usage_t,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_acquire_cred_impersonate_name = None
    }
    let mut errinfo_42 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_42 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_add_cred_impersonate_name\x00" as *const u8 as *const i8,
        &mut (*mech).gss_add_cred_impersonate_name
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_cred_usage_t,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_42,
    ) != 0
        || errinfo_42.code != 0
    {
        (*mech).gss_add_cred_impersonate_name = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_42);
    }
    if (*mech).gss_add_cred_impersonate_name
        == Some(
            crate::src::mechglue::g_acquire_cred_imp_name::gss_add_cred_impersonate_name
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_cred_usage_t,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_add_cred_impersonate_name = None
    }
    /* Naming extensions (introduced in 1.8) */
    let mut errinfo_43 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_43 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_display_name_ext\x00" as *const u8 as *const i8,
        &mut (*mech).gss_display_name_ext
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_43,
    ) != 0
        || errinfo_43.code != 0
    {
        (*mech).gss_display_name_ext = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_43);
    }
    if (*mech).gss_display_name_ext
        == Some(
            crate::src::mechglue::g_dsp_name_ext::gss_display_name_ext
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_display_name_ext = None
    }
    let mut errinfo_44 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_44 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_inquire_name\x00" as *const u8 as *const i8,
        &mut (*mech).gss_inquire_name
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: *mut i32,
                    _: *mut crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_44,
    ) != 0
        || errinfo_44.code != 0
    {
        (*mech).gss_inquire_name = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_44);
    }
    if (*mech).gss_inquire_name
        == Some(
            crate::src::mechglue::g_inq_name::gss_inquire_name
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: *mut i32,
                    _: *mut crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_inquire_name = None
    }
    let mut errinfo_45 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_45 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_get_name_attribute\x00" as *const u8 as *const i8,
        &mut (*mech).gss_get_name_attribute
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut i32,
                    _: *mut i32,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut i32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_45,
    ) != 0
        || errinfo_45.code != 0
    {
        (*mech).gss_get_name_attribute = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_45);
    }
    if (*mech).gss_get_name_attribute
        == Some(
            crate::src::mechglue::g_get_name_attr::gss_get_name_attribute
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut i32,
                    _: *mut i32,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut i32,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_get_name_attribute = None
    }
    let mut errinfo_46 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_46 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_set_name_attribute\x00" as *const u8 as *const i8,
        &mut (*mech).gss_set_name_attribute
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: i32,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_46,
    ) != 0
        || errinfo_46.code != 0
    {
        (*mech).gss_set_name_attribute = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_46);
    }
    if (*mech).gss_set_name_attribute
        == Some(
            crate::src::mechglue::g_set_name_attr::gss_set_name_attribute
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: i32,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_set_name_attribute = None
    }
    let mut errinfo_47 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_47 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_delete_name_attribute\x00" as *const u8 as *const i8,
        &mut (*mech).gss_delete_name_attribute
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_47,
    ) != 0
        || errinfo_47.code != 0
    {
        (*mech).gss_delete_name_attribute = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_47);
    }
    if (*mech).gss_delete_name_attribute
        == Some(
            crate::src::mechglue::g_del_name_attr::gss_delete_name_attribute
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_delete_name_attribute = None
    }
    let mut errinfo_48 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_48 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_export_name_composite\x00" as *const u8 as *const i8,
        &mut (*mech).gss_export_name_composite
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_48,
    ) != 0
        || errinfo_48.code != 0
    {
        (*mech).gss_export_name_composite = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_48);
    }
    if (*mech).gss_export_name_composite
        == Some(
            crate::src::mechglue::g_export_name_comp::gss_export_name_composite
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_export_name_composite = None
    }
    let mut errinfo_49 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_49 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_map_name_to_any\x00" as *const u8 as *const i8,
        &mut (*mech).gss_map_name_to_any
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: i32,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_ext_h::gss_any_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_49,
    ) != 0
        || errinfo_49.code != 0
    {
        (*mech).gss_map_name_to_any = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_49);
    }
    if (*mech).gss_map_name_to_any
        == Some(
            crate::src::mechglue::g_map_name_to_any::gss_map_name_to_any
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: i32,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_ext_h::gss_any_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_map_name_to_any = None
    }
    let mut errinfo_50 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_50 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_release_any_name_mapping\x00" as *const u8 as *const i8,
        &mut (*mech).gss_release_any_name_mapping
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_ext_h::gss_any_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_50,
    ) != 0
        || errinfo_50.code != 0
    {
        (*mech).gss_release_any_name_mapping = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_50);
    }
    if (*mech).gss_release_any_name_mapping
        == Some(
            crate::src::mechglue::g_rel_name_mapping::gss_release_any_name_mapping
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_ext_h::gss_any_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_release_any_name_mapping = None
    }
    /* RFC 4401 (introduced in 1.8) */
    let mut errinfo_51 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_51 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_pseudo_random\x00" as *const u8 as *const i8,
        &mut (*mech).gss_pseudo_random
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: i32,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::stdlib::ssize_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_51,
    ) != 0
        || errinfo_51.code != 0
    {
        (*mech).gss_pseudo_random = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_51);
    }
    if (*mech).gss_pseudo_random
        == Some(
            crate::src::mechglue::g_prf::gss_pseudo_random
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: i32,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::stdlib::ssize_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_pseudo_random = None
    }
    /* RFC 4178 (introduced in 1.8; gss_get_neg_mechs not implemented) */
    let mut errinfo_52 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_52 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_set_neg_mechs\x00" as *const u8 as *const i8,
        &mut (*mech).gss_set_neg_mechs
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_OID_set,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_52,
    ) != 0
        || errinfo_52.code != 0
    {
        (*mech).gss_set_neg_mechs = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_52);
    }
    if (*mech).gss_set_neg_mechs
        == Some(
            crate::src::mechglue::g_set_neg_mechs::gss_set_neg_mechs
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_OID_set,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_set_neg_mechs = None
    }
    /* draft-ietf-sasl-gs2 */
    let mut errinfo_53 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_53 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_inquire_saslname_for_mech\x00" as *const u8 as *const i8,
        &mut (*mech).gss_inquire_saslname_for_mech
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_53,
    ) != 0
        || errinfo_53.code != 0
    {
        (*mech).gss_inquire_saslname_for_mech = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_53);
    }
    if (*mech).gss_inquire_saslname_for_mech
        == Some(
            crate::src::mechglue::g_saslname::gss_inquire_saslname_for_mech
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_inquire_saslname_for_mech = None
    }
    let mut errinfo_54 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_54 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_inquire_mech_for_saslname\x00" as *const u8 as *const i8,
        &mut (*mech).gss_inquire_mech_for_saslname
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::gss_OID,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_54,
    ) != 0
        || errinfo_54.code != 0
    {
        (*mech).gss_inquire_mech_for_saslname = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_54);
    }
    if (*mech).gss_inquire_mech_for_saslname
        == Some(
            crate::src::mechglue::g_saslname::gss_inquire_mech_for_saslname
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::gss_OID,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_inquire_mech_for_saslname = None
    }
    /* RFC 5587 */
    let mut errinfo_55 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_55 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_inquire_attrs_for_mech\x00" as *const u8 as *const i8,
        &mut (*mech).gss_inquire_attrs_for_mech
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_const_OID,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::gss_OID_set,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_55,
    ) != 0
        || errinfo_55.code != 0
    {
        (*mech).gss_inquire_attrs_for_mech = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_55);
    }
    if (*mech).gss_inquire_attrs_for_mech
        == Some(
            crate::src::mechglue::g_mechattr::gss_inquire_attrs_for_mech
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_const_OID,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::gss_OID_set,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_inquire_attrs_for_mech = None
    }
    let mut errinfo_56 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_56 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_acquire_cred_from\x00" as *const u8 as *const i8,
        &mut (*mech).gss_acquire_cred_from
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID_set,
                    _: crate::gssapi_h::gss_cred_usage_t,
                    _: crate::gssapi_ext_h::gss_const_key_value_set_t,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_56,
    ) != 0
        || errinfo_56.code != 0
    {
        (*mech).gss_acquire_cred_from = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_56);
    }
    if (*mech).gss_acquire_cred_from
        == Some(
            crate::src::mechglue::g_acquire_cred::gss_acquire_cred_from
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID_set,
                    _: crate::gssapi_h::gss_cred_usage_t,
                    _: crate::gssapi_ext_h::gss_const_key_value_set_t,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_acquire_cred_from = None
    }
    let mut errinfo_57 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_57 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_store_cred_into\x00" as *const u8 as *const i8,
        &mut (*mech).gss_store_cred_into
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_cred_usage_t,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_ext_h::gss_const_key_value_set_t,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::gss_cred_usage_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_57,
    ) != 0
        || errinfo_57.code != 0
    {
        (*mech).gss_store_cred_into = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_57);
    }
    if (*mech).gss_store_cred_into
        == Some(
            crate::src::mechglue::g_store_cred::gss_store_cred_into
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_cred_usage_t,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_ext_h::gss_const_key_value_set_t,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::gss_cred_usage_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_store_cred_into = None
    }
    let mut errinfo_58 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_58 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssspi_acquire_cred_with_password\x00" as *const u8 as *const i8,
        &mut (*mech).gssspi_acquire_cred_with_password
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID_set,
                    _: i32,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_58,
    ) != 0
        || errinfo_58.code != 0
    {
        (*mech).gssspi_acquire_cred_with_password = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_58);
    }
    let mut errinfo_59 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_59 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_export_cred\x00" as *const u8 as *const i8,
        &mut (*mech).gss_export_cred
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_59,
    ) != 0
        || errinfo_59.code != 0
    {
        (*mech).gss_export_cred = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_59);
    }
    if (*mech).gss_export_cred
        == Some(
            crate::src::mechglue::g_export_cred::gss_export_cred
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_export_cred = None
    }
    let mut errinfo_60 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_60 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gss_import_cred\x00" as *const u8 as *const i8,
        &mut (*mech).gss_import_cred
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_60,
    ) != 0
        || errinfo_60.code != 0
    {
        (*mech).gss_import_cred = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_60);
    }
    if (*mech).gss_import_cred
        == Some(
            crate::src::mechglue::g_imp_cred::gss_import_cred
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gss_import_cred = None
    }
    let mut errinfo_61 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_61 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssspi_import_sec_context_by_mech\x00" as *const u8 as *const i8,
        &mut (*mech).gssspi_import_sec_context_by_mech
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::gss_ctx_id_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_61,
    ) != 0
        || errinfo_61.code != 0
    {
        (*mech).gssspi_import_sec_context_by_mech = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_61);
    }
    let mut errinfo_62 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_62 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssspi_import_name_by_mech\x00" as *const u8 as *const i8,
        &mut (*mech).gssspi_import_name_by_mech
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_h::gss_name_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_62,
    ) != 0
        || errinfo_62.code != 0
    {
        (*mech).gssspi_import_name_by_mech = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_62);
    }
    let mut errinfo_63 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_63 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssspi_import_cred_by_mech\x00" as *const u8 as *const i8,
        &mut (*mech).gssspi_import_cred_by_mech
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_63,
    ) != 0
        || errinfo_63.code != 0
    {
        (*mech).gssspi_import_cred_by_mech = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_63);
    }
    /* draft-zhu-negoex */
    let mut errinfo_64 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_64 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssspi_query_meta_data\x00" as *const u8 as *const i8,
        &mut (*mech).gssspi_query_meta_data
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_const_OID,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_64,
    ) != 0
        || errinfo_64.code != 0
    {
        (*mech).gssspi_query_meta_data = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_64);
    }
    if (*mech).gssspi_query_meta_data
        == Some(
            crate::src::mechglue::g_negoex::gssspi_query_meta_data
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_const_OID,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gssspi_query_meta_data = None
    }
    let mut errinfo_65 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_65 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssspi_exchange_meta_data\x00" as *const u8 as *const i8,
        &mut (*mech).gssspi_exchange_meta_data
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_const_OID,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_const_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_65,
    ) != 0
        || errinfo_65.code != 0
    {
        (*mech).gssspi_exchange_meta_data = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_65);
    }
    if (*mech).gssspi_exchange_meta_data
        == Some(
            crate::src::mechglue::g_negoex::gssspi_exchange_meta_data
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_const_OID,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_const_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gssspi_exchange_meta_data = None
    }
    let mut errinfo_66 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_66 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssspi_query_mechanism_info\x00" as *const u8 as *const i8,
        &mut (*mech).gssspi_query_mechanism_info
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_const_OID,
                    _: *mut u8,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_66,
    ) != 0
        || errinfo_66.code != 0
    {
        (*mech).gssspi_query_mechanism_info = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_66);
    }
    if (*mech).gssspi_query_mechanism_info
        == Some(
            crate::src::mechglue::g_negoex::gssspi_query_mechanism_info
                as unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_const_OID,
                    _: *mut u8,
                ) -> crate::gssapi_h::OM_uint32,
        )
    {
        (*mech).gssspi_query_mechanism_info = None
    }
    if !mech_type.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"mech_type != GSS_C_NO_OID\x00" as *const u8 as *const i8,
            b"g_initialize.c\x00" as *const u8 as *const i8,
            778u32,
            (*::std::mem::transmute::<&[u8; 55], &[i8; 55]>(
                b"gss_mechanism build_dynamicMech(void *, const gss_OID)\x00",
            ))
            .as_ptr(),
        );
    }
    (*mech).mech_type = *mech_type;
    return mech;
}
/* Build an interposer mechanism function table from dl. */

unsafe extern "C" fn build_interMech(
    mut dl: *mut libc::c_void,
    mech_type: crate::gssapi_h::gss_OID,
) -> crate::mglueP_h::gss_mechanism {
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    mech = crate::stdlib::calloc(1usize, ::std::mem::size_of::<crate::mglueP_h::gss_config>())
        as crate::mglueP_h::gss_mechanism;
    if mech.is_null() {
        return 0 as crate::mglueP_h::gss_mechanism;
    }
    let mut errinfo = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_acquire_cred\x00" as *const u8 as *const i8,
        &mut (*mech).gss_acquire_cred
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID_set,
                    _: i32,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo,
    ) != 0
        || errinfo.code != 0
    {
        (*mech).gss_acquire_cred = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo);
    }
    let mut errinfo_0 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_0 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_release_cred\x00" as *const u8 as *const i8,
        &mut (*mech).gss_release_cred
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_0,
    ) != 0
        || errinfo_0.code != 0
    {
        (*mech).gss_release_cred = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_0);
    }
    let mut errinfo_1 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_1 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_init_sec_context\x00" as *const u8 as *const i8,
        &mut (*mech).gss_init_sec_context
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_channel_bindings_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_1,
    ) != 0
        || errinfo_1.code != 0
    {
        (*mech).gss_init_sec_context = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_1);
    }
    let mut errinfo_2 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_2 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_accept_sec_context\x00" as *const u8 as *const i8,
        &mut (*mech).gss_accept_sec_context
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_channel_bindings_t,
                    _: *mut crate::gssapi_h::gss_name_t,
                    _: *mut crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_2,
    ) != 0
        || errinfo_2.code != 0
    {
        (*mech).gss_accept_sec_context = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_2);
    }
    let mut errinfo_3 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_3 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_process_context_token\x00" as *const u8 as *const i8,
        &mut (*mech).gss_process_context_token
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_3,
    ) != 0
        || errinfo_3.code != 0
    {
        (*mech).gss_process_context_token = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_3);
    }
    let mut errinfo_4 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_4 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_delete_sec_context\x00" as *const u8 as *const i8,
        &mut (*mech).gss_delete_sec_context
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_4,
    ) != 0
        || errinfo_4.code != 0
    {
        (*mech).gss_delete_sec_context = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_4);
    }
    let mut errinfo_5 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_5 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_context_time\x00" as *const u8 as *const i8,
        &mut (*mech).gss_context_time
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_5,
    ) != 0
        || errinfo_5.code != 0
    {
        (*mech).gss_context_time = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_5);
    }
    let mut errinfo_6 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_6 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_get_mic\x00" as *const u8 as *const i8,
        &mut (*mech).gss_get_mic
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_qop_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_6,
    ) != 0
        || errinfo_6.code != 0
    {
        (*mech).gss_get_mic = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_6);
    }
    let mut errinfo_7 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_7 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_verify_mic\x00" as *const u8 as *const i8,
        &mut (*mech).gss_verify_mic
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::gss_qop_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_7,
    ) != 0
        || errinfo_7.code != 0
    {
        (*mech).gss_verify_mic = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_7);
    }
    let mut errinfo_8 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_8 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_wrap\x00" as *const u8 as *const i8,
        &mut (*mech).gss_wrap
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: i32,
                    _: crate::gssapi_h::gss_qop_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut i32,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_8,
    ) != 0
        || errinfo_8.code != 0
    {
        (*mech).gss_wrap = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_8);
    }
    let mut errinfo_9 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_9 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_unwrap\x00" as *const u8 as *const i8,
        &mut (*mech).gss_unwrap
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut i32,
                    _: *mut crate::gssapi_h::gss_qop_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_9,
    ) != 0
        || errinfo_9.code != 0
    {
        (*mech).gss_unwrap = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_9);
    }
    let mut errinfo_10 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_10 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_display_status\x00" as *const u8 as *const i8,
        &mut (*mech).gss_display_status
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::OM_uint32,
                    _: i32,
                    _: crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_10,
    ) != 0
        || errinfo_10.code != 0
    {
        (*mech).gss_display_status = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_10);
    }
    let mut errinfo_11 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_11 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_indicate_mechs\x00" as *const u8 as *const i8,
        &mut (*mech).gss_indicate_mechs
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_OID_set,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_11,
    ) != 0
        || errinfo_11.code != 0
    {
        (*mech).gss_indicate_mechs = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_11);
    }
    let mut errinfo_12 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_12 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_compare_name\x00" as *const u8 as *const i8,
        &mut (*mech).gss_compare_name
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_name_t,
                    _: *mut i32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_12,
    ) != 0
        || errinfo_12.code != 0
    {
        (*mech).gss_compare_name = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_12);
    }
    let mut errinfo_13 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_13 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_display_name\x00" as *const u8 as *const i8,
        &mut (*mech).gss_display_name
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::gss_OID,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_13,
    ) != 0
        || errinfo_13.code != 0
    {
        (*mech).gss_display_name = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_13);
    }
    let mut errinfo_14 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_14 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_import_name\x00" as *const u8 as *const i8,
        &mut (*mech).gss_import_name
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_h::gss_name_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_14,
    ) != 0
        || errinfo_14.code != 0
    {
        (*mech).gss_import_name = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_14);
    }
    let mut errinfo_15 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_15 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_release_name\x00" as *const u8 as *const i8,
        &mut (*mech).gss_release_name
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_name_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_15,
    ) != 0
        || errinfo_15.code != 0
    {
        (*mech).gss_release_name = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_15);
    }
    let mut errinfo_16 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_16 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_inquire_cred\x00" as *const u8 as *const i8,
        &mut (*mech).gss_inquire_cred
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_name_t,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut i32,
                    _: *mut crate::gssapi_h::gss_OID_set,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_16,
    ) != 0
        || errinfo_16.code != 0
    {
        (*mech).gss_inquire_cred = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_16);
    }
    let mut errinfo_17 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_17 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_add_cred\x00" as *const u8 as *const i8,
        &mut (*mech).gss_add_cred
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_cred_usage_t,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_17,
    ) != 0
        || errinfo_17.code != 0
    {
        (*mech).gss_add_cred = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_17);
    }
    let mut errinfo_18 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_18 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_export_sec_context\x00" as *const u8 as *const i8,
        &mut (*mech).gss_export_sec_context
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_18,
    ) != 0
        || errinfo_18.code != 0
    {
        (*mech).gss_export_sec_context = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_18);
    }
    let mut errinfo_19 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_19 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_import_sec_context\x00" as *const u8 as *const i8,
        &mut (*mech).gss_import_sec_context
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::gss_ctx_id_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_19,
    ) != 0
        || errinfo_19.code != 0
    {
        (*mech).gss_import_sec_context = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_19);
    }
    let mut errinfo_20 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_20 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_inquire_cred_by_mech\x00" as *const u8 as *const i8,
        &mut (*mech).gss_inquire_cred_by_mech
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_h::gss_name_t,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_cred_usage_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_20,
    ) != 0
        || errinfo_20.code != 0
    {
        (*mech).gss_inquire_cred_by_mech = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_20);
    }
    let mut errinfo_21 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_21 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_inquire_names_for_mech\x00" as *const u8 as *const i8,
        &mut (*mech).gss_inquire_names_for_mech
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_h::gss_OID_set,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_21,
    ) != 0
        || errinfo_21.code != 0
    {
        (*mech).gss_inquire_names_for_mech = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_21);
    }
    let mut errinfo_22 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_22 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_inquire_context\x00" as *const u8 as *const i8,
        &mut (*mech).gss_inquire_context
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: *mut crate::gssapi_h::gss_name_t,
                    _: *mut crate::gssapi_h::gss_name_t,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut i32,
                    _: *mut i32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_22,
    ) != 0
        || errinfo_22.code != 0
    {
        (*mech).gss_inquire_context = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_22);
    }
    let mut errinfo_23 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_23 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_internal_release_oid\x00" as *const u8 as *const i8,
        &mut (*mech).gss_internal_release_oid
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_OID,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_23,
    ) != 0
        || errinfo_23.code != 0
    {
        (*mech).gss_internal_release_oid = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_23);
    }
    let mut errinfo_24 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_24 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_wrap_size_limit\x00" as *const u8 as *const i8,
        &mut (*mech).gss_wrap_size_limit
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: i32,
                    _: crate::gssapi_h::gss_qop_t,
                    _: crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_24,
    ) != 0
        || errinfo_24.code != 0
    {
        (*mech).gss_wrap_size_limit = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_24);
    }
    let mut errinfo_25 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_25 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_localname\x00" as *const u8 as *const i8,
        &mut (*mech).gss_localname
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_const_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_25,
    ) != 0
        || errinfo_25.code != 0
    {
        (*mech).gss_localname = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_25);
    }
    let mut errinfo_26 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_26 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_authorize_localname\x00" as *const u8 as *const i8,
        &mut (*mech).gssspi_authorize_localname
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_const_buffer_t,
                    _: crate::gssapi_h::gss_const_OID,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_26,
    ) != 0
        || errinfo_26.code != 0
    {
        (*mech).gssspi_authorize_localname = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_26);
    }
    let mut errinfo_27 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_27 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_export_name\x00" as *const u8 as *const i8,
        &mut (*mech).gss_export_name
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_27,
    ) != 0
        || errinfo_27.code != 0
    {
        (*mech).gss_export_name = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_27);
    }
    let mut errinfo_28 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_28 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_duplicate_name\x00" as *const u8 as *const i8,
        &mut (*mech).gss_duplicate_name
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: *mut crate::gssapi_h::gss_name_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_28,
    ) != 0
        || errinfo_28.code != 0
    {
        (*mech).gss_duplicate_name = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_28);
    }
    let mut errinfo_29 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_29 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_store_cred\x00" as *const u8 as *const i8,
        &mut (*mech).gss_store_cred
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_cred_usage_t,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::gss_cred_usage_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_29,
    ) != 0
        || errinfo_29.code != 0
    {
        (*mech).gss_store_cred = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_29);
    }
    let mut errinfo_30 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_30 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_inquire_sec_context_by_oid\x00" as *const u8 as *const i8,
        &mut (*mech).gss_inquire_sec_context_by_oid
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_30,
    ) != 0
        || errinfo_30.code != 0
    {
        (*mech).gss_inquire_sec_context_by_oid = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_30);
    }
    let mut errinfo_31 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_31 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_inquire_cred_by_oid\x00" as *const u8 as *const i8,
        &mut (*mech).gss_inquire_cred_by_oid
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_31,
    ) != 0
        || errinfo_31.code != 0
    {
        (*mech).gss_inquire_cred_by_oid = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_31);
    }
    let mut errinfo_32 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_32 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_set_sec_context_option\x00" as *const u8 as *const i8,
        &mut (*mech).gss_set_sec_context_option
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_32,
    ) != 0
        || errinfo_32.code != 0
    {
        (*mech).gss_set_sec_context_option = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_32);
    }
    let mut errinfo_33 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_33 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_set_cred_option\x00" as *const u8 as *const i8,
        &mut (*mech).gssspi_set_cred_option
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_33,
    ) != 0
        || errinfo_33.code != 0
    {
        (*mech).gssspi_set_cred_option = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_33);
    }
    let mut errinfo_34 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_34 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_mech_invoke\x00" as *const u8 as *const i8,
        &mut (*mech).gssspi_mech_invoke
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_34,
    ) != 0
        || errinfo_34.code != 0
    {
        (*mech).gssspi_mech_invoke = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_34);
    }
    let mut errinfo_35 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_35 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_wrap_aead\x00" as *const u8 as *const i8,
        &mut (*mech).gss_wrap_aead
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: i32,
                    _: crate::gssapi_h::gss_qop_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut i32,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_35,
    ) != 0
        || errinfo_35.code != 0
    {
        (*mech).gss_wrap_aead = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_35);
    }
    let mut errinfo_36 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_36 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_unwrap_aead\x00" as *const u8 as *const i8,
        &mut (*mech).gss_unwrap_aead
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut i32,
                    _: *mut crate::gssapi_h::gss_qop_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_36,
    ) != 0
        || errinfo_36.code != 0
    {
        (*mech).gss_unwrap_aead = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_36);
    }
    let mut errinfo_37 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_37 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_wrap_iov\x00" as *const u8 as *const i8,
        &mut (*mech).gss_wrap_iov
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: i32,
                    _: crate::gssapi_h::gss_qop_t,
                    _: *mut i32,
                    _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                    _: i32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_37,
    ) != 0
        || errinfo_37.code != 0
    {
        (*mech).gss_wrap_iov = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_37);
    }
    let mut errinfo_38 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_38 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_unwrap_iov\x00" as *const u8 as *const i8,
        &mut (*mech).gss_unwrap_iov
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: *mut i32,
                    _: *mut crate::gssapi_h::gss_qop_t,
                    _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                    _: i32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_38,
    ) != 0
        || errinfo_38.code != 0
    {
        (*mech).gss_unwrap_iov = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_38);
    }
    let mut errinfo_39 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_39 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_wrap_iov_length\x00" as *const u8 as *const i8,
        &mut (*mech).gss_wrap_iov_length
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: i32,
                    _: crate::gssapi_h::gss_qop_t,
                    _: *mut i32,
                    _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                    _: i32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_39,
    ) != 0
        || errinfo_39.code != 0
    {
        (*mech).gss_wrap_iov_length = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_39);
    }
    let mut errinfo_40 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_40 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_complete_auth_token\x00" as *const u8 as *const i8,
        &mut (*mech).gss_complete_auth_token
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_40,
    ) != 0
        || errinfo_40.code != 0
    {
        (*mech).gss_complete_auth_token = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_40);
    }
    /* Services4User (introduced in 1.8) */
    let mut errinfo_41 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_41 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_acquire_cred_impersonate_name\x00" as *const u8 as *const i8,
        &mut (*mech).gss_acquire_cred_impersonate_name
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID_set,
                    _: crate::gssapi_h::gss_cred_usage_t,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_41,
    ) != 0
        || errinfo_41.code != 0
    {
        (*mech).gss_acquire_cred_impersonate_name = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_41);
    }
    let mut errinfo_42 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_42 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_add_cred_impersonate_name\x00" as *const u8 as *const i8,
        &mut (*mech).gss_add_cred_impersonate_name
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_cred_usage_t,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_42,
    ) != 0
        || errinfo_42.code != 0
    {
        (*mech).gss_add_cred_impersonate_name = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_42);
    }
    /* Naming extensions (introduced in 1.8) */
    let mut errinfo_43 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_43 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_display_name_ext\x00" as *const u8 as *const i8,
        &mut (*mech).gss_display_name_ext
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_43,
    ) != 0
        || errinfo_43.code != 0
    {
        (*mech).gss_display_name_ext = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_43);
    }
    let mut errinfo_44 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_44 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_inquire_name\x00" as *const u8 as *const i8,
        &mut (*mech).gss_inquire_name
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: *mut i32,
                    _: *mut crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_44,
    ) != 0
        || errinfo_44.code != 0
    {
        (*mech).gss_inquire_name = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_44);
    }
    let mut errinfo_45 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_45 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_get_name_attribute\x00" as *const u8 as *const i8,
        &mut (*mech).gss_get_name_attribute
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut i32,
                    _: *mut i32,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut i32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_45,
    ) != 0
        || errinfo_45.code != 0
    {
        (*mech).gss_get_name_attribute = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_45);
    }
    let mut errinfo_46 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_46 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_set_name_attribute\x00" as *const u8 as *const i8,
        &mut (*mech).gss_set_name_attribute
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: i32,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_46,
    ) != 0
        || errinfo_46.code != 0
    {
        (*mech).gss_set_name_attribute = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_46);
    }
    let mut errinfo_47 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_47 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_delete_name_attribute\x00" as *const u8 as *const i8,
        &mut (*mech).gss_delete_name_attribute
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_47,
    ) != 0
        || errinfo_47.code != 0
    {
        (*mech).gss_delete_name_attribute = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_47);
    }
    let mut errinfo_48 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_48 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_export_name_composite\x00" as *const u8 as *const i8,
        &mut (*mech).gss_export_name_composite
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_48,
    ) != 0
        || errinfo_48.code != 0
    {
        (*mech).gss_export_name_composite = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_48);
    }
    let mut errinfo_49 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_49 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_map_name_to_any\x00" as *const u8 as *const i8,
        &mut (*mech).gss_map_name_to_any
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: i32,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_ext_h::gss_any_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_49,
    ) != 0
        || errinfo_49.code != 0
    {
        (*mech).gss_map_name_to_any = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_49);
    }
    let mut errinfo_50 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_50 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_release_any_name_mapping\x00" as *const u8 as *const i8,
        &mut (*mech).gss_release_any_name_mapping
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_ext_h::gss_any_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_50,
    ) != 0
        || errinfo_50.code != 0
    {
        (*mech).gss_release_any_name_mapping = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_50);
    }
    /* RFC 4401 (introduced in 1.8) */
    let mut errinfo_51 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_51 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_pseudo_random\x00" as *const u8 as *const i8,
        &mut (*mech).gss_pseudo_random
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_ctx_id_t,
                    _: i32,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::stdlib::ssize_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_51,
    ) != 0
        || errinfo_51.code != 0
    {
        (*mech).gss_pseudo_random = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_51);
    }
    /* RFC 4178 (introduced in 1.8; get_neg_mechs not implemented) */
    let mut errinfo_52 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_52 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_set_neg_mechs\x00" as *const u8 as *const i8,
        &mut (*mech).gss_set_neg_mechs
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_OID_set,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_52,
    ) != 0
        || errinfo_52.code != 0
    {
        (*mech).gss_set_neg_mechs = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_52);
    }
    /* draft-ietf-sasl-gs2 */
    let mut errinfo_53 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_53 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_inquire_saslname_for_mech\x00" as *const u8 as *const i8,
        &mut (*mech).gss_inquire_saslname_for_mech
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_53,
    ) != 0
        || errinfo_53.code != 0
    {
        (*mech).gss_inquire_saslname_for_mech = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_53);
    }
    let mut errinfo_54 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_54 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_inquire_mech_for_saslname\x00" as *const u8 as *const i8,
        &mut (*mech).gss_inquire_mech_for_saslname
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::gss_OID,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_54,
    ) != 0
        || errinfo_54.code != 0
    {
        (*mech).gss_inquire_mech_for_saslname = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_54);
    }
    /* RFC 5587 */
    let mut errinfo_55 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_55 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_inquire_attrs_for_mech\x00" as *const u8 as *const i8,
        &mut (*mech).gss_inquire_attrs_for_mech
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_const_OID,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::gss_OID_set,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_55,
    ) != 0
        || errinfo_55.code != 0
    {
        (*mech).gss_inquire_attrs_for_mech = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_55);
    }
    let mut errinfo_56 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_56 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_acquire_cred_from\x00" as *const u8 as *const i8,
        &mut (*mech).gss_acquire_cred_from
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID_set,
                    _: crate::gssapi_h::gss_cred_usage_t,
                    _: crate::gssapi_ext_h::gss_const_key_value_set_t,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_56,
    ) != 0
        || errinfo_56.code != 0
    {
        (*mech).gss_acquire_cred_from = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_56);
    }
    let mut errinfo_57 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_57 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_store_cred_into\x00" as *const u8 as *const i8,
        &mut (*mech).gss_store_cred_into
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_cred_usage_t,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_ext_h::gss_const_key_value_set_t,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::gss_cred_usage_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_57,
    ) != 0
        || errinfo_57.code != 0
    {
        (*mech).gss_store_cred_into = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_57);
    }
    let mut errinfo_58 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_58 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_acquire_cred_with_password\x00" as *const u8 as *const i8,
        &mut (*mech).gssspi_acquire_cred_with_password
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_name_t,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID_set,
                    _: i32,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                    _: *mut crate::gssapi_h::gss_OID_set,
                    _: *mut crate::gssapi_h::OM_uint32,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_58,
    ) != 0
        || errinfo_58.code != 0
    {
        (*mech).gssspi_acquire_cred_with_password = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_58);
    }
    let mut errinfo_59 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_59 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_export_cred\x00" as *const u8 as *const i8,
        &mut (*mech).gss_export_cred
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_cred_id_t,
                    _: crate::gssapi_h::gss_buffer_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_59,
    ) != 0
        || errinfo_59.code != 0
    {
        (*mech).gss_export_cred = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_59);
    }
    let mut errinfo_60 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_60 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_import_cred\x00" as *const u8 as *const i8,
        &mut (*mech).gss_import_cred
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_60,
    ) != 0
        || errinfo_60.code != 0
    {
        (*mech).gss_import_cred = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_60);
    }
    let mut errinfo_61 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_61 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_import_sec_context_by_mech\x00" as *const u8 as *const i8,
        &mut (*mech).gssspi_import_sec_context_by_mech
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::gss_ctx_id_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_61,
    ) != 0
        || errinfo_61.code != 0
    {
        (*mech).gssspi_import_sec_context_by_mech = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_61);
    }
    let mut errinfo_62 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_62 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_import_name_by_mech\x00" as *const u8 as *const i8,
        &mut (*mech).gssspi_import_name_by_mech
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: crate::gssapi_h::gss_OID,
                    _: *mut crate::gssapi_h::gss_name_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_62,
    ) != 0
        || errinfo_62.code != 0
    {
        (*mech).gssspi_import_name_by_mech = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_62);
    }
    let mut errinfo_63 = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    crate::stdlib::memset(
        &mut errinfo_63 as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl as *mut crate::k5_plugin_h::plugin_file_handle,
        b"gssi_import_cred_by_mech\x00" as *const u8 as *const i8,
        &mut (*mech).gssspi_import_cred_by_mech
            as *mut Option<
                unsafe extern "C" fn(
                    _: *mut crate::gssapi_h::OM_uint32,
                    _: crate::gssapi_h::gss_OID,
                    _: crate::gssapi_h::gss_buffer_t,
                    _: *mut crate::gssapi_h::gss_cred_id_t,
                ) -> crate::gssapi_h::OM_uint32,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo_63,
    ) != 0
        || errinfo_63.code != 0
    {
        (*mech).gssspi_import_cred_by_mech = None;
        crate::k5_err_h::k5_clear_error(&mut errinfo_63);
    }
    (*mech).mech_type = *mech_type;
    return mech;
}
/*
 * Concatenate an interposer mech OID and a real mech OID to create an
 * identifier for the interposed mech.  (The concatenation will not be a valid
 * DER OID encoding, but the OID is only used internally.)
 */

unsafe extern "C" fn interposed_oid(
    mut pre: crate::gssapi_h::gss_OID,
    mut real: crate::gssapi_h::gss_OID,
) -> crate::gssapi_h::gss_OID {
    let mut o = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    o = crate::stdlib::malloc(::std::mem::size_of::<crate::gssapi_h::gss_OID_desc>())
        as crate::gssapi_h::gss_OID;
    if o.is_null() {
        return 0 as crate::gssapi_h::gss_OID;
    }
    (*o).length = (*pre).length.wrapping_add((*real).length);
    (*o).elements = crate::stdlib::malloc((*o).length as usize);
    if (*o).elements.is_null() {
        crate::stdlib::free(o as *mut libc::c_void);
        return 0 as crate::gssapi_h::gss_OID;
    }
    crate::stdlib::memcpy((*o).elements, (*pre).elements, (*pre).length as usize);
    crate::stdlib::memcpy(
        ((*o).elements as *mut i8).offset((*pre).length as isize) as *mut libc::c_void,
        (*real).elements,
        (*real).length as usize,
    );
    return o;
}

unsafe extern "C" fn loadInterMech(mut minfo: crate::mglueP_h::gss_mech_info) {
    let mut dl = 0 as *mut crate::k5_plugin_h::plugin_file_handle;
    let mut errinfo = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    let mut isym: Option<
        unsafe extern "C" fn(_: crate::gssapi_h::gss_OID) -> crate::gssapi_h::gss_OID_set,
    > = None;
    let mut list = 0 as *mut crate::gssapi_h::gss_OID_set_desc_struct;
    let mut oid = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut min: crate::gssapi_h::OM_uint32 = 0;
    let mut mi = 0 as *mut crate::mglueP_h::gss_mech_config;
    let mut i: crate::stddef_h::size_t = 0;
    crate::stdlib::memset(
        &mut errinfo as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_open_plugin((*minfo).uLibName, &mut dl, &mut errinfo) != 0isize
        || errinfo.code != 0isize
    {
        return;
    }
    if !(crate::k5_plugin_h::krb5int_get_plugin_func(
        dl,
        b"gss_mech_interposer\x00" as *const u8 as *const i8,
        &mut isym
            as *mut Option<
                unsafe extern "C" fn(_: crate::gssapi_h::gss_OID) -> crate::gssapi_h::gss_OID_set,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo,
    ) != 0isize)
    {
        /* Get a list of mechs to interpose. */
        list = Some(isym.expect("non-null function pointer")).expect("non-null function pointer")(
            (*minfo).mech_type,
        );
        if !list.is_null() {
            (*minfo).mech = build_interMech(dl as *mut libc::c_void, (*minfo).mech_type);
            if !(*minfo).mech.is_null() {
                (*minfo).freeMech = 1i32;
                /* Add interposer fields for each interposed mech. */
                i = 0usize;
                while i < (*list).count {
                    /* Skip this mech if it doesn't exist or is already
                     * interposed. */
                    oid = &mut *(*list).elements.offset(i as isize)
                        as *mut crate::gssapi_h::gss_OID_desc_struct;
                    mi = searchMechList(oid as crate::gssapi_h::gss_const_OID);
                    if !(mi.is_null() || !(*mi).int_mech_type.is_null()) {
                        /* Construct a special OID to represent the interposed mech. */
                        (*mi).int_mech_type = interposed_oid((*minfo).mech_type, oid);
                        if !(*mi).int_mech_type.is_null() {
                            /* Save an alias to the interposer's function table. */
                            (*mi).int_mech = (*minfo).mech
                        }
                    }
                    i = i.wrapping_add(1)
                }
                crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(&mut min, &mut list);
                (*minfo).dl_handle = dl as *mut libc::c_void;
                dl = 0 as *mut crate::k5_plugin_h::plugin_file_handle
            }
        }
    }
    if !dl.is_null() {
        crate::k5_plugin_h::krb5int_close_plugin(dl);
    }
    crate::k5_err_h::k5_clear_error(&mut errinfo);
}
/*
 * Determine the mechanism to use for a caller-specified mech OID.  For the
 * real mech OID of an interposed mech, return the interposed OID.  For an
 * interposed mech OID (which an interposer mech uses when re-entering the
 * mechglue), return the real mech OID.  The returned OID is an alias and
 * should not be modified or freed.
 */
#[no_mangle]

pub unsafe extern "C" fn gssint_select_mech_type(
    mut _minor: *mut crate::gssapi_h::OM_uint32,
    mut oid: crate::gssapi_h::gss_const_OID,
    mut selected_oid: *mut crate::gssapi_h::gss_OID,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut minfo = 0 as *mut crate::mglueP_h::gss_mech_config;
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    *selected_oid = 0 as crate::gssapi_h::gss_OID;
    if gssint_mechglue_initialize_library() != 0i32 {
        return (13u32) << 16i32;
    }
    k5_mutex_lock(&mut g_mechListLock);
    /* Read conf file at least once so that interposer plugins have a
     * chance of getting initialized. */
    initMechList();
    minfo = g_mechList;
    if oid.is_null() {
        oid = (*minfo).mech_type as crate::gssapi_h::gss_const_OID
    }
    loop {
        if minfo.is_null() {
            current_block = 6057473163062296781;
            break;
        }
        if (*(*minfo).mech_type).length == (*oid).length
            && crate::stdlib::memcmp(
                (*(*minfo).mech_type).elements,
                (*oid).elements,
                (*(*minfo).mech_type).length as usize,
            ) == 0i32
        {
            if !(*minfo).int_mech_type.is_null() {
                *selected_oid = (*minfo).int_mech_type
            } else {
                *selected_oid = (*minfo).mech_type
            }
            status = 0u32;
            current_block = 2349180396633407496;
            break;
        } else if !(*minfo).int_mech_type.is_null()
            && ((*(*minfo).int_mech_type).length == (*oid).length
                && crate::stdlib::memcmp(
                    (*(*minfo).int_mech_type).elements,
                    (*oid).elements,
                    (*(*minfo).int_mech_type).length as usize,
                ) == 0i32)
        {
            *selected_oid = (*minfo).mech_type;
            status = 0u32;
            current_block = 2349180396633407496;
            break;
        } else {
            minfo = (*minfo).next
        }
    }
    match current_block {
        6057473163062296781 => status = (1u32) << 16i32,
        _ => {}
    }
    k5_mutex_unlock(&mut g_mechListLock);
    return status;
}
/* If oid is an interposed OID, return the corresponding real mech OID.  If
 * it's a real mech OID, return it unmodified.  Otherwised return null. */
#[no_mangle]

pub unsafe extern "C" fn gssint_get_public_oid(
    mut oid: crate::gssapi_h::gss_const_OID,
) -> crate::gssapi_h::gss_OID {
    let mut minfo = 0 as *mut crate::mglueP_h::gss_mech_config;
    let mut public_oid = 0 as crate::gssapi_h::gss_OID;
    /* if oid is null -> then get default which is the first in the list */
    if oid.is_null() {
        return 0 as crate::gssapi_h::gss_OID;
    }
    if gssint_mechglue_initialize_library() != 0i32 {
        return 0 as crate::gssapi_h::gss_OID;
    }
    k5_mutex_lock(&mut g_mechListLock);
    minfo = g_mechList;
    while !minfo.is_null() {
        if !((*minfo).is_interposer != 0) {
            if (*(*minfo).mech_type).length == (*oid).length
                && crate::stdlib::memcmp(
                    (*(*minfo).mech_type).elements,
                    (*oid).elements,
                    (*(*minfo).mech_type).length as usize,
                ) == 0i32
                || !(*minfo).int_mech_type.is_null()
                    && ((*(*minfo).int_mech_type).length == (*oid).length
                        && crate::stdlib::memcmp(
                            (*(*minfo).int_mech_type).elements,
                            (*oid).elements,
                            (*(*minfo).int_mech_type).length as usize,
                        ) == 0i32)
            {
                public_oid = (*minfo).mech_type;
                break;
            }
        }
        minfo = (*minfo).next
    }
    k5_mutex_unlock(&mut g_mechListLock);
    return public_oid;
}
/* Translate a vector of oids (as from a union cred struct) into a set of
 * public OIDs using gssint_get_public_oid. */
#[no_mangle]

pub unsafe extern "C" fn gssint_make_public_oid_set(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut oids: crate::gssapi_h::gss_OID,
    mut count: i32,
    mut public_set: *mut crate::gssapi_h::gss_OID_set,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut set = 0 as *mut crate::gssapi_h::gss_OID_set_desc_struct;
    let mut public_oid = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut i: i32 = 0;
    *public_set = 0 as crate::gssapi_h::gss_OID_set;
    status = crate::src::generic::oid_ops::generic_gss_create_empty_oid_set(minor_status, &mut set);
    if status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
        return status;
    }
    i = 0i32;
    while i < count {
        public_oid = gssint_get_public_oid(&mut *oids.offset(i as isize)
            as *mut crate::gssapi_h::gss_OID_desc_struct
            as crate::gssapi_h::gss_const_OID);
        if !public_oid.is_null() {
            status = crate::src::generic::oid_ops::generic_gss_add_oid_set_member(
                minor_status,
                public_oid as *const crate::gssapi_h::gss_OID_desc,
                &mut set,
            );
            if status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
                crate::src::generic::rel_oid_set::generic_gss_release_oid_set(
                    &mut tmpmin,
                    &mut set,
                );
                return status;
            }
        }
        i += 1
    }
    *public_set = set;
    return 0u32;
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
/*
 * Register a mechanism.  Called with g_mechListLock held.
 */
/*
 * given the mechanism type, return the mechanism structure
 * containing the mechanism library entry points.
 * will return NULL if mech type is not found
 * This function will also trigger the loading of the mechanism
 * module if it has not been already loaded.
 */
#[no_mangle]

pub unsafe extern "C" fn gssint_get_mechanism(
    mut oid: crate::gssapi_h::gss_const_OID,
) -> crate::mglueP_h::gss_mechanism {
    let mut aMech = 0 as *mut crate::mglueP_h::gss_mech_config;
    let mut sym: Option<
        unsafe extern "C" fn(_: crate::gssapi_h::gss_OID) -> crate::mglueP_h::gss_mechanism,
    > = None;
    let mut dl = 0 as *mut crate::k5_plugin_h::plugin_file_handle;
    let mut errinfo = crate::k5_err_h::errinfo {
        code: 0,
        msg: 0 as *mut i8,
    };
    if gssint_mechglue_initialize_library() != 0i32 {
        return 0 as crate::mglueP_h::gss_mechanism;
    }
    k5_mutex_lock(&mut g_mechListLock);
    /* Check if the mechanism is already loaded. */
    aMech = g_mechList;
    if oid.is_null() {
        oid = (*aMech).mech_type as crate::gssapi_h::gss_const_OID
    }
    while !aMech.is_null() {
        if (*(*aMech).mech_type).length == (*oid).length
            && crate::stdlib::memcmp(
                (*(*aMech).mech_type).elements,
                (*oid).elements,
                (*(*aMech).mech_type).length as usize,
            ) == 0i32
            && !(*aMech).mech.is_null()
        {
            k5_mutex_unlock(&mut g_mechListLock);
            return (*aMech).mech;
        } else {
            if !(*aMech).int_mech_type.is_null()
                && ((*(*aMech).int_mech_type).length == (*oid).length
                    && crate::stdlib::memcmp(
                        (*(*aMech).int_mech_type).elements,
                        (*oid).elements,
                        (*(*aMech).int_mech_type).length as usize,
                    ) == 0i32)
            {
                k5_mutex_unlock(&mut g_mechListLock);
                return (*aMech).int_mech;
            }
        }
        aMech = (*aMech).next
    }
    /*
     * might need to re-read the configuration file before loading
     * the mechanism to ensure we have the latest info.
     */
    updateMechList();
    aMech = searchMechList(oid);
    /* is the mechanism present in the list ? */
    if aMech.is_null() {
        k5_mutex_unlock(&mut g_mechListLock);
        return 0 as crate::mglueP_h::gss_mechanism;
    }
    /* has another thread loaded the mech */
    if !(*aMech).mech.is_null() {
        k5_mutex_unlock(&mut g_mechListLock);
        return (*aMech).mech;
    }
    crate::stdlib::memset(
        &mut errinfo as *mut crate::k5_err_h::errinfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_err_h::errinfo>(),
    );
    if crate::k5_plugin_h::krb5int_open_plugin((*aMech).uLibName, &mut dl, &mut errinfo) != 0isize
        || errinfo.code != 0isize
    {
        crate::k5_err_h::k5_clear_error(&mut errinfo);
        k5_mutex_unlock(&mut g_mechListLock);
        return 0 as crate::mglueP_h::gss_mechanism;
    }
    if crate::k5_plugin_h::krb5int_get_plugin_func(
        dl,
        b"gss_mech_initialize\x00" as *const u8 as *const i8,
        &mut sym
            as *mut Option<
                unsafe extern "C" fn(_: crate::gssapi_h::gss_OID) -> crate::mglueP_h::gss_mechanism,
            > as *mut Option<unsafe extern "C" fn() -> ()>,
        &mut errinfo,
    ) == 0isize
    {
        /* Call the symbol to get the mechanism table */
        (*aMech).mech = Some(sym.expect("non-null function pointer"))
            .expect("non-null function pointer")((*aMech).mech_type)
    } else {
        /* Try dynamic dispatch table */
        crate::k5_err_h::k5_clear_error(&mut errinfo);
        (*aMech).mech = build_dynamicMech(dl as *mut libc::c_void, (*aMech).mech_type);
        (*aMech).freeMech = 1i32
    }
    if (*aMech).mech.is_null() {
        crate::k5_plugin_h::krb5int_close_plugin(dl);
        k5_mutex_unlock(&mut g_mechListLock);
        return 0 as crate::mglueP_h::gss_mechanism;
    }
    (*aMech).dl_handle = dl as *mut libc::c_void;
    k5_mutex_unlock(&mut g_mechListLock);
    return (*aMech).mech;
}
/* gssint_get_mechanism */
/*
 * this routine is used for searching the list of mechanism data.
 *
 * this needs to be called with g_mechListLock held.
 */

unsafe extern "C" fn searchMechList(
    mut oid: crate::gssapi_h::gss_const_OID,
) -> crate::mglueP_h::gss_mech_info {
    let mut aMech = g_mechList;
    /* if oid is null -> then get default which is the first in the list */
    if oid.is_null() {
        return aMech;
    }
    while !aMech.is_null() {
        if (*(*aMech).mech_type).length == (*oid).length
            && crate::stdlib::memcmp(
                (*(*aMech).mech_type).elements,
                (*oid).elements,
                (*(*aMech).mech_type).length as usize,
            ) == 0i32
        {
            return aMech;
        }
        aMech = (*aMech).next
    }
    /* none found */
    return 0 as crate::mglueP_h::gss_mech_info;
}
/* searchMechList */
/* Return the first non-whitespace character starting from str. */

unsafe extern "C" fn skip_whitespace(mut str: *mut i8) -> *mut i8 {
    while *(*crate::stdlib::__ctype_b_loc()).offset(*str as i32 as isize) as i32
        & crate::stdlib::_ISspace as u16 as i32
        != 0
    {
        str = str.offset(1)
    }
    return str;
}
/* Truncate str at the first whitespace character and return the first
 * non-whitespace character after that point. */

unsafe extern "C" fn delimit_ws(mut str: *mut i8) -> *mut i8 {
    while *str as i32 != '\u{0}' as i32
        && *(*crate::stdlib::__ctype_b_loc()).offset(*str as i32 as isize) as i32
            & crate::stdlib::_ISspace as u16 as i32
            == 0
    {
        str = str.offset(1)
    }
    if *str as i32 != '\u{0}' as i32 {
        let fresh1 = str;
        str = str.offset(1);
        *fresh1 = '\u{0}' as i8
    }
    return skip_whitespace(str);
}
/* Truncate str at the first occurrence of delimiter and return the first
 * non-whitespace character after that point. */

unsafe extern "C" fn delimit(mut str: *mut i8, mut delimiter: i8) -> *mut i8 {
    while *str as i32 != '\u{0}' as i32 && *str as i32 != delimiter as i32 {
        str = str.offset(1)
    }
    if *str as i32 != '\u{0}' as i32 {
        let fresh2 = str;
        str = str.offset(1);
        *fresh2 = '\u{0}' as i8
    }
    return skip_whitespace(str);
}
/*
 * loads the configuration file
 * this is called while having a mutex lock on the mechanism list
 * entries for libraries that have been loaded can't be modified
 * mechNameStr and mech_type fields are not updated during updates
 */

unsafe extern "C" fn loadConfigFile(mut fileName: *const i8) {
    let mut sharedLib = 0 as *mut i8; /* while */
    let mut kernMod = 0 as *mut i8;
    let mut modOptions = 0 as *mut i8;
    let mut modType = 0 as *mut i8;
    let mut oid = 0 as *mut i8;
    let mut next = 0 as *mut i8;
    let mut buffer: [i8; 8192] = [0; 8192];
    let mut oidStr = 0 as *mut i8;
    let mut confFile = 0 as *mut crate::stdlib::FILE;
    confFile = crate::stdlib::fopen(fileName, b"r\x00" as *const u8 as *const i8);
    if confFile.is_null() {
        return;
    }
    crate::stdlib::memset(
        buffer.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[i8; 8192]>(),
    );
    while !crate::stdlib::fgets(buffer.as_mut_ptr(), 8192i32, confFile).is_null() {
        /* ignore lines beginning with # */
        if *buffer.as_mut_ptr() as i32 == '#' as i32 {
            continue;
        }
        /* Parse out the name, oid, and shared library path. */
        oidStr = buffer.as_mut_ptr();
        oid = delimit_ws(oidStr);
        if *oid as i32 == '\u{0}' as i32 {
            continue;
        }
        sharedLib = delimit_ws(oid);
        if *sharedLib as i32 == '\u{0}' as i32 {
            continue;
        }
        next = delimit_ws(sharedLib);
        /* Parse out the kernel module name if present. */
        if *next as i32 != '\u{0}' as i32
            && *next as i32 != '[' as i32
            && *next as i32 != '<' as i32
        {
            kernMod = next;
            next = delimit_ws(kernMod)
        } else {
            kernMod = 0 as *mut i8
        }
        /* Parse out the module options if present. */
        if *next as i32 == '[' as i32 {
            modOptions = next.offset(1isize);
            next = delimit(modOptions, ']' as i8)
        } else {
            modOptions = 0 as *mut i8
        }
        /* Parse out the module type if present. */
        if *next as i32 == '<' as i32 {
            modType = next.offset(1isize);
            delimit(modType, '>' as i8);
        } else {
            modType = 0 as *mut i8
        }
        addConfigEntry(oidStr, oid, sharedLib, kernMod, modOptions, modType);
    }
    crate::stdlib::fclose(confFile);
}
/* Local functions */
/* loadConfigFile */

unsafe extern "C" fn addConfigEntry(
    mut oidStr: *const i8,
    mut oid: *const i8,
    mut sharedLib: *const i8,
    mut kernMod: *const i8,
    mut modOptions: *const i8,
    mut modType: *const i8,
) {
    let mut sharedPath: [i8; 8212] = [0; 8212];
    let mut tmpStr = 0 as *mut i8;
    let mut mechOid = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut aMech = 0 as *mut crate::mglueP_h::gss_mech_config;
    let mut tmp = 0 as *mut crate::mglueP_h::gss_mech_config;
    let mut minor: crate::gssapi_h::OM_uint32 = 0;
    let mut oidBuf = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    if oid.is_null() || oidStr.is_null() {
        return;
    }
    /*
     * check if an entry for this oid already exists
     * if it does, and the library is already loaded then
     * we can't modify it, so skip it
     */
    oidBuf.value = oid as *mut libc::c_void;
    oidBuf.length = crate::stdlib::strlen(oid);
    if crate::src::generic::oid_ops::generic_gss_str_to_oid(&mut minor, &mut oidBuf, &mut mechOid)
        != 0u32
    {
        return;
    }
    aMech = searchMechList(mechOid as crate::gssapi_h::gss_const_OID);
    if !aMech.is_null() && !(*aMech).mech.is_null() {
        crate::src::generic::oid_ops::generic_gss_release_oid(&mut minor, &mut mechOid);
        return;
    }
    /*
     * If that's all, then this is a corrupt entry. Skip it.
     */
    if *sharedLib == 0 {
        crate::src::generic::oid_ops::generic_gss_release_oid(&mut minor, &mut mechOid);
        return;
    }
    if *sharedLib.offset(0isize) as i32 == '/' as i32 {
        crate::stdlib::snprintf(
            sharedPath.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 8212]>(),
            b"%s\x00" as *const u8 as *const i8,
            sharedLib,
        );
    } else {
        crate::stdlib::snprintf(
            sharedPath.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 8212]>(),
            b"%s%s\x00" as *const u8 as *const i8,
            b"/usr/local/lib/gss/\x00" as *const u8 as *const i8,
            sharedLib,
        );
    }
    /*
     * are we creating a new mechanism entry or
     * just modifying existing (non loaded) mechanism entry
     */
    if !aMech.is_null() {
        /*
         * delete any old values and set new
         * mechNameStr and mech_type are not modified
         */
        if !(*aMech).kmodName.is_null() {
            crate::stdlib::free((*aMech).kmodName as *mut libc::c_void);
            (*aMech).kmodName = 0 as *mut i8
        }
        if !(*aMech).optionStr.is_null() {
            crate::stdlib::free((*aMech).optionStr as *mut libc::c_void);
            (*aMech).optionStr = 0 as *mut i8
        }
        tmpStr = crate::stdlib::strdup(sharedPath.as_mut_ptr());
        if !tmpStr.is_null() {
            if !(*aMech).uLibName.is_null() {
                crate::stdlib::free((*aMech).uLibName as *mut libc::c_void);
            }
            (*aMech).uLibName = tmpStr
        }
        if !kernMod.is_null() {
            /* this is an optional parameter */
            (*aMech).kmodName = crate::stdlib::strdup(kernMod)
        }
        if !modOptions.is_null() {
            /* optional module options */
            (*aMech).optionStr = crate::stdlib::strdup(modOptions)
        }
        /* the oid is already set */
        crate::src::generic::oid_ops::generic_gss_release_oid(&mut minor, &mut mechOid);
        return;
    }
    /* adding a new entry */
    aMech = crate::stdlib::calloc(
        1usize,
        ::std::mem::size_of::<crate::mglueP_h::gss_mech_config>(),
    ) as crate::mglueP_h::gss_mech_info;
    if aMech.is_null() {
        crate::src::generic::oid_ops::generic_gss_release_oid(&mut minor, &mut mechOid);
        return;
    }
    (*aMech).mech_type = mechOid;
    (*aMech).uLibName = crate::stdlib::strdup(sharedPath.as_mut_ptr());
    (*aMech).mechNameStr = crate::stdlib::strdup(oidStr);
    (*aMech).freeMech = 0i32;
    /* check if any memory allocations failed - bad news */
    if (*aMech).uLibName.is_null() || (*aMech).mechNameStr.is_null() {
        if !(*aMech).uLibName.is_null() {
            crate::stdlib::free((*aMech).uLibName as *mut libc::c_void);
        }
        if !(*aMech).mechNameStr.is_null() {
            crate::stdlib::free((*aMech).mechNameStr as *mut libc::c_void);
        }
        crate::src::generic::oid_ops::generic_gss_release_oid(&mut minor, &mut mechOid);
        crate::stdlib::free(aMech as *mut libc::c_void);
        return;
    }
    if !kernMod.is_null() {
        /* this is an optional parameter */
        (*aMech).kmodName = crate::stdlib::strdup(kernMod)
    }
    if !modOptions.is_null() {
        (*aMech).optionStr = crate::stdlib::strdup(modOptions)
    }
    if !modType.is_null()
        && crate::stdlib::strcmp(modType, b"interposer\x00" as *const u8 as *const i8) == 0i32
    {
        (*aMech).is_interposer = 1i32
    }
    /*
     * add the new entry to the end of the list - make sure
     * that only complete entries are added because other
     * threads might currently be searching the list.
     */
    tmp = g_mechListTail;
    g_mechListTail = aMech;
    if !tmp.is_null() {
        (*tmp).next = aMech
    }
    if g_mechList.is_null() {
        g_mechList = aMech
    };
}
