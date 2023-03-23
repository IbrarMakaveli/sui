// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::{cell::RefCell, fmt, marker::PhantomData};

use crate::programmable_transactions::types::StorageView;

use sui_types::{
    base_types::ObjectID,
    error::SuiResult,
    error::{ExecutionError, ExecutionErrorKind},
    move_package::MovePackage,
    object::Object,
    storage::{BackingPackageStore, ChildObjectResolver, LinkageInitializer},
};

use move_core_types::{
    account_address::AccountAddress,
    language_storage::{ModuleId, StructTag},
    resolver::{LinkageResolver, ModuleResolver, ResourceResolver},
};

pub struct LinkageInfo {
    pub pkg_id: ObjectID,
    /// Move package may not always be available where linkage info is needed (e.g., when
    /// publishing)
    pub running_pkg: Option<MovePackage>,
}

pub struct StorageContext<'a, E, S> {
    pub storage_view: &'a S,
    linkage_info: RefCell<Option<LinkageInfo>>,
    _p: PhantomData<E>,
}

impl<
        'a,
        E,
        S: ResourceResolver<Error = E>
            + ModuleResolver<Error = E>
            + BackingPackageStore
            + ChildObjectResolver,
    > StorageContext<'a, E, S>
{
    pub fn new(storage_view: &'a S) -> Self {
        Self {
            storage_view,
            linkage_info: RefCell::new(None),
            _p: PhantomData,
        }
    }

    pub fn set_context(&self, pkg_id: ObjectID) -> Result<(), ExecutionError> {
        if self.linkage_info.borrow().is_some() {
            return Err(ExecutionErrorKind::VMInvariantViolation.into());
        }
        let running_pkg = match self.storage_view.get_packages(&[pkg_id]).unwrap() {
            Ok(v) => Some(v[0].clone()),
            Err(_) => None,
        };
        self.linkage_info.replace(Some(LinkageInfo {
            pkg_id,
            running_pkg,
        }));
        Ok(())
    }

    pub fn reset_context(&self) {
        self.linkage_info.replace(None);
    }
}

impl<'a, E: fmt::Debug, S: StorageView<E>> ChildObjectResolver for StorageContext<'a, E, S> {
    fn read_child_object(&self, parent: &ObjectID, child: &ObjectID) -> SuiResult<Option<Object>> {
        self.storage_view.read_child_object(parent, child)
    }
}

impl<'a, E: fmt::Debug, S: StorageView<E>> ModuleResolver for StorageContext<'a, E, S> {
    type Error = E;

    fn get_module(&self, module_id: &ModuleId) -> Result<Option<Vec<u8>>, Self::Error> {
        self.storage_view.get_module(module_id)
    }
}

impl<'a, E: fmt::Debug, S: StorageView<E>> ResourceResolver for StorageContext<'a, E, S> {
    type Error = E;

    fn get_resource(
        &self,
        address: &AccountAddress,
        struct_tag: &StructTag,
    ) -> Result<Option<Vec<u8>>, Self::Error> {
        self.storage_view.get_resource(address, struct_tag)
    }
}

impl<'a, E: fmt::Debug, S: StorageView<E>> LinkageResolver for StorageContext<'a, E, S> {
    type Error = E;

    fn link_context(&self) -> AccountAddress {
        self.linkage_info.borrow().as_ref().unwrap().pkg_id.into()
    }

    fn relocate(&self, module_id: &ModuleId) -> Result<ModuleId, Self::Error> {
        let old_id: ObjectID = (*module_id.address()).into();
        let new_module_id = match self
            .linkage_info
            .borrow()
            .as_ref()
            .unwrap()
            .running_pkg
            .as_ref()
        {
            Some(move_pkg) => {
                let upgraded_id = move_pkg.linkage_table().get(&old_id).unwrap().upgraded_id;
                ModuleId::new(upgraded_id.into(), module_id.name().into())
            }
            None => module_id.clone(),
        };
        Ok(new_module_id)
    }
}

impl<'a, E: fmt::Debug, S: StorageView<E>> LinkageInitializer for StorageContext<'a, E, S> {
    fn set_context(&self, pkg_id: ObjectID) -> Result<(), ExecutionError> {
        self.set_context(pkg_id)
    }

    fn reset_context(&self) {
        self.reset_context();
    }
}
