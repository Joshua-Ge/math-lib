use pyo3::prelude::*;

// n₁sin(θ₁) = n₂sin(θ₂)
#[pyfunction]
pub fn snells_law_calc(value1: f64, value2: f64, value3: f64, subject: &str) -> PyResult<f64> {
    let answer: f64;

    match subject {
        "n" => answer = refractive_index(value1, value2, value3)?,
        "theta" => answer = angle(value1, value2, value3)?,
        _ => {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Please select an accepted form (n, theta)",
            ));
        }
    }

    Ok(answer)
}

// n = (n_known * sin(theta_known)) / sin(theta_unknown)
fn refractive_index(known_n: f64, known_theta: f64, unknown_theta: f64) -> PyResult<f64> {
    let denominator = unknown_theta.to_radians().sin();

    if denominator.abs() < f64::EPSILON {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Angle cannot result in sin(theta) = 0",
        ));
    }

    Ok((known_n * known_theta.to_radians().sin()) / denominator)
}

// theta = asin((n_known * sin(theta_known)) / n_unknown)
fn angle(known_n: f64, known_theta: f64, unknown_n: f64) -> PyResult<f64> {
    if unknown_n == 0.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Refractive index cannot be zero",
        ));
    }

    let ratio = (known_n * known_theta.to_radians().sin()) / unknown_n;

    if !(-1.0..=1.0).contains(&ratio) {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "No valid solution exists for the supplied values",
        ));
    }

    Ok(ratio.asin().to_degrees())
}
