use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:37"]
pub mod types_h {
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:37"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:37"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:37"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/ctype.h:37"]
pub mod ctype_h {
    #[c2rust::src_loc = "46:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed = 8;
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
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed = 8192;
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/tests/asn.1/trval.c:37"]
pub mod trval_c {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "427:8"]
    pub struct typestring_table {
        pub k1: libc::c_int,
        pub k2: libc::c_int,
        pub str_0: *mut libc::c_char,
        pub new_appl: libc::c_int,
    }
    /* ***************************************************************************/
    #[c2rust::src_loc = "123:1"]
    pub unsafe extern "C" fn convert_nibble(mut ch: libc::c_int)
     -> libc::c_int {
        if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int &
               _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0 {
            return ch - '0' as i32
        }
        if ch >= 'a' as i32 && ch <= 'f' as i32 {
            return ch - 'a' as i32 + 10 as libc::c_int
        }
        if ch >= 'A' as i32 && ch <= 'F' as i32 {
            return ch - 'A' as i32 + 10 as libc::c_int
        }
        return -(1 as libc::c_int);
    }
    #[no_mangle]
    #[c2rust::src_loc = "134:1"]
    pub unsafe extern "C" fn trval(mut fin: *mut FILE, mut fout: *mut FILE)
     -> libc::c_int {
        let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut maxlen: libc::c_uint = 0;
        let mut len: libc::c_int = 0;
        let mut cc: libc::c_int = 0;
        let mut cc2: libc::c_int = 0;
        let mut n1: libc::c_int = 0;
        let mut n2: libc::c_int = 0;
        let mut r: libc::c_int = 0;
        let mut rlen: libc::c_int = 0;
        maxlen = 8192 as libc::c_int as libc::c_uint;
        p = malloc(maxlen as libc::c_ulong) as *mut libc::c_uchar;
        len = 0 as libc::c_int;
        loop  {
            cc = fgetc(fin);
            if !(cc != -(1 as libc::c_int)) { break ; }
            if len as libc::c_uint == maxlen {
                maxlen =
                    maxlen.wrapping_add(8192 as libc::c_int as libc::c_uint);
                p =
                    realloc(p as *mut libc::c_void, maxlen as libc::c_ulong)
                        as *mut libc::c_uchar
            }
            if do_hex != 0 {
                if cc == ' ' as i32 || cc == '\n' as i32 || cc == '\t' as i32
                   {
                    continue ;
                }
                cc2 = fgetc(fin);
                if cc2 == -(1 as libc::c_int) { break ; }
                n1 = convert_nibble(cc);
                n2 = convert_nibble(cc2);
                cc = (n1 << 4 as libc::c_int) + n2
            }
            let fresh0 = len;
            len = len + 1;
            *p.offset(fresh0 as isize) = cc as libc::c_uchar
        }
        fprintf(fout, b"<%d>\x00" as *const u8 as *const libc::c_char, len);
        r = trval2(fout, p, len, 0 as libc::c_int, &mut rlen);
        fprintf(fout, b"\n\x00" as *const u8 as *const libc::c_char);
        free(p as *mut libc::c_void);
        return r;
    }
    /*
 * This is the printing function for bit strings
 */
    /*
 * This is the printing function for integers
 */
    /*
 * This is the printing function which we use if it's a string or
 * other other type which is best printed as a string
 */
    /*
     * Only try this printing function with "reasonable" types
     */
    #[no_mangle]
    #[c2rust::src_loc = "480:25"]
    pub static mut krb5_types: [typestring_table; 19] =
        [{
             let mut init =
                 typestring_table{k1: 1 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Krb5 Ticket\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 2 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Krb5 Authenticator\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 3 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Krb5 Encrypted ticket part\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 10 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Krb5 AS-REQ packet\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 11 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Krb5 AS-REP packet\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 12 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Krb5 TGS-REQ packet\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 13 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Krb5 TGS-REP packet\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 14 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Krb5 AP-REQ packet\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 15 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Krb5 AP-REP packet\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 20 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Krb5 SAFE packet\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 21 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Krb5 PRIV packet\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 22 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Krb5 CRED packet\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 30 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Krb5 ERROR packet\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 25 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Krb5 Encrypted AS-REP part\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 26 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Krb5 Encrypted TGS-REP part\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 27 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Krb5 Encrypted AP-REP part\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 28 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Krb5 Encrypted PRIV part\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 29 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Krb5 Encrypted CRED part\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: -(1 as libc::c_int),
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      0 as *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         }];
    #[no_mangle]
    #[c2rust::src_loc = "502:25"]
    pub static mut krb5_fields: [typestring_table; 167] =
        [{
             let mut init =
                 typestring_table{k1: 1000 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"name-type\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1000 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"name-string\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1001 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"etype\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1001 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"kvno\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1001 as libc::c_int,
                                  k2: 2 as libc::c_int,
                                  str_0:
                                      b"cipher\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1002 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"addr-type\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1002 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"address\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1003 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"addr-type\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1003 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"address\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1004 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"ad-type\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1004 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"ad-data\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1005 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"keytype\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1005 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"keyvalue\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1006 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"cksumtype\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1006 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"checksum\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1007 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"kdc-options\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1007 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"cname\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1000 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1007 as libc::c_int,
                                  k2: 2 as libc::c_int,
                                  str_0:
                                      b"realm\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1007 as libc::c_int,
                                  k2: 3 as libc::c_int,
                                  str_0:
                                      b"sname\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1000 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1007 as libc::c_int,
                                  k2: 4 as libc::c_int,
                                  str_0:
                                      b"from\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1007 as libc::c_int,
                                  k2: 5 as libc::c_int,
                                  str_0:
                                      b"till\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1007 as libc::c_int,
                                  k2: 6 as libc::c_int,
                                  str_0:
                                      b"rtime\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1007 as libc::c_int,
                                  k2: 7 as libc::c_int,
                                  str_0:
                                      b"nonce\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1007 as libc::c_int,
                                  k2: 8 as libc::c_int,
                                  str_0:
                                      b"etype\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1007 as libc::c_int,
                                  k2: 9 as libc::c_int,
                                  str_0:
                                      b"addresses\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1003 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1007 as libc::c_int,
                                  k2: 10 as libc::c_int,
                                  str_0:
                                      b"enc-authorization-data\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1001 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1007 as libc::c_int,
                                  k2: 11 as libc::c_int,
                                  str_0:
                                      b"additional-tickets\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1008 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"padata-type\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1008 as libc::c_int,
                                  k2: 2 as libc::c_int,
                                  str_0:
                                      b"pa-data\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1009 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"user-data\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1009 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"timestamp\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1009 as libc::c_int,
                                  k2: 2 as libc::c_int,
                                  str_0:
                                      b"usec\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1009 as libc::c_int,
                                  k2: 3 as libc::c_int,
                                  str_0:
                                      b"seq-number\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1009 as libc::c_int,
                                  k2: 4 as libc::c_int,
                                  str_0:
                                      b"s-address\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1002 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1009 as libc::c_int,
                                  k2: 5 as libc::c_int,
                                  str_0:
                                      b"r-address\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1002 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1010 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"lr-type\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1010 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"lr-value\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1011 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"key\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1005 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1011 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"prealm\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1011 as libc::c_int,
                                  k2: 2 as libc::c_int,
                                  str_0:
                                      b"pname\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1000 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1011 as libc::c_int,
                                  k2: 3 as libc::c_int,
                                  str_0:
                                      b"flags\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1011 as libc::c_int,
                                  k2: 4 as libc::c_int,
                                  str_0:
                                      b"authtime\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1011 as libc::c_int,
                                  k2: 5 as libc::c_int,
                                  str_0:
                                      b"startime\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1011 as libc::c_int,
                                  k2: 6 as libc::c_int,
                                  str_0:
                                      b"endtime\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1011 as libc::c_int,
                                  k2: 7 as libc::c_int,
                                  str_0:
                                      b"renew-till\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1011 as libc::c_int,
                                  k2: 8 as libc::c_int,
                                  str_0:
                                      b"srealm\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1011 as libc::c_int,
                                  k2: 9 as libc::c_int,
                                  str_0:
                                      b"sname\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1000 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1011 as libc::c_int,
                                  k2: 10 as libc::c_int,
                                  str_0:
                                      b"caddr\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1002 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"tkt-vno\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"realm\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1 as libc::c_int,
                                  k2: 2 as libc::c_int,
                                  str_0:
                                      b"sname\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1000 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 1 as libc::c_int,
                                  k2: 3 as libc::c_int,
                                  str_0:
                                      b"tkt-enc-part\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1001 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 2 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"authenticator-vno\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 2 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"crealm\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 2 as libc::c_int,
                                  k2: 2 as libc::c_int,
                                  str_0:
                                      b"cname\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1000 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 2 as libc::c_int,
                                  k2: 3 as libc::c_int,
                                  str_0:
                                      b"cksum\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1006 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 2 as libc::c_int,
                                  k2: 4 as libc::c_int,
                                  str_0:
                                      b"cusec\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 2 as libc::c_int,
                                  k2: 5 as libc::c_int,
                                  str_0:
                                      b"ctime\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 2 as libc::c_int,
                                  k2: 6 as libc::c_int,
                                  str_0:
                                      b"subkey\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1005 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 2 as libc::c_int,
                                  k2: 7 as libc::c_int,
                                  str_0:
                                      b"seq-number\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 2 as libc::c_int,
                                  k2: 8 as libc::c_int,
                                  str_0:
                                      b"authorization-data\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1004 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 3 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"flags\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 3 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"key\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1005 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 3 as libc::c_int,
                                  k2: 2 as libc::c_int,
                                  str_0:
                                      b"crealm\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 3 as libc::c_int,
                                  k2: 3 as libc::c_int,
                                  str_0:
                                      b"cname\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1000 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 3 as libc::c_int,
                                  k2: 4 as libc::c_int,
                                  str_0:
                                      b"transited\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 3 as libc::c_int,
                                  k2: 5 as libc::c_int,
                                  str_0:
                                      b"authtime\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 3 as libc::c_int,
                                  k2: 6 as libc::c_int,
                                  str_0:
                                      b"starttime\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 3 as libc::c_int,
                                  k2: 7 as libc::c_int,
                                  str_0:
                                      b"endtime\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 3 as libc::c_int,
                                  k2: 8 as libc::c_int,
                                  str_0:
                                      b"renew-till\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 3 as libc::c_int,
                                  k2: 9 as libc::c_int,
                                  str_0:
                                      b"caddr\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1003 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 3 as libc::c_int,
                                  k2: 10 as libc::c_int,
                                  str_0:
                                      b"authorization-data\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1004 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 10 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"pvno\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 10 as libc::c_int,
                                  k2: 2 as libc::c_int,
                                  str_0:
                                      b"msg-type\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 10 as libc::c_int,
                                  k2: 3 as libc::c_int,
                                  str_0:
                                      b"padata\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1008 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 10 as libc::c_int,
                                  k2: 4 as libc::c_int,
                                  str_0:
                                      b"req-body\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1007 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 11 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"pvno\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 11 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"msg-type\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 11 as libc::c_int,
                                  k2: 2 as libc::c_int,
                                  str_0:
                                      b"padata\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1008 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 11 as libc::c_int,
                                  k2: 3 as libc::c_int,
                                  str_0:
                                      b"crealm\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 11 as libc::c_int,
                                  k2: 4 as libc::c_int,
                                  str_0:
                                      b"cname\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1000 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 11 as libc::c_int,
                                  k2: 5 as libc::c_int,
                                  str_0:
                                      b"ticket\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 11 as libc::c_int,
                                  k2: 6 as libc::c_int,
                                  str_0:
                                      b"enc-part\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1001 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 12 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"pvno\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 12 as libc::c_int,
                                  k2: 2 as libc::c_int,
                                  str_0:
                                      b"msg-type\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 12 as libc::c_int,
                                  k2: 3 as libc::c_int,
                                  str_0:
                                      b"padata\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1008 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 12 as libc::c_int,
                                  k2: 4 as libc::c_int,
                                  str_0:
                                      b"req-body\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1007 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 13 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"pvno\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 13 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"msg-type\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 13 as libc::c_int,
                                  k2: 2 as libc::c_int,
                                  str_0:
                                      b"padata\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1008 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 13 as libc::c_int,
                                  k2: 3 as libc::c_int,
                                  str_0:
                                      b"crealm\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 13 as libc::c_int,
                                  k2: 4 as libc::c_int,
                                  str_0:
                                      b"cname\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1000 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 13 as libc::c_int,
                                  k2: 5 as libc::c_int,
                                  str_0:
                                      b"ticket\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 13 as libc::c_int,
                                  k2: 6 as libc::c_int,
                                  str_0:
                                      b"enc-part\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1001 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 14 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"pvno\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 14 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"msg-type\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 14 as libc::c_int,
                                  k2: 2 as libc::c_int,
                                  str_0:
                                      b"ap-options\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 14 as libc::c_int,
                                  k2: 3 as libc::c_int,
                                  str_0:
                                      b"ticket\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 14 as libc::c_int,
                                  k2: 4 as libc::c_int,
                                  str_0:
                                      b"authenticator\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1001 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 15 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"pvno\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 15 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"msg-type\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 15 as libc::c_int,
                                  k2: 2 as libc::c_int,
                                  str_0:
                                      b"enc-part\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1001 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 20 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"pvno\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 20 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"msg-type\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 20 as libc::c_int,
                                  k2: 2 as libc::c_int,
                                  str_0:
                                      b"safe-body\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1009 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 20 as libc::c_int,
                                  k2: 3 as libc::c_int,
                                  str_0:
                                      b"cksum\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1006 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 21 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"pvno\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 21 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"msg-type\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 21 as libc::c_int,
                                  k2: 2 as libc::c_int,
                                  str_0:
                                      b"enc-part\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1001 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 22 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"pvno\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 22 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"msg-type\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 22 as libc::c_int,
                                  k2: 2 as libc::c_int,
                                  str_0:
                                      b"tickets\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 22 as libc::c_int,
                                  k2: 3 as libc::c_int,
                                  str_0:
                                      b"enc-part\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1001 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 25 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"key\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1005 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 25 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"last-req\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1010 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 25 as libc::c_int,
                                  k2: 2 as libc::c_int,
                                  str_0:
                                      b"nonce\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 25 as libc::c_int,
                                  k2: 3 as libc::c_int,
                                  str_0:
                                      b"key-expiration\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 25 as libc::c_int,
                                  k2: 4 as libc::c_int,
                                  str_0:
                                      b"flags\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 25 as libc::c_int,
                                  k2: 5 as libc::c_int,
                                  str_0:
                                      b"authtime\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 25 as libc::c_int,
                                  k2: 6 as libc::c_int,
                                  str_0:
                                      b"starttime\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 25 as libc::c_int,
                                  k2: 7 as libc::c_int,
                                  str_0:
                                      b"enddtime\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 25 as libc::c_int,
                                  k2: 8 as libc::c_int,
                                  str_0:
                                      b"renew-till\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 25 as libc::c_int,
                                  k2: 9 as libc::c_int,
                                  str_0:
                                      b"srealm\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 25 as libc::c_int,
                                  k2: 10 as libc::c_int,
                                  str_0:
                                      b"sname\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1000 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 25 as libc::c_int,
                                  k2: 11 as libc::c_int,
                                  str_0:
                                      b"caddr\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1003 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 26 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"key\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1005 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 26 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"last-req\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1010 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 26 as libc::c_int,
                                  k2: 2 as libc::c_int,
                                  str_0:
                                      b"nonce\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 26 as libc::c_int,
                                  k2: 3 as libc::c_int,
                                  str_0:
                                      b"key-expiration\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 26 as libc::c_int,
                                  k2: 4 as libc::c_int,
                                  str_0:
                                      b"flags\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 26 as libc::c_int,
                                  k2: 5 as libc::c_int,
                                  str_0:
                                      b"authtime\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 26 as libc::c_int,
                                  k2: 6 as libc::c_int,
                                  str_0:
                                      b"starttime\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 26 as libc::c_int,
                                  k2: 7 as libc::c_int,
                                  str_0:
                                      b"enddtime\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 26 as libc::c_int,
                                  k2: 8 as libc::c_int,
                                  str_0:
                                      b"renew-till\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 26 as libc::c_int,
                                  k2: 9 as libc::c_int,
                                  str_0:
                                      b"srealm\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 26 as libc::c_int,
                                  k2: 10 as libc::c_int,
                                  str_0:
                                      b"sname\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1000 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 26 as libc::c_int,
                                  k2: 11 as libc::c_int,
                                  str_0:
                                      b"caddr\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1003 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 27 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"ctime\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 27 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"cusec\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 27 as libc::c_int,
                                  k2: 2 as libc::c_int,
                                  str_0:
                                      b"subkey\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1005 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 27 as libc::c_int,
                                  k2: 3 as libc::c_int,
                                  str_0:
                                      b"seq-number\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 28 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"user-data\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 28 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"timestamp\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 28 as libc::c_int,
                                  k2: 2 as libc::c_int,
                                  str_0:
                                      b"usec\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 28 as libc::c_int,
                                  k2: 3 as libc::c_int,
                                  str_0:
                                      b"seq-number\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 28 as libc::c_int,
                                  k2: 4 as libc::c_int,
                                  str_0:
                                      b"s-address\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1002 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 28 as libc::c_int,
                                  k2: 5 as libc::c_int,
                                  str_0:
                                      b"r-address\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1002 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 29 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"ticket-info\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1011 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 29 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"nonce\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 29 as libc::c_int,
                                  k2: 2 as libc::c_int,
                                  str_0:
                                      b"timestamp\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 29 as libc::c_int,
                                  k2: 3 as libc::c_int,
                                  str_0:
                                      b"usec\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 29 as libc::c_int,
                                  k2: 4 as libc::c_int,
                                  str_0:
                                      b"s-address\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1002 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 29 as libc::c_int,
                                  k2: 5 as libc::c_int,
                                  str_0:
                                      b"r-address\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1002 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 30 as libc::c_int,
                                  k2: 0 as libc::c_int,
                                  str_0:
                                      b"pvno\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 30 as libc::c_int,
                                  k2: 1 as libc::c_int,
                                  str_0:
                                      b"msg-type\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 30 as libc::c_int,
                                  k2: 2 as libc::c_int,
                                  str_0:
                                      b"ctime\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 30 as libc::c_int,
                                  k2: 3 as libc::c_int,
                                  str_0:
                                      b"cusec\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 30 as libc::c_int,
                                  k2: 4 as libc::c_int,
                                  str_0:
                                      b"stime\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 30 as libc::c_int,
                                  k2: 5 as libc::c_int,
                                  str_0:
                                      b"susec\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 30 as libc::c_int,
                                  k2: 6 as libc::c_int,
                                  str_0:
                                      b"error-code\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 30 as libc::c_int,
                                  k2: 7 as libc::c_int,
                                  str_0:
                                      b"crealm\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 30 as libc::c_int,
                                  k2: 8 as libc::c_int,
                                  str_0:
                                      b"cname\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1000 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 30 as libc::c_int,
                                  k2: 9 as libc::c_int,
                                  str_0:
                                      b"realm\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 30 as libc::c_int,
                                  k2: 10 as libc::c_int,
                                  str_0:
                                      b"sname\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 1000 as libc::c_int,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 30 as libc::c_int,
                                  k2: 11 as libc::c_int,
                                  str_0:
                                      b"e-text\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 30 as libc::c_int,
                                  k2: 12 as libc::c_int,
                                  str_0:
                                      b"e-data\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: -(1 as libc::c_int),
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      0 as *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         }];
    #[c2rust::src_loc = "433:1"]
    pub unsafe extern "C" fn lookup_typestring(mut table:
                                                   *mut typestring_table,
                                               mut key1: libc::c_int,
                                               mut key2: libc::c_int)
     -> *mut libc::c_char {
        let mut ent: *mut typestring_table = 0 as *mut typestring_table;
        ent = table;
        while (*ent).k1 > 0 as libc::c_int {
            if (*ent).k1 == key1 && (*ent).k2 == key2 {
                if (*ent).new_appl != 0 {
                    current_appl_type = (*ent).new_appl
                }
                return (*ent).str_0
            }
            ent = ent.offset(1)
        }
        return 0 as *mut libc::c_char;
    }
    #[no_mangle]
    #[c2rust::src_loc = "451:25"]
    pub static mut univ_types: [typestring_table; 25] =
        [{
             let mut init =
                 typestring_table{k1: 0x1 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Boolean\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 0x2 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Integer\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 0x3 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Bit String\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 0x4 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Octet String\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 0x5 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Null\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 0x6 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Object Identifier\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 0x7 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Object Descriptor\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 0x8 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"External\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 0x9 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Real\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 0xa as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Enumerated type\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 0xb as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Encrypted\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 0x10 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Sequence/Sequence Of\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 0x11 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Set/Set Of\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 0x12 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Numeric String\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 0x13 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Printable String\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 0x14 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"T.61 String\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 0x15 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Videotex String\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 0x16 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"IA5 String\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 0x17 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"UTCTime\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 0x18 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Generalized Time\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 0x19 as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Graphics string (ISO2375)\x00" as
                                          *const u8 as *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 0x1a as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Visible string\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 0x1b as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"General string\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: 0x1c as libc::c_int,
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      b"Character string\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         },
         {
             let mut init =
                 typestring_table{k1: -(1 as libc::c_int),
                                  k2: -(1 as libc::c_int),
                                  str_0:
                                      0 as *const libc::c_char as
                                          *mut libc::c_char,
                                  new_appl: 0,};
             init
         }];
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 1992,1993 Trusted Information Systems, Inc.
 *
 * Permission to include this software in the Kerberos V5 distribution
 * was graciously provided by Trusted Information Systems.
 *
 * Trusted Information Systems makes no representation about the
 * suitability of this software for any purpose.  It is provided
 * "as is" without express or implied warranty.
 */
/*
 * Copyright (C) 1994 Massachusetts Institute of Technology
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
    /* ****************************************************************************
 * trval.c.c
 *****************************************************************************/
    /* IDENTIFIER OCTET = TAG CLASS | FORM OF ENCODING | TAG NUMBER */
    /* TAG CLASSES */
    /* bits 8 and 7 */
    /* 0 = universal */
    /* 1 = application */
    /* 2 = context-specific */
    /* 3 = private */
    /* FORM OF ENCODING */
    /* bit 6 */
    /* 0 = primitive */
    /* 1 = constructed */
    /* TAG NUMBERS */
    /* bits 5-1 */
    /* Boolean */
    /* Integer */
    /* Bit String */
    /* Octet String */
    /* Null */
    /* Object Identifier */
    /* Object Descriptor */
    /* External */
    /* Real */
    /* Enumerated type */
    /* Encrypted */
    /* SEQUENCE/SEQUENCE OF */
    /* SET/SET OF */
    /* Numeric String */
    /* Printable String */
    /* T.61 String */
    /* Videotex String */
    /* IA5 String */
    /* UTCTime */
    /* Generalized Time */
    /* Graphics string (ISO2375) */
    /* Visible string */
    /* General string */
    /* Character string */
    /* long or indefinite form */
    /* largest short form */
    /* mask to get number of bytes in length */
    /* indefinite length */
    /* Do krb5 application types */
    #[no_mangle]
    #[c2rust::src_loc = "703:1"]
    pub unsafe extern "C" fn print_tag_type(mut fp: *mut FILE,
                                            mut eid: libc::c_int,
                                            mut lev: libc::c_int) {
        let mut tag: libc::c_int = eid & 0x1f as libc::c_int;
        let mut do_space: libc::c_int = 1 as libc::c_int;
        let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
        fprintf(fp, b"[\x00" as *const u8 as *const libc::c_char);
        let mut current_block_13: u64;
        match eid & 0xc0 as libc::c_int {
            0 => {
                if print_types != 0 && print_skip_tagnum != 0 {
                    do_space = 0 as libc::c_int
                } else {
                    fprintf(fp,
                            b"UNIV %d\x00" as *const u8 as
                                *const libc::c_char, tag);
                }
            }
            64 => {
                current_appl_type = tag;
                if print_krb5_types != 0 {
                    str =
                        lookup_typestring(krb5_types.as_mut_ptr(), tag,
                                          -(1 as libc::c_int));
                    if !str.is_null() {
                        fputs(str, fp);
                        current_block_13 = 13472856163611868459;
                    } else { current_block_13 = 2979737022853876585; }
                } else { current_block_13 = 2979737022853876585; }
                match current_block_13 {
                    13472856163611868459 => { }
                    _ => {
                        fprintf(fp,
                                b"APPL %d\x00" as *const u8 as
                                    *const libc::c_char, tag);
                    }
                }
            }
            128 => {
                if print_krb5_types != 0 && current_appl_type != 0 {
                    str =
                        lookup_typestring(krb5_fields.as_mut_ptr(),
                                          current_appl_type, tag);
                    if !str.is_null() {
                        fputs(str, fp);
                        current_block_13 = 13472856163611868459;
                    } else { current_block_13 = 7175849428784450219; }
                } else { current_block_13 = 7175849428784450219; }
                match current_block_13 {
                    13472856163611868459 => { }
                    _ => {
                        if print_skip_context != 0 && lev != 0 {
                            fprintf(fp,
                                    b"%d\x00" as *const u8 as
                                        *const libc::c_char, tag);
                        } else {
                            fprintf(fp,
                                    b"CONT %d\x00" as *const u8 as
                                        *const libc::c_char, tag);
                        }
                    }
                }
            }
            192 => {
                fprintf(fp,
                        b"PRIV %d\x00" as *const u8 as *const libc::c_char,
                        tag);
            }
            _ => { }
        }
        if print_types != 0 && eid & 0xc0 as libc::c_int == 0 as libc::c_int {
            if do_space != 0 {
                fputs(b" \x00" as *const u8 as *const libc::c_char, fp);
            }
            str =
                lookup_typestring(univ_types.as_mut_ptr(),
                                  eid & 0x1f as libc::c_int,
                                  -(1 as libc::c_int));
            if !str.is_null() {
                fputs(str, fp);
            } else {
                fprintf(fp,
                        b"UNIV %d???\x00" as *const u8 as *const libc::c_char,
                        eid & 0x1f as libc::c_int);
            }
        }
        fprintf(fp, b"]\x00" as *const u8 as *const libc::c_char);
    }
    #[no_mangle]
    #[c2rust::src_loc = "172:1"]
    pub unsafe extern "C" fn trval2(mut fp: *mut FILE,
                                    mut enc: *mut libc::c_uchar,
                                    mut len: libc::c_int,
                                    mut lev: libc::c_int,
                                    mut rlen: *mut libc::c_int)
     -> libc::c_int {
        let mut l: libc::c_int = 0;
        let mut eid: libc::c_int = 0;
        let mut elen: libc::c_int = 0;
        let mut xlen: libc::c_int = 0;
        let mut r: libc::c_int = 0;
        let mut rlen2: libc::c_int = 0 as libc::c_int;
        let mut rlen_ext: libc::c_int = 0 as libc::c_int;
        r = 0 as libc::c_int;
        *rlen = -(1 as libc::c_int);
        if len < 2 as libc::c_int {
            fprintf(fp,
                    b"missing id and length octets (%d)\n\x00" as *const u8 as
                        *const libc::c_char, len);
            return -(1 as libc::c_int)
        }
        fprintf(fp, b"\n\x00" as *const u8 as *const libc::c_char);
        l = 0 as libc::c_int;
        while l < lev {
            fprintf(fp, b".  \x00" as *const u8 as *const libc::c_char);
            l += 1
        }
        loop  {
            eid = *enc.offset(0 as libc::c_int as isize) as libc::c_int;
            elen = *enc.offset(1 as libc::c_int as isize) as libc::c_int;
            if print_id_and_len != 0 {
                fprintf(fp, b"%02x \x00" as *const u8 as *const libc::c_char,
                        eid);
                fprintf(fp, b"%02x \x00" as *const u8 as *const libc::c_char,
                        elen);
            }
            if elen == 0x80 as libc::c_int {
                fprintf(fp,
                        b"indefinite length encoding not implemented (0x%02x)\n\x00"
                            as *const u8 as *const libc::c_char, elen);
                return -(1 as libc::c_int)
            }
            xlen = 0 as libc::c_int;
            if elen & 0x80 as libc::c_int != 0 {
                xlen = elen & 0x7f as libc::c_int;
                if xlen > len - 2 as libc::c_int {
                    fprintf(fp,
                            b"extended length too long (%d > %d - 2)\n\x00" as
                                *const u8 as *const libc::c_char, xlen, len);
                    return -(1 as libc::c_int)
                }
                elen =
                    decode_len(fp, enc.offset(2 as libc::c_int as isize),
                               xlen)
            }
            if elen > len - 2 as libc::c_int - xlen {
                fprintf(fp,
                        b"length too long (%d > %d - 2 - %d)\n\x00" as
                            *const u8 as *const libc::c_char, elen, len,
                        xlen);
                return -(1 as libc::c_int)
            }
            print_tag_type(fp, eid, lev);
            if !(print_context_shortcut != 0 &&
                     eid & 0xc0 as libc::c_int == 0x80 as libc::c_int &&
                     eid & 0x20 as libc::c_int == 0x20 as libc::c_int &&
                     lev > 0 as libc::c_int) {
                break ;
            }
            rlen_ext += 2 as libc::c_int + xlen;
            enc = enc.offset((2 as libc::c_int + xlen) as isize);
            fprintf(fp, b" \x00" as *const u8 as *const libc::c_char);
        }
        match eid & 0x20 as libc::c_int {
            0 => {
                r =
                    do_prim(fp, eid & 0x1f as libc::c_int,
                            enc.offset(2 as libc::c_int as
                                           isize).offset(xlen as isize), elen,
                            lev + 1 as libc::c_int);
                *rlen = 2 as libc::c_int + xlen + elen + rlen_ext
            }
            32 => {
                if print_constructed_length != 0 {
                    fprintf(fp,
                            b" constr\x00" as *const u8 as
                                *const libc::c_char);
                    fprintf(fp,
                            b" <%d>\x00" as *const u8 as *const libc::c_char,
                            elen);
                }
                r =
                    do_cons(fp,
                            enc.offset(2 as libc::c_int as
                                           isize).offset(xlen as isize), elen,
                            lev + 1 as libc::c_int, &mut rlen2);
                *rlen = 2 as libc::c_int + xlen + rlen2 + rlen_ext
            }
            _ => { }
        }
        return r;
    }
    #[no_mangle]
    #[c2rust::src_loc = "399:1"]
    pub unsafe extern "C" fn do_cons(mut fp: *mut FILE,
                                     mut enc: *mut libc::c_uchar,
                                     mut len: libc::c_int,
                                     mut lev: libc::c_int,
                                     mut rlen: *mut libc::c_int)
     -> libc::c_int {
        let mut n: libc::c_int = 0;
        let mut r: libc::c_int = 0 as libc::c_int;
        let mut rlen2: libc::c_int = 0;
        let mut rlent: libc::c_int = 0;
        let mut save_appl: libc::c_int = 0;
        save_appl = current_appl_type;
        n = 0 as libc::c_int;
        rlent = 0 as libc::c_int;
        while n < len {
            r = trval2(fp, enc.offset(n as isize), len - n, lev, &mut rlen2);
            current_appl_type = save_appl;
            if r != 0 as libc::c_int { return r }
            n += rlen2;
            rlent += rlen2
        }
        if rlent != len {
            fprintf(fp,
                    b"inconsistent constructed lengths (%d != %d)\n\x00" as
                        *const u8 as *const libc::c_char, rlent, len);
            return -(1 as libc::c_int)
        }
        *rlen = rlent;
        return r;
    }
    #[no_mangle]
    #[c2rust::src_loc = "330:1"]
    pub unsafe extern "C" fn do_prim_string(mut fp: *mut FILE,
                                            mut tag: libc::c_int,
                                            mut enc: *mut libc::c_uchar,
                                            mut len: libc::c_int,
                                            mut lev: libc::c_int)
     -> libc::c_int {
        let mut i: libc::c_int = 0;
        if tag < 0x12 as libc::c_int && tag != 0x4 as libc::c_int {
            return 0 as libc::c_int
        }
        i = 0 as libc::c_int;
        while i < len {
            if *(*__ctype_b_loc()).offset(*enc.offset(i as isize) as
                                              libc::c_int as isize) as
                   libc::c_int &
                   _ISprint as libc::c_int as libc::c_ushort as libc::c_int ==
                   0 {
                return 0 as libc::c_int
            }
            i += 1
        }
        fprintf(fp, b" \"%.*s\"\x00" as *const u8 as *const libc::c_char, len,
                enc);
        return 1 as libc::c_int;
    }
    #[no_mangle]
    #[c2rust::src_loc = "300:1"]
    pub unsafe extern "C" fn do_prim_int(mut fp: *mut FILE,
                                         mut tag: libc::c_int,
                                         mut enc: *mut libc::c_uchar,
                                         mut len: libc::c_int,
                                         mut lev: libc::c_int)
     -> libc::c_int {
        let mut i: libc::c_int = 0;
        let mut num: libc::c_long = 0 as libc::c_int as libc::c_long;
        if tag != 0x2 as libc::c_int || len > 4 as libc::c_int {
            return 0 as libc::c_int
        }
        if *enc.offset(0 as libc::c_int as isize) as libc::c_int &
               0x80 as libc::c_int != 0 {
            num = -(1 as libc::c_int) as libc::c_long
        }
        i = 0 as libc::c_int;
        while i < len {
            num = num << 8 as libc::c_int;
            num += *enc.offset(i as isize) as libc::c_long;
            i += 1
        }
        fprintf(fp, b" %ld\x00" as *const u8 as *const libc::c_char, num);
        return 1 as libc::c_int;
    }
    #[no_mangle]
    #[c2rust::src_loc = "273:1"]
    pub unsafe extern "C" fn do_prim_bitstring(mut fp: *mut FILE,
                                               mut tag: libc::c_int,
                                               mut enc: *mut libc::c_uchar,
                                               mut len: libc::c_int,
                                               mut lev: libc::c_int)
     -> libc::c_int {
        let mut i: libc::c_int = 0;
        let mut num: libc::c_long = 0 as libc::c_int as libc::c_long;
        if tag != 0x3 as libc::c_int || len > 5 as libc::c_int {
            return 0 as libc::c_int
        }
        i = 1 as libc::c_int;
        while i < len {
            num = num << 8 as libc::c_int;
            num += *enc.offset(i as isize) as libc::c_long;
            i += 1
        }
        fprintf(fp, b" 0x%lx\x00" as *const u8 as *const libc::c_char, num);
        if *enc.offset(0 as libc::c_int as isize) != 0 {
            fprintf(fp,
                    b" (%d unused bits)\x00" as *const u8 as
                        *const libc::c_char,
                    *enc.offset(0 as libc::c_int as isize) as libc::c_int);
        }
        return 1 as libc::c_int;
    }
    #[no_mangle]
    #[c2rust::src_loc = "352:1"]
    pub unsafe extern "C" fn do_prim(mut fp: *mut FILE, mut tag: libc::c_int,
                                     mut enc: *mut libc::c_uchar,
                                     mut len: libc::c_int,
                                     mut lev: libc::c_int) -> libc::c_int {
        let mut n: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut width: libc::c_int = 0;
        if do_prim_string(fp, tag, enc, len, lev) != 0 {
            return 0 as libc::c_int
        }
        if do_prim_int(fp, tag, enc, len, lev) != 0 {
            return 0 as libc::c_int
        }
        if do_prim_bitstring(fp, tag, enc, len, lev) != 0 {
            return 0 as libc::c_int
        }
        if print_primitive_length != 0 {
            fprintf(fp, b" <%d>\x00" as *const u8 as *const libc::c_char,
                    len);
        }
        width =
            (80 as libc::c_int - lev * 3 as libc::c_int - 8 as libc::c_int) /
                4 as libc::c_int;
        n = 0 as libc::c_int;
        while n < len {
            if n % width == 0 as libc::c_int {
                fprintf(fp, b"\n\x00" as *const u8 as *const libc::c_char);
                i = 0 as libc::c_int;
                while i < lev {
                    fprintf(fp,
                            b"   \x00" as *const u8 as *const libc::c_char);
                    i += 1
                }
            }
            fprintf(fp, b"%02x \x00" as *const u8 as *const libc::c_char,
                    *enc.offset(n as isize) as libc::c_int);
            if n % width == width - 1 as libc::c_int {
                fprintf(fp, b"    \x00" as *const u8 as *const libc::c_char);
                i = n - (width - 1 as libc::c_int);
                while i <= n {
                    if *(*__ctype_b_loc()).offset(*enc.offset(i as isize) as
                                                      libc::c_int as isize) as
                           libc::c_int &
                           _ISprint as libc::c_int as libc::c_ushort as
                               libc::c_int != 0 {
                        fprintf(fp,
                                b"%c\x00" as *const u8 as *const libc::c_char,
                                *enc.offset(i as isize) as libc::c_int);
                    } else {
                        fprintf(fp,
                                b".\x00" as *const u8 as *const libc::c_char);
                    }
                    i += 1
                }
            }
            n += 1
        }
        j = n % width;
        if j != 0 as libc::c_int {
            fprintf(fp, b"    \x00" as *const u8 as *const libc::c_char);
            i = 0 as libc::c_int;
            while i < width - j {
                fprintf(fp, b"   \x00" as *const u8 as *const libc::c_char);
                i += 1
            }
            i = n - j;
            while i < n {
                if *(*__ctype_b_loc()).offset(*enc.offset(i as isize) as
                                                  libc::c_int as isize) as
                       libc::c_int &
                       _ISprint as libc::c_int as libc::c_ushort as
                           libc::c_int != 0 {
                    fprintf(fp, b"%c\x00" as *const u8 as *const libc::c_char,
                            *enc.offset(i as isize) as libc::c_int);
                } else {
                    fprintf(fp, b".\x00" as *const u8 as *const libc::c_char);
                }
                i += 1
            }
        }
        return 0 as libc::c_int;
    }
    #[no_mangle]
    #[c2rust::src_loc = "251:1"]
    pub unsafe extern "C" fn decode_len(mut fp: *mut FILE,
                                        mut enc: *mut libc::c_uchar,
                                        mut len: libc::c_int) -> libc::c_int {
        let mut rlen: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        if print_id_and_len != 0 {
            fprintf(fp, b"%02x \x00" as *const u8 as *const libc::c_char,
                    *enc.offset(0 as libc::c_int as isize) as libc::c_int);
        }
        rlen = *enc.offset(0 as libc::c_int as isize) as libc::c_int;
        i = 1 as libc::c_int;
        while i < len {
            if print_id_and_len != 0 {
                fprintf(fp, b"%02x \x00" as *const u8 as *const libc::c_char,
                        *enc.offset(i as isize) as libc::c_int);
            }
            rlen =
                rlen * 0x100 as libc::c_int +
                    *enc.offset(i as isize) as libc::c_int;
            i += 1
        }
        return rlen;
    }
    #[no_mangle]
    #[c2rust::src_loc = "108:5"]
    pub static mut current_appl_type: libc::c_int = -(1 as libc::c_int);
    #[no_mangle]
    #[c2rust::src_loc = "105:5"]
    pub static mut print_krb5_types: libc::c_int = 0 as libc::c_int;
    #[no_mangle]
    #[c2rust::src_loc = "103:5"]
    pub static mut do_hex: libc::c_int = 0 as libc::c_int;
    #[no_mangle]
    #[c2rust::src_loc = "102:5"]
    pub static mut print_context_shortcut: libc::c_int = 0 as libc::c_int;
    #[no_mangle]
    #[c2rust::src_loc = "101:5"]
    pub static mut print_skip_tagnum: libc::c_int = 1 as libc::c_int;
    #[no_mangle]
    #[c2rust::src_loc = "100:5"]
    pub static mut print_skip_context: libc::c_int = 0 as libc::c_int;
    #[no_mangle]
    #[c2rust::src_loc = "99:5"]
    pub static mut print_primitive_length: libc::c_int = 1 as libc::c_int;
    #[no_mangle]
    #[c2rust::src_loc = "98:5"]
    pub static mut print_constructed_length: libc::c_int = 1 as libc::c_int;
    #[no_mangle]
    #[c2rust::src_loc = "97:5"]
    pub static mut print_id_and_len: libc::c_int = 1 as libc::c_int;
    #[no_mangle]
    #[c2rust::src_loc = "96:5"]
    pub static mut print_types: libc::c_int = 0 as libc::c_int;
    use super::ctype_h::{__ctype_b_loc, _ISdigit, _ISprint};
    use super::FILE_h::FILE;
    use super::stdlib_h::{malloc, realloc, free};
    use super::stdio_h::{fgetc, fprintf, fputs};
    /* ****************************************************************************/
}
#[c2rust::header_src = "/usr/include/stdlib.h:37"]
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
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:37"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "137:14"]
        pub static mut stdin: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "138:14"]
        pub static mut stdout: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "246:14"]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "485:1"]
        pub fn fgetc(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "626:1"]
        pub fn fputs(__s: *const libc::c_char, __stream: *mut FILE)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:37"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
pub use self::types_h::{__off_t, __off64_t};
pub use self::stddef_h::size_t;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::ctype_h::{C2RustUnnamed, _ISalnum, _ISpunct, _IScntrl, _ISblank,
                        _ISgraph, _ISprint, _ISspace, _ISxdigit, _ISdigit,
                        _ISalpha, _ISlower, _ISupper, __ctype_b_loc};
pub use self::trval_c::{typestring_table, convert_nibble, trval, krb5_types,
                        krb5_fields, lookup_typestring, univ_types,
                        print_tag_type, trval2, do_cons, do_prim_string,
                        do_prim_int, do_prim_bitstring, do_prim, decode_len,
                        current_appl_type, print_krb5_types, do_hex,
                        print_context_shortcut, print_skip_tagnum,
                        print_skip_context, print_primitive_length,
                        print_constructed_length, print_id_and_len,
                        print_types};
use self::stdlib_h::{malloc, realloc, free, exit};
use self::stdio_h::{stdin, stdout, stderr, fclose, fopen, fprintf, fgetc,
                    fputs};
use self::string_h::strcmp;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 1992,1993 Trusted Information Systems, Inc.
 *
 * Permission to include this software in the Kerberos V5 distribution
 * was graciously provided by Trusted Information Systems.
 *
 * Trusted Information Systems makes no representation about the
 * suitability of this software for any purpose.  It is provided
 * "as is" without express or implied warranty.
 *
 * Copyright (C) 1994 Massachusetts Institute of Technology
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
/* Split out from "#ifdef STANDALONE" code previously in trval.c, so
   that trval.o could be linked into other tests too without the
   -DSTANDALONE code.  */
#[c2rust::src_loc = "39:1"]
unsafe extern "C" fn usage() {
    fprintf(stderr,
            b"Usage: trval [--types] [--krb5] [--krb5decode] [--hex] [-notypebytes] [file]\n\x00"
                as *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
}
/*
 * Returns true if the option was selected.  Allow "-option" and
 * "--option" syntax, since we used to accept only "-option"
 */
#[c2rust::src_loc = "49:1"]
unsafe extern "C" fn check_option(mut word: *mut libc::c_char,
                                  mut option: *mut libc::c_char)
 -> libc::c_int {
    if *word.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32 {
        return 0 as libc::c_int
    }
    if *word.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32 {
        word = word.offset(1)
    }
    if strcmp(word.offset(1 as libc::c_int as isize), option) != 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "63:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut optflg: libc::c_int = 1 as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut r: libc::c_int = 0 as libc::c_int;
    loop  {
        argc -= 1;
        if !(argc > 0 as libc::c_int) { break ; }
        argv = argv.offset(1);
        if optflg != 0 &&
               **argv.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '-' as i32 {
            if check_option(*argv,
                            b"help\x00" as *const u8 as *const libc::c_char as
                                *mut libc::c_char) != 0 {
                usage();
            } else if check_option(*argv,
                                   b"types\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char) != 0 {
                print_types = 1 as libc::c_int
            } else if check_option(*argv,
                                   b"notypes\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char) != 0 {
                print_types = 0 as libc::c_int
            } else if check_option(*argv,
                                   b"krb5\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char) != 0 {
                print_krb5_types = 1 as libc::c_int
            } else if check_option(*argv,
                                   b"hex\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char) != 0 {
                do_hex = 1 as libc::c_int
            } else if check_option(*argv,
                                   b"notypebytes\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char) != 0 {
                print_id_and_len = 0 as libc::c_int
            } else if check_option(*argv,
                                   b"krb5decode\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char) != 0 {
                print_id_and_len = 0 as libc::c_int;
                print_krb5_types = 1 as libc::c_int;
                print_types = 1 as libc::c_int
            } else {
                fprintf(stderr,
                        b"trval: unknown option: %s\n\x00" as *const u8 as
                            *const libc::c_char, *argv);
                usage();
            }
        } else {
            optflg = 0 as libc::c_int;
            fp = fopen(*argv, b"r\x00" as *const u8 as *const libc::c_char);
            if fp.is_null() {
                fprintf(stderr,
                        b"trval: unable to open %s\n\x00" as *const u8 as
                            *const libc::c_char, *argv);
            } else { r = trval(fp, stdout); fclose(fp); }
        }
    }
    if optflg != 0 { r = trval(stdin, stdout) }
    exit(r);
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
