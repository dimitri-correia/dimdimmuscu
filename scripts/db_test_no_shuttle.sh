# will be launched automatically by tests
docker kill postgres_test_without_shuttle
docker rm postgres_test_without_shuttle
docker run \
  -e POSTGRES_USER=dim \
  -e POSTGRES_PASSWORD=dim \
  -e POSTGRES_DB=db_for_tests_without_shuttle \
  -p 3269:5432 \
  -d \
  --name postgres_test_without_shuttle \
  postgres
sleep 1
# connect at
# postgresql://dim:dim@localhost:3269/db_for_tests_without_shuttle
