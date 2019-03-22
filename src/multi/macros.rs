//! Parsers for applying parsers multiple times

/// `separated_list!(I -> IResult<I,T>, I -> IResult<I,O>) => I -> IResult<I, Vec<O>>`
/// separated_list(sep, X) returns Vec<X> will return Incomplete if there may be more elements
#[cfg(feature = "alloc")]
#[macro_export(local_inner_macros)]
macro_rules! separated_list(
  ($i:expr, $submac:ident!( $($args:tt)* ), $submac2:ident!( $($args2:tt)* )) => (
    separated_list!($i, |i| $submac!(i, $($args)*), |i| $submac2!(i, $($args2)*))
  );

  ($i:expr, $submac:ident!( $($args:tt)* ), $g:expr) => (
    separated_list!($i, |i| $submac!(i, $($args)*), $g);
  );

  ($i:expr, $f:expr, $submac:ident!( $($args:tt)* )) => (
    separated_list!($i, $f, |i| $submac!(i, $($args)*));
  );

  ($i:expr, $f:expr, $g:expr) => (
    $crate::separated_listc($i, $f, $g)
  );
);

/// `separated_nonempty_list!(I -> IResult<I,T>, I -> IResult<I,O>) => I -> IResult<I, Vec<O>>`
/// separated_nonempty_list(sep, X) returns Vec<X> will return Incomplete if there may be more elements
#[macro_export(local_inner_macros)]
macro_rules! separated_nonempty_list(
  ($i:expr, $submac:ident!( $($args:tt)* ), $submac2:ident!( $($args2:tt)* )) => (
    separated_nonempty_list!($i, |i| $submac!(i, $($args)*), |i| $submac2!(i, $($args2)*))
  );

  ($i:expr, $submac:ident!( $($args:tt)* ), $g:expr) => (
    separated_nonempty_list!($i, |i| $submac!(i, $($args)*), $g);
  );

  ($i:expr, $f:expr, $submac:ident!( $($args:tt)* )) => (
    separated_nonempty_list!($i, $f, |i| $submac!(i, $($args)*));
  );

  ($i:expr, $f:expr, $g:expr) => (
    $crate::separated_non_empty_listc($i, $f, $g)
  );
);

/// `separated_list_complete!(I -> IResult<I,T>, I -> IResult<I,O>) => I -> IResult<I, Vec<O>>`
/// This is equivalent to the `separated_list!` combinator, except that it will return `Error`
/// when either the separator or element subparser returns `Incomplete`.
#[macro_export(local_inner_macros)]
macro_rules! separated_list_complete(
  ($i:expr, $sep:ident!( $($args:tt)* ), $submac:ident!( $($args2:tt)* )) => ({
      separated_list!($i, complete!($sep!($($args)*)), complete!($submac!($($args2)*)))
  });

  ($i:expr, $submac:ident!( $($args:tt)* ), $g:expr) => (
      separated_list_complete!($i, $submac!($($args)*), call!($g));
  );
  ($i:expr, $f:expr, $submac:ident!( $($args:tt)* )) => (
      separated_list_complete!($i, call!($f), $submac!($($args)*));
  );
  ($i:expr, $f:expr, $g:expr) => (
      separated_list_complete!($i, call!($f), call!($g));
  );
);

/// `separated_nonempty_list_complete!(I -> IResult<I,T>, I -> IResult<I,O>) => I -> IResult<I, Vec<O>>`
/// This is equivalent to the `separated_nonempty_list!` combinator, except that it will return
/// `Error` when either the separator or element subparser returns `Incomplete`.
#[macro_export(local_inner_macros)]
macro_rules! separated_nonempty_list_complete(
  ($i:expr, $sep:ident!( $($args:tt)* ), $submac:ident!( $($args2:tt)* )) => ({
      separated_nonempty_list!($i, complete!($sep!($($args)*)), complete!($submac!($($args2)*)))
  });

  ($i:expr, $submac:ident!( $($args:tt)* ), $g:expr) => (
      separated_nonempty_list_complete!($i, $submac!($($args)*), call!($g));
  );
  ($i:expr, $f:expr, $submac:ident!( $($args:tt)* )) => (
      separated_nonempty_list_complete!($i, call!($f), $submac!($($args)*));
  );
  ($i:expr, $f:expr, $g:expr) => (
      separated_nonempty_list_complete!($i, call!($f), call!($g));
  );
);

/// `many0!(I -> IResult<I,O>) => I -> IResult<I, Vec<O>>`
/// Applies the parser 0 or more times and returns the list of results in a Vec.
///
/// The embedded parser may return Incomplete.
///
/// `many0` will only return `Error` if the embedded parser does not consume any input
/// (to avoid infinite loops).
///
/// ```
/// # #[macro_use] extern crate nom;
/// # fn main() {
///  named!(multi<&[u8], Vec<&[u8]> >, many0!( tag!( "abcd" ) ) );
///
///  let a = b"abcdabcdefgh";
///  let b = b"azerty";
///
///  let res = vec![&b"abcd"[..], &b"abcd"[..]];
///  assert_eq!(multi(&a[..]),Ok((&b"efgh"[..], res)));
///  assert_eq!(multi(&b[..]),Ok((&b"azerty"[..], Vec::new())));
/// # }
/// ```
///
#[cfg(feature = "alloc")]
#[macro_export(local_inner_macros)]
macro_rules! many0(
  ($i:expr, $submac:ident!( $($args:tt)* )) => (
    many0!($i, |i| $submac!(i, $($args)*))
  );
  ($i:expr, $f:expr) => (
    $crate::many0c($i, $f)
  );
);

/// `many1!(I -> IResult<I,O>) => I -> IResult<I, Vec<O>>`
/// Applies the parser 1 or more times and returns the list of results in a Vec
///
/// the embedded parser may return Incomplete
///
/// ```
/// # #[macro_use] extern crate nom;
/// # use nom::Err;
/// # use nom::ErrorKind;
/// # use nom::types::CompleteByteSlice;
/// # fn main() {
///  named!(multi<&[u8], Vec<&[u8]> >, many1!( tag!( "abcd" ) ) );
///
///  let a = b"abcdabcdefgh";
///  let b = b"azerty";
///
///  let res = vec![&b"abcd"[..], &b"abcd"[..]];
///  assert_eq!(multi(&a[..]), Ok((&b"efgh"[..], res)));
///  assert_eq!(multi(&b[..]), Err(Err::Error(error_position!(&b[..], ErrorKind::Many1))));
///
///  named!(multi_complete<CompleteByteSlice, Vec<CompleteByteSlice> >, many1!( tag!( "abcd" ) ) );
///  let c = CompleteByteSlice(b"abcdabcd");
///
///  let res = vec![CompleteByteSlice(b"abcd"), CompleteByteSlice(b"abcd")];
///  assert_eq!(multi_complete(c), Ok((CompleteByteSlice(b""), res)));
/// # }
/// ```
#[cfg(feature = "alloc")]
#[macro_export(local_inner_macros)]
macro_rules! many1(
  ($i:expr, $submac:ident!( $($args:tt)* )) => (
    many1!($i, |i| $submac!(i, $($args)*))
  );
  ($i:expr, $f:expr) => (
    $crate::many1c($i, $f)
  );
);

