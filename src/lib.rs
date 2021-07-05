extern crate pest;
#[macro_use]
extern crate pest_derive;

mod binary_tree;
mod binary_tree_2;
mod chapter_1;
mod chapter_2_1;
mod chapter_2_2;
mod chapter_2_3;
mod cons;
mod languages;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
