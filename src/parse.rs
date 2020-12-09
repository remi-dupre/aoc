// Call `apply` macro on this generated form of token tree;
// $ctx, { day DAY { gen GENERATOR { { sol SOLUTION } { sol SOLUTION } } } }
#[macro_export]
macro_rules! parse {
    // Read day
    (
        day $apply: ident, $ctx: tt, $val: expr;
        $day: ident : $generator: ident => $( $tail: tt )*
    ) => {
        $crate::parse!( sol $apply, $ctx, $val; { day $day { gen $generator { } } }; $( $tail )* )
    };

    // Empty rules
    ( day $apply: ident, $ctx: tt, $val: expr; ) => {};

    // Read solution
    (
        sol $apply: ident, $ctx: tt, $val: expr;
        { day $day: tt { gen $generator: tt { $( $acc: tt )* } } } ;
        $sol: ident $( $tail: tt )*
    ) => {
        $crate::parse!(
            post_sol $apply, $ctx, $val;
            { day $day { gen $generator { $( $acc )* { sol $sol } } } };
            $( $tail )*
        )
    };

    // After solution: there is new solutions
    (
        post_sol $apply: ident, $ctx: tt, $val: expr;
        $curr: tt ; , $( $tail: tt )*
    ) => {
        $crate::parse!( sol $apply, $ctx, $val; $curr; $( $tail )* )
    };

    // After solution: end of day
    (
        post_sol $apply: ident, $ctx: tt, $val: expr;
        $curr: tt ; ; $( $tail: tt )*
    ) => {{
        $val.push($apply!{ $ctx, $curr });
        $crate::parse!( day $apply, $ctx, $val; $( $tail )* );
    }};

    // Initialization
    ( $apply: ident $ctx: tt; $( $tt: tt )* ) => {{
        let mut val = Vec::new();
        $crate::parse!( day $apply, $ctx, val; $( $tt )* );
        val
    }};
}

// Extract day names from a parsed token tree
#[macro_export]
macro_rules! extract_day {
    ({}, { day $day: ident $other: tt }) => {
        stringify!($day)
    };
}
