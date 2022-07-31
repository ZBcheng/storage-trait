# v0.1.0
Supports dashmap and redis(single node).
# v0.1.1
Add examples and README.
# v0.1.2
1. Add `set_ex` method to Storage trait and it is available for RedisStorage.
2. Use get_connection() instead of client.clone().