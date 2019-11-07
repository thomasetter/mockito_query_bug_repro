By running `./run_git_bisect.sh`, we can find that `c8ae0b8e70bede019ec6cebde2394bef0d969de5` broke having a `Matcher::Exact` with a `?` in an `Matcher::Any`.

```
test query_in_exact_test ... FAILED

failures:

---- query_in_exact_test stdout ----
Ok(Response { url: "http://127.0.0.1:1234/bar?stuff", status: 501, headers: {} })
thread 'query_in_exact_test' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 
> Expected 1 request(s) to:

GET (any of)?(any)

...but received 0

> The last unmatched request was:

GET /bar?stuff
user-agent: reqwest/0.9.22
accept: */*
accept-encoding: gzip
host: 127.0.0.1:1234

> Difference:

GET (any of)?(any)
GET /bar?stuff
user-agent: reqwest/0.9.22
accept: */*
accept-encoding: gzip
host: 127.0.0.1:1234


', mockito/src/lib.rs:868:44
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.


failures:
    query_in_exact_test

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--test repro'
c8ae0b8e70bede019ec6cebde2394bef0d969de5 is the first bad commit
commit c8ae0b8e70bede019ec6cebde2394bef0d969de5
Author: Florin Lipan <florinlipan@gmail.com>
Date:   Wed Jul 10 17:47:21 2019 +0200

    Implement a query matcher

 .travis.yml    |   3 +-
 src/lib.rs     | 119 ++++++++++++++++++++++++++++++++++++++++++++++++++---
 src/request.rs |  15 +++++--
 src/server.rs  |   5 +++
 tests/lib.rs   | 127 ++++++++++++++++++++++++++++++++++++++++++++++++++-------
 5 files changed, 243 insertions(+), 26 deletions(-)
bisect run success
```