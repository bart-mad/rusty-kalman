use rand::Rng;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>>{

    let dt: f64 = 0.01;
    let T: f64  = 10.;
    let N_samples: i32 = (T/dt) as i32;

    let V_0: f64 = 10.;
    let x_0: f64 = 0.;
    let process_variance: f64 = 0.01;
    let process_noise: f64 = process_variance.sqrt();

    let mut rng: rand::prelude::ThreadRng = rand::rng();

    println!("dt {} -> T {}",dt,T);
    println!("V_0 {} -> N_samples {}",V_0,N_samples);


    let mut X: Vec<f64> = Vec::new();
    let mut V: Vec<f64> = Vec::new();

    X.push(x_0);
    V.push(V_0);

    for n in 1..=N_samples{

        let v: f64 = V[(n-1) as usize] + (rng.random_range(-1..1) as f64) * process_variance;
        let x: f64 = X[(n-1) as usize] + v * dt;

        X.push(x);
        V.push(v);

        println!("{}",x);
        println!("{}",v);
    }

     // Create a 800*600 bitmap and start drawing
     let mut backend = BitMapBackend::new("results.png", (800, 600)).into_drawing_area();

     backend.fill(&WHITE);
     let backend = backend.margin(10, 10, 10, 10);

     let mut chart = ChartBuilder::on(&backend)
     // Set the caption of the chart
     .caption("Position and Speed", ("sans-serif", 30).into_font())
     // Set the size of the label region
     .x_label_area_size(40)
     .y_label_area_size(80)
     // Finally attach a coordinate on the drawing area and make a chart context
     .build_cartesian_2d(0f64..1000f64, 0f64..100f64)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    // And we can draw something in the drawing area
    let mut sample: f64 = 0.;
    chart.draw_series(LineSeries::new(
        X.into_iter().map(|x| {
            sample += 1.0;
            return (sample, x)
        }),
        &RED,
    ))?;

    let mut sample: f64 = 0.;
    chart.draw_series(LineSeries::new(
        V.into_iter().map(|v| {
            sample += 1.0;
            return (sample, v)
        }),
        &BLUE,
    ))?;


     backend.present()?;
     Ok(())
 }
