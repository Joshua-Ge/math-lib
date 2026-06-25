use pyo3::prelude::*;

#[pyclass]
pub struct ComplexNum {
    real: f64,
    imag: f64,
}

#[pymethods]
impl ComplexNum {
    #[new]
    fn new(re: f64, im: f64) -> Self {
        ComplexNum { real: re, imag: im }
    }

    fn __add__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        if let Ok(c) = other.extract::<PyRef<ComplexNum>>() {
            let res = ComplexNum {
                real: self.real + c.real,
                imag: self.imag + c.imag,
            };

            return Ok(Py::new(other.py(), res)?.into_any());
        }

        if let Ok(c) = other.extract::<f64>() {
            let res = ComplexNum {
                real: self.real + c,
                imag: self.imag,
            };

            return Ok(Py::new(other.py(), res)?.into_any());
        }

        Ok(other.py().NotImplemented())
    }

    fn __sub__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        if let Ok(c) = other.extract::<PyRef<ComplexNum>>() {
            let res = ComplexNum {
                real: self.real - c.real,
                imag: self.imag - c.imag,
            };
            return Ok(Py::new(other.py(), res)?.into_any());
        }

        if let Ok(c) = other.extract::<f64>() {
            let res = ComplexNum {
                real: self.real - c,
                imag: self.imag,
            };

            return Ok(Py::new(other.py(), res)?.into_any());
        }

        Ok(other.py().NotImplemented())
    }

    fn __mul__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        if let Ok(c) = other.extract::<PyRef<ComplexNum>>() {
            let res = ComplexNum {
                real: self.real * c.real - self.imag * c.imag,
                imag: self.real * c.imag + self.imag * c.real,
            };
            return Ok(Py::new(other.py(), res)?.into_any());
        }

        if let Ok(c) = other.extract::<f64>() {
            let res = ComplexNum {
                real: self.real * c,
                imag: self.imag * c,
            };
            return Ok(Py::new(other.py(), res)?.into_any());
        }

        Ok(other.py().NotImplemented())
    }

    fn __truediv__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        if let Ok(c) = other.extract::<PyRef<ComplexNum>>() {
            let denom = c.real.powi(2) + c.imag.powi(2);
            if denom == 0.0 {
                return Err(pyo3::exceptions::PyZeroDivisionError::new_err(
                    "Can't divide by zero",
                ));
            }

            let res = ComplexNum {
                real: (self.real * c.real + self.imag * c.imag) / denom,
                imag: (self.imag * c.real - self.real * c.imag) / denom,
            };
            return Ok(Py::new(other.py(), res)?.into_any());
        }

        if let Ok(c) = other.extract::<f64>() {
            if c == 0.0 {
                return Err(pyo3::exceptions::PyZeroDivisionError::new_err(format!(
                    "Can't divide by zero"
                )));
            }

            let res = ComplexNum {
                real: self.real / c,
                imag: self.imag / c,
            };

            return Ok(Py::new(other.py(), res)?.into_any());
        }

        Ok(other.py().NotImplemented())
    }

    fn __radd__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        self.__add__(other)
    }

    fn __rsub__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        if let Ok(c) = other.extract::<f64>() {
            let res = ComplexNum {
                real: c - self.real,
                imag: -self.imag,
            };
            return Ok(Py::new(other.py(), res)?.into_any());
        }

        Err(pyo3::exceptions::PyNotImplementedError::new_err(format!(
            "Subtraction is not supported on this type"
        )))
    }

    fn __rmul__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        self.__mul__(other)
    }

    fn __rtruediv__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        if let Ok(c) = other.extract::<f64>() {
            let denom = self.real.powi(2) + self.imag.powi(2);
            if denom == 0.0 {
                return Err(pyo3::exceptions::PyZeroDivisionError::new_err(
                    "Can't divide by zero",
                ));
            }

            let res = ComplexNum {
                real: (c * self.real) / denom,
                imag: -(c * self.imag) / denom,
            };

            return Ok(Py::new(other.py(), res)?.into_any());
        }

        Err(pyo3::exceptions::PyNotImplementedError::new_err(format!(
            "division is not supported on this type"
        )))
    }

    fn modulus(&self) -> PyResult<f64> {
        let number: f64 = (self.real.powi(2) + self.imag.powi(2)).sqrt();

        Ok(number)
    }

    fn __abs__(&self) -> PyResult<f64> {
        self.modulus()
    }

    fn __repr__(&self) -> String {
        if self.imag >= 0.0 {
            format!("{} + {}i", self.real, self.imag)
        } else {
            format!("{} {}i", self.real, self.imag)
        }
    }

    fn conjugate(&self) -> PyResult<ComplexNum> {
        let res: ComplexNum = ComplexNum {
            real: self.real,
            imag: -1.0 * self.imag,
        };

        Ok(res)
    }

    fn __eq__(&self, other: &ComplexNum) -> PyResult<bool> {
        Ok(self.real == other.real && self.imag == other.imag)
    }

    fn __ne__(&self, other: &ComplexNum) -> PyResult<bool> {
        Ok(self.real != other.real || self.imag != other.imag)
    }

    fn phase(&self) -> PyResult<f64> {
        Ok(self.imag.atan2(self.real))
    }

    fn to_polar(&self) -> PyResult<(f64, f64)> {
        let mag = self.modulus()?;
        let angle = self.phase()?;
        Ok((mag, angle))
    }

    fn __pow__(&self, n: f64, _mod: i64) -> PyResult<ComplexNum> {
        let (r, theta) = self.to_polar()?;
        let mag = r.powf(n);
        let angle = theta * n;

        let res = ComplexNum {
            real: mag * angle.cos(),
            imag: mag * angle.sin(),
        };

        Ok(res)
    }
}
