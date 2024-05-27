FROM rust

# Install just
RUN cargo install just

RUN apt update && apt install -y nodejs npm
# installs nvm & Node
# RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash && \
#     export NVM_DIR="$HOME/.nvm" && \
#     [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh" && \
#     nvm install 20 && \
#     nvm use 20

# Copy app code
WORKDIR /app
COPY . .

RUN node -v
RUN npm install
RUN just build-tailwind

RUN rustup target add wasm32-unknown-unknown
RUN cargo install -f wasm-bindgen-cli
RUN just build-wasm-animation
RUN cargo build --release

CMD ["./target/release/stepannaumov"]