# NanoHue

A rust-based tool to mirror Philips Hue scenes to a Nanoleaf device.

## Getting Started

1. Build the docker container.

```console
docker build . -t nanohue
```

2. Create a configuration file. For this, you will need to know the IP addresses for both your Hue bridge and the Nanoleaf device, as well as the API tokens for both.

```yaml
hue:
  host: "ip_address" # IP address of the hue bridge
  group: "group_name" # Name of your Hue room/group.
  username: "token" # Hue username token
  client_key: "token" # Hue client key.

nanoleaf:
  host: "ip_address" # IP address of the nanoleaf device
  token: "token" # Nanoleaf API token
  max_brightness: 40 # Maximum allowed brightness (0-100) for the nanoleaf.
```

3. Run the container!

```console
docker run --mount type=bind,source="$(pwd)"/config.yml,target=/nanohue/config.yml,readonly  nanohue
```