/// `many_till!(I -> IResult<I,O>, I -> IResult<I,P>) => I -> IResult<I, (Vec<O>, P)>`
/// Applies the first parser until the second applies. Returns a tuple containing the list
/// of results from the first in a Vec and the result of the second.
///
/// The first embedded parser may return Incomplete
///
/// ```
/// # #[macro_use] extern crate nom;
/// # use nom::Err;
/// # use nom::ErrorKind;
/// # fn main() {
///    named!(multi<&[u8], (Vec<&[u8]>, &[u8]) >, many_till!( tag!( "abcd" ), tag!( "efgh" ) ) );
///
///    let a = b"abcdabcdefghabcd";
///    let b = b"efghabcd";
///    let c = b"azerty";
///
///    let res_a = (vec![&b"abcd"[..], &b"abcd"[..]], &b"efgh"[..]);
///    let res_b: (Vec<&[u8]>, &[u8]) = (Vec::new(), &b"efgh"[..]);
///    assert_eq!(multi(&a[..]),Ok((&b"abcd"[..], res_a)));
///    assert_eq!(multi(&b[..]),Ok((&b"abcd"[..], res_b)));
///    assert_eq!(multi(&c[..]), Err(Err::Error(error_node_position!(&c[..], ErrorKind::ManyTill,
///      error_position!(&c[..], ErrorKind::Tag)))));
/// # }
/// ```
#[cfg(feature = "alloc")]
#[macro_export(local_inner_macros)]
macro_rules! many_till(
  ($i:expr, $submac:ident!( $($args:tt)* ), $submac2:ident!( $($args2:tt)* )) => (
    many_till!($i, |i| $submac!(i, $($args)*), |i| $submac2!(i, $($args2)*))
  );

  ($i:expr, $submac:ident!( $($args:tt)* ), $g:expr) => (
    many_till!($i, |i| $submac!(i, $($args)*), $g);
  );

  ($i:expr, $f:expr, $submac:ident!( $($args:tt)* )) => (
    many_till!($i, $f, |i| $submac!(i, $($args)*));
  );

  ($i:expr, $f:expr, $g:expr) => (
    $crate::many_tillc($i, $f, $g)
  );
);

/// `many_m_n!(usize, usize, I -> IResult<I,O>) => I -> IResult<I, Vec<O>>`
/// Applies the parser between m and n times (n included) and returns the list of
/// results in a Vec
///
/// the embedded parser may return Incomplete
///
/// ```
/// # #[macro_use] extern crate nom;
/// # use nom::Err;
/// # use nom::ErrorKind;
/// # fn main() {
///  named!(multi<&[u8], Vec<&[u8]> >, many_m_n!(2, 4, tag!( "abcd" ) ) );
///
///  let a = b"abcdefgh";
///  let b = b"abcdabcdefgh";
///  let c = b"abcdabcdabcdabcdabcdefgh";
///
///  assert_eq!(multi(&a[..]), Err(Err::Error(error_position!(&a[..], ErrorKind::ManyMN))));
///  let res = vec![&b"abcd"[..], &b"abcd"[..]];
///  assert_eq!(multi(&b[..]),Ok((&b"efgh"[..], res)));
///  let res2 = vec![&b"abcd"[..], &b"abcd"[..], &b"abcd"[..], &b"abcd"[..]];
///  assert_eq!(multi(&c[..]),Ok((&b"abcdefgh"[..], res2)));
/// # }
/// ```
#[cfg(feature = "alloc")]
#[macro_export(local_inner_macros)]
macro_rules! many_m_n(
  ($i:expr, $m:expr, $n: expr, $submac:ident!( $($args:tt)* )) => (
    many_m_n!($i, $m, $n, |i| $submac!(i, $($args)*))
  );
  ($i:expr, $m:expr, $n: expr, $f:expr) => (
    $crate::many_m_nc($i, $m, $n, $f)
  );
);

/// `many0_count!(I -> IResult<I,O>) => I -> IResult<I, usize>`
/// Applies the parser 0 or more times and returns the number of times the parser was applied.
///
/// `many0_count` will only return `Error` if the embedded parser does not consume any input
/// (to avoid infinite loops).
///
/// ```
/// #[macro_use] extern crate nom;
/// use nom::digit;
///
/// named!(number<&[u8], usize>, many0_count!(pair!(digit, tag!(","))));
///
/// fn main() {
///     assert_eq!(number(&b"123,45,abc"[..]), Ok((&b"abc"[..], 2)));
/// }
/// ```
///
#[macro_export]
macro_rules! many0_count {
  ($i:expr, $submac:ident!( $($args:tt)* )) => (
    many0_count!($i, |i| $submac!(i, $($args)*))
  );

  ($i:expr, $f:expr) => (
    $crate::many0_count($i, $f)
  );
}

/// `many1_count!(I -> IResult<I,O>) => I -> IResult<I, usize>`
/// Applies the parser 1 or more times and returns the number of times the parser was applied.
///
/// ```
/// #[macro_use] extern crate nom;
/// use nom::digit;
///
/// named!(number<&[u8], usize>, many1_count!(pair!(digit, tag!(","))));
///
/// fn main() {
///     assert_eq!(number(&b"123,45,abc"[..]), Ok((&b"abc"[..], 2)));
/// }
/// ```
///
#[macro_export]
macro_rules! many1_count {
  ($i:expr, $submac:ident!( $($args:tt)* )) => (
    many1_count!($i, |i| $submac!(i, $($args)*))
  );

  ($i:expr, $f:expr) => (
    $crate::many1_count($i, $f)
  );
}

