// Copyright 2019 Centrality Investments Limited
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

use std::collections::BTreeMap;

use super::{IdentKind, Metadata, TypeDef};

pub struct Registry {
	pub types: BTreeMap<IdentKind, TypeDef>,
}

impl Registry {
	pub fn new() -> Registry {
		Registry { types: BTreeMap::new() }
	}

	pub fn register<F: Fn(&mut Registry) -> TypeDef>(&mut self, type_ident: IdentKind, f: F) {
		// simple primitives would not be actually registered, as an optimization to reduce storage
		// usage, they're assumed to be decodable by any valid decoder impl.
		let should_ignore = match type_ident {
			IdentKind::Custom(_) => false,
			IdentKind::Array(_) | IdentKind::Slice(_) | IdentKind::Tuple(_) => {
				// build-ins are also ignored but their sub-types are registered
				f(self);
				true
			}
			_ => true,
		};
		if should_ignore {
			return;
		}
		if self.exists(&type_ident) {
			return;
		}

		// insert `TypeDef::Primitive` as placeholder, instead of calling `f`, to avoid circular calling
		self.types.insert(type_ident.clone(), TypeDef::None);

		let type_def = f(self);
		self.types.insert(type_ident, type_def);
	}

	pub fn register_type<T: Metadata>(&mut self) {
		self.register(T::type_ident(), T::type_def);
	}

	pub fn exists(&self, type_ident: &IdentKind) -> bool {
		self.types.contains_key(type_ident)
	}
}
