use crate::grid::Grid;
use plotters::prelude::*;
use std::env;

fn plot_grid(grid: Grid) -> Result<(), Box<dyn std::error::Error>> {
    let cell_centers = grid.cell_centers();
    let mut output_path = env::current_dir()?;
    output_path.push("grid.png");

    // let path = env::current_dir()?;
    // println!("The current directory is {}", path.display());
    // Ok(())
    let root = BitMapBackend::new(&output_path, (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let sd = 0.13;

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .x_label_area_size(10)
        .y_label_area_size(10)
        .build_cartesian_2d(-0.1f64..1.1f64, -0.1f64..1.1f64)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    let plotting_area = chart.plotting_area();

    let range = plotting_area.get_pixel_range();

    let (xr, yr) = (chart.x_range(), chart.y_range());

    let (nx, ny) = grid.shape;

    for i in 0..nx {
        for j in 0..ny {
            plotting_area.draw_pixel(cell_centers[i][j], &BLACK);
        }
    }

    // for (x, y, c) in mandelbrot_set(xr, yr, (pw as usize, ph as usize), 100) {
    //     if c != 100 {
    //         plotting_area.draw_pixel((x, y), &MandelbrotHSL::get_color(c as f64 / 100.0))?;
    //     } else {
    //         plotting_area.draw_pixel((x, y), &BLACK)?;
    //     }
    // }

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", output_path.display());

    Ok(())
}

// const OUT_FILE_NAME: &str = "~/normal-dist.png";

fn main() {
    let grid = Grid::new((8, 8), (0.0f64, 0.0f64), (1.0f64, 1.0f64));
    plot_grid(grid).unwrap();
}

#[test]
fn entry_point() {
    main()
}
