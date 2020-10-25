use crate::helpers::stringify;
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
        today.year().try_into().map_err(stringify)?,
        today.month().try_into().map_err(stringify)?,
        today.day().try_into().map_err(stringify)?,
    )
    .ok_or("Date construction failed")?;

    let three_years_from_now = today + Duration::days(THREE_YEARS);
    let valid_to = UTCDate::ymd(
        three_years_from_now.year().try_into().map_err(stringify)?,
        three_years_from_now.month().try_into().map_err(stringify)?,
        three_years_from_now.day().try_into().map_err(stringify)?,
    )
    .ok_or("Date construction failed")?;

    Ok((valid_from, valid_to))
}

fn to_writable_bytes(cert: Cert, key: PrivateKey) -> Result<(Vec<u8>, Vec<u8>), String> {
    let cert_string: String = cert.to_pem().map_err(stringify)?.into();
    let key_string: String = key.to_pem().map_err(stringify)?;

    Ok((
        cert_string.as_bytes().to_vec(),
        key_string.as_bytes().to_vec(),
    ))
}

pub(crate) fn create_ca() -> Result<(Vec<u8>, Vec<u8>), String> {
    let private_key = PrivateKey::generate_rsa(2048).map_err(stringify)?;

    let (valid_from, valid_to) = validity_range().map_err(stringify)?;

    let ca = CertificateBuilder::new()
        .validity(valid_from, valid_to)
        .self_signed(DirectoryName::new_common_name("DevCA"), &private_key)
        .ca(true)
        .signature_hash_type(SignatureAlgorithm::RsaPkcs1v15(HashAlgorithm::SHA2_512))
        .key_id_gen_method(KeyIdGenMethod::SPKFullDER(HashAlgorithm::SHA2_384))
        .build()
        .map_err(stringify)?;

    to_writable_bytes(ca, private_key)
}

pub(crate) fn create_cert(
    name: &str,
    ca_cert_pem: &[u8],
    ca_pkey_pem: &[u8],
) -> Result<(Vec<u8>, Vec<u8>), String> {
    let ca_pkey_pem_string = String::from_utf8(ca_pkey_pem.to_vec()).map_err(stringify)?;
    let ca_key = PrivateKey::from_pem_str(&ca_pkey_pem_string).map_err(stringify)?;

    let ca_cert_pem_string = String::from_utf8(ca_cert_pem.to_vec()).map_err(stringify)?;
    let ca_cert = Cert::from_pem_str(&ca_cert_pem_string).map_err(stringify)?;

    let cert_key = PrivateKey::generate_rsa(2048).map_err(stringify)?;

    let (valid_from, valid_to) = validity_range().map_err(stringify)?;

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
        .map_err(stringify)?;

    to_writable_bytes(cert, cert_key)
}
