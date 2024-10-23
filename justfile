# watch and generate TailWind CSS
pnpm-dev:
	(cd crates/lib/webcomponents && pnpm i && pnpm dev)
	
# watch source files and serve the web app
serve-watch *ARGS:
  systemfd --no-pid -s http::0.0.0.0:8181 -- cargo watch -q -c --ignore '**/generated_at_build.rs' -w . -x "run --all-features -p serve {{ARGS}}"

# reduce build time by using the Cranelift compiler backend instead of LLVM
# this improves the build time by 36% on my machine
#
# install Cranelift with 
# `rustup component add rustc-codegen-cranelift-preview --toolchain nightly`
#
# install the nightly toolchain with
# `rustup toolchain install nightly`

# reduce build time by using the Cranelift compiler backend instead of LLVM
serve-watch-cranelift *ARGS:
  rustup override set nightly
  CARGO_PROFILE_DEV_CODEGEN_BACKEND=cranelift systemfd --no-pid -s http::0.0.0.0:8181 --  cargo watch -q -c --ignore '**/generated_at_build.rs' -w . -x "+nightly run -Zcodegen-backend --all-features -p serve {{ARGS}}"

# watch source files and run all tests
webcomponents-test *ARGS:
  cargo watch -c --ignore '**/generated_at_build.rs' -w . -x "nextest run --all-features --verbose -p webcomponents {{ARGS}}"
