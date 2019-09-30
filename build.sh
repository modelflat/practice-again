# builds project and copies all produced executables into root
cargo test --lib && cargo clean && cargo build --release 

for file in `find target/release -perm +111 -type f -depth 1` ; do
	cp "$file" .
done

