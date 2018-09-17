//TODO add ai graph with fixed number of inputs and outputs but costom layers
//TODO More tests that check if the graph is vaild

#![feature(tool_lints)]
#![warn(clippy::pedantic)]
//! Ai Graph is a new tool for creating machine learning that runs blazingly fast when learning has finnished.
mod breed;
mod new_genes;
mod output;
mod tests;
mod validate;
#[derive(Clone, Copy, Debug)]
enum MutationLine {
    Pass,
    Reset,
    Multiply(i8),
    Divide(i8),
    Add(i8),
}
#[derive(Clone, Copy, Debug)]
enum MutationNode {
    Multiply,
    Divide,
    Add,
}

#[derive(Clone, Debug)]
/// Gene stores the "graph". Different graphs will form different output.
pub struct Gene {
    line_dna: Vec<Vec<Vec<MutationLine>>>,
    node_dna: Vec<Vec<MutationNode>>,
}
