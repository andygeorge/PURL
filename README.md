# PURL
`curl` in PERL.

## Installation

```sh
git clone "https://github.com/andygeorge/PURL.git"
```

## Usage

```
Usage: ./PURL/PURL.pl <URL>
```

eg:
```sh
cd PURL
./PURL.pl "https://example.com"
```

## Upgrading

```sh
rm -rf PURL
git clone "https://github.com/andygeorge/PURL.git"
```

## License

None.

# Using a container

```sh
podman run --rm "ghcr.io/andygeorge/purl:latest" "http://example.com"
```
