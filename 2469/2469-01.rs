impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        return [celsius + 273.15, celsius * 1.80 + 32.00].to_vec();
    }
}
