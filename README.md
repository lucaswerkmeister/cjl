# check JSON lines

A simple program to check for errors in a JSON stream.

Reads a stream of JSON values, one per line, from standard input.
(This format is variously known as line delimited JSON (ldjson),
[newline delimited JSON (ndjson)][ndjson],
or [JSON lines (jsonl)][jsonl].)
When a line cannot be parsed as valid JSON,
prints a description of the parse error,
followed by the erroneous line with some context
(previous four lines and next line, if any).

Built for investigating [T276643][], but may also be useful elsewhere.

## License

Published under the GNU Affero General Public License, version 3 or later.

[ndjson]: http://ndjson.org/
[jsonl]: http://jsonlines.org/
[T276643]: https://phabricator.wikimedia.org/T276643
