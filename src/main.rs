use cardinal::{Scene, scene, Component, component::text::text, start, setup, Spawner};

#[tokio::main]
async fn main() {
	let s = Spawner::new();
	let mut scene = vec![

	];
	let setup = setup("test").await;
	start(setup, scene).unwrap();
}