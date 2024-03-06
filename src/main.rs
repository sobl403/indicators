mod binance;
mod models;
mod statistics;
mod utils;
use plotters::prelude::*;
use std::collections::HashMap;
use std::io;
use std::rc::Rc;
#[cfg(test)]
mod test_statistics;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = utils::get_client();
    let result = binance::get_klines(client.clone(), "15m", "BTCUSDT", 500).await;

    // println!("{:?}", result);

    let kline_data = match result {
        Some(kline_data) => kline_data,
        _ => {
            panic!("Smth went wrong")
        }
    };

    // println!("first result: {:?}", kline_data[2]);
    let price_data: Vec<f64> = kline_data.iter().rev().take(100).map(|f| f.close).collect();

    // let price_to_time_data = HashMap::new();

    // for price in &price_data {
    //     price_to_time_data.insert(kline_data[0], price);
    // }
    println!("Type what you want: SMA or RSI?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Type SMA or RSI");
    let result = if input.trim().to_lowercase() == "sma" {
        statistics::simple_moving_average(&price_data, 25)
    } else if input.trim().to_lowercase() == "rsi" {
        statistics::rsi(&price_data, 25)
    } else {
        None
    };

    if input.trim().to_lowercase() == "sma" {
        let text = "SMA".to_string();
        draw(&result, 0.0, 100.0, 30000.0, 75000.0, text);
        println!("SMA: {:?}", result);
    } else {
        let text = "RSI".to_string();
        draw(&result, 0.0, 100.0, 0.0, 100.0, text);
        let test_text = "hi".to_string();
        println!("RSI: {:?}", result);
    };
    Ok(())
}

fn draw(
    result: &Option<Vec<f64>>,
    spacexleft: f32,
    spacexright: f32,
    spaceydown: f32,
    spaceytop: f32,
    text: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("graph.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE);
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption(text, ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(spacexleft..spacexright, spaceydown..spaceytop)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    let mut new_vec = Vec::new();
    if let Some(data) = result {
        for (x, y) in data.into_iter().enumerate() {
            new_vec.push((x as f32, *y as f32));
        }
    }

    chart.draw_series(LineSeries::new(new_vec, &RED))?;
    root.present()?;
    Ok(())
}
