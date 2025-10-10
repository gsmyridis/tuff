pub struct ProfileStat {
    label: &'static str,

    metric_type: Metric,
    metric_inclusive: u64,
    metric_exclusive: u64,

    hit_count: u64,
}

pub struct ProfileReport {
    stats: Vec<ProfileStat>,
}
