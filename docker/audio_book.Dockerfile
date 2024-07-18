FROM rust:slim-bullseye

WORKDIR /app

RUN mkdir src

COPY ./Cargo.toml ./Cargo.lock ./src ./

# FOR DEVELOPMENT ENVIRONMENT
#============================
CMD [ "cargo", "run" ] 


# FOR PRODUCTION ENVIRONMENT
#============================
# RUN cargo build --release
#
# CMD [ "./target/release/audio_book_api" ]
