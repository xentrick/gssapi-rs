use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:2"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:2"]
pub mod types_h {
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:2"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:2"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:2"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/util/profile/prof_int.h:2"]
pub mod prof_int_h {
    use super::com_err_h::errcode_t;
    extern "C" {
        #[c2rust::src_loc = "33:9"]
        pub type profile_node;
        /* prof_tree.c */
        #[no_mangle]
        #[c2rust::src_loc = "135:1"]
        pub fn profile_free_node(relation: *mut profile_node);
        #[no_mangle]
        #[c2rust::src_loc = "150:1"]
        pub fn profile_make_node_final(node: *mut profile_node) -> errcode_t;
        #[no_mangle]
        #[c2rust::src_loc = "145:1"]
        pub fn profile_add_node(section: *mut profile_node,
                                name: *const libc::c_char,
                                value: *const libc::c_char,
                                ret_node: *mut *mut profile_node)
         -> errcode_t;
        #[no_mangle]
        #[c2rust::src_loc = "178:1"]
        pub fn profile_get_node_parent(section: *mut profile_node,
                                       parent: *mut *mut profile_node)
         -> errcode_t;
        #[no_mangle]
        #[c2rust::src_loc = "173:1"]
        pub fn profile_find_node_subsection(section: *mut profile_node,
                                            name: *const libc::c_char,
                                            state: *mut *mut libc::c_void,
                                            ret_name: *mut *mut libc::c_char,
                                            subsection:
                                                *mut *mut profile_node)
         -> errcode_t;
        #[no_mangle]
        #[c2rust::src_loc = "138:1"]
        pub fn profile_create_node(name: *const libc::c_char,
                                   value: *const libc::c_char,
                                   ret_node: *mut *mut profile_node)
         -> errcode_t;
        #[no_mangle]
        #[c2rust::src_loc = "153:1"]
        pub fn profile_is_node_final(node: *mut profile_node) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "168:1"]
        pub fn profile_find_node_relation(section: *mut profile_node,
                                          name: *const libc::c_char,
                                          state: *mut *mut libc::c_void,
                                          ret_name: *mut *mut libc::c_char,
                                          value: *mut *mut libc::c_char)
         -> errcode_t;
    }
    /* prof_set.c -- included from profile.h */
    /* Others included from profile.h */
}
#[c2rust::header_src = "/usr/include/ctype.h:11"]
pub mod ctype_h {
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed = 8192;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed = 8;
    #[c2rust::src_loc = "46:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "58:3"]
    pub const _ISpunct: C2RustUnnamed = 4;
    #[c2rust::src_loc = "57:3"]
    pub const _IScntrl: C2RustUnnamed = 2;
    #[c2rust::src_loc = "56:3"]
    pub const _ISblank: C2RustUnnamed = 1;
    #[c2rust::src_loc = "55:3"]
    pub const _ISgraph: C2RustUnnamed = 32768;
    #[c2rust::src_loc = "54:3"]
    pub const _ISprint: C2RustUnnamed = 16384;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed = 4096;
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed = 2048;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed = 256;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    }
}
#[c2rust::header_src = "/usr/include/string.h:2"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "225:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:2"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:2"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "246:14"]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "564:1"]
        pub fn fgets(__s: *mut libc::c_char, __n: libc::c_int,
                     __stream: *mut FILE) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "626:1"]
        pub fn fputs(__s: *const libc::c_char, __stream: *mut FILE)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "759:1"]
        pub fn feof(__stream: *mut FILE) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:2"]
