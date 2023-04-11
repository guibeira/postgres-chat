# Postgres-chat

A humble implementation of stateless chat using NOTIFY and LISTEN functions of Postgres

This is a command-line chat application built using Rust and Postgres builtin function. It allows users to send and receive messages in a chat room.

![chat sample](https://github.com/guibeira/postgres-chat/raw/main/image/pg_chat.gif "sample")
## Development
To install this application, you will need Rust and Cargo installed on your machine. Then, run the following command:

```bash
 git clone https://github.com/guibeira/postgres-chat.git
 cd postgres-chat
 cargo run
```
## Usage
To start the chat, run the following command:

```bash
pg-chat --channel-name <channel_name> --username <username>
``` 
Replace <channel_name> and <username> with your desired channel name and username.

Once the chat has started, you can send messages by typing them into the command prompt and pressing enter. You will receive messages from other users in real-time.

To exit the chat, press Ctrl-C.

License
This project is licensed under the MIT License. See the LICENSE file for details
