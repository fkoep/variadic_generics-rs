#![no_std]

// ++++++++++++++++++++ va_invoke ++++++++++++++++++++

/// Invokes a macro multiple times with optional args, a length, a growing list
/// of distinct identifiers and a growing list of successive indices.
///
/// ```ignore
/// va_invoke!(mymacro arg1 arg2);
/// ```
///
/// will be expanded to
///
/// ```ignore
/// mymacro!(arg1 arg2 (1) (_Va0) (0));
/// mymacro!(arg1 arg2 (2) (_Va0, _Va1) (0, 1));
/// mymacro!(arg1 arg2 (3) (_Va0, _Va1, _Va2) (0, 1, 2));
/// // ...
/// mymacro!(arg1 arg2 (12) (_Va0, ..., _Va11) (0, ..., 11));
/// ```
///
/// The lists will grow up to 12 elements, which is the same amount of elements
/// up to which standard traits like `Debug` and `Clone` are implemented for
/// tuples. If, for some strange reason, this isn't enough for you, use
/// `[va_invoke_more](macro.va_invoke_more.html)`.
#[macro_export]
macro_rules! va_invoke {
    ($mac:ident $($cur_args:tt)*) => {
        $mac!($($cur_args)* (1) (_Va0) (0));
        $mac!($($cur_args)* (2) (_Va0, _Va1) (0, 1));
        $mac!($($cur_args)* (3) (_Va0, _Va1, _Va2) (0, 1, 2));
        $mac!($($cur_args)* (4) (_Va0, _Va1, _Va2, _Va3) (0, 1, 2, 3));
        $mac!($($cur_args)* (5) (_Va0, _Va1, _Va2, _Va3, _Va4) (0, 1, 2, 3, 4));
        $mac!($($cur_args)* (6) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5) (0, 1, 2, 3, 4, 5));
        $mac!($($cur_args)* (7) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6) (0, 1, 2, 3, 4, 5, 6));
        $mac!($($cur_args)* (8) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7) (0, 1, 2, 3, 4, 5, 6, 7));
        $mac!($($cur_args)* (9) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7, _Va8) (0, 1, 2, 3, 4, 5, 6, 7, 8));
        $mac!($($cur_args)* (10) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7, _Va8, _Va9) (0, 1, 2, 3, 4, 5, 6, 7, 8, 9));
        $mac!($($cur_args)* (11) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7, _Va8, _Va9, _Va10) (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10));
        $mac!($($cur_args)* (12) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7, _Va8, _Va9, _Va10, _Va11) (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11));
    };
}

