pub struct MACD {
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
    fast_ema: Vec<f64>,
    slow_ema: Vec<f64>,
    macd_line: Vec<f64>,
    signal_line: Vec<f64>,
    histogram: Vec<f64>,
}

impl MACD {
    pub fn new(fast_period: usize, slow_period: usize, signal_period: usize) -> Self {
        Self {
            fast_period,
            slow_period,
            signal_period,
            fast_ema: Vec::new(),
            slow_ema: Vec::new(),
            macd_line: Vec::new(),
            signal_line: Vec::new(),
            histogram: Vec::new(),
        }
    }
    
    pub fn calculate(data: &[f64], fast_period: usize, slow_period: usize, signal_period: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        if data.len() < slow_period {
            return (Vec::new(), Vec::new(), Vec::new());
        }
        
        // 计算快速EMA
        let fast_ema = Self::calculate_ema(data, fast_period);
        
        // 计算慢速EMA
        let slow_ema = Self::calculate_ema(data, slow_period);
        
        // 计算MACD线
        let mut macd_line = Vec::with_capacity(slow_ema.len());
        for i in 0..slow_ema.len() {
            let fast_idx = i + (fast_ema.len() - slow_ema.len());
            macd_line.push(fast_ema[fast_idx] - slow_ema[i]);
        }
        
        // 计算信号线
        let signal_line = Self::calculate_ema(&macd_line, signal_period);
        
        // 计算柱状图
        let mut histogram = Vec::with_capacity(signal_line.len());
        for i in 0..signal_line.len() {
            let macd_idx = i + (macd_line.len() - signal_line.len());
            histogram.push(macd_line[macd_idx] - signal_line[i]);
        }
        
        (macd_line, signal_line, histogram)
    }
    
    fn calculate_ema(data: &[f64], period: usize) -> Vec<f64> {
        if data.len() < period {
            return Vec::new();
        }
        
        let mut ema = Vec::with_capacity(data.len() - period + 1);
        
        // 第一个EMA值使用SMA
        let mut current_ema = data[0..period].iter().sum::<f64>() / period as f64;
        ema.push(current_ema);
        
        // 计算后续EMA值
        let multiplier = 2.0 / (period as f64 + 1.0);
        
        for i in period..data.len() {
            current_ema = (data[i] - current_ema) * multiplier + current_ema;
            ema.push(current_ema);
        }
        
        ema
    }
    
    pub fn update(&mut self, value: f64) -> Option<(f64, f64, f64)> {
        // 更新快速EMA
        if self.fast_ema.is_empty() {
            // 初始化
            if self.fast_period <= 1 {
                self.fast_ema.push(value);
            } else {
                // 需要更多数据
                return None;
            }
        } else {
            let multiplier = 2.0 / (self.fast_period as f64 + 1.0);
            let new_ema = (value - self.fast_ema.last().unwrap()) * multiplier + self.fast_ema.last().unwrap();
            self.fast_ema.push(new_ema);
        }
        
        // 更新慢速EMA
        if self.slow_ema.is_empty() {
            // 初始化
            if self.slow_period <= 1 {
                self.slow_ema.push(value);
            } else {
                // 需要更多数据
                return None;
            }
        } else {
            let multiplier = 2.0 / (self.slow_period as f64 + 1.0);
            let new_ema = (value - self.slow_ema.last().unwrap()) * multiplier + self.slow_ema.last().unwrap();
            self.slow_ema.push(new_ema);
        }
        
        // 计算MACD线
        if self.fast_ema.len() > 0 && self.slow_ema.len() > 0 {
            let macd = self.fast_ema.last().unwrap() - self.slow_ema.last().unwrap();
            self.macd_line.push(macd);
            
            // 更新信号线
            if self.macd_line.len() >= self.signal_period {
                if self.signal_line.is_empty() {
                    // 初始化信号线
                    let sum: f64 = self.macd_line[0..self.signal_period].iter().sum();
                    let signal = sum / self.signal_period as f64;
                    self.signal_line.push(signal);
                } else {
                    let multiplier = 2.0 / (self.signal_period as f64 + 1.0);
                    let new_signal = (macd - self.signal_line.last().unwrap()) * multiplier + self.signal_line.last().unwrap();
                    self.signal_line.push(new_signal);
                }
                
                // 计算柱状图
                let histogram = macd - self.signal_line.last().unwrap();
                self.histogram.push(histogram);
                
                return Some((macd, self.signal_line.last().unwrap().clone(), histogram));
            }
        }
        
        None
    }
    
    pub fn current(&self) -> Option<(f64, f64, f64)> {
        if !self.macd_line.is_empty() && !self.signal_line.is_empty() && !self.histogram.is_empty() {
            Some((
                *self.macd_line.last().unwrap(),
                *self.signal_line.last().unwrap(),
                *self.histogram.last().unwrap(),
            ))
        } else {
            None
        }
    }
}