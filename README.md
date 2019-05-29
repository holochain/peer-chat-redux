# Holochain Basic Chat

A simple chat app designed to get new users up, running and developing on Holochain

![Alt text](doc/screen.png?raw=true)

# Get chatting using nix & the latest released basic-chat
- Install nix tooling on mac/linux
```
  curl https://nixos.org/nix/install | sh
```

- Clone the holochain-rust repo to access the nix commands
```
  git clone https://github.com/holochain/holochain-rust.git
```

- Start a new nix-shell that has everything we need
```
  nix-shell ./holochain-rust/default.nix
```

- Get the latest released basic-chat
```
  wget --header='Accept:application/octet-stream' https://api.github.com/repos/holochain/holochain-basic-chat/releases/assets/12405253 -O basic-chat.zip
  unzip basic-chat.zip
```

- Open the my-conductor-config.toml file in your favourite editor
- Create yourself an Agent Keystore and update the [[agents]] section of the my-conductor-config.toml following the steps at the top of the [[agents]] section
```
  hc keygen

  This will create a new agent keystore and populate it with an agent keybundle
  containing a public and a private key, for signing and encryption by the agent.
  This keybundle will be stored encrypted by passphrase within the keystore file.
  The passphrase is securing the keys and will be needed, together with the file,
  in order to use the key.
  Please enter a secret passphrase below. You will have to enter it again
  when unlocking the keybundle to use within a Holochain conductor.
  Passphrase:
  Re-enter passphrase:
  Generating keystore (this will take a few moments)...

  Succesfully created new agent keystore.

  Public address: HcScjdnxMF3Stafva7bpiFhst7Sywx9c3ip7AnuaFfhjeerdyIdWBOiVn6udsar
  Keystore written to: /Users/philipbeadle/Library/Preferences/org.holochain.holochain/keys/HcScjdnxMF3Stafva7bpiFhst7Sywx9c3ip7AnuaFfhjeerdyIdWBOiVn6udsar

  You can set this file in a conductor config as keystore_file for an agent.
```
- Run basic-chat on holochain
```
  holochain -c ./my-conductor-config.toml
```



## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### Prerequisites

At a minimum you will need to have the binaries for the holochain conductor installed on your system. See [the official installation instructions](https://developer.holochain.org/start.html).

**Be sure to use the v0.0.17-alpha2 release for the CLI and Conductor.**

### Running

From the project root, run the script to download the DNA/UI and then start the conductor
```
npm install
npm run fetch
npm run hc:start
```

or alternatively to call the conductor directly

```
npm install
npm run fetch
holochain -c ./conductor-config.toml
```

This will run the DNA as well serve the user interface. To load the app, just browse to [http://localhost:3000](http://localhost:3000).


### Networking

If you want to try and connect to other nodes this also requires the holochain networking library [n3h](https://github.com/holochain/n3h).

Instructions for networking can be found in the [conductor config](conductor-config.toml) file.

## Building from Source

### Holochain DNA

Building the DNA also requires that the holochain developer CLI, `hc`, is installed. You can run the helper script

```
npm run hc:build
```

or use the CLI directly

```
mkdir -p dna
cd dna-src
hc package --strip-meta -o ../dna/holo-chat.hcpkg
```

*Be careful!* If you are trying to network with other agents it is best to use a pre-built DNA. Any differences in compiler configuration may lead to the DNA hash being different and the nodes will not be able to communicate.

### UI

```
cd ui-src
npm install
npm run build
```

## Built With

* [Holochain](https://developer.holochain.org/)
* [React](https://reactjs.org/)

A huge acknowledgement to Pusher for providing an open source React chat UI (https://github.com/pusher/react-slack-clone)

## Authors

* **Willem Olding** - *Initial work* - [willemolding](https://github.com/willemolding)

## License

This project is licensed under the GPL-3 License - see the [LICENSE.md](LICENSE.md) file for details
