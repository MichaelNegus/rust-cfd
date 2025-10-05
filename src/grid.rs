/// Type alias for a 2D scalar field represented as a vector of vectors of f64 values.
pub type ScalarField = Vec<Vec<f64>>;

/// Struct for a 2D computational grid.
pub struct Grid {
    pub shape: (usize, usize), // Number of cells in each dimension
    pub origin: (f64, f64),    // Coordinates of the lower-left corner
    pub size: (f64, f64),      // Size of the grid in each dimension
}

impl Grid {
    /// Creates a new grid with the given shape, origin, and size.
    pub fn new(shape: (usize, usize), origin: (f64, f64), size: (f64, f64)) -> Self {
        Grid {
            shape,
            origin,
            size,
        }
    }

    pub fn dx(&self) -> f64 {
        self.size.0 / self.shape.0 as f64
    }

    pub fn dy(&self) -> f64 {
        self.size.1 / self.shape.1 as f64
    }

    pub fn cell_volume(&self) -> f64 {
        self.dx() * self.dy()
    }

    /// Get the cell-centers of the grid.
    ///
    /// Returns a 2D vector of cell centers for (i, j) indexing, where i = 0..nx-1 and j = 0..ny-1.
    pub fn cell_centers(&self) -> Vec<Vec<(f64, f64)>> {
        let (nx, ny) = self.shape;
        let (x0, y0) = self.origin;

        let mut cell_centers = Vec::with_capacity(nx * ny);

        for i in 0..nx {
            let mut row = Vec::with_capacity(ny);
            let x = x0 + (i as f64 + 0.5) * self.dx();
            for j in 0..ny {
                let y = y0 + (j as f64 + 0.5) * self.dy();
                row.push((x, y));
            }
            cell_centers.push(row);
        }

        cell_centers
    }

    // Get the x-face centers
    //
    // These are of shape (nx, ny), and are for each cell
    pub fn x_face_centers(&self) -> Vec<Vec<(f64, f64)>> {
        let (nx, ny) = self.shape;
        let (x0, y0) = self.origin;

        let mut x_face_centers = Vec::with_capacity(nx * ny);

        for i in 0..nx {
            let mut row = Vec::with_capacity(ny);
            let x = x0 + (i as f64 + 1.0) * self.dx();
            for j in 0..ny {
                let y = y0 + (j as f64 + 0.5) * self.dy();
                row.push((x, y));
            }
            x_face_centers.push(row);
        }

        x_face_centers
    }

    // Get the y-face centers
    //
    // These are of shape (nx, ny), and are for each cell
    pub fn y_face_centers(&self) -> Vec<Vec<(f64, f64)>> {
        let (nx, ny) = self.shape;
        let (x0, y0) = self.origin;

        let mut y_face_centers = Vec::with_capacity(nx * ny);

        for i in 0..nx {
            let mut row = Vec::with_capacity(ny);
            let x = x0 + (i as f64 + 0.5) * self.dx();
            for j in 0..ny {
                let y = y0 + (j as f64 + 1.0) * self.dy();
                row.push((x, y));
            }
            y_face_centers.push(row);
        }

        y_face_centers
    }
}
