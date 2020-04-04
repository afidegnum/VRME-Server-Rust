initSidebarItems({"enum":[["ChannelBinding","Channel binding configuration."],["ConfigError","An error which is returned by `Config::create_pool` if something is wrong with the configuration."],["RecyclingMethod","This enum is used to control how the connection is recycled. Attention: The current default is `Verified` but will be changed to `Fast` in the next minor release of `deadpool-postgres`. Please make sure to explicitly state this if you want to keep using the `Verified` recycling method."],["SslMode","TLS configuration."],["TargetSessionAttrs","Properties required of a session."]],"struct":[["Config","Configuration object. By enabling the `config` feature you can read the configuration using the `config` crate. ## Example environment `env PG_HOST=pg.example.com PG_USER=john_doe PG_PASSWORD=topsecret PG_DBNAME=example PG_POOL.MAX_SIZE=16 PG_POOL.TIMEOUTS.WAIT.SECS=5 PG_POOL.TIMEOUTS.WAIT.NANOS=0 ` ## Example usage `rust,ignore Config::from_env(\"PG\"); `"],["ManagerConfig","Configuration object for the manager. This currently only makes it possible to specify which recycling method should be used when retrieving existing objects from the pool."]]});