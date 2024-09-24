FROM rust:1.81.0

WORKDIR /app

COPY . .

RUN cargo build

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["cargo","run"]