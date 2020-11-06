const { spawnSync } = require('child_process');
const { dirname, join } = require('path');
const {
  copyFileSync,
  mkdirSync,
  readdirSync,
  lstatSync,
  readFileSync,
  writeFileSync,
} = require('fs');

const magenta = '\x1b[1m\x1b[36m';
const reset = '\x1b[0m';

const run = (command, cwd = '.', env = {}) => {
  console.log(`${magenta}Command${reset}:`, command);

  const result = spawnSync(command, {
    shell: true,
    stdio: 'inherit',
    cwd,
    env: { RUST_BACKTRACE: 1, ...process.env, ...env },
  });

  if (result.status != 0) {
    throw new Error(`Command exited with non-zero exit code: ${result.status}`);
  }
};

const copyFile = (src, dst) => {
  console.log(`${magenta}Copy file${reset}:`, src, `${magenta}->${reset}`, dst);
  mkdirSync(dirname(dst), { recursive: true });
  copyFileSync(src, dst);
};

const copyDir = (src, dst) => {
  readdirSync(src).forEach((element) => {
    const stat = lstatSync(join(src, element));
    if (stat.isFile()) {
      copyFile(join(src, element), join(dst, element));
    } else if (stat.isDirectory()) {
      copyDir(join(src, element), join(dst, element));
    }
  });
};

const walkDir = (matches, callback, options = {}) => {
  const defaults = { cwd: '.', ignore: [/^target/, /node_modules/] };
  const opt = { ...defaults, ...options };

  readdirSync(opt.cwd).forEach((element) => {
    const stat = lstatSync(join(opt.cwd, element));
    if (stat.isFile()) {
      const path = join(opt.cwd, element);
      if (
        matches.some((m) => path.match(m)) &&
        !opt.ignore.some((m) => path.match(m))
      ) {
        callback(path);
      }
    } else if (stat.isDirectory()) {
      walkDir(matches, callback, { ...opt, cwd: join(opt.cwd, element) });
    }
  });
};

const findReplace = (fileMatch, textMatches) => {
  walkDir([fileMatch], (path) => {
    const initial = readFileSync(path, { encoding: 'utf8' });
    let contents = initial;
    textMatches.forEach(([from, to]) => {
      contents = contents.replace(from, to);
    });

    if (initial != contents) {
      console.log(`${magenta}Writing file${reset}:`, path);
      writeFileSync(path, contents);
    }
  });
};

exports.run = run;
exports.copyDir = copyDir;
exports.copyFile = copyFile;
exports.walkDir = walkDir;
exports.findReplace = findReplace;
