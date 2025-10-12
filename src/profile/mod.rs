pub mod profiler;
pub use profiler::{ProfileBlock, Profiler};

#[macro_export]
macro_rules! profile_block {
    ([ $label: literal, $index:expr ] $($body:tt)*) => {
        ::paste::paste! {
            let [<__profile_block _ $label _ $index>] = $crate::ProfileBlock::new($label, $index);
            $($body)*
            drop([<__profile_block _ $label _ $index>]);
        }
    };
}
