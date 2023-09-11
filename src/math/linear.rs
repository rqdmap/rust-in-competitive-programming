use super::traits::*;

#[derive(Debug, Clone)]
pub struct Matrix<T> where 
    T: Clone + Copy,
{
    data: Vec<Vec<T>>,
    rows: usize,
    cols: usize,
}

impl<T> Matrix<T> where
    T: Clone + Copy,
{
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();
        for i in 1..rows {
            assert_eq!(cols, data[i].len());
        }
        Self { data, rows, cols }
    }

    pub fn get(&self) -> &Vec<Vec<T>> {
        &self.data
    }
}

impl<T> fmt::Display for Matrix<T> where
    T: Clone + fmt::Display + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let rows = self.rows;
        let cols = self.cols;

        for i in 0..rows {
            write!(f, "{}", match i {
                0 => "┌ ",
                i if i == rows - 1 => "└ ",
                _ => "│ "
            })?;
            for j in 0..cols {
                write!(f, "{}", self.data[i][j])?;
                    write!(f, " ")?;
            }
            // TODO: 如何在不同长度下格式化右侧的括号?
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<T> Add<Self> for Matrix<T>
where
    T: Add<Output = T> + Clone + Copy
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = vec![];
        for i in 0..self.rows {
            let mut tmp = vec![];
            for j in 0..self.cols {
                tmp.push(self.data[i][j] + rhs.data[i][j]);
            }
            res.push(tmp)
        }
        Matrix::new(res)
    }
}

impl<T> Mul<Self> for Matrix<T>
where
    T: Add<Output = T> + Mul<Output = T> + Clone + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.cols, rhs.rows);
        let mut res = vec![];
        for i in 0..self.rows {
            let mut tmp = vec![];
            for j in 0..rhs.cols {
                let mut sum = self.data[i][0] * rhs.data[0][j];
                for k in 1..self.cols {
                    sum = sum + self.data[i][k] * rhs.data[k][j];
                }
                tmp.push(sum);
            }
            res.push(tmp);
        }
        Matrix::new(res)
    }
}

impl<T> Rem<u64> for Matrix<T> where
    T: Copy + Clone + Rem<u64, Output = T>
{
    type Output = Self;

    fn rem(self, rhs: u64) -> Self::Output {
        let mut now = self.clone();
        for i in 0..self.rows {
            for j in 0..self.cols {
                now.data[i][j] = now.data[i][j] % rhs;
            }
        }
        now
    }
}

impl<T: From<u8>> One for Matrix<T> where
    T: Clone + Copy,
{
    fn one(&self) -> Self {
        let mut res = vec![];
        let n = self.cols.min(self.rows);
        for _ in 0..n {
            let mut tmp = vec![];
            for _ in 0..n { tmp.push(T::from(0)); }
            res.push(tmp);
        }
        for i in 0..n { res[i][i] = T::from(1); }
        Matrix::new(res)
    }
}

