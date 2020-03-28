# yesno
CLI to get funny gifs from https://yesno.wtf

## Usage

The tool accepts one argument that should be either `yes`, `no` or `maybe`. It prints a link in
stdout. If run without arguments it prints a gif for randomly chosen answer.

### Examples

- Open in your browser: `yesno yes | xargs open`
- Copy into the clipboard: `yesno maybe | pbcopy`

