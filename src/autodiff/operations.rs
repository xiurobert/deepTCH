use std::ops;
use crate::autodiff::variable::{PartialDerivative, Variable};

impl ops::Add<Variable> for Variable {

    type Output = Variable;

    fn add(self, _rhs: Variable) -> Variable {
        let result = self.value + _rhs.value;
        let gradients = vec![
            PartialDerivative::new(self.value, 1.0),
            PartialDerivative::new(_rhs.value, 1.0),
        ];
        Variable::new(result, gradients)
    }
}

impl ops::Mul<Variable> for Variable {

    type Output = Variable;

    fn mul(self, _rhs: Variable) -> Variable {
        let result = self.value * _rhs.value;
        let gradients = vec![
            PartialDerivative::new(self.value, _rhs.value),
            PartialDerivative::new(_rhs.value, self.value),
        ];
        Variable::new(result, gradients)
    }
}