use crate::helpers::stringify;
use openssl::asn1::{Asn1Time, Asn1Integer};
use openssl::bn::BigNum;
use openssl::ec::{EcKey, EcGroup};
use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::pkey::{Private, PKey};
use openssl::x509::{X509Builder, X509NameBuilder, X509Extension};

// TODO also return pkey
pub(crate) fn create_ca() -> Result<(Vec<u8>, Vec<u8>), String> {
    let mut builder = X509Builder::new().map_err(stringify)?;

    let not_before = Asn1Time::days_from_now(0).map_err(stringify)?;
    builder.set_not_before(&not_before).map_err(stringify)?;

    let not_after = Asn1Time::days_from_now(1000).map_err(stringify)?;
    builder.set_not_after(&not_after).map_err(stringify)?;

    let serial_bn = BigNum::from_u32(1).map_err(stringify)?;
    let serial_number = Asn1Integer::from_bn(&serial_bn).map_err(stringify)?;
    builder.set_serial_number(&serial_number).map_err(stringify)?;

    let mut subject_name_builder = X509NameBuilder::new().map_err(stringify)?;
    subject_name_builder.append_entry_by_text("CN", "DevCA").map_err(stringify)?;
    let subject_name = subject_name_builder.build();
    builder.set_subject_name(&subject_name).map_err(stringify)?;

    let ca_cert_context = builder.x509v3_context(None, None);

    let subject_key_id = X509Extension::new(None, Some(&ca_cert_context), "subjectKeyIdentifier", "hash").map_err(stringify)?;
    let authority_key_id = X509Extension::new(None, Some(&ca_cert_context), "authorityKeyIdentifier", "keyid, issuer").map_err(stringify)?;
    let basic_constraints = X509Extension::new(None, Some(&ca_cert_context), "basicConstraints", "critical, CA:true").map_err(stringify)?;
    let key_usage = X509Extension::new(None, Some(&ca_cert_context), "keyUsage", "keyCertSign, cRLSign").map_err(stringify)?;
    builder.append_extension(subject_key_id).map_err(stringify)?;
    builder.append_extension(authority_key_id).map_err(stringify)?;
    builder.append_extension(basic_constraints).map_err(stringify)?;
    builder.append_extension(key_usage).map_err(stringify)?;

    let ec_group = EcGroup::from_curve_name(Nid::SECP521R1).map_err(stringify)?;
    let ca_key = EcKey::generate(&ec_group).map_err(stringify)?;
    let ca_pkey = PKey::from_ec_key(ca_key).map_err(stringify)?;
    builder.set_pubkey(&ca_pkey).map_err(stringify)?;
    builder.sign(&ca_pkey, MessageDigest::sha512()).map_err(stringify)?;

    let ca_cert = builder.build();

    let ca_cert_pem = ca_cert.to_pem().map_err(stringify)?;
    let ca_key_pem = ca_pkey.private_key_to_pem_pkcs8().map_err(stringify)?;

    Ok((ca_cert_pem, ca_key_pem))
}

// TODO take bytes instead for key. Reason: keep openssl encapsulated.
pub(crate) fn create_cert(name: &str, ca_pkey_pem: &[u8]) -> Result<(Vec<u8>, Vec<u8>), String> {
    let ca_pkey = PKey::private_key_from_pem(ca_pkey_pem).map_err(stringify)?;

    let mut builder = X509Builder::new().map_err(stringify)?;

    let not_before = Asn1Time::days_from_now(0).map_err(stringify)?;
    builder.set_not_before(&not_before).map_err(stringify)?;

    let not_after = Asn1Time::days_from_now(1000).map_err(stringify)?;
    builder.set_not_after(&not_after).map_err(stringify)?;

    let serial_bn = BigNum::from_u32(1).map_err(stringify)?;
    let serial_number = Asn1Integer::from_bn(&serial_bn).map_err(stringify)?;
    builder.set_serial_number(&serial_number).map_err(stringify)?;

    let mut subject_name_builder = X509NameBuilder::new().map_err(stringify)?;
    subject_name_builder.append_entry_by_text("CN", name).map_err(stringify)?;
    let subject_name = subject_name_builder.build();
    builder.set_subject_name(&subject_name).map_err(stringify)?;

    let ca_cert_context = builder.x509v3_context(None, None);

    let subject_key_id = X509Extension::new(None, Some(&ca_cert_context), "subjectKeyIdentifier", "hash").map_err(stringify)?;
    let basic_constraints = X509Extension::new(None, Some(&ca_cert_context), "basicConstraints", "CA:false").map_err(stringify)?;
    let key_usage = X509Extension::new(None, Some(&ca_cert_context), "keyUsage", "digitalSignature, keyEncipherment").map_err(stringify)?;
    builder.append_extension(subject_key_id).map_err(stringify)?;
    builder.append_extension(basic_constraints).map_err(stringify)?;
    builder.append_extension(key_usage).map_err(stringify)?;

    let ec_group = EcGroup::from_curve_name(Nid::SECP521R1).map_err(stringify)?;
    let cert_key = EcKey::generate(&ec_group).map_err(stringify)?;
    let cert_pkey = PKey::from_ec_key(cert_key).map_err(stringify)?;
    builder.set_pubkey(&cert_pkey).map_err(stringify)?;

    builder.sign(&ca_pkey, MessageDigest::sha512()).map_err(stringify)?;

    let cert = builder.build();

    let cert_pem = cert.to_pem().map_err(stringify)?;
    let key_pem = cert_pkey.private_key_to_pem_pkcs8().map_err(stringify)?;

    Ok((cert_pem, key_pem))
}