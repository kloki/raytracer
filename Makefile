default:
	cargo build --release
	./target/release/raytracer
	display output.ppm


