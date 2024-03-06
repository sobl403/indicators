#[cfg(test)]
mod test_statistics {
    use super::super::statistics::*;

    #[test]
    fn test_simple_moving_average() {
        let data_set = vec![5.0, 6.0, 4.0, 2.0];

        let result = simple_moving_average(&data_set, 2).unwrap();
        assert_eq!(3, result.len());
        assert_eq!(vec![5.5, 5.0, 3.0], result);

        let result = simple_moving_average(&data_set, 3).unwrap();
        assert_eq!(2, result.len());
        assert_eq!(vec![5.0, 4.0], result);

        let result = simple_moving_average(&data_set, 4).unwrap();
        assert_eq!(1, result.len());
        assert_eq!(vec![4.25], result);

        let result = simple_moving_average(&data_set, 5);
        assert_eq!(None, result);
    }
    #[test]
    fn test_rsi() {
        let data_set = vec![
            5.0, 6.0, 4.0, 2.0, 1.5, 1.0, 2.0, 3.0, 3.5, 3.5, 4.0, 4.5, 5.0,
        ];

        let result = rsi(&data_set, 14);
        assert_eq!(result, None);

        let result = rsi(&data_set, 8).unwrap();
        assert_eq!(5, result.len());
        assert_eq!(
            vec![
                56.852791878172596,
                56.852791878172596,
                59.17295654731064,
                61.256328819550575,
                63.16578540011347
            ],
            result
        );
    }
}
