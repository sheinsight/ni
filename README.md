# n

💡 Use the right package manager by rust

💡 Inspired by [ni](https://github.com/antfu/ni)

# Why

[ni](https://github.com/antfu/ni) is nice , but [ni](https://github.com/antfu/ni) is based on Node.

it is difficult to collaborate well with node version management tools like [fnm](https://github.com/Schniz/fnm) and [nvm](https://github.com/nvm-sh/nvm).

Therefore, we need an executable file that does not depend on any environment.

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
n i react
# npm i react
# yarn add react
# pnpm add react
```

```bash
# Ready ❌
n i react -D
# npm i react -D
# yarn add react -D
# pnpm add react -D
```

```bash
# Ready ✅
n i --frozen
# npm ci
# yarn install --frozen-lockfile (Yarn 1)
# pnpm install --frozen-lockfile
```

```bash
# Ready ❌
n i -g eslint

# npm i -g eslint
# yarn global add eslint (Yarn 1)
# pnpm add -g eslint

```

## run

```bash
# Ready ❌
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
# Ready ❌
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
