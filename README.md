# PostgreSQL Logical Replication Setup

This repository provides an example of configuring and running two PostgreSQL instances—**prod-db1** (source) and **prod-db2** (target)—within Docker, using **logical replication** to migrate data efficiently and with minimal downtime.

## Key Features

- **Logical Replication**: Stream data changes from the old database (prod-db1) to the new database (prod-db2) in near real-time.
- **Zero-Downtime Migration**: Keep prod1 online and serving traffic while data is continuously replicated to prod2.
- **Custom Configuration**: Tune PostgreSQL parameters (e.g., `wal_level`, `shared_buffers`, `maintenance_work_mem`) for optimal replication and performance.
- **Automated Setup**: Initialization scripts and Docker Compose ensure that the publication on prod1 and the subscription on prod2 are created and managed automatically.

## How It Works

1. **Publication on prod1**: Creates a logical publication that makes all tables available for replication.
2. **Subscription on prod2**: Connects to prod1’s publication, taking an initial snapshot of data and then continuously applying changes as they occur.
3. **Cutover**: After the data on prod2 is current and stable, switch your application’s traffic to prod2. No downtime is required, as prod1 remains fully operational until you’re ready to retire it.

## Getting Started

1. **Configure**: Adjust `prod1_postgresql.conf` and any environment variables as needed.
2. **Run**: Launch services with `docker-compose up -d`.
3. **Verify**: Check that prod2 receives data from prod1 by monitoring `pg_stat_subscription` on prod2.
4. **Switch Over**: Point your application to prod2 whenever you’re ready. Prod1 can then be decommissioned.

## Benefits

- **Minimal Disruption**: No need to stop your current database.
- **Flexibility**: Replicate all or selected tables, and easily integrate schema changes.
- **Scalability**: Logical replication can handle high-load environments with proper tuning.

This setup demonstrates a robust, low-latency migration strategy, ensuring your data moves from old to new without pausing your business operations. Enjoy seamless migrations and reduced downtime!
