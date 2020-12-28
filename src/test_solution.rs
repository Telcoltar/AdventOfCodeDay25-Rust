#[cfg(test)]
mod test_solution {
    use crate::solution_part_1;

    #[test]
    fn test_solution_part_1() {
        assert_eq!(14897079, solution_part_1("testData.txt"));
    }
}