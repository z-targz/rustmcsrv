use std::cmp;

use std::num::NonZeroUsize;




pub trait CanTakeForced {
    fn take_forced(self, n: usize) -> TakeForced<Self> 
        where Self: Sized;
}

impl<I: Iterator> CanTakeForced for I {
    fn take_forced(self, n: usize) -> TakeForced<Self> 
        where Self: Sized + Iterator
    {
        TakeForced::new(self, n)
    }
}

#[derive(Clone, Debug)]
pub struct TakeForced<I> {
    iter: I,
    n: usize,
}

impl<I> TakeForced<I> {
    pub fn new(iter: I, n: usize) -> TakeForced<I> {
        TakeForced { iter, n }
    }
}

impl<I> Iterator for TakeForced<I>
where
    I: Iterator,
{
    type Item = <I as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        if self.n != 0 {
            self.n -= 1;
            self.iter.next()
        } else {
            None
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<I::Item> {
        if self.n > n {
            self.n -= n + 1;
            self.iter.nth(n)
        } else {
            if self.n > 0 {
                self.iter.nth(self.n - 1);
                self.n = 0;
            }
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.n == 0 {
            return (0, Some(0));
        }

        let (lower, upper) = self.iter.size_hint();

        let lower = cmp::min(lower, self.n);

        let upper = match upper {
            Some(x) if x < self.n => Some(x),
            _ => Some(self.n),
        };

        (lower, upper)
    }



}
trait SpecAdvanceBy {
    fn advance_by(&mut self, n: usize) -> Result<(), NonZeroUsize>;
}

impl<I> SpecAdvanceBy for TakeForced<I> 
where 
    I: Iterator,
{
    #[inline]
    fn advance_by(&mut self, n: usize) -> Result<(), NonZeroUsize> {

        let min = self.n.min(n);
        
        let v: Result<(), NonZeroUsize> = {
            for i in 0..n {
                if self.next().is_none() {
                    // SAFETY: `i` is always less than `n`.
                    return Err(unsafe { NonZeroUsize::new_unchecked(n - i) });
                }
            }
            Ok(()) 
        };
        let rem = match v
        {
            Ok(()) => 0,
            Err(rem) => rem.get(),
        };

        let advanced = min - rem;
        self.n -= advanced;
        NonZeroUsize::new(n - advanced).map_or(Ok(()), Err)
    
    }
}

trait AdvanceBy {
    fn advance_by(&mut self, n: usize) -> Result<(), NonZeroUsize>;
}