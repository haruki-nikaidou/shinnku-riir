export enum Language {
  cn = 'cn',
  hk = 'hk',
  tw = 'tw'
}

export function getLanguage(): Language {
  const userLanguage = navigator.language || navigator.languages[0];
  switch (userLanguage) {
    case 'zh-CN':
      return Language.cn;
    case 'zh-HK':
      return Language.hk;
    case 'zh-TW':
      return Language.tw;
    default:
      return Language.cn;
  }
}

export interface Translation {
  cn: string;
  hk: string;
  tw: string;
}