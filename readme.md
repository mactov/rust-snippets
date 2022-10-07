# template-engine
This is a simple example of rust for those of you who, like me, are starting with this great language. I have tried to keep it very simple and it is not "bullet-proof" (aka as prod-ready). The only intention is help people with some very simple regex, string substitution with a functional example. Do not hesitate to create pull requests to improve it.

The program reads a template file searching for {{ var }} and replaces {{ var }} occurences by the values that were given as arguments. Make sure you provide all values with the same spelling or the progrgam will panic.

## Usage

When in the template-engine directory, just type the following command
```
cargo run template.txt name Chris location Paris
```
and you should see this
```
- Hi from Paris. My name is Chris.
- Hi Chris, nice to meet you!
```
