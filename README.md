# n

💡 Use the right package manager by rust

💡 Inspired by [ni](https://github.com/antfu/ni)

# Why

[ni](https://github.com/antfu/ni) is nice , but [ni](https://github.com/antfu/ni) is based on Node.

it is difficult to collaborate well with node version management tools like [fnm](https://github.com/Schniz/fnm) and [nvm](https://github.com/nvm-sh/nvm).

Therefore, we need an executable file that does not depend on any environment.

# Installation

## Using a script (macOS/Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/sheinsight/ni/main/install.sh | bash
```

# Use case

## install

```bash
# Ready ✅
n i
# npm install
# yarn install
# pnpm install
```

```bash
# Ready ✅
n ci
# npm ci
# yarn install --frozen-lockfile (Yarn 1)
# pnpm install --frozen-lockfile
```

## add

```bash
# Ready ✅
n add react
# npm add react
# yarn add react
# pnpm add react
```

```bash
# Ready ✅
n add react -D
# npm add --save-dev react
# yarn add --save-dev react
# pnpm add --save-dev react
```

```bash
# Ready ✅
n add react -S
# npm add --save react
# yarn add --save react
# pnpm add --save react
```

```bash
# Ready ✅
n add react -O
# npm add --save-optional react
# yarn add --save-optional react
# pnpm add --save-optional react
```

```bash
# Ready ✅
n add -g eslint

# npm add --global eslint
# yarn global add eslint (Yarn 1)
# pnpm add --global eslint

```

## run

```bash
# Ready ✅
n r dev
# npm run dev
# yarn run dev
# pnpm run dev
```

```bash
# Ready ✅
n r dev --port=3000
# npm run dev -- --port=3000
# yarn run dev --port=3000
# pnpm run dev --port=3000
```

```bash
# Ready ❌
n r
# interactively select the script to run
# supports https://www.npmjs.com/package/npm-scripts-info convention
```

## npx

```bash
# Ready ❌
n x tsx
# npx vitest
# yarn dlx vitest
# pnpm dlx vitest
```

## upgrade

```bash
# Ready ✅
n u
# npm upgrade
# yarn upgrade (Yarn 1)
# pnpm update
```

## uninstall

```bash
# Ready ❌
n un
# npm uninstall webpack
# yarn remove webpack
# pnpm remove webpack
```

```bash
# Ready ❌
n un -g silent

# npm uninstall -g silent
# yarn global remove silent
# pnpm remove -g silent
```

# How

Unlike ni, n requires you to configure packageManager in package.json.

Because we hope everything is clear.
