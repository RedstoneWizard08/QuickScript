pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub fn left(self) -> L {
        self.try_left().unwrap()
    }

    pub fn right(self) -> R {
        self.try_right().unwrap()
    }

    pub fn try_left(self) -> Option<L> {
        match self {
            Self::Left(val) => Some(val),
            _ => None,
        }
    }

    pub fn try_right(self) -> Option<R> {
        match self {
            Self::Right(val) => Some(val),
            _ => None,
        }
    }
}

impl<L: Clone, R: Clone> Clone for Either<L, R> {
    fn clone(&self) -> Self {
        match self {
            Self::Left(val) => Self::Left(val.clone()),
            Self::Right(val) => Self::Right(val.clone()),
        }
    }
}

impl<L: Copy, R: Copy> Copy for Either<L, R> {}

unsafe impl<L: Send, R: Send> Send for Either<L, R> {}
unsafe impl<L: Sync, R: Sync> Sync for Either<L, R> {}
