#!/usr/bin/env bash
set -e

# Useless код который ждет пока поднимется postgres если он не поднялся
until psql -U admin -d prod1 -c "SELECT 1;" &>/dev/null; do
  echo "Waiting for prod1..."
  sleep 2
done

# Создаем публикацию
psql -U admin -d prod1 -c "CREATE PUBLICATION mypub FOR ALL TABLES;"
