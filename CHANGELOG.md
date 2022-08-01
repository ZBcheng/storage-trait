# v0.1.0
Supports dashmap and redis(single node).
# v0.1.1
Add examples and README.
# v0.1.2
1. Add `set_ex` method to Storage trait and it is available for `RedisStorage`.
2. Use get_connection() instead of client.clone().

# v0.1.3
issue: https://github.com/ZBcheng/storage-trait/issues/1
1. Check if connection is eatablished while building `RedisStorage`.
2. Add method `try_build` to `RedisStorage`. Use `try_build` if you wanna get an Err resp or use `build` if you wanna panic immediately when error occurs.