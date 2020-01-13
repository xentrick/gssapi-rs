use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/include/bits/types.h:41"]
pub mod types_h {
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:41"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:41"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:41"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:41"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:41"]
pub mod krb5_h {
    #[c2rust::src_loc = "138:1"]
    pub type krb5_ui_2 = uint16_t;
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
    use super::stdint_uintn_h::{uint16_t, uint32_t};
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/unicode/ure/ure.h:49"]
pub mod ure_h {
    #[c2rust::src_loc = "104:1"]
    pub type ucs4_t = krb5_ui_4;
    #[c2rust::src_loc = "105:1"]
    pub type ucs2_t = krb5_ui_2;
    /*
 * Opaque type for memory used when compiling expressions.
 */
    #[c2rust::src_loc = "110:1"]
    pub type ure_buffer_t = *mut _ure_buffer_t;
    /*
 * Opaque type for the minimal DFA used when matching.
 */
    #[c2rust::src_loc = "115:1"]
    pub type ure_dfa_t = *mut _ure_dfa_t;
    use super::krb5_h::{krb5_ui_4, krb5_ui_2};
    use super::{_ure_buffer_t, _ure_dfa_t};
    extern "C" {
        /* ************************************************************************
 *
 * Prototypes for stub functions used for URE.  These need to be rewritten to
 * use the Unicode support available on the system.
 *
 *************************************************************************/
        #[no_mangle]
        #[c2rust::src_loc = "147:1"]
        pub fn _ure_tolower(c: ucs4_t) -> ucs4_t;
        #[no_mangle]
        #[c2rust::src_loc = "149:1"]
        pub fn _ure_matches_properties(props: libc::c_ulong, c: ucs4_t)
         -> libc::c_int;
    }
    /* _h_ure */
}
#[c2rust::header_src = "/usr/include/stdio.h:41"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "522:1"]
        pub fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:41"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:41"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
pub use self::types_h::{__uint16_t, __uint32_t, __off_t, __off64_t};
pub use self::stddef_h::size_t;
pub use self::stdint_uintn_h::{uint16_t, uint32_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::krb5_h::{krb5_ui_2, krb5_ui_4};
pub use self::ure_h::{ucs4_t, ucs2_t, ure_buffer_t, ure_dfa_t, _ure_tolower,
                      _ure_matches_properties};
use self::stdio_h::{fprintf, putc};
use self::string_h::{memcmp, memset, memcpy};
use self::stdlib_h::{malloc, calloc, realloc, free};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "207:16"]
pub struct _ure_buffer_t {
    pub reducing: libc::c_int,
    pub error: libc::c_int,
    pub flags: libc::c_ulong,
    pub stack: _ure_stlist_t,
    pub symtab: *mut _ure_symtab_t,
    pub symtab_size: ucs2_t,
    pub symtab_used: ucs2_t,
    pub expr: *mut _ure_elt_t,
    pub expr_used: ucs2_t,
    pub expr_size: ucs2_t,
    pub states: _ure_statetable_t,
    pub equiv: *mut _ure_equiv_t,
    pub equiv_used: ucs2_t,
    pub equiv_size: ucs2_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "199:9"]
pub struct _ure_equiv_t {
    pub l: ucs2_t,
    pub r: ucs2_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "189:9"]
pub struct _ure_statetable_t {
    pub states: *mut _ure_state_t,
    pub states_size: ucs2_t,
    pub states_used: ucs2_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "176:9"]
pub struct _ure_state_t {
    pub id: ucs2_t,
    pub accepting: ucs2_t,
    pub pad: ucs2_t,
    pub st: _ure_stlist_t,
    pub trans: *mut _ure_elt_t,
    pub trans_size: ucs2_t,
    pub trans_used: ucs2_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "143:9"]
pub struct _ure_elt_t {
    pub reg: ucs2_t,
    pub onstack: ucs2_t,
    pub type_0: ucs2_t,
    pub lhs: ucs2_t,
    pub rhs: ucs2_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "154:9"]
pub struct _ure_stlist_t {
    pub slist: *mut ucs2_t,
    pub slist_size: ucs2_t,
    pub slist_used: ucs2_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "164:9"]
pub struct _ure_symtab_t {
    pub id: ucs2_t,
    pub type_0: ucs2_t,
    pub mods: libc::c_ulong,
    pub props: libc::c_ulong,
    pub sym: _ure_sym_t,
    pub states: _ure_stlist_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "134:9"]
pub union _ure_sym_t {
    pub chr: ucs4_t,
    pub ccl: _ure_ccl_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "128:9"]
pub struct _ure_ccl_t {
    pub ranges: *mut _ure_range_t,
    pub ranges_used: ucs2_t,
    pub ranges_size: ucs2_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "123:9"]
pub struct _ure_range_t {
    pub min_code: ucs4_t,
    pub max_code: ucs4_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "253:16"]
pub struct _ure_dfa_t {
    pub flags: libc::c_ulong,
    pub syms: *mut _ure_symtab_t,
    pub nsyms: ucs2_t,
    pub states: *mut _ure_dstate_t,
    pub nstates: ucs2_t,
    pub trans: *mut _ure_trans_t,
    pub ntrans: ucs2_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "242:9"]
pub struct _ure_trans_t {
    pub symbol: ucs2_t,
    pub next_state: ucs2_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "247:9"]
pub struct _ure_dstate_t {
    pub accepting: ucs2_t,
    pub ntrans: ucs2_t,
    pub trans: *mut _ure_trans_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
#[c2rust::src_loc = "551:9"]
pub struct _ure_trie_t {
    pub key: ucs2_t,
    #[bitfield(name = "len", ty = "libc::c_uint", bits = "0..=7")]
    #[bitfield(name = "next", ty = "libc::c_uint", bits = "8..=15")]
    pub len_next: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub func: _ure_cclsetup_t,
    pub mask: libc::c_ulong,
}
#[c2rust::src_loc = "545:1"]
pub type _ure_cclsetup_t
    =
    Option<unsafe extern "C" fn(_: *mut _ure_symtab_t, _: libc::c_ulong,
                                _: *mut _ure_buffer_t) -> ()>;
#[c2rust::src_loc = "57:22"]
static mut cclass_flags: [libc::c_ulong; 33] =
    [0 as libc::c_int as libc::c_ulong, 0x1 as libc::c_int as libc::c_ulong,
     0x2 as libc::c_int as libc::c_ulong, 0x4 as libc::c_int as libc::c_ulong,
     0x8 as libc::c_int as libc::c_ulong,
     0x10 as libc::c_int as libc::c_ulong,
     0x20 as libc::c_int as libc::c_ulong,
     0x40 as libc::c_int as libc::c_ulong,
     0x80 as libc::c_int as libc::c_ulong,
     0x100 as libc::c_int as libc::c_ulong,
     0x200 as libc::c_int as libc::c_ulong,
     0x400 as libc::c_int as libc::c_ulong,
     0x800 as libc::c_int as libc::c_ulong,
     0x1000 as libc::c_int as libc::c_ulong,
     0x2000 as libc::c_int as libc::c_ulong,
     0x4000 as libc::c_int as libc::c_ulong,
     0x8000 as libc::c_int as libc::c_ulong,
     0x10000 as libc::c_int as libc::c_ulong,
     0x20000 as libc::c_int as libc::c_ulong,
     0x40000 as libc::c_int as libc::c_ulong,
     0x80000 as libc::c_int as libc::c_ulong,
     0x100000 as libc::c_int as libc::c_ulong,
     0x200000 as libc::c_int as libc::c_ulong,
     0x400000 as libc::c_int as libc::c_ulong,
     0x800000 as libc::c_int as libc::c_ulong,
     0x1000000 as libc::c_int as libc::c_ulong,
     0x2000000 as libc::c_int as libc::c_ulong,
     0x4000000 as libc::c_int as libc::c_ulong,
     0x8000000 as libc::c_int as libc::c_ulong,
     0x10000000 as libc::c_int as libc::c_ulong,
     0x20000000 as libc::c_int as libc::c_ulong,
     0x40000000 as libc::c_int as libc::c_ulong,
     0x80000000 as libc::c_uint as libc::c_ulong];
/* ************************************************************************
 *
 * Functions.
 *
 *************************************************************************/
#[c2rust::src_loc = "272:1"]
unsafe extern "C" fn _ure_memmove(mut dest: *mut libc::c_char,
                                  mut src: *mut libc::c_char,
                                  mut bytes: libc::c_ulong) {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    i = bytes as libc::c_long;
    j = i & 7 as libc::c_int as libc::c_long;
    i = i + 7 as libc::c_int as libc::c_long >> 3 as libc::c_int;
    /*
     * Do a memmove using Ye Olde Duff's Device for efficiency.
     */
    if src < dest {
        src = src.offset(bytes as isize);
        dest = dest.offset(bytes as isize);
        let mut current_block_12: u64;
        match j {
            0 => { current_block_12 = 11006700562992250127; }
            7 => { current_block_12 = 13264933720371784297; }
            6 => { current_block_12 = 7451279748152143041; }
            5 => { current_block_12 = 6266472726643588263; }
            4 => { current_block_12 = 4432029799472520199; }
            3 => { current_block_12 = 4912292479518253705; }
            2 => { current_block_12 = 13464891994565038520; }
            1 => { current_block_12 = 6484990028451387192; }
            _ => { current_block_12 = 5689001924483802034; }
        }
        loop  {
            match current_block_12 {
                5689001924483802034 => { return; }
                11006700562992250127 => {
                    src = src.offset(-1);
                    dest = dest.offset(-1);
                    *dest = *src;
                    current_block_12 = 13264933720371784297;
                }
                13264933720371784297 => {
                    src = src.offset(-1);
                    dest = dest.offset(-1);
                    *dest = *src;
                    current_block_12 = 7451279748152143041;
                }
                7451279748152143041 => {
                    src = src.offset(-1);
                    dest = dest.offset(-1);
                    *dest = *src;
                    current_block_12 = 6266472726643588263;
                }
                6266472726643588263 => {
                    src = src.offset(-1);
                    dest = dest.offset(-1);
                    *dest = *src;
                    current_block_12 = 4432029799472520199;
                }
                4432029799472520199 => {
                    src = src.offset(-1);
                    dest = dest.offset(-1);
                    *dest = *src;
                    current_block_12 = 4912292479518253705;
                }
                4912292479518253705 => {
                    src = src.offset(-1);
                    dest = dest.offset(-1);
                    *dest = *src;
                    current_block_12 = 13464891994565038520;
                }
                13464891994565038520 => {
                    src = src.offset(-1);
                    dest = dest.offset(-1);
                    *dest = *src;
                    current_block_12 = 6484990028451387192;
                }
                _ => {
                    src = src.offset(-1);
                    dest = dest.offset(-1);
                    *dest = *src;
                    i -= 1;
                    if i > 0 as libc::c_int as libc::c_long {
                        current_block_12 = 11006700562992250127;
                    } else { current_block_12 = 5689001924483802034; }
                }
            }
        }
    } else if src > dest {
        let mut current_block_22: u64;
        match j {
            0 => { current_block_22 = 15089075282327824602; }
            7 => { current_block_22 = 7430496983381784292; }
            6 => { current_block_22 = 11467596646983695934; }
            5 => { current_block_22 = 7702736575060247376; }
            4 => { current_block_22 = 4735242710682322288; }
            3 => { current_block_22 = 6011889920987618024; }
            2 => { current_block_22 = 18184655071983663978; }
            1 => { current_block_22 = 7846112615066368553; }
            _ => { current_block_22 = 18377268871191777778; }
        }
        loop  {
            match current_block_22 {
                18377268871191777778 => { return; }
                15089075282327824602 => {
                    let fresh0 = src;
                    src = src.offset(1);
                    let fresh1 = dest;
                    dest = dest.offset(1);
                    *fresh1 = *fresh0;
                    current_block_22 = 7430496983381784292;
                }
                7430496983381784292 => {
                    let fresh2 = src;
                    src = src.offset(1);
                    let fresh3 = dest;
                    dest = dest.offset(1);
                    *fresh3 = *fresh2;
                    current_block_22 = 11467596646983695934;
                }
                11467596646983695934 => {
                    let fresh4 = src;
                    src = src.offset(1);
                    let fresh5 = dest;
                    dest = dest.offset(1);
                    *fresh5 = *fresh4;
                    current_block_22 = 7702736575060247376;
                }
                7702736575060247376 => {
                    let fresh6 = src;
                    src = src.offset(1);
                    let fresh7 = dest;
                    dest = dest.offset(1);
                    *fresh7 = *fresh6;
                    current_block_22 = 4735242710682322288;
                }
                4735242710682322288 => {
                    let fresh8 = src;
                    src = src.offset(1);
                    let fresh9 = dest;
                    dest = dest.offset(1);
                    *fresh9 = *fresh8;
                    current_block_22 = 6011889920987618024;
                }
                6011889920987618024 => {
                    let fresh10 = src;
                    src = src.offset(1);
                    let fresh11 = dest;
                    dest = dest.offset(1);
                    *fresh11 = *fresh10;
                    current_block_22 = 18184655071983663978;
                }
                18184655071983663978 => {
                    let fresh12 = src;
                    src = src.offset(1);
                    let fresh13 = dest;
                    dest = dest.offset(1);
                    *fresh13 = *fresh12;
                    current_block_22 = 7846112615066368553;
                }
                _ => {
                    let fresh14 = src;
                    src = src.offset(1);
                    let fresh15 = dest;
                    dest = dest.offset(1);
                    *fresh15 = *fresh14;
                    i -= 1;
                    if i > 0 as libc::c_int as libc::c_long {
                        current_block_22 = 15089075282327824602;
                    } else { current_block_22 = 18377268871191777778; }
                }
            }
        }
    };
}
#[c2rust::src_loc = "316:1"]
unsafe extern "C" fn _ure_push(mut v: ucs2_t, mut b: *mut _ure_buffer_t) {
    let mut s: *mut _ure_stlist_t = 0 as *mut _ure_stlist_t;
    if b.is_null() { return }
    /*
     * If the `reducing' parameter is non-zero, check to see if the value
     * passed is already on the stack.
     */
    if (*b).reducing != 0 as libc::c_int &&
           (*(*b).expr.offset(v as isize)).onstack as libc::c_int !=
               0 as libc::c_int {
        return
    }
    s = &mut (*b).stack;
    if (*s).slist_used as libc::c_int == (*s).slist_size as libc::c_int {
        if (*s).slist_size as libc::c_int == 0 as libc::c_int {
            (*s).slist =
                malloc((::std::mem::size_of::<ucs2_t>() as libc::c_ulong) <<
                           3 as libc::c_int) as *mut ucs2_t
        } else {
            (*s).slist =
                realloc((*s).slist as *mut libc::c_char as *mut libc::c_void,
                        (::std::mem::size_of::<ucs2_t>() as
                             libc::c_ulong).wrapping_mul(((*s).slist_size as
                                                              libc::c_int +
                                                              8 as
                                                                  libc::c_int)
                                                             as
                                                             libc::c_ulong))
                    as *mut ucs2_t
        }
        (*s).slist_size =
            ((*s).slist_size as libc::c_int + 8 as libc::c_int) as ucs2_t
    }
    let fresh16 = (*s).slist_used;
    (*s).slist_used = (*s).slist_used.wrapping_add(1);
    *(*s).slist.offset(fresh16 as isize) = v;
    /*
     * If the `reducing' parameter is non-zero, flag the element as being on
     * the stack.
     */
    if (*b).reducing != 0 as libc::c_int {
        (*(*b).expr.offset(v as isize)).onstack = 1 as libc::c_int as ucs2_t
    };
}
#[c2rust::src_loc = "350:1"]
unsafe extern "C" fn _ure_peek(mut b: *mut _ure_buffer_t) -> ucs2_t {
    if b.is_null() || (*b).stack.slist_used as libc::c_int == 0 as libc::c_int
       {
        return 0xffff as libc::c_int as ucs2_t
    }
    return *(*b).stack.slist.offset(((*b).stack.slist_used as libc::c_int -
                                         1 as libc::c_int) as isize);
}
#[c2rust::src_loc = "359:1"]
unsafe extern "C" fn _ure_pop(mut b: *mut _ure_buffer_t) -> ucs2_t {
    let mut v: ucs2_t = 0;
    if b.is_null() || (*b).stack.slist_used as libc::c_int == 0 as libc::c_int
       {
        return 0xffff as libc::c_int as ucs2_t
    }
    (*b).stack.slist_used = (*b).stack.slist_used.wrapping_sub(1);
    v = *(*b).stack.slist.offset((*b).stack.slist_used as isize);
    if (*b).reducing != 0 {
        (*(*b).expr.offset(v as isize)).onstack = 0 as libc::c_int as ucs2_t
    }
    return v;
}
/* ************************************************************************
 *
 * Start symbol parse functions.
 *
 *************************************************************************/
/*
 * Parse a comma-separated list of integers that represent character
 * properties.  Combine them into a mask that is returned in the `mask'
 * variable, and return the number of characters consumed.
 */
#[c2rust::src_loc = "385:1"]
unsafe extern "C" fn _ure_prop_list(mut pp: *mut ucs2_t,
                                    mut limit: libc::c_ulong,
                                    mut mask: *mut libc::c_ulong,
                                    mut b: *mut _ure_buffer_t)
 -> libc::c_ulong {
    let mut n: libc::c_ulong = 0;
    let mut m: libc::c_ulong = 0;
    let mut sp: *mut ucs2_t = 0 as *mut ucs2_t;
    let mut ep: *mut ucs2_t = 0 as *mut ucs2_t;
    sp = pp;
    ep = sp.offset(limit as isize);
    n = 0 as libc::c_int as libc::c_ulong;
    m = n;
    while (*b).error == 0 as libc::c_int && sp < ep {
        if *sp as libc::c_int == ',' as i32 {
            /*
             * Encountered a comma, so select the next character property flag
             * and reset the number.
             */
            m |= cclass_flags[n as usize];
            n = 0 as libc::c_int as libc::c_ulong
        } else {
            if !(*sp as libc::c_int >= '0' as i32 &&
                     *sp as libc::c_int <= '9' as i32) {
                break ;
            }
            /*
           * Encountered a digit, so start or continue building the cardinal
           * that represents the character property flag.
           */
            n =
                n.wrapping_mul(10 as libc::c_int as
                                   libc::c_ulong).wrapping_add((*sp as
                                                                    libc::c_int
                                                                    -
                                                                    '0' as
                                                                        i32)
                                                                   as
                                                                   libc::c_ulong)
        }
        /*
         * If a property number greater than 32 occurs, then there is a
         * problem.  Most likely a missing comma separator.
         */
        if n > 32 as libc::c_int as libc::c_ulong {
            (*b).error = -(4 as libc::c_int)
        }
        sp = sp.offset(1)
    }
    if (*b).error == 0 as libc::c_int &&
           n != 0 as libc::c_int as libc::c_ulong {
        m |= cclass_flags[n as usize]
    }
    /*
     * Set the mask that represents the group of character properties.
     */
    *mask = m;
    /*
     * Return the number of characters consumed.
     */
    return sp.wrapping_offset_from(pp) as libc::c_long as libc::c_ulong;
}
/*
 * Collect a hex number with 1 to 4 digits and return the number
 * of characters used.
 */
#[c2rust::src_loc = "442:1"]
unsafe extern "C" fn _ure_hex(mut np: *mut ucs2_t, mut limit: libc::c_ulong,
                              mut n: *mut ucs4_t) -> libc::c_ulong {
    let mut i: ucs2_t = 0;
    let mut sp: *mut ucs2_t = 0 as *mut ucs2_t;
    let mut ep: *mut ucs2_t = 0 as *mut ucs2_t;
    let mut nn: ucs4_t = 0;
    sp = np;
    ep = sp.offset(limit as isize);
    nn = 0 as libc::c_int as ucs4_t;
    i = 0 as libc::c_int as ucs2_t;
    while (i as libc::c_int) < 4 as libc::c_int && sp < ep {
        if *sp as libc::c_int >= '0' as i32 &&
               *sp as libc::c_int <= '9' as i32 {
            nn =
                (nn <<
                     4 as
                         libc::c_int).wrapping_add((*sp as libc::c_int -
                                                        '0' as i32) as
                                                       libc::c_uint)
        } else if *sp as libc::c_int >= 'A' as i32 &&
                      *sp as libc::c_int <= 'F' as i32 {
            nn =
                (nn <<
                     4 as
                         libc::c_int).wrapping_add((*sp as libc::c_int -
                                                        'A' as i32 +
                                                        10 as libc::c_int) as
                                                       libc::c_uint)
        } else {
            if !(*sp as libc::c_int >= 'a' as i32 &&
                     *sp as libc::c_int <= 'f' as i32) {
                break ;
            }
            nn =
                (nn <<
                     4 as
                         libc::c_int).wrapping_add((*sp as libc::c_int -
                                                        'a' as i32 +
                                                        10 as libc::c_int) as
                                                       libc::c_uint)
        }
        i = i.wrapping_add(1);
        sp = sp.offset(1)
    }
    /*
     * Assign the character code collected and return the number of
     * characters used.
     */
    *n = nn;
    return sp.wrapping_offset_from(np) as libc::c_long as libc::c_ulong;
}
/*
 * Insert a range into a character class, removing duplicates and ordering
 * them in increasing range-start order.
 */
#[c2rust::src_loc = "479:1"]
unsafe extern "C" fn _ure_add_range(mut ccl: *mut _ure_ccl_t,
                                    mut r: *mut _ure_range_t,
                                    mut b: *mut _ure_buffer_t) {
    let mut i: ucs2_t = 0;
    let mut tmp: ucs4_t = 0;
    let mut rp: *mut _ure_range_t = 0 as *mut _ure_range_t;
    /*
     * If the `casefold' flag is set, then make sure both endpoints of the
     * range are converted to lower case.
     */
    if (*b).flags & 0x1 as libc::c_int as libc::c_ulong != 0 {
        (*r).min_code = _ure_tolower((*r).min_code);
        (*r).max_code = _ure_tolower((*r).max_code)
    }
    /*
     * Swap the range endpoints if they are not in increasing order.
     */
    if (*r).min_code > (*r).max_code {
        tmp = (*r).min_code;
        (*r).min_code = (*r).max_code;
        (*r).max_code = tmp
    }
    i = 0 as libc::c_int as ucs2_t;
    rp = (*ccl).ranges;
    while (i as libc::c_int) < (*ccl).ranges_used as libc::c_int &&
              (*r).min_code < (*rp).min_code {
        i = i.wrapping_add(1);
        rp = rp.offset(1)
    }
    /*
     * Check for a duplicate.
     */
    if (i as libc::c_int) < (*ccl).ranges_used as libc::c_int &&
           (*r).min_code == (*rp).min_code && (*r).max_code == (*rp).max_code
       {
        return
    }
    if (*ccl).ranges_used as libc::c_int == (*ccl).ranges_size as libc::c_int
       {
        if (*ccl).ranges_size as libc::c_int == 0 as libc::c_int {
            (*ccl).ranges =
                malloc((::std::mem::size_of::<_ure_range_t>() as
                            libc::c_ulong) << 3 as libc::c_int) as
                    *mut _ure_range_t
        } else {
            (*ccl).ranges =
                realloc((*ccl).ranges as *mut libc::c_char as
                            *mut libc::c_void,
                        (::std::mem::size_of::<_ure_range_t>() as
                             libc::c_ulong).wrapping_mul(((*ccl).ranges_size
                                                              as libc::c_int +
                                                              8 as
                                                                  libc::c_int)
                                                             as
                                                             libc::c_ulong))
                    as *mut _ure_range_t
        }
        (*ccl).ranges_size =
            ((*ccl).ranges_size as libc::c_int + 8 as libc::c_int) as ucs2_t
    }
    rp = (*ccl).ranges.offset((*ccl).ranges_used as libc::c_int as isize);
    if (i as libc::c_int) < (*ccl).ranges_used as libc::c_int {
        _ure_memmove(rp.offset(1 as libc::c_int as isize) as
                         *mut libc::c_char, rp as *mut libc::c_char,
                     (::std::mem::size_of::<_ure_range_t>() as
                          libc::c_ulong).wrapping_mul(((*ccl).ranges_used as
                                                           libc::c_int -
                                                           i as libc::c_int)
                                                          as libc::c_ulong));
    }
    (*ccl).ranges_used = (*ccl).ranges_used.wrapping_add(1);
    (*rp).min_code = (*r).min_code;
    (*rp).max_code = (*r).max_code;
}
#[c2rust::src_loc = "559:1"]
unsafe extern "C" fn _ure_ccl_setup(mut sym: *mut _ure_symtab_t,
                                    mut mask: libc::c_ulong,
                                    mut b: *mut _ure_buffer_t) {
    (*sym).props |= mask;
}
#[c2rust::src_loc = "565:1"]
unsafe extern "C" fn _ure_space_setup(mut sym: *mut _ure_symtab_t,
                                      mut mask: libc::c_ulong,
                                      mut b: *mut _ure_buffer_t) {
    let mut range: _ure_range_t = _ure_range_t{min_code: 0, max_code: 0,};
    (*sym).props |= mask;
    /*
     * Add the additional characters needed for handling isspace().
     */
    range.max_code = '\t' as i32 as ucs4_t;
    range.min_code = range.max_code;
    _ure_add_range(&mut (*sym).sym.ccl, &mut range, b);
    range.max_code = '\r' as i32 as ucs4_t;
    range.min_code = range.max_code;
    _ure_add_range(&mut (*sym).sym.ccl, &mut range, b);
    range.max_code = '\n' as i32 as ucs4_t;
    range.min_code = range.max_code;
    _ure_add_range(&mut (*sym).sym.ccl, &mut range, b);
    range.max_code = '\u{c}' as i32 as ucs4_t;
    range.min_code = range.max_code;
    _ure_add_range(&mut (*sym).sym.ccl, &mut range, b);
    range.max_code = 0xfeff as libc::c_int as ucs4_t;
    range.min_code = range.max_code;
    _ure_add_range(&mut (*sym).sym.ccl, &mut range, b);
}
#[c2rust::src_loc = "587:1"]
unsafe extern "C" fn _ure_xdigit_setup(mut sym: *mut _ure_symtab_t,
                                       mut mask: libc::c_ulong,
                                       mut b: *mut _ure_buffer_t) {
    let mut range: _ure_range_t = _ure_range_t{min_code: 0, max_code: 0,};
    /*
     * Add the additional characters needed for handling isxdigit().
     */
    range.min_code = '0' as i32 as ucs4_t;
    range.max_code = '9' as i32 as ucs4_t;
    _ure_add_range(&mut (*sym).sym.ccl, &mut range, b);
    range.min_code = 'A' as i32 as ucs4_t;
    range.max_code = 'F' as i32 as ucs4_t;
    _ure_add_range(&mut (*sym).sym.ccl, &mut range, b);
    range.min_code = 'a' as i32 as ucs4_t;
    range.max_code = 'f' as i32 as ucs4_t;
    _ure_add_range(&mut (*sym).sym.ccl, &mut range, b);
}
// Initialized in run_static_initializers
#[c2rust::src_loc = "606:26"]
static mut cclass_trie: [_ure_trie_t; 65] =
    [_ure_trie_t{key: 0,
                 len_next: [0; 2],
                 c2rust_padding: [0; 4],
                 func: None,
                 mask: 0,}; 65];
/*
 * Probe for one of the POSIX colon delimited character classes in the static
 * trie.
 */
#[c2rust::src_loc = "678:1"]
unsafe extern "C" fn _ure_posix_ccl(mut cp: *mut ucs2_t,
                                    mut limit: libc::c_ulong,
                                    mut sym: *mut _ure_symtab_t,
                                    mut b: *mut _ure_buffer_t)
 -> libc::c_ulong {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_ulong = 0;
    let mut tp: *const _ure_trie_t = 0 as *const _ure_trie_t;
    let mut sp: *mut ucs2_t = 0 as *mut ucs2_t;
    let mut ep: *mut ucs2_t = 0 as *mut ucs2_t;
    /*
     * If the number of characters left is less than 7, then this cannot be
     * interpreted as one of the colon delimited classes.
     */
    if limit < 7 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as libc::c_ulong
    }
    sp = cp;
    ep = sp.offset(limit as isize);
    tp = cclass_trie.as_ptr();
    i = 0 as libc::c_int;
    while sp < ep && i < 8 as libc::c_int {
        n = (*tp).len() as libc::c_ulong;
        while n > 0 as libc::c_int as libc::c_ulong &&
                  (*tp).key as libc::c_int != *sp as libc::c_int {
            tp = tp.offset(1);
            n = n.wrapping_sub(1)
        }
        if n == 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int as libc::c_ulong
        }
        if *sp as libc::c_int == ':' as i32 &&
               (i == 6 as libc::c_int || i == 7 as libc::c_int) {
            sp = sp.offset(1);
            break ;
        } else {
            if sp.offset(1 as libc::c_int as isize) < ep {
                tp =
                    cclass_trie.as_ptr().offset((*tp).next() as libc::c_int as
                                                    isize)
            }
            i += 1;
            sp = sp.offset(1)
        }
    }
    if (*tp).func.is_none() { return 0 as libc::c_int as libc::c_ulong }
    Some((*tp).func.expect("non-null function pointer")).expect("non-null function pointer")(sym,
                                                                                             (*tp).mask,
                                                                                             b);
    return sp.wrapping_offset_from(cp) as libc::c_long as libc::c_ulong;
}
/*
 * Construct a list of ranges and return the number of characters consumed.
 */
#[c2rust::src_loc = "723:1"]
unsafe extern "C" fn _ure_cclass(mut cp: *mut ucs2_t,
                                 mut limit: libc::c_ulong,
                                 mut symp: *mut _ure_symtab_t,
                                 mut b: *mut _ure_buffer_t) -> libc::c_ulong {
    let mut range_end: libc::c_int = 0;
    let mut n: libc::c_ulong = 0;
    let mut sp: *mut ucs2_t = 0 as *mut ucs2_t;
    let mut ep: *mut ucs2_t = 0 as *mut ucs2_t;
    let mut c: ucs4_t = 0;
    let mut last: ucs4_t = 0;
    let mut cclp: *mut _ure_ccl_t = 0 as *mut _ure_ccl_t;
    let mut range: _ure_range_t = _ure_range_t{min_code: 0, max_code: 0,};
    sp = cp;
    ep = sp.offset(limit as isize);
    if *sp as libc::c_int == '^' as i32 {
        (*symp).type_0 = 4 as libc::c_int as ucs2_t;
        sp = sp.offset(1)
    } else { (*symp).type_0 = 3 as libc::c_int as ucs2_t }
    let mut current_block_60: u64;
    last = 0 as libc::c_int as ucs4_t;
    range_end = 0 as libc::c_int;
    while (*b).error == 0 as libc::c_int && sp < ep &&
              *sp as libc::c_int != ']' as i32 {
        let fresh17 = sp;
        sp = sp.offset(1);
        c = *fresh17 as ucs4_t;
        if c == '\\' as i32 as libc::c_uint {
            if sp == ep {
                /*
                 * The EOS was encountered when expecting the reverse solidus
                 * to be followed by the character it is escaping.  Set an
                 * error code and return the number of characters consumed up
                 * to this point.
                 */
                (*b).error = -(1 as libc::c_int);
                return sp.wrapping_offset_from(cp) as libc::c_long as
                           libc::c_ulong
            }
            let fresh18 = sp;
            sp = sp.offset(1);
            c = *fresh18 as ucs4_t;
            match c {
                97 => {
                    current_block_60 = 5425920993883413897;
                    match current_block_60 {
                        10202095235443474371 => {
                            if sp < ep &&
                                   (*sp as libc::c_int >= '0' as i32 &&
                                        *sp as libc::c_int <= '9' as i32 ||
                                        *sp as libc::c_int >= 'A' as i32 &&
                                            *sp as libc::c_int <= 'F' as i32
                                        ||
                                        *sp as libc::c_int >= 'a' as i32 &&
                                            *sp as libc::c_int <= 'f' as i32)
                               {
                                sp =
                                    sp.offset(_ure_hex(sp,
                                                       ep.wrapping_offset_from(sp)
                                                           as libc::c_long as
                                                           libc::c_ulong,
                                                       &mut c) as isize)
                            }
                        }
                        5425920993883413897 => {
                            c = 0x7 as libc::c_int as ucs4_t
                        }
                        11304045863834688353 => {
                            c = 0x8 as libc::c_int as ucs4_t
                        }
                        2991706011871633112 => {
                            c = 0xc as libc::c_int as ucs4_t
                        }
                        10116763051596782344 => {
                            c = 0xa as libc::c_int as ucs4_t
                        }
                        6685896254725134263 => {
                            c = 0xd as libc::c_int as ucs4_t
                        }
                        17601247583483846226 => {
                            c = 0x9 as libc::c_int as ucs4_t
                        }
                        2042511421509206405 => {
                            c = 0xb as libc::c_int as ucs4_t
                        }
                        _ => {
                            sp =
                                sp.offset(_ure_prop_list(sp,
                                                         ep.wrapping_offset_from(sp)
                                                             as libc::c_long
                                                             as libc::c_ulong,
                                                         &mut (*symp).props,
                                                         b) as isize);
                            /*
                 * Invert the bit mask of the properties if this is a negated
                 * character class or if 'P' is used to specify a list of
                 * character properties that should *not* match in a
                 * character class.
                 */
                            if c == 'P' as i32 as libc::c_uint {
                                (*symp).props = !(*symp).props
                            }
                            continue ;
                        }
                    }
                }
                98 => {
                    current_block_60 = 11304045863834688353;
                    match current_block_60 {
                        10202095235443474371 => {
                            if sp < ep &&
                                   (*sp as libc::c_int >= '0' as i32 &&
                                        *sp as libc::c_int <= '9' as i32 ||
                                        *sp as libc::c_int >= 'A' as i32 &&
                                            *sp as libc::c_int <= 'F' as i32
                                        ||
                                        *sp as libc::c_int >= 'a' as i32 &&
                                            *sp as libc::c_int <= 'f' as i32)
                               {
                                sp =
                                    sp.offset(_ure_hex(sp,
                                                       ep.wrapping_offset_from(sp)
                                                           as libc::c_long as
                                                           libc::c_ulong,
                                                       &mut c) as isize)
                            }
                        }
                        5425920993883413897 => {
                            c = 0x7 as libc::c_int as ucs4_t
                        }
                        11304045863834688353 => {
                            c = 0x8 as libc::c_int as ucs4_t
                        }
                        2991706011871633112 => {
                            c = 0xc as libc::c_int as ucs4_t
                        }
                        10116763051596782344 => {
                            c = 0xa as libc::c_int as ucs4_t
                        }
                        6685896254725134263 => {
                            c = 0xd as libc::c_int as ucs4_t
                        }
                        17601247583483846226 => {
                            c = 0x9 as libc::c_int as ucs4_t
                        }
                        2042511421509206405 => {
                            c = 0xb as libc::c_int as ucs4_t
                        }
                        _ => {
                            sp =
                                sp.offset(_ure_prop_list(sp,
                                                         ep.wrapping_offset_from(sp)
                                                             as libc::c_long
                                                             as libc::c_ulong,
                                                         &mut (*symp).props,
                                                         b) as isize);
                            if c == 'P' as i32 as libc::c_uint {
                                (*symp).props = !(*symp).props
                            }
                            continue ;
                        }
                    }
                }
                102 => {
                    current_block_60 = 2991706011871633112;
                    match current_block_60 {
                        10202095235443474371 => {
                            if sp < ep &&
                                   (*sp as libc::c_int >= '0' as i32 &&
                                        *sp as libc::c_int <= '9' as i32 ||
                                        *sp as libc::c_int >= 'A' as i32 &&
                                            *sp as libc::c_int <= 'F' as i32
                                        ||
                                        *sp as libc::c_int >= 'a' as i32 &&
                                            *sp as libc::c_int <= 'f' as i32)
                               {
                                sp =
                                    sp.offset(_ure_hex(sp,
                                                       ep.wrapping_offset_from(sp)
                                                           as libc::c_long as
                                                           libc::c_ulong,
                                                       &mut c) as isize)
                            }
                        }
                        5425920993883413897 => {
                            c = 0x7 as libc::c_int as ucs4_t
                        }
                        11304045863834688353 => {
                            c = 0x8 as libc::c_int as ucs4_t
                        }
                        2991706011871633112 => {
                            c = 0xc as libc::c_int as ucs4_t
                        }
                        10116763051596782344 => {
                            c = 0xa as libc::c_int as ucs4_t
                        }
                        6685896254725134263 => {
                            c = 0xd as libc::c_int as ucs4_t
                        }
                        17601247583483846226 => {
                            c = 0x9 as libc::c_int as ucs4_t
                        }
                        2042511421509206405 => {
                            c = 0xb as libc::c_int as ucs4_t
                        }
                        _ => {
                            sp =
                                sp.offset(_ure_prop_list(sp,
                                                         ep.wrapping_offset_from(sp)
                                                             as libc::c_long
                                                             as libc::c_ulong,
                                                         &mut (*symp).props,
                                                         b) as isize);
                            if c == 'P' as i32 as libc::c_uint {
                                (*symp).props = !(*symp).props
                            }
                            continue ;
                        }
                    }
                }
                110 => {
                    current_block_60 = 10116763051596782344;
                    match current_block_60 {
                        10202095235443474371 => {
                            if sp < ep &&
                                   (*sp as libc::c_int >= '0' as i32 &&
                                        *sp as libc::c_int <= '9' as i32 ||
                                        *sp as libc::c_int >= 'A' as i32 &&
                                            *sp as libc::c_int <= 'F' as i32
                                        ||
                                        *sp as libc::c_int >= 'a' as i32 &&
                                            *sp as libc::c_int <= 'f' as i32)
                               {
                                sp =
                                    sp.offset(_ure_hex(sp,
                                                       ep.wrapping_offset_from(sp)
                                                           as libc::c_long as
                                                           libc::c_ulong,
                                                       &mut c) as isize)
                            }
                        }
                        5425920993883413897 => {
                            c = 0x7 as libc::c_int as ucs4_t
                        }
                        11304045863834688353 => {
                            c = 0x8 as libc::c_int as ucs4_t
                        }
                        2991706011871633112 => {
                            c = 0xc as libc::c_int as ucs4_t
                        }
                        10116763051596782344 => {
                            c = 0xa as libc::c_int as ucs4_t
                        }
                        6685896254725134263 => {
                            c = 0xd as libc::c_int as ucs4_t
                        }
                        17601247583483846226 => {
                            c = 0x9 as libc::c_int as ucs4_t
                        }
                        2042511421509206405 => {
                            c = 0xb as libc::c_int as ucs4_t
                        }
                        _ => {
                            sp =
                                sp.offset(_ure_prop_list(sp,
                                                         ep.wrapping_offset_from(sp)
                                                             as libc::c_long
                                                             as libc::c_ulong,
                                                         &mut (*symp).props,
                                                         b) as isize);
                            if c == 'P' as i32 as libc::c_uint {
                                (*symp).props = !(*symp).props
                            }
                            continue ;
                        }
                    }
                }
                114 => {
                    current_block_60 = 6685896254725134263;
                    match current_block_60 {
                        10202095235443474371 => {
                            if sp < ep &&
                                   (*sp as libc::c_int >= '0' as i32 &&
                                        *sp as libc::c_int <= '9' as i32 ||
                                        *sp as libc::c_int >= 'A' as i32 &&
                                            *sp as libc::c_int <= 'F' as i32
                                        ||
                                        *sp as libc::c_int >= 'a' as i32 &&
                                            *sp as libc::c_int <= 'f' as i32)
                               {
                                sp =
                                    sp.offset(_ure_hex(sp,
                                                       ep.wrapping_offset_from(sp)
                                                           as libc::c_long as
                                                           libc::c_ulong,
                                                       &mut c) as isize)
                            }
                        }
                        5425920993883413897 => {
                            c = 0x7 as libc::c_int as ucs4_t
                        }
                        11304045863834688353 => {
                            c = 0x8 as libc::c_int as ucs4_t
                        }
                        2991706011871633112 => {
                            c = 0xc as libc::c_int as ucs4_t
                        }
                        10116763051596782344 => {
                            c = 0xa as libc::c_int as ucs4_t
                        }
                        6685896254725134263 => {
                            c = 0xd as libc::c_int as ucs4_t
                        }
                        17601247583483846226 => {
                            c = 0x9 as libc::c_int as ucs4_t
                        }
                        2042511421509206405 => {
                            c = 0xb as libc::c_int as ucs4_t
                        }
                        _ => {
                            sp =
                                sp.offset(_ure_prop_list(sp,
                                                         ep.wrapping_offset_from(sp)
                                                             as libc::c_long
                                                             as libc::c_ulong,
                                                         &mut (*symp).props,
                                                         b) as isize);
                            if c == 'P' as i32 as libc::c_uint {
                                (*symp).props = !(*symp).props
                            }
                            continue ;
                        }
                    }
                }
                116 => {
                    current_block_60 = 17601247583483846226;
                    match current_block_60 {
                        10202095235443474371 => {
                            if sp < ep &&
                                   (*sp as libc::c_int >= '0' as i32 &&
                                        *sp as libc::c_int <= '9' as i32 ||
                                        *sp as libc::c_int >= 'A' as i32 &&
                                            *sp as libc::c_int <= 'F' as i32
                                        ||
                                        *sp as libc::c_int >= 'a' as i32 &&
                                            *sp as libc::c_int <= 'f' as i32)
                               {
                                sp =
                                    sp.offset(_ure_hex(sp,
                                                       ep.wrapping_offset_from(sp)
                                                           as libc::c_long as
                                                           libc::c_ulong,
                                                       &mut c) as isize)
                            }
                        }
                        5425920993883413897 => {
                            c = 0x7 as libc::c_int as ucs4_t
                        }
                        11304045863834688353 => {
                            c = 0x8 as libc::c_int as ucs4_t
                        }
                        2991706011871633112 => {
                            c = 0xc as libc::c_int as ucs4_t
                        }
                        10116763051596782344 => {
                            c = 0xa as libc::c_int as ucs4_t
                        }
                        6685896254725134263 => {
                            c = 0xd as libc::c_int as ucs4_t
                        }
                        17601247583483846226 => {
                            c = 0x9 as libc::c_int as ucs4_t
                        }
                        2042511421509206405 => {
                            c = 0xb as libc::c_int as ucs4_t
                        }
                        _ => {
                            sp =
                                sp.offset(_ure_prop_list(sp,
                                                         ep.wrapping_offset_from(sp)
                                                             as libc::c_long
                                                             as libc::c_ulong,
                                                         &mut (*symp).props,
                                                         b) as isize);
                            if c == 'P' as i32 as libc::c_uint {
                                (*symp).props = !(*symp).props
                            }
                            continue ;
                        }
                    }
                }
                118 => {
                    current_block_60 = 2042511421509206405;
                    match current_block_60 {
                        10202095235443474371 => {
                            if sp < ep &&
                                   (*sp as libc::c_int >= '0' as i32 &&
                                        *sp as libc::c_int <= '9' as i32 ||
                                        *sp as libc::c_int >= 'A' as i32 &&
                                            *sp as libc::c_int <= 'F' as i32
                                        ||
                                        *sp as libc::c_int >= 'a' as i32 &&
                                            *sp as libc::c_int <= 'f' as i32)
                               {
                                sp =
                                    sp.offset(_ure_hex(sp,
                                                       ep.wrapping_offset_from(sp)
                                                           as libc::c_long as
                                                           libc::c_ulong,
                                                       &mut c) as isize)
                            }
                        }
                        5425920993883413897 => {
                            c = 0x7 as libc::c_int as ucs4_t
                        }
                        11304045863834688353 => {
                            c = 0x8 as libc::c_int as ucs4_t
                        }
                        2991706011871633112 => {
                            c = 0xc as libc::c_int as ucs4_t
                        }
                        10116763051596782344 => {
                            c = 0xa as libc::c_int as ucs4_t
                        }
                        6685896254725134263 => {
                            c = 0xd as libc::c_int as ucs4_t
                        }
                        17601247583483846226 => {
                            c = 0x9 as libc::c_int as ucs4_t
                        }
                        2042511421509206405 => {
                            c = 0xb as libc::c_int as ucs4_t
                        }
                        _ => {
                            sp =
                                sp.offset(_ure_prop_list(sp,
                                                         ep.wrapping_offset_from(sp)
                                                             as libc::c_long
                                                             as libc::c_ulong,
                                                         &mut (*symp).props,
                                                         b) as isize);
                            if c == 'P' as i32 as libc::c_uint {
                                (*symp).props = !(*symp).props
                            }
                            continue ;
                        }
                    }
                }
                112 | 80 => {
                    current_block_60 = 8090688287736135867;
                    match current_block_60 {
                        10202095235443474371 => {
                            if sp < ep &&
                                   (*sp as libc::c_int >= '0' as i32 &&
                                        *sp as libc::c_int <= '9' as i32 ||
                                        *sp as libc::c_int >= 'A' as i32 &&
                                            *sp as libc::c_int <= 'F' as i32
                                        ||
                                        *sp as libc::c_int >= 'a' as i32 &&
                                            *sp as libc::c_int <= 'f' as i32)
                               {
                                sp =
                                    sp.offset(_ure_hex(sp,
                                                       ep.wrapping_offset_from(sp)
                                                           as libc::c_long as
                                                           libc::c_ulong,
                                                       &mut c) as isize)
                            }
                        }
                        5425920993883413897 => {
                            c = 0x7 as libc::c_int as ucs4_t
                        }
                        11304045863834688353 => {
                            c = 0x8 as libc::c_int as ucs4_t
                        }
                        2991706011871633112 => {
                            c = 0xc as libc::c_int as ucs4_t
                        }
                        10116763051596782344 => {
                            c = 0xa as libc::c_int as ucs4_t
                        }
                        6685896254725134263 => {
                            c = 0xd as libc::c_int as ucs4_t
                        }
                        17601247583483846226 => {
                            c = 0x9 as libc::c_int as ucs4_t
                        }
                        2042511421509206405 => {
                            c = 0xb as libc::c_int as ucs4_t
                        }
                        _ => {
                            sp =
                                sp.offset(_ure_prop_list(sp,
                                                         ep.wrapping_offset_from(sp)
                                                             as libc::c_long
                                                             as libc::c_ulong,
                                                         &mut (*symp).props,
                                                         b) as isize);
                            if c == 'P' as i32 as libc::c_uint {
                                (*symp).props = !(*symp).props
                            }
                            continue ;
                        }
                    }
                }
                120 | 88 | 117 | 85 => {
                    current_block_60 = 10202095235443474371;
                    match current_block_60 {
                        10202095235443474371 => {
                            if sp < ep &&
                                   (*sp as libc::c_int >= '0' as i32 &&
                                        *sp as libc::c_int <= '9' as i32 ||
                                        *sp as libc::c_int >= 'A' as i32 &&
                                            *sp as libc::c_int <= 'F' as i32
                                        ||
                                        *sp as libc::c_int >= 'a' as i32 &&
                                            *sp as libc::c_int <= 'f' as i32)
                               {
                                sp =
                                    sp.offset(_ure_hex(sp,
                                                       ep.wrapping_offset_from(sp)
                                                           as libc::c_long as
                                                           libc::c_ulong,
                                                       &mut c) as isize)
                            }
                        }
                        5425920993883413897 => {
                            c = 0x7 as libc::c_int as ucs4_t
                        }
                        11304045863834688353 => {
                            c = 0x8 as libc::c_int as ucs4_t
                        }
                        2991706011871633112 => {
                            c = 0xc as libc::c_int as ucs4_t
                        }
                        10116763051596782344 => {
                            c = 0xa as libc::c_int as ucs4_t
                        }
                        6685896254725134263 => {
                            c = 0xd as libc::c_int as ucs4_t
                        }
                        17601247583483846226 => {
                            c = 0x9 as libc::c_int as ucs4_t
                        }
                        2042511421509206405 => {
                            c = 0xb as libc::c_int as ucs4_t
                        }
                        _ => {
                            sp =
                                sp.offset(_ure_prop_list(sp,
                                                         ep.wrapping_offset_from(sp)
                                                             as libc::c_long
                                                             as libc::c_ulong,
                                                         &mut (*symp).props,
                                                         b) as isize);
                            if c == 'P' as i32 as libc::c_uint {
                                (*symp).props = !(*symp).props
                            }
                            continue ;
                        }
                    }
                }
                _ => { }
            }
        } else if c == ':' as i32 as libc::c_uint {
            /*
             * Probe for a POSIX colon delimited character class.
             */
            sp = sp.offset(-1);
            n =
                _ure_posix_ccl(sp,
                               ep.wrapping_offset_from(sp) as libc::c_long as
                                   libc::c_ulong, symp, b);
            if n == 0 as libc::c_int as libc::c_ulong {
                sp = sp.offset(1)
            } else { sp = sp.offset(n as isize); continue ; }
        }
        cclp = &mut (*symp).sym.ccl;
        /*
         * Check to see if the current character is a low surrogate that needs
         * to be combined with a preceding high surrogate.
         */
        if last != 0 as libc::c_int as libc::c_uint {
            if c >= 0xdc00 as libc::c_int as libc::c_uint &&
                   c <= 0xdfff as libc::c_int as libc::c_uint {
                /*
               * Construct the UTF16 character code.
               */
                c =
                    (0x10000 as libc::c_int as
                         libc::c_uint).wrapping_add((last &
                                                         0x3ff as libc::c_int
                                                             as libc::c_uint)
                                                        << 10 as libc::c_int |
                                                        c &
                                                            0x3ff as
                                                                libc::c_int as
                                                                libc::c_uint)
            } else {
                /*
                 * Add the isolated high surrogate to the range.
                 */
                if range_end == 1 as libc::c_int {
                    range.max_code =
                        last & 0xffff as libc::c_int as libc::c_uint
                } else {
                    range.max_code =
                        last & 0xffff as libc::c_int as libc::c_uint;
                    range.min_code = range.max_code
                }
                _ure_add_range(cclp, &mut range, b);
                range_end = 0 as libc::c_int
            }
        }
        /*
         * Clear the last character code.
         */
        last = 0 as libc::c_int as ucs4_t;
        /*
         * This slightly awkward code handles the different cases needed to
         * construct a range.
         */
        if c >= 0xd800 as libc::c_int as libc::c_uint &&
               c <= 0xdbff as libc::c_int as libc::c_uint {
            /*
             * If the high surrogate is followed by a range indicator, simply
             * add it as the range start.  Otherwise, save it in case the next
             * character is a low surrogate.
             */
            if *sp as libc::c_int == '-' as i32 {
                sp = sp.offset(1);
                range.min_code = c;
                range_end = 1 as libc::c_int
            } else { last = c }
        } else if range_end == 1 as libc::c_int {
            range.max_code = c;
            _ure_add_range(cclp, &mut range, b);
            range_end = 0 as libc::c_int
        } else {
            range.max_code = c;
            range.min_code = range.max_code;
            if *sp as libc::c_int == '-' as i32 {
                sp = sp.offset(1);
                range_end = 1 as libc::c_int
            } else { _ure_add_range(cclp, &mut range, b); }
        }
    }
    if sp < ep && *sp as libc::c_int == ']' as i32 {
        sp = sp.offset(1)
    } else {
        /*
       * The parse was not terminated by the character class close symbol
       * (']'), so set an error code.
       */
        (*b).error = -(2 as libc::c_int)
    }
    return sp.wrapping_offset_from(cp) as libc::c_long as libc::c_ulong;
}
/*
 * Probe for a low surrogate hex code.
 */
#[c2rust::src_loc = "893:1"]
unsafe extern "C" fn _ure_probe_ls(mut ls: *mut ucs2_t,
                                   mut limit: libc::c_ulong,
                                   mut c: *mut ucs4_t) -> libc::c_ulong {
    let mut i: ucs4_t = 0;
    let mut code: ucs4_t = 0;
    let mut sp: *mut ucs2_t = 0 as *mut ucs2_t;
    let mut ep: *mut ucs2_t = 0 as *mut ucs2_t;
    code = 0 as libc::c_int as ucs4_t;
    i = code;
    sp = ls;
    ep = sp.offset(limit as isize);
    while i < 4 as libc::c_int as libc::c_uint && sp < ep {
        if *sp as libc::c_int >= '0' as i32 &&
               *sp as libc::c_int <= '9' as i32 {
            code =
                (code <<
                     4 as
                         libc::c_int).wrapping_add((*sp as libc::c_int -
                                                        '0' as i32) as
                                                       libc::c_uint)
        } else if *sp as libc::c_int >= 'A' as i32 &&
                      *sp as libc::c_int <= 'F' as i32 {
            code =
                (code <<
                     4 as
                         libc::c_int).wrapping_add((*sp as libc::c_int -
                                                        'A' as i32 +
                                                        10 as libc::c_int) as
                                                       libc::c_uint)
        } else {
            if !(*sp as libc::c_int >= 'a' as i32 &&
                     *sp as libc::c_int <= 'f' as i32) {
                break ;
            }
            code =
                (code <<
                     4 as
                         libc::c_int).wrapping_add((*sp as libc::c_int -
                                                        'a' as i32 +
                                                        10 as libc::c_int) as
                                                       libc::c_uint)
        }
        sp = sp.offset(1)
    }
    *c = code;
    return if 0xdc00 as libc::c_int as libc::c_uint <= code &&
                  code <= 0xdfff as libc::c_int as libc::c_uint {
               sp.wrapping_offset_from(ls) as libc::c_long
           } else { 0 as libc::c_int as libc::c_long } as libc::c_ulong;
}
#[c2rust::src_loc = "914:1"]
unsafe extern "C" fn _ure_compile_symbol(mut sym: *mut ucs2_t,
                                         mut limit: libc::c_ulong,
                                         mut symp: *mut _ure_symtab_t,
                                         mut b: *mut _ure_buffer_t)
 -> libc::c_ulong {
    let mut c: ucs4_t = 0;
    let mut sp: *mut ucs2_t = 0 as *mut ucs2_t;
    let mut ep: *mut ucs2_t = 0 as *mut ucs2_t;
    sp = sym;
    ep = sym.offset(limit as isize);
    let fresh19 = sp;
    sp = sp.offset(1);
    c = *fresh19 as ucs4_t;
    if c == '\\' as i32 as libc::c_uint {
        if sp == ep {
            /*
             * The EOS was encountered when expecting the reverse solidus to
             * be followed by the character it is escaping.  Set an error code
             * and return the number of characters consumed up to this point.
             */
            (*b).error = -(1 as libc::c_int);
            return sp.wrapping_offset_from(sym) as libc::c_long as
                       libc::c_ulong
        }
        let fresh20 = sp;
        sp = sp.offset(1);
        c = *fresh20 as ucs4_t;
        let mut current_block_26: u64;
        match c {
            112 | 80 => {
                (*symp).type_0 =
                    if c == 'p' as i32 as libc::c_uint {
                        3 as libc::c_int
                    } else { 4 as libc::c_int } as ucs2_t;
                sp =
                    sp.offset(_ure_prop_list(sp,
                                             ep.wrapping_offset_from(sp) as
                                                 libc::c_long as
                                                 libc::c_ulong,
                                             &mut (*symp).props, b) as isize);
                current_block_26 = 14434620278749266018;
            }
            97 => {
                (*symp).type_0 = 2 as libc::c_int as ucs2_t;
                (*symp).sym.chr = 0x7 as libc::c_int as ucs4_t;
                current_block_26 = 14434620278749266018;
            }
            98 => {
                (*symp).type_0 = 2 as libc::c_int as ucs2_t;
                (*symp).sym.chr = 0x8 as libc::c_int as ucs4_t;
                current_block_26 = 14434620278749266018;
            }
            102 => {
                (*symp).type_0 = 2 as libc::c_int as ucs2_t;
                (*symp).sym.chr = 0xc as libc::c_int as ucs4_t;
                current_block_26 = 14434620278749266018;
            }
            110 => {
                (*symp).type_0 = 2 as libc::c_int as ucs2_t;
                (*symp).sym.chr = 0xa as libc::c_int as ucs4_t;
                current_block_26 = 14434620278749266018;
            }
            114 => {
                (*symp).type_0 = 2 as libc::c_int as ucs2_t;
                (*symp).sym.chr = 0xd as libc::c_int as ucs4_t;
                current_block_26 = 14434620278749266018;
            }
            116 => {
                (*symp).type_0 = 2 as libc::c_int as ucs2_t;
                (*symp).sym.chr = 0x9 as libc::c_int as ucs4_t;
                current_block_26 = 14434620278749266018;
            }
            118 => {
                (*symp).type_0 = 2 as libc::c_int as ucs2_t;
                (*symp).sym.chr = 0xb as libc::c_int as ucs4_t;
                current_block_26 = 14434620278749266018;
            }
            120 | 88 | 117 | 85 => {
                /*
             * Collect between 1 and 4 digits representing a UCS2 code.  Fall
             * through to the next case.
             */
                if sp < ep &&
                       (*sp as libc::c_int >= '0' as i32 &&
                            *sp as libc::c_int <= '9' as i32 ||
                            *sp as libc::c_int >= 'A' as i32 &&
                                *sp as libc::c_int <= 'F' as i32 ||
                            *sp as libc::c_int >= 'a' as i32 &&
                                *sp as libc::c_int <= 'f' as i32) {
                    sp =
                        sp.offset(_ure_hex(sp,
                                           ep.wrapping_offset_from(sp) as
                                               libc::c_long as libc::c_ulong,
                                           &mut c) as isize)
                }
                current_block_26 = 2373645303499600572;
            }
            _ => { current_block_26 = 2373645303499600572; }
        }
        match current_block_26 {
            2373645303499600572 =>
            /* FALLTHROUGH */
            /*
             * Simply add an escaped character here.
             */
            {
                (*symp).type_0 = 2 as libc::c_int as ucs2_t;
                (*symp).sym.chr = c
            }
            _ => { }
        }
    } else if c == '^' as i32 as libc::c_uint ||
                  c == '$' as i32 as libc::c_uint {
        /*
       * Handle the BOL and EOL anchors.  This actually consists simply of
       * setting a flag that indicates that the user supplied anchor match
       * function should be called.  This needs to be done instead of simply
       * matching line/paragraph separators because beginning-of-text and
       * end-of-text tests are needed as well.
       */
        (*symp).type_0 =
            if c == '^' as i32 as libc::c_uint {
                5 as libc::c_int
            } else { 6 as libc::c_int } as ucs2_t
    } else if c == '[' as i32 as libc::c_uint {
        /*
       * Construct a character class.
       */
        sp =
            sp.offset(_ure_cclass(sp,
                                  ep.wrapping_offset_from(sp) as libc::c_long
                                      as libc::c_ulong, symp, b) as isize)
    } else if c == '.' as i32 as libc::c_uint {
        (*symp).type_0 = 1 as libc::c_int as ucs2_t
    } else {
        (*symp).type_0 = 2 as libc::c_int as ucs2_t;
        (*symp).sym.chr = c
    }
    /*
     * If the symbol type happens to be a character and is a high surrogate,
     * then probe forward to see if it is followed by a low surrogate that
     * needs to be added.
     */
    if sp < ep && (*symp).type_0 as libc::c_int == 2 as libc::c_int &&
           0xd800 as libc::c_int as libc::c_uint <= (*symp).sym.chr &&
           (*symp).sym.chr <= 0xdbff as libc::c_int as libc::c_uint {
        if 0xdc00 as libc::c_int <= *sp as libc::c_int &&
               *sp as libc::c_int <= 0xdfff as libc::c_int {
            (*symp).sym.chr =
                (0x10000 as libc::c_int as
                     libc::c_uint).wrapping_add(((*symp).sym.chr &
                                                     0x3ff as libc::c_int as
                                                         libc::c_uint) <<
                                                    10 as libc::c_int |
                                                    (*sp as libc::c_int &
                                                         0x3ff as libc::c_int)
                                                        as libc::c_uint);
            sp = sp.offset(1)
        } else if *sp as libc::c_int == '\\' as i32 &&
                      (*sp.offset(1 as libc::c_int as isize) as libc::c_int ==
                           'x' as i32 ||
                           *sp.offset(1 as libc::c_int as isize) as
                               libc::c_int == 'X' as i32 ||
                           *sp.offset(1 as libc::c_int as isize) as
                               libc::c_int == 'u' as i32 ||
                           *sp.offset(1 as libc::c_int as isize) as
                               libc::c_int == 'U' as i32) {
            sp =
                sp.offset(_ure_probe_ls(sp.offset(2 as libc::c_int as isize),
                                        ep.wrapping_offset_from(sp.offset(2 as
                                                                              libc::c_int
                                                                              as
                                                                              isize))
                                            as libc::c_long as libc::c_ulong,
                                        &mut c) as isize);
            if 0xdc00 as libc::c_int as libc::c_uint <= c &&
                   c <= 0xdfff as libc::c_int as libc::c_uint {
                /*
                 * Take into account the \[xu] in front of the hex code.
                 */
                sp = sp.offset(2 as libc::c_int as isize);
                (*symp).sym.chr =
                    (0x10000 as libc::c_int as
                         libc::c_uint).wrapping_add(((*symp).sym.chr &
                                                         0x3ff as libc::c_int
                                                             as libc::c_uint)
                                                        << 10 as libc::c_int |
                                                        c &
                                                            0x3ff as
                                                                libc::c_int as
                                                                libc::c_uint)
            }
        }
    }
    /*
     * Last, make sure any _URE_CHAR type symbols are changed to lower case if
     * the `casefold' flag is set.
     */
    if (*b).flags & 0x1 as libc::c_int as libc::c_ulong != 0 &&
           (*symp).type_0 as libc::c_int == 2 as libc::c_int {
        (*symp).sym.chr = _ure_tolower((*symp).sym.chr)
    }
    /*
     * If the symbol constructed is anything other than one of the anchors,
     * make sure the _URE_DFA_BLANKLINE flag is removed.
     */
    if (*symp).type_0 as libc::c_int != 5 as libc::c_int &&
           (*symp).type_0 as libc::c_int != 6 as libc::c_int {
        (*b).flags &= !(0x2 as libc::c_int) as libc::c_ulong
    }
    /*
     * Return the number of characters consumed.
     */
    return sp.wrapping_offset_from(sym) as libc::c_long as libc::c_ulong;
}
#[c2rust::src_loc = "1059:1"]
unsafe extern "C" fn _ure_sym_neq(mut a: *mut _ure_symtab_t,
                                  mut b: *mut _ure_symtab_t) -> libc::c_int {
    if (*a).type_0 as libc::c_int != (*b).type_0 as libc::c_int ||
           (*a).mods != (*b).mods || (*a).props != (*b).props {
        return 1 as libc::c_int
    }
    if (*a).type_0 as libc::c_int == 3 as libc::c_int ||
           (*a).type_0 as libc::c_int == 4 as libc::c_int {
        if (*a).sym.ccl.ranges_used as libc::c_int !=
               (*b).sym.ccl.ranges_used as libc::c_int {
            return 1 as libc::c_int
        }
        if (*a).sym.ccl.ranges_used as libc::c_int > 0 as libc::c_int &&
               memcmp((*a).sym.ccl.ranges as *mut libc::c_char as
                          *const libc::c_void,
                      (*b).sym.ccl.ranges as *mut libc::c_char as
                          *const libc::c_void,
                      (::std::mem::size_of::<_ure_range_t>() as
                           libc::c_ulong).wrapping_mul((*a).sym.ccl.ranges_used
                                                           as libc::c_ulong))
                   != 0 as libc::c_int {
            return 1 as libc::c_int
        }
    } else if (*a).type_0 as libc::c_int == 2 as libc::c_int &&
                  (*a).sym.chr != (*b).sym.chr {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/*
 * Construct a symbol, but only keep unique symbols.
 */
#[c2rust::src_loc = "1080:1"]
unsafe extern "C" fn _ure_make_symbol(mut sym: *mut ucs2_t,
                                      mut limit: libc::c_ulong,
                                      mut consumed: *mut libc::c_ulong,
                                      mut b: *mut _ure_buffer_t) -> ucs2_t {
    let mut i: ucs2_t = 0;
    let mut sp: *mut _ure_symtab_t = 0 as *mut _ure_symtab_t;
    let mut symbol: _ure_symtab_t =
        _ure_symtab_t{id: 0,
                      type_0: 0,
                      mods: 0,
                      props: 0,
                      sym: _ure_sym_t{chr: 0,},
                      states:
                          _ure_stlist_t{slist: 0 as *mut ucs2_t,
                                        slist_size: 0,
                                        slist_used: 0,},};
    /*
     * Build the next symbol so we can test to see if it is already in the
     * symbol table.
     */
    memset(&mut symbol as *mut _ure_symtab_t as *mut libc::c_char as
               *mut libc::c_void, '\u{0}' as i32,
           ::std::mem::size_of::<_ure_symtab_t>() as libc::c_ulong);
    *consumed = _ure_compile_symbol(sym, limit, &mut symbol, b);
    /*
     * Check to see if the symbol exists.
     */
    i = 0 as libc::c_int as ucs2_t;
    sp = (*b).symtab;
    while (i as libc::c_int) < (*b).symtab_used as libc::c_int &&
              _ure_sym_neq(&mut symbol, sp) != 0 {
        i = i.wrapping_add(1);
        sp = sp.offset(1)
    }
    if (i as libc::c_int) < (*b).symtab_used as libc::c_int {
        /*
         * Free up any ranges used for the symbol.
         */
        if (symbol.type_0 as libc::c_int == 3 as libc::c_int ||
                symbol.type_0 as libc::c_int == 4 as libc::c_int) &&
               symbol.sym.ccl.ranges_size as libc::c_int > 0 as libc::c_int {
            free(symbol.sym.ccl.ranges as *mut libc::c_char as
                     *mut libc::c_void);
        }
        return (*(*b).symtab.offset(i as isize)).id
    }
    /*
     * Need to add the new symbol.
     */
    if (*b).symtab_used as libc::c_int == (*b).symtab_size as libc::c_int {
        if (*b).symtab_size as libc::c_int == 0 as libc::c_int {
            (*b).symtab =
                malloc((::std::mem::size_of::<_ure_symtab_t>() as
                            libc::c_ulong) << 3 as libc::c_int) as
                    *mut _ure_symtab_t
        } else {
            (*b).symtab =
                realloc((*b).symtab as *mut libc::c_char as *mut libc::c_void,
                        (::std::mem::size_of::<_ure_symtab_t>() as
                             libc::c_ulong).wrapping_mul(((*b).symtab_size as
                                                              libc::c_int +
                                                              8 as
                                                                  libc::c_int)
                                                             as
                                                             libc::c_ulong))
                    as *mut _ure_symtab_t
        }
        sp = (*b).symtab.offset((*b).symtab_size as libc::c_int as isize);
        memset(sp as *mut libc::c_char as *mut libc::c_void, '\u{0}' as i32,
               (::std::mem::size_of::<_ure_symtab_t>() as libc::c_ulong) <<
                   3 as libc::c_int);
        (*b).symtab_size =
            ((*b).symtab_size as libc::c_int + 8 as libc::c_int) as ucs2_t
    }
    let fresh21 = (*b).symtab_used;
    (*b).symtab_used = (*b).symtab_used.wrapping_add(1);
    symbol.id = fresh21;
    memcpy(&mut *(*b).symtab.offset(symbol.id as isize) as *mut _ure_symtab_t
               as *mut libc::c_char as *mut libc::c_void,
           &mut symbol as *mut _ure_symtab_t as *mut libc::c_char as
               *const libc::c_void,
           ::std::mem::size_of::<_ure_symtab_t>() as libc::c_ulong);
    return symbol.id;
}
/* ************************************************************************
 *
 * End symbol parse functions.
 *
 *************************************************************************/
#[c2rust::src_loc = "1139:1"]
unsafe extern "C" fn _ure_make_expr(mut type_0: ucs2_t, mut lhs: ucs2_t,
                                    mut rhs: ucs2_t,
                                    mut b: *mut _ure_buffer_t) -> ucs2_t {
    let mut i: ucs2_t = 0;
    if b.is_null() { return 0xffff as libc::c_int as ucs2_t }
    /*
     * Determine if the expression already exists or not.
     */
    i = 0 as libc::c_int as ucs2_t;
    while (i as libc::c_int) < (*b).expr_used as libc::c_int {
        if (*(*b).expr.offset(i as isize)).type_0 as libc::c_int ==
               type_0 as libc::c_int &&
               (*(*b).expr.offset(i as isize)).lhs as libc::c_int ==
                   lhs as libc::c_int &&
               (*(*b).expr.offset(i as isize)).rhs as libc::c_int ==
                   rhs as libc::c_int {
            break ;
        }
        i = i.wrapping_add(1)
    }
    if (i as libc::c_int) < (*b).expr_used as libc::c_int { return i }
    /*
     * Need to add a new expression.
     */
    if (*b).expr_used as libc::c_int == (*b).expr_size as libc::c_int {
        if (*b).expr_size as libc::c_int == 0 as libc::c_int {
            (*b).expr =
                malloc((::std::mem::size_of::<_ure_elt_t>() as libc::c_ulong)
                           << 3 as libc::c_int) as *mut _ure_elt_t
        } else {
            (*b).expr =
                realloc((*b).expr as *mut libc::c_char as *mut libc::c_void,
                        (::std::mem::size_of::<_ure_elt_t>() as
                             libc::c_ulong).wrapping_mul(((*b).expr_size as
                                                              libc::c_int +
                                                              8 as
                                                                  libc::c_int)
                                                             as
                                                             libc::c_ulong))
                    as *mut _ure_elt_t
        }
        (*b).expr_size =
            ((*b).expr_size as libc::c_int + 8 as libc::c_int) as ucs2_t
    }
    (*(*b).expr.offset((*b).expr_used as isize)).onstack =
        0 as libc::c_int as ucs2_t;
    (*(*b).expr.offset((*b).expr_used as isize)).type_0 = type_0;
    (*(*b).expr.offset((*b).expr_used as isize)).lhs = lhs;
    (*(*b).expr.offset((*b).expr_used as isize)).rhs = rhs;
    let fresh22 = (*b).expr_used;
    (*b).expr_used = (*b).expr_used.wrapping_add(1);
    return fresh22;
}
#[c2rust::src_loc = "1179:22"]
static mut spmap: [libc::c_uchar; 32] =
    [0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0xf as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x80 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x10 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar];
/*
 * Convert the regular expression into an NFA in a form that will be easy to
 * reduce to a DFA.  The starting state for the reduction will be returned.
 */
#[c2rust::src_loc = "1192:1"]
unsafe extern "C" fn _ure_re2nfa(mut re: *mut ucs2_t,
                                 mut relen: libc::c_ulong,
                                 mut b: *mut _ure_buffer_t) -> ucs2_t {
    let mut c: ucs2_t = 0;
    let mut state: ucs2_t = 0;
    let mut top: ucs2_t = 0;
    let mut sym: ucs2_t = 0;
    let mut sp: *mut ucs2_t = 0 as *mut ucs2_t;
    let mut ep: *mut ucs2_t = 0 as *mut ucs2_t;
    let mut used: libc::c_ulong = 0;
    state = 0xffff as libc::c_int as ucs2_t;
    sp = re;
    ep = sp.offset(relen as isize);
    while (*b).error == 0 as libc::c_int && sp < ep {
        let fresh23 = sp;
        sp = sp.offset(1);
        c = *fresh23;
        match c as libc::c_int {
            40 => { _ure_push(11 as libc::c_int as ucs2_t, b); }
            41 => {
                /*
             * Check for the case of too many close parentheses.
             */
                if _ure_peek(b) as libc::c_int == 0xffff as libc::c_int {
                    (*b).error = -(3 as libc::c_int)
                } else {
                    loop  {
                        top = _ure_peek(b);
                        if !(top as libc::c_int == 16 as libc::c_int ||
                                 top as libc::c_int == 17 as libc::c_int) {
                            break ;
                        }
                        /*
               * Make an expression with the AND or OR operator and its right
               * hand side.
               */
                        state =
                            _ure_make_expr(_ure_pop(b), _ure_pop(b), state, b)
                    }
                    /*
             * Remove the _URE_PAREN off the stack.
             */
                    _ure_pop(b);
                }
            }
            42 => {
                state =
                    _ure_make_expr(13 as libc::c_int as ucs2_t, state,
                                   0xffff as libc::c_int as ucs2_t, b)
            }
            43 => {
                state =
                    _ure_make_expr(14 as libc::c_int as ucs2_t, state,
                                   0xffff as libc::c_int as ucs2_t, b)
            }
            63 => {
                state =
                    _ure_make_expr(12 as libc::c_int as ucs2_t, state,
                                   0xffff as libc::c_int as ucs2_t, b)
            }
            124 => {
                loop  {
                    top = _ure_peek(b);
                    if !(top as libc::c_int == 16 as libc::c_int ||
                             top as libc::c_int == 17 as libc::c_int) {
                        break ;
                    }
                    /*
               * Make an expression with the AND or OR operator and its right
               * hand side.
               */
                    state = _ure_make_expr(_ure_pop(b), _ure_pop(b), state, b)
                }
                _ure_push(state, b);
                _ure_push(17 as libc::c_int as ucs2_t, b);
            }
            _ => {
                sp = sp.offset(-1);
                sym =
                    _ure_make_symbol(sp,
                                     ep.wrapping_offset_from(sp) as
                                         libc::c_long as libc::c_ulong,
                                     &mut used, b);
                sp = sp.offset(used as isize);
                state =
                    _ure_make_expr(10 as libc::c_int as ucs2_t, sym,
                                   0xffff as libc::c_int as ucs2_t, b)
            }
        }
        if c as libc::c_int != '(' as i32 && c as libc::c_int != '|' as i32 &&
               sp < ep &&
               (!(*sp as libc::c_int > 0x20 as libc::c_int &&
                      (*sp as libc::c_int) < 0x7f as libc::c_int &&
                      spmap[(*sp as libc::c_int >> 3 as libc::c_int) as usize]
                          as libc::c_int &
                          (1 as libc::c_int) <<
                              (*sp as libc::c_int & 7 as libc::c_int) != 0) ||
                    *sp as libc::c_int == '(' as i32) {
            _ure_push(state, b);
            _ure_push(16 as libc::c_int as ucs2_t, b);
        }
    }
    loop  {
        top = _ure_peek(b);
        if !(top as libc::c_int == 16 as libc::c_int ||
                 top as libc::c_int == 17 as libc::c_int) {
            break ;
        }
        /*
       * Make an expression with the AND or OR operator and its right
       * hand side.
       */
        state = _ure_make_expr(_ure_pop(b), _ure_pop(b), state, b)
    }
    if (*b).stack.slist_used as libc::c_int > 0 as libc::c_int {
        (*b).error = -(3 as libc::c_int)
    }
    return if (*b).error == 0 as libc::c_int {
               state as libc::c_int
           } else { 0xffff as libc::c_int } as ucs2_t;
}
#[c2rust::src_loc = "1276:1"]
unsafe extern "C" fn _ure_add_symstate(mut sym: ucs2_t, mut state: ucs2_t,
                                       mut b: *mut _ure_buffer_t) {
    let mut i: ucs2_t = 0;
    let mut stp: *mut ucs2_t = 0 as *mut ucs2_t;
    let mut sp: *mut _ure_symtab_t = 0 as *mut _ure_symtab_t;
    /*
     * Locate the symbol in the symbol table so the state can be added.
     * If the symbol doesn't exist, then a real problem exists.
     */
    i = 0 as libc::c_int as ucs2_t;
    sp = (*b).symtab;
    while (i as libc::c_int) < (*b).symtab_used as libc::c_int &&
              sym as libc::c_int != (*sp).id as libc::c_int {
        i = i.wrapping_add(1);
        sp = sp.offset(1)
    }
    /*
     * Now find out if the state exists in the symbol's state list.
     */
    i = 0 as libc::c_int as ucs2_t;
    stp = (*sp).states.slist;
    while (i as libc::c_int) < (*sp).states.slist_used as libc::c_int &&
              state as libc::c_int > *stp as libc::c_int {
        i = i.wrapping_add(1);
        stp = stp.offset(1)
    }
    if i as libc::c_int == (*sp).states.slist_used as libc::c_int ||
           (state as libc::c_int) < *stp as libc::c_int {
        /*
         * Need to add the state in order.
         */
        if (*sp).states.slist_used as libc::c_int ==
               (*sp).states.slist_size as libc::c_int {
            if (*sp).states.slist_size as libc::c_int == 0 as libc::c_int {
                (*sp).states.slist =
                    malloc((::std::mem::size_of::<ucs2_t>() as libc::c_ulong)
                               << 3 as libc::c_int) as *mut ucs2_t
            } else {
                (*sp).states.slist =
                    realloc((*sp).states.slist as *mut libc::c_char as
                                *mut libc::c_void,
                            (::std::mem::size_of::<ucs2_t>() as
                                 libc::c_ulong).wrapping_mul(((*sp).states.slist_size
                                                                  as
                                                                  libc::c_int
                                                                  +
                                                                  8 as
                                                                      libc::c_int)
                                                                 as
                                                                 libc::c_ulong))
                        as *mut ucs2_t
            }
            (*sp).states.slist_size =
                ((*sp).states.slist_size as libc::c_int + 8 as libc::c_int) as
                    ucs2_t
        }
        if (i as libc::c_int) < (*sp).states.slist_used as libc::c_int {
            _ure_memmove((*sp).states.slist.offset(i as libc::c_int as
                                                       isize).offset(1 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                             as *mut libc::c_char,
                         (*sp).states.slist.offset(i as libc::c_int as isize)
                             as *mut libc::c_char,
                         (::std::mem::size_of::<ucs2_t>() as
                              libc::c_ulong).wrapping_mul(((*sp).states.slist_used
                                                               as libc::c_int
                                                               -
                                                               i as
                                                                   libc::c_int)
                                                              as
                                                              libc::c_ulong));
        }
        *(*sp).states.slist.offset(i as isize) = state;
        (*sp).states.slist_used = (*sp).states.slist_used.wrapping_add(1)
    };
}
#[c2rust::src_loc = "1317:1"]
unsafe extern "C" fn _ure_add_state(mut nstates: ucs2_t,
                                    mut states: *mut ucs2_t,
                                    mut b: *mut _ure_buffer_t) -> ucs2_t {
    let mut i: ucs2_t = 0;
    let mut sp: *mut _ure_state_t = 0 as *mut _ure_state_t;
    i = 0 as libc::c_int as ucs2_t;
    sp = (*b).states.states;
    while (i as libc::c_int) < (*b).states.states_used as libc::c_int {
        if (*sp).st.slist_used as libc::c_int == nstates as libc::c_int &&
               memcmp(states as *mut libc::c_char as *const libc::c_void,
                      (*sp).st.slist as *mut libc::c_char as
                          *const libc::c_void,
                      (::std::mem::size_of::<ucs2_t>() as
                           libc::c_ulong).wrapping_mul(nstates as
                                                           libc::c_ulong)) ==
                   0 as libc::c_int {
            break ;
        }
        i = i.wrapping_add(1);
        sp = sp.offset(1)
    }
    if i as libc::c_int == (*b).states.states_used as libc::c_int {
        /*
         * Need to add a new DFA state (set of NFA states).
         */
        if (*b).states.states_used as libc::c_int ==
               (*b).states.states_size as libc::c_int {
            if (*b).states.states_size as libc::c_int == 0 as libc::c_int {
                (*b).states.states =
                    malloc((::std::mem::size_of::<_ure_state_t>() as
                                libc::c_ulong) << 3 as libc::c_int) as
                        *mut _ure_state_t
            } else {
                (*b).states.states =
                    realloc((*b).states.states as *mut libc::c_char as
                                *mut libc::c_void,
                            (::std::mem::size_of::<_ure_state_t>() as
                                 libc::c_ulong).wrapping_mul(((*b).states.states_size
                                                                  as
                                                                  libc::c_int
                                                                  +
                                                                  8 as
                                                                      libc::c_int)
                                                                 as
                                                                 libc::c_ulong))
                        as *mut _ure_state_t
            }
            sp =
                (*b).states.states.offset((*b).states.states_size as
                                              libc::c_int as isize);
            memset(sp as *mut libc::c_char as *mut libc::c_void,
                   '\u{0}' as i32,
                   (::std::mem::size_of::<_ure_state_t>() as libc::c_ulong) <<
                       3 as libc::c_int);
            (*b).states.states_size =
                ((*b).states.states_size as libc::c_int + 8 as libc::c_int) as
                    ucs2_t
        }
        let fresh24 = (*b).states.states_used;
        (*b).states.states_used = (*b).states.states_used.wrapping_add(1);
        sp = (*b).states.states.offset(fresh24 as libc::c_int as isize);
        (*sp).id = i;
        if (*sp).st.slist_used as libc::c_int + nstates as libc::c_int >
               (*sp).st.slist_size as libc::c_int {
            if (*sp).st.slist_size as libc::c_int == 0 as libc::c_int {
                (*sp).st.slist =
                    malloc((::std::mem::size_of::<ucs2_t>() as
                                libc::c_ulong).wrapping_mul(((*sp).st.slist_used
                                                                 as
                                                                 libc::c_int +
                                                                 nstates as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_ulong))
                        as *mut ucs2_t
            } else {
                (*sp).st.slist =
                    realloc((*sp).st.slist as *mut libc::c_char as
                                *mut libc::c_void,
                            (::std::mem::size_of::<ucs2_t>() as
                                 libc::c_ulong).wrapping_mul(((*sp).st.slist_used
                                                                  as
                                                                  libc::c_int
                                                                  +
                                                                  nstates as
                                                                      libc::c_int)
                                                                 as
                                                                 libc::c_ulong))
                        as *mut ucs2_t
            }
            (*sp).st.slist_size =
                ((*sp).st.slist_used as libc::c_int + nstates as libc::c_int)
                    as ucs2_t
        }
        (*sp).st.slist_used = nstates;
        memcpy((*sp).st.slist as *mut libc::c_char as *mut libc::c_void,
               states as *mut libc::c_char as *const libc::c_void,
               (::std::mem::size_of::<ucs2_t>() as
                    libc::c_ulong).wrapping_mul(nstates as libc::c_ulong));
    }
    /*
     * Return the ID of the DFA state representing a group of NFA states.
     */
    return i;
}
#[c2rust::src_loc = "1371:1"]
unsafe extern "C" fn _ure_reduce(mut start: ucs2_t,
                                 mut b: *mut _ure_buffer_t) {
    let mut i: ucs2_t = 0;
    let mut j: ucs2_t = 0;
    let mut state: ucs2_t = 0;
    let mut eval: ucs2_t = 0;
    let mut syms: ucs2_t = 0;
    let mut rhs: ucs2_t = 0;
    let mut s1: ucs2_t = 0;
    let mut s2: ucs2_t = 0;
    let mut ns1: ucs2_t = 0;
    let mut ns2: ucs2_t = 0;
    let mut sp: *mut _ure_state_t = 0 as *mut _ure_state_t;
    let mut smp: *mut _ure_symtab_t = 0 as *mut _ure_symtab_t;
    (*b).reducing = 1 as libc::c_int;
    /*
     * Add the starting state for the reduction.
     */
    _ure_add_state(1 as libc::c_int as ucs2_t, &mut start, b);
    /*
     * Process each set of NFA states that get created.
     */
    i = 0 as libc::c_int as ucs2_t;
    while (i as libc::c_int) < (*b).states.states_used as libc::c_int {
        sp = (*b).states.states.offset(i as libc::c_int as isize);
        /*
         * Push the current states on the stack.
         */
        j = 0 as libc::c_int as ucs2_t;
        while (j as libc::c_int) < (*sp).st.slist_used as libc::c_int {
            _ure_push(*(*sp).st.slist.offset(j as isize), b);
            j = j.wrapping_add(1)
        }
        /*
         * Reduce the NFA states.
         */
        syms = 0 as libc::c_int as ucs2_t;
        (*sp).accepting = syms;
        j = (*sp).accepting;
        while (j as libc::c_int) < (*b).stack.slist_used as libc::c_int {
            state = *(*b).stack.slist.offset(j as isize);
            eval = 1 as libc::c_int as ucs2_t;
            /*
             * This inner loop is the iterative equivalent of recursively
             * reducing subexpressions generated as a result of a reduction.
             */
            while eval != 0 {
                match (*(*b).expr.offset(state as isize)).type_0 as
                          libc::c_int {
                    10 => {
                        ns1 =
                            _ure_make_expr(15 as libc::c_int as ucs2_t,
                                           0xffff as libc::c_int as ucs2_t,
                                           0xffff as libc::c_int as ucs2_t,
                                           b);
                        _ure_add_symstate((*(*b).expr.offset(state as
                                                                 isize)).lhs,
                                          ns1, b);
                        syms = syms.wrapping_add(1);
                        eval = 0 as libc::c_int as ucs2_t
                    }
                    15 => {
                        (*sp).accepting = 1 as libc::c_int as ucs2_t;
                        eval = 0 as libc::c_int as ucs2_t
                    }
                    12 => {
                        s1 = (*(*b).expr.offset(state as isize)).lhs;
                        ns1 =
                            _ure_make_expr(15 as libc::c_int as ucs2_t,
                                           0xffff as libc::c_int as ucs2_t,
                                           0xffff as libc::c_int as ucs2_t,
                                           b);
                        state =
                            _ure_make_expr(17 as libc::c_int as ucs2_t, ns1,
                                           s1, b)
                    }
                    14 => {
                        s1 = (*(*b).expr.offset(state as isize)).lhs;
                        ns1 =
                            _ure_make_expr(13 as libc::c_int as ucs2_t, s1,
                                           0xffff as libc::c_int as ucs2_t,
                                           b);
                        state =
                            _ure_make_expr(16 as libc::c_int as ucs2_t, s1,
                                           ns1, b)
                    }
                    13 => {
                        s1 = (*(*b).expr.offset(state as isize)).lhs;
                        ns1 =
                            _ure_make_expr(15 as libc::c_int as ucs2_t,
                                           0xffff as libc::c_int as ucs2_t,
                                           0xffff as libc::c_int as ucs2_t,
                                           b);
                        ns2 =
                            _ure_make_expr(14 as libc::c_int as ucs2_t, s1,
                                           0xffff as libc::c_int as ucs2_t,
                                           b);
                        state =
                            _ure_make_expr(17 as libc::c_int as ucs2_t, ns1,
                                           ns2, b)
                    }
                    17 => {
                        s1 = (*(*b).expr.offset(state as isize)).lhs;
                        s2 = (*(*b).expr.offset(state as isize)).rhs;
                        _ure_push(s1, b);
                        _ure_push(s2, b);
                        eval = 0 as libc::c_int as ucs2_t
                    }
                    16 => {
                        s1 = (*(*b).expr.offset(state as isize)).lhs;
                        s2 = (*(*b).expr.offset(state as isize)).rhs;
                        match (*(*b).expr.offset(s1 as isize)).type_0 as
                                  libc::c_int {
                            10 => {
                                _ure_add_symstate((*(*b).expr.offset(s1 as
                                                                         isize)).lhs,
                                                  s2, b);
                                syms = syms.wrapping_add(1);
                                eval = 0 as libc::c_int as ucs2_t
                            }
                            15 => { state = s2 }
                            12 => {
                                ns1 = (*(*b).expr.offset(s1 as isize)).lhs;
                                ns2 =
                                    _ure_make_expr(16 as libc::c_int as
                                                       ucs2_t, ns1, s2, b);
                                state =
                                    _ure_make_expr(17 as libc::c_int as
                                                       ucs2_t, s2, ns2, b)
                            }
                            14 => {
                                ns1 = (*(*b).expr.offset(s1 as isize)).lhs;
                                ns2 =
                                    _ure_make_expr(17 as libc::c_int as
                                                       ucs2_t, s2, state, b);
                                state =
                                    _ure_make_expr(16 as libc::c_int as
                                                       ucs2_t, ns1, ns2, b)
                            }
                            13 => {
                                ns1 = (*(*b).expr.offset(s1 as isize)).lhs;
                                ns2 =
                                    _ure_make_expr(16 as libc::c_int as
                                                       ucs2_t, ns1, state, b);
                                state =
                                    _ure_make_expr(17 as libc::c_int as
                                                       ucs2_t, s2, ns2, b)
                            }
                            17 => {
                                ns1 = (*(*b).expr.offset(s1 as isize)).lhs;
                                ns2 = (*(*b).expr.offset(s1 as isize)).rhs;
                                ns1 =
                                    _ure_make_expr(16 as libc::c_int as
                                                       ucs2_t, ns1, s2, b);
                                ns2 =
                                    _ure_make_expr(16 as libc::c_int as
                                                       ucs2_t, ns2, s2, b);
                                state =
                                    _ure_make_expr(17 as libc::c_int as
                                                       ucs2_t, ns1, ns2, b)
                            }
                            16 => {
                                ns1 = (*(*b).expr.offset(s1 as isize)).lhs;
                                ns2 = (*(*b).expr.offset(s1 as isize)).rhs;
                                ns2 =
                                    _ure_make_expr(16 as libc::c_int as
                                                       ucs2_t, ns2, s2, b);
                                state =
                                    _ure_make_expr(16 as libc::c_int as
                                                       ucs2_t, ns1, ns2, b)
                            }
                            _ => { }
                        }
                    }
                    _ => { }
                }
            }
            j = j.wrapping_add(1)
        }
        /*
         * Clear the state stack.
         */
        while _ure_pop(b) as libc::c_int != 0xffff as libc::c_int { }
        /*
         * Reset the state pointer because the reduction may have moved it
         * during a reallocation.
         */
        sp = (*b).states.states.offset(i as libc::c_int as isize);
        /*
         * Generate the DFA states for the symbols collected during the
         * current reduction.
         */
        if (*sp).trans_used as libc::c_int + syms as libc::c_int >
               (*sp).trans_size as libc::c_int {
            if (*sp).trans_size as libc::c_int == 0 as libc::c_int {
                (*sp).trans =
                    malloc((::std::mem::size_of::<_ure_elt_t>() as
                                libc::c_ulong).wrapping_mul(((*sp).trans_used
                                                                 as
                                                                 libc::c_int +
                                                                 syms as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_ulong))
                        as *mut _ure_elt_t
            } else {
                (*sp).trans =
                    realloc((*sp).trans as *mut libc::c_char as
                                *mut libc::c_void,
                            (::std::mem::size_of::<_ure_elt_t>() as
                                 libc::c_ulong).wrapping_mul(((*sp).trans_used
                                                                  as
                                                                  libc::c_int
                                                                  +
                                                                  syms as
                                                                      libc::c_int)
                                                                 as
                                                                 libc::c_ulong))
                        as *mut _ure_elt_t
            }
            (*sp).trans_size =
                ((*sp).trans_used as libc::c_int + syms as libc::c_int) as
                    ucs2_t
        }
        /*
         * Go through the symbol table and generate the DFA state transitions
         * for each symbol that has collected NFA states.
         */
        syms = 0 as libc::c_int as ucs2_t;
        j = syms;
        smp = (*b).symtab;
        while (j as libc::c_int) < (*b).symtab_used as libc::c_int {
            sp = (*b).states.states.offset(i as libc::c_int as isize);
            if (*smp).states.slist_used as libc::c_int > 0 as libc::c_int {
                (*(*sp).trans.offset(syms as isize)).lhs = (*smp).id;
                rhs =
                    _ure_add_state((*smp).states.slist_used,
                                   (*smp).states.slist, b);
                /*
                 * Reset the state pointer in case the reallocation moves it
                 * in memory.
                 */
                sp = (*b).states.states.offset(i as libc::c_int as isize);
                (*(*sp).trans.offset(syms as isize)).rhs = rhs;
                (*smp).states.slist_used = 0 as libc::c_int as ucs2_t;
                syms = syms.wrapping_add(1)
            }
            j = j.wrapping_add(1);
            smp = smp.offset(1)
        }
        /*
         * Set the number of transitions actually used.
         */
        (*sp).trans_used = syms;
        i = i.wrapping_add(1)
    }
    (*b).reducing = 0 as libc::c_int;
}
#[c2rust::src_loc = "1546:1"]
unsafe extern "C" fn _ure_add_equiv(mut l: ucs2_t, mut r: ucs2_t,
                                    mut b: *mut _ure_buffer_t) {
    let mut tmp: ucs2_t = 0;
    l = (*(*b).states.states.offset(l as isize)).id;
    r = (*(*b).states.states.offset(r as isize)).id;
    if l as libc::c_int == r as libc::c_int { return }
    if l as libc::c_int > r as libc::c_int { tmp = l; l = r; r = tmp }
    /*
     * Check to see if the equivalence pair already exists.
     */
    tmp = 0 as libc::c_int as ucs2_t;
    while (tmp as libc::c_int) < (*b).equiv_used as libc::c_int &&
              ((*(*b).equiv.offset(tmp as isize)).l as libc::c_int !=
                   l as libc::c_int ||
                   (*(*b).equiv.offset(tmp as isize)).r as libc::c_int !=
                       r as libc::c_int) {
        tmp = tmp.wrapping_add(1)
    }
    if (tmp as libc::c_int) < (*b).equiv_used as libc::c_int { return }
    if (*b).equiv_used as libc::c_int == (*b).equiv_size as libc::c_int {
        if (*b).equiv_size as libc::c_int == 0 as libc::c_int {
            (*b).equiv =
                malloc((::std::mem::size_of::<_ure_equiv_t>() as
                            libc::c_ulong) << 3 as libc::c_int) as
                    *mut _ure_equiv_t
        } else {
            (*b).equiv =
                realloc((*b).equiv as *mut libc::c_char as *mut libc::c_void,
                        (::std::mem::size_of::<_ure_equiv_t>() as
                             libc::c_ulong).wrapping_mul(((*b).equiv_size as
                                                              libc::c_int +
                                                              8 as
                                                                  libc::c_int)
                                                             as
                                                             libc::c_ulong))
                    as *mut _ure_equiv_t
        }
        (*b).equiv_size =
            ((*b).equiv_size as libc::c_int + 8 as libc::c_int) as ucs2_t
    }
    (*(*b).equiv.offset((*b).equiv_used as isize)).l = l;
    (*(*b).equiv.offset((*b).equiv_used as isize)).r = r;
    (*b).equiv_used = (*b).equiv_used.wrapping_add(1);
}
/*
 * Merge the DFA states that are equivalent.
 */
#[c2rust::src_loc = "1590:1"]
unsafe extern "C" fn _ure_merge_equiv(mut b: *mut _ure_buffer_t) {
    let mut i: ucs2_t = 0;
    let mut j: ucs2_t = 0;
    let mut k: ucs2_t = 0;
    let mut eq: ucs2_t = 0;
    let mut done: ucs2_t = 0;
    let mut sp1: *mut _ure_state_t = 0 as *mut _ure_state_t;
    let mut sp2: *mut _ure_state_t = 0 as *mut _ure_state_t;
    let mut ls: *mut _ure_state_t = 0 as *mut _ure_state_t;
    let mut rs: *mut _ure_state_t = 0 as *mut _ure_state_t;
    i = 0 as libc::c_int as ucs2_t;
    while (i as libc::c_int) < (*b).states.states_used as libc::c_int {
        sp1 = (*b).states.states.offset(i as libc::c_int as isize);
        if !((*sp1).id as libc::c_int != i as libc::c_int) {
            j = 0 as libc::c_int as ucs2_t;
            while (j as libc::c_int) < i as libc::c_int {
                sp2 = (*b).states.states.offset(j as libc::c_int as isize);
                if !((*sp2).id as libc::c_int != j as libc::c_int) {
                    (*b).equiv_used = 0 as libc::c_int as ucs2_t;
                    _ure_add_equiv(i, j, b);
                    eq = 0 as libc::c_int as ucs2_t;
                    done = 0 as libc::c_int as ucs2_t;
                    while (eq as libc::c_int) < (*b).equiv_used as libc::c_int
                          {
                        ls =
                            (*b).states.states.offset((*(*b).equiv.offset(eq
                                                                              as
                                                                              isize)).l
                                                          as libc::c_int as
                                                          isize);
                        rs =
                            (*b).states.states.offset((*(*b).equiv.offset(eq
                                                                              as
                                                                              isize)).r
                                                          as libc::c_int as
                                                          isize);
                        if (*ls).accepting as libc::c_int !=
                               (*rs).accepting as libc::c_int ||
                               (*ls).trans_used as libc::c_int !=
                                   (*rs).trans_used as libc::c_int {
                            done = 1 as libc::c_int as ucs2_t;
                            break ;
                        } else {
                            k = 0 as libc::c_int as ucs2_t;
                            while (k as libc::c_int) <
                                      (*ls).trans_used as libc::c_int &&
                                      (*(*ls).trans.offset(k as isize)).lhs as
                                          libc::c_int ==
                                          (*(*rs).trans.offset(k as
                                                                   isize)).lhs
                                              as libc::c_int {
                                k = k.wrapping_add(1)
                            }
                            if (k as libc::c_int) <
                                   (*ls).trans_used as libc::c_int {
                                done = 1 as libc::c_int as ucs2_t;
                                break ;
                            } else {
                                k = 0 as libc::c_int as ucs2_t;
                                while (k as libc::c_int) <
                                          (*ls).trans_used as libc::c_int {
                                    _ure_add_equiv((*(*ls).trans.offset(k as
                                                                            isize)).rhs,
                                                   (*(*rs).trans.offset(k as
                                                                            isize)).rhs,
                                                   b);
                                    k = k.wrapping_add(1)
                                }
                                eq = eq.wrapping_add(1)
                            }
                        }
                    }
                    if done as libc::c_int == 0 as libc::c_int { break ; }
                }
                j = j.wrapping_add(1)
            }
            eq = 0 as libc::c_int as ucs2_t;
            while (j as libc::c_int) < i as libc::c_int &&
                      (eq as libc::c_int) < (*b).equiv_used as libc::c_int {
                (*(*b).states.states.offset((*(*b).equiv.offset(eq as
                                                                    isize)).r
                                                as isize)).id =
                    (*(*b).states.states.offset((*(*b).equiv.offset(eq as
                                                                        isize)).l
                                                    as isize)).id;
                eq = eq.wrapping_add(1)
            }
        }
        i = i.wrapping_add(1)
    }
    /*
     * Renumber the states appropriately.
     */
    eq = 0 as libc::c_int as ucs2_t;
    i = eq;
    sp1 = (*b).states.states;
    while (i as libc::c_int) < (*b).states.states_used as libc::c_int {
        (*sp1).id =
            if (*sp1).id as libc::c_int == i as libc::c_int {
                let fresh25 = eq;
                eq = eq.wrapping_add(1);
                fresh25 as libc::c_int
            } else {
                (*(*b).states.states.offset((*sp1).id as isize)).id as
                    libc::c_int
            } as ucs2_t;
        sp1 = sp1.offset(1);
        i = i.wrapping_add(1)
    };
}
/* ************************************************************************
 *
 * API.
 *
 *************************************************************************/
/* ************************************************************************
 *
 * API.
 *
 *************************************************************************/
#[no_mangle]
#[c2rust::src_loc = "1646:1"]
pub unsafe extern "C" fn ure_buffer_create() -> ure_buffer_t {
    let mut b: ure_buffer_t = 0 as *mut _ure_buffer_t;
    b =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<_ure_buffer_t>() as libc::c_ulong) as
            ure_buffer_t;
    return b;
}
#[no_mangle]
#[c2rust::src_loc = "1656:1"]
pub unsafe extern "C" fn ure_buffer_free(mut buf: ure_buffer_t) {
    let mut i: libc::c_ulong = 0;
    if buf.is_null() { return }
    if (*buf).stack.slist_size as libc::c_int > 0 as libc::c_int {
        free((*buf).stack.slist as *mut libc::c_char as *mut libc::c_void);
    }
    if (*buf).expr_size as libc::c_int > 0 as libc::c_int {
        free((*buf).expr as *mut libc::c_char as *mut libc::c_void);
    }
    i = 0 as libc::c_int as libc::c_ulong;
    while i < (*buf).symtab_size as libc::c_ulong {
        if (*(*buf).symtab.offset(i as isize)).states.slist_size as
               libc::c_int > 0 as libc::c_int {
            free((*(*buf).symtab.offset(i as isize)).states.slist as
                     *mut libc::c_char as *mut libc::c_void);
        }
        i = i.wrapping_add(1)
    }
    if (*buf).symtab_size as libc::c_int > 0 as libc::c_int {
        free((*buf).symtab as *mut libc::c_char as *mut libc::c_void);
    }
    i = 0 as libc::c_int as libc::c_ulong;
    while i < (*buf).states.states_size as libc::c_ulong {
        if (*(*buf).states.states.offset(i as isize)).trans_size as
               libc::c_int > 0 as libc::c_int {
            free((*(*buf).states.states.offset(i as isize)).trans as
                     *mut libc::c_char as *mut libc::c_void);
        }
        if (*(*buf).states.states.offset(i as isize)).st.slist_size as
               libc::c_int > 0 as libc::c_int {
            free((*(*buf).states.states.offset(i as isize)).st.slist as
                     *mut libc::c_char as *mut libc::c_void);
        }
        i = i.wrapping_add(1)
    }
    if (*buf).states.states_size as libc::c_int > 0 as libc::c_int {
        free((*buf).states.states as *mut libc::c_char as *mut libc::c_void);
    }
    if (*buf).equiv_size as libc::c_int > 0 as libc::c_int {
        free((*buf).equiv as *mut libc::c_char as *mut libc::c_void);
    }
    free(buf as *mut libc::c_char as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "1694:1"]
pub unsafe extern "C" fn ure_compile(mut re: *mut ucs2_t,
                                     mut relen: libc::c_ulong,
                                     mut casefold: libc::c_int,
                                     mut buf: ure_buffer_t) -> ure_dfa_t {
    let mut i: ucs2_t = 0;
    let mut j: ucs2_t = 0;
    let mut state: ucs2_t = 0;
    let mut sp: *mut _ure_state_t = 0 as *mut _ure_state_t;
    let mut dsp: *mut _ure_dstate_t = 0 as *mut _ure_dstate_t;
    let mut tp: *mut _ure_trans_t = 0 as *mut _ure_trans_t;
    let mut dfa: ure_dfa_t = 0 as *mut _ure_dfa_t;
    if re.is_null() || *re as libc::c_int == 0 as libc::c_int ||
           relen == 0 as libc::c_int as libc::c_ulong || buf.is_null() {
        return 0 as ure_dfa_t
    }
    /*
     * Reset the various fields of the compilation buffer.  Default the flags
     * to indicate the presense of the "^$" pattern.  If any other pattern
     * occurs, then this flag will be removed.  This is done to catch this
     * special pattern and handle it specially when matching.
     */
    (*buf).flags =
        (0x2 as libc::c_int |
             (if casefold != 0 {
                  0x1 as libc::c_int
              } else { 0 as libc::c_int })) as libc::c_ulong;
    (*buf).reducing = 0 as libc::c_int;
    (*buf).stack.slist_used = 0 as libc::c_int as ucs2_t;
    (*buf).expr_used = 0 as libc::c_int as ucs2_t;
    i = 0 as libc::c_int as ucs2_t;
    while (i as libc::c_int) < (*buf).symtab_used as libc::c_int {
        (*(*buf).symtab.offset(i as isize)).states.slist_used =
            0 as libc::c_int as ucs2_t;
        i = i.wrapping_add(1)
    }
    (*buf).symtab_used = 0 as libc::c_int as ucs2_t;
    i = 0 as libc::c_int as ucs2_t;
    while (i as libc::c_int) < (*buf).states.states_used as libc::c_int {
        (*(*buf).states.states.offset(i as isize)).st.slist_used =
            0 as libc::c_int as ucs2_t;
        (*(*buf).states.states.offset(i as isize)).trans_used =
            0 as libc::c_int as ucs2_t;
        i = i.wrapping_add(1)
    }
    (*buf).states.states_used = 0 as libc::c_int as ucs2_t;
    /*
     * Construct the NFA.  If this stage returns a 0, then an error occurred or
     * an empty expression was passed.
     */
    state = _ure_re2nfa(re, relen, buf);
    if state as libc::c_int == 0xffff as libc::c_int { return 0 as ure_dfa_t }
    /*
     * Do the expression reduction to get the initial DFA.
     */
    _ure_reduce(state, buf);
    /*
     * Merge all the equivalent DFA states.
     */
    _ure_merge_equiv(buf);
    /*
     * Construct the minimal DFA.
     */
    dfa =
        malloc(::std::mem::size_of::<_ure_dfa_t>() as libc::c_ulong) as
            ure_dfa_t;
    memset(dfa as *mut libc::c_char as *mut libc::c_void, '\u{0}' as i32,
           ::std::mem::size_of::<_ure_dfa_t>() as libc::c_ulong);
    (*dfa).flags =
        (*buf).flags &
            (0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_ulong;
    /*
     * Free up the NFA state groups and transfer the symbols from the buffer
     * to the DFA.
     */
    i = 0 as libc::c_int as ucs2_t;
    while (i as libc::c_int) < (*buf).symtab_size as libc::c_int {
        if (*(*buf).symtab.offset(i as isize)).states.slist_size as
               libc::c_int > 0 as libc::c_int {
            free((*(*buf).symtab.offset(i as isize)).states.slist as
                     *mut libc::c_char as *mut libc::c_void);
        }
        i = i.wrapping_add(1)
    }
    (*dfa).syms = (*buf).symtab;
    (*dfa).nsyms = (*buf).symtab_used;
    (*buf).symtab_size = 0 as libc::c_int as ucs2_t;
    (*buf).symtab_used = (*buf).symtab_size;
    /*
     * Collect the total number of states and transitions needed for the DFA.
     */
    state = 0 as libc::c_int as ucs2_t;
    i = state;
    sp = (*buf).states.states;
    while (i as libc::c_int) < (*buf).states.states_used as libc::c_int {
        if (*sp).id as libc::c_int == state as libc::c_int {
            (*dfa).nstates = (*dfa).nstates.wrapping_add(1);
            (*dfa).ntrans =
                ((*dfa).ntrans as libc::c_int +
                     (*sp).trans_used as libc::c_int) as ucs2_t;
            state = state.wrapping_add(1)
        }
        i = i.wrapping_add(1);
        sp = sp.offset(1)
    }
    /*
     * Allocate enough space for the states and transitions.
     */
    (*dfa).states =
        malloc((::std::mem::size_of::<_ure_dstate_t>() as
                    libc::c_ulong).wrapping_mul((*dfa).nstates as
                                                    libc::c_ulong)) as
            *mut _ure_dstate_t;
    (*dfa).trans =
        malloc((::std::mem::size_of::<_ure_trans_t>() as
                    libc::c_ulong).wrapping_mul((*dfa).ntrans as
                                                    libc::c_ulong)) as
            *mut _ure_trans_t;
    /*
     * Actually transfer the DFA states from the buffer.
     */
    dsp = (*dfa).states;
    tp = (*dfa).trans;
    state = 0 as libc::c_int as ucs2_t;
    i = state;
    sp = (*buf).states.states;
    while (i as libc::c_int) < (*buf).states.states_used as libc::c_int {
        if (*sp).id as libc::c_int == state as libc::c_int {
            (*dsp).trans = tp;
            (*dsp).ntrans = (*sp).trans_used;
            (*dsp).accepting = (*sp).accepting;
            /*
             * Add the transitions for the state.
             */
            j = 0 as libc::c_int as ucs2_t;
            while (j as libc::c_int) < (*dsp).ntrans as libc::c_int {
                (*tp).symbol = (*(*sp).trans.offset(j as isize)).lhs;
                (*tp).next_state =
                    (*(*buf).states.states.offset((*(*sp).trans.offset(j as
                                                                           isize)).rhs
                                                      as isize)).id;
                j = j.wrapping_add(1);
                tp = tp.offset(1)
            }
            dsp = dsp.offset(1);
            state = state.wrapping_add(1)
        }
        i = i.wrapping_add(1);
        sp = sp.offset(1)
    }
    return dfa;
}
#[no_mangle]
#[c2rust::src_loc = "1812:1"]
pub unsafe extern "C" fn ure_dfa_free(mut dfa: ure_dfa_t) {
    let mut i: ucs2_t = 0;
    if dfa.is_null() { return }
    i = 0 as libc::c_int as ucs2_t;
    while (i as libc::c_int) < (*dfa).nsyms as libc::c_int {
        if ((*(*dfa).syms.offset(i as isize)).type_0 as libc::c_int ==
                3 as libc::c_int ||
                (*(*dfa).syms.offset(i as isize)).type_0 as libc::c_int ==
                    4 as libc::c_int) &&
               (*(*dfa).syms.offset(i as isize)).sym.ccl.ranges_size as
                   libc::c_int > 0 as libc::c_int {
            free((*(*dfa).syms.offset(i as isize)).sym.ccl.ranges as
                     *mut libc::c_char as *mut libc::c_void);
        }
        i = i.wrapping_add(1)
    }
    if (*dfa).nsyms as libc::c_int > 0 as libc::c_int {
        free((*dfa).syms as *mut libc::c_char as *mut libc::c_void);
    }
    if (*dfa).nstates as libc::c_int > 0 as libc::c_int {
        free((*dfa).states as *mut libc::c_char as *mut libc::c_void);
    }
    if (*dfa).ntrans as libc::c_int > 0 as libc::c_int {
        free((*dfa).trans as *mut libc::c_char as *mut libc::c_void);
    }
    free(dfa as *mut libc::c_char as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "1836:1"]
pub unsafe extern "C" fn ure_write_dfa(mut dfa: ure_dfa_t,
                                       mut out: *mut FILE) {
    let mut i: ucs2_t = 0;
    let mut j: ucs2_t = 0;
    let mut k: ucs2_t = 0;
    let mut h: ucs2_t = 0;
    let mut l: ucs2_t = 0;
    let mut sp: *mut _ure_dstate_t = 0 as *mut _ure_dstate_t;
    let mut sym: *mut _ure_symtab_t = 0 as *mut _ure_symtab_t;
    let mut rp: *mut _ure_range_t = 0 as *mut _ure_range_t;
    if dfa.is_null() || out.is_null() { return }
    /*
     * Write all the different character classes.
     */
    i = 0 as libc::c_int as ucs2_t;
    sym = (*dfa).syms;
    while (i as libc::c_int) < (*dfa).nsyms as libc::c_int {
        if (*sym).type_0 as libc::c_int == 3 as libc::c_int ||
               (*sym).type_0 as libc::c_int == 4 as libc::c_int {
            fprintf(out, b"C%hd = \x00" as *const u8 as *const libc::c_char,
                    (*sym).id as libc::c_int);
            if (*sym).sym.ccl.ranges_used as libc::c_int > 0 as libc::c_int {
                putc('[' as i32, out);
                if (*sym).type_0 as libc::c_int == 4 as libc::c_int {
                    putc('^' as i32, out);
                }
            }
            if (*sym).props != 0 as libc::c_int as libc::c_ulong {
                if (*sym).type_0 as libc::c_int == 4 as libc::c_int {
                    fprintf(out,
                            b"\\P\x00" as *const u8 as *const libc::c_char);
                } else {
                    fprintf(out,
                            b"\\p\x00" as *const u8 as *const libc::c_char);
                }
                h = 0 as libc::c_int as ucs2_t;
                k = h;
                while (k as libc::c_int) < 32 as libc::c_int {
                    if (*sym).props &
                           ((1 as libc::c_int) << k as libc::c_int) as
                               libc::c_ulong != 0 {
                        if h as libc::c_int != 0 as libc::c_int {
                            putc(',' as i32, out);
                        }
                        fprintf(out,
                                b"%d\x00" as *const u8 as *const libc::c_char,
                                k as libc::c_int + 1 as libc::c_int);
                        h = 1 as libc::c_int as ucs2_t
                    }
                    k = k.wrapping_add(1)
                }
            }
            /*
             * Dump the ranges.
             */
            k = 0 as libc::c_int as ucs2_t;
            rp = (*sym).sym.ccl.ranges;
            while (k as libc::c_int) <
                      (*sym).sym.ccl.ranges_used as libc::c_int {
                /*
                 * Check for UTF16 characters.
                 */
                if 0x10000 as libc::c_int as libc::c_uint <= (*rp).min_code &&
                       (*rp).min_code <=
                           0x10ffff as libc::c_int as libc::c_uint {
                    h =
                        ((*rp).min_code.wrapping_sub(0x10000 as libc::c_int as
                                                         libc::c_uint) >>
                             10 as
                                 libc::c_int).wrapping_add(0xd800 as
                                                               libc::c_int as
                                                               libc::c_uint)
                            as ucs2_t;
                    l =
                        ((*rp).min_code.wrapping_sub(0x10000 as libc::c_int as
                                                         libc::c_uint) &
                             1023 as libc::c_int as
                                 libc::c_uint).wrapping_add(0xdc00 as
                                                                libc::c_int as
                                                                libc::c_uint)
                            as ucs2_t;
                    fprintf(out,
                            b"\\x%04hX\\x%04hX\x00" as *const u8 as
                                *const libc::c_char, h as libc::c_int,
                            l as libc::c_int);
                } else {
                    fprintf(out,
                            b"\\x%04lX\x00" as *const u8 as
                                *const libc::c_char,
                            ((*rp).min_code &
                                 0xffff as libc::c_int as libc::c_uint) as
                                libc::c_ulong);
                }
                if (*rp).max_code != (*rp).min_code {
                    putc('-' as i32, out);
                    if (*rp).max_code >=
                           0x10000 as libc::c_int as libc::c_uint &&
                           (*rp).max_code <=
                               0x10ffff as libc::c_int as libc::c_uint {
                        h =
                            ((*rp).max_code.wrapping_sub(0x10000 as
                                                             libc::c_int as
                                                             libc::c_uint) >>
                                 10 as
                                     libc::c_int).wrapping_add(0xd800 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint)
                                as ucs2_t;
                        l =
                            ((*rp).max_code.wrapping_sub(0x10000 as
                                                             libc::c_int as
                                                             libc::c_uint) &
                                 1023 as libc::c_int as
                                     libc::c_uint).wrapping_add(0xdc00 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
                                as ucs2_t;
                        fprintf(out,
                                b"\\x%04hX\\x%04hX\x00" as *const u8 as
                                    *const libc::c_char, h as libc::c_int,
                                l as libc::c_int);
                    } else {
                        fprintf(out,
                                b"\\x%04lX\x00" as *const u8 as
                                    *const libc::c_char,
                                ((*rp).max_code &
                                     0xffff as libc::c_int as libc::c_uint) as
                                    libc::c_ulong);
                    }
                }
                k = k.wrapping_add(1);
                rp = rp.offset(1)
            }
            if (*sym).sym.ccl.ranges_used as libc::c_int > 0 as libc::c_int {
                putc(']' as i32, out);
            }
            putc('\n' as i32, out);
        }
        i = i.wrapping_add(1);
        sym = sym.offset(1)
    }
    i = 0 as libc::c_int as ucs2_t;
    sp = (*dfa).states;
    while (i as libc::c_int) < (*dfa).nstates as libc::c_int {
        fprintf(out, b"S%hd = \x00" as *const u8 as *const libc::c_char,
                i as libc::c_int);
        if (*sp).accepting != 0 {
            fprintf(out, b"1 \x00" as *const u8 as *const libc::c_char);
            if (*sp).ntrans != 0 {
                fprintf(out, b"| \x00" as *const u8 as *const libc::c_char);
            }
        }
        j = 0 as libc::c_int as ucs2_t;
        while (j as libc::c_int) < (*sp).ntrans as libc::c_int {
            if j as libc::c_int > 0 as libc::c_int {
                fprintf(out, b"| \x00" as *const u8 as *const libc::c_char);
            }
            sym =
                (*dfa).syms.offset((*(*sp).trans.offset(j as isize)).symbol as
                                       libc::c_int as isize);
            match (*sym).type_0 as libc::c_int {
                2 => {
                    if 0x10000 as libc::c_int as libc::c_uint <=
                           (*sym).sym.chr &&
                           (*sym).sym.chr <=
                               0x10ffff as libc::c_int as libc::c_uint {
                        /*
                     * Take care of UTF16 characters.
                     */
                        h =
                            ((*sym).sym.chr.wrapping_sub(0x10000 as
                                                             libc::c_int as
                                                             libc::c_uint) >>
                                 10 as
                                     libc::c_int).wrapping_add(0xd800 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint)
                                as ucs2_t;
                        l =
                            ((*sym).sym.chr.wrapping_sub(0x10000 as
                                                             libc::c_int as
                                                             libc::c_uint) &
                                 1023 as libc::c_int as
                                     libc::c_uint).wrapping_add(0xdc00 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
                                as ucs2_t;
                        fprintf(out,
                                b"\\x%04hX\\x%04hX \x00" as *const u8 as
                                    *const libc::c_char, h as libc::c_int,
                                l as libc::c_int);
                    } else {
                        fprintf(out,
                                b"\\x%04lX \x00" as *const u8 as
                                    *const libc::c_char,
                                ((*sym).sym.chr &
                                     0xffff as libc::c_int as libc::c_uint) as
                                    libc::c_ulong);
                    }
                }
                1 => {
                    fprintf(out,
                            b"<any> \x00" as *const u8 as
                                *const libc::c_char);
                }
                5 => {
                    fprintf(out,
                            b"<bol-anchor> \x00" as *const u8 as
                                *const libc::c_char);
                }
                6 => {
                    fprintf(out,
                            b"<eol-anchor> \x00" as *const u8 as
                                *const libc::c_char);
                }
                3 | 4 => {
                    fprintf(out,
                            b"[C%hd] \x00" as *const u8 as
                                *const libc::c_char,
                            (*sym).id as libc::c_int);
                }
                _ => { }
            }
            fprintf(out, b"S%hd\x00" as *const u8 as *const libc::c_char,
                    (*(*sp).trans.offset(j as isize)).next_state as
                        libc::c_int);
            if (j as libc::c_int + 1 as libc::c_int) <
                   (*sp).ntrans as libc::c_int {
                putc(' ' as i32, out);
            }
            j = j.wrapping_add(1)
        }
        putc('\n' as i32, out);
        i = i.wrapping_add(1);
        sp = sp.offset(1)
    };
}
#[no_mangle]
#[c2rust::src_loc = "1956:1"]
pub unsafe extern "C" fn ure_exec(mut dfa: ure_dfa_t, mut flags: libc::c_int,
                                  mut text: *mut ucs2_t,
                                  mut textlen: libc::c_ulong,
                                  mut match_start: *mut libc::c_ulong,
                                  mut match_end: *mut libc::c_ulong)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut matched: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut ms: libc::c_ulong = 0;
    let mut me: libc::c_ulong = 0;
    let mut c: ucs4_t = 0;
    let mut sp: *mut ucs2_t = 0 as *mut ucs2_t;
    let mut ep: *mut ucs2_t = 0 as *mut ucs2_t;
    let mut lp: *mut ucs2_t = 0 as *mut ucs2_t;
    let mut stp: *mut _ure_dstate_t = 0 as *mut _ure_dstate_t;
    let mut sym: *mut _ure_symtab_t = 0 as *mut _ure_symtab_t;
    let mut rp: *mut _ure_range_t = 0 as *mut _ure_range_t;
    if dfa.is_null() || text.is_null() { return 0 as libc::c_int }
    /*
     * Handle the special case of an empty string matching the "^$" pattern.
     */
    if textlen == 0 as libc::c_int as libc::c_ulong &&
           (*dfa).flags & 0x2 as libc::c_int as libc::c_ulong != 0 {
        *match_end = 0 as libc::c_int as libc::c_ulong;
        *match_start = *match_end;
        return 1 as libc::c_int
    }
    sp = text;
    ep = sp.offset(textlen as isize);
    me = !(0 as libc::c_int) as libc::c_ulong;
    ms = me;
    stp = (*dfa).states;
    found = 0 as libc::c_int;
    while found == 0 as libc::c_int && sp < ep {
        lp = sp;
        let fresh26 = sp;
        sp = sp.offset(1);
        c = *fresh26 as ucs4_t;
        /*
         * Check to see if this is a high surrogate that should be
         * combined with a following low surrogate.
         */
        if sp < ep && 0xd800 as libc::c_int as libc::c_uint <= c &&
               c <= 0xdbff as libc::c_int as libc::c_uint &&
               0xdc00 as libc::c_int <= *sp as libc::c_int &&
               *sp as libc::c_int <= 0xdfff as libc::c_int {
            let fresh27 = sp;
            sp = sp.offset(1);
            c =
                (0x10000 as libc::c_int as
                     libc::c_uint).wrapping_add((c &
                                                     0x3ff as libc::c_int as
                                                         libc::c_uint) <<
                                                    10 as libc::c_int |
                                                    (*fresh27 as libc::c_int &
                                                         0x3ff as libc::c_int)
                                                        as libc::c_uint)
        }
        /*
         * Determine if the character is non-spacing and should be skipped.
         */
        if _ure_matches_properties(0x1 as libc::c_int as libc::c_ulong, c) !=
               0 && flags & 0x1 as libc::c_int != 0 {
            sp = sp.offset(1)
        } else {
            if (*dfa).flags & 0x1 as libc::c_int as libc::c_ulong != 0 {
                c = _ure_tolower(c)
            }
            /*
         * See if one of the transitions matches.
         */
            i = 0 as libc::c_int;
            matched = 0 as libc::c_int;
            while matched == 0 as libc::c_int &&
                      i < (*stp).ntrans as libc::c_int {
                sym =
                    (*dfa).syms.offset((*(*stp).trans.offset(i as
                                                                 isize)).symbol
                                           as libc::c_int as isize);
                match (*sym).type_0 as libc::c_int {
                    1 => {
                        if flags & 0x2 as libc::c_int != 0 ||
                               !(c == '\n' as i32 as libc::c_uint ||
                                     c == '\r' as i32 as libc::c_uint ||
                                     c ==
                                         0x2028 as libc::c_int as libc::c_uint
                                     ||
                                     c ==
                                         0x2029 as libc::c_int as
                                             libc::c_uint) {
                            matched = 1 as libc::c_int
                        }
                    }
                    2 => {
                        if c == (*sym).sym.chr { matched = 1 as libc::c_int }
                    }
                    5 => {
                        if lp == text {
                            sp = lp;
                            matched = 1 as libc::c_int
                        } else if c == '\n' as i32 as libc::c_uint ||
                                      c == '\r' as i32 as libc::c_uint ||
                                      c ==
                                          0x2028 as libc::c_int as
                                              libc::c_uint ||
                                      c ==
                                          0x2029 as libc::c_int as
                                              libc::c_uint {
                            if c == '\r' as i32 as libc::c_uint && sp < ep &&
                                   *sp as libc::c_int == '\n' as i32 {
                                sp = sp.offset(1)
                            }
                            lp = sp;
                            matched = 1 as libc::c_int
                        }
                    }
                    6 => {
                        if c == '\n' as i32 as libc::c_uint ||
                               c == '\r' as i32 as libc::c_uint ||
                               c == 0x2028 as libc::c_int as libc::c_uint ||
                               c == 0x2029 as libc::c_int as libc::c_uint {
                            /*
                     * Put the pointer back before the separator so the match
                     * end position will be correct.  This case will also
                     * cause the `sp' pointer to be advanced over the current
                     * separator once the match end point has been recorded.
                     */
                            sp = lp;
                            matched = 1 as libc::c_int
                        }
                    }
                    3 | 4 => {
                        if (*sym).props != 0 as libc::c_int as libc::c_ulong {
                            matched = _ure_matches_properties((*sym).props, c)
                        }
                        j = 0 as libc::c_int;
                        rp = (*sym).sym.ccl.ranges;
                        while j < (*sym).sym.ccl.ranges_used as libc::c_int {
                            if (*rp).min_code <= c && c <= (*rp).max_code {
                                matched = 1 as libc::c_int
                            }
                            j += 1;
                            rp = rp.offset(1)
                        }
                        if (*sym).type_0 as libc::c_int == 4 as libc::c_int {
                            matched = (matched == 0) as libc::c_int
                        }
                    }
                    _ => { }
                }
                if matched != 0 {
                    if ms == !(0 as libc::c_ulong) {
                        ms =
                            lp.wrapping_offset_from(text) as libc::c_long as
                                libc::c_ulong
                    } else {
                        me =
                            sp.wrapping_offset_from(text) as libc::c_long as
                                libc::c_ulong
                    }
                    stp =
                        (*dfa).states.offset((*(*stp).trans.offset(i as
                                                                       isize)).next_state
                                                 as libc::c_int as isize);
                    /*
                 * If the match was an EOL anchor, adjust the pointer past the
                 * separator that caused the match.  The correct match
                 * position has been recorded already.
                 */
                    if (*sym).type_0 as libc::c_int == 6 as libc::c_int {
                        /*
                     * Skip the character that caused the match.
                     */
                        sp = sp.offset(1);
                        /*
                     * Handle the infamous CRLF situation.
                     */
                        if sp < ep && c == '\r' as i32 as libc::c_uint &&
                               *sp as libc::c_int == '\n' as i32 {
                            sp = sp.offset(1)
                        }
                    }
                }
                i += 1
            }
            if matched == 0 as libc::c_int {
                if (*stp).accepting as libc::c_int == 0 as libc::c_int {
                    /*
                 * If the last state was not accepting, then reset
                 * and start over.
                 */
                    stp = (*dfa).states;
                    me = !(0 as libc::c_int) as libc::c_ulong;
                    ms = me
                } else {
                    /*
               * The last state was accepting, so terminate the matching
               * loop to avoid more work.
               */
                    found = 1 as libc::c_int
                }
            } else if sp == ep {
                if (*stp).accepting == 0 {
                    /*
                 * This ugly hack is to make sure the end-of-line anchors
                 * match when the source text hits the end.  This is only done
                 * if the last subexpression matches.
                 */
                    i = 0 as libc::c_int;
                    while found == 0 as libc::c_int &&
                              i < (*stp).ntrans as libc::c_int {
                        sym =
                            (*dfa).syms.offset((*(*stp).trans.offset(i as
                                                                         isize)).symbol
                                                   as libc::c_int as isize);
                        if (*sym).type_0 as libc::c_int == 6 as libc::c_int {
                            stp =
                                (*dfa).states.offset((*(*stp).trans.offset(i
                                                                               as
                                                                               isize)).next_state
                                                         as libc::c_int as
                                                         isize);
                            if !((*stp).accepting != 0) { break ; }
                            me =
                                sp.wrapping_offset_from(text) as libc::c_long
                                    as libc::c_ulong;
                            found = 1 as libc::c_int
                        }
                        i += 1
                    }
                } else {
                    /*
                 * Make sure any conditions that match all the way to the end
                 * of the string match.
                 */
                    found = 1 as libc::c_int;
                    me =
                        sp.wrapping_offset_from(text) as libc::c_long as
                            libc::c_ulong
                }
            }
        }
    }
    if found == 0 as libc::c_int {
        me = !(0 as libc::c_int) as libc::c_ulong;
        ms = me
    }
    *match_start = ms;
    *match_end = me;
    return if ms != !(0 as libc::c_ulong) {
               1 as libc::c_int
           } else { 0 as libc::c_int };
}
unsafe extern "C" fn run_static_initializers() {
    cclass_trie =
        [{
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x3a as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(1 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x61 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(9 as libc::c_int as libc::c_uint);
             init.set_next(10 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x63 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(8 as libc::c_int as libc::c_uint);
             init.set_next(19 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x64 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(7 as libc::c_int as libc::c_uint);
             init.set_next(24 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x67 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(6 as libc::c_int as libc::c_uint);
             init.set_next(29 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x6c as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(5 as libc::c_int as libc::c_uint);
             init.set_next(34 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x70 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(4 as libc::c_int as libc::c_uint);
             init.set_next(39 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x73 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(3 as libc::c_int as libc::c_uint);
             init.set_next(49 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x75 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(2 as libc::c_int as libc::c_uint);
             init.set_next(54 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x78 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(59 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x6c as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(11 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x6e as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(2 as libc::c_int as libc::c_uint);
             init.set_next(13 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x70 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(16 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x75 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(14 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x6d as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(15 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x3a as libc::c_int as ucs2_t,
                             func:
                                 Some(_ure_ccl_setup as
                                          unsafe extern "C" fn(_:
                                                                   *mut _ure_symtab_t,
                                                               _:
                                                                   libc::c_ulong,
                                                               _:
                                                                   *mut _ure_buffer_t)
                                              -> ()),
                             mask:
                                 (0x200 as libc::c_int | 0x400 as libc::c_int
                                      | 0x2000 as libc::c_int |
                                      0x1000 as libc::c_int |
                                      0x800 as libc::c_int |
                                      0x1 as libc::c_int | 0x2 as libc::c_int
                                      | 0x4 as libc::c_int) as
                                     libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(16 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x68 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(17 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x61 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(18 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x3a as libc::c_int as ucs2_t,
                             func:
                                 Some(_ure_ccl_setup as
                                          unsafe extern "C" fn(_:
                                                                   *mut _ure_symtab_t,
                                                               _:
                                                                   libc::c_ulong,
                                                               _:
                                                                   *mut _ure_buffer_t)
                                              -> ()),
                             mask:
                                 (0x200 as libc::c_int | 0x400 as libc::c_int
                                      | 0x2000 as libc::c_int |
                                      0x1000 as libc::c_int |
                                      0x800 as libc::c_int |
                                      0x1 as libc::c_int | 0x2 as libc::c_int)
                                     as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(19 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x6e as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(20 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x74 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(21 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x72 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(22 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x6c as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(23 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x3a as libc::c_int as ucs2_t,
                             func:
                                 Some(_ure_ccl_setup as
                                          unsafe extern "C" fn(_:
                                                                   *mut _ure_symtab_t,
                                                               _:
                                                                   libc::c_ulong,
                                                               _:
                                                                   *mut _ure_buffer_t)
                                              -> ()),
                             mask: 0x80 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(24 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x69 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(25 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x67 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(26 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x69 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(27 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x74 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(28 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x3a as libc::c_int as ucs2_t,
                             func:
                                 Some(_ure_ccl_setup as
                                          unsafe extern "C" fn(_:
                                                                   *mut _ure_symtab_t,
                                                               _:
                                                                   libc::c_ulong,
                                                               _:
                                                                   *mut _ure_buffer_t)
                                              -> ()),
                             mask: 0x4 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(29 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x72 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(30 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x61 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(31 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x70 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(32 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x68 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(33 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x3a as libc::c_int as ucs2_t,
                             func:
                                 Some(_ure_ccl_setup as
                                          unsafe extern "C" fn(_:
                                                                   *mut _ure_symtab_t,
                                                               _:
                                                                   libc::c_ulong,
                                                               _:
                                                                   *mut _ure_buffer_t)
                                              -> ()),
                             mask:
                                 (0x4 as libc::c_int | 0x8 as libc::c_int |
                                      (0x200 as libc::c_int |
                                           0x400 as libc::c_int |
                                           0x2000 as libc::c_int |
                                           0x1000 as libc::c_int |
                                           0x800 as libc::c_int |
                                           0x1 as libc::c_int |
                                           0x2 as libc::c_int) |
                                      0x40000 as libc::c_int |
                                      0x80000 as libc::c_int |
                                      0x100000 as libc::c_int) as
                                     libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(34 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x6f as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(35 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x77 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(36 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x65 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(37 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x72 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(38 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x3a as libc::c_int as ucs2_t,
                             func:
                                 Some(_ure_ccl_setup as
                                          unsafe extern "C" fn(_:
                                                                   *mut _ure_symtab_t,
                                                               _:
                                                                   libc::c_ulong,
                                                               _:
                                                                   *mut _ure_buffer_t)
                                              -> ()),
                             mask: 0x400 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(39 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x72 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(2 as libc::c_int as libc::c_uint);
             init.set_next(41 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x75 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(45 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x69 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(42 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x6e as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(43 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x74 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(44 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x3a as libc::c_int as ucs2_t,
                             func:
                                 Some(_ure_ccl_setup as
                                          unsafe extern "C" fn(_:
                                                                   *mut _ure_symtab_t,
                                                               _:
                                                                   libc::c_ulong,
                                                               _:
                                                                   *mut _ure_buffer_t)
                                              -> ()),
                             mask:
                                 (0x4 as libc::c_int | 0x8 as libc::c_int |
                                      (0x200 as libc::c_int |
                                           0x400 as libc::c_int |
                                           0x2000 as libc::c_int |
                                           0x1000 as libc::c_int |
                                           0x800 as libc::c_int |
                                           0x1 as libc::c_int |
                                           0x2 as libc::c_int) |
                                      0x40000 as libc::c_int |
                                      0x80000 as libc::c_int |
                                      0x100000 as libc::c_int |
                                      0x10 as libc::c_int) as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(45 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x6e as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(46 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x63 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(47 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x74 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(48 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x3a as libc::c_int as ucs2_t,
                             func:
                                 Some(_ure_ccl_setup as
                                          unsafe extern "C" fn(_:
                                                                   *mut _ure_symtab_t,
                                                               _:
                                                                   libc::c_ulong,
                                                               _:
                                                                   *mut _ure_buffer_t)
                                              -> ()),
                             mask:
                                 (0x4000 as libc::c_int |
                                      0x8000 as libc::c_int |
                                      0x10000 as libc::c_int |
                                      0x20000 as libc::c_int) as
                                     libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(49 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x70 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(50 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x61 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(51 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x63 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(52 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x65 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(53 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x3a as libc::c_int as ucs2_t,
                             func:
                                 Some(_ure_space_setup as
                                          unsafe extern "C" fn(_:
                                                                   *mut _ure_symtab_t,
                                                               _:
                                                                   libc::c_ulong,
                                                               _:
                                                                   *mut _ure_buffer_t)
                                              -> ()),
                             mask:
                                 (0x10 as libc::c_int | 0x20 as libc::c_int |
                                      0x40 as libc::c_int) as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(54 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x70 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(55 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x70 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(56 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x65 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(57 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x72 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(58 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x3a as libc::c_int as ucs2_t,
                             func:
                                 Some(_ure_ccl_setup as
                                          unsafe extern "C" fn(_:
                                                                   *mut _ure_symtab_t,
                                                               _:
                                                                   libc::c_ulong,
                                                               _:
                                                                   *mut _ure_buffer_t)
                                              -> ()),
                             mask: 0x200 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(59 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x64 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(60 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x69 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(61 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x67 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(62 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x69 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(63 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x74 as libc::c_int as ucs2_t,
                             func: None,
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(64 as libc::c_int as libc::c_uint);
             init
         },
         {
             let mut init =
                 _ure_trie_t{len_next: [0; 2],
                             c2rust_padding: [0; 4],
                             key: 0x3a as libc::c_int as ucs2_t,
                             func:
                                 Some(_ure_xdigit_setup as
                                          unsafe extern "C" fn(_:
                                                                   *mut _ure_symtab_t,
                                                               _:
                                                                   libc::c_ulong,
                                                               _:
                                                                   *mut _ure_buffer_t)
                                              -> ()),
                             mask: 0 as libc::c_int as libc::c_ulong,};
             init.set_len(1 as libc::c_int as libc::c_uint);
             init.set_next(65 as libc::c_int as libc::c_uint);
             init
         }]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
