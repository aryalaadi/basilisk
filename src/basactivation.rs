/*
    file: src/basactivation.rs
    license: LGPL3
*/

#[derive(Clone)]
pub enum BASActivation {
    NONE,
    RELU,
    LOGSIG,
}

pub fn relu(x: f64) -> f64 {
    if x > 0.0 {
        return x;
    } else {
        return 0.0;
    }
}

pub fn logsig(x: f64) -> f64 {
    1.0 / (1.0 + f64::powf(std::f64::consts::E, x * (-1.0)))
}

impl BASActivation {
    pub fn activate(self, func: fn(f64) -> f64, x: f64) -> f64 {
        match self {
            BASActivation::NONE => return func(x),
            BASActivation::RELU => return relu(func(x)),
            BASActivation::LOGSIG => return logsig(func(x)),
        }
    }
}
