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
  },
  'tutorial': {
    cn: '新手教程',
    hk: '新手教程',
    tw: '新手教程'
  },
  'random_roll_one': {
    cn: '试试手气',
    hk: '試試手氣',
    tw: '試試手氣'
  }
}

export function t_ui(key: string): string {
  return UiText[key]?.[getLanguage()] || key;
}