/// `count!(I -> IResult<I,O>, nb) => I -> IResult<I, Vec<O>>`
/// Applies the child parser a specified number of times
///
/// ```
/// # #[macro_use] extern crate nom;
/// # use nom::Err;
/// # use nom::ErrorKind;
/// # fn main() {
///  named!(counter< Vec<&[u8]> >, count!( tag!( "abcd" ), 2 ) );
///
///  let a = b"abcdabcdabcdef";
///  let b = b"abcdefgh";
///  let res = vec![&b"abcd"[..], &b"abcd"[..]];
///
///  assert_eq!(counter(&a[..]),Ok((&b"abcdef"[..], res)));
///  assert_eq!(counter(&b[..]), Err(Err::Error(error_position!(&b[..], ErrorKind::Count))));
/// # }
/// ```
///
#[cfg(feature = "alloc")]
#[macro_export(local_inner_macros)]
macro_rules! count(
  ($i:expr, $submac:ident!( $($args:tt)* ), $count: expr) => (
    count!($i, |i| $submac!(i, $($args)*), $count)
  );
  ($i:expr, $f:expr, $count: expr) => (
    $crate::count($i, $f, $count)
  );
);

/// `count_fixed!(O, I -> IResult<I,O>, nb) => I -> IResult<I, [O; nb]>`
/// Applies the child parser a fixed number of times and returns a fixed size array
/// The type must be specified and it must be `Copy`
///
/// ```
/// # #[macro_use] extern crate nom;
/// # use nom::Err;
/// # use nom::ErrorKind;
/// # fn main() {
///  named!(counter< [&[u8]; 2] >, count_fixed!( &[u8], tag!( "abcd" ), 2 ) );
///  // can omit the type specifier if returning slices
///  // named!(counter< [&[u8]; 2] >, count_fixed!( tag!( "abcd" ), 2 ) );
///
///  let a = b"abcdabcdabcdef";
///  let b = b"abcdefgh";
///  let res = [&b"abcd"[..], &b"abcd"[..]];
///
///  assert_eq!(counter(&a[..]),Ok((&b"abcdef"[..], res)));
///  assert_eq!(counter(&b[..]), Err(Err::Error(error_position!(&b[..], ErrorKind::Count))));
/// # }
/// ```
///
#[macro_export(local_inner_macros)]
macro_rules! count_fixed (
  ($i:expr, $typ:ty, $submac:ident!( $($args:tt)* ), $count: expr) => (
    {
      use $crate::lib::std::result::Result::*;
      use $crate::Err;

      let ret;
      let mut input = $i.clone();
      // `$typ` must be Copy, and thus having no destructor, this is panic safe
      let mut res: [$typ; $count] = unsafe{[$crate::lib::std::mem::uninitialized(); $count as usize]};
      let mut cnt: usize = 0;

      loop {
        if cnt == $count {
          ret = Ok((input, res)); break;
        }

        match $submac!(input, $($args)*) {
          Ok((i,o)) => {
            res[cnt] = o;
            cnt += 1;
            input = i;
          },
          Err(Err::Error(e))  => {
            fn unify_types<T>(_: &T, _: &T) {}
            let e2 = error_position!($i, $crate::ErrorKind::Count);
            unify_types(&e, &e2);
            ret = Err(Err::Error(e2));
            break;
          },
          Err(e) => {
            ret = Err(e);
            break;
          },
        }
      }

      ret
    }
);
  ($i:expr, $typ: ty, $f:expr, $count: expr) => (
    count_fixed!($i, $typ, call!($f), $count);
  );
);

/// `length_count!(I -> IResult<I, nb>, I -> IResult<I,O>) => I -> IResult<I, Vec<O>>`
/// gets a number from the first parser, then applies the second parser that many times
#[macro_export(local_inner_macros)]
macro_rules! length_count(
  ($i:expr, $submac:ident!( $($args:tt)* ), $submac2:ident!( $($args2:tt)* )) => (
    {
      use $crate::lib::std::result::Result::*;
      use $crate::{Err,Convert};

      match $submac!($i, $($args)*) {
        Err(e)     => Err(Err::convert(e)),
        Ok((i, o)) => {
          match count!(i, $submac2!($($args2)*), o as usize) {
            Err(e)       => Err(Err::convert(e)),
            Ok((i2, o2)) => Ok((i2, o2))
          }
        }
      }
    }
  );

  ($i:expr, $submac:ident!( $($args:tt)* ), $g:expr) => (
    length_count!($i, $submac!($($args)*), call!($g));
  );

  ($i:expr, $f:expr, $submac:ident!( $($args:tt)* )) => (
    length_count!($i, call!($f), $submac!($($args)*));
  );

  ($i:expr, $f:expr, $g:expr) => (
    length_count!($i, call!($f), call!($g));
  );
);

/// `length_data!(I -> IResult<I, nb>) => O`
///
/// `length_data` gets a number from the first parser, than takes a subslice of the input
/// of that size, and returns that subslice
#[macro_export(local_inner_macros)]
macro_rules! length_data(
  ($i:expr, $submac:ident!( $($args:tt)* )) => ({
    use $crate::lib::std::result::Result::*;
    use $crate::{Convert,Err};

    match $submac!($i, $($args)*) {
      Err(e)     => Err(e),
      Ok((i, o)) => {
        match take!(i, o as usize) {
          Err(e)       => Err(Err::convert(e)),
          Ok((i2, o2)) => Ok((i2, o2))
        }
      }
    }
  });

  ($i:expr, $f:expr) => (
    length_data!($i, call!($f));
  );
);

/// `length_value!(I -> IResult<I, nb>, I -> IResult<I,O>) => I -> IResult<I, O>`
///
/// Gets a number from the first parser, takes a subslice of the input of that size,
/// then applies the second parser on that subslice. If the second parser returns
/// `Incomplete`, `length_value` will return an error
#[macro_export(local_inner_macros)]
macro_rules! length_value(
  ($i:expr, $submac:ident!( $($args:tt)* ), $submac2:ident!( $($args2:tt)* )) => (
    {
      use $crate::lib::std::result::Result::*;
      use $crate::{Err,Convert};

      match $submac!($i, $($args)*) {
        Err(e)     => Err(e),
        Ok((i, o)) => {
          match take!(i, o as usize) {
            Err(e)       => Err(Err::convert(e)),
            Ok((i2, o2)) => {
              match complete!(o2, $submac2!($($args2)*)) {
                Err(e)      => Err(Err::convert(e)),
                Ok((_, o3)) => Ok((i2, o3))
              }
            }
          }
        }
      }
    }
  );

  ($i:expr, $submac:ident!( $($args:tt)* ), $g:expr) => (
    length_value!($i, $submac!($($args)*), call!($g));
  );

  ($i:expr, $f:expr, $submac:ident!( $($args:tt)* )) => (
    length_value!($i, call!($f), $submac!($($args)*));
  );

  ($i:expr, $f:expr, $g:expr) => (
    length_value!($i, call!($f), call!($g));
  );
);

