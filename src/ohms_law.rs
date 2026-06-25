use pyo3::prelude::*;

// V=IR
#[pyfunction]
pub fn ohms_law_calc(value1: f64, value2: f64, subject: &str) -> PyResult<f64> {
    let answer: f64;
    match subject {
        "v" => answer = volts_calc(value1, value2),
        "i" => answer = current(value1, value2)?,
        "r" => answer = resistance(value1, value2)?,
        _ => {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Please select an accepted form (v,i,r)"
            )));
        }
    }

    Ok(answer)
}

fn volts_calc(current: f64, resistance: f64) -> f64 {
    current * resistance
}

fn current(voltage: f64, resistance: f64) -> PyResult<f64> {
    if resistance == 0.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Resistance can't be zero"
        )));
    }
    Ok(voltage / resistance)
}

fn resistance(voltage: f64, current: f64) -> PyResult<f64> {
    if current == 0.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "current can't be zero"
        )));
    }
    Ok(voltage / current)
}
