FROM volf52/cargo-chef:0.1.48 as chef
FROM volf52/tini:0.19 as tini
FROM volf52/rust-wasm:1.65.0 as fe-img
FROM volf52/rust-trunk:0.16.0 as trunk-src
FROM volf52/rust-musl-builder:1.65.0-slim-mold as img

FROM img as userconfig

ARG USER=fsrust
ARG USERID=10001

ENV USER=${USER}
ENV UID=${USERID}

# prevent using sudo
USER root:root

#See https://stackoverflow.com/a/55757473/12429735
RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "${UID}" \
  "${USER}"

# reset back to user space
USER volfy:volfy

# --------------- Finish user config ---------

FROM img as base

WORKDIR /app

COPY --from=chef /bin/cargo-chef /home/volfy/.cargo/bin/cargo-chef

FROM fe-img as fe-base

WORKDIR /app

COPY --from=chef /bin/cargo-chef /home/volfy/.cargo/bin/cargo-chef

# ------ Recipes -----

FROM base as common-pkg-recipe

COPY common .

RUN cargo chef prepare --recipe-path recipe.json

FROM fe-base as frontend-recipe

COPY frontend .

RUN cargo chef prepare --recipe-path recipe.json

FROM base as backend-recipe

COPY backend .

RUN cargo chef prepare --recipe-path recipe.json

# ------- Builds ----------

FROM base as common-build

COPY --from=common-pkg-recipe /app/recipe.json .

RUN cargo chef cook --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY common .

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM fe-base as frontend-build

COPY --from=frontend-recipe /app/recipe.json .
COPY --from=common-build /app /common

RUN cargo chef cook --target wasm32-unknown-unknown --recipe-path recipe.json

COPY --from=trunk-src /bin/trunk /bin/trunk

COPY frontend .

RUN /bin/trunk build --release

FROM base as backend-build

COPY --from=common-build /app /common
COPY --from=backend-recipe /app/recipe.json .

RUN cargo chef cook --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY backend .

RUN SQLX_OFFLINE=true cargo build --release --target x86_64-unknown-linux-musl

# ---- Release ------

FROM scratch

ARG USER=fsrust
ARG PORT=3000

USER ${USER}:${USER}

COPY --from=userconfig /etc/passwd /etc/passwd
COPY --from=userconfig /etc/group /etc/group
# COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

EXPOSE ${PORT}

COPY --from=tini /bin/tini /bin/tini
COPY --from=frontend-build /app/dist /app/dist
COPY --from=backend-build /app/target/x86_64-unknown-linux-musl/release/backend /app/server

ENTRYPOINT ["/bin/tini", "--","/app/server"]