/// `fold_many0!(I -> IResult<I,O>, R, Fn(R, O) -> R) => I -> IResult<I, R>`
/// Applies the parser 0 or more times and folds the list of return values
///
/// the embedded parser may return Incomplete
///
/// ```
/// # #[macro_use] extern crate nom;
/// # fn main() {
///  named!(multi<&[u8], Vec<&[u8]> >,
///    fold_many0!( tag!( "abcd" ), Vec::new(), |mut acc: Vec<_>, item| {
///      acc.push(item);
///      acc
///  }));
///
///  let a = b"abcdabcdefgh";
///  let b = b"azerty";
///
///  let res = vec![&b"abcd"[..], &b"abcd"[..]];
///  assert_eq!(multi(&a[..]),Ok((&b"efgh"[..], res)));
///  assert_eq!(multi(&b[..]),Ok((&b"azerty"[..], Vec::new())));
/// # }
/// ```
/// 0 or more
#[macro_export(local_inner_macros)]
macro_rules! fold_many0(
  ($i:expr, $submac:ident!( $($args:tt)* ), $init:expr, $fold_f:expr) => (
    fold_many0!($i, |i| $submac!(i, $($args)*), $init, $fold_f)
  );
  ($i:expr, $f:expr, $init:expr, $fold_f:expr) => (
    $crate::fold_many0($i, $f, $init, $fold_f)
  );
);

/// `fold_many1!(I -> IResult<I,O>, R, Fn(R, O) -> R) => I -> IResult<I, R>`
/// Applies the parser 1 or more times and folds the list of return values
///
/// the embedded parser may return Incomplete
///
/// ```
/// # #[macro_use] extern crate nom;
/// # use nom::Err;
/// # use nom::ErrorKind;
/// # fn main() {
///  named!(multi<&[u8], Vec<&[u8]> >,
///    fold_many1!( tag!( "abcd" ), Vec::new(), |mut acc: Vec<_>, item| {
///      acc.push(item);
///      acc
///  }));
///
///  let a = b"abcdabcdefgh";
///  let b = b"azerty";
///
///  let res = vec![&b"abcd"[..], &b"abcd"[..]];
///  assert_eq!(multi(&a[..]),Ok((&b"efgh"[..], res)));
///  assert_eq!(multi(&b[..]), Err(Err::Error(error_position!(&b[..], ErrorKind::Many1))));
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! fold_many1(
  ($i:expr, $submac:ident!( $($args:tt)* ), $init:expr, $fold_f:expr) => (
    fold_many1!($i, |i| $submac!(i, $($args)*), $init, $fold_f)
  );
  ($i:expr, $f:expr, $init:expr, $fold_f:expr) => (
    $crate::fold_many1($i, $f, $init, $fold_f)
  );
  ($i:expr, $f:expr, $init:expr, $fold_f:expr) => (
    fold_many1!($i, call!($f), $init, $fold_f);
  );
);

/// `fold_many_m_n!(usize, usize, I -> IResult<I,O>, R, Fn(R, O) -> R) => I -> IResult<I, R>`
/// Applies the parser between m and n times (n included) and folds the list of return value
///
/// the embedded parser may return Incomplete
///
/// ```
/// # #[macro_use] extern crate nom;
/// # use nom::Err;
/// # use nom::ErrorKind;
/// # fn main() {
///  named!(multi<&[u8], Vec<&[u8]> >,
///    fold_many_m_n!(2, 4, tag!( "abcd" ), Vec::new(), |mut acc: Vec<_>, item| {
///      acc.push(item);
///      acc
///  }));
///
///  let a = b"abcdefgh";
///  let b = b"abcdabcdefgh";
///  let c = b"abcdabcdabcdabcdabcdefgh";
///
///  assert_eq!(multi(&a[..]), Err(Err::Error(error_position!(&a[..], ErrorKind::ManyMN))));
///  let res = vec![&b"abcd"[..], &b"abcd"[..]];
///  assert_eq!(multi(&b[..]),Ok((&b"efgh"[..], res)));
///  let res2 = vec![&b"abcd"[..], &b"abcd"[..], &b"abcd"[..], &b"abcd"[..]];
///  assert_eq!(multi(&c[..]),Ok((&b"abcdefgh"[..], res2)));
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! fold_many_m_n(
  ($i:expr, $m:expr, $n:expr, $submac:ident!( $($args:tt)* ), $init:expr, $fold_f:expr) => (
    fold_many_m_n!($i, $m, $n, |i| $submac!(i, $($args)*), $init, $fold_f)
  );
  ($i:expr, $m:expr, $n:expr, $f:expr, $init:expr, $fold_f:expr) => (
    $crate::fold_many_m_n($i, $m, $n, $f, $init, $fold_f)
  );
);

#[cfg(test)]
mod tests {
  use internal::{Err, IResult, Needed};
  use lib::std::str::{self, FromStr};
  #[cfg(feature = "alloc")]
  use lib::std::vec::Vec;
  use nom::{be_u16, be_u8, digit, le_u16};
  use util::ErrorKind;

  // reproduce the tag and take macros, because of module import order
  macro_rules! tag (
    ($i:expr, $inp: expr) => (
      {
        #[inline(always)]
        fn as_bytes<T: $crate::AsBytes>(b: &T) -> &[u8] {
          b.as_bytes()
        }

        let expected = $inp;
        let bytes    = as_bytes(&expected);

        tag_bytes!($i,bytes)
      }
    );
  );

  macro_rules! tag_bytes (
    ($i:expr, $bytes: expr) => (
      {
        use $crate::lib::std::cmp::min;
        let len = $i.len();
        let blen = $bytes.len();
        let m   = min(len, blen);
        let reduced = &$i[..m];
        let b       = &$bytes[..m];

        let res: IResult<_,_,u32> = if reduced != b {
          Err($crate::Err::Error($crate::Context::Code($i, $crate::ErrorKind::Tag::<u32>)))
        } else if m < blen {
          Err($crate::Err::Incomplete(Needed::Size(blen)))
        } else {
          Ok((&$i[blen..], reduced))
        };
        res
      }
    );
  );

