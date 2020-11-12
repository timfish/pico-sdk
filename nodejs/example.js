const { PicoDevice } = require('./dist');

const keypress = async () => {
  process.stdin.setRawMode(true);
  return new Promise((resolve) =>
    process.stdin.once('data', () => {
      process.stdin.setRawMode(false);
      resolve();
    })
  );
};

async function main() {
  const devices = await PicoDevice.enumerate(true);
  console.log('Discovered devices:', devices);

  const device = await PicoDevice.open(undefined, true);
  console.log('Device variant:', device.variant);
  console.log('Device serial:', device.serial);

  device.enableChannel('A', '200mV');
  device.enableChannel('b', '20 v', 'dc');
  const ranges = device.getChannelRanges('A');
  console.log('Valid ranges for channel A: ', ranges);

  device.setCallback((data) => {
    console.log('Received streaming data...');
    data.forEach((samples, ch) => {
      console.log(`Channel ${ch} has ${samples.length} samples`);
    });
  });

  const samplesPerSecond = await device.startStreaming(1_000_000);

  console.log(`Started streaming with ${samplesPerSecond} samples per second`);

  console.log('Hit any key to stop and exit');
  await keypress();

  device.stopStreaming();
  device.close();
}

main()
  .then(async () => {
    process.exit(0);
  })
  .catch((e) => {
    console.error(e);
    process.exit(1);
  });
