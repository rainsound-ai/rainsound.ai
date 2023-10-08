pub fn map_filename_without_extension(
    input: impl Into<String>,
    map: impl FnOnce(&str) -> String,
) -> String {
    map_last_segment(input, |filename| {
        let mut split = filename.split('.');
        let filename_without_extension = split.next().unwrap();
        let extension = split.next().unwrap();

        let new_filename_without_extension = map(filename_without_extension);

        format!("{}.{}", new_filename_without_extension, extension)
    })
}

fn map_last_segment(input: impl Into<String>, map: impl FnOnce(&str) -> String) -> String {
    let input = input.into();
    let mut segments = input.split('/').map(String::from).collect::<Vec<_>>();
    let index = segments.len() - 1;
    let segment = &segments[index];
    let mapped_segment = map(segment);
    segments[index] = mapped_segment;
    segments.join("/")
}
