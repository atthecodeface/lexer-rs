//a Imports

//a Macros
//mi one_f_one_r - for e.g. first_of_n( [ ] )
// Macro to allow multiple functions with the same return type in a slice
//
// Produces:
//   *  <fn>_dyn_ref([&dyn Fn() -> ParseFnResult<R>]) -> impl ParserFn<P, R>
//   *  <fn>_dyn_ref_else([&dyn Fn() -> ParseFnResult<R>], Fn()-> Error) -> impl ParserFn<P, R>
//
macro_rules! one_f_one_r_slice {
    ( $fn_name:ident,
      $fs:ident,
      $stream : ident
      { $($content:tt)* }
    ) => {

        paste::paste! {

pub fn [<$fn_name _dyn_ref>] <'b, P, I: ParserInputStream<P>, R, const N : usize>(
    $fs: [ &'b (dyn Fn(I) -> ParseFnResult<P, R> +'b) ; N]
    ) -> impl Fn(I) -> ParseFnResult<P, R> + 'b
    where
        P: ParserInput<Stream = I>,
{
    move |$stream| { $($content)* }
} // pub fn

pub fn [<$fn_name _dyn_ref_else>] <'b, P, I: ParserInputStream<P>, R, G, const N : usize>(
    $fs: [ &'b (dyn Fn(I) -> ParseFnResult<P, R> +'b) ; N],
    g : G,
    ) -> impl Fn(I) -> ParseFnResult<P, R> + 'b
    where
        P: ParserInput<Stream = I>,
        G : Fn() -> <P as ParserInput>::Error + 'b,
{
    move |$stream| {
        match ( { $($content)* } )? {
            ParseResult::Mismatched => {
                Err(g())
            }
            x => Ok(x),
        }
    }
} // pub fn

        } // paste
    }} // macro_rules

//mi many_f_one_r - e.g. for first_of_2/3/4
// Macro to allow multiple functions with the same return type individually
//
// Produces:
//   *  <fn>(f1:F1, f2:F2, ...) -> impl ParserFn<P, R>
//   *  <fn>_else(f1:F1, f2:F2, ..., Fn()-> Error) -> impl ParserFn<P, R>
//   *  <fn>_ref(f1:&F1, f2:&F2, ...) -> impl ParserFn<P, R>
//   *  <fn>_ref_else(f1:&F1, f2:&F2, ..., Fn()-> Error) -> impl ParserFn<P, R>
//   *  <fn>_dyn_ref(f1:&dyn F1, f2:&dyn F2, ...) -> impl ParserFn<P, R>
//   *  <fn>_dyn_ref_else(f1:&dyn F1, f2:&dyn F2, ..., Fn()-> Error) -> impl ParserFn<P, R>
//
macro_rules! many_f_one_r {
    ( $fn_name:ident,
      ( $($f:ident : $ft:ident  , )+  $(,)? )
      $stream : ident
      { $($content:tt)* }
    ) => {

        paste::paste! {

pub fn $fn_name<P, I: ParserInputStream<P>, R, $($ft, )*>(
    $( $f : $ft , )*
    ) -> impl Fn(I) -> ParseFnResult<P, R>
    where
        P: ParserInput<Stream = I>,
        $( $ft: Fn(I) -> ParseFnResult<P, R>, )*
{
    move |$stream| { $($content)* }
} // pub fn

pub fn [< $fn_name _else >] <P, I: ParserInputStream<P>, R, $($ft, )* G>(
    $( $f : $ft , )*
    g : G,
    ) -> impl Fn(I) -> ParseFnResult<P, R>
    where
        P: ParserInput<Stream = I>,
        G : Fn() -> <P as ParserInput>::Error,
        $( $ft: Fn(I) -> ParseFnResult<P, R>, )*
{
    move |$stream|
        match ( { $($content)* } )? {
            ParseResult::Mismatched => {
                Err(g())
            }
            x => Ok(x),
        }
} // pub fn

pub fn [< $fn_name _ref>] <'b, P, I: ParserInputStream<P>, R, $($ft, )*>(
    $( $f : &'b $ft , )*
    ) -> impl Fn(I) -> ParseFnResult<P, R> + 'b
            where
                P: ParserInput<Stream = I>,
            $( $ft: Fn(I) -> ParseFnResult<P, R> +'b, )*
{
    move |$stream| { $($content)* }
} // pub fn

pub fn [< $fn_name _ref_else>] <'b, P, I: ParserInputStream<P>, R, $($ft, )* G>(
    $( $f : &'b $ft , )*
    g : G,
    ) -> impl Fn(I) -> ParseFnResult<P, R> + 'b
    where
        P: ParserInput<Stream = I>,
        G : Fn() -> <P as ParserInput>::Error + 'b,
        $( $ft: Fn(I) -> ParseFnResult<P, R> +'b, )*
{
    move |$stream|
        match ( { $($content)* } )? {
            ParseResult::Mismatched => {
                Err(g())
            }
            x => Ok(x),
        }
} // pub fn

pub fn [< $fn_name _dyn_ref>] <'b, P, I: ParserInputStream<P>, R>(
    $( $f : &'b (dyn Fn(I) -> ParseFnResult<P, R> +'b) , )*
    ) -> impl Fn(I) -> ParseFnResult<P, R> + 'b
    where
        P: ParserInput<Stream = I>,
{
    move |$stream| { $($content)* }
} // pub fn

pub fn [< $fn_name _dyn_ref_else>] <'b, P, I: ParserInputStream<P>, R, G>(
    $( $f : &'b (dyn Fn(I) -> ParseFnResult<P, R> +'b) , )*
    g: G,
    ) -> impl Fn(I) -> ParseFnResult<P, R> + 'b
    where
        P: ParserInput<Stream = I>,
        G : Fn() -> <P as ParserInput>::Error + 'b,
{
    move |$stream|
        match ( { $($content)* } )? {
            ParseResult::Mismatched => {
                Err(g())
            }
            x => Ok(x),
        }
} // pub fn

        } // paste
    }} // macro_rules

