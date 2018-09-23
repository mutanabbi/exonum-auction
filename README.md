требуется реализовать простой сервис аукциона. Базовая функциональноcть:
 - Добавление лота и открытие аукциона по нему;
 - Добавления ставки на лот. Данный запрос должен быть реализован через callback. Т.е. отправляем запрос, ждем, когда транзакция выпустилась в блоке, и только потом возвращаем в ответе хеш транзакции и номер блока, в который попала транзакция;
 - Вывод текущего результата ставок по id лоту.

Процесс, связанный с временем жизни аукциона и закрытием, можно опустить.


Примерный план:
 - (Main) Реализовать весь функционал блокчейн бэкенда (API добавление/поиска, описание хранимых данных, и т.д.), используя фреймворк exonum (https://exonum.com/).
 - Написать краткую документацию по реализованному API
 - (Опционально) Продумать и описать схему закрытого аукциона с использованием протокола доказательства с нулевым разглашением (zero-knowledge proof).


Iteration 1 version (not complete)
 - run server. `server -h` for help
     server -a 6666
 - anounce test lot
     curl -H "Content-Type: application/json" -X POST -d '{"body":{"anouncer":"ce31f4e587014199694a9be3c935418b5cedfba59c6ab9474f83c9c5d50184cf","desc":"Tesre":"18e852b2698d587d1f8eb29b293367a946d96f71762a896b2cd760d85e7df88eb6aa6197373ac375ceec6903127d312190f720e6cc60ab5c75deac743e4c2207"}' 127.0.0.1:6666/api/services/auction/v1/lot -v
 - anounce test lot with light client utility. `client -h` for help
     client -i clnt-n1 -u 127.0.0.1:6666/api/services/auction/v1/lot
 - get info about lots
     get 127.0.0.1:6666/api/services/auction/v1/lots
