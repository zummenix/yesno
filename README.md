# yesno
CLI to get funny gifs from https://yesno.wtf

## Usage

The tool accepts one argument that should be either `yes`, `no` or `maybe` and prints a link to a
gif in stdout. You can run it without arguments to get a link for a random answer.

### Examples

- Open in your browser: `yesno yes | xargs open`
- Copy into the clipboard: `yesno maybe | pbcopy`

