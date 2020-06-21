# Archived in favour of https://deno.land/std/archive/tar.ts

# deno-tar

[![license](https://img.shields.io/github/license/divy-work/deno-tar)](https://github.com/divy-work/deno-tar/blob/master/LICENSE)
[![stars](https://img.shields.io/github/stars/divy-work/deno-tar)](https://github.com/divy-work/deno-tar/stargazers)
[![deno version](https://img.shields.io/badge/deno-1.0.2-success)](https://github.com/denoland/deno)

Read and Create a TAR using Deno.

## Example

Run the following code with the `-A` and `--unstable` (and `-r` if you have
used this module before) flags enabled to get the example shown above:

### Create an archive

```ts
import Archive from "mod.ts";
let myTar = new Archive("archive.tar");
myTar.add("sometext.txt").add("someimg.png");
myTar.archive();
```

### Read an archive

```ts
import { ArchiveReader } from "mod.ts";
let myTar = new ArchiveReader("archive.tar");
console.log(myTar.open());
```

## Contributing

Contributions either in the form of pull requests or issues are always welcome.
