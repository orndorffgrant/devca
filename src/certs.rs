use chrono::prelude::*;
use chrono::Duration;
use picky::key::PrivateKey;
use picky::{
    hash::HashAlgorithm,
    signature::SignatureAlgorithm,
    x509::{
        certificate::{Cert, CertificateBuilder},
        date::UTCDate,
        key_id_gen_method::KeyIdGenMethod,
        name::DirectoryName,
    },
};
use std::convert::TryInto;

const THREE_YEARS: i64 = 365 * 3;

fn validity_range() -> Result<(UTCDate, UTCDate), String> {
    let today = Local::today();
    let valid_from = UTCDate::ymd(
        today.year().try_into().unwrap(),
        today.month().try_into().unwrap(),
        today.day().try_into().unwrap(),
    )
    .unwrap();

    let three_years_from_now = today + Duration::days(THREE_YEARS);
    let valid_to = UTCDate::ymd(
        three_years_from_now.year().try_into().unwrap(),
        three_years_from_now.month().try_into().unwrap(),
        three_years_from_now.day().try_into().unwrap(),
    )
    .unwrap();

    Ok((valid_from, valid_to))
}

fn to_writable_bytes(cert: Cert, key: PrivateKey) -> Result<(Vec<u8>, Vec<u8>), String> {
    let cert_string: String = cert.to_pem().unwrap().into();
    let key_string: String = key.to_pem().unwrap();

    Ok((
        cert_string.as_bytes().to_vec(),
        key_string.as_bytes().to_vec(),
    ))
}

pub(crate) fn create_ca() -> Result<(Vec<u8>, Vec<u8>), String> {
    let private_key =
        PrivateKey::generate_rsa(2048).expect("couldn't extract private key from pkcs8");

    let (valid_from, valid_to) = validity_range().unwrap();

    let ca = CertificateBuilder::new()
        .validity(valid_from, valid_to)
        .self_signed(DirectoryName::new_common_name("DevCA"), &private_key)
        .ca(true)
        .signature_hash_type(SignatureAlgorithm::RsaPkcs1v15(HashAlgorithm::SHA2_512))
        .key_id_gen_method(KeyIdGenMethod::SPKFullDER(HashAlgorithm::SHA2_384))
        .build()
        .expect("couldn't generate root ca");

    to_writable_bytes(ca, private_key)
}

pub(crate) fn create_cert(
    name: &str,
    ca_cert_pem: &[u8],
    ca_pkey_pem: &[u8],
) -> Result<(Vec<u8>, Vec<u8>), String> {
    let ca_pkey_pem_string = String::from_utf8(ca_pkey_pem.to_vec()).unwrap();
    let ca_key = PrivateKey::from_pem_str(&ca_pkey_pem_string).unwrap();

    let ca_cert_pem_string = String::from_utf8(ca_cert_pem.to_vec()).unwrap();
    let ca_cert = Cert::from_pem_str(&ca_cert_pem_string).unwrap();

    let cert_key = PrivateKey::generate_rsa(2048).expect("couldn't extract private key from pkcs8");

    let (valid_from, valid_to) = validity_range().unwrap();

    let cert = CertificateBuilder::new()
        .validity(valid_from, valid_to)
        .subject(
            DirectoryName::new_common_name(name),
            cert_key.to_public_key(),
        )
        .signature_hash_type(SignatureAlgorithm::RsaPkcs1v15(HashAlgorithm::SHA2_512))
        .key_id_gen_method(KeyIdGenMethod::SPKFullDER(HashAlgorithm::SHA2_384))
        .issuer_cert(&ca_cert, &ca_key)
        .build()
        .expect("couldn't generate leaf cert");

    to_writable_bytes(cert, cert_key)
}