  macro_rules! take (
    ($i:expr, $count:expr) => (
      {
        let cnt = $count as usize;
        let res:IResult<&[u8],&[u8],u32> = if $i.len() < cnt {
          Err($crate::Err::Incomplete(Needed::Size(cnt)))
        } else {
          Ok((&$i[cnt..],&$i[0..cnt]))
        };
        res
      }
    )
  );

  #[test]
  #[cfg(feature = "alloc")]
  fn separated_list() {
    named!(multi<&[u8],Vec<&[u8]> >, separated_list!(tag!(","), tag!("abcd")));
    named!(multi_empty<&[u8],Vec<&[u8]> >, separated_list!(tag!(","), tag!("")));
    named!(multi_longsep<&[u8],Vec<&[u8]> >, separated_list!(tag!(".."), tag!("abcd")));

    let a = &b"abcdef"[..];
    let b = &b"abcd,abcdef"[..];
    let c = &b"azerty"[..];
    let d = &b",,abc"[..];
    let e = &b"abcd,abcd,ef"[..];
    let f = &b"abc"[..];
    let g = &b"abcd."[..];
    let h = &b"abcd,abc"[..];

    let res1 = vec![&b"abcd"[..]];
    assert_eq!(multi(a), Ok((&b"ef"[..], res1)));
    let res2 = vec![&b"abcd"[..], &b"abcd"[..]];
    assert_eq!(multi(b), Ok((&b"ef"[..], res2)));
    assert_eq!(multi(c), Ok((&b"azerty"[..], Vec::new())));
    assert_eq!(multi_empty(d), Err(Err::Error(error_position!(d, ErrorKind::SeparatedList))));
    //let res3 = vec![&b""[..], &b""[..], &b""[..]];
    //assert_eq!(multi_empty(d),Ok((&b"abc"[..], res3)));
    let res4 = vec![&b"abcd"[..], &b"abcd"[..]];
    assert_eq!(multi(e), Ok((&b",ef"[..], res4)));

    assert_eq!(multi(f), Err(Err::Incomplete(Needed::Size(4))));
    assert_eq!(multi_longsep(g), Err(Err::Incomplete(Needed::Size(2))));
    assert_eq!(multi(h), Err(Err::Incomplete(Needed::Size(4))));
  }

  #[test]
  #[cfg(feature = "alloc")]
  fn separated_list_complete() {
    use nom::alpha;

    named!(multi<&[u8],Vec<&[u8]> >, separated_list_complete!(tag!(","), alpha));
    let a = &b"abcdef;"[..];
    let b = &b"abcd,abcdef;"[..];
    let c = &b"abcd,abcd,ef;"[..];
    let d = &b"abc."[..];
    let e = &b"abcd,ef."[..];
    let f = &b"123"[..];

    assert_eq!(multi(a), Ok((&b";"[..], vec![&a[..a.len() - 1]])));
    assert_eq!(multi(b), Ok((&b";"[..], vec![&b"abcd"[..], &b"abcdef"[..]])));
    assert_eq!(multi(c), Ok((&b";"[..], vec![&b"abcd"[..], &b"abcd"[..], &b"ef"[..]])));
    assert_eq!(multi(d), Ok((&b"."[..], vec![&b"abc"[..]])));
    assert_eq!(multi(e), Ok((&b"."[..], vec![&b"abcd"[..], &b"ef"[..]])));
    assert_eq!(multi(f), Ok((&b"123"[..], Vec::new())));
  }

  #[test]
  #[cfg(feature = "alloc")]
  fn separated_nonempty_list() {
    named!(multi<&[u8],Vec<&[u8]> >, separated_nonempty_list!(tag!(","), tag!("abcd")));
    named!(multi_longsep<&[u8],Vec<&[u8]> >, separated_nonempty_list!(tag!(".."), tag!("abcd")));

    let a = &b"abcdef"[..];
    let b = &b"abcd,abcdef"[..];
    let c = &b"azerty"[..];
    let d = &b"abcd,abcd,ef"[..];

    let f = &b"abc"[..];
    let g = &b"abcd."[..];
    let h = &b"abcd,abc"[..];

    let res1 = vec![&b"abcd"[..]];
    assert_eq!(multi(a), Ok((&b"ef"[..], res1)));
    let res2 = vec![&b"abcd"[..], &b"abcd"[..]];
    assert_eq!(multi(b), Ok((&b"ef"[..], res2)));
    assert_eq!(multi(c), Err(Err::Error(error_position!(c, ErrorKind::Tag))));
    let res3 = vec![&b"abcd"[..], &b"abcd"[..]];
    assert_eq!(multi(d), Ok((&b",ef"[..], res3)));

    assert_eq!(multi(f), Err(Err::Incomplete(Needed::Size(4))));
    assert_eq!(multi_longsep(g), Err(Err::Incomplete(Needed::Size(2))));
    assert_eq!(multi(h), Err(Err::Incomplete(Needed::Size(4))));
  }

  #[test]
  #[cfg(feature = "alloc")]
  fn separated_nonempty_list_complete() {
    use nom::alpha;

    named!(multi<&[u8],Vec<&[u8]> >, separated_nonempty_list_complete!(tag!(","), alpha));
    let a = &b"abcdef;"[..];
    let b = &b"abcd,abcdef;"[..];
    let c = &b"abcd,abcd,ef;"[..];
    let d = &b"abc."[..];
    let e = &b"abcd,ef."[..];
    let f = &b"123"[..];

    assert_eq!(multi(a), Ok((&b";"[..], vec![&a[..a.len() - 1]])));
    assert_eq!(multi(b), Ok((&b";"[..], vec![&b"abcd"[..], &b"abcdef"[..]])));
    assert_eq!(multi(c), Ok((&b";"[..], vec![&b"abcd"[..], &b"abcd"[..], &b"ef"[..]])));
    assert_eq!(multi(d), Ok((&b"."[..], vec![&b"abc"[..]])));
    assert_eq!(multi(e), Ok((&b"."[..], vec![&b"abcd"[..], &b"ef"[..]])));
    assert_eq!(multi(f), Err(Err::Error(error_position!(&b"123"[..], ErrorKind::Alpha))));
  }

