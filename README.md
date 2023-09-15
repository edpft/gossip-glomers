# Gossip Glomers

Solving the [Gossip Glomers](https://fly.io/dist-sys/) distributed systems challenges with rust.

## Project setup

### Initialising git

Create an empty git repo

```bash
git init gossip-glomers
```

Open the repo in VS Code

```bash
code gossip-glomers/
```

Add a remote

```bash
git remote add origin https://github.com/edpft/gossip-glomers.git
```

Set an upstream branch

```bash
git push --set-upstream origin main
```

### Initialise a Nix flake

Initialise a flake with my rust template

```bash
nix flake init -t flake-templates#rust
```

Allow `direnv`

```bash
direnv allow
```

### Initialise a cargo project

```bash
cargo init
```

Replace the default `.gitignore` with the GitHub rust `.gitignore`

```bash
wget https://raw.githubusercontent.com/github/gitignore/main/Rust.gitignore -O .gitignore
```

Update the `.gitignore` to ignore the `.direnv` folder

```diff
# .gitignore

# ...
# MSVC Windows builds of rustc generate these, which store debugging information
*.pdb

# direnv directory
.direnv/

```

### Install `maelstrom`

Update flake to include `jdk`, `graphviz`, and `gnuplot`

```diff
# flake.nix

{
    # ...
      devShell.${system} = mkShell {
        nativeBuildInputs = with pkgs; [
          cargo-watch
+         jdk17
+         graphviz 
+         gnuplot 
        ];
        buildInputs = [ rustToolchain ];
      };
    #   ...
}
```

Reload direnv

```bash
direnv reload
```

Confirm that `java`, `graphviz`, and `gnuplot` have been installed

```bash
java -version
dot --version
gnuplot --version
```

Download the Maelstron tarball

```bash
wget https://github.com/jepsen-io/maelstrom/releases/download/v0.2.3/maelstrom.tar.bz2
```

Extract the archive

```bash
tar -xf maelstrom.tar.bz2
```

Remove the tarball

```bash
rm -rf maelstrom.tar.bz2  
```

Confirm that `maelstrom` is working

```bash
./maelstrom/maelstrom test
```

Update the `.gitignore` to include `maelstrom/`

```diff
# .gitignore

# ...
# direnv directory
.direnv/

# maelstrom directory
maelstrom/

```