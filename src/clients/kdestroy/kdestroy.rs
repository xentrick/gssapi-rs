use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:27"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:27"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:27"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:27"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:28"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    /* *
 * Used to convey an operation status.  The value 0 indicates success; any
 * other values are com_err codes.  Use krb5_get_error_message() to obtain a
 * string describing the error.
 */
    #[c2rust::src_loc = "204:1"]
    pub type krb5_error_code = krb5_int32;
    #[c2rust::src_loc = "206:1"]
    pub type krb5_magic = krb5_error_code;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "208:16"]
    pub struct _krb5_data {
        pub magic: krb5_magic,
        pub length: libc::c_uint,
        pub data: *mut libc::c_char,
    }
    #[c2rust::src_loc = "208:1"]
    pub type krb5_data = _krb5_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "228:16"]
    pub struct krb5_principal_data {
        pub magic: krb5_magic,
        pub realm: krb5_data,
        pub data: *mut krb5_data,
        pub length: krb5_int32,
        pub type_0: krb5_int32,
    }
    #[c2rust::src_loc = "236:1"]
    pub type krb5_principal = *mut krb5_principal_data;
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
    /* * Cursor for iterating over all ccaches */
    #[c2rust::src_loc = "2287:1"]
    pub type krb5_cccol_cursor = *mut _krb5_cccol_cursor;
    use super::stdint_intn_h::int32_t;
    extern "C" {
        /* per Kerberos v5 protocol spec */
        /* not yet in the spec... */
        /* macros to determine if a type is a local type */
        /*
 * end "hostaddr.h"
 */
        #[c2rust::src_loc = "350:8"]
        pub type _krb5_context;
        #[c2rust::src_loc = "2280:8"]
        pub type _krb5_ccache;
        #[c2rust::src_loc = "2285:8"]
        pub type _krb5_cccol_cursor;
        /* *
 * Retrieve the full name of a credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  cache           Credential cache handle
 * @param [out] fullname_out    Full name of cache
 *
 * Use krb5_free_string() to free @a fullname_out when it is no longer needed.
 *
 * @version New in 1.10
 */
        #[no_mangle]
        #[c2rust::src_loc = "2344:1"]
        pub fn krb5_cc_get_full_name(context: krb5_context,
                                     cache: krb5_ccache,
                                     fullname_out: *mut *mut libc::c_char)
         -> krb5_error_code;
        /* *
 * Destroy a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * This function destroys any existing contents of @a cache and closes the
 * handle to it.
 *
 * @retval
 * 0  Success
 * @return
 * Permission errors
 */
        #[no_mangle]
        #[c2rust::src_loc = "2386:1"]
        pub fn krb5_cc_destroy(context: krb5_context, cache: krb5_ccache)
         -> krb5_error_code;
        /* *
 * Close a credential cache handle.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * This function closes a credential cache handle @a cache without affecting
 * the contents of the cache.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2403:1"]
        pub fn krb5_cc_close(context: krb5_context, cache: krb5_ccache)
         -> krb5_error_code;
        /* *
 * Prepare to iterate over the collection of known credential caches.
 *
 * @param [in]  context         Library context
 * @param [out] cursor          Cursor
 *
 * Get a new cache iteration @a cursor that will iterate over all known
 * credential caches independent of type.
 *
 * Use krb5_cccol_cursor_free() to release @a cursor when it is no longer
 * needed.
 *
 * @sa krb5_cccol_cursor_next()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2635:1"]
        pub fn krb5_cccol_cursor_new(context: krb5_context,
                                     cursor: *mut krb5_cccol_cursor)
         -> krb5_error_code;
        /* *
 * Get the next credential cache in the collection.
 *
 * @param [in]  context         Library context
 * @param [in]  cursor          Cursor
 * @param [out] ccache          Credential cache handle
 *
 * @note When all caches are iterated over and the end of the list is reached,
 * @a ccache is set to NULL.
 *
 * Use krb5_cc_close() to close @a ccache when it is no longer needed.
 *
 * @sa krb5_cccol_cursor_new(), krb5_cccol_cursor_free()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2654:1"]
        pub fn krb5_cccol_cursor_next(context: krb5_context,
                                      cursor: krb5_cccol_cursor,
                                      ccache: *mut krb5_ccache)
         -> krb5_error_code;
        /* *
 * Free a credential cache collection cursor.
 *
 * @param [in] context          Library context
 * @param [in] cursor           Cursor
 *
 * @sa krb5_cccol_cursor_new(), krb5_cccol_cursor_next()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2668:1"]
        pub fn krb5_cccol_cursor_free(context: krb5_context,
                                      cursor: *mut krb5_cccol_cursor)
         -> krb5_error_code;
        /*
 * end "keytab.h"
 */
        /*
 * begin "func-proto.h"
 */
        /* *< Use secure context configuration */
        /* *< Use KDC configuration if available */
        /* *
 * Create a krb5 library context.
 *
 * @param [out] context         Library context
 *
 * The @a context must be released by calling krb5_free_context() when
 * it is no longer needed.
 *
 * @warning Any program or module that needs the Kerberos code to not trust the
 * environment must use krb5_init_secure_context(), or clean out the
 * environment.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2922:1"]
        pub fn krb5_init_context(context: *mut krb5_context)
         -> krb5_error_code;
        /* *
 * Free a krb5 library context.
 *
 * @param [in] context          Library context
 *
 * This function frees a @a context that was created by krb5_init_context()
 * or krb5_init_secure_context().
 */
        #[no_mangle]
        #[c2rust::src_loc = "2972:1"]
        pub fn krb5_free_context(context: krb5_context);
        /* *
 * Convert a string principal name to a krb5_principal structure.
 *
 * @param [in]  context         Library context
 * @param [in]  name            String representation of a principal name
 * @param [out] principal_out   New principal
 *
 * Convert a string representation of a principal name to a krb5_principal
 * structure.
 *
 * A string representation of a Kerberos name consists of one or more principal
 * name components, separated by slashes, optionally followed by the \@
 * character and a realm name.  If the realm name is not specified, the local
 * realm is used.
 *
 * To use the slash and \@ symbols as part of a component (quoted) instead of
 * using them as a component separator or as a realm prefix), put a backslash
 * (\) character in front of the symbol.  Similarly, newline, tab, backspace,
 * and NULL characters can be included in a component by using @c n, @c t, @c b
 * or @c 0, respectively.
 *
 * @note The realm in a Kerberos @a name cannot contain slash, colon,
 * or NULL characters.
 *
 * Use krb5_free_principal() to free @a principal_out when it is no longer
 * needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3427:1"]
        pub fn krb5_parse_name(context: krb5_context,
                               name: *const libc::c_char,
                               principal_out: *mut krb5_principal)
         -> krb5_error_code;
        /* *
 * Set the default credential cache name.
 *
 * @param [in] context          Library context
 * @param [in] name             Default credential cache name or NULL
 *
 * Set the default credential cache name to @a name for future operations using
 * @a context.  If @a name is NULL, clear any previous application-set default
 * name and forget any cached value of the default name for @a context.
 *
 * Calls to this function invalidate the result of any previous calls to
 * krb5_cc_default_name() using @a context.
 *
 * @retval
 *  0  Success
 * @retval
 *  KV5M_CONTEXT          Bad magic number for @c _krb5_context structure
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4403:1"]
        pub fn krb5_cc_set_default_name(context: krb5_context,
                                        name: *const libc::c_char)
         -> krb5_error_code;
        /* *
 * Resolve the default credential cache name.
 *
 * @param [in]  context         Library context
 * @param [out] ccache          Pointer to credential cache name
 *
 * Create a handle to the default credential cache as given by
 * krb5_cc_default_name().
 *
 * @retval
 * 0  Success
 * @retval
 * KV5M_CONTEXT            Bad magic number for @c _krb5_context structure
 * @retval
 * KRB5_FCC_INTERNAL       The name of the default credential cache cannot be
 *                         obtained
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4425:1"]
        pub fn krb5_cc_default(context: krb5_context,
                               ccache: *mut krb5_ccache) -> krb5_error_code;
        /* *
 * Find a credential cache with a specified client principal.
 *
 * @param [in]  context         Library context
 * @param [in]  client          Client principal
 * @param [out] cache_out       Credential cache handle
 *
 * Find a cache within the collection whose default principal is @a client.
 * Use @a krb5_cc_close to close @a ccache when it is no longer needed.
 *
 * @retval 0 Success
 * @retval KRB5_CC_NOTFOUND
 *
 * @sa krb5_cccol_cursor_new
 *
 * @version New in 1.10
 */
        #[no_mangle]
        #[c2rust::src_loc = "4547:1"]
        pub fn krb5_cc_cache_match(context: krb5_context,
                                   client: krb5_principal,
                                   cache_out: *mut krb5_ccache)
         -> krb5_error_code;
        /* krb5_free.c */
/* *
 * Free the storage assigned to a principal.
 *
 * @param [in] context          Library context
 * @param [in] val              Principal to be freed
 */
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
        /* *
 * Free a string allocated by a krb5 function.
 *
 * @param [in] context          Library context
 * @param [in] val              String to be freed
 *
 * @version New in 1.10
 */
        #[no_mangle]
        #[c2rust::src_loc = "4778:1"]
        pub fn krb5_free_string(context: krb5_context,
                                val: *mut libc::c_char);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:28"]
pub mod com_err_h {
    #[c2rust::src_loc = "26:1"]
    pub type errcode_t = libc::c_long;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn com_err(_: *const libc::c_char, _: errcode_t,
                       _: *const libc::c_char, _: ...);
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "252:14"]
        pub fn strrchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:27"]
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
#[c2rust::header_src = "/usr/include/bits/getopt_core.h:27"]
pub mod getopt_core_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "36:14"]
        pub static mut optarg: *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "50:12"]
        pub static mut optind: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "91:1"]
        pub fn getopt(___argc: libc::c_int, ___argv: *const *mut libc::c_char,
                      __shortopts: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:27"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/locale.h:30"]