  #[test]
  #[cfg(feature = "alloc")]
  fn many0() {
    named!(tag_abcd, tag!("abcd"));
    named!(tag_empty, tag!(""));
    named!( multi<&[u8],Vec<&[u8]> >, many0!(tag_abcd) );
    named!( multi_empty<&[u8],Vec<&[u8]> >, many0!(tag_empty) );

    assert_eq!(multi(&b"abcdef"[..]), Ok((&b"ef"[..], vec![&b"abcd"[..]])));
    assert_eq!(multi(&b"abcdabcdefgh"[..]), Ok((&b"efgh"[..], vec![&b"abcd"[..], &b"abcd"[..]])));
    assert_eq!(multi(&b"azerty"[..]), Ok((&b"azerty"[..], Vec::new())));
    assert_eq!(multi(&b"abcdab"[..]), Err(Err::Incomplete(Needed::Size(4))));
    assert_eq!(multi(&b"abcd"[..]), Err(Err::Incomplete(Needed::Size(4))));
    assert_eq!(multi(&b""[..]), Err(Err::Incomplete(Needed::Size(4))));
    assert_eq!(
      multi_empty(&b"abcdef"[..]),
      Err(Err::Error(error_position!(&b"abcdef"[..], ErrorKind::Many0)))
    );
  }

  #[cfg(nightly)]
  use test::Bencher;

  #[cfg(nightly)]
  #[bench]
  fn many0_bench(b: &mut Bencher) {
    named!(multi<&[u8],Vec<&[u8]> >, many0!(tag!("abcd")));
    b.iter(|| multi(&b"abcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcd"[..]));
  }

  #[test]
  #[cfg(feature = "alloc")]
  fn many1() {
    named!(multi<&[u8],Vec<&[u8]> >, many1!(dbg_dmp!(tag!("abcd"))));

    let a = &b"abcdef"[..];
    let b = &b"abcdabcdefgh"[..];
    let c = &b"azerty"[..];
    let d = &b"abcdab"[..];

    let res1 = vec![&b"abcd"[..]];
    assert_eq!(multi(a), Ok((&b"ef"[..], res1)));
    let res2 = vec![&b"abcd"[..], &b"abcd"[..]];
    assert_eq!(multi(b), Ok((&b"efgh"[..], res2)));
    assert_eq!(multi(c), Err(Err::Error(error_position!(c, ErrorKind::Many1))));
    assert_eq!(multi(d), Err(Err::Incomplete(Needed::Size(4))));
  }

  #[test]
  #[cfg(feature = "alloc")]
  fn many_till() {
    named!(multi<&[u8], (Vec<&[u8]>, &[u8]) >, many_till!( tag!( "abcd" ), tag!( "efgh" ) ) );

    let a = b"abcdabcdefghabcd";
    let b = b"efghabcd";
    let c = b"azerty";

    let res_a = (vec![&b"abcd"[..], &b"abcd"[..]], &b"efgh"[..]);
    let res_b: (Vec<&[u8]>, &[u8]) = (Vec::new(), &b"efgh"[..]);
    assert_eq!(multi(&a[..]), Ok((&b"abcd"[..], res_a)));
    assert_eq!(multi(&b[..]), Ok((&b"abcd"[..], res_b)));
    assert_eq!(
      multi(&c[..]),
      Err(Err::Error(error_node_position!(
        &c[..],
        ErrorKind::ManyTill,
        error_position!(&c[..], ErrorKind::Tag)
      )))
    );
  }

  #[test]
  #[cfg(feature = "std")]
  fn infinite_many() {
    fn tst(input: &[u8]) -> IResult<&[u8], &[u8]> {
      println!("input: {:?}", input);
      Err(Err::Error(error_position!(input, ErrorKind::Custom(0u32))))
    }

    // should not go into an infinite loop
    named!(multi0<&[u8],Vec<&[u8]> >, many0!(tst));
    let a = &b"abcdef"[..];
    assert_eq!(multi0(a), Ok((a, Vec::new())));

    named!(multi1<&[u8],Vec<&[u8]> >, many1!(tst));
    let a = &b"abcdef"[..];
    assert_eq!(multi1(a), Err(Err::Error(error_position!(a, ErrorKind::Many1))));
  }

