use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Cursor<T>
where
    T: Clone,
{
    inner: Vec<T>,
    cursor: usize,
}

impl<T> Cursor<T>
where
    T: Clone,
{
    pub fn new(inner: Vec<T>) -> Self {
        Self { inner, cursor: 0 }
    }

    pub fn next(&mut self) -> Option<T> {
        let val = self.inner.get(self.cursor);

        self.cursor += 1;

        val.cloned()
    }

    pub fn prev(&mut self) -> Option<T> {
        self.cursor -= 1;

        self.inner.get(self.cursor).cloned()
    }

    pub fn peek(&mut self, n: usize) -> Option<T> {
        self.inner.get(self.cursor + n).cloned()
    }

    pub fn has_next(&mut self) -> bool {
        self.peek(0).is_some()
    }

    pub fn next_result<S>(&mut self, err: S) -> Result<T>
    where
        S: AsRef<str>,
    {
        self.next().ok_or(anyhow!(err.as_ref().to_string()))
    }
}

impl<T> Cursor<T>
where
    T: std::fmt::Debug + Clone + PartialEq,
{
    pub fn next_is(&mut self, val: T) -> Result<()> {
        let tkn = self.next().ok_or(anyhow!("Could not get next item!"))?;

        if tkn == val {
            return Ok(());
        }

        Err(anyhow!("Expected {:?}, found {:?}!", val, tkn))
    }

    pub fn next_is_peek(&mut self, val: T, n: usize) -> Result<()> {
        let tkn = self
            .peek(n)
            .ok_or(anyhow!("Could not peek at next item!"))?;

        if tkn == val {
            return Ok(());
        }

        Err(anyhow!("Expected {:?}, found {:?}!", val, tkn))
    }

    pub fn read_until(&mut self, end: T) -> Vec<T> {
        let mut res = Vec::new();

        while let Some(item) = self.next() {
            if item == end {
                break;
            }

            res.push(item);
        }

        res
    }

    pub fn read_until_counted(&mut self, open: T, close: T) -> Vec<T> {
        let mut res = Vec::new();
        let mut cnt = 1;

        while let Some(item) = self.next() {
            if item == open {
                cnt += 1;
            }

            if item == close {
                cnt -= 1;
            }

            if cnt == 0 {
                break;
            }

            res.push(item);
        }

        res
    }

    pub fn peek_until(&mut self, end: T) -> Vec<T> {
        let mut res = Vec::new();
        let mut n = 0;

        while let Some(item) = self.peek(n) {
            if item == end {
                break;
            }

            res.push(item);
            n += 1;
        }

        res
    }
}
