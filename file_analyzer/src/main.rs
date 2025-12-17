mod data_analysed;
use data_analysed::DataAnalysed;

fn main() {

    let analyzed_tab: [f32;6] = [14.0, 28.0,62.0,48.0,79.0,65.0];
    let mut data = DataAnalysed::new(analyzed_tab);

    data.calculate_mean();
    data.calculate_median();
    data.calculate_variance();
    data.calculate_maximum();
    data.calculate_minimum();
    data.calculate_range();

    println!("Mean: {}", data.mean);
    println!("Median: {}", data.median);
    println!("Variance: {}", data.variance);
    println!("Maximum: {}", data.maximum);
    println!("Minimum: {}", data.minimum);
    println!("Range: {}", data.range);

}

