use shared::Clip;

fn main() {
    let test_clip = Clip::new("Hello from CLI!");
    println!("Plucked: '{} @ {}'", test_clip.content, test_clip.timestamp);
}
