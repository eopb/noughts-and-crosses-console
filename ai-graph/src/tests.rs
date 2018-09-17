#[cfg(test)]
use crate::Gene;
#[test]
fn random_gene_is_valid() {
    let test_gene = Gene::new_random_gene();
    if !test_gene.validate() {
        panic!("Gene is not valid")
    };
}

// To get output run cargo test tests::random_breed_is_valid -- --nocapture
#[test]
fn random_breed_is_valid() {
    let mut test_gene = Gene::new_random_gene();
    println!("test_gene{:#?}", test_gene);
    let test_gene2 = Gene::new_random_gene();
    println!("test_gene2{:#?}", test_gene2);
    test_gene = test_gene.breed(&test_gene2);
    println!("test_gene2 mut{:#?}", test_gene);
    if !test_gene.validate() {
        panic!("Gene is not valid")
    };
}
