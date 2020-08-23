# arbron-queryer

A service of `SOMBRA::arbron`, whose job is to basically act as a proxy for [VirusTotal](https://www.virustotal.com) as well as handling the whole shady key carousel thing.

## Running

Before running, make sure to refer to the **Options** section below, as well as creating 

### Directly

This service is a [Rust](https://www.rust-lang.org/) program, and can be run like so:

```bash
git submodule update --init --recursive
cargo run --release
```

### Dockerfile

There is also a [Dockerfile](https://www.docker.com/) that can be built and run.

```bash
git submodule update --init --recursive
docker build . -t arbron-queryer
docker run --expose 1542 -v keys.txt:/keys.txt arbron-queryer
```

## Keys

Half the point of the system is to somewhat bypass the rate limit imposed by the API through the use of multiple keys, to a reasonable extent of course. As of the time of writing this README, the rate limit imposed by the API is 4 requests per minute.

The service cycles keys based on a timeout that can be changed in the **Options** below. As it stands, the timeout of each key is updated once per second, to implement a somewhat reasonable rate limit of our own.

Due to the aforementioned rate limit, the optimum number of keys one can source is 15-16. Any less and there will be periods of downtime in the querying, any more and some keys will never get used.

Keys are supplied to the service in a file. Put each key on it's own line, with only the new lines as delimiters. The location of this keys file can then be supplied in **Options**, described below.

VirusTotal API keys can be attained by signing up for an account [here](https://www.virustotal.com/gui/join-us).

## Options

Options are supplied to the service via environment variables. There is also facility to load a `.env` file.

| Name | Default | Description |
| ---- | ------- | ----------- |
| RUST_LOG | errror | Log level. Can be `debug`, `trace`, `info`, `debug`, `warn` or `error` |
| KEYS_FILE | ./keys.txt | File path to the file containing the keys. See above section **Keys** for more information. |
| TIMEOUT | 15 | Timeout (in seconds) for an individual key. See above section **Keys** for more information. |
| LISTEN | 0.0.0.0:1539 | The address for this service to listen on. |


## Usage

This service is a [Cap'n Proto](https://capnproto.org/) RPC service, implementin the `HashQuery` interface. More information can be found in the [arbron-protos](https://git.octalorca.me/sombra/arbron/protos) README.
