## This exist to remind me what i was doing here

### Implementing DataTable::insert_tuple()

- I accept that only Column Layout is supported (no indirection layer)
- Each tile group holds a vector of sub-schema, each of which only holds
  a vector of column
- already implement insert_tuple for DataTable, next is implementing
  InsertExecutor in insert.rs file
- implemented populate_table without tx_manager.perform_insert
### Current goals:
Deeply understands how query executor works including:
- seq scan
- index scan
- bitmap heap scan
- Join: merge join/hash join/grace hash join 


### Next steps:
Impl Insert with index
Impl SeqScan
Write test
<!-- - the main point is see how the benchmark works
- impl seq_scan
- impl test for seq_scan
- impl test for tile/tile_group
- impl test for insert_executor -->

### In the middle of some detail
- need to read paper MVOCC 2011 again and summary into wiki on github
- revisit NSM/DSM/FSM/PAX is a must now
- implemented populate_table without tx_manager.perform_insert
- implementing seqscan, in the middle between is_visible and commit_tx.
- implement TGH to extract tuple begin_ts end end_ts to start writing
  test for visibilty check function
- look into the index of peloton


### Big TODO: since Peloton source code is really messy, it is a bad
reference, let's take a look at noise page instead
