use ::libc;
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/ss/ss.h:2"]
pub mod ss_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "22:27"]
    pub struct _ss_request_entry {
        pub command_names: *const *const libc::c_char,
        pub function: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _:
                                                      *const *const libc::c_char,
                                                  _: libc::c_int,
                                                  _: *mut libc::c_void)
                                 -> ()>,
        pub info_string: *const libc::c_char,
        pub flags: libc::c_int,
    }
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1987, 1988 by MIT Student Information Processing Board
 *
 * For copyright information, see mit-sipb-copyright.h.
 */
    #[c2rust::src_loc = "22:1"]
    pub type ss_request_entry = _ss_request_entry;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "29:27"]
    pub struct _ss_request_table {
        pub version: libc::c_int,
        pub requests: *const ss_request_entry,
    }
    #[c2rust::src_loc = "29:1"]
    pub type ss_request_table = _ss_request_table;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "49:1"]
        pub fn ss_list_requests(_: libc::c_int, _: *const *const libc::c_char,
                                _: libc::c_int, _: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "50:1"]
        pub fn ss_quit(_: libc::c_int, _: *const *const libc::c_char,
                       _: libc::c_int, _: *mut libc::c_void);
    }
    /* whatever */
    /* foo */
    /* NULL */
    /* 0 */
    /* _ss_h */
}
pub use self::ss_h::{_ss_request_entry, ss_request_entry, _ss_request_table,
                     ss_request_table, ss_list_requests, ss_quit};
