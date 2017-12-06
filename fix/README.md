
# fix

Holds the model, parsing and generation code, exposed as a lib. Mostly separated because the model/parsing/gen are huge and compilation times become an issue.


Useful:

> cargo bench --features fix42  # runs the benchmarks

> cargo test --features fix42  -- --nocapture  # runs the tests with fix42 enabled


