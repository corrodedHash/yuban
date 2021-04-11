set -e
WEBPAGE_PATH=../client/dist
MYSQL_PORT=52583

if [ "$(docker ps -a -f name="test_db" | wc -l)" -eq "2" ]; then
    if [ "$(docker ps -f name="test_db" | wc -l)" -ne "2" ]; then
        docker start test_db
    fi
else
    docker run \
    --detach \
    -e MYSQL_USER=yubanmanager \
    -e MYSQL_DATABASE=yuban \
    -e MYSQL_PASSWORD=secret \
    -e MYSQL_ROOT_PASSWORD=secret \
    -p 127.0.0.1:52583:3306 \
    --name test_db yuban-db

    sleep 10

    YUBAN_ADD_USER_PW=adminpw YUBAN_MYSQL_PASSWORD=secret \
    cargo run -- --mysql-port "${MYSQL_PORT}" --add-user admin
    exit
fi

(
    cd ../client;
    echo running
    npm run serve;
)&

YUBAN_MYSQL_PASSWORD=secret cargo run -- --mysql-port "${MYSQL_PORT}"
