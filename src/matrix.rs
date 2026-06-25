use pyo3::prelude::*;
use rayon::prelude::*;

#[pyclass]
pub struct Matrix {
    #[pyo3(get)]
    rows: usize,
    #[pyo3(get)]
    cols: usize,
    pub data: Vec<f64>,
}

#[pymethods]
impl Matrix {
    #[new]
    fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    fn set(&mut self, values: Vec<Vec<f64>>) -> PyResult<()> {
        let r = values.len();

        // if the amount of rows and the selected amount of rows are not the same throw an error
        if r != self.rows {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Expected {} rows got {}",
                self.rows, r
            )));
        }
        // collects the indentation and the value
        for (i, row) in values.iter().enumerate() {
            // stores the value in rows in this case it will be one row from the matrix
            let rows: &Vec<f64> = row;

            // if the specifide amount of cols and the amount given are not the same then throw an error
            if rows.len() != self.cols {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "Expected {} colums got {}",
                    self.cols,
                    rows.len()
                )));
            }
            // if everything else is fine then we can populate the matrix
            for (j, &val) in rows.iter().enumerate() {
                self.data[i * self.cols + j] = val;
            }
        }

        Ok(())
    }

    #[getter(shape)]
    fn get_shape(&self) -> PyResult<(usize, usize)> {
        Ok((self.rows, self.cols))
    }

    fn __add__(&self, other: &Matrix) -> PyResult<Matrix> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Dimensions Mismatch"
            )));
        }

        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a + b)
            .collect();

        Ok(Matrix {
            rows: self.rows,
            cols: self.cols,
            data,
        })
    }

    fn __sub__(&self, other: &Matrix) -> PyResult<Matrix> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Dimensions Mismatch"
            )));
        }

        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a - b)
            .collect();

        Ok(Matrix {
            rows: self.rows,
            cols: self.cols,
            data,
        })
    }

    fn __rmul__(&self, scalar: f64) -> PyResult<Matrix> {
        let data = self.data.iter().map(|x| x * scalar).collect();
        Ok(Matrix {
            rows: self.rows,
            cols: self.cols,
            data,
        })
    }

    fn __neg__(&self) -> PyResult<Matrix> {
        let data = self.data.iter().map(|x| x * -1.0).collect();
        Ok(Matrix {
            rows: self.rows,
            cols: self.cols,
            data,
        })
    }

    fn __eq__(&self, other: &Matrix) -> PyResult<bool> {
        let mut ans: bool = false;

        if self.rows != other.rows || self.cols != other.cols {
            ans = false;
        } else {
            if self.data == other.data {
                ans = true;
            }
        }

        Ok(ans)
    }

    fn __matmul__(&self, other: &Matrix) -> PyResult<Matrix> {
        if self.cols != other.rows {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Dimensions Mismatch {}x{}, {}x{}",
                self.rows, self.cols, other.rows, other.cols
            )));
        }

        let mut data = vec![0.0; self.rows * other.cols];

        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    data[i * other.cols + j] +=
                        self.data[i * self.cols + k] * other.data[k * other.cols + j]
                }
            }
        }

        Ok(Matrix {
            rows: self.rows,
            cols: other.cols,
            data: data,
        })
    }

    fn get(&self, x: usize, y: usize) -> PyResult<f64> {
        if self.rows <= y || self.cols <= x {
            return Err(pyo3::exceptions::PyIndexError::new_err(format!(
                "Can not index out of bounds"
            )));
        }

        Ok(self.data[y * self.cols + x])
    }

    fn round(&self, dec: usize) -> PyResult<Matrix> {
        let factor: f64 = 10_f64.powi(dec as i32);
        let mut res: Vec<f64> = vec![0.0; self.data.len()];
        for i in 0..self.data.len() {
            res[i] = (self.data[i] * factor).round() / factor;
        }

        Ok(Matrix {
            rows: self.rows,
            cols: self.cols,
            data: res,
        })
    }

    fn __repr__(&self) -> String {
        let mut out: String = String::new();

        for i in 0..self.rows {
            let row: Vec<String> = (0..self.cols)
                .map(|x| format!("{}", self.data[i * self.cols + x]))
                .collect();
            out += &format!("|{}|\n", row.join(" "));
        }

        out
    }

    fn inverse(&self) -> PyResult<Matrix> {
        if self.rows != self.cols {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Matrix must be square to invert",
            ));
        }

        let n = self.rows;

        // Step 1: Perform LU decomposition with partial pivoting
        let (l, u, p) = self.lu_decompose()?;

        // Step 2: Solve LU × X = P for each column of identity matrix
        let mut result = vec![0.0; n * n];

        for col in 0..n {
            // Create column vector from permuted identity matrix
            let mut b = vec![0.0; n];
            for i in 0..n {
                if p[i] == col {
                    b[i] = 1.0;
                }
            }

            // Forward substitution: solve Ly = b
            let mut y = vec![0.0; n];
            for i in 0..n {
                let mut sum = b[i];
                for j in 0..i {
                    sum -= l.data[i * n + j] * y[j];
                }
                y[i] = sum; // L has 1's on diagonal
            }

            // Backward substitution: solve Ux = y
            let mut x = vec![0.0; n];
            for i in (0..n).rev() {
                let mut sum = y[i];
                for j in (i + 1)..n {
                    sum -= u.data[i * n + j] * x[j];
                }
                x[i] = sum / u.data[i * n + i];
            }

            // Store column in result
            for i in 0..n {
                result[i * n + col] = x[i];
            }
        }

        Ok(Matrix {
            rows: n,
            cols: n,
            data: result,
        })
    }

    // Helper function: LU decomposition with partial pivoting
    // Returns (L, U, permutation vector)
    fn lu_decompose(&self) -> PyResult<(Matrix, Matrix, Vec<usize>)> {
        let n = self.rows;

        // Initialize L as identity, U as copy of self
        let mut l_data = vec![0.0; n * n];
        let mut u_data = self.data.clone();

        // L starts as identity
        for i in 0..n {
            l_data[i * n + i] = 1.0;
        }

        // Permutation vector (tracks row swaps)
        let mut p: Vec<usize> = (0..n).collect();

        for k in 0..n {
            // Find pivot
            let mut max_val = u_data[k * n + k].abs();
            let mut max_row = k;

            for i in (k + 1)..n {
                if u_data[i * n + k].abs() > max_val {
                    max_val = u_data[i * n + k].abs();
                    max_row = i;
                }
            }

            // Check for singular matrix
            if max_val < 1e-12 {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "Matrix is singular (not invertible)",
                ));
            }

            // Swap rows in U and track permutation
            if max_row != k {
                for j in 0..n {
                    u_data.swap(k * n + j, max_row * n + j);
                    if j < k {
                        l_data.swap(k * n + j, max_row * n + j);
                    }
                }
                p.swap(k, max_row);
            }

            // Elimination
            for i in (k + 1)..n {
                let factor = u_data[i * n + k] / u_data[k * n + k];
                l_data[i * n + k] = factor;

                for j in k..n {
                    u_data[i * n + j] -= factor * u_data[k * n + j];
                }
            }
        }

        let l = Matrix {
            rows: n,
            cols: n,
            data: l_data,
        };

        let u = Matrix {
            rows: n,
            cols: n,
            data: u_data,
        };

        Ok((l, u, p))
    }

    fn square(&self) -> PyResult<Matrix> {
        Ok(matmul(self, self)?)
    }

    fn cube(&self) -> PyResult<Matrix> {
        let b: Matrix = matmul(self, self)?;
        let ans: Matrix = matmul(&b, self)?;
        Ok(ans)
    }

    fn _repr_latex_(&self) -> String {
        let mut out = String::new();

        out.push_str("\\begin{bmatrix}");

        for r in 0..self.rows {
            for c in 0..self.cols {
                let idx = r * self.cols + c;
                out.push_str(&format!("{}", self.data[idx]));

                if c != self.cols - 1 {
                    out.push_str(" & ");
                }
            }

            if r != self.rows - 1 {
                out.push_str(" \\\\ ");
            }
        }

        out.push_str("\\end{bmatrix}");

        out
    }

    fn display(&self) -> PyResult<String> {
        let mut out = String::new();

        for r in 0..self.rows {
            out.push('[');

            for c in 0..self.cols {
                let index = r * self.cols + c;

                if c > 0 {
                    out.push_str("  ");
                }

                out.push_str(&format!("{}", self.data[index]));
            }

            out.push(']');
            out.push('\n');

            if r != self.rows - 1 {
                out.push('\n');
            }
        }

        Ok(out)
    }

    fn determinant2x2_bool(&self) -> PyResult<bool> {
        if self.cols != 2 && self.rows != 2 {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Must be a 2x2 matrix"
            )));
        }
        let response: bool;
        let det: f64 = self.data[0] * self.data[3] - self.data[1] * self.data[2];
        // return true when can inverse
        // return false when can't inverse
        if det != 0.0 {
            response = true;
        } else {
            response = false;
        }

        Ok(response)
    }

    fn determinant2x2_calc(&self) -> PyResult<f64> {
        if self.cols != 2 && self.rows != 2 {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Must be a 2x2 matrix"
            )));
        }
        let det: f64 = self.data[0] * self.data[3] - self.data[1] * self.data[2];

        Ok(det)
    }

    fn inverse2x2(&self) -> PyResult<Matrix> {
        if self.cols != 2 && self.rows != 2 {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Must be a 2x2 matrix"
            )));
        }

        if self.determinant2x2_bool()? == false {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Can not invert"
            )));
        }

        let mut data: Vec<f64> = self.data.clone();
        let temp: f64;

        temp = data[0];
        data[0] = data[3];
        data[3] = temp;
        data[1] = data[1] * -1.0;
        data[2] = data[2] * -1.0;

        let det: f64 = self.determinant2x2_calc()?;

        for i in 0..self.data.len() {
            data[i] = det * data[i];
        }

        Ok(Matrix {
            rows: self.rows,
            cols: self.cols,
            data,
        })
    }

    fn transpose(&self) -> PyResult<Matrix> {
        let mut data: Vec<f64> = vec![0.0; self.cols * self.rows];

        for i in 0..self.rows {
            for j in 0..self.cols {
                data[j * self.rows + i] = self.data[j * self.cols + i]
            }
        }

        Ok(Matrix {
            rows: self.cols,
            cols: self.rows,
            data: data,
        })
    }

    fn dot(&self, other: &Matrix) -> PyResult<Matrix> {
        if self.cols != other.rows {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Dimensions Mismatch {}x{}, {}x{}",
                self.rows, self.cols, other.rows, other.cols
            )));
        }

        let other_t = other.transpose()?;

        let mut data = vec![0.0; self.rows * other.cols];

        data.par_iter_mut().enumerate().for_each(|(index, value)| {
            let r = index / other.cols;
            let c = index % other.cols;
            *value = (0..self.cols)
                .map(|k| self.data[r * self.cols + k] * other_t.data[c * other.cols + k])
                .sum();
        });

        Ok(Matrix {
            rows: self.rows,
            cols: other.cols,
            data: data,
        })
    }
}

pub fn matmul(a: &Matrix, other: &Matrix) -> PyResult<Matrix> {
    if a.cols != other.rows {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Dimensions Mismatch {}x{}, {}x{}",
            a.rows, a.cols, other.rows, other.cols
        )));
    }

    let mut data = vec![0.0; a.rows * other.cols];

    for i in 0..a.rows {
        for j in 0..other.cols {
            for k in 0..a.cols {
                data[i * other.cols + j] += a.data[i * a.cols + k] * other.data[k * other.cols + j]
            }
        }
    }

    Ok(Matrix {
        rows: a.rows,
        cols: other.cols,
        data: data,
    })
}
