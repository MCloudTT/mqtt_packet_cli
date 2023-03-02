# mqtt_packet_cli
A small tool to test packets against an MQTT broker.

## How to use
Provide packets in hexadecimal form and use the following flags:

| Flag            | Description                                                               |
|-----------------|---------------------------------------------------------------------------|
| `-p --packet` | Used to send a single packet to the broker                                |
| `-f --file`   | Send a sequence of packets from a file (each line is treated as a packet) |
| `--host`        | Set broker host. Defaults to `localhost`                                  |
| `--port`        | Set broker port. Defaults to 1883                                         |
