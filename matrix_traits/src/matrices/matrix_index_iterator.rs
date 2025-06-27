pub struct MatrixIndexIterator {
    nrows    : usize,
    ncols    : usize,
    curr_row : usize,
    curr_col : usize
}

impl MatrixIndexIterator {
    pub fn new(nrows: usize, ncols: usize) -> Self {
        let curr_row=if ncols == 0 {
            nrows
        } else {
            0
        };
        MatrixIndexIterator { nrows, ncols, curr_row , curr_col: 0}
    }
}

impl Iterator for MatrixIndexIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_row >= self.nrows {
            return None;
        }
        let rv=(self.curr_row, self.curr_col);
        // prepare for next
        if self.curr_col < self.ncols-1 {
            self.curr_col+=1;
        } else { // self.curr_col == self.ncols-1
            self.curr_row+=1;
            self.curr_col=0;
        }
        Some(rv)
    }
}

impl ExactSizeIterator for MatrixIndexIterator {
    fn len(&self) -> usize {
        self.ncols*(self.nrows - self.curr_row) - self.curr_col
    }
}