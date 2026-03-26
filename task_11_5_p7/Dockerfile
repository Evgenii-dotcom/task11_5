FROM python:3.12-slim AS builder

RUN apt-get update && apt-get install -y \
    build-essential \
    curl

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app

COPY rust_ext ./rust_ext

RUN pip install maturin

WORKDIR /app/rust_ext
RUN maturin build --release

FROM python:3.12-slim

WORKDIR /app

COPY --from=builder /app/rust_ext/target/wheels/*.whl .

RUN pip install *.whl

COPY python_app ./python_app

WORKDIR /app/python_app

CMD ["python", "app.py"]