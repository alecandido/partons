# To do

- [ ] add `Source`-dependant conversion layer
    - selectable via an optional source kind
    - by default, do nothing
    - if LHAPDF, parse and dump in new format
- [ ] convert cached resources
    - when requested:
      - download and
      - save the original one in a temporary location
    - right after (before returning):
      - apply conversion
      - dump new file
      - delete the temporary one
- [ ] add a further state in downloading:
    - if resource is present: load and return
    - if present in a temporary location: convert, dump, delete, and return
    - if absent, download
- [ ] parse LHAPDF grids
- [ ] define new data structure
- [ ] de-duplicate/fix YAML during conversion


## Later on
- [ ] make a dummy registry to run all tests without an internet connection
  - the easiest way is to store a pre-filled cache, that intercepts all requests to the remote