pub mod k5_platform_h {
    extern "C" {
        /*
 * Localization macros.  If we have gettext, define _ appropriately for
 * translating a string.  If we do not have gettext, define _ and
 * bindtextdomain as no-ops.  N_ is always a no-op; it marks a string for
 * extraction to pot files but does not translate it.
 */
        /* HAVE_GETOPT */
        /* HAVE_GETOPT_LONG */
        /* Set *fnames_out to a null-terminated list of filenames within dirname,
 * sorted according to strcmp().  Return 0 on success, or ENOENT/ENOMEM. */
        #[no_mangle]
        #[c2rust::src_loc = "1148:1"]
        pub fn k5_dir_filenames(dirname: *const libc::c_char,
                                fnames_out: *mut *mut *mut libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1149:1"]
        pub fn k5_free_filenames(fnames: *mut *mut libc::c_char);
    }
    /* K5_PLATFORM_H */
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__off_t, __off64_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::com_err_h::errcode_t;
use self::prof_int_h::{profile_node, profile_free_node,
                       profile_make_node_final, profile_add_node,
                       profile_get_node_parent, profile_find_node_subsection,
                       profile_create_node, profile_is_node_final,
                       profile_find_node_relation};
pub use self::ctype_h::{_ISspace, _ISalnum, C2RustUnnamed, _ISpunct, _IScntrl,
                        _ISblank, _ISgraph, _ISprint, _ISxdigit, _ISdigit,
                        _ISalpha, _ISlower, _ISupper, __ctype_b_loc};
use self::string_h::{strcmp, strncmp, strdup, strchr, strlen, memcpy};
use self::stdlib_h::{malloc, realloc, free};
use self::stdio_h::{fclose, fopen, asprintf, fgets, fputs, feof};
use self::k5_platform_h::{k5_dir_filenames, k5_free_filenames};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "22:8"]
pub struct parse_state {
    pub state: libc::c_int,
    pub group_level: libc::c_int,
    pub root_section: *mut profile_node,
    pub current_section: *mut profile_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "584:8"]
pub struct prof_buf {
    pub base: *mut libc::c_char,
    pub cur: size_t,
    pub max: size_t,
    pub err: libc::c_int,
}
#[c2rust::src_loc = "32:1"]
unsafe extern "C" fn skip_over_blanks(mut cp: *mut libc::c_char)
 -> *mut libc::c_char {
    while *cp as libc::c_int != 0 &&
              *(*__ctype_b_loc()).offset(*cp as libc::c_int as isize) as
                  libc::c_int &
                  _ISspace as libc::c_int as libc::c_ushort as libc::c_int !=
                  0 {
        cp = cp.offset(1)
    }
    return cp;
}
#[c2rust::src_loc = "39:1"]
unsafe extern "C" fn strip_line(mut line: *mut libc::c_char) {
    let mut p: *mut libc::c_char = line.offset(strlen(line) as isize);
    while p > line &&
              (*p.offset(-(1 as libc::c_int) as isize) as libc::c_int ==
                   '\n' as i32 ||
                   *p.offset(-(1 as libc::c_int) as isize) as libc::c_int ==
                       '\r' as i32) {
        p = p.offset(-1);
        *p = 0 as libc::c_int as libc::c_char
    };
}
#[c2rust::src_loc = "46:1"]
unsafe extern "C" fn parse_quoted_string(mut str: *mut libc::c_char) {
    let mut to: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut from: *mut libc::c_char = 0 as *mut libc::c_char;
    from = str;
    to = from;
    while *from as libc::c_int != 0 && *from as libc::c_int != '\"' as i32 {
        if *from as libc::c_int == '\\' as i32 &&
               *from.offset(1 as libc::c_int as isize) as libc::c_int !=
                   '\u{0}' as i32 {
            from = from.offset(1);
            match *from as libc::c_int {
                110 => { *to = '\n' as i32 as libc::c_char }
                116 => { *to = '\t' as i32 as libc::c_char }
                98 => { *to = '\u{8}' as i32 as libc::c_char }
                _ => { *to = *from }
            }
        } else { *to = *from }
        to = to.offset(1);
        from = from.offset(1)
    }
    *to = '\u{0}' as i32 as libc::c_char;
}
#[c2rust::src_loc = "74:1"]
unsafe extern "C" fn parse_std_line(mut line: *mut libc::c_char,
                                    mut state: *mut parse_state)
 -> errcode_t {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ch: libc::c_char = 0;
    let mut tag: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retval: errcode_t = 0;
    let mut node: *mut profile_node = 0 as *mut profile_node;
    let mut do_subsection: libc::c_int = 0 as libc::c_int;
    let mut iter: *mut libc::c_void = 0 as *mut libc::c_void;
    if *line as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int as errcode_t
    }
    cp = skip_over_blanks(line);
    if *cp.offset(0 as libc::c_int as isize) as libc::c_int == ';' as i32 ||
           *cp.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
       {
        return 0 as libc::c_int as errcode_t
    }
    strip_line(cp);
    ch = *cp;
    if ch as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int as errcode_t
    }
    if ch as libc::c_int == '[' as i32 {
        if (*state).group_level > 0 as libc::c_int {
            return -(1429577715 as libc::c_long)
        }
        cp = cp.offset(1);
        p = strchr(cp, ']' as i32);
        if p.is_null() { return -(1429577714 as libc::c_long) }
        *p = '\u{0}' as i32 as libc::c_char;
        retval =
            profile_find_node_subsection((*state).root_section, cp, &mut iter,
                                         0 as *mut *mut libc::c_char,
                                         &mut (*state).current_section);
        if retval == -(1429577726 as libc::c_long) {
            retval =
                profile_add_node((*state).root_section, cp,
                                 0 as *const libc::c_char,
                                 &mut (*state).current_section);
            if retval != 0 { return retval }
        } else if retval != 0 { return retval }
        /*
         * Finish off the rest of the line.
         */
        cp = p.offset(1 as libc::c_int as isize);
        if *cp as libc::c_int == '*' as i32 {
            profile_make_node_final((*state).current_section);
            cp = cp.offset(1)
        }
        /*
         * A space after ']' should not be fatal
         */
        cp = skip_over_blanks(cp);
        if *cp != 0 { return -(1429577714 as libc::c_long) }
        return 0 as libc::c_int as errcode_t
    }
    if ch as libc::c_int == '}' as i32 {
        if (*state).group_level == 0 as libc::c_int {
            return -(1429577712 as libc::c_long)
        }
        if *cp.offset(1 as libc::c_int as isize) as libc::c_int == '*' as i32
           {
            profile_make_node_final((*state).current_section);
        }
        retval =
            profile_get_node_parent((*state).current_section,
                                    &mut (*state).current_section);
        if retval != 0 { return retval }
        (*state).group_level -= 1;
        return 0 as libc::c_int as errcode_t
    }
    /*
     * Parse the relations
     */
    tag = cp;
    cp = strchr(cp, '=' as i32);
    if cp.is_null() { return -(1429577713 as libc::c_long) }
    if cp == tag { return -(1429577713 as libc::c_long) }
    *cp = '\u{0}' as i32 as libc::c_char;
    p = tag;
    /* Look for whitespace on left-hand side.  */
    while p < cp &&
              *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as
                  libc::c_int &
                  _ISspace as libc::c_int as libc::c_ushort as libc::c_int ==
                  0 {
        p = p.offset(1)
    }
    if p < cp {
        /* Found some sort of whitespace.  */
        let fresh0 = p;
        p = p.offset(1);
        *fresh0 = 0 as libc::c_int as libc::c_char;
        /* If we have more non-whitespace, it's an error.  */
        while p < cp {
            if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as
                   libc::c_int &
                   _ISspace as libc::c_int as libc::c_ushort as libc::c_int ==
                   0 {
                return -(1429577713 as libc::c_long)
            }
            p = p.offset(1)
        }
    }
    cp = skip_over_blanks(cp.offset(1 as libc::c_int as isize));
    value = cp;
    if *value.offset(0 as libc::c_int as isize) as libc::c_int == '\"' as i32
       {
        value = value.offset(1);
        parse_quoted_string(value);
    } else if *value.offset(0 as libc::c_int as isize) as libc::c_int ==
                  0 as libc::c_int {
        do_subsection += 1;
        (*state).state = 3 as libc::c_int
    } else if *value.offset(0 as libc::c_int as isize) as libc::c_int ==
                  '{' as i32 &&
                  *skip_over_blanks(value.offset(1 as libc::c_int as isize))
                      as libc::c_int == 0 as libc::c_int {
        do_subsection += 1
    } else {
        cp =
            value.offset(strlen(value) as
                             isize).offset(-(1 as libc::c_int as isize));
        while cp > value &&
                  *(*__ctype_b_loc()).offset(*cp as libc::c_int as isize) as
                      libc::c_int &
                      _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                      != 0 {
            let fresh1 = cp;
            cp = cp.offset(-1);
            *fresh1 = 0 as libc::c_int as libc::c_char
        }
    }
    if do_subsection != 0 {
        p = strchr(tag, '*' as i32);
        if !p.is_null() { *p = '\u{0}' as i32 as libc::c_char }
        retval =
            profile_add_node((*state).current_section, tag,
                             0 as *const libc::c_char,
                             &mut (*state).current_section);
        if retval != 0 { return retval }
        if !p.is_null() { profile_make_node_final((*state).current_section); }
        (*state).group_level += 1;
        return 0 as libc::c_int as errcode_t
    }
    p = strchr(tag, '*' as i32);
    if !p.is_null() { *p = '\u{0}' as i32 as libc::c_char }
    profile_add_node((*state).current_section, tag, value, &mut node);
    if !p.is_null() { profile_make_node_final(node); }
    return 0 as libc::c_int as errcode_t;
}
/* Open and parse an included profile file. */
#[c2rust::src_loc = "202:1"]
unsafe extern "C" fn parse_include_file(mut filename: *const libc::c_char,
                                        mut root_section: *mut profile_node)
 -> errcode_t {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut retval: errcode_t = 0 as libc::c_int as errcode_t;
    let mut state: parse_state =
        parse_state{state: 0,
                    group_level: 0,
                    root_section: 0 as *mut profile_node,
                    current_section: 0 as *mut profile_node,};
    /* Create a new state so that fragments are syntactically independent but
     * share a root section. */
    state.state = 1 as libc::c_int;
    state.group_level = 0 as libc::c_int;
    state.root_section = root_section;
    state.current_section = 0 as *mut profile_node;
    fp = fopen(filename, b"r\x00" as *const u8 as *const libc::c_char);
    if fp.is_null() { return -(1429577697 as libc::c_long) }
    retval = parse_file(fp, &mut state, 0 as *mut *mut libc::c_char);
    fclose(fp);
    return retval;
}
/* Return non-zero if filename contains only alphanumeric characters, dashes,
 * and underscores, or if the filename ends in ".conf" and is not a dotfile. */