  #[test]
  #[cfg(feature = "alloc")]
  fn many_m_n() {
    named!(multi<&[u8],Vec<&[u8]> >, many_m_n!(2, 4, tag!("Abcd")));

    let a = &b"Abcdef"[..];
    let b = &b"AbcdAbcdefgh"[..];
    let c = &b"AbcdAbcdAbcdAbcdefgh"[..];
    let d = &b"AbcdAbcdAbcdAbcdAbcdefgh"[..];
    let e = &b"AbcdAb"[..];

    assert_eq!(multi(a), Err(Err::Error(error_position!(a, ErrorKind::ManyMN))));
    let res1 = vec![&b"Abcd"[..], &b"Abcd"[..]];
    assert_eq!(multi(b), Ok((&b"efgh"[..], res1)));
    let res2 = vec![&b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..]];
    assert_eq!(multi(c), Ok((&b"efgh"[..], res2)));
    let res3 = vec![&b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..]];
    assert_eq!(multi(d), Ok((&b"Abcdefgh"[..], res3)));
    assert_eq!(multi(e), Err(Err::Incomplete(Needed::Size(4))));
  }

  #[test]
  #[cfg(feature = "alloc")]
  fn count() {
    const TIMES: usize = 2;
    named!(tag_abc, tag!("abc"));
    named!( cnt_2<&[u8], Vec<&[u8]> >, count!(tag_abc, TIMES ) );

    assert_eq!(cnt_2(&b"abcabcabcdef"[..]), Ok((&b"abcdef"[..], vec![&b"abc"[..], &b"abc"[..]])));
    assert_eq!(cnt_2(&b"ab"[..]), Err(Err::Incomplete(Needed::Size(3))));
    assert_eq!(cnt_2(&b"abcab"[..]), Err(Err::Incomplete(Needed::Size(3))));
    assert_eq!(cnt_2(&b"xxx"[..]), Err(Err::Error(error_position!(&b"xxx"[..], ErrorKind::Count))));
    assert_eq!(
      cnt_2(&b"xxxabcabcdef"[..]),
      Err(Err::Error(error_position!(&b"xxxabcabcdef"[..], ErrorKind::Count)))
    );
    assert_eq!(
      cnt_2(&b"abcxxxabcdef"[..]),
      Err(Err::Error(error_position!(&b"abcxxxabcdef"[..], ErrorKind::Count)))
    );
  }

  #[test]
  #[cfg(feature = "alloc")]
  fn count_zero() {
    const TIMES: usize = 0;
    named!(tag_abc, tag!("abc"));
    named!( counter_2<&[u8], Vec<&[u8]> >, count!(tag_abc, TIMES ) );

    let done = &b"abcabcabcdef"[..];
    let parsed_done = Vec::new();
    let rest = done;
    let incomplete_1 = &b"ab"[..];
    let parsed_incompl_1 = Vec::new();
    let incomplete_2 = &b"abcab"[..];
    let parsed_incompl_2 = Vec::new();
    let error = &b"xxx"[..];
    let error_remain = &b"xxx"[..];
    let parsed_err = Vec::new();
    let error_1 = &b"xxxabcabcdef"[..];
    let parsed_err_1 = Vec::new();
    let error_1_remain = &b"xxxabcabcdef"[..];
    let error_2 = &b"abcxxxabcdef"[..];
    let parsed_err_2 = Vec::new();
    let error_2_remain = &b"abcxxxabcdef"[..];

    assert_eq!(counter_2(done), Ok((rest, parsed_done)));
    assert_eq!(counter_2(incomplete_1), Ok((incomplete_1, parsed_incompl_1)));
    assert_eq!(counter_2(incomplete_2), Ok((incomplete_2, parsed_incompl_2)));
    assert_eq!(counter_2(error), Ok((error_remain, parsed_err)));
    assert_eq!(counter_2(error_1), Ok((error_1_remain, parsed_err_1)));
    assert_eq!(counter_2(error_2), Ok((error_2_remain, parsed_err_2)));
  }

  #[test]
  fn count_fixed() {
    const TIMES: usize = 2;
    named!(tag_abc, tag!("abc"));
    named!( cnt_2<&[u8], [&[u8]; TIMES] >, count_fixed!(&[u8], tag_abc, TIMES ) );

    assert_eq!(cnt_2(&b"abcabcabcdef"[..]), Ok((&b"abcdef"[..], [&b"abc"[..], &b"abc"[..]])));
    assert_eq!(cnt_2(&b"ab"[..]), Err(Err::Incomplete(Needed::Size(3))));
    assert_eq!(cnt_2(&b"abcab"[..]), Err(Err::Incomplete(Needed::Size(3))));
    assert_eq!(cnt_2(&b"xxx"[..]), Err(Err::Error(error_position!(&b"xxx"[..], ErrorKind::Count))));
    assert_eq!(
      cnt_2(&b"xxxabcabcdef"[..]),
      Err(Err::Error(error_position!(&b"xxxabcabcdef"[..], ErrorKind::Count)))
    );
    assert_eq!(
      cnt_2(&b"abcxxxabcdef"[..]),
      Err(Err::Error(error_position!(&b"abcxxxabcdef"[..], ErrorKind::Count)))
    );
  }

  #[allow(dead_code)]
  pub fn compile_count_fixed(input: &[u8]) -> IResult<&[u8], ()> {
    do_parse!(input, tag!("abcd") >> count_fixed!(u16, le_u16, 4) >> eof!() >> ())
  }

  #[derive(Debug, Clone, PartialEq)]
  pub struct NilError;

  impl From<u32> for NilError {
    fn from(_: u32) -> Self {
      NilError
    }
  }

  #[allow(unused_variables)]
  #[test]
  fn count_fixed_no_type() {
    const TIMES: usize = 2;
    named!(tag_abc, tag!("abc"));
    named!( counter_2<&[u8], [&[u8]; TIMES], NilError >, count_fixed!(&[u8], fix_error!(NilError, tag_abc), TIMES ) );

    let done = &b"abcabcabcdef"[..];
    let parsed_main = [&b"abc"[..], &b"abc"[..]];
    let rest = &b"abcdef"[..];
    let incomplete_1 = &b"ab"[..];
    let incomplete_2 = &b"abcab"[..];
    let error = &b"xxx"[..];
    let error_1 = &b"xxxabcabcdef"[..];
    let error_1_remain = &b"xxxabcabcdef"[..];
    let error_2 = &b"abcxxxabcdef"[..];
    let error_2_remain = &b"abcxxxabcdef"[..];

    assert_eq!(counter_2(done), Ok((rest, parsed_main)));
    assert_eq!(counter_2(incomplete_1), Err(Err::Incomplete(Needed::Size(3))));
    assert_eq!(counter_2(incomplete_2), Err(Err::Incomplete(Needed::Size(3))));
    assert_eq!(counter_2(error), Err(Err::Error(error_position!(error, ErrorKind::Count))));
    assert_eq!(
      counter_2(error_1),
      Err(Err::Error(error_position!(error_1_remain, ErrorKind::Count)))
    );
    assert_eq!(
      counter_2(error_2),
      Err(Err::Error(error_position!(error_2_remain, ErrorKind::Count)))
    );
  }

  named!(pub number<u32>, map_res!(
    map_res!(
      digit,
      str::from_utf8
    ),
    FromStr::from_str
  ));

  #[test]
  #[cfg(feature = "alloc")]
  fn length_count() {
    named!(tag_abc, tag!(&b"abc"[..]));
    named!( cnt<&[u8], Vec<&[u8]> >, length_count!(number, tag_abc) );

    assert_eq!(cnt(&b"2abcabcabcdef"[..]), Ok((&b"abcdef"[..], vec![&b"abc"[..], &b"abc"[..]])));
    assert_eq!(cnt(&b"2ab"[..]), Err(Err::Incomplete(Needed::Size(3))));
    assert_eq!(cnt(&b"3abcab"[..]), Err(Err::Incomplete(Needed::Size(3))));
    assert_eq!(cnt(&b"xxx"[..]), Err(Err::Error(error_position!(&b"xxx"[..], ErrorKind::Digit))));
    assert_eq!(
      cnt(&b"2abcxxx"[..]),
      Err(Err::Error(error_position!(&b"abcxxx"[..], ErrorKind::Count)))
    );
  }

  #[test]
  fn length_data() {
    named!( take<&[u8], &[u8]>, length_data!(number) );

    assert_eq!(take(&b"6abcabcabcdef"[..]), Ok((&b"abcdef"[..], &b"abcabc"[..])));
    assert_eq!(take(&b"3ab"[..]), Err(Err::Incomplete(Needed::Size(3))));
    assert_eq!(take(&b"xxx"[..]), Err(Err::Error(error_position!(&b"xxx"[..], ErrorKind::Digit))));
    assert_eq!(take(&b"2abcxxx"[..]), Ok((&b"cxxx"[..], &b"ab"[..])));
  }

  #[test]
  fn length_value_test() {
    named!(length_value_1<&[u8], u16 >, length_value!(be_u8, be_u16));
    named!(length_value_2<&[u8], (u8, u8) >, length_value!(be_u8, tuple!(be_u8, be_u8)));

    let i1 = [0, 5, 6];
    assert_eq!(length_value_1(&i1), Err(Err::Error(error_position!(&b""[..], ErrorKind::Complete))));
    assert_eq!(length_value_2(&i1), Err(Err::Error(error_position!(&b""[..], ErrorKind::Complete))));

    let i2 = [1, 5, 6, 3];
    assert_eq!(
      length_value_1(&i2),
      Err(Err::Error(error_position!(&i2[1..2], ErrorKind::Complete)))
    );
    assert_eq!(
      length_value_2(&i2),
      Err(Err::Error(error_position!(&i2[1..2], ErrorKind::Complete)))
    );

    let i3 = [2, 5, 6, 3, 4, 5, 7];
    assert_eq!(length_value_1(&i3), Ok((&i3[3..], 1286)));
    assert_eq!(length_value_2(&i3), Ok((&i3[3..], (5, 6))));

    let i4 = [3, 5, 6, 3, 4, 5];
    assert_eq!(length_value_1(&i4), Ok((&i4[4..], 1286)));
    assert_eq!(length_value_2(&i4), Ok((&i4[4..], (5, 6))));
  }

  #[test]
  #[cfg(feature = "alloc")]
  fn fold_many0() {
    fn fold_into_vec<T>(mut acc: Vec<T>, item: T) -> Vec<T> {
      acc.push(item);
      acc
    };
    named!(tag_abcd, tag!("abcd"));
    named!(tag_empty, tag!(""));
    named!( multi<&[u8],Vec<&[u8]> >, fold_many0!(tag_abcd, Vec::new(), fold_into_vec) );
    named!( multi_empty<&[u8],Vec<&[u8]> >, fold_many0!(tag_empty, Vec::new(), fold_into_vec) );

    assert_eq!(multi(&b"abcdef"[..]), Ok((&b"ef"[..], vec![&b"abcd"[..]])));
    assert_eq!(multi(&b"abcdabcdefgh"[..]), Ok((&b"efgh"[..], vec![&b"abcd"[..], &b"abcd"[..]])));
    assert_eq!(multi(&b"azerty"[..]), Ok((&b"azerty"[..], Vec::new())));
    assert_eq!(multi(&b"abcdab"[..]), Err(Err::Incomplete(Needed::Size(4))));
    assert_eq!(multi(&b"abcd"[..]), Err(Err::Incomplete(Needed::Size(4))));
    assert_eq!(multi(&b""[..]), Err(Err::Incomplete(Needed::Size(4))));
    assert_eq!(
      multi_empty(&b"abcdef"[..]),
      Err(Err::Error(error_position!(&b"abcdef"[..], ErrorKind::Many0)))
    );
  }

  #[test]
  #[cfg(feature = "alloc")]
  fn fold_many1() {
    fn fold_into_vec<T>(mut acc: Vec<T>, item: T) -> Vec<T> {
      acc.push(item);
      acc
    };
    named!(multi<&[u8],Vec<&[u8]> >, fold_many1!(tag!("abcd"), Vec::new(), fold_into_vec));

    let a = &b"abcdef"[..];
    let b = &b"abcdabcdefgh"[..];
    let c = &b"azerty"[..];
    let d = &b"abcdab"[..];

    let res1 = vec![&b"abcd"[..]];
    assert_eq!(multi(a), Ok((&b"ef"[..], res1)));
    let res2 = vec![&b"abcd"[..], &b"abcd"[..]];
    assert_eq!(multi(b), Ok((&b"efgh"[..], res2)));
    assert_eq!(multi(c), Err(Err::Error(error_position!(c, ErrorKind::Many1))));
    assert_eq!(multi(d), Err(Err::Incomplete(Needed::Size(4))));
  }

  #[test]
  #[cfg(feature = "alloc")]
  fn fold_many_m_n() {
    fn fold_into_vec<T>(mut acc: Vec<T>, item: T) -> Vec<T> {
      acc.push(item);
      acc
    };
    named!(multi<&[u8],Vec<&[u8]> >, fold_many_m_n!(2, 4, tag!("Abcd"), Vec::new(), fold_into_vec));

    let a = &b"Abcdef"[..];
    let b = &b"AbcdAbcdefgh"[..];
    let c = &b"AbcdAbcdAbcdAbcdefgh"[..];
    let d = &b"AbcdAbcdAbcdAbcdAbcdefgh"[..];
    let e = &b"AbcdAb"[..];

    assert_eq!(multi(a), Err(Err::Error(error_position!(a, ErrorKind::ManyMN))));
    let res1 = vec![&b"Abcd"[..], &b"Abcd"[..]];
    assert_eq!(multi(b), Ok((&b"efgh"[..], res1)));
    let res2 = vec![&b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..]];
    assert_eq!(multi(c), Ok((&b"efgh"[..], res2)));
    let res3 = vec![&b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..]];
    assert_eq!(multi(d), Ok((&b"Abcdefgh"[..], res3)));
    assert_eq!(multi(e), Err(Err::Incomplete(Needed::Size(4))));
  }

  #[test]
  fn many0_count() {
    named!(
      count0_nums(&[u8]) -> usize,
      many0_count!(pair!(digit, tag!(",")))
    );

    assert_eq!(count0_nums(&b"123,junk"[..]), Ok((&b"junk"[..], 1)));

    assert_eq!(count0_nums(&b"123,45,junk"[..]), Ok((&b"junk"[..], 2)));

    assert_eq!(count0_nums(&b"1,2,3,4,5,6,7,8,9,0,junk"[..]), Ok((&b"junk"[..], 10)));

    assert_eq!(count0_nums(&b"hello"[..]), Ok((&b"hello"[..], 0)));
  }

  #[test]
  fn many1_count() {
    named!(
      count1_nums(&[u8]) -> usize,
      many1_count!(pair!(digit, tag!(",")))
    );

    assert_eq!(count1_nums(&b"123,45,junk"[..]), Ok((&b"junk"[..], 2)));

    assert_eq!(count1_nums(&b"1,2,3,4,5,6,7,8,9,0,junk"[..]), Ok((&b"junk"[..], 10)));

    assert_eq!(
      count1_nums(&b"hello"[..]),
      Err(Err::Error(error_position!(&b"hello"[..], ErrorKind::Many1Count)))
    );
  }

}
