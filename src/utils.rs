use std::ptr::null;
use ed25519_dalek::Keypair;
use serde::de::Unexpected::Str;

pub fn loadKeyPair(path : &String) -> Result<Keypair,String>
{
    match csv::ReaderBuilder::new().has_headers(false).from_path(path)
    {
        Ok(mut reader) =>
            {
                for keypair in reader.deserialize()
                {
                    match keypair
                    {
                        Ok(kp) =>
                            {
                                return Ok(kp)
                            },
                        Err(err) => { return Err(err.to_string()); }
                    }
                }
            }
        Err(err) => { return  Err(err.to_string()); }
    }
    return Err(String::from("The given file is empty!"))
}