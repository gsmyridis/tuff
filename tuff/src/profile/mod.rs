pub mod profiler;
pub use profiler::{CallSite, ProfileBlock, Profiler};

#[macro_export]
macro_rules! profile_block {
    // Specify the label and anchor index
    ([label=$label:literal, id=$index:expr] $($body:tt)*) => {
        $crate::paste::paste! {
            let [<__profile_block _ $label _ $index>] = $crate::ProfileBlock::new($label, $index);
            $($body)*
            drop([<__profile_block _ $label _ $index>]);
        }
    };

    // Specify only the label
    ([$label:literal] $($body:tt)*) => {
        let __idx = {
            static CALLSITE_ID: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
            *CALLSITE_ID.get_or_init(|| $crate::Profiler::next_id())
        };

        $crate::paste::paste! {
            let [<__profile_block _ $label>] = $crate::ProfileBlock::new($label, __idx);
            $($body)*
            drop([<__profile_block _ $label>]);
        }
    }
}
