pub struct RSI {
    period: usize,
    prev_value: Option<f64>,
    gains: Vec<f64>,
    losses: Vec<f64>,
    current_rsi: Option<f64>,
}

impl RSI {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            prev_value: None,
            gains: Vec::new(),
            losses: Vec::new(),
            current_rsi: None,
        }
    }
    
    pub fn update(&mut self, value: f64) -> Option<f64> {
        if let Some(prev) = self.prev_value {
            let change = value - prev;
            
            let gain = if change > 0.0 { change } else { 0.0 };
            let loss = if change < 0.0 { -change } else { 0.0 };
            
            self.gains.push(gain);
            self.losses.push(loss);
            
            if self.gains.len() > self.period {
                self.gains.remove(0);
                self.losses.remove(0);
            }
            
            if self.gains.len() == self.period {
                let avg_gain: f64 = self.gains.iter().sum::<f64>() / self.period as f64;
                let avg_loss: f64 = self.losses.iter().sum::<f64>() / self.period as f64;
                
                if avg_loss == 0.0 {
                    self.current_rsi = Some(100.0);
                } else {
                    let rs = avg_gain / avg_loss;
                    let rsi = 100.0 - (100.0 / (1.0 + rs));
                    self.current_rsi = Some(rsi);
                }
            }
        }
        
        self.prev_value = Some(value);
        self.current_rsi
    }
    
    pub fn current(&self) -> Option<f64> {
        self.current_rsi
    }
    
    pub fn calculate(data: &[f64], period: usize) -> Vec<f64> {
        if data.len() <= period {
            return Vec::new();
        }
        
        let mut result = Vec::with_capacity(data.len() - period);
        let mut gains = Vec::with_capacity(period);
        let mut losses = Vec::with_capacity(period);
        
        // 计算第一个周期的涨跌
        for i in 1..=period {
            let change = data[i] - data[i - 1];
            gains.push(if change > 0.0 { change } else { 0.0 });
            losses.push(if change < 0.0 { -change } else { 0.0 });
        }
        
        // 计算第一个RSI值
        let avg_gain = gains.iter().sum::<f64>() / period as f64;
        let avg_loss = losses.iter().sum::<f64>() / period as f64;
        
        let mut prev_avg_gain = avg_gain;
        let mut prev_avg_loss = avg_loss;
        
        if avg_loss == 0.0 {
            result.push(100.0);
        } else {
            let rs = avg_gain / avg_loss;
            result.push(100.0 - (100.0 / (1.0 + rs)));
        }
        
        // 计算后续RSI值
        for i in (period + 1)..data.len() {
            let change = data[i] - data[i - 1];
            let gain = if change > 0.0 { change } else { 0.0 };
            let loss = if change < 0.0 { -change } else { 0.0 };
            
            // 使用平滑RSI公式
            let avg_gain = (prev_avg_gain * (period as f64 - 1.0) + gain) / period as f64;
            let avg_loss = (prev_avg_loss * (period as f64 - 1.0) + loss) / period as f64;
            
            prev_avg_gain = avg_gain;
            prev_avg_loss = avg_loss;
            
            if avg_loss == 0.0 {
                result.push(100.0);
            } else {
                let rs = avg_gain / avg_loss;
                result.push(100.0 - (100.0 / (1.0 + rs)));
            }
        }
        
        result
    }
}