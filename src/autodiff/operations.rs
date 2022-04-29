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

impl ops::Sub<Variable> for Variable {
    type Output = Variable;
    
    fn sub(self, _rhs: Variable) -> Variable {
        let result = self.value - _rhs.value;
        let gradients = vec![
            PartialDerivative::new(self.value, 1.0),
            PartialDerivative::new(_rhs.value, -1.0),
        ];
        Variable::new(result, gradients)
    }
}

impl ops::Div<Variable> for Variable {
    type Output = Variable;
    
    fn div(self, _rhs: Variable) -> Variable {
        let result = self.value / _rhs.value;
        let gradients = vec![
            PartialDerivative::new(self.value, 1.0 / _rhs.value.powi(2)),
            PartialDerivative::new(_rhs.value, -self.value / _rhs.value.powi(2)),
        ];
        Variable::new(result, gradients)
    }
}

#[cfg(test)]
mod tests {
    use crate::Variable;

    #[test]
    fn test_add() {
        let a = Variable::new_no_gradients(2.0);
        let b = Variable::new_no_gradients(3.0);
        let c = a + b;
        assert_eq!(c.value, 5.0);
    }

    #[test]
    fn test_sub() {
        let a = Variable::new_no_gradients(2.0);
        let b = Variable::new_no_gradients(3.0);
        let c = a - b;
        assert_eq!(c.value, -1.0);
    }

    #[test]
    fn test_multiply() {
        let a = Variable::new_no_gradients(2.0);
        let b = Variable::new_no_gradients(3.0);
        let c = a * b;
        assert_eq!(c.value, 6.0);
    }

    #[test]
    fn test_div() {
        let a = Variable::new_no_gradients(2.0);
        let b = Variable::new_no_gradients(3.0);
        let c = a / b;
        assert_eq!(c.value, 2.0/3.0);
    }

    #[test]
    fn test_fwd_ad_add() {
        let a = Variable::new_no_gradients(3.0);
        let b = Variable::new_no_gradients(4.0);
        let result = a + b;
        assert_eq!(result.gradients.len(), 2);
        assert_eq!(result.gradients[0].value_wrt, 3.0);
        assert_eq!(result.gradients[1].value_wrt, 4.0);
        assert_eq!(result.gradients[0].gradient_wrt_value, 1.0);
        assert_eq!(result.gradients[1].gradient_wrt_value, 1.0);
    }

    #[test]
    fn test_fwd_ad_sub() {
        let a = Variable::new_no_gradients(3.0);
        let b = Variable::new_no_gradients(4.0);
        let result = a - b;
        assert_eq!(result.gradients.len(), 2);
        assert_eq!(result.gradients[0].value_wrt, 3.0);
        assert_eq!(result.gradients[1].value_wrt, 4.0);
        assert_eq!(result.gradients[0].gradient_wrt_value, 1.0);
        assert_eq!(result.gradients[1].gradient_wrt_value, -1.0);
    }

    #[test]
    fn test_fwd_ad_mul() {
        let a = Variable::new_no_gradients(3.0);
        let b = Variable::new_no_gradients(4.0);
        let result = a * b;
        assert_eq!(result.gradients.len(), 2);
        assert_eq!(result.gradients[0].value_wrt, 3.0);
        assert_eq!(result.gradients[1].value_wrt, 4.0);
        assert_eq!(result.gradients[0].gradient_wrt_value, 4.0);
        assert_eq!(result.gradients[1].gradient_wrt_value, 3.0);
    }

    #[test]
    fn test_fwd_ad_div() {
        let a = Variable::new_no_gradients(3.0);
        let b = Variable::new_no_gradients(4.0);
        let result = a / b;
        assert_eq!(result.gradients.len(), 2);
        assert_eq!(result.gradients[0].value_wrt, 3.0);
        assert_eq!(result.gradients[1].value_wrt, 4.0);
        assert_eq!(result.gradients[0].gradient_wrt_value, 1.0/16.0);
        assert_eq!(result.gradients[1].gradient_wrt_value, -3.0/16.0);
    }
}