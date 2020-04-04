initSidebarItems({"mod":[["config","This module describes configuration used for [`Pool`] creation."]],"struct":[["ClientWrapper","A wrapper for `tokio_postgres::Client` which includes a statement cache."],["Manager","The manager for creating and recyling postgresql connections"],["StatementCache","This structure holds the cached statements and provides access to functions for retrieving the current size and clearing the cache."],["Transaction","A wrapper for `tokio_postgres::Transaction` which uses the statement cache from the client object it was created by."]],"type":[["Client","A type alias for using `deadpool::Object` with `tokio_postgres`"],["Pool","A type alias for using `deadpool::Pool` with `tokio_postgres`"],["PoolError","A type alias for using `deadpool::PoolError` with `tokio_postgres`"]]});