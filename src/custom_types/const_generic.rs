//
// Const Generics and Type-Level Programming
//
// -- Using const generics for compile-time array and matrix operations
// Type-level array operations using const generics
#[derive(Debug, Clone)]
pub struct Array<T, const N: usize> {
    data: [T; N],
}

impl<T: Default + Copy, const N: usize> Array<T, N> {
    pub fn new() -> Self {
        Self {
            data: [T::default(); N],
        }
    }

    pub fn from_array(data: [T; N]) -> Self {
        Self { data }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn set(&mut self, index: usize, value: T) -> Result<(), &'static str> {
        if index < N {
            self.data[index] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn len(&self) -> usize {
        N
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }
}

// Simple concat operation for specific sizes (due to const generic limitations)
impl<T: Copy + Default> Array<T, 2> {
    pub fn concat_with_3(&self, other: &Array<T, 3>) -> Array<T, 5> {
        let mut result = [T::default(); 5];
        result[0] = self.data[0];
        result[1] = self.data[1];
        result[2] = other.data[0];
        result[3] = other.data[1];
        result[4] = other.data[2];
        Array { data: result }
    }
}

// Compile-time dimension checking for matrix operations
#[derive(Debug, Clone)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}

impl<T: Default + Copy, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn new() -> Self {
        Self {
            data: [[T::default(); C]; R],
        }
    }

    pub fn from_data(data: [[T; C]; R]) -> Self {
        Self { data }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(row)?.get(col)
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) -> Result<(), &'static str> {
        if row < R && col < C {
            self.data[row][col] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn rows(&self) -> usize {
        R
    }

    pub fn cols(&self) -> usize {
        C
    }
}

// Specific matrix multiplication implementations (due to const generic limitations)
impl<T> Matrix<T, 2, 3> 
where
    T: Default + Copy + std::ops::Mul<Output = T> + std::ops::AddAssign,
{
    pub fn multiply_with_3x2(&self, other: &Matrix<T, 3, 2>) -> Matrix<T, 2, 2> {
        let mut result = Matrix::new();

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..3 {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        result
    }
}

// Additional helpful implementations
impl<T: std::fmt::Display, const N: usize> Array<T, N> {
    pub fn display(&self) {
        print!("[");
        for (i, item) in self.data.iter().enumerate() {
            if i > 0 { print!(", "); }
            print!("{}", item);
        }
        println!("]");
    }
}

impl<T: std::fmt::Display, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn display(&self) {
        println!("Matrix {}x{}:", R, C);
        for row in &self.data {
            print!("  [");
            for (j, item) in row.iter().enumerate() {
                if j > 0 { print!(", "); }
                print!("{:>3}", item);
            }
            println!("]");
        }
    }
}

// Demonstration of different sized types
pub fn demonstrate_different_sizes() {
    println!("    🎯 Different compile-time sizes:");
    
    // These are all different types at compile time!
    let small: Array<i32, 3> = Array::new();
    let medium: Array<i32, 5> = Array::new();
    let large: Array<i32, 10> = Array::new();
    
    println!("      Small array (3 elements): {} bytes", std::mem::size_of_val(&small));
    println!("      Medium array (5 elements): {} bytes", std::mem::size_of_val(&medium));
    println!("      Large array (10 elements): {} bytes", std::mem::size_of_val(&large));
}

// Type-level size validation
pub fn compile_time_size_check() {
    // These are checked at compile time!
    const ARRAY_SIZE: usize = 5;
    const MATRIX_ROWS: usize = 3;
    const MATRIX_COLS: usize = 4;
    
    println!("🔢 Compile-time constants:");
    println!("    Array size: {}", ARRAY_SIZE);
    println!("    Matrix dimensions: {}x{}", MATRIX_ROWS, MATRIX_COLS);
}
