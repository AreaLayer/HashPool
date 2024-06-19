FROM rust-Lightning
FROM rust-bitcoin 
FROM bdk 
RUN cargo install --git https://github.com/AreaLayer/HashPool.git --branch master --locked
RUN cargo install --git https://github.com/AreaLayer/HashPool.git --branch master --locked --features=bitcoin
RUN cargo install --git https://github.com/AreaLayer/HashPool.git --branch master --locked --features=lightning
RUN cargo install --git https://github.com/AreaLayer/HashPool.git --branch master --locked --features=bitcoin,lightning
COPY --chown=user:group --from=rust-Lightning /usr/local/cargo/bin/lightning-hashpool /usr/local/bin/lightning-hashpool
