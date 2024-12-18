/*
 * @Author: xhliu
 * @Date: 2024-08-01 11:06:06
 * @LastEditors: Z&N
 * @LastEditTime: 2024-11-01 17:34:47
 * @Description:
 * Copyright (C) 2016-2024 - Beijing Linx Software Corp.
 */
import { deepCopy } from "@/utils/clone"

type DoneFn = (cancel?: boolean) => void

export type DialogBeforeCloseFn = (done: DoneFn) => void

export interface DialogInfoInstance {
  title: string,
  width: string | number,
  height?: string | number,
  component: Comment | null,
  beforeClose?: DialogBeforeCloseFn,
  componentData?: Object,
  componentEvent?: Object
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

export function getDefaultDialogInfo(): DialogInfoInstance {
  return deepCopy(DefaultDialogInfo) as DialogInfoInstance
}
