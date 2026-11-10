### catalog-cluster

This application is responsible for clustering and indexing records into the Princeton University Library catalog.


### Local development

1. Install devbox
1. `devbox shell`
1. `devbox setup`
1. `cargo run`

#### Running using a different port

Sometimes you will already have a postgres running on port 5432.  In these cases, you can run postgres on a
different port:
1. `devbox shell` (if you are not already in a shell)
1. `devbox services down` (if it is already running)
1. `export PGPORT=4321`
1. `devbox setup`
1. `cargo run`
