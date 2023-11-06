use bevy::prelude::*;
pub mod test;

fn main() {
    let mut juicebox: App = App::new();
	juicebox.add_plugins((DefaultPlugins, test::HelloWorld));
	juicebox.run();
}