#[c2rust::src_loc = "226:1"]
unsafe extern "C" fn valid_name(mut filename: *const libc::c_char)
 -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = strlen(filename);
    /* Ignore dotfiles, which might be editor or filesystem artifacts. */
    if *filename as libc::c_int == '.' as i32 { return 0 as libc::c_int }
    if len >= 5 as libc::c_int as libc::c_ulong &&
           strcmp(filename.offset(len as
                                      isize).offset(-(5 as libc::c_int as
                                                          isize)),
                  b".conf\x00" as *const u8 as *const libc::c_char) == 0 {
        return 1 as libc::c_int
    }
    p = filename;
    while *p as libc::c_int != '\u{0}' as i32 {
        if *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as
                                          isize) as libc::c_int &
               _ISalnum as libc::c_int as libc::c_ushort as libc::c_int == 0
               && *p as libc::c_int != '-' as i32 &&
               *p as libc::c_int != '_' as i32 {
            return 0 as libc::c_int
        }
        p = p.offset(1)
    }
    return 1 as libc::c_int;
}
/*
 * Include files within dirname.  Only files with names ending in ".conf", or
 * consisting entirely of alphanumeric characters, dashes, and underscores are
 * included.  This restriction avoids including editor backup files, .rpmsave
 * files, and the like.  Files are processed in alphanumeric order.
 */
