/*
    Appellation: default <tests>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
*/
#[cfg(test)]
mod tests {

    #[test]
    fn lib_compiles() {
        let f = |i: usize| i + 1;
        assert_eq!(f(10), 11)
    }
}