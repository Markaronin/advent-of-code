FROM ubuntu:22.04

RUN <<EOF
    apt-get update
    apt-get -y install git curl unzip build-essential sudo zip
EOF

ARG REMOTE_USER
ARG REMOTE_UID
ARG REMOTE_GID
RUN <<EOF
    useradd ${REMOTE_USER} --create-home
EOF

ENV HOME /home/${REMOTE_USER}

USER ${REMOTE_USER}

RUN <<EOF
    curl https://sh.rustup.rs -sSf | bash -s -- -y
EOF
ENV PATH $HOME/.cargo/bin:$PATH
