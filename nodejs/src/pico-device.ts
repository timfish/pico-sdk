import { Native } from './native';
import { readCString, isNull, reinterpret } from 'ref-napi';
import { Lazy } from './lazy';
import { arch, platform } from 'os';
import { resolve, join } from 'path';

const native = new Lazy<Native>(() => {
  const paths: { [platform: string]: string } = {
    'win32-x64': 'artifacts/windows-x86_64/pico_native.dll',
    'win32-x32': 'artifacts/windows-i686/pico_native.dll',
    'darwin-x64': 'artifacts/macos-x86_64/libpico_native.dylib',
    'linux-x64': 'artifacts/linux-x86_64/libpico_native.so',
    'linux-arm': 'artifacts/linux-arm/libpico_native.so',
  };

  const path = paths[`${platform()}-${arch()}`];

  if (path == undefined) {
    throw new Error(
      `Unsupported platform/architecture combination: ${platform()}-${arch()}`
    );
  }

  const absPath = resolve(join(__dirname, path));
  return new Native(absPath);
});

export interface DiscoveredDevice {
  variant: string;
  serial: string;
}

export class PicoError extends Error {
  constructor(message: string) {
    super(message);
  }
}

export type CallbackFn = (data: Map<string, Float32Array>) => void;

export class PicoDevice {
  private callback?: CallbackFn;

  public static async enumerate(
    downloadMissingDrivers: boolean = false
  ): Promise<DiscoveredDevice[]> {
    const strPtr = await native.value.enumerate_devices(downloadMissingDrivers);

    return this.readString(strPtr)
      .split(',')
      .filter((d) => d.includes(':'))
      .map((d) => {
        const [variant, serial] = d.split(':');
        return { variant, serial };
      });
  }

  public static async open(
    serial?: string,
    downloadMissingDrivers: boolean = false
  ): Promise<PicoDevice> {
    const devicePtr = await native.value.device_open(
      serial,
      downloadMissingDrivers
    );

    if (isNull(devicePtr)) {
      this.throwError();
    }

    return new PicoDevice(devicePtr);
  }

  private constructor(private readonly devicePtr: Buffer) {
    const result = native.value.device_set_callback(
      this.devicePtr,
      (chNames, samplesPtr, samplesPerCh) =>
        this.callbackHandler(chNames, samplesPtr, samplesPerCh)
    );

    if (result == false) {
      PicoDevice.throwError();
    }
  }

  public get variant(): string {
    const strPtr = native.value.device_get_variant(this.devicePtr);
    return PicoDevice.readString(strPtr);
  }

  public get serial(): string {
    const strPtr = native.value.device_get_serial(this.devicePtr);
    return PicoDevice.readString(strPtr);
  }

  public close(): void {
    native.value.device_free(this.devicePtr);
  }

  public getChannelRanges(channel: string): string[] {
    const strPtr = native.value.device_get_channel_ranges(
      this.devicePtr,
      channel
    );
    const ranges = PicoDevice.readString(strPtr);
    return ranges.split(',');
  }

  public enableChannel(
    channel: string,
    range: string,
    coupling: string = 'DC'
  ): void {
    const result = native.value.device_enable_channel(
      this.devicePtr,
      channel,
      range,
      coupling
    );

    if (result == false) {
      PicoDevice.throwError();
    }
  }

  public disableChannel(channel: string): void {
    const result = native.value.device_disable_channel(this.devicePtr, channel);

    if (result == false) {
      PicoDevice.throwError();
    }
  }

  public setCallback(callback: CallbackFn): void {
    this.callback = callback;
  }

  public async startStreaming(samplesPerSecond: number): Promise<number> {
    const result = await native.value.device_start_streaming(
      this.devicePtr,
      samplesPerSecond
    );

    if (result === 0) {
      PicoDevice.throwError();
    }

    return result;
  }

  public stopStreaming(): void {
    const result = native.value.device_stop_streaming(this.devicePtr);

    if (result == false) {
      PicoDevice.throwError();
    }
  }

  private static throwError() {
    const strPtr = native.value.last_error();

    if (isNull(strPtr)) {
      throw new PicoError('Invalid error returned');
    }

    const errStr = this.readString(strPtr);

    throw new PicoError(errStr);
  }

  private static readString(ptr: Buffer): string {
    if (isNull(ptr)) {
      this.throwError();
    }

    try {
      return readCString(ptr);
    } finally {
      native.value.string_free(ptr);
    }
  }

  private callbackHandler(
    chNames: Buffer,
    samplesPtr: Buffer,
    samplesPerCh: number
  ): void {
    if (this.callback) {
      const channels = readCString(chNames).split(',');

      let samplesBuffer = reinterpret(
        samplesPtr,
        samplesPerCh * 4 * channels.length
      );

      let floatSamples = new Float32Array(
        samplesBuffer.buffer,
        0,
        samplesPerCh * channels.length
      );

      const data = new Map();

      let position = 0;
      for (var channel of channels) {
        const chSamples = floatSamples.slice(position, position + samplesPerCh);

        data.set(channel, chSamples);
        position += samplesPerCh;
      }

      this.callback(data);
    }
  }
}