extern "C" {
    #[no_mangle]
    #[c2rust::src_loc = "14:1"]
    pub fn kadmin_addprinc(_: libc::c_int, _: *const *const libc::c_char,
                           _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "20:1"]
    pub fn kadmin_delprinc(_: libc::c_int, _: *const *const libc::c_char,
                           _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "26:1"]
    pub fn kadmin_modprinc(_: libc::c_int, _: *const *const libc::c_char,
                           _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "32:1"]
    pub fn kadmin_renameprinc(_: libc::c_int, _: *const *const libc::c_char,
                              _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "38:1"]
    pub fn kadmin_cpw(_: libc::c_int, _: *const *const libc::c_char,
                      _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "44:1"]
    pub fn kadmin_getprinc(_: libc::c_int, _: *const *const libc::c_char,
                           _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "52:1"]
    pub fn kadmin_getprincs(_: libc::c_int, _: *const *const libc::c_char,
                            _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "58:1"]
    pub fn kadmin_addpol(_: libc::c_int, _: *const *const libc::c_char,
                         _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "64:1"]
    pub fn kadmin_modpol(_: libc::c_int, _: *const *const libc::c_char,
                         _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "70:1"]
    pub fn kadmin_delpol(_: libc::c_int, _: *const *const libc::c_char,
                         _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "76:1"]
    pub fn kadmin_getpol(_: libc::c_int, _: *const *const libc::c_char,
                         _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "84:1"]
    pub fn kadmin_getpols(_: libc::c_int, _: *const *const libc::c_char,
                          _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "90:1"]
    pub fn kadmin_getprivs(_: libc::c_int, _: *const *const libc::c_char,
                           _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "96:1"]
    pub fn kadmin_keytab_add(_: libc::c_int, _: *const *const libc::c_char,
                             _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "102:1"]
    pub fn kadmin_keytab_remove(_: libc::c_int, _: *const *const libc::c_char,
                                _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "107:1"]
    pub fn kadmin_lock(_: libc::c_int, _: *const *const libc::c_char,
                       _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "112:1"]
    pub fn kadmin_unlock(_: libc::c_int, _: *const *const libc::c_char,
                         _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "117:1"]
    pub fn kadmin_purgekeys(_: libc::c_int, _: *const *const libc::c_char,
                            _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "123:1"]
    pub fn kadmin_getstrings(_: libc::c_int, _: *const *const libc::c_char,
                             _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "129:1"]
    pub fn kadmin_setstring(_: libc::c_int, _: *const *const libc::c_char,
                            _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "135:1"]
    pub fn kadmin_delstring(_: libc::c_int, _: *const *const libc::c_char,
                            _: libc::c_int, _: *mut libc::c_void);
}
/* kadmin_ct.c - automatically generated from kadmin_ct.ct */
#[c2rust::src_loc = "8:27"]
static mut ssu00001: [*const libc::c_char; 4] =
    [b"add_principal\x00" as *const u8 as *const libc::c_char,
     b"addprinc\x00" as *const u8 as *const libc::c_char,
     b"ank\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "15:27"]
static mut ssu00002: [*const libc::c_char; 3] =
    [b"delete_principal\x00" as *const u8 as *const libc::c_char,
     b"delprinc\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "21:27"]
static mut ssu00003: [*const libc::c_char; 3] =
    [b"modify_principal\x00" as *const u8 as *const libc::c_char,
     b"modprinc\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "27:27"]
static mut ssu00004: [*const libc::c_char; 3] =
    [b"rename_principal\x00" as *const u8 as *const libc::c_char,
     b"renprinc\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "33:27"]
static mut ssu00005: [*const libc::c_char; 3] =
    [b"change_password\x00" as *const u8 as *const libc::c_char,
     b"cpw\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "39:27"]
static mut ssu00006: [*const libc::c_char; 3] =
    [b"get_principal\x00" as *const u8 as *const libc::c_char,
     b"getprinc\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "45:27"]
static mut ssu00007: [*const libc::c_char; 5] =
    [b"list_principals\x00" as *const u8 as *const libc::c_char,
     b"listprincs\x00" as *const u8 as *const libc::c_char,
     b"get_principals\x00" as *const u8 as *const libc::c_char,
     b"getprincs\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "53:27"]
static mut ssu00008: [*const libc::c_char; 3] =
    [b"add_policy\x00" as *const u8 as *const libc::c_char,
     b"addpol\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "59:27"]
static mut ssu00009: [*const libc::c_char; 3] =
    [b"modify_policy\x00" as *const u8 as *const libc::c_char,
     b"modpol\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "65:27"]
static mut ssu00010: [*const libc::c_char; 3] =
    [b"delete_policy\x00" as *const u8 as *const libc::c_char,
     b"delpol\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "71:27"]
static mut ssu00011: [*const libc::c_char; 3] =
    [b"get_policy\x00" as *const u8 as *const libc::c_char,
     b"getpol\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "77:27"]
static mut ssu00012: [*const libc::c_char; 5] =
    [b"list_policies\x00" as *const u8 as *const libc::c_char,
     b"listpols\x00" as *const u8 as *const libc::c_char,
     b"get_policies\x00" as *const u8 as *const libc::c_char,
     b"getpols\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "85:27"]
static mut ssu00013: [*const libc::c_char; 3] =
    [b"get_privs\x00" as *const u8 as *const libc::c_char,
     b"getprivs\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "91:27"]
static mut ssu00014: [*const libc::c_char; 3] =
    [b"ktadd\x00" as *const u8 as *const libc::c_char,
     b"xst\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "97:27"]
static mut ssu00015: [*const libc::c_char; 3] =
    [b"ktremove\x00" as *const u8 as *const libc::c_char,
     b"ktrem\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "103:27"]
static mut ssu00016: [*const libc::c_char; 2] =
    [b"lock\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "108:27"]
static mut ssu00017: [*const libc::c_char; 2] =
    [b"unlock\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "113:27"]
static mut ssu00018: [*const libc::c_char; 2] =
    [b"purgekeys\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "118:27"]
static mut ssu00019: [*const libc::c_char; 3] =
    [b"get_strings\x00" as *const u8 as *const libc::c_char,
     b"getstrs\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "124:27"]
static mut ssu00020: [*const libc::c_char; 3] =
    [b"set_string\x00" as *const u8 as *const libc::c_char,
     b"setstr\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "130:27"]
static mut ssu00021: [*const libc::c_char; 3] =
    [b"del_string\x00" as *const u8 as *const libc::c_char,
     b"delstr\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "136:27"]
static mut ssu00022: [*const libc::c_char; 4] =
    [b"list_requests\x00" as *const u8 as *const libc::c_char,
     b"lr\x00" as *const u8 as *const libc::c_char,
     b"?\x00" as *const u8 as *const libc::c_char, 0 as *const libc::c_char];
#[c2rust::src_loc = "143:27"]
static mut ssu00023: [*const libc::c_char; 4] =
    [b"quit\x00" as *const u8 as *const libc::c_char,
     b"exit\x00" as *const u8 as *const libc::c_char,
     b"q\x00" as *const u8 as *const libc::c_char, 0 as *const libc::c_char];
#[c2rust::src_loc = "150:25"]
static mut ssu00024: [ss_request_entry; 24] =
    unsafe {
        [{
             let mut init =
                 _ss_request_entry{command_names: ssu00001.as_ptr(),
                                   function:
                                       Some(kadmin_addprinc as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Add principal\x00" as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00002.as_ptr(),
                                   function:
                                       Some(kadmin_delprinc as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Delete principal\x00" as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00003.as_ptr(),
                                   function:
                                       Some(kadmin_modprinc as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Modify principal\x00" as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00004.as_ptr(),
                                   function:
                                       Some(kadmin_renameprinc as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Rename principal\x00" as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00005.as_ptr(),
                                   function:
                                       Some(kadmin_cpw as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Change password\x00" as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00006.as_ptr(),
                                   function:
                                       Some(kadmin_getprinc as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Get principal\x00" as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00007.as_ptr(),
                                   function:
                                       Some(kadmin_getprincs as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"List principals\x00" as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00008.as_ptr(),
                                   function:
                                       Some(kadmin_addpol as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Add policy\x00" as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00009.as_ptr(),
                                   function:
                                       Some(kadmin_modpol as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Modify policy\x00" as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00010.as_ptr(),
                                   function:
                                       Some(kadmin_delpol as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Delete policy\x00" as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00011.as_ptr(),
                                   function:
                                       Some(kadmin_getpol as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Get policy\x00" as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00012.as_ptr(),
                                   function:
                                       Some(kadmin_getpols as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"List policies\x00" as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00013.as_ptr(),
                                   function:
                                       Some(kadmin_getprivs as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Get privileges\x00" as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00014.as_ptr(),
                                   function:
                                       Some(kadmin_keytab_add as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Add entry(s) to a keytab\x00" as
                                           *const u8 as *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00015.as_ptr(),
                                   function:
                                       Some(kadmin_keytab_remove as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Remove entry(s) from a keytab\x00" as
                                           *const u8 as *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00016.as_ptr(),
                                   function:
                                       Some(kadmin_lock as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Lock database exclusively (use with extreme caution!)\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00017.as_ptr(),
                                   function:
                                       Some(kadmin_unlock as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Release exclusive database lock\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00018.as_ptr(),
                                   function:
                                       Some(kadmin_purgekeys as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Purge previously retained old keys from a principal\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00019.as_ptr(),
                                   function:
                                       Some(kadmin_getstrings as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Show string attributes on a principal\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00020.as_ptr(),
                                   function:
                                       Some(kadmin_setstring as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Set a string attribute on a principal\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00021.as_ptr(),
                                   function:
                                       Some(kadmin_delstring as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Delete a string attribute on a principal\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00022.as_ptr(),
                                   function:
                                       Some(ss_list_requests as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"List available requests.\x00" as
                                           *const u8 as *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00023.as_ptr(),
                                   function:
                                       Some(ss_quit as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Exit program.\x00" as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names:
                                       0 as *const *const libc::c_char,
                                   function: None,
                                   info_string: 0 as *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         }]
    };
#[no_mangle]
#[c2rust::src_loc = "246:18"]
pub static mut kadmin_cmds: ss_request_table =
    unsafe {
        {
            let mut init =
                _ss_request_table{version: 2 as libc::c_int,
                                  requests: ssu00024.as_ptr(),};
            init
        }
    };
