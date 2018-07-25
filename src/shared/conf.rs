
/// These sizes determine the maximum number of tables, columns and rows,
/// but also have a big impact on memory and disk usage.
/// Changing any of these is a backward-incompatible, breaking change.

/// Table pointer size (>=1)
//noinspection RsTypeAliasNaming
pub type uTab = u16;
/// Column pointer size within table (>=1)
//noinspection RsTypeAliasNaming
pub type uCol = u16;
/// Row pointer size within table (>=1)
//noinspection RsTypeAliasNaming
pub type uRow = u32;
/// Global string interning pool pointer size (>=1)
//noinspection RsTypeAliasNaming
pub type uStr = u32;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexing() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let index: uTab = 7;
        assert_eq!(8, data[index as usize]);
        let index: uCol = 7;
        assert_eq!(8, data[index as usize]);
        let index: uRow = 7;
        assert_eq!(8, data[index as usize]);
        let index: uStr = 7;
        assert_eq!(8, data[index as usize]);
    }
}
