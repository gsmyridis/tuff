pub mod profiler;
pub use profiler::{CallSite, ProfileBlock, Profiler};

#[macro_export]
macro_rules! profile_block {
    // Specify the label and anchor index
    ([$label:literal, $index:expr] $($body:tt)*) => {
        ::paste::paste! {
            let [<__profile_block _ $label _ $index>] = $crate::ProfileBlock::new($label, $index);
            $($body)*
            drop([<__profile_block _ $label _ $index>]);
        }
    };

    // Specify only the label
    ([$label:literal] $($body:tt)*) => {
        let __idx = {
            const __CALL_SITE: $crate::CallSite = $crate::CallSite::new(file!(), line!(), column!());
            $crate::Profiler::get_or_insert(__CALL_SITE)
        };

        ::paste::paste! {
            let [<__profile_block _ $label>] = $crate::ProfileBlock::new($label, __idx);
            $($body)*
            drop([<__profile_block _ $label>]);
        }
    }
}
