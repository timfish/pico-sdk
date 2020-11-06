const { run, copyFile } = require('./tools');
const { platform } = require('os');

const platforms = {
  win32: {
    cmds: [
      'cargo build --release --target x86_64-pc-windows-msvc',
      'cargo build --release --target i686-pc-windows-msvc',
    ],
    copies: [
      [
        'target/x86_64-pc-windows-msvc/release/pico_native.dll',
        'artifacts/windows-x86_64/pico_native.dll',
      ],
      [
        'target/i686-pc-windows-msvc/release/pico_native.dll',
        'artifacts/windows-i686/pico_native.dll',
      ],
    ],
  },
  darwin: {
    cmds: ['cargo build --release'],
    copies: [
      [
        'target/release/libpico_native.dylib',
        'artifacts/macos-x86_64/libpico_native.dylib',
      ],
    ],
  },
  linux: {
    cmds: ['cargo build --release'],
    copies: [
      [
        'target/release/libpico_native.so',
        'artifacts/linux-x86_64/libpico_native.so',
      ],
    ],
  },
};

const thisPlatform = platforms[platform()];
thisPlatform.cmds.forEach((c) => run(c));
thisPlatform.copies.forEach(([src, dst]) => copyFile(src, dst));
