// Copyright 2017, 2020 Parity Technologies
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

//! Trie query recorder.

use crate::{TrieAccess, TrieRecorder, rstd::vec::Vec, TrieLayout, TrieHash};
use hashbrown::{HashMap, HashSet};

/// Records trie nodes as they pass it.
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Recorder<L: TrieLayout> {
	nodes: HashMap<TrieHash<L>, Vec<u8>>,
	keys: HashSet<Vec<u8>>,
}

impl<L: TrieLayout> Default for Recorder<L> {
	fn default() -> Self {
		Recorder::new()
	}
}

impl<L: TrieLayout> Recorder<L> {
	/// Create a new `Recorder` which records all given nodes.
	pub fn new() -> Self {
		Self {
			nodes: Default::default(),
			keys: Default::default(),
		}
	}

	/// Drain all visited records.
	pub fn drain(&mut self) -> Vec<(TrieHash<L>, Vec<u8>)> {
		self.nodes.drain().collect::<Vec<_>>()
	}
}

impl<L: TrieLayout> TrieRecorder<TrieHash<L>> for Recorder<L> {
	fn record<'a>(&mut self, access: TrieAccess<'a, TrieHash<L>>) {
		match access {
			TrieAccess::EncodedNode { hash, encoded_node } => { self.nodes.insert(hash, encoded_node.to_vec()); },
			TrieAccess::NodeOwned { hash, node_owned } => { self.nodes.insert(hash, node_owned.to_encoded::<L::Codec>()); }
			TrieAccess::Key(key) => { self.keys.insert(key.to_vec()); }
		}
	}
}
