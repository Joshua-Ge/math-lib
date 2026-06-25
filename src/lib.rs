use pyo3::prelude::*;

mod complex;
mod matrix;
mod physics_1;
mod vector2d;

pub use complex::ComplexNum;
pub use matrix::{Matrix, matmul};
pub use physics_1::f_ma;
pub use vector2d::{Vector2D, from_angle_deg, from_angle_rad, projection_angle};

#[pymodule]
fn math_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Matrix>()?;
    m.add_class::<ComplexNum>()?;
    m.add_function(wrap_pyfunction!(f_ma, m)?)?;
    m.add_function(wrap_pyfunction!(from_angle_deg, m)?)?;
    m.add_function(wrap_pyfunction!(from_angle_rad, m)?)?;
    m.add_function(wrap_pyfunction!(projection_angle, m)?)?;
    Ok(())
}
