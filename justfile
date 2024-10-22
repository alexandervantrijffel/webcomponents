pnpm-dev:
	(cd crates/lib/webcomponents && pnpm i)
	
serve-watch *ARGS:
  systemfd --no-pid -s http::0.0.0.0:8181 -- cargo watch -q -c --ignore '**/generated_at_build.rs' -w . -x "run --all-features -p serve {{ARGS}}"

webcomponents-test:
  cargo watch -c --ignore '**/generated_at_build.rs' -w . -x "nextest run --all-features --verbose -p webcomponents"
