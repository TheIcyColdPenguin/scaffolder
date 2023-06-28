# scaffolder
A helpful scaffolder for any kind of project

Example `scaffolder.toml`

```toml
### Typescript Webpack starter

[[projects]]
names = ["ts-web", "ts-webpack"]
description = "A Typescript starter project bundled with webpack"

[[projects.commands]]
type = "copy"
src_file = "ts-web/webpack.config.js"
dest_file = "webpack.config.js"

[[projects.commands]]
type = "copy"
src_file = "ts-web/tsconfig.json"
dest_file = "tsconfig.json"

[[projects.commands]]
type = "create"
file = "src/index.ts"
contents = "export {}"

[[projects.commands]]
type = "create"
file = ".gitignore"
contents = """
dist/*
!dist/index.html
"""

[[projects.commands]]
type = "template"
template = "index.html"
dest_file = "dist/index.html"
[projects.commands.replacements]
title = "Typescript & Webpack"
head = ""
body = """<script src="./bundle.js"></script>"""

[[projects.commands]]
type = "template"
template = "package.json"
dest_file = "package.json"
[projects.commands.replacements]
main = ""
type = "commonjs"
scripts = """
        "build": "webpack",
        "dev": "webpack serve --open"
"""

[[projects.commands]]
type = "command"
command = "yarn"
args =  ["add", "-D", "webpack", "webpack-dev-server", "webpack-cli", "typescript", "ts-loader"]



### Python starter

[[projects]]
names = ["color-python"]
description = "A basic python starter project with colorama installed"

[[projects.commands]]
type = "create"
file = "main.py"
contents = """from colorama import Fore, Style
print(Fore.BLUE + 'hi' + Style.RESET_ALL)"""

[[projects.commands]]
type = "command"
command = "python3"
args =  ["-m", "venv", "venv"]

# hack to preserve environment variables across multiple commands
[[projects.commands]]
type = "command"
command = "sh"
args =  ["-c", """. ./venv/bin/activate \
                    && python3 -m pip install colorama \
                    && python3 -m pip freeze > requirements.txt \
"""]

```
