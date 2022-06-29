struct History {

}

pub trait Variable {
    /// Returns the function calls that created this variable, or none if it is a constant variable
    fn history(&self) -> Option<History>;
    /// Returns the unique name for this variable
    fn name(&self) -> Option<String>;
    /// Returns the derivative of the variable after backpropagation
    fn derivative(&self) -> Option<f64>;
    /// Alias for `derivative`
    fn grad(&self) -> Option<f64>;
    /// Sets the requires_grad flag to `requires_grad` on this variable.
    fn requires_grad_(&mut self, requires_grad: bool);
    /// Adds `d` to the derivative accumulated on this variable.
    /// Only called during backpropagation.
    fn accumulate_derivative(&mut self, d: f64);
    /// Resets the derivative accumulated on this variable.
    fn zero_derivative_(&mut self);
    /// Alias for `zero_derivative_`
    fn zero_grad_(&mut self);
}

struct Context {

}

/// Function that can act on Variable arguments producing a Variable output
trait FunctionBase {
    // fn variable(raw: Box<dyn std::any::Any>, history: History);
    fn forward(&self, ctx: &Context, args: Vec<Box<dyn Variable>>) -> Result<Box<dyn Variable>, String>;
    // fn chain_rule(fun: Self::Func, context: &mut Context, inputs: Vec<dyn Variable>, d_output: f64) -> Result<Vec<dyn Variable>, String>;
}

/// Applies the function to the given arguments.
fn apply_function(fun: Box<dyn FunctionBase>, args: Vec<Box<dyn Variable>>) -> Result<Vec<Box<dyn Variable>>, String> {
    let mut raw_vals = Vec::new();
    for v in args {
        if v.history().is_some() {
            let need_grad = true;
        }
        // v.used += 1;
        raw_vals.push(v);
    }
    Ok(raw_vals)

}