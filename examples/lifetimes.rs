//a Lifetimes
type BoxOpDynFnS<'a, S> = Box<dyn for<'call> Fn(&'call S) -> usize + 'a>;

#[derive(Debug)]
struct RefInt<'a>(&'a usize);
impl<'a> RefInt<'a> {
    fn iter_usize<'iter>(&'iter self) -> IterRefInt<'a, 'iter> {
        IterRefInt { max: self, n: 0 }
    }

    // Choices:
    //
    // If this is invoked then Self is borrowed for static
    // and there are lifetime errors on usage:
    //
    // fn iter_map_self<'iter> (
    // &'iter self,
    // _fns: &'iter [BoxOpDynFnS<'iter, Self>],
    //
    // Same for this (functions are borrowed for 'static):
    //
    // fn iter_map_s<'iter> (
    // &'iter self,
    // _fns: &'iter [BoxOpDynFnS<'iter, RefInt<'iter>>],
    //
    // Same for this (self is borrowed for 'static):
    //
    // fn iter_map_s<'iter> (
    // &'iter self,
    // _fns: &'iter [BoxOpDynFnS<'iter, RefInt<'a>>],

    // If this is invoked then any S is borrowed for the lifetime of iter at most
    // fn iter_map_s<'iter, S> (
    // &'iter self,
    // _fns: &'iter [BoxOpDynFnS<'iter, S>],

    fn iter_map_s<'iter, S>(
        &'iter self,
        _fns: &'iter [BoxOpDynFnS<'iter, S>],
    ) -> IterRefInt<'a, 'iter> {
        IterRefInt { max: self, n: 0 }
    }
}

struct IterRefInt<'a, 'iter> {
    max: &'iter RefInt<'a>,
    n: usize,
}
impl<'a, 'iter> Iterator for IterRefInt<'a, 'iter> {
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

fn double_r(r: &RefInt) -> usize {
    2 * *(r.0)
}

fn main() {
    // let dbl_usize = [Box::new(double) as BoxOpDynFnS<usize>];
    let dbl_ref = [Box::new(double_r) as BoxOpDynFnS<RefInt>];

    let box_3 = Box::new(3);
    let r = RefInt(&box_3);
    assert_eq!(r.iter_usize().collect::<Vec<_>>(), [0, 1, 2]);

    //    let r = RefInt(&box_3);
    //    r.iter_map_s(&dbl_usize);

    let r = RefInt(&box_3);
    r.iter_map_s(&dbl_ref);

    let r = RefInt(&box_3);
    r.iter_map_s(&dbl_ref);

    drop(dbl_ref);
    dbg!(&r);

    //     let r = RefInt(&box_3);
    //     r.iter_map_s(&dbl_usize);
}
