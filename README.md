# What is stackcheck?

Stackcheck is a cli tool to get meta data about any project, just point it to a project directory and it will attempt to get as many meta data from it as possible.

You can use the `--mode` flag to control how much meta data to return.

- `--mode=low`: return programming language, framework used (if any), and their respective versions.
- `--mode=medium`: return more info about packages.
- `--mode=hight`: return as much data as possible.

You can use the `--output` flag to set where you want to output the results.

- `--output=console`: output to stdout in a human readable format.
- `--output=json`: output to json file.
- `--output=yaml`: output to yaml file.
- `--output=html`: output to html page.
- `--output=pdf`: output to pdf file.

## Roadmap

Stackcheck currently supports extracting metadata from projects written in:

- [ ] Laravel
- [ ] Django
- [ ] Nest.js
- [ ] Express.js
- [ ] Next.js
- [ ] Nuxt
- [ ] Sveltekit
- [ ] Remix
