use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:15"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:15"]
pub mod types_h {
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = libc::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:15"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::__uint64_t;
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:15"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:15"]
pub mod thread_shared_types_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:16"]
    pub struct __pthread_internal_list {
        pub __prev: *mut __pthread_internal_list,
        pub __next: *mut __pthread_internal_list,
    }
    #[c2rust::src_loc = "82:1"]
    pub type __pthread_list_t = __pthread_internal_list;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:8"]
    pub struct __pthread_mutex_s {
        pub __lock: libc::c_int,
        pub __count: libc::c_uint,
        pub __owner: libc::c_int,
        pub __nusers: libc::c_uint,
        pub __kind: libc::c_int,
        pub __spins: libc::c_short,
        pub __elision: libc::c_short,
        pub __list: __pthread_list_t,
    }
}
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:15"]
pub mod pthreadtypes_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "67:9"]
    pub union pthread_mutex_t {
        pub __data: __pthread_mutex_s,
        pub __size: [libc::c_char; 40],
        pub __align: libc::c_long,
    }
    use super::thread_shared_types_h::__pthread_mutex_s;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:15"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:15"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:15"]
pub mod k5_thread_h {
    #[c2rust::src_loc = "283:1"]
    pub type k5_os_mutex = pthread_mutex_t;
    /* is pthreads always available? */
    #[c2rust::src_loc = "354:1"]
    pub type k5_mutex_t = k5_os_mutex;
    #[inline]
    #[c2rust::src_loc = "356:1"]
    pub unsafe extern "C" fn k5_mutex_init(mut m: *mut k5_mutex_t)
     -> libc::c_int {
        return k5_os_mutex_init(m);
    }
    #[inline]
    #[c2rust::src_loc = "367:1"]
    pub unsafe extern "C" fn k5_mutex_lock(mut m: *mut k5_mutex_t) {
        let mut r: libc::c_int = k5_os_mutex_lock(m);
        if r != 0 as libc::c_int {
            fprintf(stderr,
                    b"k5_mutex_lock: Received error %d (%s)\n\x00" as
                        *const u8 as *const libc::c_char, r, strerror(r));
        }
        if r == 0 as libc::c_int {
        } else {
            __assert_fail(b"r == 0\x00" as *const u8 as *const libc::c_char,
                          b"../../include/k5-thread.h\x00" as *const u8 as
                              *const libc::c_char,
                          376 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 33],
                                                    &[libc::c_char; 33]>(b"void k5_mutex_lock(k5_mutex_t *)\x00")).as_ptr());
        };
    }
    #[inline]
    #[c2rust::src_loc = "379:1"]
    pub unsafe extern "C" fn k5_mutex_unlock(mut m: *mut k5_mutex_t) {
        let mut r: libc::c_int = k5_os_mutex_unlock(m);
        if r != 0 as libc::c_int {
            fprintf(stderr,
                    b"k5_mutex_unlock: Received error %d (%s)\n\x00" as
                        *const u8 as *const libc::c_char, r, strerror(r));
        }
        if r == 0 as libc::c_int {
        } else {
            __assert_fail(b"r == 0\x00" as *const u8 as *const libc::c_char,
                          b"../../include/k5-thread.h\x00" as *const u8 as
                              *const libc::c_char,
                          388 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 35],
                                                    &[libc::c_char; 35]>(b"void k5_mutex_unlock(k5_mutex_t *)\x00")).as_ptr());
        };
    }
    use super::pthreadtypes_h::pthread_mutex_t;
    use super::stdio_h::{fprintf, stderr};
    use super::string_h::strerror;
    use super::assert_h::__assert_fail;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "290:1"]
        pub fn k5_os_mutex_init(m: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "292:1"]
        pub fn k5_os_mutex_lock(m: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "293:1"]
        pub fn k5_os_mutex_unlock(m: *mut k5_os_mutex) -> libc::c_int;
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:15"]
pub mod k5_plugin_h {
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2006 Massachusetts Institute of Technology.
 * All Rights Reserved.
 *
 * This software is being provided to you, the LICENSEE, by the
 * Massachusetts Institute of Technology (M.I.T.) under the following
 * license.  By obtaining, using and/or copying this software, you agree
 * that you have read, understood, and will comply with these terms and
 * conditions:
 *
 * Export of this software from the United States of America may
 * require a specific license from the United States Government.
 * It is the responsibility of any person or organization contemplating
 * export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify and distribute
 * this software and its documentation for any purpose and without fee or
 * royalty is hereby granted, provided that you agree to comply with the
 * following copyright notice and statements, including the disclaimer, and
 * that the same appear on ALL copies of the software and documentation,
 * including modifications that you make for internal use or for
 * distribution:
 *
 * THIS SOFTWARE IS PROVIDED "AS IS", AND M.I.T. MAKES NO REPRESENTATIONS
 * OR WARRANTIES, EXPRESS OR IMPLIED.  By way of example, but not
 * limitation, M.I.T. MAKES NO REPRESENTATIONS OR WARRANTIES OF
 * MERCHANTABILITY OR FITNESS FOR ANY PARTICULAR PURPOSE OR THAT THE USE OF
 * THE LICENSED SOFTWARE OR DOCUMENTATION WILL NOT INFRINGE ANY THIRD PARTY
 * PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
 *
 * The name of the Massachusetts Institute of Technology or M.I.T. may NOT
 * be used in advertising or publicity pertaining to distribution of the
 * software.  Title to copyright in this software and any associated
 * documentation shall at all times remain with M.I.T., and USER agrees to
 * preserve same.
 *
 * Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 */
        /* Just those definitions which are needed by util/support/plugins.c,
   which gets compiled before util/et is built, which happens before
   we can construct krb5.h, which is included by k5-int.h.

   So, no krb5 types.  */
        /*
 * Plugins normally export fixed symbol names, but when statically
 * linking plugins, we need a different symbol name for each plugin.
 * The first argument to PLUGIN_SYMBOL_NAME acts as the
 * differentiator, and is only used for static plugin linking.
 *
 * Although this macro (and thus this header file) are used in plugins
 * whose code lies inside the krb5 tree, plugins maintained separately
 * from the krb5 tree do not need it; they can just use the fixed
 * symbol name unconditionally.
 */
        #[c2rust::src_loc = "80:8"]
        pub type plugin_file_handle;
    }
    /* K5_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:15"]
pub mod com_err_h {
    /*
 * Copyright 1988, Student Information Processing Board of the
 * Massachusetts Institute of Technology.
 *
 * Copyright 1995 by Cygnus Support.
 *
 * For copyright and distribution info, see the documentation supplied
 * with this package.
 */
    /* Header file for common error description library. */
    #[c2rust::src_loc = "26:1"]
    pub type errcode_t = libc::c_long;
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/profile/prof_int.h:15"]
pub mod prof_int_h {
    /*
 * prof-int.h
 */
    /*
 * This is the structure which stores the profile information for a
 * particular configuration file.
 *
 * Locking strategy:
 * - filespec, fslen are fixed after creation
 * - refcount and next should only be tweaked with the global lock held
 * - other fields can be tweaked after grabbing the in-struct lock
 */
    /* time tree was last updated from file */
    /* fractional part of timestamp, if any */
    /* r/w, dirty */
    /* incremented when data changes */
    /* Some separation between fields controlled by different
	   mutexes.  Theoretically, both could be accessed at the same
	   time from different threads on different CPUs with separate
	   caches.  Don't let the threads clobber each other's
	   changes.  One mutex controlling the whole thing would be
	   better, but sufficient separation might suffice.

	   This is icky.  I just hope it's adequate.

	   For next major release, fix this.  */
    /* prf_file_t references */
    /* Was: "profile_filespec_t filespec".  Now: flexible char
	   array ... except, we need to work in C89, so an array
	   length must be specified.  */
    /*
 * The profile flags
 */
    /*
 * This structure defines the high-level, user visible profile_t
 * object, which is used as a handle by users who need to query some
 * configuration file(s)
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "93:8"]
    pub struct _profile_t {
        pub magic: prf_magic_t,
        pub first_file: prf_file_t,
        pub vt: *mut profile_vtable,
        pub cbdata: *mut libc::c_void,
        pub lib_handle: prf_lib_handle_t,
    }
    #[c2rust::src_loc = "86:1"]
    pub type prf_lib_handle_t = *mut _prf_lib_handle_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "80:8"]
    pub struct _prf_lib_handle_t {
        pub lock: k5_mutex_t,
        pub refcount: libc::c_int,
        pub plugin_handle: *mut plugin_file_handle,
    }
    #[c2rust::src_loc = "71:1"]
    pub type prf_file_t = *mut _prf_file_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "65:8"]
    pub struct _prf_file_t {
        pub magic: prf_magic_t,
        pub data: *mut _prf_data_t,
        pub next: *mut _prf_file_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "30:8"]
    pub struct _prf_data_t {
        pub magic: prf_magic_t,
        pub lock: k5_mutex_t,
        pub root: *mut profile_node,
        pub last_stat: time_t,
        pub timestamp: time_t,
        pub frac_ts: libc::c_ulong,
        pub flags: libc::c_int,
        pub upd_serial: libc::c_int,
        pub fslen: size_t,
        pub pad: C2RustUnnamed,
        pub refcount: libc::c_int,
        pub next: *mut _prf_data_t,
        pub filespec: [libc::c_char; 15],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:2"]
    pub union C2RustUnnamed {
        pub d: libc::c_double,
        pub p: *mut libc::c_void,
        pub ll: uint64_t,
        pub m: k5_mutex_t,
    }
    #[c2rust::src_loc = "19:1"]
    pub type prf_magic_t = libc::c_long;
    #[c2rust::src_loc = "62:1"]
    pub type prf_data_t = *mut _prf_data_t;
    use super::profile_h::profile_vtable;
    use super::k5_thread_h::k5_mutex_t;
    use super::k5_plugin_h::plugin_file_handle;
    use super::time_t_h::time_t;
    use super::stddef_h::size_t;
    use super::stdint_uintn_h::uint64_t;
    use super::com_err_h::errcode_t;
    extern "C" {
        #[c2rust::src_loc = "33:9"]
        pub type profile_node;
        /*
 * Used by the profile iterator in prof_get.c
 */
        /*
 * Check if a filespec is last in a list (NULL on UNIX, invalid FSSpec on MacOS
 */
        /* profile_parse.c */
        /* prof_tree.c */
        #[no_mangle]
        #[c2rust::src_loc = "200:1"]
        pub fn profile_remove_node(node: *mut profile_node) -> errcode_t;
        #[no_mangle]
        #[c2rust::src_loc = "203:1"]
        pub fn profile_set_relation_value(node: *mut profile_node,
                                          new_value: *const libc::c_char)
         -> errcode_t;
        #[no_mangle]
        #[c2rust::src_loc = "162:1"]
        pub fn profile_find_node(section: *mut profile_node,
                                 name: *const libc::c_char,
                                 value: *const libc::c_char,
                                 section_flag: libc::c_int,
                                 state: *mut *mut libc::c_void,
                                 node: *mut *mut profile_node) -> errcode_t;
        /* prof_file.c */
        #[no_mangle]
        #[c2rust::src_loc = "218:1"]
        pub fn profile_update_file_data(profile: prf_data_t,
                                        ret_modspec: *mut *mut libc::c_char)
         -> errcode_t;
        #[no_mangle]
        #[c2rust::src_loc = "248:1"]
        pub fn profile_unlock_global();
        #[no_mangle]
        #[c2rust::src_loc = "245:1"]
        pub fn profile_dereference_data_locked(_: prf_data_t);
        #[no_mangle]
        #[c2rust::src_loc = "63:1"]
        pub fn profile_make_prf_data(_: *const libc::c_char) -> prf_data_t;
        #[no_mangle]
        #[c2rust::src_loc = "247:1"]
        pub fn profile_lock_global();
        #[no_mangle]
        #[c2rust::src_loc = "206:1"]
        pub fn profile_rename_node(node: *mut profile_node,
                                   new_name: *const libc::c_char)
         -> errcode_t;
        #[no_mangle]
        #[c2rust::src_loc = "145:1"]
        pub fn profile_add_node(section: *mut profile_node,
                                name: *const libc::c_char,
                                value: *const libc::c_char,
                                ret_node: *mut *mut profile_node)
         -> errcode_t;
    }
    /* prof_set.c -- included from profile.h */
    /* Others included from profile.h */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/profile/profile.h:15"]
