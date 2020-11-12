const { run, walkDir } = require('./tools');
const { writeFileSync } = require('fs');

// Rust
try {
  if (process.env.CRATES_IO_TOKEN) {
    run(`cargo login ${process.env.CRATES_IO_TOKEN}`);
    run('cargo install cargo-release');
    run('cargo release --no-confirm --no-dev-version');
  }
} catch (e) {
  console.error('Failed to publish Rust');
}

// dotnet
try {
  if (process.env.NUGET_API_KEY) {
    walkDir(
      /\.nupkg/,
      (path) => {
        run(
          `dotnet nuget push ${path} --api-key ${process.env.NUGET_API_KEY} --skip-duplicate --source https://api.nuget.org/v3/index.json`
        );
      },
      { cwd: 'dotnet/bin/Release' }
    );
  }
} catch (e) {
  console.error('Failed to publish dotnet');
}

// Python
try {
  if (process.env.PYPI_PASSWORD) {
    run(
      `python -m twine upload --skip-existing --non-interactive --disable-progress-bar --verbose ./dist`,
      'python',
      {
        TWINE_USERNAME: '__TOKEN__',
        TWINE_PASSWORD: process.env.PYPI_PASSWORD,
      }
    );
  }
} catch (e) {
  console.error('Failed to publish Python');
}

// nodejs
try {
  if (process.env.NPM_TOKEN) {
    const npmrc = `//registry.npmjs.org/:_authToken=${process.env.NPM_TOKEN}`;
    writeFileSync('.npmrc', npmrc);

    run('npm publish', 'nodejs');
  }
} catch (e) {
  console.error('Failed to publish nodejs');
}
