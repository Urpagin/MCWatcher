# MCWatcher

Probes a Minecraft server at regular intervals to collect comprehensive data for later analysis and visualization.

## Quickstart ⚡

1. Set the `MC_ADDR` environment variable in a `.env` file at the root of the project as follows:

> [!TIP]
> Copy-paste the `.env.example` file into a `.env` file and modify its contents.

```bash
MC_ADDR=mc.hypixel.net
```

2. Compile and run the program with:

```bash
cargo run
```

Let the program probe the server. Once sufficient data has been collected, stop the program and analyze the generated `data.csv` file.

## Docker Compose 🐳

1. Populate the `.env` file just as described in the [Quickstart](#quickstart) heading.

2. Launch the program:

```bash
docker compose up -d
```

3. All done. A `./data` directory will be created with `data.csv` inside. To follow the container logs:

```bash
docker compose logs -f
```

## Data format 📊

The `data.csv` file is appended every 30 seconds and structured as follows:

| MC Address | Timestamp Probed | Latency | Version | Max Players | Online Players | Sample Players |
| ---------- | ---------------- | ------- | ------- | ----------- | -------------- | -------------- |

The `Sample Players` column lists entries as `name@uuid` separated by `!` with no trailing delimiter, like this:

```
<player_name>@<player_uuid>!<player_name>@<player_uuid>!<player_name>@<player_uuid>
```

**Example:**

```
Technoblade@b876ec32-e396-476b-a115-8438d83c67d4!popbob@0f75a81d-70e5-43c5-b892-f33c524284f2!Notch@069a79f4-44e9-4726-a5be-fca90e38aaf5
```

## TODO / Ideas 🧠

* Support for probing multiple servers.
* API endpoint for dynamically adding servers.
* Simple frontend for managing servers, downloading data, and generating graphs.
* Use Parquet or an SQL database for data storage.
* Configurable probe interval via `.env` file.
