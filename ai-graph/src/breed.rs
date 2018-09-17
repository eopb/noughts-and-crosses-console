use crate::Gene;
use crate::MutationLine;
use crate::MutationNode;
use rand::Rng;

//TODO check gene types are the same so that only breedable values are passed.
impl Gene {
    /// This function merges two genes together to find an avarage genes. Lines and Nodes that can't be merged to an avarage are randomly selected.
    pub fn breed(&self, second_gene: &Self) -> Self {
        let mut new_values = self.clone();
        if !self.validate_two(second_gene) {
            panic!("Genes not compatible. Can't breed.");
        };
        // Merge nodes.
        for (node_line_index, node_line) in self.node_dna.iter().enumerate() {
            for (node_index, node) in node_line.iter().enumerate() {
                new_values.node_dna[node_line_index][node_index] =
                    node.node_merge(second_gene.node_dna[node_line_index][node_index]);
            }
        }
        // Merge lines.
        for (block_index, block) in self.line_dna.iter().enumerate() {
            for (node_point_index, node_point) in block.iter().enumerate() {
                for (line_index, line) in node_point.iter().enumerate() {
                    new_values.line_dna[block_index][node_point_index][line_index] = line
                        .line_merge(second_gene.line_dna[block_index][node_point_index][line_index])
                }
            }
        }
        new_values.clone()
    }
}

impl MutationNode {
    fn node_merge(self, second_node: Self) -> Self {
        let mut rng = rand::thread_rng();
        let node_types = [self, second_node];
        *rng.choose(&node_types).unwrap()
    }
}

impl MutationLine {
    fn line_merge(self, second_line: Self) -> Self {
        let rng = rand::thread_rng().gen_range(0, 2);
        let node_types = [self, second_line];
        match node_types[rng] {
            MutationLine::Pass => MutationLine::Pass,
            MutationLine::Reset => MutationLine::Reset,
            MutationLine::Multiply(value) => match node_types[if rng == 0 { 1 } else { 0 }] {
                MutationLine::Pass | MutationLine::Reset => MutationLine::Multiply(value),
                MutationLine::Multiply(value2)
                | MutationLine::Divide(value2)
                | MutationLine::Add(value2) => {
                    MutationLine::Multiply(mean_avg_of_two(value, value2))
                }
            },
            MutationLine::Divide(value) => match node_types[if rng == 0 { 1 } else { 0 }] {
                MutationLine::Pass | MutationLine::Reset => MutationLine::Divide(value),
                MutationLine::Multiply(value2)
                | MutationLine::Divide(value2)
                | MutationLine::Add(value2) => MutationLine::Divide(mean_avg_of_two(value, value2)),
            },
            MutationLine::Add(value) => match node_types[if rng == 0 { 1 } else { 0 }] {
                MutationLine::Pass | MutationLine::Reset => MutationLine::Add(value),
                MutationLine::Multiply(value2)
                | MutationLine::Divide(value2)
                | MutationLine::Add(value2) => MutationLine::Add(mean_avg_of_two(value, value2)),
            },
        }
    }
}
#[allow(clippy::cast_possible_truncation)]
fn mean_avg_of_two(one: i8, two: i8) -> i8 {
    ((i16::from(one) + i16::from(two)) / 2) as i8
}

// fn _mean_avg_of_two(one: i8, two: i8) -> i8 {
//     let max = std::cmp::max(one, two);
//     let min = std::cmp::min(one, two);
//     min + (max - min) / 2
// }
