# Stats

Stats is a high-performance, standalone analytics provider designed for self-hosting, enabling the collection and viewing of event data from websites, apps, and more through a web API. It comes with a real-time dashboard and is ideal as a minimal and privacy-preserving analytics solution for small to medium traffic services.

**`Current status: it works, but it's not finished.`**

![Stats dashboard on iPad](./preview.png)

## Key-features

- Real-time analytics and dashboard
- Lightweight and efficient, with minimal resource usage
- Easy integration with websites, mobile apps, and other platforms

# Local development

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) - V1.77>
- [Diesel CLI](https://diesel.rs/guides/getting-started)

## Steps

1. Create a `.env` file following the example provided by `.env.example`. See [Configuration](#configuration) for details on the values.

2. Run the stored migrations: `diesel migration run`

3. For event location data you will need to add the [GeoLite2-City.mmdb](https://git.io/GeoLite2-City.mmdb) & [cities5000](ttps://github.com/PrismaPhonic/filter-cities-by-country/raw/master/cities5000.txt) files to `/data`. See [Setup](#setup) for minimal file structure.

4. `cargo run`

# Getting started

Stats is ran as an executable hosted on a server of your choice designed to for enhanced portability and scalability.


**Build production release**

```
cargo build --release
```

This command will create a production ready executable in the `/target/release` folder.

With the executable running at an accessible address you can collect events by embedding the associated JS script into your website. This script can be targetted at `http(s)://<ADDRESS_OF_HOSTED_STATS>/stats.js`. The below example shows how this can be embedded in the simplest form:

```js
<script>
  // Stats analytics var head = document.head ||
  document.getElementsByTagName("head")[0]; var script =
  document.createElement("script"); script.setAttribute("src",
  "http://localhost:5775/stats.js"); // REPLACE WITH ACTUAL URL
  script.setAttribute("onload", () => window.collectStats());
  script.setAttribute("type", "text/javascript"); script.setAttribute("charset",
  "utf8"); script.setAttribute("async", ""); head.appendChild(script);
</script>
```

> Be sure to add the address of the website to the `CORS_DOMAINS` variable on stats such that incoming requests are permitted.

## Setup

Minimum set of folders & files required to run this application.

```
stats/
├── data/
│ ├── GeoLite2-City.mmdb
│ ├── cities5000.txt
│ └── stats.sqlite
├── ui/
├── stats // copy executable from target/release/stats
└── .env
```

## Toggling stats

The collection of events can be enabled/disabled on a browser level. Appending `#toggle-stats` to any URL of the website will enable or disabled collection. An alert will be sent to the browser informing the user of the current configuration. This is stored in local storage so is valid as long as storage is not cleaned.

# Configuration

These options must be defined in a `.env` file before starting the server.

| Variable              | Default                                | Summary                                                                                                                                                                      |
| --------------------- | -------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| APP_URL               | http://localhost:5775                  | Full domain you are hosting this service on                                                                                                                                  |
| SERVICE_PORT          | 5775                                   | Port you want the service to be hosted from                                                                                                                                  |
| DATABASE_URL          | /data/stats.sqlite                     | Path to .sqlite file to use as database.                                                                                                                                     |
| CORS_DOMAINS          | http://localhost:5775,https://udara.io | Comma-separated list of allowed domains. The service will only accept analytics events from these domains.                                                                   |
| PROCESSING_BATCH_SIZE | 500                                    | Max limit for events buffer used to queue and batch analytics events for processing. When the limit is hit, new events are dropped until items are processed from the queue. |

# Docker

The contained [Dockerfile](.Dockerfile) can be used to run Stats as an image. An associated volume mount can be used to persist data across containers.

1. Build the docker image: `docker build . -t stats:latest`

2. Run the container: `docker run -d --name stats -p 5775:5775 stats:latest`

> Assuming the `SERVICE_PORT` is set to 5775 then the above command will expose this port to it's host machine - <host>:<container>. Change this if the underlying port has been altered or you pass an alternative e.g. `-e SERVICE_PORT='9001'`.

3. Alternatively run with a volume (to persist data): `docker run -d --name stats -p 5775:5775 --mount source=stats,target=/app/data stats:latest`

> The above command will mount a Docker volume named _stats_ to the `/app/data` directory of the container. As a result this folder, which contains the SQLite DB, will be persisted.
