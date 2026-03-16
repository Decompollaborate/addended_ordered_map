/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use alloc::sync::Arc;

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PyInt;

use crate::fallible::AddendedOrderedMapFallible;
use crate::python_bindings::py_alias::{PyK, PyS, PyV};
use crate::python_bindings::{PyFindSettings, PyIntoIter, PyRangeMut};

#[pyclass(name = "AddendedOrderedMap", module = "addended_ordered_map", generic)]
pub struct PyAddendedOrderedMap {
    // We use Arc because Py can't be just cloned
    // https://pyo3.rs/v0.28.2/migration.html#pyclone-is-now-gated-behind-the-py-clone-feature
    inner: AddendedOrderedMapFallible<PyK, Arc<PyV>, PyS, PyErr>,
}

#[pymethods]
impl PyAddendedOrderedMap {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: AddendedOrderedMapFallible::new(),
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
        key: PyK,
        settings: PyFindSettings,
    ) -> PyResult<Option<(Py<PyInt>, &PyV)>> {
        let find_settings = settings.into_inner();

        if let Some((k, v)) = self.inner.find(&key, find_settings)? {
            let k2 = Python::try_attach(|py| k.into_pyobject(py).map(|x| x.unbind()))
                .ok_or_else(|| PyRuntimeError::new_err("Internal conversion error"))??;

            Ok(Some((k2, v)))
        } else {
            Ok(None)
        }
    }

    #[pyo3(signature = (key, settings = PyFindSettings::new(true)))]
    pub fn find_key<'py>(&self, key: PyK, settings: PyFindSettings) -> PyResult<Option<Py<PyInt>>> {
        Ok(self.find(key, settings)?.map(|x| x.0))
    }

    #[pyo3(signature = (key, settings = PyFindSettings::new(true)))]
    pub fn find_value<'py>(&self, key: PyK, settings: PyFindSettings) -> PyResult<Option<&PyV>> {
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
        key: PyK,
        new_value: PyV,
        settings: PyFindSettings,
    ) -> PyResult<(&PyV, bool)> {
        let find_settings = settings.into_inner();

        let (v, newly_created) = self
            .inner
            .find_mut_or_insert_with(key, find_settings, || Ok(Arc::new(new_value)))?;
        Ok((v, newly_created))
    }

    #[pyo3(signature = (key, new_default, settings = PyFindSettings::new(true)))]
    pub fn find_or_insert_with<'py>(
        &mut self,
        key: PyK,
        new_default: &Bound<'py, PyAny>,
        settings: PyFindSettings,
    ) -> PyResult<(&PyV, bool)> {
        // if !new_default.is_callable()
        let find_settings = settings.into_inner();

        let (v, newly_created) = self.inner.find_mut_or_insert_with(key, find_settings, || {
            // call a callable python object/function/lambda/etc
            let result = new_default.call0()?;
            let casted = result.cast().map(|x| x.clone().unbind())?;
            Ok(Arc::new(casted))
        })?;
        Ok((v, newly_created))
    }

    pub fn contains_key_exact(&self, key: PyK) -> bool {
        self.inner.contains_key_exact(&key)
    }

    pub fn pop_exact<'py>(&mut self, py: Python<'py>, key: PyK) -> Option<(PyK, PyV)> {
        self.inner
            .pop_exact(&key)
            .map(|(k, v)| (k, v.clone_ref(py)))
    }

    pub fn pop_range<'py>(
        &mut self,
        py: Python<'py>,
        left: Option<PyK>,
        right: Option<PyK>,
    ) -> Vec<(PyK, PyV)> {
        fn map_impl<'py>(
            py: Python<'py>,
            iter: impl Iterator<Item = (PyK, Arc<PyV>)>,
        ) -> Vec<(PyK, PyV)> {
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

    pub fn range(&mut self, left: Option<PyK>, right: Option<PyK>) -> PyRangeMut {
        PyRangeMut::new(&mut self.inner, left, right)
    }

    // keys

    // values

    pub fn __repr__<'py>(&self, py: Python<'py>) -> PyResult<String> {
        let mut out = String::new();
        out += "AddendedOrderedMapFallible({";
        out += self.repr_body(py)?.as_str();
        out += "})";
        Ok(out)
    }

    pub fn __str__<'py>(&self, py: Python<'py>) -> PyResult<String> {
        self.__repr__(py)
    }
}

impl PyAddendedOrderedMap {
    pub fn repr_body<'py>(&self, py: Python<'py>) -> PyResult<String> {
        let mut body = String::new();
        let mut iter = self.inner.iter();

        // special case the first case to allow adding the comma in the loop
        if let Some((k, v)) = iter.next() {
            let v_repr = v.call_method0(py, "__repr__")?;

            body += &format!("{k}: {v_repr}");
        } else {
            return Ok(body);
        }

        for (k, v) in iter {
            let v_repr = v.call_method0(py, "__repr__")?;

            body += &format!(", {k}: {v_repr}");
        }

        Ok(body)
    }
}