/// Like `[va_invoke](macro.va_invoke.html)`, but with lists of up to 32
/// elements.
///
/// ```ignore
/// va_invoke_more!(mymacro arg1 arg2);
/// ```
///
/// will be expanded to
///
/// ```ignore
/// mymacro!(arg1 arg2 (1) (_Va0) (0));
/// mymacro!(arg1 arg2 (2) (_Va0, _Va1) (0, 1));
/// mymacro!(arg1 arg2 (3) (_Va0, _Va1, _Va2) (0, 1, 2));
/// // ...
/// mymacro!(arg1 arg2 (32) (_Va0, ..., _Va31) (0, ..., 31));
/// ```
///
/// Be aware this will **significally** worsen your compilation times.
#[macro_export]
macro_rules! va_invoke_more {
    ($mac:ident $($cur_args:tt)*) => {
        va_invoke!($mac $($cur_args)*);
        $mac!($($cur_args)* (13) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7, _Va8, _Va9, _Va10, _Va11, _Va12) (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12));
        $mac!($($cur_args)* (14) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7, _Va8, _Va9, _Va10, _Va11, _Va12, _Va13) (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13));
        $mac!($($cur_args)* (15) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7, _Va8, _Va9, _Va10, _Va11, _Va12, _Va13, _Va14) (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14));
        $mac!($($cur_args)* (16) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7, _Va8, _Va9, _Va10, _Va11, _Va12, _Va13, _Va14, _Va15) (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
        $mac!($($cur_args)* (17) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7, _Va8, _Va9, _Va10, _Va11, _Va12, _Va13, _Va14, _Va15, _Va16) (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16));
        $mac!($($cur_args)* (18) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7, _Va8, _Va9, _Va10, _Va11, _Va12, _Va13, _Va14, _Va15, _Va16, _Va17) (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17));
        $mac!($($cur_args)* (19) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7, _Va8, _Va9, _Va10, _Va11, _Va12, _Va13, _Va14, _Va15, _Va16, _Va17, _Va18) (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18));
        $mac!($($cur_args)* (20) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7, _Va8, _Va9, _Va10, _Va11, _Va12, _Va13, _Va14, _Va15, _Va16, _Va17, _Va18, _Va19) (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19));
        $mac!($($cur_args)* (21) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7, _Va8, _Va9, _Va10, _Va11, _Va12, _Va13, _Va14, _Va15, _Va16, _Va17, _Va18, _Va19, _Va20) (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20));
        $mac!($($cur_args)* (22) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7, _Va8, _Va9, _Va10, _Va11, _Va12, _Va13, _Va14, _Va15, _Va16, _Va17, _Va18, _Va19, _Va20, _Va21) (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21));
        $mac!($($cur_args)* (23) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7, _Va8, _Va9, _Va10, _Va11, _Va12, _Va13, _Va14, _Va15, _Va16, _Va17, _Va18, _Va19, _Va20, _Va21, _Va22) (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22));
        $mac!($($cur_args)* (24) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7, _Va8, _Va9, _Va10, _Va11, _Va12, _Va13, _Va14, _Va15, _Va16, _Va17, _Va18, _Va19, _Va20, _Va21, _Va22, _Va23) (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23));
        $mac!($($cur_args)* (25) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7, _Va8, _Va9, _Va10, _Va11, _Va12, _Va13, _Va14, _Va15, _Va16, _Va17, _Va18, _Va19, _Va20, _Va21, _Va22, _Va23, _Va24) (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24));
        $mac!($($cur_args)* (26) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7, _Va8, _Va9, _Va10, _Va11, _Va12, _Va13, _Va14, _Va15, _Va16, _Va17, _Va18, _Va19, _Va20, _Va21, _Va22, _Va23, _Va24, _Va25) (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25));
        $mac!($($cur_args)* (27) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7, _Va8, _Va9, _Va10, _Va11, _Va12, _Va13, _Va14, _Va15, _Va16, _Va17, _Va18, _Va19, _Va20, _Va21, _Va22, _Va23, _Va24, _Va25, _Va26) (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26));
        $mac!($($cur_args)* (28) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7, _Va8, _Va9, _Va10, _Va11, _Va12, _Va13, _Va14, _Va15, _Va16, _Va17, _Va18, _Va19, _Va20, _Va21, _Va22, _Va23, _Va24, _Va25, _Va26, _Va27) (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27));
        $mac!($($cur_args)* (29) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7, _Va8, _Va9, _Va10, _Va11, _Va12, _Va13, _Va14, _Va15, _Va16, _Va17, _Va18, _Va19, _Va20, _Va21, _Va22, _Va23, _Va24, _Va25, _Va26, _Va27, _Va28) (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28));
        $mac!($($cur_args)* (30) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7, _Va8, _Va9, _Va10, _Va11, _Va12, _Va13, _Va14, _Va15, _Va16, _Va17, _Va18, _Va19, _Va20, _Va21, _Va22, _Va23, _Va24, _Va25, _Va26, _Va27, _Va28, _Va29) (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29));
        $mac!($($cur_args)* (31) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7, _Va8, _Va9, _Va10, _Va11, _Va12, _Va13, _Va14, _Va15, _Va16, _Va17, _Va18, _Va19, _Va20, _Va21, _Va22, _Va23, _Va24, _Va25, _Va26, _Va27, _Va28, _Va29, _Va30) (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30));
        $mac!($($cur_args)* (32) (_Va0, _Va1, _Va2, _Va3, _Va4, _Va5, _Va6, _Va7, _Va8, _Va9, _Va10, _Va11, _Va12, _Va13, _Va14, _Va15, _Va16, _Va17, _Va18, _Va19, _Va20, _Va21, _Va22, _Va23, _Va24, _Va25, _Va26, _Va27, _Va28, _Va29, _Va30, _Va31) (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31));
    };
}

