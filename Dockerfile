FROM debian
MAINTAINER okrplay <32576280+okrplay@users.noreply.github.com>
WORKDIR /root
RUN /bin/bash -c "apt update && apt upgrade -y && apt install curl git build-essential -y"
RUN /bin/bash -c "curl -o rustup-init.sh https://sh.rustup.rs && chmod +x rustup-init.sh && ./rustup-init.sh -y"
RUN /bin/bash -c "source /root/.cargo/env && git clone -b v0.1.0 https://github.com/okrplay/iodine_storage.git && cd iodine_storage && cargo build --release"
RUN /bin/bash -c "cp /root/iodine_storage/target/release/iodine_storage /opt/iodine_storage && chmod +x /opt/iodine_storage"
RUN /bin/bash -c "source /root/.cargo/env && rm -rf /root/iodine_storage && rustup self uninstall -y && apt purge curl git build-essential -y && apt autoremove -y"
CMD /bin/bash -c "/opt/iodine_storage"