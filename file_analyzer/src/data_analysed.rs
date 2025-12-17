pub struct DataAnalysed {
    pub analyzed_tab: [f32;6],
    pub mean: f32,
    pub median: f32,
    pub variance: f32,
    pub maximum: f32,
    pub minimum: f32,
    pub range: f32,
}

impl DataAnalysed{
    pub fn new(data: [f32;6]) -> Self {
        Self {
            analyzed_tab: data,
            mean: 0.0,
            median: 0.0,
            variance: 0.0,
            maximum: 0.0,
            minimum: 0.0,
            range: 0.0,
        }
    }


    pub fn calculate_mean(&mut self){
        for i in 0.. self.analyzed_tab.len(){
            self.mean += self.analyzed_tab[i]
        }
        self.mean / self.analyzed_tab.len() as f32;
    }

    pub fn calculate_median(&mut self){
        let mut sumsq = 0.0;
        for &x in &self.analyzed_tab {
            let d = x - self.mean;
            sumsq += d * d; // ou d.powi(2)
        }
        self.variance = sumsq / (self.analyzed_tab.len() as f32);
    }
    
    pub fn calculate_variance(&mut self){
        let mut sumsq = 0.0;
        for &x in &self.analyzed_tab {
            let d = x - self.mean;
            sumsq += d * d; // ou d.powi(2)
        }
        self.variance = sumsq / (self.analyzed_tab.len() as f32);
    }

    pub fn calculate_maximum(&mut self){
        self.maximum= self.analyzed_tab[0];
        for i in 1 .. self.analyzed_tab.len(){
            if self.analyzed_tab[i] > self.maximum
            {
                self.maximum = self.analyzed_tab[i];
            }
        }
    }

    pub fn calculate_minimum(&mut self){
    self.minimum= self.analyzed_tab[0];
        for i in 1 .. self.analyzed_tab.len(){
            if self.analyzed_tab[i] < self.minimum
            {
                self.minimum = self.analyzed_tab[i];
            }
        }
    }

    pub fn calculate_range(&mut self){
        self.range =self.maximum - self.minimum;
    }

}