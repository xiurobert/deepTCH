# deepTCH
*It's like pytorch, but worse*


## Roadmap (in rough order of priority)
- [ ] Automatic differentiation
    - [ ] For scalars
    - [ ] For Tensors
- [ ] Tensors
- [ ] Layer API
    - [ ] Linear Layers
    - [ ] Convolutional Layers
- [ ] Optimizers
    - [ ] SGD
    - [ ] Adam
- [ ] Losses
    - [ ] MSE
    - [ ] Cross-entropy
- [ ] GPU Support
    - [ ] CUDA support
    - [ ] OpenCL
    - [ ] Metal Performance Shaders (?)
    - [ ] AMD ROCm support

## Usage

### Rust interface

Hopefully, it'll look something like this

(Again, very far off from this right now)

```rust

use deeptch::prelude::*;

fn main() {
    let input_tensor = deeptch::rand!(32, 3, 28, 28);
    let labels = deeptch::rand!(32, 1);
    
    let model = deeptch::Model::sequential!(
        deeptch::layers::conv2d::Conv2D::new(3, 5, 5, 1, 1, 1, 1, 0, 0),
        deeptch::layers::relu::ReLU::new(),
        deeptch::layers::max_pool2d::MaxPool2D::new(2, 2, 2, 2),
        deeptch::layers::conv2d::Conv2D::new(16, 5, 5, 1, 1, 1, 1, 0, 0),
        deeptch::layers::relu::ReLU::new(),
        deeptch::layers::max_pool2d::MaxPool2D::new(2, 2, 2, 2),
        deeptch::layers::flatten::Flatten::new(),
        deeptch::layers::dense::Dense::new(120, 1),
        deeptch::layers::relu::ReLU::new(),
        deeptch::layers::dense::Dense::new(84, 1),
        deeptch::layers::relu::ReLU::new(),
        deeptch::layers::dense::Dense::new(10, 1),
        deeptch::layers::softmax::Softmax::new()
    );
    
    let loss = deeptch::losses::cross_entropy::CrossEntropy::new();
    let optimizer = deeptch::optimizers::sgd::SGD::new(0.01);
    let metrics = deeptch::metrics::accuracy::Accuracy::new();
    
    let mut trainer = deeptch::Trainer::new(model, loss, optimizer, metrics);
    
    trainer.fit(&input_tensor, &labels, 1);
}
```

### Python Interface

This library theoretically should maintain the same interface as pytorch
(aka, a drop in replacement). 
We are still very very far off from that.

```python
import deeptch as torch

class SmolNN(torch.nn.Module):
    def __init__(self):
        super().__init__()
        self.linear = torch.nn.Linear(1, 1)

    def forward(self, x):
        return self.linear(x)
```


Alternatively, for people who prefer keras for some reason

```python
import deeptch as tch

model = tch.Sequential([
    tch.nn.Linear(1, 1)
])

model.compile(optimizer='sgd', loss='mse')
model.fit(x, y, epochs=1) # internally creates a trainer and applies that
```