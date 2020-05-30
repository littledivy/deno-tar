import Archiver from "../mod.ts";

let tar = new Archiver("bruh.tar");

tar.add("./examples/blocking.ts");

tar.archive();