#[c2rust::src_loc = "251:1"]
unsafe extern "C" fn parse_include_dir(mut dirname: *const libc::c_char,
                                       mut root_section: *mut profile_node)
 -> errcode_t {
    let mut retval: errcode_t = 0 as libc::c_int as errcode_t;
    let mut fnames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut pathname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if k5_dir_filenames(dirname, &mut fnames) != 0 as libc::c_int {
        return -(1429577696 as libc::c_long)
    }
    i = 0 as libc::c_int;
    while !fnames.is_null() && !(*fnames.offset(i as isize)).is_null() {
        if !(valid_name(*fnames.offset(i as isize)) == 0) {
            if asprintf(&mut pathname as *mut *mut libc::c_char,
                        b"%s/%s\x00" as *const u8 as *const libc::c_char,
                        dirname, *fnames.offset(i as isize)) <
                   0 as libc::c_int {
                retval = 12 as libc::c_int as errcode_t;
                break ;
            } else {
                retval = parse_include_file(pathname, root_section);
                free(pathname as *mut libc::c_void);
                if retval != 0 { break ; }
            }
        }
        i += 1
    }
    k5_free_filenames(fnames);
    return retval;
}
#[c2rust::src_loc = "277:1"]
unsafe extern "C" fn parse_line(mut line: *mut libc::c_char,
                                mut state: *mut parse_state,
                                mut ret_modspec: *mut *mut libc::c_char)
 -> errcode_t {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if strncmp(line, b"include\x00" as *const u8 as *const libc::c_char,
               7 as libc::c_int as libc::c_ulong) == 0 as libc::c_int &&
           *(*__ctype_b_loc()).offset(*line.offset(7 as libc::c_int as isize)
                                          as libc::c_int as isize) as
               libc::c_int &
               _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        cp = skip_over_blanks(line.offset(7 as libc::c_int as isize));
        strip_line(cp);
        return parse_include_file(cp, (*state).root_section)
    }
    if strncmp(line, b"includedir\x00" as *const u8 as *const libc::c_char,
               10 as libc::c_int as libc::c_ulong) == 0 as libc::c_int &&
           *(*__ctype_b_loc()).offset(*line.offset(10 as libc::c_int as isize)
                                          as libc::c_int as isize) as
               libc::c_int &
               _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        cp = skip_over_blanks(line.offset(10 as libc::c_int as isize));
        strip_line(cp);
        return parse_include_dir(cp, (*state).root_section)
    }
    let mut current_block_27: u64;
    match (*state).state {
        1 => {
            if strncmp(line,
                       b"module\x00" as *const u8 as *const libc::c_char,
                       6 as libc::c_int as libc::c_ulong) == 0 as libc::c_int
                   &&
                   *(*__ctype_b_loc()).offset(*line.offset(6 as libc::c_int as
                                                               isize) as
                                                  libc::c_int as isize) as
                       libc::c_int &
                       _ISspace as libc::c_int as libc::c_ushort as
                           libc::c_int != 0 {
                /*
             * If we are expecting a module declaration, fill in *ret_modspec
             * and return PROF_MODULE, which will cause parsing to abort and
             * the module to be loaded instead.  If we aren't expecting a
             * module declaration, return PROF_MODULE without filling in
             * *ret_modspec, which will be treated as an ordinary error.
             */
                if !ret_modspec.is_null() {
                    cp =
                        skip_over_blanks(line.offset(6 as libc::c_int as
                                                         isize));
                    strip_line(cp);
                    *ret_modspec = strdup(cp);
                    if (*ret_modspec).is_null() {
                        return 12 as libc::c_int as errcode_t
                    }
                }
                return -(1429577693 as libc::c_long)
            }
            if *line.offset(0 as libc::c_int as isize) as libc::c_int !=
                   '[' as i32 {
                return 0 as libc::c_int as errcode_t
            }
            (*state).state = 2 as libc::c_int;
            current_block_27 = 17349618414521068618;
        }
        2 => { current_block_27 = 17349618414521068618; }
        3 => {
            cp = skip_over_blanks(line);
            if *cp as libc::c_int != '{' as i32 {
                return -(1429577711 as libc::c_long)
            }
            (*state).state = 2 as libc::c_int;
            current_block_27 = 16203760046146113240;
        }
        _ => { current_block_27 = 16203760046146113240; }
    }
    match current_block_27 {
        16203760046146113240 => { }
        _ => { return parse_std_line(line, state) }
    }
    return 0 as libc::c_int as errcode_t;
}
#[c2rust::src_loc = "325:1"]
unsafe extern "C" fn parse_file(mut f: *mut FILE, mut state: *mut parse_state,
                                mut ret_modspec: *mut *mut libc::c_char)
 -> errcode_t {
    let mut bptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retval: errcode_t = 0;
    bptr = malloc(2048 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    if bptr.is_null() { return 12 as libc::c_int as errcode_t }
    while feof(f) == 0 {
        if fgets(bptr, 2048 as libc::c_int, f).is_null() { break ; }
        retval = parse_line(bptr, state, ret_modspec);
        if retval != 0 { free(bptr as *mut libc::c_void); return retval }
    }
    free(bptr as *mut libc::c_void);
    return 0 as libc::c_int as errcode_t;
}
#[no_mangle]
#[c2rust::src_loc = "400:1"]
pub unsafe extern "C" fn profile_parse_file(mut f: *mut FILE,
                                            mut root: *mut *mut profile_node,
                                            mut ret_modspec:
                                                *mut *mut libc::c_char)
 -> errcode_t {
    let mut state: parse_state =
        parse_state{state: 0,
                    group_level: 0,
                    root_section: 0 as *mut profile_node,
                    current_section: 0 as *mut profile_node,};
    let mut retval: errcode_t = 0;
    *root = 0 as *mut profile_node;
    /* Initialize parsing state with a new root node. */
    state.state = 1 as libc::c_int;
    state.group_level = 0 as libc::c_int;
    state.current_section = 0 as *mut profile_node;
    retval =
        profile_create_node(b"(root)\x00" as *const u8 as *const libc::c_char,
                            0 as *const libc::c_char,
                            &mut state.root_section);
    if retval != 0 { return retval }
    retval = parse_file(f, &mut state, ret_modspec);
    if retval != 0 { profile_free_node(state.root_section); return retval }
    *root = state.root_section;
    return 0 as libc::c_int as errcode_t;
}
#[no_mangle]
#[c2rust::src_loc = "425:1"]
pub unsafe extern "C" fn profile_process_directory(mut dirname:
                                                       *const libc::c_char,
                                                   mut root:
                                                       *mut *mut profile_node)
 -> errcode_t {
    let mut retval: errcode_t = 0;
    let mut node: *mut profile_node = 0 as *mut profile_node;
    *root = 0 as *mut profile_node;
    retval =
        profile_create_node(b"(root)\x00" as *const u8 as *const libc::c_char,
                            0 as *const libc::c_char, &mut node);
    if retval != 0 { return retval }
    retval = parse_include_dir(dirname, node);
    if retval != 0 { profile_free_node(node); return retval }
    *root = node;
    return 0 as libc::c_int as errcode_t;
}
/*
 * Return TRUE if the string begins or ends with whitespace
 */
#[c2rust::src_loc = "447:1"]
unsafe extern "C" fn need_double_quotes(mut str: *mut libc::c_char)
 -> libc::c_int {
    if str.is_null() { return 0 as libc::c_int }
    if *str.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
       {
        return 1 as libc::c_int
    }
    if *(*__ctype_b_loc()).offset(*str as libc::c_int as isize) as libc::c_int
           & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0 ||
           *(*__ctype_b_loc()).offset(*str.offset(strlen(str) as
                                                      isize).offset(-(1 as
                                                                          libc::c_int
                                                                          as
                                                                          isize))
                                          as libc::c_int as isize) as
               libc::c_int &
               _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        return 1 as libc::c_int
    }
    if !strchr(str, '\n' as i32).is_null() ||
           !strchr(str, '\t' as i32).is_null() ||
           !strchr(str, '\u{8}' as i32).is_null() {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/*
 * Output a string with double quotes, doing appropriate backquoting
 * of characters as necessary.
 */
#[c2rust::src_loc = "464:1"]
unsafe extern "C" fn output_quoted_string(mut str: *mut libc::c_char,
                                          mut cb:
                                              Option<unsafe extern "C" fn(_:
                                                                              *const libc::c_char,
                                                                          _:
                                                                              *mut libc::c_void)
                                                         -> ()>,
                                          mut data: *mut libc::c_void) {
    let mut ch: libc::c_char = 0;
    let mut buf: [libc::c_char; 2] = [0; 2];
    cb.expect("non-null function pointer")(b"\"\x00" as *const u8 as
                                               *const libc::c_char, data);
    if str.is_null() {
        cb.expect("non-null function pointer")(b"\"\x00" as *const u8 as
                                                   *const libc::c_char, data);
        return
    }
    buf[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    loop  {
        let fresh2 = str;
        str = str.offset(1);
        ch = *fresh2;
        if !(ch != 0) { break ; }
        match ch as libc::c_int {
            92 => {
                cb.expect("non-null function pointer")(b"\\\\\x00" as
                                                           *const u8 as
                                                           *const libc::c_char,
                                                       data);
            }
            10 => {
                cb.expect("non-null function pointer")(b"\\n\x00" as *const u8
                                                           as
                                                           *const libc::c_char,
                                                       data);
            }
            9 => {
                cb.expect("non-null function pointer")(b"\\t\x00" as *const u8
                                                           as
                                                           *const libc::c_char,
                                                       data);
            }
            8 => {
                cb.expect("non-null function pointer")(b"\\b\x00" as *const u8
                                                           as
                                                           *const libc::c_char,
                                                       data);
            }
            _ => {
                /* This would be a lot faster if we scanned
               forward for the next "interesting"
               character.  */
                buf[0 as libc::c_int as usize] = ch;
                cb.expect("non-null function pointer")(buf.as_mut_ptr(),
                                                       data);
            }
        }
    }
    cb.expect("non-null function pointer")(b"\"\x00" as *const u8 as
                                               *const libc::c_char, data);
}
/* Errors should be returned, not ignored!  */
#[c2rust::src_loc = "513:1"]
unsafe extern "C" fn dump_profile(mut root: *mut profile_node,
                                  mut level: libc::c_int,
                                  mut cb:
                                      Option<unsafe extern "C" fn(_:
                                                                      *const libc::c_char,
                                                                  _:
                                                                      *mut libc::c_void)
                                                 -> ()>,
                                  mut data: *mut libc::c_void) {
    let mut i: libc::c_int = 0; /* xxx = { ... } */
    let mut p: *mut profile_node = 0 as *mut profile_node;
    let mut iter: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut retval: libc::c_long = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    iter = 0 as *mut libc::c_void;
    loop  {
        retval =
            profile_find_node_relation(root, 0 as *const libc::c_char,
                                       &mut iter, &mut name, &mut value);
        if retval != 0 { break ; }
        i = 0 as libc::c_int;
        while i < level {
            cb.expect("non-null function pointer")(b"\t\x00" as *const u8 as
                                                       *const libc::c_char,
                                                   data);
            i += 1
        }
        if need_double_quotes(value) != 0 {
            cb.expect("non-null function pointer")(name, data);
            cb.expect("non-null function pointer")(b" = \x00" as *const u8 as
                                                       *const libc::c_char,
                                                   data);
            output_quoted_string(value, cb, data);
            cb.expect("non-null function pointer")(b"\n\x00" as *const u8 as
                                                       *const libc::c_char,
                                                   data);
        } else {
            cb.expect("non-null function pointer")(name, data);
            cb.expect("non-null function pointer")(b" = \x00" as *const u8 as
                                                       *const libc::c_char,
                                                   data);
            cb.expect("non-null function pointer")(value, data);
            cb.expect("non-null function pointer")(b"\n\x00" as *const u8 as
                                                       *const libc::c_char,
                                                   data);
        }
        if iter.is_null() { break ; }
    }
    iter = 0 as *mut libc::c_void;
    loop  {
        retval =
            profile_find_node_subsection(root, 0 as *const libc::c_char,
                                         &mut iter, &mut name, &mut p);
        if retval != 0 { break ; }
        if level == 0 as libc::c_int {
            /* [xxx] */
            cb.expect("non-null function pointer")(b"[\x00" as *const u8 as
                                                       *const libc::c_char,
                                                   data);
            cb.expect("non-null function pointer")(name, data);
            cb.expect("non-null function pointer")(b"]\x00" as *const u8 as
                                                       *const libc::c_char,
                                                   data);
            cb.expect("non-null function pointer")(if profile_is_node_final(p)
                                                          != 0 {
                                                       b"*\x00" as *const u8
                                                           as
                                                           *const libc::c_char
                                                   } else {
                                                       b"\x00" as *const u8 as
                                                           *const libc::c_char
                                                   }, data);
            cb.expect("non-null function pointer")(b"\n\x00" as *const u8 as
                                                       *const libc::c_char,
                                                   data);
            dump_profile(p, level + 1 as libc::c_int, cb, data);
            cb.expect("non-null function pointer")(b"\n\x00" as *const u8 as
                                                       *const libc::c_char,
                                                   data);
        } else {
            i = 0 as libc::c_int;
            while i < level {
                cb.expect("non-null function pointer")(b"\t\x00" as *const u8
                                                           as
                                                           *const libc::c_char,
                                                       data);
                i += 1
            }
            cb.expect("non-null function pointer")(name, data);
            cb.expect("non-null function pointer")(b" = {\x00" as *const u8 as
                                                       *const libc::c_char,
                                                   data);
            cb.expect("non-null function pointer")(b"\n\x00" as *const u8 as
                                                       *const libc::c_char,
                                                   data);
            dump_profile(p, level + 1 as libc::c_int, cb, data);
            i = 0 as libc::c_int;
            while i < level {
                cb.expect("non-null function pointer")(b"\t\x00" as *const u8
                                                           as
                                                           *const libc::c_char,
                                                       data);
                i += 1
            }
            cb.expect("non-null function pointer")(b"}\x00" as *const u8 as
                                                       *const libc::c_char,
                                                   data);
            cb.expect("non-null function pointer")(if profile_is_node_final(p)
                                                          != 0 {
                                                       b"*\x00" as *const u8
                                                           as
                                                           *const libc::c_char
                                                   } else {
                                                       b"\x00" as *const u8 as
                                                           *const libc::c_char
                                                   }, data);
            cb.expect("non-null function pointer")(b"\n\x00" as *const u8 as
                                                       *const libc::c_char,
                                                   data);
        }
        if iter.is_null() { break ; }
    };
}
#[c2rust::src_loc = "573:1"]
unsafe extern "C" fn dump_profile_to_file_cb(mut str: *const libc::c_char,
                                             mut data: *mut libc::c_void) {
    fputs(str, data as *mut FILE);
}
#[no_mangle]
#[c2rust::src_loc = "578:1"]
pub unsafe extern "C" fn profile_write_tree_file(mut root: *mut profile_node,
                                                 mut dstfile: *mut FILE)
 -> errcode_t {
    dump_profile(root, 0 as libc::c_int,
                 Some(dump_profile_to_file_cb as
                          unsafe extern "C" fn(_: *const libc::c_char,
                                               _: *mut libc::c_void) -> ()),
                 dstfile as *mut libc::c_void);
    return 0 as libc::c_int as errcode_t;
}
#[c2rust::src_loc = "590:1"]
unsafe extern "C" fn add_data_to_buffer(mut b: *mut prof_buf,
                                        mut d: *const libc::c_void,
                                        mut len: size_t) {
    if (*b).err != 0 { return }
    if (*b).max.wrapping_sub((*b).cur) < len {
        let mut newsize: size_t = 0;
        let mut newptr: *mut libc::c_char = 0 as *mut libc::c_char;
        newsize =
            (*b).max.wrapping_add((*b).max >>
                                      1 as
                                          libc::c_int).wrapping_add(len).wrapping_add(1024
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong);
        newptr =
            realloc((*b).base as *mut libc::c_void, newsize) as
                *mut libc::c_char;
        if newptr.is_null() { (*b).err = 1 as libc::c_int; return }
        (*b).base = newptr;
        (*b).max = newsize
    }
    memcpy((*b).base.offset((*b).cur as isize) as *mut libc::c_void, d, len);
    (*b).cur =
        ((*b).cur as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    /* ignore overflow */
}
#[c2rust::src_loc = "611:1"]
unsafe extern "C" fn dump_profile_to_buffer_cb(mut str: *const libc::c_char,
                                               mut data: *mut libc::c_void) {
    add_data_to_buffer(data as *mut prof_buf, str as *const libc::c_void,
                       strlen(str));
}
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
/* If non-null, use vtable operations instead of native ones. */
/*
 * Used by the profile iterator in prof_get.c
 */
/*
 * Check if a filespec is last in a list (NULL on UNIX, invalid FSSpec on MacOS
 */
/* profile_parse.c */
#[no_mangle]
#[c2rust::src_loc = "616:1"]
pub unsafe extern "C" fn profile_write_tree_to_buffer(mut root:
                                                          *mut profile_node,
                                                      mut buf:
                                                          *mut *mut libc::c_char)
 -> errcode_t {
    let mut prof_buf: prof_buf =
        {
            let mut init =
                prof_buf{base: 0 as *mut libc::c_char,
                         cur: 0 as libc::c_int as size_t,
                         max: 0 as libc::c_int as size_t,
                         err: 0 as libc::c_int,}; /* append nul */
            init
        };
    dump_profile(root, 0 as libc::c_int,
                 Some(dump_profile_to_buffer_cb as
                          unsafe extern "C" fn(_: *const libc::c_char,
                                               _: *mut libc::c_void) -> ()),
                 &mut prof_buf as *mut prof_buf as *mut libc::c_void);
    if prof_buf.err != 0 {
        *buf = 0 as *mut libc::c_char;
        return 12 as libc::c_int as errcode_t
    }
    add_data_to_buffer(&mut prof_buf,
                       b"\x00" as *const u8 as *const libc::c_char as
                           *const libc::c_void, 1 as libc::c_int as size_t);
    if prof_buf.max.wrapping_sub(prof_buf.cur) >
           prof_buf.max >> 3 as libc::c_int {
        let mut newptr: *mut libc::c_char =
            realloc(prof_buf.base as *mut libc::c_void, prof_buf.cur) as
                *mut libc::c_char;
        if !newptr.is_null() { prof_buf.base = newptr }
    }
    *buf = prof_buf.base;
    return 0 as libc::c_int as errcode_t;
}
