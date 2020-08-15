use crate::helpers::stringify;
use crate::certs::{create_ca, create_cert};
use crate::dirs::{ca_dir, cert_dir};

use serde::{Serialize, Deserialize};
use std::fs::{read, write};

fn get_ca_key() -> Result<Vec<u8>, String> {
    let mut key_path = ca_dir()?;
    let mut cert_path = key_path.clone();
    key_path.push("key.pem");
    cert_path.push("cert.pem");
    let key_pem = {
        if key_path.exists() {
            read(key_path).map_err(stringify)?
        } else {
            let (ca_cert_pem, ca_key_pem) = create_ca()?;
            write(&key_path, &ca_key_pem).map_err(stringify)?;
            write(&cert_path, &ca_cert_pem).map_err(stringify)?;
            println!("Created CA private key: {}", key_path.to_str().unwrap());
            println!("Created CA certificate: {}", cert_path.to_str().unwrap());
            ca_key_pem
        }
    };
    Ok(key_pem)
}

#[derive(Serialize, Deserialize)]
struct CAState {
    serial_number: u32
}

fn get_ca_state() -> Result<CAState, String> {
    let mut ca_state_path = ca_dir()?;
    ca_state_path.push("state.json");
    let mut curr_ca_state = {
        if ca_state_path.exists() {
            let raw_ca_state = read(&ca_state_path).map_err(stringify)?;
            serde_json::from_slice(&raw_ca_state).map_err(stringify)?
        } else {
            CAState{ serial_number: 0 }
        }
    };

    curr_ca_state.serial_number += 1;

    let serialized_ca_state = serde_json::to_vec(&curr_ca_state).map_err(stringify)?;

    write(&ca_state_path, serialized_ca_state).map_err(stringify)?;

    Ok(curr_ca_state)
}

pub(crate) fn new_cert(name: &str) -> Result<(), String> {
    let ca_key_pem = get_ca_key()?;
    let ca_state = get_ca_state()?;

    let mut key_path = cert_dir(name)?;
    let mut cert_path = key_path.clone();
    key_path.push("key.pem");
    cert_path.push("cert.pem");

    // TODO check if cert name exists
    // TODO prompt if cert name already exists
    let (cert_pem, key_pem) = create_cert(name, &ca_key_pem, ca_state.serial_number)?;
    write(&key_path, &key_pem).map_err(stringify)?;
    write(&cert_path, &cert_pem).map_err(stringify)?;

    println!("Created private key for \"{}\": {}", name, key_path.to_str().unwrap());
    println!("Created certificate for \"{}\": {}", name, cert_path.to_str().unwrap());
    Ok(())
}
