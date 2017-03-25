// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use traits::{Extensions, Extras};

enum_number! {
    Target {
        ArrayBuffer = 34962,
        ElementArrayBuffer = 34963,
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Buffer<E: Extensions, X: Extras> {
    /// The uri of the buffer.
    ///
    /// Relative paths are relative to the .gltf file. Instead of referencing an
    /// external file, the uri can also be a data-uri.
    pub uri: String,

    /// The length of the buffer in bytes.
    #[serde(rename = "byteLength")]
    #[serde(default)]
    pub byte_length: usize,

    /// XMLHttpRequest responseType.
    #[serde(rename = "type")]
    pub kind: Option<String>,

    /// The user-defined name of this object.
    ///
    /// This is not necessarily unique, e.g., a buffer and a bufferView could
    /// have the same name, or two buffers could even have the same name.
    pub name: Option<String>,

    /// A dictionary object containing extension-specific data.
    #[serde(default)]
    pub extensions: <E as Extensions>::Buffer,

    /// Application-specific data.
    #[serde(default)]
    pub extras: <X as Extras>::Buffer,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BufferView<E: Extensions, X: Extras> {
    /// The ID of the buffer.
    pub buffer: String,

    /// The offset into the buffer in bytes.
    #[serde(rename = "byteOffset")]
    #[serde(default)]
    pub byte_offset: usize,

    /// The length of the bufferView in bytes.
    #[serde(rename = "byteLength")]
    #[serde(default)]
    pub byte_length: usize,

    /// The target that the WebGL buffer should be bound to.
    ///
    /// When this is not provided, the bufferView contains animation or skin
    /// data.
    pub target: Option<Target>,

    /// The user-defined name of this object.
    ///
    /// This is not necessarily unique, e.g., a bufferView and a buffer could
    /// have the same name, or two bufferViews could even have the same name.
    pub name: Option<String>,

    /// A dictionary object containing extension-specific data.
    #[serde(default)]
    pub extensions: <E as Extensions>::BufferView,

    /// Application-specific data.
    #[serde(default)]
    pub extras: <X as Extras>::BufferView,
}