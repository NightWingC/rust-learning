use linfa::prelude::*;
use linfa_datasets::iris;
use linfa_trees::DecisionTree;
use std::collections::HashSet;

fn main() {
    //Load the ires dataset
    let dataset = iris();

    // Display basic dataset information
    println!("Number of samples; {}", dataset.nsamples());
    println!("Number of features: {}", dataset.nfeatures());

    let unique_targets: HashSet<_> = dataset.targets().iter().cloned().collect();
    println!("Target names: {:?}", unique_targets);

    // split the datasset into 80% training and 20% testing
    let (train, test) = dataset.split_with_ratio(0.8);
    println!("Training samples: {}", train.nsamples());
    println!("Testing samples: {}", test.nsamples());

    // Train the decision tree model 
    let model = DecisionTree::params().fit(&train).unwrap();

    // Predict using the test data
    let predictions = model.predict(&test);

    // Evaluate model accuracy
    let cm = predictions.confusion_matrix(&test).unwrap();
    let accuracy = cm.accuracy();
    println!("Model accuracy: {:.2}%", accuracy * 100.0);

}
