use crate::grid::{Grid, ScalarField};

pub fn upwind_interpolation(
    field: &ScalarField,
    x_vel: &ScalarField,
    y_vel: &ScalarField,
    grid: &Grid,
) -> (ScalarField, ScalarField) {
    let (nx, ny) = grid.shape;
    let mut x_interp = vec![vec![0.0; ny]; nx];
    let mut y_interp = vec![vec![0.0; ny]; nx];

    for i in 0..nx {
        for j in 0..ny {
            // Do the x-interpolation
            if x_vel[i][j] > 0.0 {
                x_interp[i][j] = field[i][j];
            } else {
                x_interp[i][j] = field[i + 1][j];
            }

            // Do the y-interpolation
            if y_vel[i][j] > 0.0 {
                y_interp[i][j] = field[i][j];
            } else {
                y_interp[i][j] = field[i][j + 1];
            }
        }
    }

    // Handle the boundary conditions.
    for j in 0..ny {
        let i = nx - 1;
        if x_vel[i][j] > 0.0 {
            x_interp[i][j] = field[i][j];
        } else {
            x_interp[i][j] = field[0][j];
        }
    }

    for i in 0..nx {
        let j = ny - 1;
        if y_vel[i][j] > 0.0 {
            y_interp[i][j] = field[i][j];
        } else {
            y_interp[i][j] = field[i][0];
        }
    }

    (x_interp, y_interp)
}

pub fn advection(
    field: &ScalarField,
    x_vel: &ScalarField,
    y_vel: &ScalarField,
    grid: &Grid,
) -> ScalarField {
    let (nx, ny) = grid.shape;

    let (field_x, field_y) = upwind_interpolation(field, x_vel, y_vel, grid);
    let mut advected_field = vec![vec![0.0; ny]; nx];

    for i in 0..nx {
        for j in 0..ny {
            // Add the face values to the cell center
            let right_face = (x_vel[i][j] * field_x[i][j]) * grid.dy();
            let top_face = (y_vel[i][j] * field_y[i][j]) * grid.dx();

            let left_face = if i > 0 {
                (x_vel[i - 1][j] * field_x[i - 1][j]) * grid.dy()
            } else {
                (x_vel[nx - 1][j] * field_x[nx - 1][j]) * grid.dy()
            };

            let bottom_face = if j > 0 {
                (y_vel[i][j - 1] * field_y[i][j - 1]) * grid.dx()
            } else {
                (y_vel[i][ny - 1] * field_y[i][ny - 1]) * grid.dx()
            };

            advected_field[i][j] =
                (right_face - left_face + top_face - bottom_face) / grid.cell_volume();
        }
    }

    advected_field
}
