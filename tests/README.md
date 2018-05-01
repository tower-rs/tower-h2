# tower-h2 integration tests

This is a separate crate because it uses a git dependency on h2-support (from
the h2 crate). When deploying to crates.io, there cannot be any git
dependencies. This applies to dev dependencies as well.
