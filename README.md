# baby-sleeper

`baby-sleeper` is a small Rust CLI that loops `shh.mp3` and tracks two durations:

- time until the baby falls asleep
- time the baby stays asleep

## Run

The repo includes `shh.mp3`, a short loopable audio file.

```sh
cargo run
```

## Controls

- `s`: mark the baby asleep and start the sleep stopwatch
- `q`: finish the session and stop playback
- `Ctrl-C`: finish the session when captured by the terminal input loop

## Example Output

```text
Baby fell asleep after 00:09:36 (13:35)
Baby slept for 01:14:00 (14:49)
```
