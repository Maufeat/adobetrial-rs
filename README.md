# Adobe Trial Reset in Rust

This project was after I watched my first 2 hours course of basic Rust. Decided to recreate [LeagueRainis](https://github.com/LeagueRaINi) [Adobe Trial Reset Tool](https://github.com/LeagueRaINi/Trial-Tool) from C# into Rust.

- Instead of XML Parsing, I just did a RegEx with a [StrReplace Module](https://users.rust-lang.org/t/how-to-get-a-substring-of-a-string/1351/10) I found while reading online. I had problems with the quick-xml cargo, I may update this later when I am more expierenced.
- Since Admin Priviledges are required, I use the winapi and a [windows.rs Module](https://users.rust-lang.org/t/how-do-i-determine-if-i-have-admin-rights-on-windows/35710) I found online to detect if the application is run elevated.
- ~~No freaking error handling at all. The course I've watched was just a basic crash course, I don't know yet how to handle errors and what Ok(()) does...~~
