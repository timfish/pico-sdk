const { run, copyDir } = require('./tools');

// Python
copyDir('artifacts', 'python/pico_sdk/artifacts');
run('python setup.py sdist', 'python');

// nodejs
copyDir('artifacts', 'nodejs/dist/artifacts');
run('yarn && yarn build && yarn pack', 'nodejs');

// dotnet
run(`dotnet build ./dotnet/PicoSDK.csproj --force --configuration Release`);
run('dotnet nuget locals all --clear');
