use crate::Gene;
use crate::MutationLine;
use crate::MutationNode;

// Temp way of storing values and type of nodes while outputs are being calculated. Gene.node_dna only stores type MutationNodeStorage stores type and any data it contains.
#[derive(Debug, Clone, Copy)]
struct MutationNodeStorage {
    node_type: MutationNode,
    stored_data: Option<f64>,
}
impl Gene {
    /// This function calculats an output using a set of inputs and a gene.
    /// If this function takes hard coded values it can be heavily optimised.
    pub fn output(self, input: &[i32]) -> Vec<f64> {
        let mut output = Vec::new();
        let mut node_values = node_value_calc(&self.node_dna);
        if !self.validate() {
            panic!("Gene is not valid")
        };
        for (node_index, node_tree) in self.line_dna[0].iter().enumerate() {
            for (line_index, line) in node_tree.iter().enumerate() {
                node_values[0][line_index].stored_data =
                    match node_values[0][line_index].stored_data {
                        Some(_x) => Some(
                            node_values[0][line_index]
                                .calc_pass_value(line.calc_pass_value(input[node_index].into())),
                        ),
                        None => Some(
                            node_values[0][line_index]
                                .calc_pass_value(line.calc_pass_value(input[node_index].into())),
                        ),
                    };
            }
        }
        for (block_index, block) in self.line_dna.iter().enumerate() {
            if block_index == 0 {
                continue;
            }
            for (node_index, node_tree) in block.iter().enumerate() {
                for (line_index, line) in node_tree.iter().enumerate() {
                    node_values[block_index][line_index].stored_data =
                        match node_values[block_index][line_index].stored_data {
                            Some(_x) => Some(node_values[block_index][line_index].calc_pass_value(
                                line.calc_pass_value(
                                    match node_values[block_index - 1][node_index].stored_data {
                                        Some(x) => x,
                                        None => panic!("Error 1"),
                                    },
                                ),
                            )),
                            None => Some(node_values[block_index][line_index].calc_pass_value(
                                line.calc_pass_value(
                                    match node_values[block_index - 1][node_index].stored_data {
                                        Some(x) => x,
                                        None => panic!("Error 1"),
                                    },
                                ),
                            )),
                        };
                }
            }
        }
        for node_values in node_values[node_values.len() - 1].clone() {
            output.push(match node_values.stored_data {
                Some(x) => x,
                None => panic!("Error 2"),
            })
        }
        output
    }
}

fn node_value_calc(node_dna: &[Vec<MutationNode>]) -> Vec<Vec<MutationNodeStorage>> {
    let mut output = Vec::new();
    for row in node_dna {
        output.push(node_value_calc_row(&row));
    }
    output
}
fn node_value_calc_row(row: &[MutationNode]) -> Vec<MutationNodeStorage> {
    let mut output = Vec::new();
    for node in row {
        output.push(convert_mut_node_to_mut_node_store(*node));
    }
    output
}

fn convert_mut_node_to_mut_node_store(node: MutationNode) -> MutationNodeStorage {
    MutationNodeStorage {
        node_type: node,
        stored_data: None,
    }
}

impl MutationLine {
    fn calc_pass_value(self, input_value: f64) -> f64 {
        match self {
            MutationLine::Multiply(x) => input_value * f64::from(x),
            MutationLine::Add(x) => input_value + f64::from(x),
            MutationLine::Divide(x) => input_value / if x == 0 { 1.0 } else { f64::from(x) },
            MutationLine::Pass => input_value,
            MutationLine::Reset => 0.0,
        }
    }
}

impl MutationNodeStorage {
    fn calc_pass_value(&self, input_value: f64) -> f64 {
        match self.stored_data {
            Some(data) => match self.node_type {
                MutationNode::Add => data + input_value,
                MutationNode::Divide => if data == 0.0 {
                    input_value
                } else if input_value == 0.0 {
                    data
                } else {
                    data / input_value
                },
                MutationNode::Multiply => {
                    // println!("data {}, inputvalue {}", data, input_value);
                    if data == 0.0 {
                        input_value
                    } else if input_value == 0.0 {
                        data
                    } else {
                        data * input_value
                    }
                }
            },
            None => input_value,
        }
    }
}
