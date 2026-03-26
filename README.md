# Лабораторная работа №11: Контейнеризация мультиязычных приложений

**Студент:** Гуляев Евгений Александрович 
**Группа:** 221331  
**Вариант:** 5  

---

## Выполненные задания

### Средней сложности
- **Задание 5** — docker-compose для Python, Go и Rust сервисов  
- **Задание 7** — обмен данными через volume между контейнерами  
- **Задание 9** — ограничение ресурсов (CPU, память)  

### Повышенной сложности
- **Задание P5** — оптимизация Docker образа Python приложения
- **Задание P7** — cоздание multi-stage сборку для Python-приложения с Rust-расширением  
- **Задание P2** — Rust приложение с musl для полностью статической сборки  

### 1. Задание 5 — docker-compose
```bash
cd task11_5/task11_5_5
docker-compose up --buil
```
### 2. Задание 7 — volume
```bash
cd task11_5/task11_5_7
docker-compose up --build
```
### 3. Задание 9 — ограничение ресурсов
```bash
cd task11_5/task11_5_9
docker-compose up --build
docker stats
```
### 4. Задание P5 — оптимизация Docker
```bash
cd task11_5/task11_5_p5/python_app
docker build -t optimized-python .
docker run -p 5000:5000 optimized-python
docker images
```
### 5. Задание P2 — Rust musl
```bash
cd task11_5/task11_5_p2
docker build -t rust-musl .
docker run rust-musl 
docker images
```
### 6. Задание P7 — Python + Rust (PyO3) multi-stage сборка
```bash
cd task11_5/task11_5_p7
docker build -t python-rust-app .
docker run python-rust-app 
```