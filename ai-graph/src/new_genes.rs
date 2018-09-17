use crate::Gene;
use crate::MutationLine;
use crate::MutationNode;
use rand::Rng;

impl Gene {
    pub fn new_gene() -> Self {
        Self {
            line_dna: vec![vec![vec![MutationLine::Reset; 9]; 9]; 2],
            node_dna: vec![vec![MutationNode::Add; 9]; 2],
        }
    }
    pub fn new_random_gene() -> Self {
        Self {
            line_dna: MutationLine::rand_vec3(2, 9, 9),
            node_dna: MutationNode::rand_vec2(2, 9),
        }
    }
    pub fn new_random_basic_gene(depth: u8, hight: u8) -> Self {
        Self {
            line_dna: MutationLine::rand_vec3(depth, hight, hight),
            node_dna: MutationNode::rand_vec2(depth, hight),
        }
    }
}

trait RandVec
where
    Self: Sized,
{
    fn rand_mut() -> Self;
    fn rand_vec3(num: u8, num2: u8, num3: u8) -> Vec<Vec<Vec<Self>>> {
        let mut rand_vec = Vec::new();
        for _x in 0..num {
            rand_vec.push(Self::rand_vec2(num2, num3));
        }
        rand_vec
    }
    fn rand_vec2(num: u8, num2: u8) -> Vec<Vec<Self>> {
        let mut rand_vec = Vec::new();
        for _x in 0..num {
            rand_vec.push(Self::rand_vec(num2));
        }
        rand_vec
    }
    fn rand_vec(num: u8) -> Vec<Self> {
        let mut rand_vec = Vec::new();
        for _x in 0..num {
            rand_vec.push(Self::rand_mut());
        }
        rand_vec
    }
}

impl RandVec for MutationNode {
    fn rand_mut() -> Self {
        let mut rng = rand::thread_rng();
        let node_types = [
            MutationNode::Multiply,
            MutationNode::Divide,
            MutationNode::Add,
        ];
        *rng.choose(&node_types).unwrap()
    }
}

impl RandVec for MutationLine {
    fn rand_mut() -> Self {
        let mut rng = rand::thread_rng();
        let line_types = [
            MutationLine::Pass,
            MutationLine::Reset,
            MutationLine::Multiply(rand_i8()),
            MutationLine::Divide(rand_i8()),
            MutationLine::Add(rand_i8()),
        ];
        *rng.choose(&line_types).unwrap()
    }
}

fn rand_i8() -> i8 {
    rand::thread_rng().gen_range(i8::min_value(), i8::max_value())
}
