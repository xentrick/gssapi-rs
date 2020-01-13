use ::libc;
/*
 * Novell key-format scheme:
 *
 * KrbKeySet ::= SEQUENCE {
 * attribute-major-vno       [0] UInt16,
 * attribute-minor-vno       [1] UInt16,
 * kvno                      [2] UInt32,
 * mkvno                     [3] UInt32 OPTIONAL,
 * keys                      [4] SEQUENCE OF KrbKey,
 * ...
 * }
 *
 * KrbKey ::= SEQUENCE {
 * salt      [0] KrbSalt OPTIONAL,
 * key       [1] EncryptionKey,
 * s2kparams [2] OCTET STRING OPTIONAL,
 *  ...
 * }
 *
 * KrbSalt ::= SEQUENCE {
 * type      [0] Int32,
 * salt      [1] OCTET STRING OPTIONAL
 * }
 *
 * EncryptionKey ::= SEQUENCE {
 * keytype   [0] Int32,
 * keyvalue  [1] OCTET STRING
 * }
 *
 */
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* ... copyright ... */
