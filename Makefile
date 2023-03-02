default:
	cargo build --release
	./target/release/raytracer > output.ppm
	display output.ppm