//mi many_f_many_r - e.g. for pair, tuple3, delimited, etc
// Macro to allow multiple functions with the individual return types
//
// Produces:
//   *  <fn>(f1:F1, f2:F2, ...) -> impl ParserFn<P, R>
//   *  <fn>_else(f1:F1, f2:F2, ..., Fn()-> Error) -> impl ParserFn<P, R>
//   *  <fn>_ref(f1:&F1, f2:&F2, ...) -> impl ParserFn<P, R>
//   *  <fn>_ref_else(f1:&F1, f2:&F2, ..., Fn()-> Error) -> impl ParserFn<P, R>
//   *  <fn>_dyn_ref(f1:&dyn F1, f2:&dyn F2, ...) -> impl ParserFn<P, R>
//   *  <fn>_dyn_ref_else(f1:&dyn F1, f2:&dyn F2, ..., Fn()-> Error) -> impl ParserFn<P, R>
//
macro_rules! many_f_many_r {
    ( $(#[$outer:meta])*
      $fn_name:ident,
      ( $($f:ident : $ft:ident : $rt:ident),+  $(,)? ),
      $r:ty,
      $stream : ident
      { $($content:tt)* }
    ) => {

        paste::paste! {

$(#[$outer])*
///
/// The provided parser functions are consumed into a closure
pub fn $fn_name<P, I: ParserInputStream<P>, $($rt,)* $($ft, )*>(
    $( $f : $ft , )*
    ) -> impl Fn(I) -> ParseFnResult<P, $r >
    where
        P: ParserInput<Stream = I>,
        $( $ft: Fn(I) -> ParseFnResult<P, $rt>, )*
{
    move |$stream| { $($content)* }
} // pub fn

$(#[$outer])*
///
/// The functions are borrowed, so the returned parser function has a
/// lifetime 'b that matches that; the input (lifetime 'a) must
/// outlive the resultant parser function
pub fn [<$fn_name _ref>] <'b, P, I: ParserInputStream<P>, $($rt,)* $($ft, )*>(
    $( $f : &'b $ft , )*
) -> impl Fn(I) -> ParseFnResult<P, $r> + 'b
where
    P: ParserInput<Stream = I>,
    $( $ft: Fn(I) -> ParseFnResult<P, $rt> +'b, )*
{
    move |$stream| { $($content)* }
} // pub fn

        } // paste
    }} // macro_rules

