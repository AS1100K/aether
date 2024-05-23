# Install `gh`

```shell
(type -p wget >/dev/null || (sudo apt update && sudo apt-get install wget -y)) \
&& sudo mkdir -p -m 755 /etc/apt/keyrings \
&& wget -qO- https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo tee /etc/apt/keyrings/githubcli-archive-keyring.gpg > /dev/null \
&& sudo chmod go+r /etc/apt/keyrings/githubcli-archive-keyring.gpg \
&& echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null \
&& sudo apt update \
&& sudo apt install gh -y
```

```shell
sudo apt update
sudo apt install gh
```

# Login in `gh`

```shell
gh auth login
```

# Install Java
For Minecraft 1.20.4 install Java 17. For more information, [read this article](https://www.cherryservers.com/blog/how-to-install-java-on-ubuntu).

```shell
sudo apt install openjdk-17-jdk -y
```
# Install Binaries
```shell
gh release download aether-core@v0.1.0 --repo AS1100K/aether
```

# Execute Binary
```shell
sudo chmod a+x ./aether-core-arc-linux
```

```shell
screen
sudo ./aether-core-arc-linux
```

Use `Ctrl + a + d` to detach.

To save the log use

```shell
sudo RUST_LOG=info ./aether-core-arc-linux 2>&1 | tee log.txt
```
