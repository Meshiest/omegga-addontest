import OmeggaPlugin, { OL, PS, PC } from 'omegga';

type Config = { foo: string };
type Storage = { bar: string };

import { sum, getVersion } from './addontest.node';

export default class Plugin implements OmeggaPlugin<Config, Storage> {
  omegga: OL;
  config: PC<Config>;
  store: PS<Storage>;

  constructor(omegga: OL, config: PC<Config>, store: PS<Storage>) {
    this.omegga = omegga;
    this.config = config;
    this.store = store;
  }

  async init() {
    this.omegga.on('cmd:sum', (speaker: string, a: string, b: string) => {
      const numA = Math.round(Number(a));
      const numB = Math.round(Number(b));
      if (Number.isNaN(numA) || Number.isNaN(numB)) {
        Omegga.whisper(
          speaker,
          OMEGGA_UTIL.chat.sanitize('usage: /sum <number> <number>')
        );
        return;
      }

      Omegga.whisper(speaker, `${numA} + ${numB} = ${sum(numA, numB)}`);
    });

    this.omegga.on('cmd:version', () => {
      getVersion(this.omegga);
    });

    return { registeredCommands: ['sum', 'version'] };
  }

  async stop() {}
}
