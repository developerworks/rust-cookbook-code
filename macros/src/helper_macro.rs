//// Definitions in the `helper_macro` crate.
#[macro_export]
macro_rules! helped {
    // () => { helper!() } // This might lead to an error due to 'helper' not being in scope.
    () => {
        $crate::helper!()
    };
}

#[macro_export]
macro_rules! helper {
    () => {
        println!("helper");
    };
}


// std::ops::Fn
// pub trait Fn<Args>: FnMut<Args> {
//     extern "rust-call" fn call(&self, args: Args) -> Self::Output;
// }

// // std::ops::FnMut
// pub trait FnMut<Args>: FnOnce<Args> {
//     extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
// }

// // std::ops::FnOnce
// pub trait FnOnce<Args> {
//     type Output;
// pub trait FnOnce<Args> {
// pub trait FnMut<Args>: FnOnce<Args> {    
// pub trait Fn<Args>: FnMut<Args> {    
//     extern "rust-call" fn call_once(self,      args: Args) -> Self::Output;
//     extern "rust-call" fn call_mut (&mut self, args: Args) -> Self::Output;
//     extern "rust-call" fn call     (&self,     args: Args) -> Self::Output;
// }

