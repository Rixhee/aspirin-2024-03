#[derive(Debug, PartialEq)]
pub enum MatrixError {
    EmptyVector,
    DimensionMismatch,
    InvalidShape,
}

#[allow(dead_code)]
fn dot_product_prescriptive(vec1: &[f64], vec2: &[f64]) -> Result<f64, MatrixError> {
    let mut sum: f64 = 0.0;
    if vec1.is_empty() || vec2.is_empty() {
        return Err(MatrixError::EmptyVector);
    } else if vec1.len() != vec2.len() {
        return Err(MatrixError::DimensionMismatch);
    } else {
        for i in 0..vec1.len() {
            sum += vec1[i] * vec2[i];
        }
    }

    Ok(sum)
}

#[allow(dead_code)]
fn dot_product_functional(vec1: &[f64], vec2: &[f64]) -> Result<f64, MatrixError> {
    if vec1.is_empty() || vec2.is_empty() {
        Err(MatrixError::EmptyVector)
    } else if vec1.len() != vec2.len() {
        Err(MatrixError::DimensionMismatch)
    } else {
        vec1.iter()
            .zip(vec2)
            .try_fold(0.0, |acc, (x, y)| Ok(acc + x * y))
    }
}

#[allow(dead_code)]
fn multiply_matrices(vec1: &[Vec<f64>], vec2: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, MatrixError> {
    if vec1.is_empty() || vec2.is_empty() {
        return Err(MatrixError::EmptyVector);
    } else if vec1.iter().any(|row| row.len() != vec1[0].len())
        || vec2.iter().any(|row| row.len() != vec2[0].len())
    {
        return Err(MatrixError::InvalidShape);
    } else if vec1[0].len() != vec2.len() {
        return Err(MatrixError::DimensionMismatch);
    }
    let mut output = vec![vec![0.0; vec2[0].len()]; vec1.len()];

    for row in 0..vec1.len() {
        for column in 0..vec2[0].len() {
            for (element, _) in vec2.iter().enumerate() {
                output[row][column] += vec1[row][element] * vec2[element][column]
            }
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_product_prescriptive() {
        let empty_vec = Vec::new();
        assert_eq!(
            dot_product_prescriptive(&empty_vec, &empty_vec),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            dot_product_prescriptive(&vec![0.0, 1.0, 2.0, 3.0], &empty_vec),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            dot_product_prescriptive(&empty_vec, &vec![4.0, 3.0]),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            dot_product_prescriptive(&vec![0.0, 1.0, 2.0], &vec![3.0, 4.0]),
            Err(MatrixError::DimensionMismatch)
        );
        assert_eq!(
            dot_product_prescriptive(&vec![0.0, 1.0], &vec![2.0, 3.0, 4.0]),
            Err(MatrixError::DimensionMismatch)
        );
        assert_eq!(dot_product_prescriptive(&vec![0.0], &vec![0.0]), Ok(0.0));
        assert_eq!(
            dot_product_prescriptive(&vec![1.0, 0.0], &vec![0.0, 1.0]),
            Ok(0.0)
        );
        assert_eq!(
            dot_product_prescriptive(
                &vec![0.0, 1.0, 2.0, 3.0, 4.0],
                &vec![0.0, 1.0, 2.0, 3.0, 4.0]
            ),
            Ok(30.0)
        );
        assert_eq!(
            dot_product_prescriptive(&vec![2.0, 3.0, -1.0], &vec![-1.0, 0.0, 4.0]),
            Ok(-6.0)
        );
    }
    #[test]
    fn test_dot_product_functional() {
        let empty_vec = Vec::new();
        assert_eq!(
            dot_product_functional(&empty_vec.clone(), &empty_vec.clone()),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            dot_product_functional(&vec![0.0, 1.0, 2.0, 3.0], &empty_vec.clone()),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            dot_product_functional(&empty_vec.clone(), &vec![4.0, 3.0]),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            dot_product_functional(&vec![0.0, 1.0, 2.0], &vec![3.0, 4.0]),
            Err(MatrixError::DimensionMismatch)
        );
        assert_eq!(
            dot_product_functional(&vec![0.0, 1.0], &vec![2.0, 3.0, 4.0]),
            Err(MatrixError::DimensionMismatch)
        );
        assert_eq!(dot_product_functional(&vec![0.0], &vec![0.0]), Ok(0.0));
        assert_eq!(
            dot_product_functional(&vec![1.0, 0.0], &vec![0.0, 1.0]),
            Ok(0.0)
        );
        assert_eq!(
            dot_product_functional(
                &vec![0.0, 1.0, 2.0, 3.0, 4.0],
                &vec![0.0, 1.0, 2.0, 3.0, 4.0]
            ),
            Ok(30.0)
        );
        assert_eq!(
            dot_product_functional(&vec![2.0, 3.0, -1.0], &vec![-1.0, 0.0, 4.0]),
            Ok(-6.0)
        );
    }

    #[test]
    fn test_multiply_matrices() {
        let empty_vec: Vec<Vec<f64>> = Vec::new();
        let two_by_two_identity = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let three_by_three_identity = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ];
        assert_eq!(
            multiply_matrices(&empty_vec, &empty_vec),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            multiply_matrices(&empty_vec, &two_by_two_identity),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            multiply_matrices(&three_by_three_identity, &empty_vec),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            multiply_matrices(&three_by_three_identity, &two_by_two_identity),
            Err(MatrixError::DimensionMismatch)
        );
        assert_eq!(
            multiply_matrices(
                &vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0],],
                &vec![vec![1.0, 2.0], vec![3.0, 4.0]]
            ),
            Err(MatrixError::DimensionMismatch)
        );
        assert_eq!(
            multiply_matrices(
                &vec![vec![1.0, 2.0, 3.0, 4.0], vec![4.0, 5.0, 6.0],],
                &vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]]
            ),
            Err(MatrixError::InvalidShape)
        );
        assert_eq!(
            multiply_matrices(
                &vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0],],
                &vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]]
            ),
            Ok(vec![vec![22.0, 28.0], vec![49.0, 64.0]])
        );
        assert_eq!(
            multiply_matrices(
                &vec![
                    vec![1.0, 2.0, 3.0],
                    vec![0.0, 0.0, 0.0],
                    vec![10.0, 20.0, 30.0],
                    vec![0.0, 0.0, 0.0],
                    vec![-1.0, -1.0, -1.0]
                ],
                &vec![vec![1.0, 2.0], vec![1.0, 4.0], vec![1.0, 6.0]]
            ),
            Ok(vec![
                vec![6.0, 28.0],
                vec![0.0, 0.0],
                vec![60.0, 280.0],
                vec![0.0, 0.0],
                vec![-3.0, -12.0]
            ])
        );
        assert_eq!(
            multiply_matrices(&three_by_three_identity, &three_by_three_identity),
            Ok(three_by_three_identity)
        );
    }
}
