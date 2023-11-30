# create-parcel-plugin

Creates a Parcel plugin scaffold for custom Parcel plugins.

Flow:

- What kind of plugin? (Select/type one)
- (Optional) Specify a namespace for this plugin.
  - blurb
- Please provide a name for this plugin @parcel/<plugin-type>-<your-name>:
  - Confirmation with new plugin name?
- Should this use the default plugin?
- js/ts/flow

Later:

- Should we update your .parcelrc?
- adding to parcelrc is left as an exercise to the reader

## Usage

```
Usage: create-parcel-plugin [NAME] <COMMAND>

Commands:
  namer        Create namer
  optimizer
  packager
  reporter
  resolver
  transformer
  bundler
  help         Print this message or the help of the given subcommand(s)

Arguments:
  [NAME]  Name of plugin

Options:
  -h, --help     Print help
  -V, --version  Print version
```
