
//! #   Icon Utils
//! `icon_utils` is a library for serializing and signing transactions for the icon network.
pub mod serializer {
    //! # Icon Transaction Serializer
    //! `serializer` is a module for serializing structs to the transaction format for the icon network.
    use serde::{ser, Serialize};
    use thiserror::Error;
#[derive(Error, Debug)]
pub enum SerializeError {
    #[error("Failed to serialize")]
    FailedToSerialize(String)
}
impl serde::ser::Error for SerializeError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        SerializeError::FailedToSerialize(msg.to_string())
    }
}

pub struct Serializer {
    output: String
}
fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}
/// Converts any struct to transaction format for the icon network.
///
/// # Example
/// 
///  ```
/// println!("{}", serialize_string(&ExampleStruct { a: 1, b: 2 }).unwrap());
/// ```
pub fn serialize_string<T>(value: &T) -> Result<String, SerializeError>
where
    T: Serialize,
{
    let mut serializer = Serializer { output: String::new() };
    value.serialize(&mut serializer)?;
    Ok(rem_first_and_last(&serializer.output).to_string())
}
impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();

    type Error = SerializeError;

    type SerializeSeq = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.output += if v { "true"} else { "false" };
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }
    //TODO: Use itoa
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();
        Ok(())

    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.output += v;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.output += "\0";
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize {
            self.output += "{";
            variant.serialize(&mut *self)?;
            self.output += ".";
            value.serialize(&mut *self)?;
            self.output += "}";
            Ok(())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.output += "[";
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += ".[";
        Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.output += "{";
        Ok(self)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += ".{";
        Ok(self)
    }
}
impl<'a> ser::SerializeSeq for &'a mut Serializer {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = SerializeError;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok,Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ".";
        }
        value.serialize(&mut **self)
    }

    // Close the sequence.
    fn end(self) -> Result<Self::Ok,Self::Error> {
        self.output += "]";
        Ok(())
    }
}
impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok,Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ".";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok,Self::Error> {
        self.output += "]";
        Ok(())
    }
}
impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok,Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ".";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok,Self::Error> {
        self.output += "]";
        Ok(())
    }
}
impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok,Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ".";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok,Self::Error> {
        self.output += "]}";
        Ok(())
    }
}
impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = SerializeError;


    fn serialize_key<T>(&mut self, key: &T) -> Result<Self::Ok,Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('{') {
            self.output += ".";
        }
        key.serialize(&mut **self)
    }


    fn serialize_value<T>(&mut self, value: &T) -> Result<Self::Ok,Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.output += ".";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok,Self::Error> {
        self.output += "}";
        Ok(())
    }
}
impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok,Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('{') {
            self.output += ".";
        }
        key.serialize(&mut **self)?;
        self.output += ".";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok,Self::Error> {
        self.output += "}";
        Ok(())
    }
}
impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok,Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('{') {
            self.output += ".";
        }
        key.serialize(&mut **self)?;
        self.output += ":";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok,Self::Error> {
        self.output += "}}";
        Ok(())
    }
}
}
pub mod wallet {
    //! # Icon Wallet
    //! `wallet` is a module for ICON wallets, and transaction signing.
    use std::path::PathBuf;

use k256::{ecdsa::{SigningKey, recoverable, VerifyingKey, signature::Signer}, EncodedPoint, schnorr::signature::{hazmat::PrehashSigner, Signature}};
use sha3::Sha3_256;
use sha3::Digest;
use eth_keystore::{decrypt_key,encrypt_key,new};
pub struct Wallet {
    pub privkey: SigningKey,
    pub pubkey: VerifyingKey
}
impl Wallet {
    /// Creates a wallet from a hex private key
///
/// # Example
/// 
///  ```
/// let wallet = wallet::Wallet::new("68ee9ca94b71c42ba79375b8677e29a717fb8072bcb17cd5ee288f9b77bc2894");
/// ```
    pub fn new(key: &str) -> Self{
        let signingkey = SigningKey::from_bytes(&hex::decode(key).unwrap()).unwrap();
        let verifiying_key = signingkey.verifying_key();
        Wallet {privkey: signingkey, pubkey: verifiying_key}
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let signingkey = SigningKey::from_bytes(bytes).unwrap();
        let verifiying_key = signingkey.verifying_key();
        Wallet {privkey: signingkey, pubkey: verifiying_key}
    }
    /// Signs a serialized transaction and returns it as a base64 String
///
/// # Example
/// 
///  ```
/// let sig = wallet.sign("example.example");
/// ``` 
    pub fn sign(&self, data: &str) -> String {
        let newdata = Sha3_256::new_with_prefix(data).finalize();
        let signature: recoverable::Signature = self.privkey.sign_prehash(&newdata).unwrap();
        
        return base64::encode(signature.as_ref());
    }
    /// Creates a wallet from a keystore and password
///
/// # Example
/// 
///  ```
/// let wallet = wallet::Wallet::wallet_from_store(PathBuf::from("keystore.json"), "password");
/// ```
    pub fn wallet_from_store(path: PathBuf,password: String) -> Wallet {
        let bytes = decrypt_key(path.as_path(), password).unwrap();
        Wallet::from_bytes(&bytes)
    }
}

}