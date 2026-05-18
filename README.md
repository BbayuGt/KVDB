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


