import {
  Callback,
  Library,
  Method,
  never,
  stdCallOnWin32,
} from 'ffi-decorators';

export type DataCallback = (
  channelNames: Buffer,
  samples: Buffer,
  samplesPerChannel: number
) => void;

@Library()
export class Native {
  // tslint:disable-next-line:unnecessary-constructor
  public constructor(path?: string) {}

  @Method({ types: ['char *', ['bool']] })
  public enumerate_devices(
    downloadMissingDrivers: boolean = true
  ): Promise<Buffer> {
    return never();
  }

  @Method({ types: ['pointer', ['string', 'bool']] })
  public device_open(
    serial: string | undefined,
    downloadMissingDrivers: boolean
  ): Promise<Buffer> {
    return never();
  }

  @Method({ types: ['void', ['pointer']] })
  public device_free(devicePtr: Buffer): void {
    return never();
  }

  @Method({ types: ['char *', ['pointer']] })
  public device_get_variant(devicePtr: Buffer): Buffer {
    return never();
  }

  @Method({ types: ['char *', ['pointer']] })
  public device_get_serial(devicePtr: Buffer): Buffer {
    return never();
  }

  @Method({ types: ['char *', ['pointer', 'string']] })
  public device_get_channel_ranges(devicePtr: Buffer, channel: string): Buffer {
    return never();
  }

  @Method({ types: ['bool', ['pointer', 'string', 'string', 'string']] })
  public device_enable_channel(
    devicePtr: Buffer,
    channel: string,
    range: string,
    coupling: string
  ): boolean {
    return never();
  }

  @Method({ types: ['bool', ['pointer', 'string']] })
  public device_disable_channel(devicePtr: Buffer, channel: string): boolean {
    return never();
  }

  @Method({ types: ['bool', ['pointer', 'pointer']] })
  public device_set_callback(
    devicePtr: Buffer,
    @Callback({
      abi: stdCallOnWin32(),
      types: ['void', ['char *', 'float**', 'uint32']],
    })
    callback: DataCallback
  ): boolean {
    return never();
  }

  @Method({ types: ['uint32', ['pointer', 'uint32']] })
  public device_start_streaming(
    devicePtr: Buffer,
    samplesPerSecond: number
  ): Promise<number> {
    return never();
  }

  @Method({ types: ['bool', ['pointer']] })
  public device_stop_streaming(devicePtr: Buffer): boolean {
    return never();
  }

  @Method({ types: ['char *', []] })
  public last_error(): Buffer {
    return never();
  }

  @Method({ types: ['void', ['char *']] })
  public string_free(stringPtr: Buffer): void {
    return never();
  }
}
