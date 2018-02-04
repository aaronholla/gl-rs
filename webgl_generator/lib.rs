// Copyright 2015 Brendan Zabarauskas and the gl-rs developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A WebGL bindings generator. It defines a function named `generate_bindings` which can be
//! used to generate all constants and functions of a given OpenGL version.
//!
//! # Example
//!
//! In `build.rs`:
//!
//! ```no_run
//! extern crate webgl_generator;
//!
//! use webgl_generator::{Registry, Api, Profile, Fallbacks, StdwebGenerator};
//! use std::env;
//! use std::fs::File;
//! use std::path::Path;
//!
//! fn main() {
//!     let dest = env::var("OUT_DIR").unwrap();
//!     let mut file = File::create(&Path::new(&dest).join("gl_bindings.rs")).unwrap();
//!
//!     Registry::new(Api::Gl, (4, 5), Profile::Core, Fallbacks::All, [])
//!         .write_bindings(StdwebGenerator, &mut file)
//!         .unwrap();
//! }
//! ```
//!
//! In your project:
//!
//! ```ignore
//! include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
//! ```
//!
#[macro_use]
extern crate serde_derive;

mod webgl_generators;
mod webgl_registry;

pub use webgl_generators::Generator;
pub use webgl_generators::stdweb_gen::StdwebGenerator;

pub use webgl_registry::*;