pub mod locale_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "122:1"]
        pub fn setlocale(__category: libc::c_int,
                         __locale: *const libc::c_char) -> *mut libc::c_char;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__int32_t, __off_t, __off64_t};
pub use self::stdint_intn_h::int32_t;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::krb5_h::{krb5_int32, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_principal_data, krb5_principal,
                       krb5_context, krb5_ccache, krb5_cccol_cursor,
                       _krb5_context, _krb5_ccache, _krb5_cccol_cursor,
                       krb5_cc_get_full_name, krb5_cc_destroy, krb5_cc_close,
                       krb5_cccol_cursor_new, krb5_cccol_cursor_next,
                       krb5_cccol_cursor_free, krb5_init_context,
                       krb5_free_context, krb5_parse_name,
                       krb5_cc_set_default_name, krb5_cc_default,
                       krb5_cc_cache_match, krb5_free_principal,
                       krb5_free_string};
pub use self::com_err_h::{errcode_t, com_err};
use self::string_h::strrchr;
use self::stdlib_h::exit;
use self::stdio_h::{stderr, fprintf};
use self::getopt_core_h::{optarg, optind, getopt};
use self::libintl_h::dgettext;
use self::locale_h::setlocale;
#[no_mangle]
#[c2rust::src_loc = "46:7"]
pub static mut progname: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "49:1"]
unsafe extern "C" fn usage() {
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"Usage: %s [-A] [-q] [-c cache_name] [-p princ_name]\n\x00"
                         as *const u8 as *const libc::c_char), progname);
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-A destroy all credential caches in collection\n\x00"
                         as *const u8 as *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-q quiet mode\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-c specify name of credentials cache\n\x00" as
                         *const u8 as *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-p specify principal name within collection\n\x00" as
                         *const u8 as *const libc::c_char));
    exit(2 as libc::c_int);
}
/* Print a warning if there are still un-destroyed caches in the collection. */
#[c2rust::src_loc = "62:1"]
unsafe extern "C" fn print_remaining_cc_warning(mut context: krb5_context) {
    let mut ret: krb5_error_code = 0;
    let mut cache: krb5_ccache = 0 as *mut _krb5_ccache;
    let mut cursor: krb5_cccol_cursor = 0 as *mut _krb5_cccol_cursor;
    ret = krb5_cccol_cursor_new(context, &mut cursor);
    if ret != 0 {
        com_err(progname, ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while listing credential caches\x00" as *const u8
                             as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    ret = krb5_cccol_cursor_next(context, cursor, &mut cache);
    if ret == 0 as libc::c_int && !cache.is_null() {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Other credential caches present, use -A to destroy all\n\x00"
                             as *const u8 as *const libc::c_char));
        krb5_cc_close(context, cache);
    }
    krb5_cccol_cursor_free(context, &mut cursor);
}
#[c2rust::src_loc = "85:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut context: krb5_context = 0 as *mut _krb5_context;
    let mut ret: krb5_error_code = 0;
    let mut cache: krb5_ccache = 0 as krb5_ccache;
    let mut cursor: krb5_cccol_cursor = 0 as *mut _krb5_cccol_cursor;
    let mut princ: krb5_principal = 0 as *mut krb5_principal_data;
    let mut cache_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut princ_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut code: libc::c_int = 0 as libc::c_int;
    let mut errflg: libc::c_int = 0 as libc::c_int;
    let mut quiet: libc::c_int = 0 as libc::c_int;
    let mut all: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    setlocale(6 as libc::c_int, b"\x00" as *const u8 as *const libc::c_char);
    progname =
        if !strrchr(*argv.offset(0 as libc::c_int as isize),
                    '/' as i32).is_null() {
            strrchr(*argv.offset(0 as libc::c_int as isize),
                    '/' as i32).offset(1 as libc::c_int as isize)
        } else { *argv.offset(0 as libc::c_int as isize) };
    loop  {
        c =
            getopt(argc, argv as *const *mut libc::c_char,
                   b"54Aqc:p:\x00" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) { break ; }
        match c {
            65 => { all = 1 as libc::c_int }
            113 => { quiet = 1 as libc::c_int }
            99 => {
                if !cache_name.is_null() {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Only one -c option allowed\n\x00" as
                                         *const u8 as *const libc::c_char));
                    errflg += 1
                } else { cache_name = optarg }
            }
            112 => {
                if !princ_name.is_null() {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Only one -p option allowed\n\x00" as
                                         *const u8 as *const libc::c_char));
                    errflg += 1
                } else { princ_name = optarg }
            }
            52 => {
                fprintf(stderr,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"Kerberos 4 is no longer supported\n\x00" as
                                     *const u8 as *const libc::c_char));
                exit(3 as libc::c_int);
            }
            53 => { }
            63 | _ => { errflg += 1 }
        }
    }
    if all != 0 && !princ_name.is_null() {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"-A option is exclusive with -p option\n\x00" as
                             *const u8 as *const libc::c_char));
        errflg += 1
    }
    if optind != argc { errflg += 1 }
    if errflg != 0 { usage(); }
    ret = krb5_init_context(&mut context);
    if ret != 0 {
        com_err(progname, ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while initializing krb5\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    if !cache_name.is_null() {
        code = krb5_cc_set_default_name(context, cache_name);
        if code != 0 {
            com_err(progname, code as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while setting default cache name\x00" as
                                 *const u8 as *const libc::c_char));
            exit(1 as libc::c_int);
        }
    }
    if all != 0 {
        code = krb5_cccol_cursor_new(context, &mut cursor);
        if code != 0 {
            com_err(progname, code as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while listing credential caches\x00" as
                                 *const u8 as *const libc::c_char));
            exit(1 as libc::c_int);
        }
        while krb5_cccol_cursor_next(context, cursor, &mut cache) ==
                  0 as libc::c_int && !cache.is_null() {
            code = krb5_cc_get_full_name(context, cache, &mut cache_name);
            if code != 0 {
                com_err(progname, code as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"composing ccache name\x00" as *const u8 as
                                     *const libc::c_char));
                exit(1 as libc::c_int);
            }
            code = krb5_cc_destroy(context, cache);
            if code != 0 &&
                   code as libc::c_long != -(1765328189 as libc::c_long) {
                com_err(progname, code as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while destroying cache %s\x00" as *const u8
                                     as *const libc::c_char), cache_name);
            }
            krb5_free_string(context, cache_name);
        }
        krb5_cccol_cursor_free(context, &mut cursor);
        krb5_free_context(context);
        return 0 as libc::c_int
    }
    if !princ_name.is_null() {
        code = krb5_parse_name(context, princ_name, &mut princ);
        if code != 0 {
            com_err(progname, code as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while parsing principal name %s\x00" as
                                 *const u8 as *const libc::c_char),
                    princ_name);
            exit(1 as libc::c_int);
        }
        code = krb5_cc_cache_match(context, princ, &mut cache);
        if code != 0 {
            com_err(progname, code as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while finding cache for %s\x00" as *const u8 as
                                 *const libc::c_char), princ_name);
            exit(1 as libc::c_int);
        }
        krb5_free_principal(context, princ);
    } else {
        code = krb5_cc_default(context, &mut cache);
        if code != 0 {
            com_err(progname, code as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while resolving ccache\x00" as *const u8 as
                                 *const libc::c_char));
            exit(1 as libc::c_int);
        }
    }
    code = krb5_cc_destroy(context, cache);
    if code != 0 as libc::c_int {
        com_err(progname, code as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while destroying cache\x00" as *const u8 as
                             *const libc::c_char));
        if code as libc::c_long != -(1765328189 as libc::c_long) {
            if quiet != 0 {
                fprintf(stderr,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"Ticket cache NOT destroyed!\n\x00" as
                                     *const u8 as *const libc::c_char));
            } else {
                fprintf(stderr,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"Ticket cache %cNOT%c destroyed!\n\x00" as
                                     *const u8 as *const libc::c_char),
                        '\u{7}' as i32, '\u{7}' as i32);
            }
            errflg = 1 as libc::c_int
        }
    }
    if quiet == 0 && errflg == 0 && princ_name.is_null() {
        print_remaining_cc_warning(context);
    }
    krb5_free_context(context);
    return errflg;
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
