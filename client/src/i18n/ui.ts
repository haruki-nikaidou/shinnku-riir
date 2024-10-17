import {getLanguage, Translation} from "./utils.ts";

const UiText: { [key: string]: Translation} = {
  'search': {
    cn: '搜索',
    hk: '搜索',
    tw: '搜索'
  },
  'translated': {
    cn: '汉化版',
    hk: '中文版',
    tw: '中文版'
  },
  'download': {
    cn: '下载',
    hk: '下載',
    tw: '下載'
  }
}

export function t_ui(key: string): string {
  return UiText[key]?.[getLanguage()] || key;
}