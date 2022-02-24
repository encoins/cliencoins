use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::ptr::null;
use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use crate::pub_key_converter::{comp_pub_key_from_string, string_from_compr_pub_key};
use crate::UserId;

pub fn loadKeyPair(path : &String) -> Result<Keypair,String>
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
    let mut buf = BufReader::new(File::open(path).unwrap());
    let mut line_nb = 0;
    for lines in buf.lines()
    {
        if line_nb == 0
        {
            pub_key = PublicKey::from_bytes(string_to_user_id(&lines.unwrap()).as_ref()).unwrap();
        }
        else if line_nb == 1
        {
            let sec_key = string_to_secret_key(&lines.unwrap());
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


pub fn exportKeyPair(path : &String, keypair : &Keypair)
{
    let final_path = format!("{}{}", path, ".wallet");
    let mut file = File::create(final_path).unwrap();
    file.write_all(user_id_to_string(&keypair.public.to_bytes()).as_bytes()).unwrap();
    file.write_all(b"\n");
    file.write_all(secrete_key_to_string(&keypair.secret).as_bytes()).unwrap();
    file.flush();
}



 pub fn user_id_to_string(user : &UserId) -> String
{
    string_from_compr_pub_key(user)
}

pub fn string_to_user_id(str : &String) -> UserId
{
    comp_pub_key_from_string(str)
}

fn secrete_key_to_string(sec_key : &SecretKey) -> String
{
    string_from_compr_pub_key(&sec_key.to_bytes())
}

fn string_to_secret_key(str : &String) -> SecretKey
{
    SecretKey::from_bytes(comp_pub_key_from_string(str).as_ref()).unwrap()
}

fn get_extension_from_filename(path: &str) -> Option<&str> {
    Path::new(path)
        .extension()
        .and_then(OsStr::to_str)
}