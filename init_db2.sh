#!/usr/bin/env bash
set -e

# Useless код который ждет пока поднимется postgres если он не поднялся

until psql -U admin -d prod2 -c "SELECT 1;" &>/dev/null; do
  echo "Waiting for prod2..."
  sleep 2
done

# Useless код который ждет пока поднимется postgres если он не поднялся

until psql -U admin -d prod1 -h prod-db1 -p 5432 -c "SELECT 1;" &>/dev/null; do
  echo "Waiting for prod1..."
  sleep 2
done

# Добавляем расширение
psql -U admin -d prod2 -c "
CREATE SUBSCRIPTION mysub
  CONNECTION 'host=prod-db1 port=5432 dbname=prod1 user=admin password=secret'
  PUBLICATION mypub;
"

