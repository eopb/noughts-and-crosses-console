use crate::Gene;
use rayon::prelude::*;

//TODO Not all checks are finnished.
impl Gene {
    pub fn validate(&self) -> bool {
        // Paralell and.
        let predicates: [&(Fn() -> bool + Sync); 3] = [
            &|| self.sum_lines_per_row_equal(),
            &|| self.all_output_avalible(),
            &|| self.number_of_lines_same_as_number_of_next_nodes(),
        ];
        predicates.par_iter().all(|f| f())
    }
    pub fn validate_two(&self, second_gene: &Self) -> bool {
        self.validate() && second_gene.validate() && self.equal_size(second_gene)
    }

    fn sum_lines_per_row_equal(&self) -> bool {
        for line_block in &self.line_dna {
            let number = line_block[0].len();
            for line_set in line_block {
                if !line_set.len() == number {
                    return false;
                }
            }
        }
        true
    }

    fn all_output_avalible(&self) -> bool {
        true
    }

    fn number_of_lines_same_as_number_of_next_nodes(&self) -> bool {
        // only need to check one line set because other check (sum_lines_per_row_equal) makes sure they are all the same.
        self.line_dna
            .par_iter()
            .enumerate()
            .any(|(block_index, line_block)| {
                (line_block[0].len() == self.node_dna[block_index].len())
            })
    }
    fn equal_size(&self, _second_gene: &Self) -> bool {
        true
    }
}
