id="$(( ( RANDOM % 999999999 )  + 1 ))"
docker run --name postgres-$id -e POSTGRES_PASSWORD=pass -p 5432:5432 -d postgres