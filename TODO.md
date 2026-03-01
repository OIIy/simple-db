### Todo
- [ ] Make updates to tables async
- [ ] Support different data types
- [ ] Support custom error types
- [ ] Add stricter definitions for rows, they should be constrained 
by column types
- [ ] Add value indexing

### Objective
Create a lightweight datastore capable of

- 100 writes/second
- 200 reads/second
- <10ms response times

Data will be non-relational and stored in a single table.
Data will be streamed and sent through a TCP listener.
Operation results are then returned to the client.

### Use Case Scenarios
1. I want to send instructions to my database to modify my store.
2. When I ask to read data I should get an object back containing all
the columns and matching records that I've asked for.
3. When I create/update/delete data, I should get a message back indicating 
how many records were updated.
4. I should be able to perform operations on my table simultaneously.

### Scenario Breakdown
1. The DB will need a way of parsing instructions.
2. Reads should have parameters for filtering columns and rows.
3. Each operation should have a corresponding response type.
4. Operations should be non-blocking.

### Design
- Q. How should we interpret instructions such as create/read/update?
- Q. How do we handle non-blocking IO?
- Q. How do we store/format table data?

#### How should we interpret instructions such as create/read/update?
Instructions should be sent in a format similar to SQL.
- **GET**: 'IN table_name GET [columns] WHERE [expression]',
- **SET**: 'IN table_name SET [columns], ([values]) WHERE [expression]',

We will need a simple parsing pipeline for this.
- The instruction comes in over TCP. 
- The bytes are fed to the pipeline.
- The pipeline first splits the instruction into its parts. (Tokenization)
- ["IN", "table_name", "GET", "col_1", "WHERE", "col_1", "=", "hello world!"]
- Then the tokens are semantically parsed and converted to rust types.

#### How do we handle non-blocking IO?
To begin with, we will simply lock the table while it is being
performing some operation.

#### How do we store/format table data?
We'll just start with a hashmap and serialize to JSON to begin with.