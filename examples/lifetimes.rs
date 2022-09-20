//a Lifetimes
type BoxOpDynFn<'a> = Box<dyn for <'call> Fn(&'call usize) -> usize + 'a>;

struct RefInt <'a> (&'a usize);
impl <'a> RefInt<'a> {
    fn iter_usize<'iter> (
        &'iter self,
    ) -> IterRefInt<'a, 'iter>
    {
        IterRefInt { max:self, n:0 }
    }

    fn iter_map<'iter> (
        &'iter self,
        _fns: &'iter [BoxOpDynFn<'iter>]
    ) -> IterRefInt<'a, 'iter>
    {
        IterRefInt { max:self, n:0 }
    }
}

struct IterRefInt<'a, 'iter> { max:&'iter RefInt<'a>, n:usize }
impl <'a, 'iter> Iterator for IterRefInt<'a, 'iter> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n < *self.max.0 {
            let r = self.n;
            self.n += 1;
            Some(r)
        } else {
            None
        }
    }
}

fn double(x:&usize) -> usize {  2 * *x }
fn inc(x:&usize) -> usize { *x + 1 }

fn main() {
    let dbl_plus_one = [
        Box::new(double) as BoxOpDynFn,
    ];
    
    let box_3 = Box::new(3);
    let r = RefInt(&box_3);
    assert_eq!(r.iter_usize().collect::<Vec<_>>(), [0,1,2]);

    let r = RefInt(&box_3);
    r.iter_map(&dbl_plus_one);
}
