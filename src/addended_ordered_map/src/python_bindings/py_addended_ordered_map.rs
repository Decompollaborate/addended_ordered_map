/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use alloc::sync::Arc;

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PyInt;

use crate::python_bindings::{PyFindSettings, PyIntoIter, PyRangeMut, PySizedValueBase};
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
        key: u64,
        settings: PyFindSettings,
    ) -> PyResult<Option<(Py<PyInt>, &Py<PySizedValueBase>)>> {
        let find_settings = settings.into_inner();

        if let Some((k, v)) = self.inner.find(&key, find_settings) {
            let k2 = Python::try_attach(|py| k.into_pyobject(py).map(|x| x.unbind()))
                .ok_or_else(|| PyRuntimeError::new_err("Internal conversion error"))??;

            Ok(Some((k2, v)))
        } else {
            Ok(None)
        }
    }

    #[pyo3(signature = (key, settings = PyFindSettings::new(true)))]
    pub fn find_key<'py>(&self, key: u64, settings: PyFindSettings) -> PyResult<Option<Py<PyInt>>> {
        Ok(self.find(key, settings)?.map(|x| x.0))
    }

    #[pyo3(signature = (key, settings = PyFindSettings::new(true)))]
    pub fn find_value<'py>(
        &self,
        key: u64,
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
        key: u64,
        new_value: Py<PySizedValueBase>,
        settings: PyFindSettings,
    ) -> PyResult<(&Py<PySizedValueBase>, bool)> {
        let find_settings = settings.into_inner();

        let (v, newly_created) = self
            .inner
            .find_mut_or_insert_with(key, find_settings, || Arc::new(new_value));
        Ok((v, newly_created))
    }

    #[pyo3(signature = (key, new_default, settings = PyFindSettings::new(true)))]
    pub fn find_or_insert_with<'py>(
        &mut self,
        key: u64,
        new_default: &Bound<'py, PyAny>,
        settings: PyFindSettings,
    ) -> PyResult<(&Py<PySizedValueBase>, bool)> {
        // if !new_default.is_callable()
        let find_settings = settings.into_inner();

        let (v, newly_created) = self.inner.find_mut_or_insert_with(key, find_settings, || {
            // call a callable python object/function/lambda/etc
            let result = new_default.call0().unwrap();
            let casted = result.cast().map(|x| x.clone().unbind());
            Arc::new(casted.unwrap())
        });
        Ok((v, newly_created))
    }

    pub fn contains_key_exact(&self, key: u64) -> bool {
        self.inner.contains_key_exact(&key)
    }

    pub fn pop_exact<'py>(
        &mut self,
        py: Python<'py>,
        key: u64,
    ) -> Option<(u64, Py<PySizedValueBase>)> {
        self.inner
            .pop_exact(&key)
            .map(|(k, v)| (k, v.clone_ref(py)))
    }

    pub fn pop_range<'py>(
        &mut self,
        py: Python<'py>,
        left: Option<u64>,
        right: Option<u64>,
    ) -> Vec<(u64, Py<PySizedValueBase>)> {
        fn map_impl<'py>(
            py: Python<'py>,
            iter: impl Iterator<Item = (u64, Arc<Py<PySizedValueBase>>)>,
        ) -> Vec<(u64, Py<PySizedValueBase>)> {
            iter.map(|(k, v)| (k, v.clone_ref(py))).collect()
        }
        match (left, right) {
            (Some(l), Some(r)) => map_impl(py, self.inner.extract_if(l..r, |_k, _v| true)),
            (Some(l), None) => map_impl(py, self.inner.extract_if(l.., |_k, _v| true)),
            (None, Some(r)) => map_impl(py, self.inner.extract_if(..r, |_k, _v| true)),
            (None, None) => map_impl(py, self.inner.extract_if(.., |_k, _v| true)),
        }
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /*
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

    pub fn range(&mut self, left: Option<u64>, right: Option<u64>) -> PyRangeMut {
        PyRangeMut::new(&mut self.inner, left, right)
    }

    // keys

    // values

    pub fn __repr__<'py>(&self, py: Python<'py>) -> String {
        let mut out = String::new();
        out += "AddendedOrderedMap({";
        out += self.repr_body(py).as_str();
        out += "})";
        out
    }

    pub fn __str__<'py>(&self, py: Python<'py>) -> String {
        self.__repr__(py)
    }
}

impl PyAddendedOrderedMap {
    pub fn repr_body<'py>(&self, py: Python<'py>) -> String {
        let mut body = String::new();
        let mut iter = self.inner.iter();

        // special case the first case to allow adding the comma in the loop
        if let Some((k, v)) = iter.next() {
            let v_repr = v.call_method0(py, "__repr__").unwrap();

            body += &format!("{k}: {v_repr}");
        } else {
            return body;
        }

        for (k, v) in iter {
            let v_repr = v.call_method0(py, "__repr__").unwrap();

            body += &format!(", {k}: {v_repr}");
        }

        body
    }
}
