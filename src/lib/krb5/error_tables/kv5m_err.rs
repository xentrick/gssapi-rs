use ::libc;
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:88"]
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "30:8"]
    pub struct error_table {
        pub msgs: *const *const libc::c_char,
        pub base: libc::c_long,
        pub n_msgs: libc::c_uint,
    }
    extern "C" {
        /*@modifies internalState@*/
        #[no_mangle]
        #[c2rust::src_loc = "57:1"]
        pub fn add_error_table(_: *const error_table) -> errcode_t;
    }
    /* ! defined(__COM_ERR_H) */
}
pub use self::com_err_h::{errcode_t, error_table, add_error_table};
/* Lclint doesn't handle null annotations on arrays
   properly, so we need this typedef in each
   generated .c file.  */
/*@-redef@*/
#[c2rust::src_loc = "19:1"]
pub type ncptr = *const libc::c_char;
/*@null@*/
/*@=redef@*/
#[c2rust::src_loc = "22:20"]
static mut text: [ncptr; 63] =
    [b"Kerberos V5 magic number table\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_principal structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_data structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_keyblock structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_checksum structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_encrypt_block structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_enc_data structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_cryptosystem_entry structure\x00" as
         *const u8 as *const libc::c_char,
     b"Bad magic number for krb5_cs_table_entry structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_checksum_entry structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_authdata structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_transited structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_enc_tkt_part structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_ticket structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_authenticator structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_tkt_authent structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_creds structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_last_req_entry structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_pa_data structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_kdc_req structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_enc_kdc_rep_part structure\x00" as *const u8
         as *const libc::c_char,
     b"Bad magic number for krb5_kdc_rep structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_error structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_ap_req structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_ap_rep structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_ap_rep_enc_part structure\x00" as *const u8
         as *const libc::c_char,
     b"Bad magic number for krb5_response structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_safe structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_priv structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_priv_enc_part structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_cred structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_cred_info structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_cred_enc_part structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_pwd_data structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_address structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_keytab_entry structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_context structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_os_context structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_alt_method structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_etype_info_entry structure\x00" as *const u8
         as *const libc::c_char,
     b"Bad magic number for krb5_db_context structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_auth_context structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_keytab structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_rcache structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_ccache structure\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_preauth_ops\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_sam_challenge\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_sam_challenge_2\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_sam_key\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_enc_sam_response_enc\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_enc_sam_response_enc\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_sam_response\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_sam_response 2\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_predicted_sam_response\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for passwd_phrase_element\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for GSSAPI OID\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for GSSAPI QUEUE\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for fast armored request\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for FAST request\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for FAST response\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic number for krb5_authdata_context\x00" as *const u8 as
         *const libc::c_char,
     b"mit-krb5\x00" as *const u8 as *const libc::c_char, 0 as ncptr];
#[no_mangle]
#[c2rust::src_loc = "90:26"]
pub static mut et_kv5m_error_table: error_table =
    unsafe {
        {
            let mut init =
                error_table{msgs: text.as_ptr(),
                            base: -(1760647424 as libc::c_long),
                            n_msgs: 61 as libc::c_int as libc::c_uint,};
            init
        }
    };
/*
 * et-c-kv5m_err.c:
 * This file is automatically generated; please do not edit it.
 */
#[no_mangle]
#[c2rust::src_loc = "93:1"]
pub unsafe extern "C" fn initialize_kv5m_error_table() 
 /*@modifies internalState@*/
 {
    add_error_table(&et_kv5m_error_table);
}
