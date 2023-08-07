# [Grande Sonnerie](https://en.wikipedia.org/wiki/Repeater_(horology))

The goal of this app is to provide hour and minute repeater on your PC in the
same way that is found in such watches as Casio (everyone knows their hourly
beep-beep), [Vacheron Constantin](https://youtu.be/rgqcd2mTBUM?t=25) and [Patek
Phillippe](https://youtu.be/SGPjFFMD3c0?t=573).

## Installation

```console
$ cargo install grande-sonnerie
```

## Usage

### Configuration

Upon first launch two default config files will be created in
`~/.config/grande-sonnerie`: `config.toml` and `casio.toml`, as well as
a directory with default `coucou` [movement](https://youtu.be/V3HN9V1C8jE).

#### `config.toml`

* `offset` is responsible for setting timezone
* `sonnerie` will load sounds from the directory of the same name
* `movement` will load configuration of the same name for setting chimes

#### `casio.toml`

* `grand` is a list of integers on which `grand` will chime
* `hours` is a list of integers that defines hours on which they will be
  repeated
* `hours_div` is an integer which allows equally splitiing hours to chime.
  `hours_div: 3` will make hours chime every third hour
* `minutes` similar to `hours`
* `minutes_div` similar to `hours_div`, but splits hour into equal parts.
  `minutes_div: 15` will make minutes chime every quarter-hour
* `twelve_hour` 13th hour will chime one time instead of thirteen etc
  (recommended with `multichime: true`)
* `multichime` will make sounds repeat according to hour/minute, see examples
  below

#### Examples

##### Defaults

`config.toml`:

```toml
offset = [0, 0, 0]
sonnerie = 'coucou'
movement = 'casio'
```

`casio.toml`:

```toml
hours_div = 1
twelve_hour = false
multichime = false
```

UTC timezone, chimes once every hour with `coucou/hour.wav`.

##### My personal config

`config.toml:`

```toml
offset = [3, 0, 0]
sonnerie = 'coucou'
movement = 'pp'
```

`pp.toml`:

```toml
grand = [10, 18]
hours_div = 1
minutes_div = 15
twelve_hour = true
multichime = true
```

UTC+3, chimes:
* `coucou/grand.wav` on 10:00 and 18:00 before everything else
* `coucou/hour.wav` every hour, repeats according to the hours in 12h format
* `coucou/minute.wav` every 15 minutes of an hour, repeats according to
  quarters of hour

So at 18:00 it will chime Grand, then Hour six times. At 18:45 it will just
chime Minutes three times (45 is 3/4 of an hour).

### Sounds

Three sounds are expected: `grand`, `hour`, and `minute`. Currently only `.wav`
is supported:

```console
$ ffmpeg -i yoda-death-sound-effect.mp3 grand.wav
```

## To do

- [ ] Implement arrays of sounds to play consecutively (Westminster chimes)
* [ ] Change CPU-eating sound implementation to something more sane
* [ ] Actual repeater (press a button to hear time)

