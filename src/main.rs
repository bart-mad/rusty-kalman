#![allow(non_snake_case)]

use plotters::{prelude::*, style::full_palette::PURPLE};
use rand::Rng;

#[derive(Clone)]
struct Gaussian {
    mean: f64,
    variance: f64,
}

fn update(prior: &Gaussian, x_measured: f64, measurement_variance: f64) -> Gaussian {
    let mut update: Gaussian = prior.clone();
    update.mean = (prior.variance * x_measured + measurement_variance * prior.mean)
        / (prior.variance + measurement_variance);
    update.variance =
        (prior.variance * measurement_variance) / (prior.variance + measurement_variance);

    update
}

fn predict(prior: &Gaussian, velocity: f64, dt: f64, model_variance: f64) -> Gaussian {
    let mut predicted: Gaussian = prior.clone();
    predicted.mean = prior.mean + velocity * dt;
    predicted.variance = prior.variance + model_variance;

    predicted
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dt: f64 = 0.01;
    let T: f64 = 10.;
    let N_samples: i32 = (T / dt) as i32;

    let V_0: f64 = 10.;
    let x_0: f64 = 0.;
    let process_variance: f64 = 0.00001;
    let process_noise: f64 = process_variance.sqrt();

    let mut rng: rand::prelude::ThreadRng = rand::rng();

    let mut X: Vec<f64> = Vec::new();
    let mut V: Vec<f64> = Vec::new();

    X.push(x_0);
    V.push(V_0);

    for n in 1..=N_samples {
        let v: f64 = V[(n - 1) as usize] + (rng.random_range(-1..1) as f64) * process_noise;
        let x: f64 = X[(n - 1) as usize] + v * dt;

        X.push(x);
        V.push(v);
    }

    // Measurement model

    let measurement_variance: f64 = 200.;
    let measurement_noise: f64 = measurement_variance.sqrt();

    let mut X_measured: Vec<f64> = Vec::new();

    for n in 0..=N_samples {
        let x: f64 =
            X[(n) as usize] + (rng.random_range(-measurement_noise..measurement_noise) as f64); //* measurement_noise;

        X_measured.push(x);
    }

    // Kalman

    // Initial state estimation

    let x_prior: f64 = 1.;
    let v_model: f64 = 9.0;
    let model_variance: f64 = 3.;

    let mut X_result: Vec<f64> = Vec::new();

    let mut prior: Gaussian = Gaussian {
        mean: x_prior,
        variance: 0.,
    };

    let mut posterior: Gaussian = update(&prior, X_measured[0], measurement_variance);

    X_result.push(posterior.mean);

    prior = posterior.clone();

    for n in 1..=N_samples {
        prior = predict(&prior, v_model, dt, model_variance);
        posterior = update(&prior, X_measured[n as usize], measurement_variance);

        X_result.push(posterior.mean);

        prior = posterior.clone();
    }

    // Create a 800*600 bitmap and start drawing
    let backend = BitMapBackend::new("results.png", (800, 600)).into_drawing_area();

    let _ = backend.fill(&WHITE);
    let backend = backend.margin(10, 10, 10, 10);

    let mut chart = ChartBuilder::on(&backend)
        // Set the caption of the chart
        .caption("Position and Speed", ("sans-serif", 30).into_font())
        // Set the size of the label region
        .x_label_area_size(40)
        .y_label_area_size(80)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(0 as f64..N_samples as f64, 0f64..300f64)?;

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
            return (sample, x);
        }),
        &RED,
    ))?;

    let mut sample: f64 = 0.;
    chart.draw_series(LineSeries::new(
        V.into_iter().map(|v| {
            sample += 1.0;
            return (sample, v);
        }),
        &BLUE,
    ))?;

    // And we can draw something in the drawing area
    let mut sample: f64 = 0.;
    chart.draw_series(LineSeries::new(
        X_measured.into_iter().map(|x| {
            sample += 1.0;
            return (sample, x);
        }),
        &GREEN,
    ))?;

    // And we can draw something in the drawing area
    let mut sample: f64 = 0.;
    chart.draw_series(LineSeries::new(
        X_result.into_iter().map(|x| {
            sample += 1.0;
            return (sample, x);
        }),
        &PURPLE,
    ))?;

    backend.present()?;
    Ok(())
}