/// Like `[va_invoke](macro.va_invoke.html)`, but with lists of up to 32
/// elements
/// and also includes an "empty list".
///
/// ```ignore
/// va_invoke_more!(mymacro arg1 arg2);
/// ```
///
/// will be expanded to
///
/// ```ignore
/// mymacro!(arg1 arg2 (0) () ());
/// mymacro!(arg1 arg2 (1) (_Va0) (0));
/// mymacro!(arg1 arg2 (2) (_Va0, _Va1) (0, 1));
/// // ...
/// mymacro!(arg1 arg2 (12) (_Va0, ..., _Va11) (0, ..., 11));
/// ```
#[macro_export]
macro_rules! va_invoke_with_nil {
    ($mac:ident $($cur_args:tt)*) => {
        $mac!($($cur_args)* (0) () ());
        va_invoke!($mac $($cur_args)*);
    };
}

/// Like `[va_invoke](macro.va_invoke_more.html)`, but uses  also includes an
/// "empty list".
///
/// ```ignore
/// va_invoke_more!(mymacro arg1 arg2);
/// ```
///
/// will be expanded to
///
/// ```ignore
/// mymacro!(arg1 arg2 (0) () ());
/// mymacro!(arg1 arg2 (1) (_Va0) (0));
/// mymacro!(arg1 arg2 (2) (_Va0, _Va1) (0, 1));
/// // ...
/// mymacro!(arg1 arg2 (32) (_Va0, ..., _Va31) (0, ..., 31));
/// ```
#[macro_export]
macro_rules! va_invoke_more_with_nil {
    ($mac:ident $($cur_args:tt)*) => {
        $mac!($($cur_args)* (0) () ());
        va_invoke_more!($mac $($cur_args)*);
    };
}

// ++++++++++++++++++++ va_expand ++++++++++++++++++++

#[doc(hidden)]
#[macro_export]
macro_rules! _va_expand {
    ($invoke_mac:ident ($($pat_va_len:tt)+) $pat_va_idents:tt $pat_va_indices:tt $($mac_body:tt)*) => {
        macro_rules! _va_mac {
            (($($pat_va_len)+) $pat_va_idents $pat_va_indices) => { $($mac_body)* }
        }
        $invoke_mac!(_va_mac);
    };
    ($invoke_mac:ident $dummy_mac:ident $pat_va_len:tt $pat_va_idents:tt $pat_va_indices:tt $($mac_body:tt)*) => {
        macro_rules! $dummy_mac {
            ($pat_va_len $pat_va_idents $pat_va_indices) => { $($mac_body)* }
        }
        $invoke_mac!($dummy_mac);
    };
}

/// Generates a macro for you and immediatly passes it on to
/// `[va_invoke](macro.va_invoke.html)`.
///
/// ```ignore
/// va_expand!{ ($va_len:tt) ($($va_idents),+) ($($va_indices:tt),+)
///     ...
/// }
/// ```
///
/// will be expanded to
///
/// ```ignore
/// macro_rules! _va_mac {
///     (($va_len:tt) ($($va_idents),+) ($($va_indices:tt),+)) => {
///         ...
///     }
/// }
/// va_invoke!(_va_mac);
/// ```
///
/// ---
/// **NOTE** Due to language limitations (macro-expanded `macro_rules!`s may
/// not shadow existing macros (see RFC 1560)), when calling this macro twice
/// or more within the same module, you need to supply a unique identifier as
/// the very first argument (which will be the name of the generated macro).
/// You can choose it to be random or descriptive.
///
/// ```ignore
/// va_expand!{ _asf ($va_len:tt) ($($va_idents),+) ($($va_indices:tt),+)
///     ...
/// }
/// va_expand!{ _impl_mytraits ($va_len:tt) ($($va_idents),+)
/// ($($va_indices:tt),+)
///     ...
/// }
/// ```
#[macro_export]
macro_rules! va_expand {
    ($($tt:tt)+) => {
        _va_expand!(va_invoke $($tt)*);
    };
}

