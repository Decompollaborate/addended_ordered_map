/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use alloc::sync::Arc;

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PyInt;

use crate::python_bindings::{PyFindSettings, PyIntoIter, PySizedValueBase};
use crate::AddendedOrderedMap;

#[pyclass(name = "AddendedOrderedMap", module = "addended_ordered_map", generic)]
pub struct PyAddendedOrderedMap {
    // We use Arc because Py can't be just cloned
    // https://pyo3.rs/v0.28.2/migration.html#pyclone-is-now-gated-behind-the-py-clone-feature
    inner: AddendedOrderedMap<u64, Arc<Py<PySizedValueBase>>, u64>,
}

#[pymethods]
impl PyAddendedOrderedMap {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: AddendedOrderedMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[pyo3(signature = (key, settings = PyFindSettings::new(true)))]
    pub fn find<'py>(
        &self,
        key: &Bound<'py, PyAny>,
        settings: PyFindSettings,
    ) -> PyResult<Option<(Py<PyInt>, &Py<PySizedValueBase>)>> {
        let extracted_key: u64 = key.extract()?;
        let find_settings = settings.into_inner();

        if let Some((k, v)) = self.inner.find(&extracted_key, find_settings) {
            let k2 = Python::try_attach(|py| k.into_pyobject(py).map(|x| x.unbind()))
                .ok_or_else(|| PyRuntimeError::new_err("Internal conversion error"))??;

            Ok(Some((k2, v)))
        } else {
            Ok(None)
        }
    }

    #[pyo3(signature = (key, settings = PyFindSettings::new(true)))]
    pub fn find_key<'py>(
        &self,
        key: &Bound<'py, PyAny>,
        settings: PyFindSettings,
    ) -> PyResult<Option<Py<PyInt>>> {
        Ok(self.find(key, settings)?.map(|x| x.0))
    }

    #[pyo3(signature = (key, settings = PyFindSettings::new(true)))]
    pub fn find_value<'py>(
        &self,
        key: &Bound<'py, PyAny>,
        settings: PyFindSettings,
    ) -> PyResult<Option<&Py<PySizedValueBase>>> {
        Ok(self.find(key, settings)?.map(|x| x.1))
    }

    /*
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized + Ord,
    {
        self.inner.contains_key(key)
    }
    */

    #[pyo3(signature = (key, new_value, settings = PyFindSettings::new(true)))]
    pub fn find_or_insert<'py>(
        &mut self,
        key: &Bound<'py, PyAny>,
        new_value: Py<PySizedValueBase>,
        settings: PyFindSettings,
    ) -> PyResult<(&Py<PySizedValueBase>, bool)> {
        let extracted_key: u64 = key.extract()?;
        let find_settings = settings.into_inner();

        let (v, newly_created) =
            self.inner
                .find_mut_or_insert_with(extracted_key, find_settings, || Arc::new(new_value));
        Ok((v, newly_created))
    }

    #[pyo3(signature = (key, new_default, settings = PyFindSettings::new(true)))]
    pub fn find_or_insert_with<'py>(
        &mut self,
        key: &Bound<'py, PyAny>,
        new_default: &Bound<'py, PyAny>,
        settings: PyFindSettings,
    ) -> PyResult<(&Py<PySizedValueBase>, bool)> {
        // if !new_default.is_callable()
        let extracted_key: u64 = key.extract()?;
        let find_settings = settings.into_inner();

        let (v, newly_created) =
            self.inner
                .find_mut_or_insert_with(extracted_key, find_settings, || {
                    // call a callable python object/function/lambda/etc
                    let result = new_default.call0().unwrap();
                    let casted = result.cast().map(|x| x.clone().unbind());
                    Arc::new(casted.unwrap())
                });
        Ok((v, newly_created))
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /*
    pub fn remove<Q>(&mut self, value: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Ord,
    {
        self.inner.remove(value)
    }

    pub fn remove_entry<Q>(&mut self, value: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Ord,
    {
        self.inner.remove_entry(value)
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.inner.retain(f);
    }
    */

    pub fn __iter__(&self) -> PyIntoIter {
        PyIntoIter::new(self.inner.clone())
    }

    // range

    // keys

    // values

    // __str__

    // __repr__
}
