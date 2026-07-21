build:
  scripts/tailwind_build.sh
  cargo build --release
  
run: build
  cargo run --release

enable_service:
  cp contrib/lbs.service ~/.config/systemd/user/lbs.service
  systemctl --user enable --now lbs

deploy_service:
  cargo install --path .
  systemctl --user restart lbs
  
