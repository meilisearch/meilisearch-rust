# Meilisearch test macro

This crate defines the `meilisearch_test` macro.

Since the code is a little bit harsh to read, here is a complete explanation of how to use it.
The macro aims to ease the writing of tests by:

1. Reducing the amount of code you need to write and maintain for each test.
2. Ensuring All your indexes as a unique name so they can all run in parallel.
3. Ensuring you never forget to delete your index if you need one.

Before explaining its usage, we're going to see a simple test _before_ this macro:

```rust
#[async_test]
async fn test_get_tasks() -> Result<(), Error> {
  let client = Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);

  let index = client
    .create_index("test_get_tasks", None)
    .await?
    .wait_for_completion(&client, None, None)
    .await?
    .try_make_index(&client)
    .unwrap();

  let tasks = index.get_tasks().await?;
  // The only task is the creation of the index
  assert_eq!(status.results.len(), 1);

  index.delete()
    .await?
    .wait_for_completion(&client, None, None)
    .await?;
  Ok(())
}
```

I have multiple problems with this test:

- `let client = Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);`: This line is always the same in every test.
  And if you make a typo on the http addr or the master key, you'll have an error.
- `let index = client.create_index("test_get_tasks", None)...`: Each test needs to have an unique name.
  This means we currently need to write the name of the test everywhere; it's not practical.
- There are 11 lines dedicated to the creation and deletion of the index; this is once again something that'll never change
  whatever the test is. But, if you ever forget to delete the index at the end, you'll get in some trouble to re-run
  the tests.

---

With this macro, all these problems are solved. See a rewrite of this test:

```rust
#[meilisearch_test]
async fn test_get_tasks(index: Index, client: Client) -> Result<(), Error> {
  let tasks = index.get_tasks().await?;
  // The only task is the creation of the index
  assert_eq!(status.results.len(), 1);
}
```

So now you're probably seeing what happened. By using an index and a client in the parameter of
the test, the macro automatically did the same thing we've seen before.
There are a few rules, though:

1. The macro only handles three types of arguments:

- `String`: It returns the name of the test.
- `Client`: It creates a client like that: `Client::new("http://localhost:7700", "masterKey")`.
- `Index`: It creates and deletes an index, as we've seen before.

2. You only get what you asked for. That means if you don't ask for an index, no index will be created in meilisearch.
   So, if you are testing the creation of indexes, you can ask for a `Client` and a `String` and then create it yourself.
   The index won't be present in meilisearch.
3. You can put your parameters in the order you want it won't change anything.
4. Everything you use **must** be in scope directly. If you're using an `Index`, you must write `Index` in the parameters,
   not `meilisearch_rust::Index` or `crate::Index`.
5. And I think that's all, use and abuse it 🎉