pub mod profile_h {
    /*
 * profile.h
 */
    /* Used by profile_init_flags(). */
    /* Allow module declaration */
    /*
 * Used by the profile iterator in prof_get.c
 */
    /* __cplusplus */
    /* path as C string */
    /* list of : separated paths, C string */
    /* path as C string */
    /* list of : separated paths, C string */
    /*
 * profile_init_vtable allows a caller to create a profile-compatible object
 * with a different back end.
 */
    /*
 * Mandatory: Look up all of the relations for names, placing the resulting
 * values in *ret_values.  If no relations exist, return PROF_NO_RELATION, or
 * PROF_NO_SECTION to indicate that one of the intermediate names does not
 * exist as a section.  The list will be freed with free_values.
 */
    /* Mandatory: Free a list of strings returned by get_values. */
    /* Optional: Release any data associated with the profile. */
    /*
 * Optional (mandatory if cleanup is defined): Generate a new cbdata pointer
 * for a copy of the profile.  If not implemented, the new profile will receive
 * the same cbdata pointer as the old one.
 */
    /*
 * Optional: Create an iterator handle.
 *
 * If flags contains PROFILE_ITER_LIST_SECTION, iterate over all of the
 * relations and sections within names.  Otherwise, iterate over the relation
 * values for names, or produce a single section result if names is a section.
 *
 * If flags contains PROFILE_ITER_SECTIONS_ONLY, produce only sections.
 *
 * If flags contains PROFILE_ITER_RELATIONS_ONLY, produce only relations.
 */
    /*
 * Optional (mandatory if iterator_create is defined): Produce the next
 * relation or section in an iteration.  If producing a section result, set
 * *ret_value to NULL.  The returned strings will be freed with free_string.
 */
    /*
 * Optional (mandatory if iterator_create is defined): Free the memory for an
 * iterator.
 */
    /* Optional (mandatory if iterator is defined): Free a string value. */
    /*
 * Optional: Determine if a profile is writable.  If not implemented, the
 * profile is never writable.
 */
    /*
 * Optional: Determine if a profile is modified in memory relative to the
 * persistent store.  If not implemented, the profile is assumed to never be
 * modified.
 */
    /*
 * Optional: Change the value of a relation, or remove it if new_value is NULL.
 * If old_value is set and the relation does not have that value, return
 * PROF_NO_RELATION.
 */
    /*
 * Optional: Rename a section to new_name, or remove the section if new_name is
 * NULL.
 */
    /*
 * Optional: Add a new relation, or a new section if new_value is NULL.  Add
 * any intermediate sections as necessary.
 */
    /*
 * Optional: Flush any pending memory updates to the persistent store.  If
 * implemented, this function will be called by profile_release as well as
 * profile_flush, so make sure it's not inefficient to flush an unmodified
 * profile.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "242:8"]
    pub struct profile_vtable {
        pub minor_ver: libc::c_int,
        pub get_values: profile_get_values_fn,
        pub free_values: profile_free_values_fn,
        pub cleanup: profile_cleanup_fn,
        pub copy: profile_copy_fn,
        pub iterator_create: profile_iterator_create_fn,
        pub iterator: profile_iterator_fn,
        pub iterator_free: profile_iterator_free_fn,
        pub free_string: profile_free_string_fn,
        pub writable: profile_writable_fn,
        pub modified: profile_modified_fn,
        pub update_relation: profile_update_relation_fn,
        pub rename_section: profile_rename_section_fn,
        pub add_relation: profile_add_relation_fn,
        pub flush: profile_flush_fn,
    }
    #[c2rust::src_loc = "239:1"]
    pub type profile_flush_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_long>;
    #[c2rust::src_loc = "229:1"]
    pub type profile_add_relation_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut *const libc::c_char,
                                    _: *const libc::c_char) -> libc::c_long>;
    #[c2rust::src_loc = "221:1"]
    pub type profile_rename_section_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut *const libc::c_char,
                                    _: *const libc::c_char) -> libc::c_long>;
    #[c2rust::src_loc = "213:1"]
    pub type profile_update_relation_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut *const libc::c_char,
                                    _: *const libc::c_char,
                                    _: *const libc::c_char) -> libc::c_long>;
    #[c2rust::src_loc = "205:1"]
    pub type profile_modified_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_int)
                   -> libc::c_long>;
    #[c2rust::src_loc = "197:1"]
    pub type profile_writable_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_int)
                   -> libc::c_long>;
    #[c2rust::src_loc = "190:1"]
    pub type profile_free_string_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut libc::c_char) -> ()>;
    #[c2rust::src_loc = "186:1"]
    pub type profile_iterator_free_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "178:1"]
    pub type profile_iterator_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut libc::c_void,
                                    _: *mut *mut libc::c_char,
                                    _: *mut *mut libc::c_char)
                   -> libc::c_long>;
    #[c2rust::src_loc = "169:1"]
    pub type profile_iterator_create_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *const *const libc::c_char,
                                    _: libc::c_int, _: *mut *mut libc::c_void)
                   -> libc::c_long>;
    #[c2rust::src_loc = "155:1"]
    pub type profile_copy_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut *mut libc::c_void)
                   -> libc::c_long>;
    #[c2rust::src_loc = "147:1"]
    pub type profile_cleanup_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "143:1"]
    pub type profile_free_values_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut *mut libc::c_char) -> ()>;
    #[c2rust::src_loc = "138:1"]
    pub type profile_get_values_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *const *const libc::c_char,
                                    _: *mut *mut *mut libc::c_char)
                   -> libc::c_long>;
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::prof_int_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/string.h:15"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "396:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:15"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:15"]
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
#[c2rust::header_src = "/usr/include/assert.h:15"]
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
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint64_t, __off_t, __off64_t, __time_t};
pub use self::stdint_uintn_h::uint64_t;
pub use self::time_t_h::time_t;
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::pthread_mutex_t;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::k5_thread_h::{k5_os_mutex, k5_mutex_t, k5_mutex_init,
                            k5_mutex_lock, k5_mutex_unlock, k5_os_mutex_init,
                            k5_os_mutex_lock, k5_os_mutex_unlock};
use self::k5_plugin_h::plugin_file_handle;
pub use self::com_err_h::errcode_t;
pub use self::prof_int_h::{_profile_t, prf_lib_handle_t, _prf_lib_handle_t,
                           prf_file_t, _prf_file_t, _prf_data_t,
                           C2RustUnnamed, prf_magic_t, prf_data_t,
                           profile_node, profile_remove_node,
                           profile_set_relation_value, profile_find_node,
                           profile_update_file_data, profile_unlock_global,
                           profile_dereference_data_locked,
                           profile_make_prf_data, profile_lock_global,
                           profile_rename_node, profile_add_node};
pub use self::profile_h::{profile_vtable, profile_flush_fn,
                          profile_add_relation_fn, profile_rename_section_fn,
                          profile_update_relation_fn, profile_modified_fn,
                          profile_writable_fn, profile_free_string_fn,
                          profile_iterator_free_fn, profile_iterator_fn,
                          profile_iterator_create_fn, profile_copy_fn,
                          profile_cleanup_fn, profile_free_values_fn,
                          profile_get_values_fn, profile_t};
use self::string_h::strerror;
use self::stdlib_h::free;
use self::stdio_h::{stderr, fprintf};
use self::assert_h::__assert_fail;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * prof_set.c --- routines that expose the public interfaces for
 *      inserting, updating and deleting items from the profile.
 *
 * WARNING: These routines only look at the first file opened in the
 * profile.  It's not clear how to handle multiple files, actually.
 * In the future it may be necessary to modify this public interface,
 * or possibly add higher level functions to support this correctly.
 *
 * WARNING: We're not yet doing locking yet, either.
 *
 */
#[c2rust::src_loc = "24:1"]
unsafe extern "C" fn rw_setup(mut profile: profile_t) -> errcode_t {
    let mut file: prf_file_t = 0 as *mut _prf_file_t;
    let mut retval: errcode_t = 0 as libc::c_int as errcode_t;
    if profile.is_null() { return -(1429577704 as libc::c_long) }
    if (*profile).magic != -(1429577710 as libc::c_long) {
        return -(1429577710 as libc::c_long)
    }
    file = (*profile).first_file;
    profile_lock_global();
    /* Don't update the file if we've already made modifications */
    if (*(*file).data).flags & 0x2 as libc::c_int != 0 {
        profile_unlock_global();
        return 0 as libc::c_int as errcode_t
    }
    if (*(*file).data).flags & 0x4 as libc::c_int != 0 as libc::c_int {
        let mut new_data: prf_data_t = 0 as *mut _prf_data_t;
        new_data = profile_make_prf_data((*(*file).data).filespec.as_ptr());
        if new_data.is_null() {
            retval = 12 as libc::c_int as errcode_t
        } else {
            retval = k5_mutex_init(&mut (*new_data).lock) as errcode_t;
            if retval == 0 as libc::c_int as libc::c_long {
                (*new_data).root = 0 as *mut profile_node;
                (*new_data).flags =
                    (*(*file).data).flags & !(0x4 as libc::c_int);
                (*new_data).timestamp = 0 as libc::c_int as time_t;
                (*new_data).upd_serial = (*(*file).data).upd_serial
            }
        }
        if retval != 0 as libc::c_int as libc::c_long {
            profile_unlock_global();
            free(new_data as *mut libc::c_void);
            return retval
        }
        profile_dereference_data_locked((*file).data);
        (*file).data = new_data
    }
    profile_unlock_global();
    retval =
        profile_update_file_data((*file).data, 0 as *mut *mut libc::c_char);
    return retval;
}
/*
 * Delete or update a particular child node
 *
 * ADL - 2/23/99, rewritten TYT 2/25/99
 */
#[no_mangle]
#[c2rust::src_loc = "81:1"]
pub unsafe extern "C" fn profile_update_relation(mut profile: profile_t,
                                                 mut names:
                                                     *mut *const libc::c_char,
                                                 mut old_value:
                                                     *const libc::c_char,
                                                 mut new_value:
                                                     *const libc::c_char)
 -> libc::c_long {
    let mut retval: errcode_t = 0;
    let mut section: *mut profile_node = 0 as *mut profile_node;
    let mut node: *mut profile_node = 0 as *mut profile_node;
    let mut state: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut cpp: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    if !(*profile).vt.is_null() {
        if (*(*profile).vt).update_relation.is_none() {
            return -(1429577695 as libc::c_long)
        }
        return (*(*profile).vt).update_relation.expect("non-null function pointer")((*profile).cbdata,
                                                                                    names,
                                                                                    old_value,
                                                                                    new_value)
    }
    retval = rw_setup(profile);
    if retval != 0 { return retval }
    if names.is_null() || (*names.offset(0 as libc::c_int as isize)).is_null()
           || (*names.offset(1 as libc::c_int as isize)).is_null() {
        return -(1429577705 as libc::c_long)
    }
    if old_value.is_null() || *old_value == 0 {
        return -(1429577717 as libc::c_long)
    }
    k5_mutex_lock(&mut (*(*(*profile).first_file).data).lock);
    section = (*(*(*profile).first_file).data).root;
    cpp = names;
    while !(*cpp.offset(1 as libc::c_int as isize)).is_null() {
        state = 0 as *mut libc::c_void;
        retval =
            profile_find_node(section, *cpp, 0 as *const libc::c_char,
                              1 as libc::c_int, &mut state, &mut section);
        if retval != 0 {
            k5_mutex_unlock(&mut (*(*(*profile).first_file).data).lock);
            return retval
        }
        cpp = cpp.offset(1)
    }
    state = 0 as *mut libc::c_void;
    retval =
        profile_find_node(section, *cpp, old_value, 0 as libc::c_int,
                          &mut state, &mut node);
    if retval == 0 as libc::c_int as libc::c_long {
        if !new_value.is_null() {
            retval = profile_set_relation_value(node, new_value)
        } else { retval = profile_remove_node(node) }
    }
    if retval == 0 as libc::c_int as libc::c_long {
        (*(*(*profile).first_file).data).flags |= 0x2 as libc::c_int
    }
    k5_mutex_unlock(&mut (*(*(*profile).first_file).data).lock);
    return retval;
}
/*
 * Clear a particular all of the relations with a specific name.
 *
 * TYT - 2/25/99
 */
#[no_mangle]
#[c2rust::src_loc = "139:1"]
pub unsafe extern "C" fn profile_clear_relation(mut profile: profile_t,
                                                mut names:
                                                    *mut *const libc::c_char)
 -> libc::c_long {
    let mut retval: errcode_t = 0;
    let mut section: *mut profile_node = 0 as *mut profile_node;
    let mut node: *mut profile_node = 0 as *mut profile_node;
    let mut state: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut cpp: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    if !(*profile).vt.is_null() {
        if (*(*profile).vt).update_relation.is_none() {
            return -(1429577695 as libc::c_long)
        }
        return (*(*profile).vt).update_relation.expect("non-null function pointer")((*profile).cbdata,
                                                                                    names,
                                                                                    0
                                                                                        as
                                                                                        *const libc::c_char,
                                                                                    0
                                                                                        as
                                                                                        *const libc::c_char)
    }
    retval = rw_setup(profile);
    if retval != 0 { return retval }
    if names.is_null() || (*names.offset(0 as libc::c_int as isize)).is_null()
           || (*names.offset(1 as libc::c_int as isize)).is_null() {
        return -(1429577705 as libc::c_long)
    }
    section = (*(*(*profile).first_file).data).root;
    cpp = names;
    while !(*cpp.offset(1 as libc::c_int as isize)).is_null() {
        state = 0 as *mut libc::c_void;
        retval =
            profile_find_node(section, *cpp, 0 as *const libc::c_char,
                              1 as libc::c_int, &mut state, &mut section);
        if retval != 0 { return retval }
        cpp = cpp.offset(1)
    }
    state = 0 as *mut libc::c_void;
    loop  {
        retval =
            profile_find_node(section, *cpp, 0 as *const libc::c_char,
                              0 as libc::c_int, &mut state, &mut node);
        if retval != 0 { return retval }
        retval = profile_remove_node(node);
        if retval != 0 { return retval }
        if state.is_null() { break ; }
    }
    (*(*(*profile).first_file).data).flags |= 0x2 as libc::c_int;
    return 0 as libc::c_int as libc::c_long;
}
/*
 * Rename a particular section; if the new_section name is NULL,
 * delete it.
 *
 * ADL - 2/23/99, rewritten TYT 2/25/99
 */
#[no_mangle]
#[c2rust::src_loc = "191:1"]
pub unsafe extern "C" fn profile_rename_section(mut profile: profile_t,
                                                mut names:
                                                    *mut *const libc::c_char,
                                                mut new_name:
                                                    *const libc::c_char)
 -> libc::c_long {
    let mut retval: errcode_t = 0;
    let mut section: *mut profile_node = 0 as *mut profile_node;
    let mut node: *mut profile_node = 0 as *mut profile_node;
    let mut state: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut cpp: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    if !(*profile).vt.is_null() {
        if (*(*profile).vt).rename_section.is_none() {
            return -(1429577695 as libc::c_long)
        }
        return (*(*profile).vt).rename_section.expect("non-null function pointer")((*profile).cbdata,
                                                                                   names,
                                                                                   new_name)
    }
    retval = rw_setup(profile);
    if retval != 0 { return retval }
    if names.is_null() || (*names.offset(0 as libc::c_int as isize)).is_null()
       {
        return -(1429577705 as libc::c_long)
    }
    k5_mutex_lock(&mut (*(*(*profile).first_file).data).lock);
    section = (*(*(*profile).first_file).data).root;
    cpp = names;
    while !(*cpp.offset(1 as libc::c_int as isize)).is_null() {
        state = 0 as *mut libc::c_void;
        retval =
            profile_find_node(section, *cpp, 0 as *const libc::c_char,
                              1 as libc::c_int, &mut state, &mut section);
        if retval != 0 {
            k5_mutex_unlock(&mut (*(*(*profile).first_file).data).lock);
            return retval
        }
        cpp = cpp.offset(1)
    }
    state = 0 as *mut libc::c_void;
    retval =
        profile_find_node(section, *cpp, 0 as *const libc::c_char,
                          1 as libc::c_int, &mut state, &mut node);
    if retval == 0 as libc::c_int as libc::c_long {
        if !new_name.is_null() {
            retval = profile_rename_node(node, new_name)
        } else { retval = profile_remove_node(node) }
    }
    if retval == 0 as libc::c_int as libc::c_long {
        (*(*(*profile).first_file).data).flags |= 0x2 as libc::c_int
    }
    k5_mutex_unlock(&mut (*(*(*profile).first_file).data).lock);
    return retval;
}
/*
 * Insert a new relation.  If the new_value argument is NULL, then
 * create a new section instead.
 *
 * Note: if the intermediate sections do not exist, this function will
 * automatically create them.
 *
 * ADL - 2/23/99, rewritten TYT 2/25/99
 */
#[no_mangle]
#[c2rust::src_loc = "248:1"]
pub unsafe extern "C" fn profile_add_relation(mut profile: profile_t,
                                              mut names:
                                                  *mut *const libc::c_char,
                                              mut new_value:
                                                  *const libc::c_char)
 -> libc::c_long {
    let mut retval: errcode_t = 0;
    let mut section: *mut profile_node = 0 as *mut profile_node;
    let mut cpp: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut state: *mut libc::c_void = 0 as *mut libc::c_void;
    if !(*profile).vt.is_null() {
        if (*(*profile).vt).add_relation.is_none() {
            return -(1429577695 as libc::c_long)
        }
        return (*(*profile).vt).add_relation.expect("non-null function pointer")((*profile).cbdata,
                                                                                 names,
                                                                                 new_value)
    }
    retval = rw_setup(profile);
    if retval != 0 { return retval }
    /* Require at least two names for a new relation, one for a new section. */
    if names.is_null() || (*names.offset(0 as libc::c_int as isize)).is_null()
           ||
           (*names.offset(1 as libc::c_int as isize)).is_null() &&
               !new_value.is_null() {
        return -(1429577705 as libc::c_long)
    }
    k5_mutex_lock(&mut (*(*(*profile).first_file).data).lock);
    section = (*(*(*profile).first_file).data).root;
    cpp = names;
    while !(*cpp.offset(1 as libc::c_int as isize)).is_null() {
        state = 0 as *mut libc::c_void;
        retval =
            profile_find_node(section, *cpp, 0 as *const libc::c_char,
                              1 as libc::c_int, &mut state, &mut section);
        if retval == -(1429577726 as libc::c_long) {
            retval =
                profile_add_node(section, *cpp, 0 as *const libc::c_char,
                                 &mut section)
        }
        if retval != 0 {
            k5_mutex_unlock(&mut (*(*(*profile).first_file).data).lock);
            return retval
        }
        cpp = cpp.offset(1)
    }
    if new_value.is_null() {
        state = 0 as *mut libc::c_void;
        retval =
            profile_find_node(section, *cpp, 0 as *const libc::c_char,
                              1 as libc::c_int, &mut state,
                              0 as *mut *mut profile_node);
        if retval == 0 as libc::c_int as libc::c_long {
            k5_mutex_unlock(&mut (*(*(*profile).first_file).data).lock);
            return -(1429577701 as libc::c_long)
        } else {
            if retval != -(1429577726 as libc::c_long) {
                k5_mutex_unlock(&mut (*(*(*profile).first_file).data).lock);
                return retval
            }
        }
    }
    retval =
        profile_add_node(section, *cpp, new_value,
                         0 as *mut *mut profile_node);
    if retval != 0 {
        k5_mutex_unlock(&mut (*(*(*profile).first_file).data).lock);
        return retval
    }
    (*(*(*profile).first_file).data).flags |= 0x2 as libc::c_int;
    k5_mutex_unlock(&mut (*(*(*profile).first_file).data).lock);
    return 0 as libc::c_int as libc::c_long;
}
