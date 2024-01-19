
use std::io::stdin;

use triton_grow::{network::{network::Network, activations::Activations, layer::{layers::LayerTypes}, input::*, matrix::Matrix, matrix3d::Matrix3D}, helper::{mnist::MnistEntry, categorical::to_categorical}};

#[tokio::main]
async fn main() {
    //Convolutional Example
    /*
    let mut inputs: Vec<&dyn Input> = vec![];
    let mut outputs: Vec<Vec<f32>>;
    let mut true_outputs: Vec<Vec<f32>> = vec![];

    let inputs_undyn: Vec<Matrix>;
    let outputs_uncat: Vec<usize>;

    println!("Generating MNIST....");
    (inputs_undyn, outputs_uncat) = MnistEntry::generate_mnist();
    println!("Done Generating MNIST");

    outputs = to_categorical(outputs_uncat);
    for i in 0..inputs_undyn.len(){
        inputs.push(&inputs_undyn[i]);
        true_outputs.push(outputs[i].clone());
    }

    let mut network = Network::new(128);

    network.add_layer(LayerTypes::DENSE(784, Activations::RELU, 0.01));
    network.add_layer(LayerTypes::DENSE(64, Activations::RELU, 0.01));
    network.add_layer(LayerTypes::DENSE(32, Activations::RELU, 0.01));
    network.add_layer(LayerTypes::DENSE(10, Activations::SOFTMAX, 0.01));

    network.compile();

    network.fit_minibatch(&inputs, &true_outputs, 10).await;
    for i in 0..10{
        println!("predicted: {:?} \n\n actual: {:?}\n", network.predict(inputs[i]), true_outputs[i]);
    }*/
    //Dense Example
    //
    let mut inputs: Vec<&dyn Input> = vec![];
    let input_1 = vec![1.0,1.0];
    let input_2 = vec![vec![0.0], vec![1.0]];
    let input_3 = Matrix::from(vec![vec![1.0],vec![0.0]]);
    let input_4 = Matrix3D::from(vec![vec![vec![0.0,0.0]]]);
    inputs.push(&input_1);
    inputs.push(&input_2);
    inputs.push(&input_3);
    inputs.push(&input_4);
    
    let outputs: Vec<Vec<f32>> = vec![vec![0.0],vec![1.0],vec![1.0], vec![0.0]];

    let mut new_net = Network::new(4);


    new_net.add_layer(LayerTypes::DENSE(2, Activations::RELU, 0.01));
    new_net.add_layer(LayerTypes::DENSE(3, Activations::RELU, 0.01));
    new_net.add_layer(LayerTypes::DENSE(1, Activations::SOFTMAX, 0.01));

    /*
     * User Input Seed
    let mut buffer = String::new();
    stdin().read_line(&mut buffer);

    println!("{}", buffer);

    new_net.set_seed(&buffer);*/

    //new_net.set_seed("meat");
    new_net.compile();

    new_net.fit_minibatch(&inputs, &outputs, 100).await;

    println!("1 and 0: {:?}", new_net.predict(&vec![1.0,0.0])[0]);
    println!("0 and 1: {:?}", new_net.predict(&vec![0.0,1.0])[0]);
    println!("1 and 1: {:?}", new_net.predict(&vec![1.0,1.0])[0]);
    println!("0 and 0: {:?}", new_net.predict(&vec![0.0,0.0])[0]);
}
