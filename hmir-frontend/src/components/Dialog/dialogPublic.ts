/*
 * @Author: xhliu
 * @Date: 2024-08-01 11:06:06
 * @LastEditors: Z&N dev17101@linx-info.com
 * @LastEditTime: 2024-10-25 16:43:15
 * @Description:
 * Copyright (C) 2016-2024 - Beijing Linx Software Corp.
 */
import { deepCopy } from "@/utils/clone"

type DoneFn = (cancel?: boolean) => void

export type DialogBeforeCloseFn = (done: DoneFn) => void

interface DialogInfoInstance {
  title: string,
  width: string | number,
  height: string | number,
  component: Comment | null,
  beforeClose: DialogBeforeCloseFn,
  componentData: Object,
  componentEvent: Object
}

const DefaultDialogInfo: DialogInfoInstance = {
  title: '',
  width: '',
  height: 'auto',
  component: null,
  beforeClose: (done: DoneFn) => done(),
  componentData: {},
  componentEvent: {}
}

export function getDefaultDialogInfo() {
  return deepCopy(DefaultDialogInfo)
}
