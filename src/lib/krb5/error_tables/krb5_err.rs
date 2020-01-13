use ::libc;
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:283"]
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
static mut text: [ncptr; 258] =
    [b"No error\x00" as *const u8 as *const libc::c_char,
     b"Client\'s entry in database has expired\x00" as *const u8 as
         *const libc::c_char,
     b"Server\'s entry in database has expired\x00" as *const u8 as
         *const libc::c_char,
     b"Requested protocol version not supported\x00" as *const u8 as
         *const libc::c_char,
     b"Client\'s key is encrypted in an old master key\x00" as *const u8 as
         *const libc::c_char,
     b"Server\'s key is encrypted in an old master key\x00" as *const u8 as
         *const libc::c_char,
     b"Client not found in Kerberos database\x00" as *const u8 as
         *const libc::c_char,
     b"Server not found in Kerberos database\x00" as *const u8 as
         *const libc::c_char,
     b"Principal has multiple entries in Kerberos database\x00" as *const u8
         as *const libc::c_char,
     b"Client or server has a null key\x00" as *const u8 as
         *const libc::c_char,
     b"Ticket is ineligible for postdating\x00" as *const u8 as
         *const libc::c_char,
     b"Requested effective lifetime is negative or too short\x00" as *const u8
         as *const libc::c_char,
     b"KDC policy rejects request\x00" as *const u8 as *const libc::c_char,
     b"KDC can\'t fulfill requested option\x00" as *const u8 as
         *const libc::c_char,
     b"KDC has no support for encryption type\x00" as *const u8 as
         *const libc::c_char,
     b"KDC has no support for checksum type\x00" as *const u8 as
         *const libc::c_char,
     b"KDC has no support for padata type\x00" as *const u8 as
         *const libc::c_char,
     b"KDC has no support for transited type\x00" as *const u8 as
         *const libc::c_char,
     b"Client\'s credentials have been revoked\x00" as *const u8 as
         *const libc::c_char,
     b"Credentials for server have been revoked\x00" as *const u8 as
         *const libc::c_char,
     b"TGT has been revoked\x00" as *const u8 as *const libc::c_char,
     b"Client not yet valid - try again later\x00" as *const u8 as
         *const libc::c_char,
     b"Server not yet valid - try again later\x00" as *const u8 as
         *const libc::c_char,
     b"Password has expired\x00" as *const u8 as *const libc::c_char,
     b"Preauthentication failed\x00" as *const u8 as *const libc::c_char,
     b"Additional pre-authentication required\x00" as *const u8 as
         *const libc::c_char,
     b"Requested server and ticket don\'t match\x00" as *const u8 as
         *const libc::c_char,
     b"Server principal valid for user2user only\x00" as *const u8 as
         *const libc::c_char,
     b"KDC policy rejects transited path\x00" as *const u8 as
         *const libc::c_char,
     b"A service is not available that is required to process the request\x00"
         as *const u8 as *const libc::c_char,
     b"KRB5 error code 30\x00" as *const u8 as *const libc::c_char,
     b"Decrypt integrity check failed\x00" as *const u8 as
         *const libc::c_char,
     b"Ticket expired\x00" as *const u8 as *const libc::c_char,
     b"Ticket not yet valid\x00" as *const u8 as *const libc::c_char,
     b"Request is a replay\x00" as *const u8 as *const libc::c_char,
     b"The ticket isn\'t for us\x00" as *const u8 as *const libc::c_char,
     b"Ticket/authenticator don\'t match\x00" as *const u8 as
         *const libc::c_char,
     b"Clock skew too great\x00" as *const u8 as *const libc::c_char,
     b"Incorrect net address\x00" as *const u8 as *const libc::c_char,
     b"Protocol version mismatch\x00" as *const u8 as *const libc::c_char,
     b"Invalid message type\x00" as *const u8 as *const libc::c_char,
     b"Message stream modified\x00" as *const u8 as *const libc::c_char,
     b"Message out of order\x00" as *const u8 as *const libc::c_char,
     b"Illegal cross-realm ticket\x00" as *const u8 as *const libc::c_char,
     b"Key version is not available\x00" as *const u8 as *const libc::c_char,
     b"Service key not available\x00" as *const u8 as *const libc::c_char,
     b"Mutual authentication failed\x00" as *const u8 as *const libc::c_char,
     b"Incorrect message direction\x00" as *const u8 as *const libc::c_char,
     b"Alternative authentication method required\x00" as *const u8 as
         *const libc::c_char,
     b"Incorrect sequence number in message\x00" as *const u8 as
         *const libc::c_char,
     b"Inappropriate type of checksum in message\x00" as *const u8 as
         *const libc::c_char,
     b"Policy rejects transited path\x00" as *const u8 as *const libc::c_char,
     b"Response too big for UDP, retry with TCP\x00" as *const u8 as
         *const libc::c_char,
     b"KRB5 error code 53\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 54\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 55\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 56\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 57\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 58\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 59\x00" as *const u8 as *const libc::c_char,
     b"Generic error (see e-text)\x00" as *const u8 as *const libc::c_char,
     b"Field is too long for this implementation\x00" as *const u8 as
         *const libc::c_char,
     b"Client not trusted\x00" as *const u8 as *const libc::c_char,
     b"KDC not trusted\x00" as *const u8 as *const libc::c_char,
     b"Invalid signature\x00" as *const u8 as *const libc::c_char,
     b"Key parameters not accepted\x00" as *const u8 as *const libc::c_char,
     b"Certificate mismatch\x00" as *const u8 as *const libc::c_char,
     b"No ticket granting ticket\x00" as *const u8 as *const libc::c_char,
     b"Realm not local to KDC\x00" as *const u8 as *const libc::c_char,
     b"User to user required\x00" as *const u8 as *const libc::c_char,
     b"Can\'t verify certificate\x00" as *const u8 as *const libc::c_char,
     b"Invalid certificate\x00" as *const u8 as *const libc::c_char,
     b"Revoked certificate\x00" as *const u8 as *const libc::c_char,
     b"Revocation status unknown\x00" as *const u8 as *const libc::c_char,
     b"Revocation status unavailable\x00" as *const u8 as *const libc::c_char,
     b"Client name mismatch\x00" as *const u8 as *const libc::c_char,
     b"KDC name mismatch\x00" as *const u8 as *const libc::c_char,
     b"Inconsistent key purpose\x00" as *const u8 as *const libc::c_char,
     b"Digest in certificate not accepted\x00" as *const u8 as
         *const libc::c_char,
     b"Checksum must be included\x00" as *const u8 as *const libc::c_char,
     b"Digest in signed-data not accepted\x00" as *const u8 as
         *const libc::c_char,
     b"Public key encryption not supported\x00" as *const u8 as
         *const libc::c_char,
     b"KRB5 error code 82\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 83\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 84\x00" as *const u8 as *const libc::c_char,
     b"The IAKERB proxy could not find a KDC\x00" as *const u8 as
         *const libc::c_char,
     b"The KDC did not respond to the IAKERB proxy\x00" as *const u8 as
         *const libc::c_char,
     b"KRB5 error code 87\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 88\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 89\x00" as *const u8 as *const libc::c_char,
     b"Preauthentication expired\x00" as *const u8 as *const libc::c_char,
     b"More preauthentication data is required\x00" as *const u8 as
         *const libc::c_char,
     b"KRB5 error code 92\x00" as *const u8 as *const libc::c_char,
     b"An unsupported critical FAST option was requested\x00" as *const u8 as
         *const libc::c_char,
     b"KRB5 error code 94\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 95\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 96\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 97\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 98\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 99\x00" as *const u8 as *const libc::c_char,
     b"No acceptable KDF offered\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 101\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 102\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 103\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 104\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 105\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 106\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 107\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 108\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 109\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 110\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 111\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 112\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 113\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 114\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 115\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 116\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 117\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 118\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 119\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 120\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 121\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 122\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 123\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 124\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 125\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 126\x00" as *const u8 as *const libc::c_char,
     b"KRB5 error code 127\x00" as *const u8 as *const libc::c_char,
     b"$Id$\x00" as *const u8 as *const libc::c_char,
     b"Invalid flag for file lock mode\x00" as *const u8 as
         *const libc::c_char,
     b"Cannot read password\x00" as *const u8 as *const libc::c_char,
     b"Password mismatch\x00" as *const u8 as *const libc::c_char,
     b"Password read interrupted\x00" as *const u8 as *const libc::c_char,
     b"Illegal character in component name\x00" as *const u8 as
         *const libc::c_char,
     b"Malformed representation of principal\x00" as *const u8 as
         *const libc::c_char,
     b"Can\'t open/find Kerberos configuration file\x00" as *const u8 as
         *const libc::c_char,
     b"Improper format of Kerberos configuration file\x00" as *const u8 as
         *const libc::c_char,
     b"Insufficient space to return complete information\x00" as *const u8 as
         *const libc::c_char,
     b"Invalid message type specified for encoding\x00" as *const u8 as
         *const libc::c_char,
     b"Credential cache name malformed\x00" as *const u8 as
         *const libc::c_char,
     b"Unknown credential cache type\x00" as *const u8 as *const libc::c_char,
     b"Matching credential not found\x00" as *const u8 as *const libc::c_char,
     b"End of credential cache reached\x00" as *const u8 as
         *const libc::c_char,
     b"Request did not supply a ticket\x00" as *const u8 as
         *const libc::c_char,
     b"Wrong principal in request\x00" as *const u8 as *const libc::c_char,
     b"Ticket has invalid flag set\x00" as *const u8 as *const libc::c_char,
     b"Requested principal and ticket don\'t match\x00" as *const u8 as
         *const libc::c_char,
     b"KDC reply did not match expectations\x00" as *const u8 as
         *const libc::c_char,
     b"Clock skew too great in KDC reply\x00" as *const u8 as
         *const libc::c_char,
     b"Client/server realm mismatch in initial ticket request\x00" as
         *const u8 as *const libc::c_char,
     b"Program lacks support for encryption type\x00" as *const u8 as
         *const libc::c_char,
     b"Program lacks support for key type\x00" as *const u8 as
         *const libc::c_char,
     b"Requested encryption type not used in message\x00" as *const u8 as
         *const libc::c_char,
     b"Program lacks support for checksum type\x00" as *const u8 as
         *const libc::c_char,
     b"Cannot find KDC for requested realm\x00" as *const u8 as
         *const libc::c_char,
     b"Kerberos service unknown\x00" as *const u8 as *const libc::c_char,
     b"Cannot contact any KDC for requested realm\x00" as *const u8 as
         *const libc::c_char,
     b"No local name found for principal name\x00" as *const u8 as
         *const libc::c_char,
     b"Mutual authentication failed\x00" as *const u8 as *const libc::c_char,
     b"Replay cache type is already registered\x00" as *const u8 as
         *const libc::c_char,
     b"No more memory to allocate (in replay cache code)\x00" as *const u8 as
         *const libc::c_char,
     b"Replay cache type is unknown\x00" as *const u8 as *const libc::c_char,
     b"Generic unknown RC error\x00" as *const u8 as *const libc::c_char,
     b"Message is a replay\x00" as *const u8 as *const libc::c_char,
     b"Replay cache I/O operation failed\x00" as *const u8 as
         *const libc::c_char,
     b"Replay cache type does not support non-volatile storage\x00" as
         *const u8 as *const libc::c_char,
     b"Replay cache name parse/format error\x00" as *const u8 as
         *const libc::c_char,
     b"End-of-file on replay cache I/O\x00" as *const u8 as
         *const libc::c_char,
     b"No more memory to allocate (in replay cache I/O code)\x00" as *const u8
         as *const libc::c_char,
     b"Permission denied in replay cache code\x00" as *const u8 as
         *const libc::c_char,
     b"I/O error in replay cache i/o code\x00" as *const u8 as
         *const libc::c_char,
     b"Generic unknown RC/IO error\x00" as *const u8 as *const libc::c_char,
     b"Insufficient system space to store replay information\x00" as *const u8
         as *const libc::c_char,
     b"Can\'t open/find realm translation file\x00" as *const u8 as
         *const libc::c_char,
     b"Improper format of realm translation file\x00" as *const u8 as
         *const libc::c_char,
     b"Can\'t open/find lname translation database\x00" as *const u8 as
         *const libc::c_char,
     b"No translation available for requested principal\x00" as *const u8 as
         *const libc::c_char,
     b"Improper format of translation database entry\x00" as *const u8 as
         *const libc::c_char,
     b"Cryptosystem internal error\x00" as *const u8 as *const libc::c_char,
     b"Key table name malformed\x00" as *const u8 as *const libc::c_char,
     b"Unknown Key table type\x00" as *const u8 as *const libc::c_char,
     b"Key table entry not found\x00" as *const u8 as *const libc::c_char,
     b"End of key table reached\x00" as *const u8 as *const libc::c_char,
     b"Cannot write to specified key table\x00" as *const u8 as
         *const libc::c_char,
     b"Error writing to key table\x00" as *const u8 as *const libc::c_char,
     b"Cannot find ticket for requested realm\x00" as *const u8 as
         *const libc::c_char,
     b"DES key has bad parity\x00" as *const u8 as *const libc::c_char,
     b"DES key is a weak key\x00" as *const u8 as *const libc::c_char,
     b"Bad encryption type\x00" as *const u8 as *const libc::c_char,
     b"Key size is incompatible with encryption type\x00" as *const u8 as
         *const libc::c_char,
     b"Message size is incompatible with encryption type\x00" as *const u8 as
         *const libc::c_char,
     b"Credentials cache type is already registered.\x00" as *const u8 as
         *const libc::c_char,
     b"Key table type is already registered.\x00" as *const u8 as
         *const libc::c_char,
     b"Credentials cache I/O operation failed\x00" as *const u8 as
         *const libc::c_char,
     b"Credentials cache permissions incorrect\x00" as *const u8 as
         *const libc::c_char,
     b"No credentials cache found\x00" as *const u8 as *const libc::c_char,
     b"Internal credentials cache error\x00" as *const u8 as
         *const libc::c_char,
     b"Error writing to credentials cache\x00" as *const u8 as
         *const libc::c_char,
     b"No more memory to allocate (in credentials cache code)\x00" as
         *const u8 as *const libc::c_char,
     b"Bad format in credentials cache\x00" as *const u8 as
         *const libc::c_char,
     b"No credentials found with supported encryption types\x00" as *const u8
         as *const libc::c_char,
     b"Invalid KDC option combination (library internal error)\x00" as
         *const u8 as *const libc::c_char,
     b"Request missing second ticket\x00" as *const u8 as *const libc::c_char,
     b"No credentials supplied to library routine\x00" as *const u8 as
         *const libc::c_char,
     b"Bad sendauth version was sent\x00" as *const u8 as *const libc::c_char,
     b"Bad application version was sent (via sendauth)\x00" as *const u8 as
         *const libc::c_char,
     b"Bad response (during sendauth exchange)\x00" as *const u8 as
         *const libc::c_char,
     b"Server rejected authentication (during sendauth exchange)\x00" as
         *const u8 as *const libc::c_char,
     b"Unsupported preauthentication type\x00" as *const u8 as
         *const libc::c_char,
     b"Required preauthentication key not supplied\x00" as *const u8 as
         *const libc::c_char,
     b"Generic preauthentication failure\x00" as *const u8 as
         *const libc::c_char,
     b"Unsupported replay cache format version number\x00" as *const u8 as
         *const libc::c_char,
     b"Unsupported credentials cache format version number\x00" as *const u8
         as *const libc::c_char,
     b"Unsupported key table format version number\x00" as *const u8 as
         *const libc::c_char,
     b"Program lacks support for address type\x00" as *const u8 as
         *const libc::c_char,
     b"Message replay detection requires rcache parameter\x00" as *const u8 as
         *const libc::c_char,
     b"Hostname cannot be canonicalized\x00" as *const u8 as
         *const libc::c_char,
     b"Cannot determine realm for host\x00" as *const u8 as
         *const libc::c_char,
     b"Conversion to service principal undefined for name type\x00" as
         *const u8 as *const libc::c_char,
     b"Initial Ticket response appears to be Version 4 error\x00" as *const u8
         as *const libc::c_char,
     b"Cannot resolve network address for KDC in requested realm\x00" as
         *const u8 as *const libc::c_char,
     b"Requesting ticket can\'t get forwardable tickets\x00" as *const u8 as
         *const libc::c_char,
     b"Bad principal name while trying to forward credentials\x00" as
         *const u8 as *const libc::c_char,
     b"Looping detected inside krb5_get_in_tkt\x00" as *const u8 as
         *const libc::c_char,
     b"Configuration file does not specify default realm\x00" as *const u8 as
         *const libc::c_char,
     b"Bad SAM flags in obtain_sam_padata\x00" as *const u8 as
         *const libc::c_char,
     b"Invalid encryption type in SAM challenge\x00" as *const u8 as
         *const libc::c_char,
     b"Missing checksum in SAM challenge\x00" as *const u8 as
         *const libc::c_char,
     b"Bad checksum in SAM challenge\x00" as *const u8 as *const libc::c_char,
     b"Keytab name too long\x00" as *const u8 as *const libc::c_char,
     b"Key version number for principal in key table is incorrect\x00" as
         *const u8 as *const libc::c_char,
     b"This application has expired\x00" as *const u8 as *const libc::c_char,
     b"This Krb5 library has expired\x00" as *const u8 as *const libc::c_char,
     b"New password cannot be zero length\x00" as *const u8 as
         *const libc::c_char,
     b"Password change failed\x00" as *const u8 as *const libc::c_char,
     b"Bad format in keytab\x00" as *const u8 as *const libc::c_char,
     b"Encryption type not permitted\x00" as *const u8 as *const libc::c_char,
     b"No supported encryption types (config file error?)\x00" as *const u8 as
         *const libc::c_char,
     b"Program called an obsolete, deleted function\x00" as *const u8 as
         *const libc::c_char,
     b"unknown getaddrinfo failure\x00" as *const u8 as *const libc::c_char,
     b"no data available for host/domain name\x00" as *const u8 as
         *const libc::c_char,
     b"host/domain name not found\x00" as *const u8 as *const libc::c_char,
     b"service name unknown\x00" as *const u8 as *const libc::c_char,
     b"Cannot determine realm for numeric host address\x00" as *const u8 as
         *const libc::c_char,
     b"Invalid key generation parameters from KDC\x00" as *const u8 as
         *const libc::c_char,
     b"service not available\x00" as *const u8 as *const libc::c_char,
     b"Ccache function not supported: read-only ccache type\x00" as *const u8
         as *const libc::c_char,
     b"Ccache function not supported: not implemented\x00" as *const u8 as
         *const libc::c_char,
     b"Invalid format of Kerberos lifetime or clock skew string\x00" as
         *const u8 as *const libc::c_char,
     b"Supplied data not handled by this plugin\x00" as *const u8 as
         *const libc::c_char,
     b"Plugin does not support the operation\x00" as *const u8 as
         *const libc::c_char,
     b"Invalid UTF-8 string\x00" as *const u8 as *const libc::c_char,
     b"FAST protected pre-authentication required but not supported by KDC\x00"
         as *const u8 as *const libc::c_char,
     b"Auth context must contain local address\x00" as *const u8 as
         *const libc::c_char,
     b"Auth context must contain remote address\x00" as *const u8 as
         *const libc::c_char,
     b"Tracing unsupported\x00" as *const u8 as *const libc::c_char,
     b"mit-krb5\x00" as *const u8 as *const libc::c_char, 0 as ncptr];
#[no_mangle]
#[c2rust::src_loc = "285:26"]
pub static mut et_krb5_error_table: error_table =
    unsafe {
        {
            let mut init =
                error_table{msgs: text.as_ptr(),
                            base: -(1765328384 as libc::c_long),
                            n_msgs: 256 as libc::c_int as libc::c_uint,};
            init
        }
    };
/*
 * et-c-krb5_err.c:
 * This file is automatically generated; please do not edit it.
 */
#[no_mangle]
#[c2rust::src_loc = "288:1"]
pub unsafe extern "C" fn initialize_krb5_error_table() 
 /*@modifies internalState@*/
 {
    add_error_table(&et_krb5_error_table);
}
