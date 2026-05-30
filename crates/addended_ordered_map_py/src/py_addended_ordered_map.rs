/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use pyo3::prelude::*;

use addended_ordered_map::fallible::AddendedOrderedMapFallible;

use crate::py_alias::{PyK, PyKWA, PyS, PyV, PyVWA};
use crate::{PyFindSettings, PyIntoIter, PyRangeMut};

#[pyclass(name = "AddendedOrderedMap", module = "addended_ordered_map", generic)]
pub struct PyAddendedOrderedMap {
    // We use Arc because Py can't be just cloned
    // https://pyo3.rs/v0.28.2/migration.html#pyclone-is-now-gated-behind-the-py-clone-feature
    inner: AddendedOrderedMapFallible<PyKWA, PyVWA, PyS, PyErr>,
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
    pub fn find(&self, key: PyK, settings: PyFindSettings) -> PyResult<Option<(&PyK, &PyV)>> {
        let find_settings = settings.into_inner();

        if let Some((k, v)) = self.inner.find(&PyKWA::new(key), find_settings)? {
            Ok(Some((k, v)))
        } else {
            Ok(None)
        }
    }

    #[pyo3(signature = (key, settings = PyFindSettings::new(true)))]
    pub fn find_key(&self, key: PyK, settings: PyFindSettings) -> PyResult<Option<&PyK>> {
        Ok(self.find(key, settings)?.map(|x| x.0))
    }

    #[pyo3(signature = (key, settings = PyFindSettings::new(true)))]
    pub fn find_value(&self, key: PyK, settings: PyFindSettings) -> PyResult<Option<&PyV>> {
        Ok(self.find(key, settings)?.map(|x| x.1))
    }

    #[pyo3(signature = (key, inclusive = false))]
    pub fn find_left_of(&self, key: PyK, inclusive: bool) -> PyResult<Option<(&PyK, &PyV)>> {
        if let Some((k, v)) = self.inner.find_left_of(&PyKWA::new(key), inclusive) {
            Ok(Some((k, v)))
        } else {
            Ok(None)
        }
    }

    #[pyo3(signature = (key, inclusive = false))]
    pub fn find_right_of(&self, key: PyK, inclusive: bool) -> PyResult<Option<(&PyK, &PyV)>> {
        if let Some((k, v)) = self.inner.find_right_of(&PyKWA::new(key), inclusive) {
            Ok(Some((k, v)))
        } else {
            Ok(None)
        }
    }

    #[pyo3(signature = (key, new_value, settings = PyFindSettings::new(true)))]
    pub fn find_or_insert(
        &mut self,
        key: PyK,
        new_value: PyV,
        settings: PyFindSettings,
    ) -> PyResult<(&PyV, bool)> {
        let find_settings = settings.into_inner();

        let (v, newly_created) =
            self.inner
                .find_mut_or_insert_with(PyKWA::new(key), find_settings, || {
                    Ok(PyVWA::new(new_value))
                })?;
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

        let (v, newly_created) =
            self.inner
                .find_mut_or_insert_with(PyKWA::new(key), find_settings, || {
                    // call a callable python object/function/lambda/etc
                    let result = new_default.call0()?;
                    let casted = result.cast().map(|x| x.clone().unbind())?;
                    Ok(PyVWA::new(casted))
                })?;
        Ok((v, newly_created))
    }

    pub fn contains_key_exact(&self, key: PyK) -> bool {
        self.inner.contains_key_exact(&PyKWA::new(key))
    }

    pub fn pop_exact(&mut self, py: Python, key: PyK) -> Option<(PyK, PyV)> {
        self.inner
            .pop_exact(&PyKWA::new(key))
            .map(|(k, v)| (k.clone_ref(py), v.clone_ref(py)))
    }

    pub fn pop_range(
        &mut self,
        py: Python,
        left: Option<PyK>,
        right: Option<PyK>,
    ) -> Vec<(PyK, PyV)> {
        fn map_impl(py: Python, iter: impl Iterator<Item = (PyKWA, PyVWA)>) -> Vec<(PyK, PyV)> {
            iter.map(|(k, v)| (k.clone_ref(py), v.clone_ref(py)))
                .collect()
        }

        let left = left.map(PyKWA::new);
        let right = right.map(PyKWA::new);

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

    pub fn __repr__(&self, py: Python) -> PyResult<String> {
        let mut out = String::new();
        out += "AddendedOrderedMapFallible({";
        out += self.repr_body(py)?.as_str();
        out += "})";
        Ok(out)
    }

    pub fn __str__(&self, py: Python) -> PyResult<String> {
        self.__repr__(py)
    }
}

impl PyAddendedOrderedMap {
    pub fn repr_body(&self, py: Python) -> PyResult<String> {
        let mut body = String::new();
        let mut iter = self.inner.iter();

        // special case the first case to allow adding the comma in the loop
        if let Some((k, v)) = iter.next() {
            let k_repr = k.call_method0(py, "__repr__")?;
            let v_repr = v.call_method0(py, "__repr__")?;

            body += &format!("{k_repr}: {v_repr}");
        } else {
            return Ok(body);
        }

        for (k, v) in iter {
            let k_repr = k.call_method0(py, "__repr__")?;
            let v_repr = v.call_method0(py, "__repr__")?;

            body += &format!(", {k_repr}: {v_repr}");
        }

        Ok(body)
    }
}

impl Default for PyAddendedOrderedMap {
    fn default() -> Self {
        Self::new()
    }
}
