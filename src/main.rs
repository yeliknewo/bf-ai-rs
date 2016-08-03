extern crate rand;
extern crate rusty_machine;

use rand::Rng;
use rusty_machine::learning::nnet::{NeuralNet, BCECriterion};
use rusty_machine::learning::toolkit::regularization::Regularization;
use rusty_machine::learning::optim::grad_desc::StochasticGD;
use rusty_machine::linalg::Matrix;
use rusty_machine::learning::SupModel;


mod bf;

pub const VALID_CHARS: [char; 8] = [',', '.', '+', '-', '<', '>', '[', ']'];

fn main() {
    let mut src = String::new();

    let inputs = Matrix::new(1, 3, vec![
            0.1, 0.2, 0.3
        ]);

    let targets = Matrix::new(1, 3, vec![
            0.9, 0.8, 0.7
        ]);

    let layers = &[3, 3];

    let criterion = BCECriterion::new(Regularization::L2(0.1));

    let mut model = NeuralNet::new(layers, criterion, StochasticGD::default());

    for _ in 0..100000 {
        model.train(&inputs, &targets);
    }

    let test_inputs = Matrix::new(1, 3, vec!(0.1, 0.2, 0.3));

    println!("{:?}", model.predict(&test_inputs));

    println!("{:?}", src);

    println!("{:?}", bf::run(src));
}
