build:
  scripts/tailwind_build.sh
  cargo build --release
  
run: build
  cargo run --release
  
