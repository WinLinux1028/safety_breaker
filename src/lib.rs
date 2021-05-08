/// # Add forcemut() method for all types.
/// ## Example
/// ```
/// use safety_breaker::ForceMut;
///
/// fn main() {
///     let a = String::from("Hello, Rustaceans!"); //not mutable
///     println!("{}", &a); //Hello, Rustaceans!
///     unsafe {
///         let b = a.forcemut(); //b is mutable reference
///         *b = String::from("Hello, unsafe!");
///         println!("{}", &a); //Hello, unsafe!
///     }
/// }
/// ```
#[allow(clippy::needless_doctest_main)]
#[allow(clippy::mut_from_ref)]
#[allow(clippy::needless_lifetimes)]
pub trait ForceMut {
    /// Converts an immutable reference to a mutable reference.
    /// ## Safety
    /// Don't use to immutable global variables.  
    /// It'll crash your software.  
    unsafe fn forcemut<'a>(&'a self) -> &'a mut Self;
}

#[allow(clippy::needless_lifetimes)]
#[allow(clippy::cast_ref_to_mut)]
impl<T: ?Sized> ForceMut for T {
    #[inline(always)]
    unsafe fn forcemut<'a>(&'a self) -> &'a mut Self {
        &mut *(self as *const _ as *mut Self)
    }
}

/// # Changes the type of a reference to another type
/// ## Example
/// ```
/// #[macro_use]
/// extern crate safety_breaker;
///
/// fn main() {
///     let a: u32 = 65; //In ASCII, 65 (decimal) means 'A'.
///     unsafe {
///         let b = force_convert!(&a, char); //b is &char
///         println!("{}", b); //'A' will be displayed.
///     }
/// }
/// ```
/// ## How to use
/// The first argument is a reference to the variable which you want to change its type.
/// The second argument is the type of the conversion target.
/// This macro must be used in an unsafe block.
/// This macro can also be used to change the type parameters and the lifetime annotations
#[macro_export]
macro_rules! force_convert {
    ($x:expr, $y:ty) => {
        &*($x as *const _ as *mut $y)
    };
}
