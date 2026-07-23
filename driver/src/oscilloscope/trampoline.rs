use std::ffi::c_void;

pub unsafe fn split_closure<C, Args, Ret>(closure: &mut C) -> (*mut c_void, C::Trampoline)
where
    C: Split<Args, Ret>,
{
    (closure as *mut C as *mut c_void, C::trampoline())
}

pub trait Split<Args, Ret> {
    type Trampoline;

    fn trampoline() -> Self::Trampoline;
}

macro_rules! impl_split {
    ($( $outer:ident ),* ; $( $inner:ident ),*) => {
        impl<Func, Ret, $($outer),*> Split<($( $outer, )*), Ret> for Func
        where
            Func: FnMut($($outer),*) -> Ret,
        {
            type Trampoline = unsafe extern "C" fn($($outer,)* *mut c_void) -> Ret;

            fn trampoline() -> Self::Trampoline {
                // declare a trampoline function which will turn our pointer
                // back into an `F` and invoke it

                // Note: we're deliberately using `$inner` to generate an ident
                // for the argument
                #[allow(non_snake_case)]
                unsafe extern "C" fn trampoline<T, Ret_, $( $inner ),*>($($inner: $inner,)* ptr: *mut c_void) -> Ret_
                where
                    T: FnMut($($inner),*) -> Ret_,
                {
                    debug_assert!(!ptr.is_null());

                    let callback: &mut T = &mut *(ptr as *mut T);

                    callback($($inner),*)
                }

                trampoline::<Func, Ret, $($outer,)*>
            }
        }
    };
}

impl_split!(;);
impl_split!(A; A);
impl_split!(A, B; A, B);
impl_split!(A, B, C; A, B, C);
impl_split!(A, B, C, D; A, B, C, D);
impl_split!(A, B, C, D, E; A, B, C, D, E);
impl_split!(A, B, C, D, E, F; A, B, C, D, E, F);
impl_split!(A, B, C, D, E, F, G; A, B, C, D, E, F, G);
impl_split!(A, B, C, D, E, F, G, H; A, B, C, D, E, F, G, H);
impl_split!(A, B, C, D, E, F, G, H, I; A, B, C, D, E, F, G, H, I);
impl_split!(A, B, C, D, E, F, G, H, I, K; A, B, C, D, E, F, G, H, I, K);
impl_split!(A, B, C, D, E, F, G, H, I, K, L; A, B, C, D, E, F, G, H, I, K, L);
impl_split!(A, B, C, D, E, F, G, H, I, K, L, M; A, B, C, D, E, F, G, H, I, K, L, M);
impl_split!(A, B, C, D, E, F, G, H, I, K, L, M, N; A, B, C, D, E, F, G, H, I, K, L, M, N);
impl_split!(A, B, C, D, E, F, G, H, I, K, L, M, N, O; A, B, C, D, E, F, G, H, I, K, L, M, N, O);
