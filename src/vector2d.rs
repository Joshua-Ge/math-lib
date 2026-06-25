use pyo3::prelude::*;

const PI: f64 = std::f64::consts::PI;
// the 2D vector data type consists of 2 f64 feilds which represent the x and y coordinates on a plane
#[pyclass]
pub struct Vector2D {
    #[pyo3(get)]
    x: f64,
    #[pyo3(get)]
    y: f64,
}

#[pymethods]
impl Vector2D {
    // creates a new vector
    #[new]
    fn new(x: f64, y: f64) -> Self {
        Vector2D { x, y }
    }

    // enables the alteration of a vector after its been set so you don't have to do A.new() instead A.set()
    fn set(&mut self, x: f64, y: f64) -> PyResult<()> {
        self.x = x;
        self.y = y;
        Ok(())
    }

    // enables the addition logic for python for the "+" operator
    fn __add__(&self, other: &Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    // enables addition if the vector is in a diffrent position and __add__ fails
    fn __radd__(&self, other: &Vector2D) -> Vector2D {
        self.__add__(other)
    }

    // enables vector subtraction
    fn __sub__(&self, other: &Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    // enables vector subtraction in a diffrent direction
    fn __rsub__(&self, other: &Vector2D) -> Vector2D {
        Vector2D {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }

    // enables vector multiplication for scalars only
    fn __mul__(&self, other: f64) -> Vector2D {
        let x: f64 = self.x * other;
        let y: f64 = self.y * other;

        Vector2D { x, y }
    }

    // enables vector multiplication for scalars only but in the reverse
    fn __rmul__(&self, other: f64) -> Vector2D {
        self.__mul__(other)
    }

    // enabling vectors to be put to the negitive by placing a "-" sign in front.
    fn __neg__(&self) -> Vector2D {
        let x = self.x * -1.0;
        let y = self.y * -1.0;

        Vector2D { x, y }
    }

    // adds the dot product for vectors using the "@" symbole
    fn __matmul__(&self, other: &Vector2D) -> f64 {
        self.x * other.x + self.y * other.y
    }

    // enables the comparison between diffrent vectors and returns a boolian
    fn __eq__(&self, other: &Vector2D) -> bool {
        if self.x == other.x && self.y == other.y {
            true
        } else {
            false
        }
    }

    // adds a parallel check for two vectors
    // this uses the angle between two vectors rule dot product of A * B - |A|*|B| = 0
    fn parallel(&self, other: &Vector2D) -> bool {
        (self.__matmul__(other)).abs() - (self.magnitude() * other.magnitude().abs()) < 1e-10
    }

    // adds a perpendicular check for two vectors
    // this also uses the angle between two vectors
    // A * B = |A| * |B| * cos(0), |A| * |B| * cos(0) = 0
    // therefor A * B = 0
    fn perpendicular(&self, other: &Vector2D) -> bool {
        (self.__matmul__(other)).abs() < 1e-10
    }

    // enable the calculation of the magnituid for two vectors
    fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    // finds the midpoint between two vectors
    fn midpoint(&self, other: &Vector2D) -> Vector2D {
        let x: Vector2D = Vector2D {
            x: (other.x + self.x) / 2.0,
            y: (other.y + self.y) / 2.0,
        };
        x
    }

    // finds the angle between two vectors using the relationship:
    // theta = acos( (A*B) / ( |A| * |B| ) )
    fn angle_between_vectors(&self, other: &Vector2D) -> f64 {
        let value: f64 = self.__matmul__(other) / (self.magnitude() * other.magnitude());
        value.clamp(-1.0, 1.0).acos()
    }
}

// function for generating a vector from the magnitude and the angle in radians
#[pyfunction]
pub fn from_angle_rad(angle: f64, magnitude: f64) -> Vector2D {
    let x = magnitude * angle.cos();
    let y = magnitude * angle.sin();

    Vector2D { x, y }
}

// function for generating a vector from the magnitude and the angle in degrees
#[pyfunction]
pub fn from_angle_deg(angle: f64, magnitude: f64) -> Vector2D {
    let x = magnitude * (angle * (PI / 180.0)).cos();
    let y = magnitude * (angle * (PI / 180.0)).sin();

    Vector2D { x, y }
}

// creates a projection vector where A is projected on B
#[pyfunction]
pub fn projection_angle(a: &Vector2D, b: &Vector2D) -> Vector2D {
    b.__mul__(a.__matmul__(b) / (b.magnitude().powf(2.0)))
}
