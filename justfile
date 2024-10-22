# watch and generate TailWind CSS
pnpm-dev:
	(cd crates/lib/webcomponents && pnpm i && pnpm dev)
	
# watch source files and serve the web app
serve-watch *ARGS:
  systemfd --no-pid -s http::0.0.0.0:8181 -- cargo watch -q -c --ignore '**/generated_at_build.rs' -w . -x "run --all-features -p serve {{ARGS}}"

# watch source files and run all tests
webcomponents-test *ARGS:
  cargo watch -c --ignore '**/generated_at_build.rs' -w . -x "nextest run --all-features --verbose -p webcomponents {{ARGS}}"
