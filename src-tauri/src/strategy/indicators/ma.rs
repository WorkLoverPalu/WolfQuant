pub struct MA {
    period: usize,
    values: Vec<f64>,
    current_ma: Option<f64>,
}

impl MA {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            values: Vec::new(),
            current_ma: None,
        }
    }
    
    pub fn update(&mut self, value: f64) -> Option<f64> {
        self.values.push(value);
        
        if self.values.len() > self.period {
            self.values.remove(0);
        }
        
        if self.values.len() == self.period {
            let sum: f64 = self.values.iter().sum();
            let ma = sum / self.period as f64;
            self.current_ma = Some(ma);
            Some(ma)
        } else {
            None
        }
    }
    
    pub fn current(&self) -> Option<f64> {
        self.current_ma
    }
    
    pub fn calculate(data: &[f64], period: usize) -> Vec<f64> {
        if data.len() < period {
            return Vec::new();
        }
        
        let mut result = Vec::with_capacity(data.len() - period + 1);
        
        for i in 0..=(data.len() - period) {
            let sum: f64 = data[i..(i + period)].iter().sum();
            let ma = sum / period as f64;
            result.push(ma);
        }
        
        result
    }
}