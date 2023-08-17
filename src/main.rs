use rand::Rng;
use plotters::prelude::*;

pub const NUM_EDGES : u32 = 3;
pub const NUM_ITER : u32 = 1000000;
pub const RADIUS : f64 = 10000.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let verts : Vec<(f64, f64)> = get_vertices_vec(NUM_EDGES);
    let points : Vec<(f64, f64)> = get_points(verts, NUM_ITER);
    graph_points(points)?;

    Ok(())
}


pub fn get_vertices_vec(num_edges : u32) -> Vec<(f64, f64)> {
    let mut out : Vec<(f64, f64)> = Vec::new();
    let angle_step : f64 = 2.0 * std::f64::consts::PI / (num_edges as f64);
    let mut current_angle : f64 = 0.0;

    for _ in 0..num_edges {
        out.push((
            current_angle.sin() * RADIUS,
            current_angle.cos() * RADIUS,
        ));

        current_angle += angle_step;
    }

    out
}


pub fn get_points(verts : Vec<(f64, f64)>, num_iter : u32) -> Vec<(f64, f64)> {
    let mut out : Vec<(f64, f64)> = Vec::new();

    let mut current_point : (f64, f64) = verts[0];
    let mut thread = rand::thread_rng();
    let num_verts = verts.len();
    for _ in 0..num_iter {
        let vertex = verts[thread.gen_range(0..num_verts)];

        current_point = find_midpoint(current_point, vertex);

        out.push(current_point);
    }

    out
}


pub fn find_midpoint(a : (f64, f64), b : (f64, f64)) -> (f64, f64) {
    (
        (a.0 + b.0) / 2.0,
        (a.1 + b.1) / 2.0,
    )
}


pub fn graph_points(points : Vec<(f64, f64)>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("fractal.png", (640, 480))
        .into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(10, 10, 10, 10);
    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(-10000f64..10000f64, -10000f64..10000f64)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(PointSeries::of_element(
        points, 
        1, 
        &BLACK, 
        &|c, _s, st| {
            EmptyElement::at(c) + Pixel::new((0,0), st.filled())
        }
    ))?;

    root.present()?;

    Ok(())
}
