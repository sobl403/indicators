pub fn simple_moving_average(data_set: &Vec<f64>, window_size: usize) -> Option<Vec<f64>> {
    if window_size > data_set.len() {
        return None;
    }

    let mut result: Vec<f64> = Vec::new();
    let mut window_start = 0;
    while window_start + window_size <= data_set.len() {
        let window_end = window_start + window_size;
        let data_slice = &data_set[window_start..window_end];
        let sum: f64 = data_slice.iter().sum();
        let average = sum / window_size as f64;
        result.push(average);
        window_start += 1;
    }

    Some(result)
}

pub fn rsi(data_set: &Vec<f64>, window_size: usize) -> Option<Vec<f64>> {
    let mut result: Vec<f64> = Vec::new();
    if window_size > data_set.len() {
        return None;
    }
    let mut previous_average_gain;
    let mut previous_average_loss;
    // RSI Step one
    let mut gains_sum = 0.0;
    let mut loss_sum = 0.0;
    for i in 0..(window_size + 1) {
        let gain = if i == 0 {
            0.0
        } else {
            (100.0 / data_set[i - 1]) * data_set[i] - 100.0
        };
        if gain >= 0.0 {
            gains_sum += gain;
        } else {
            loss_sum += gain.abs();
        }
    }
    let current_average_gain = gains_sum / window_size as f64;
    let current_average_loss = loss_sum / window_size as f64;
    let rsi_a = 100.0 - 100.0 / (1.0 + (current_average_gain / current_average_loss));
    previous_average_gain = current_average_gain;
    previous_average_loss = current_average_loss;
    result.push(rsi_a);
    // RSI Step two
    for i in (window_size + 1)..data_set.len() {
        let gain = (100.0 / data_set[i - 1]) * data_set[i] - 100.0;
        let (current_gain, current_loss) = if gain > 0.0 {
            (gain, 0.0)
        } else {
            (0.0, gain.abs())
        };
        let current_average_gain = (previous_average_gain * (window_size as f64 - 1.0)
            + current_gain)
            / window_size as f64;
        let current_average_loss = (previous_average_loss * (window_size as f64 - 1.0)
            + current_loss)
            / window_size as f64;
        previous_average_gain = current_average_gain;
        previous_average_loss = current_average_loss;
        let rsi = 100.0 - 100.0 / (1.0 + current_average_gain / current_average_loss);
        result.push(rsi);
    }
    Some(result)
}
