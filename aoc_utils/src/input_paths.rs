pub fn make_sample_path(day_name: i32, samle_index: i32) -> String {
    assert!(day_name >= 0 && day_name < 100);
    assert!(samle_index >= 0 && samle_index < 100);
    format!("inputs/sample/{:02}.{:02}.txt", day_name, samle_index)
}

pub fn make_real_path(day_name: i32) -> String {
    assert!(day_name >= 0 && day_name < 100);
    format!("inputs/real/{:02}.txt", day_name)
}
