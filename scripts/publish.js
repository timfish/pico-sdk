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
  console.error('Failed to publish Rust', e);
}

// dotnet
try {
  if (process.env.NUGET_API_KEY) {
    walkDir([/\.nupkg/], (path) => {
      run(
        `dotnet nuget push ${path} --api-key ${process.env.NUGET_API_KEY} --skip-duplicate --source https://api.nuget.org/v3/index.json`
      );
    });
  }
} catch (e) {
  console.error('Failed to publish dotnet', e);
}

// Python
try {
  if (process.env.PYPI_PASSWORD) {
    run('python -m pip install --user --upgrade twine');
    run(
      `cd python && python -m twine upload --skip-existing --non-interactive --disable-progress-bar --verbose ./dist`,
      {
        TWINE_USERNAME: '__TOKEN__',
        TWINE_PASSWORD: process.env.PYPI_PASSWORD,
      }
    );
  }
} catch (e) {
  console.error('Failed to publish Python', e);
}

// nodejs
try {
  if (process.env.NPM_TOKEN) {
    const npmrc = `//registry.npmjs.org/:_authToken=${process.env.NPM_TOKEN}`;
    writeFileSync('.npmrc', npmrc);

    run('cd nodejs && npm publish');
  }
} catch (e) {
  console.error('Failed to publish nodejs', e);
}
