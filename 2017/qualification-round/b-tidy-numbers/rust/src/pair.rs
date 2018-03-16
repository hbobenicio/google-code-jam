// Attempt to create an Pair Iterator (x, y)
// Like: vec![1,2,3].iter().pair(|(x, y)| )

#[derive(Clone)]
pub struct Pair<I, F> {
    iter: I,
    f: F
}

impl<B, I: Iterator, F> Iterator for Pair<I, F>
    where F: FnMut((&I::Item, &I::Item)) -> B
{
    type Item = (I::Item, I::Item);

    #[inline]
    fn next(&mut self) -> Option<(I::Item, I::Item)> {
        None
    }
}


pub fn iter_pair<T, F>(v: &Vec<T>, f: F) -> ()
{

}
