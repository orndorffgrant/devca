use crate::certs::{create_ca, create_cert};
use crate::dirs::{ca_dir, cert_dir, certs_dir};
use crate::helpers::stringify;

use serde::{Deserialize, Serialize};
use std::fs::{read, remove_dir_all, write};
use std::io;
use std::io::prelude::*;

fn get_ca(force_create_new: bool) -> Result<(Vec<u8>, Vec<u8>), String> {
    let mut key_path = ca_dir()?;
    let mut cert_path = key_path.clone();
    key_path.push("key.pem");
    cert_path.push("cert.pem");
    let pems = {
        if !key_path.exists() || force_create_new {
            let (ca_cert_pem, ca_key_pem) = create_ca()?;
            write(&key_path, &ca_key_pem).map_err(stringify)?;
            write(&cert_path, &ca_cert_pem).map_err(stringify)?;
            println!("Created CA private key: {}", key_path.to_str().unwrap());
            println!("Created CA certificate: {}", cert_path.to_str().unwrap());
            (ca_cert_pem, ca_key_pem)
        } else {
            let ca_key_pem = read(key_path).map_err(stringify)?;
            let ca_cert_pem = read(cert_path).map_err(stringify)?;
            (ca_cert_pem, ca_key_pem)
        }
    };
    Ok(pems)
}

#[derive(Serialize, Deserialize)]
struct CAState {
    serial_number: u32,
}

fn get_ca_state() -> Result<CAState, String> {
    let mut ca_state_path = ca_dir()?;
    ca_state_path.push("state.json");
    let mut curr_ca_state = {
        if ca_state_path.exists() {
            let raw_ca_state = read(&ca_state_path).map_err(stringify)?;
            serde_json::from_slice(&raw_ca_state).map_err(stringify)?
        } else {
            CAState { serial_number: 0 }
        }
    };

    curr_ca_state.serial_number += 1;

    let serialized_ca_state = serde_json::to_vec(&curr_ca_state).map_err(stringify)?;

    write(&ca_state_path, serialized_ca_state).map_err(stringify)?;

    Ok(curr_ca_state)
}

fn get_cert_list() -> Result<Vec<String>, String> {
    let mut cert_list = vec![];
    let dir = certs_dir()?;
    let dir_iterator = dir.read_dir().map_err(stringify)?;
    for cert_dir in dir_iterator {
        let cert_dir = cert_dir.map_err(stringify)?;
        let cert_dir_path = cert_dir.path();

        let mut cert_dir_key_path = cert_dir_path.clone();
        let mut cert_dir_cert_path = cert_dir_path.clone();
        cert_dir_key_path.push("key.pem");
        cert_dir_cert_path.push("cert.pem");

        if cert_dir_key_path.exists() && cert_dir_cert_path.exists() {
            let cert_name_os = cert_dir_path
                .file_name()
                .ok_or("error getting directory name")?;
            let cert_name = cert_name_os
                .to_str()
                .ok_or("error converting directory name")?
                .to_owned();
            cert_list.push(cert_name);
        }
    }
    Ok(cert_list)
}

pub(crate) fn new_cert(name: &str, force_overwrite: bool) -> Result<(), String> {
    let (ca_cert_pem, ca_key_pem) = get_ca(false)?;
    let ca_state = get_ca_state()?;

    let mut key_path = cert_dir(name)?;
    let mut cert_path = key_path.clone();
    key_path.push("key.pem");
    cert_path.push("cert.pem");

    if key_path.exists() && !force_overwrite {
        print!("{}", format!("**** A certificate for \"{}\" already exists. Would you like to overwrite it? y/N: ", name));
        io::stdout().flush().map_err(stringify)?;
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).map_err(stringify)?;
        if answer.to_ascii_lowercase().chars().nth(0) != Some('y') {
            println!("Aborting. Nothing was created.");
            return Ok(());
        }
    }

    let (cert_pem, key_pem) = create_cert(name, &ca_cert_pem, &ca_key_pem, ca_state.serial_number)?;
    write(&key_path, &key_pem).map_err(stringify)?;
    write(&cert_path, &cert_pem).map_err(stringify)?;

    println!(
        "Created private key for \"{}\": {}",
        name,
        key_path.to_str().unwrap()
    );
    println!(
        "Created certificate for \"{}\": {}",
        name,
        cert_path.to_str().unwrap()
    );
    Ok(())
}

pub(crate) fn ls() -> Result<(), String> {
    let cert_list = get_cert_list()?;
    for cert_name in cert_list {
        println!("{}", cert_name);
    }
    Ok(())
}

pub(crate) fn path_to(name: &str) -> Result<(), String> {
    let mut dir = certs_dir()?;
    dir.push(name);
    if dir.exists() {
        println!("{}", dir.to_str().ok_or("error converting directory name")?);
        Ok(())
    } else {
        Err(format!(
            "Certificate with that name has not been created. Create it with \"devca new {}\"",
            name
        ))
    }
}

pub(crate) fn delete(name: &str) -> Result<(), String> {
    let mut dir = certs_dir()?;
    dir.push(name);
    if dir.exists() {
        let dir_str = dir
            .to_str()
            .ok_or("error converting directory name")?
            .to_owned();
        println!(
            "{}",
            format!("**** This will delete all contents of {}", dir_str)
        );
        print!("**** Would you like to proceed? Y/n: ");
        io::stdout().flush().map_err(stringify)?;
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).map_err(stringify)?;
        if answer.to_ascii_lowercase().chars().nth(0) == Some('n') {
            println!("Aborting. Nothing was deleted.");
            return Ok(());
        }
        remove_dir_all(dir).map_err(stringify)?;
        println!("{}", format!("Deleted \"{}\": {}", name, dir_str));
        Ok(())
    } else {
        Err("Certificate with that name does not exist. Nothing was deleted.".to_string())
    }
}

pub(crate) fn regen() -> Result<(), String> {
    println!("**** This will regenerate the CA and all certificates.");
    print!("**** Would you like to proceed? Y/n: ");
    io::stdout().flush().map_err(stringify)?;
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).map_err(stringify)?;
    if answer.to_ascii_lowercase().chars().nth(0) == Some('n') {
        println!("Aborting. Nothing was deleted or created.");
        return Ok(());
    }

    let _ca_pems = get_ca(true)?;
    let cert_list = get_cert_list()?;
    for cert_name in cert_list {
        new_cert(&cert_name, true)?;
    }

    Ok(())
}
