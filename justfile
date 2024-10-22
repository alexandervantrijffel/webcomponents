serve-watch:
  systemfd --no-pid -s http::0.0.0.0:8181 -- cargo watch -q -c --ignore '**/generated_at_build.rs' -w . -x "run --all-features --timings -p serve"

