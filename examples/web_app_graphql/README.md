# Meilisearch example with graphql using `diesel`, `async_graphql` and `postgres`

## Contents

Setting up a graphql server using `async_graphql` and `actix-web`

Using `diesel` to query the database

Using `meilisearch-sdk` to search for records that match a given criteria

## Running the example

The meilisearch server needs to be running. You can run it by the command below

```bash
meilisearch --master-key <your master key>
```

Then you can run the application by simply running

```bash
cargo run --release
```

The above command will display a link to your running instance and you can simply proceed by clicking the link or navigating to your browser.

### Running the resolvers

On your browser, you will see a graphql playground in which you can use to run some queries

You can use the `searchUsers` query as follows:

```gpl
query {
  users{
    search(queryString: "Eugene"){
      lastName
      firstName
      email
    }
  }
}
```
