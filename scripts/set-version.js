const { findReplace } = require('./tools');

function setVersion(newVersion) {
  findReplace(/Cargo\.toml$/, [
    [/^version = "\d+\.\d+\.\d+"/gm, `version = "${newVersion}"`],
    [/, version = "\d+\.\d+\.\d+"}/gm, `, version = "${newVersion}"}`],
  ]);

  findReplace(/package\.json$/, [
    [/"version": "\d+\.\d+\.\d+",/g, `"version": "${newVersion}",`],
  ]);

  findReplace(/README\.md$/, [
    [/pico-sdk = "\d+\.\d+\.\d+"/g, `pico-sdk = "${newVersion}"`],
  ]);

  findReplace(/\.csproj$/, [
    [/<Version>\d+\.\d+\.\d+<\/Version>/g, `<Version>${newVersion}</Version>`],
    [/Version="\d+\.\d+\.\d+"/g, `Version="${newVersion}"`],
  ]);

  findReplace(/setup\.py$/, [
    [/version='\d+\.\d+\.\d+'/g, `version='${newVersion}'`],
  ]);
}

const argVersion = process.argv.slice(2)[0];

if (argVersion && process.argv[1] === __filename) {
  console.log('Version', argVersion, 'set via command line.');
  setVersion(argVersion);
  return;
}

const ref = process.env.GITHUB_REF || '';
const result = ref.match(/refs\/tags\/v?(\d+\.\d+\.\d+)/);
const tagVersion = result ? result[1] : undefined;

if (tagVersion) {
  console.log('Version', tagVersion, 'from GitHub tag.');
  setVersion(tagVersion);
  return;
}

console.log('No version change required');
