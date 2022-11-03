use std::{process::Command, env::{set_current_dir, self}, fs::File, io::Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let project_name = args.get(1).unwrap();
    println!("Creating {}...", project_name);
    Command::new("mkdir").args([project_name]).output().expect("failed to execute process");
    set_current_dir(&project_name).expect("Unable to change into [path to executable]/nvs");
    let mut packagejson_file = File::create("package.json").unwrap();
    packagejson_file.write_all(format!(r#"
{{
  "name": "{}",
  "version": "1.0.0",
  "description": "",
  "main": "index.js",
  "scripts": {{
    "test": "echo \"Error: no test specified\" && exit 1",
    "clean": "rm -rf dist",
    "build": "npm run clean && tsc && cp package.json README.md ./dist"
  }},
  "keywords": [],
  "author": "",
  "license": "ISC",
  "devDependencies": {{
  }}
}}
"#, project_name).trim().as_bytes()).unwrap();
    Command::new("npm").args(["add", "-D", "typescript", "react", "react-dom", "@types/react"]).output().expect("failed to execute process");
    Command::new("npm").args(["install"]).output().expect("failed to execute process");
    let mut gitignore_file = File::create(".gitignore").unwrap();
    gitignore_file.write_all(b"\
node_modules
dist
    ").unwrap();
    let mut tsconfig_file = File::create("tsconfig.json").unwrap();
    tsconfig_file.write_all(format!(r#"{{
  "compilerOptions": {{
    "strict": true,
    "jsx": "react",
    "declaration": true,
    "esModuleInterop": true,
    "outDir": "dist",
    "target": "es6",
    "module": "es6",
    "moduleResolution": "node"
  }},
  "include": ["src"]
}}
"#).trim().as_bytes()).unwrap();
    let mut readme_file = File::create("README.md").unwrap();
    readme_file.write_all(format!(r#"
#{}

# Build
```bash
$ npm run build
```

# Publish 
> Login to npm
```bash
$ npm login
```
> Publish
```bash
$ npm publish ./dist
```

# Notes
Project inspiration came from [here](https://blog.fildon.me/publishing-typescript-react-components-to-npm)
"#, project_name).trim().as_bytes()).unwrap();
    Command::new("mkdir").args(["src"]).output().expect("failed to execute process");
    set_current_dir("src").expect("Unable to change into [path to executable]/nvs");
    let mut countertsx_example_file = File::create("counter.tsx").unwrap();
    countertsx_example_file.write_all(format!(r#"
import * as React from "react";

export function Counter() {{
  const [count, setCount] = React.useState(0);
  return (
    <>
      <p>You clicked {{count}} times</p>
      <button onClick={{() => setCount(count + 1)}}>Click me</button>
    </>
  );
}}
"#).trim().as_bytes()).unwrap();
    let mut indexts_example_file = File::create("index.ts").unwrap();
    indexts_example_file.write_all(format!(r#"
export {{ Counter }} from "./counter";
"#).trim().as_bytes()).unwrap();
}
