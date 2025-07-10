# MCWatcher
Probes a Minecraft server at regular intervals to collect comprehensive data for later analysis and visualization.


# Quickstart

1. Set the `MC_ADDR` environment variable in a `.env` file.

2. `cargo run`

The `data.csv` file will be populated every 30 seconds like so:

| Month    | Savings |
| -------- | ------- |



# TODO / Ideas

* Add multiple server support
* API to add servers.
* Simple frontend? To add/remove servers, download data, generate graphs.
* Use parquet or SQL file.
* Interval in .env

