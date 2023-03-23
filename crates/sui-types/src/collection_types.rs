// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::id::ID;

/// Rust version of the Move sui::vec_map::VecMap type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct VecMap<K, V> {
    pub contents: Vec<Entry<K, V>>,
}

/// Rust version of the Move sui::vec_map::Entry type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Entry<K, V> {
    pub key: K,
    pub value: V,
}

/// Rust version of the Move sui::vec_set::VecSet type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct VecSet<T> {
    pub contents: Vec<T>,
}

/// Rust version of the Move sui::table::Table type.
#[derive(Debug, Default, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct TableVec {
    pub contents: Table,
}

/// Rust version of the Move sui::table::Table type.
#[derive(Debug, Default, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Table {
    pub id: ID,
    pub size: u64,
}

/// Rust version of the Move sui::linked_table::LinkedTable type.
#[derive(Debug, Default, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct LinkedTable<K> {
    pub id: ID,
    pub size: u64,
    pub head: Option<K>,
    pub tail: Option<K>,
}

/// Rust version of the Move sui::bag::Bag type.
#[derive(Debug, Default, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Bag {
    pub id: ID,
    pub size: u64,
}
