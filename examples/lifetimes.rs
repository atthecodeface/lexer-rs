//a Lifetimes
type BoxOpDynFn<'a> = Box<dyn for <'call> Fn(&'call usize) -> usize + 'a>;
type BoxOpDynFnS<'a, S> = Box<dyn for <'call> Fn(&'call S) -> usize + 'a>;

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

    fn iter_map_s<'iter> (
        &'iter self,
        _fns: &'iter [BoxOpDynFnS<'iter, Self>],
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
fn double_r(r:&RefInt) -> usize {  2 * *(r.0) }

fn main() {
    let dbl_usize = [
        Box::new(double) as BoxOpDynFn,
    ];
    let dbl_ref = [
        Box::new(double_r) as BoxOpDynFnS<RefInt>,
    ];
    
    let box_3 = Box::new(3);
    let r = RefInt(&box_3);
    assert_eq!(r.iter_usize().collect::<Vec<_>>(), [0,1,2]);

    let r = RefInt(&box_3);
    r.iter_map(&dbl_usize);

    let r = RefInt(&box_3);
    r.iter_map_s(&dbl_ref);
}