/// Like `[va_expand](macro.va_expand.html)`, but uses
/// `[va_invoke_more](macro.va_invoke_more.html)`.
#[macro_export]
macro_rules! va_expand_more {
    ($($tt:tt)+) => {
        _va_expand!(va_invoke_more $($tt)*);
    };
}

/// Like `[va_expand](macro.va_expand.html)`, but uses
/// `[va_invoke_with_nil](macro.va_invoke_with_nil.html)`.
///
/// ---
/// **NOTE** `$va_idents` and `$va_indices` can be empty, so unlike
/// `[va_expand](macro.va_expand.html)` or
/// `[va_expand_more](macro.va_expand_more.html)`,
/// you are required to use zero-or-more repetitions here (`$()*`).
#[macro_export]
macro_rules! va_expand_with_nil {
    ($($tt:tt)+) => {
        _va_expand!(va_invoke_with_nil $($tt)*);
    };
}

/// Like `[va_expand](macro.va_expand.html)`, but uses
/// `[va_invoke_more_with_nil](macro.va_invoke_more_with_nil.html)`.
///
/// ---
/// **NOTE** `$va_idents` and `$va_indices` can be empty, so unlike
/// `[va_expand](macro.va_expand.html)` or
/// `[va_expand_more](macro.va_expand_more.html)`,
/// you are required to use zero-or-more repetitions here (`$()*`).
#[macro_export]
macro_rules! va_expand_more_with_nil {
    ($($tt:tt)+) => {
        _va_expand!(va_invoke_more_with_nil $($tt)*);
    };
}

// ++++++++++++++++++++ Tests ++++++++++++++++++++

#[test]
fn test_va_len() {
    let mut lengths: [usize; 33] = [0; 33];

    va_expand_more_with_nil!{ ($va_len:tt) ($($va_idents:ident),*) ($($va_indices:tt),*)
        lengths[$va_len] = $va_len;
    }

    for (expected_len, &len) in lengths.iter().enumerate() {
        assert_eq!(expected_len, len);
    }
}

#[test]
fn test_va_indices() {
    let mut indices: [[usize; 33]; 33] = [[0; 33]; 33];

    va_expand_more_with_nil!{ ($va_len:tt) ($($va_idents:ident),*) ($($va_indices:tt),*)
        $(
            indices[$va_len][$va_indices] = $va_indices;
        )*
    }

    for (len, indices) in indices.iter().enumerate() {
        for (expected_idx, &idx) in indices[..len].iter().enumerate() {
            assert_eq!(expected_idx, idx);
        }
    }
}

#[test]
fn test_va_idents() {
    let mut idents: [[&'static str; 33]; 33] = [[""; 33]; 33];

    va_expand_more_with_nil!{ ($va_len:tt) ($($va_idents:ident),*) ($($va_indices:tt),*)
        $(
            idents[$va_len][$va_indices] = stringify!($va_idents);
        )*
    }

    for (len, idents) in idents.iter().enumerate() {
        for (expected_idx, ident) in idents[..len].iter().enumerate() {
            assert!(ident.starts_with("_Va"));
            let idx = ident
                .split("_Va")
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            assert_eq!(expected_idx, idx);
        }
    }
}

#[test]
fn test_dummy_identifier_syntax() {
    let mut i = 0;

    va_expand!{ _foo ($va_len:tt) ($($va_idents:ident),*) ($($va_indices:tt),*)
        i += $va_len;
    }

    va_expand_more!{ _asf ($va_len:tt) ($($va_idents:ident),*) ($($va_indices:tt),*)
        i += $va_len;
    }

    va_expand_with_nil!{ _gfi ($va_len:tt) ($($va_idents:ident),*) ($($va_indices:tt),*)
        i += $va_len;
    }

    va_expand_more_with_nil!{ _nmb ($va_len:tt) ($($va_idents:ident),*) ($($va_indices:tt),*)
        i += $va_len;
    }

    let expected_i = (1..13).fold(0, |acc, next| acc + next) * 2 +
                     (1..33).fold(0, |acc, next| acc + next) * 2;
    assert_eq!(expected_i, i);
}
