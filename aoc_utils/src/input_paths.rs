pub fn make_sample_path(day_name: i32, samle_index: i32) -> String {
    assert!((0..100).contains(&day_name));
    assert!((0..100).contains(&samle_index));
    format!("inputs/sample/{:02}.{:02}.txt", day_name, samle_index)
}

pub fn make_real_path(day_name: i32) -> String {
    assert!((0..100).contains(&day_name));
    format!("inputs/real/{:02}.txt", day_name)
}
