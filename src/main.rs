use cardinal::{CardinalResult, component::text::text, start};

#[tokio::main]
async fn main() -> CardinalResult<()> {
	let scene = vec![
		text("asdf")
	];
	start("Testing window", scene).await
}