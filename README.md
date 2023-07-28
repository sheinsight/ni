# n

💡 Inspired by [ni](https://github.com/antfu/ni)

Choose the correct command prefix (npm, yarn, pnpm) based on the `packageManager` configuration in your `package.json`.

# Why

Why do we need to do this when we already have [ni](https://github.com/antfu/ni) 🤔️ ?

Because [ni](https://github.com/antfu/ni) runs on the Node environment, which means that when we switch between different versions of Node using tools like [fnm](https://github.com/Schniz/fnm) and [nvm](https://github.com/nvm-sh/nvm), we always need to globally install [ni](https://github.com/antfu/ni), which is not elegant.

We hope to have a tool similar to [ni](https://github.com/antfu/ni), but it does not depend on the Node environment, so that we can better collaborate with [fnm](https://github.com/Schniz/fnm) and [nvm](https://github.com/nvm-sh/nvm).

# Differences from ni

Our philosophy is somewhat different from [ni](https://github.com/antfu/ni). We are strict and require you to declare `packageManager` in `package.json`.

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
# Ready ✅
n dlx tsx
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
# Ready ✅
n un
# npm uninstall webpack
# yarn remove webpack
# pnpm remove webpack
```

```bash
# Ready ✅
n un -g silent

# npm uninstall -g silent
# yarn global remove silent
# pnpm remove -g silent
```

```bash
# Ready ✅
n set-cache /root
# npm config set cache /root
# yarn config set cache-folder /root
# pnpm config set store-dir /root
```
