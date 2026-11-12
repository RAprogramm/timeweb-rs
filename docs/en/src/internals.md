# Architecture

The SDK has three layers. Only the middle one is generated.

<figure class="diagram">
  <a href="img/layers.svg" target="_blank" rel="noopener">
    <img src="img/layers.svg" width="552" alt="The vendored specification goes through the xtask pipeline into the generated layer; the hand-written layer is added on top of it, and together they are the crate.">
  </a>
  <figcaption>The three layers. Click the diagram to open it full size.</figcaption>
</figure>

## The generated layer

`src/apis` and `src/models` come out of `openapi-generator` with the `rust`
generator and the `reqwest` library. The generator is pinned to a version in
`openapitools.json`, so a regeneration produces the same code.

Every operation is a free async function that takes a `Configuration` and
returns `Result<T, Error<E>>`. Every model is a `serde` struct with the
constructors the generator emits.

## The normalizer

The vendor's specification is not quite what the generator expects. The
normalizer is a set of passes that reconcile it before the generation:

- **Path parameters.** A parameter declared `in: path` whose name has no
  `{placeholder}` in the route makes the generator emit an unused format
  argument; a `{placeholder}` without a declared parameter makes it reference an
  undefined variable. The passes reconcile the two, and add the missing
  declarations.
- **Tags.** Every tag is written in Russian, which the generator cannot turn
  into Rust module names — all operations would collapse into a single
  `default_api` module. The English name of every tag is carried in
  `x-name-i18n`, and the passes put it in place.
- **Unions.** The generator renders a union of a string and a list as an empty
  struct, and names the variants of a union after the item type, which produces
  identifiers like `ArrayVecstd::collections::HashMap(...)`. The passes rewrite
  the shapes the generator cannot render.
- **Responses.** `response_id` is a required `uuid` upstream, while the API
  returns `null` on some endpoints; several documented bodies contradict their
  own schema. The passes relax those schemas.

The normalizer is idempotent: running it twice produces the same document.

## The model patcher

The generator follows the specification to the letter, and the live API is
more forgiving than the specification. The patcher repairs the difference in
the generated models:

- **Nullable fields.** A nullable property the generator emits as
  `Option<T>` with `deserialize_with = "Option::deserialize"` is still
  *mandatory* for `serde`: a response that omits the field fails with "missing
  field `x`". The patcher adds `default`, so an absent field deserializes to
  `None` while a present `null` keeps mapping to `None`.
- **Timestamps.** The API sends some timestamps without a UTC offset, which the
  strict `chrono::DateTime<FixedOffset>` deserializer rejects. The patched
  fields use the tolerant deserializers of the crate, which accept both
  shapes.
- **The panic.** The generated `parse_deep_object` aborts the process when it
  is handed a value that is not an object. The patched version returns an
  empty list.

## The hand-written layer

Five small modules carry everything the generator cannot:

| Module | What it does |
| --- | --- |
| `client.rs` | The constructors: the base URLs, the token, the timeout. |
| `retry.rs` | `TimewebClient`, `RetryPolicy`, the jitter and `Retry-After`. |
| `pagination.rs` | `paginate`, the stream over the offset pagination. |
| `error_body.rs` | `ErrorDetails`, the uniform envelope of the API. |
| `datetime.rs` | The deserializers that accept a timestamp with or without an offset. |

A call goes through those modules in a straight line: the client decides whether
a failure is worth another attempt, the generated operation builds the request,
and `reqwest` carries it to the API.

<figure class="diagram">
  <a href="img/request.svg" target="_blank" rel="noopener">
    <img src="img/request.svg" width="407" alt="Your code hands a closure to TimewebClient.execute, the retry policy decides whether to wait and retry, the generated operation sends the request to api.timeweb.cloud through reqwest.">
  </a>
  <figcaption>The path of one retried call.</figcaption>
</figure>

## The pipeline

The whole generation is reproducible and lives in the `xtask` crate:

```sh
cargo xtask normalize-spec openapi/timeweb-cloud.json /tmp/normalized.json
# … openapi-generator …
cargo xtask layout
cargo xtask feature-gates
cargo xtask patch-models
cargo xtask example-tests
cargo xtask api-index
cargo xtask readme
cargo +nightly fmt --all
```

The CI checks that the committed code matches the pipeline on every pull
request, so a hand edit of the generated code cannot slip through.

## The weekly sync

A scheduled workflow compares the vendored specification with the upstream
bundle. When the API changes, it regenerates the client, summarizes the API
diff with `oasdiff`, opens a pull request and merges it once CI is green —
unless `cargo-semver-checks` finds breaking changes, in which case the pull
request stays open for review.

<figure class="diagram">
  <a href="img/sync.svg" target="_blank" rel="noopener">
    <img src="img/sync.svg" width="463" alt="Every Monday the upstream specification is compared with the vendored file; if it is the same nothing happens, otherwise the client is regenerated and a pull request is opened, CI checks it, and it is merged unless the semver checks find a breaking change.">
  </a>
  <figcaption>The weekly synchronization of the specification.</figcaption>
</figure>
