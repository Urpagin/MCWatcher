# MCWatcher

Probes a Minecraft server at regular intervals to collect comprehensive data for later analysis and visualization.

## Quickstart

1. Set the `MC_ADDR` environment variable in a `.env` file at the root of the project as follows:

> [!TIP]
> Copy-paste the `.env.example` file into a `.env` file and modify the content of the latter.

```bash
MC_ADDR=mc.hypixel.net
```

2. Compile and run the program with:

```bash
cargo run
```

Allow the program to probe the server. Once sufficient data has been collected, stop the program and interpret the results in the generated `data.csv` file.

## Docker Compose

1. Populate the `.env` file just as described in the [Quickstart](#quickstart) heading


2. Launch the program
```bash
docker compose up -d
```

3. All done. A `./data/` directory will be made with the `data.csv` inside. To interactively inspect the logs of the container, do:
```bash
docker compose logs -f
```

## Data format

The `data.csv` file is appended every 30 seconds and structured as follows:

| MC Address | Timestamp Probed | Latency | Version | Max Players | Online Players | Sample Players |
| ---------- | ---------------- | ------- | ------- | ----------- | -------------- | -------------- |

The `Sample Players` column is formatted by separating the player's name and UUID with an `@` symbol, and individual players with a `!` symbol, like so:

```
<player_name>@<player_uuid>!<player_name>@<player_uuid>!
```

**Example:**

```
Technoblade@b876ec32-e396-476b-a115-8438d83c67d4!popbob@0f75a81d-70e5-43c5-b892-f33c524284f2!Notch@069a79f4-44e9-4726-a5be-fca90e38aaf5
```

## TODO / Ideas

* Support for probing multiple servers.
* API endpoint for dynamically adding servers.
* Simple frontend for managing servers, downloading data, and generating graphs.
* Use Parquet or an SQL database for data storage.
* Configurable probe interval via `.env` file.
