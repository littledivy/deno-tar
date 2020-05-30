import {
  TarArchive,
} from "./plugin/index.ts";

import { ArchiveParams } from "./types.ts";

class Tar {
  public name: string = "archive.tar";
  public files: string[] = [];
  constructor(name?: string) {
    if (name) this.name = name;
  }
  add(path: string) {
    this.files.push(path);
    return this;
  }
  archive() {
    TarArchive(
      {
        tarname: this.name,
        files: this.files,
      } as ArchiveParams,
    );
  }
}

export default Tar;
