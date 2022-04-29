# wichtel-randomizer
![Build](https://github.com/engelphi/wichtel-randomizer/actions/workflows/build.yaml/badge.svg)

Very simple randomizer for Schrottwichteln

# Instructions

Simply run:
```
cargo run -- -i input.json
```

where input.json is a json file with the following format:
```
{
    "persons": ["nameA", "nameB",....]
}
```

Optionally one can write the results to a file instead of the commandline like this:
```
cargo run -- -i input.json -o output.json
```

For the full list of available commandline arguments run:
```
cargo run -- -h
```


