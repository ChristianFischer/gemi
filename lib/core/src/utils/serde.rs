/*
 * Copyright (C) 2022-2024 by Christian Fischer
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

#[cfg(feature = "serde")]
pub trait SerdeSupport: serde::Serialize + for<'de> serde::Deserialize<'de> {}

#[cfg(feature = "serde")]
impl<T: serde::Serialize + for<'de> serde::Deserialize<'de>> SerdeSupport for T {}

#[cfg(not(feature = "serde"))]
pub trait SerdeSupport {}

#[cfg(not(feature = "serde"))]
impl<T> SerdeSupport for T {}


#[cfg(feature = "serde")]
pub mod serialize {
    use base64::Engine;
    use serde::de::Error;
    use serde::Deserialize;
    use std::mem::size_of;
    use std::ptr::slice_from_raw_parts;


    /// Serialize a slice of data as a stream of bytes.
    /// The data will be compressed and for human-readable export formats like JSON
    /// written as Base64 encoded string to keep data size low.
    pub fn serialize_bytes<T, S>(serializer: S, data: &[T]) -> Result<S::Ok, S::Error>
    where
            S: serde::Serializer,
            T: Sized,
    {
        // convert the data into a byte slice
        let bytes = unsafe {
            let data_ptr = data.as_ptr() as *const u8;
            let data_len = data.len() * size_of::<T>();

            &*slice_from_raw_parts(data_ptr, data_len)
        };

        // compress using zstd
        let compressed = zstd::encode_all(bytes, 1).unwrap_or_else(|_| Vec::from(bytes));

        if serializer.is_human_readable() {
            // for human-readable formats: serialize as base64 string
            let encoded = base64::engine::general_purpose::URL_SAFE.encode(compressed);

            serializer.serialize_str(&encoded)
        }
        else {
            // for binary formats: serialize as a byte array
            serializer.serialize_bytes(&compressed)
        }
    }


    /// Deserializes a stream of bytes into a Vector of type T.
    /// The data is compressed and serialized as Base64 string for human-readable formats.
    pub fn deserialize_bytes<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
            D: serde::Deserializer<'de>,
            T: Sized + Clone
    {
        // deserialize either from base64 or plain byte array
        let v = if deserializer.is_human_readable() {
            let encoded = String::deserialize(deserializer)?;

            base64::engine::general_purpose::URL_SAFE.decode(encoded)
                    .map_err(|e| Error::custom(format!("Failed to decode base64: {e}")))?
        }
        else {
            Vec::<u8>::deserialize(deserializer)?
        };

        // decompress zstd compressed data
        let uncompressed = zstd::decode_all(v.as_slice())
                .map_err(|e| Error::custom(format!("Failed to decompress: {e}")))?;

        // convert into vector of type `T`.
        let typed = {
            let data_ptr = uncompressed.as_ptr() as *const T;
            let data_len = uncompressed.len() / size_of::<T>();

            let slice = unsafe {
                &*slice_from_raw_parts(data_ptr, data_len)
            };

            slice.to_vec()
        };

        Ok(typed)
    }

}
