use std::fmt;

pub struct Variable {
    pub value: f32,
    pub gradients: Vec<PartialDerivative>,
}

pub struct PartialDerivative {
    pub value_wrt: f32,
    pub gradient_wrt_value: f32
}

impl Variable {
    pub fn new(value: f32, gradients: Vec<PartialDerivative>) -> Self {
        Variable {
            value,
            gradients,
        }
    }

    pub fn new_no_gradients(value: f32) -> Self {
        Variable {
            value,
            gradients: vec![],
        }
    }

}

impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Variable: {} | Derivatives: {:?}", self.value, self.gradients)
    }
}

impl PartialDerivative {
    pub fn new(value_wrt: f32, gradient_wrt_value: f32) -> Self {
        PartialDerivative {
            value_wrt,
            gradient_wrt_value,
        }
    }
}

impl fmt::Debug for PartialDerivative {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Value WRT: {} | Gradient: {}]", self.value_wrt, self.gradient_wrt_value)
    }
}