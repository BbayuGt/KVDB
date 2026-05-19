# KVDB - Simple Key-Value Store Database written in Rust. No AI Used.

## Why do this?

With how much AI has progressed in the last few years, I felt like I lost skill to do research and programming, or how to read documentation in general. So I'm making this project as a challenge for myself to gain back those skills. Fuck those AI Agents, I'll do it myself.

This is also my "learning rust" project.

## The Challenge

I may only allowed to access these websites:

* DuckDuckGo HTML (Javascript disabled, no AI summaries)
* Google Search (&udm=14 enabled, no AI summaries)
* Any tutorial website and will not touch any AI features (Rust documentations, W3School, StackOverflow, etc.)

I may not use these tools

* AI Code completions (Github Copilot, etc.)
* Agentic AI/AI cli (Claude Code, Gemini Cli, OpenCode, etc.)

## The Features

The features of this database is:

| Feature | Description |
|---------|-------------|
| GET \<key\> | Gets data from \<key\>, returns string |
| SET \<key\> \<value\> | Sets \<key\> into database with value \<value\>. Cannot replace if already exist, will throw error |
| UPDATE \<key\> \<value\> | Update \<key\> with new value \<value\>. Cannot update if not exist, will throw error |
| DELETE \<key\> | Delete \<key\> from the database. Will throw error if not exist |

Notes:

* Space is not allowed for keys, value is fine.

## Postmortem

The project is done! It's a lot more simple than I expected! So here's what I learned:

* Using your hands (and your mind) to write code is still fun!
* Rust is quite easy to understand when you know how to play with it (the borrowchecker).
* I'm not saying that it's an easy language, but it's not THAT bad (You should try it!)
* My Javascript-ass background couldn't comprehend ownership and borrowing at first, WHAT DO YOU MEAN I CAN'T USE A VARIABLE ANYWHERE I WANT
* But in the end, I can see how this strictness can help with writing safer code.

I'll see ya in my next projects! (If I ever will do these again!)
