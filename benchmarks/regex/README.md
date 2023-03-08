This regular expression benchmark measures the time taken to match e-mails,
URIs, and IP addresses in an input text. It is written in Rust and can be
re-compiled using the `Dockerfile` (see [`build.sh`]).

It is significantly adapted from the [regex-benchmark] project, which is [MIT]
licensed. The [`default.input`] text is a concatenation of [Learn X in Y
minutes] which is published under the [Creative Commons Attribution-ShareAlike
3.0 Unported] license.

[`build.sh`]: ../build.sh
[regex-benchmark]: https://github.com/mariomka/regex-benchmark
[MIT]: https://github.com/mariomka/regex-benchmark/blob/master/LICENSE
[`default.input`]: ./default.input
[Learn X in Y minutes]: https://github.com/adambard/learnxinyminutes-docs
[Creative Commons Attribution-ShareAlike 3.0 Unported]: http://creativecommons.org/licenses/by-sa/3.0/deed.en_US
