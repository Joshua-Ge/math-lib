use pyo3::prelude::*;

#[pyfunction]
pub fn f_ma(option1: f64, option2: f64, subject: &str) -> PyResult<f64> {
    let answer: f64;
    match subject {
        "f" => answer = force_calc(option1, option2),
        "m" => answer = mass_calc(option1, option2)?,
        "a" => answer = accel_calc(option1, option2)?,
        _ => {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Please select an accepted form (f,m,a)"
            )));
        }
    }

    Ok(answer)
}

fn force_calc(mass: f64, acceleration: f64) -> f64 {
    mass * acceleration
}

fn mass_calc(force: f64, acceleration: f64) -> PyResult<f64> {
    if acceleration == 0.0 {
        return Err(pyo3::exceptions::PyArithmeticError::new_err(format!(
            "Can't Divide by zero"
        )));
    }

    Ok(force / acceleration)
}

fn accel_calc(force: f64, mass: f64) -> PyResult<f64> {
    if mass == 0.0 {
        return Err(pyo3::exceptions::PyArithmeticError::new_err(format!(
            "Can't Divide by zero"
        )));
    }

    Ok(force / mass)
}
