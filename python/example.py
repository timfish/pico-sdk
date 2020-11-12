from pico_sdk import PicoDevice

found = PicoDevice.enumerate(True)
for device in found:
    print('PicoScope ' + device.variant + ' with serial ' + device.serial)

device = PicoDevice.open(None, True)

print('Device Serial: ' + device.get_serial())
print('Device variant: ' + device.get_variant())
print('Valid ranges for channel A: ' +
      ', '.join(device.get_channel_ranges('A')))

device.enable_channel('A', '200mV')
device.enable_channel('b', '20 v', 'dc')


def callback(data):
    print('Received streaming data...')
    for channel in data:
        print('Channel ' + channel + ' has ' +
              str(data[channel].size) + ' samples')


device.set_callback(callback)
samples_per_second = device.start_streaming(1_000_000)

print('Streaming started with ' + str(samples_per_second) + ' samples per second')
input('Hit ENTER to stop and exit\n')
