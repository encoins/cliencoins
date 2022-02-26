//! Various utilities used by the client
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use crate::pub_key_converter::{comp_pub_key_from_string, string_from_compr_pub_key};
use crate::UserId;

/// Loads a [`Keypair`] from a given file path
/// # Errors
/// Returns an error if no file were found at the given path, if the given file has not the right
/// extension or is corrupted
pub fn load_key_pair(path : &String) -> Result<Keypair,String>
{

    let file_ext = get_extension_from_filename(path);
    match file_ext
    {
        None =>
            {
                return Err(format!("Could not find file {}. (The given file should be of type *.wallet) ", path))
            }
        Some(ext) =>
            {
                match ext
                {
                    "wallet" => {}
                    _ =>
                        {
                            return Err(format!("Given file does not have the right extension! (The given file should be of type *.wallet)"))
                        }
                }
            }
    }
    let mut pub_key = Default::default();
    let file = match File::open(path)
    {
        Ok(f) => { f }
        Err(_) => { return Err(format!("Could not find file {}. (The given file should be of type *.wallet) ", path)) }
    };
    let buf = BufReader::new(file);
    let mut line_nb = 0;
    for lines in buf.lines()
    {
        if line_nb == 0
        {
            match string_to_user_id(&lines.unwrap())
            {
                Ok(uid) =>
                    {
                        pub_key = match PublicKey::from_bytes(uid.as_ref())
                        {
                            Ok( pk ) => { pk }
                            Err(e) => { return Err(e.to_string()) }
                        };
                    }
                Err(err) => { return Err(err) }
            }
        }
        else if line_nb == 1
        {
            let sec_key =  match string_to_secret_key(&lines.unwrap())
            {
                Ok(sk) => {  sk }
                Err(err) => {  return Err(err) }
            };
            return Ok(Keypair{ secret: sec_key, public: pub_key })

        }

        line_nb+=1;

        if line_nb >1
        {
            break;
        }
    }

    Err(String::from("The given file does not contain both a public key and a private key"))



}

/// Writes a [`Keypair`] into a `.wallet` file
pub fn export_key_pair(path : &String, keypair : &Keypair)
{
    let final_path = format!("{}{}", path, ".wallet");
    let mut file = File::create(final_path).unwrap();
    file.write_all(user_id_to_string(&keypair.public.to_bytes()).as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();
    file.write_all(secrete_key_to_string(&keypair.secret).as_bytes()).unwrap();
    file.flush().unwrap();
}


/// Converts a [`UserId`] to a [`String`]
pub fn user_id_to_string(user : &UserId) -> String
{
    string_from_compr_pub_key(user)
}

/// Converts a [`String`] to a [`UserId`]
pub fn string_to_user_id(str : &String) -> Result<UserId,String>
{
    comp_pub_key_from_string(str)
}

fn secrete_key_to_string(sec_key : &SecretKey) -> String
{
    string_from_compr_pub_key(&sec_key.to_bytes())
}

fn string_to_secret_key(str : &String) -> Result<SecretKey, String>
{
    let key = match comp_pub_key_from_string(str)
    {
        Ok(k) => {k}
        Err(s) => { return Err(s) }
    };

    match SecretKey::from_bytes( key.as_ref() )
    {
        Ok(sec_key) => { Ok(sec_key) }
        Err(e) => { Err(e.to_string()) }
    }
}

fn get_extension_from_filename(path: &str) -> Option<&str> {
    Path::new(path)
        .extension()
        .and_then(OsStr::to_str)
}