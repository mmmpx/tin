# tin: Twitch IRC nommer

[nom](https://github.com/Geal/nom) parser for parsing/normalising Twitch (and probably all) IRC messages.

```rust
let line = ":tmi.twitch.tv 372 justinfan123 :You are in a maze of twisty passages, all alike.\r\n";
let _m = message(line);
if let Ok((_, m)) = _m {
    println!("{:?}", m);
}

> Message {
      tags: None,
      source: Some("tmi.twitch.tv"),
      command: "372",
      parameters: ["justinfan123", "You are in a maze of twisty passages, all alike."]
  }
```

- Follows rules from the following docs:
  - https://modern.ircdocs.horse/#client-to-server-protocol-structure
  - https://dev.twitch.tv/docs/irc/commands
  - https://dev.twitch.tv/docs/irc/example-parser
- Only one dependency
  - `nom = "7.1.1"`

## Twitch-specific commands

Listed here as quick reference only:
- **CLEARCHAT**  
`:tmi.twitch.tv CLEARCHAT #<channel> :<user>`
- **CLEARMSG**  
`:tmi.twitch.tv CLEARMSG #<channel> :<message>`
- **GLOBALUSERSTATE**  
`:tmi.twitch.tv GLOBALUSERSTATE`
- **HOSTTARGET**  
`:tmi.twitch.tv HOSTTARGET #<hosting-channel> :[-|<channel>] <number-of-viewers>`
- **NOTICE**  
`:tmi.twitch.tv NOTICE #<channel> :<message>`
- **RECONNECT**  
`:tmi.twitch.tv RECONNECT`
- **ROOMSTATE**  
`:tmi.twitch.tv ROOMSTATE #<channel>`
- **USERNOTICE**  
`:tmi.twitch.tv USERNOTICE #<channel> :[<message>]`
- **USERSTATE**  
`:tmi.twitch.tv USERSTATE #<channel>`
- **WHISPER**  
`:<to-user>!<to-user>@<to-user>.tmi.twitch.tv WHISPER <from-user> :<message>`