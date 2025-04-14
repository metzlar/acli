# AclI

A clone of github copilot for the terminal.
The intend is to make it AI agnostic, and possibly use multiple AIs and use the best answers.
The current implementation is for ChatGPT only.

## Usage

Configure the `OPENAI_API_KEY` environment variable with your api key.


To have the AI write a shell command for you:
```bash
acli do "Stage all .png files in the pic folder in each sub folder with git"

find . -type d -name 'pic' -exec git add {}/*.png \;
```

To get an explaination of a shell command from the AI:
```bash
acli explain "find . -type d -name 'pic' -exec git add {}/*.png \;"

Here's a breakdown of what each part does:

- `find .`: This initiates the `find` command, starting the search from the current directory (`.`).

- `-type d`: This specifies that the search is looking for directories (`d` stands for directory).

- `-name 'pic'`: This restricts the search to directories named 'pic'.

- `-exec git add {}/*.png \;`: For each directory found, this part executes a command. Here, it adds all `.png` files within the found `pic` directories to the Git staging area. 

  - `-exec`: Allows you to execute a command on the found items.
  
  - `git add {}/*.png`: The `{}` is replaced by the path of each found directory, so this command operates on every `/pic` directory found, adding all `.png` files within it to the Git staging area.

- `\;`: This signifies the end of the command to be executed on each found item. The backslash (`\`) escapes the semicolon (`;`).

In summary, this command finds all directories named 'pic' starting from the current directory and adds all `.png` files within those directories to the Git staging area.
```