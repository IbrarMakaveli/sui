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
    storage::{BackingPackageStore, ChildObjectResolver},
};

use move_core_types::{
    account_address::AccountAddress,
    language_storage::{ModuleId, StructTag},
    resolver::{LinkageResolver, ModuleResolver, ResourceResolver},
};

pub struct LinkageInfo {
    pub running_pkg: Option<MovePackage>,
}

pub struct StorageContext<'a, E, S> {
    pub storage_view: &'a S,
    linkage_info: RefCell<LinkageInfo>,
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
            linkage_info: RefCell::new(LinkageInfo { running_pkg: None }),
            _p: PhantomData,
        }
    }

    pub fn set_context(&self, running_pkg: MovePackage) -> Result<(), ExecutionError> {
        if self.linkage_info.borrow().running_pkg.is_some() {
            return Err(ExecutionErrorKind::VMInvariantViolation.into());
        }
        self.linkage_info.replace(LinkageInfo {
            running_pkg: Some(running_pkg),
        });
        Ok(())
        //Ok(self.linkage_info = Some(RefCell::new(LinkageInfo { running_pkg })))
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
        self.linkage_info
            .borrow()
            .running_pkg
            .as_ref()
            .unwrap()
            .id()
            .into()
    }

    fn relocate(&self, module_id: &ModuleId) -> Result<ModuleId, Self::Error> {
        Ok(module_id.clone())
        /*
        let old_id: ObjectID = (*module_id.address()).into();
        let new_id = self
            .linkage_info
            .running_pkg
            .linkage_table()
            .get(&old_id)
            .unwrap()
            .upgraded_id;
        Ok(ModuleId::new(new_id.into(), module_id.name().into()))
        */
    }
}
